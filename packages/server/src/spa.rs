use std::{
    collections::HashMap,
    convert::Infallible,
    io::{Error, ErrorKind},
    path::PathBuf,
};

use futures::StreamExt;
use poem::{
    endpoint::StaticFilesEndpoint,
    http::{StatusCode, Uri},
    Body, Endpoint, Request, Response, Result,
};

pub struct SpaEndpoint {
    static_files: StaticFilesEndpoint,
    index_html_prefix: String,
    index_html_suffix: String,
}

impl SpaEndpoint {
    pub fn setup(assets_dir: impl Into<PathBuf>) -> std::io::Result<Self> {
        let assets_dir = assets_dir.into();

        tracing::debug!(
            "Loading index.html from assets directory '{}'",
            assets_dir.display()
        );

        let index_html = std::fs::read_to_string(assets_dir.join("index.html"))?;
        let body_start = index_html.find("<body>").ok_or(Error::new(
            ErrorKind::NotFound,
            "Invalid index.html, could not find a <body> tag",
        ))? + 6;
        let (index_html_prefix, index_html_suffix) = index_html.split_at(body_start);
        let (index_html_prefix, index_html_suffix) =
            (index_html_prefix.to_owned(), index_html_suffix.to_owned());

        let static_files = StaticFilesEndpoint::new(assets_dir);

        Ok(Self {
            static_files,
            index_html_prefix,
            index_html_suffix,
        })
    }

    fn handle_spa(&self, url: Uri, query: HashMap<String, String>) -> Result<Response> {
        let url = url.to_string();

        let renderer =
            yew::ServerRenderer::<ui::ServerApp>::with_props(move || ui::ServerAppProps {
                url: url.into(),
                query,
            });

        let prefix: std::result::Result<String, Infallible> = Ok(self.index_html_prefix.clone());
        let suffix: std::result::Result<String, Infallible> = Ok(self.index_html_suffix.clone());

        let prefix = futures::stream::once(async move { prefix });
        let suffix = futures::stream::once(async move { suffix });
        let content = renderer.render_stream().map(Ok);

        let stream = prefix.chain(content).chain(suffix);

        Ok(Response::builder()
            .status(StatusCode::OK)
            .content_type("text/html; charset=utf-8")
            .body(Body::from_bytes_stream(stream)))
    }
}

#[poem::async_trait]
impl Endpoint for SpaEndpoint {
    type Output = Response;

    async fn call(&self, req: Request) -> Result<Self::Output> {
        let url = req.uri().to_owned();
        let query = req
            .params::<HashMap<String, String>>()
            .unwrap_or(HashMap::new());

        match self.static_files.call(req).await {
            Ok(res) => Ok(res),
            Err(err) => match err.status() {
                StatusCode::NOT_FOUND => self.handle_spa(url, query),
                _ => Err(err),
            },
        }
    }
}
