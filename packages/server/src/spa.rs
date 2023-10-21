use poem::{endpoint::StaticFilesEndpoint, Endpoint, Request, Result};

pub struct SpaEndpoint {
    static_files: StaticFilesEndpoint,
}

#[poem::async_trait]
impl Endpoint for SpaEndpoint {
    type Output = <StaticFilesEndpoint as Endpoint>::Output;

    async fn call(&self, req: Request) -> Result<Self::Output> {
        todo!()
    }
}

// pub async fn serve<B>(
//     assets_dir: PathBuf,
// ) -> impl tower::Service<
//     Request<B>,
//     Error = Infallible,
//     Response = impl IntoResponse,
//     Future = impl Send + 'static,
// > + Clone
//        + Send
//        + 'static {
//     let index_html_s = tokio::fs::read_to_string(assets_dir.join("index.html"))
//         .await
//         .expect("failed to read index.html");

//     let (index_html_before, index_html_after) = index_html_s.split_once("<body>").unwrap();
//     let mut index_html_before = index_html_before.to_owned();
//     index_html_before.push_str("<body>");

//     let index_html_after = index_html_after.to_owned();

//     let handle_error = |e| async move {
//         (
//             StatusCode::INTERNAL_SERVER_ERROR,
//             format!("error occurred: {e}"),
//         )
//     };

//     HandleError::new(
//         ServeDir::new(assets_dir)
//             .append_index_html_on_directories(false)
//             .fallback(
//                 get(render)
//                     .with_state((index_html_before.clone(), index_html_after.clone()))
//                     .into_service()
//                     .map_err(|err| -> std::io::Error { match err {} }),
//             ),
//         handle_error,
//     )
// }

// async fn render(
//     url: Uri,
//     Query(query): Query<HashMap<String, String>>,
//     State((index_html_before, index_html_after)): State<(String, String)>,
// ) -> impl IntoResponse {
//     let url = url.to_string();

//     let renderer = yew::ServerRenderer::<ui::ServerApp>::with_props(move || ui::ServerAppProps {
//         url: url.into(),
//         query,
//     });

//     StreamBody::new(
//         stream::once(async move { index_html_before })
//             .chain(renderer.render_stream())
//             .chain(stream::once(async move { index_html_after }))
//             .map(Result::<_, Infallible>::Ok),
//     )
// }

// // fn handle_error(error: E) {}
