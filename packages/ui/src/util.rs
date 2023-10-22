use std::rc::Rc;

pub struct ServiceProp<T: ?Sized>(pub Rc<T>);

impl<T> From<T> for ServiceProp<T> {
    fn from(value: T) -> Self {
        Self(Rc::new(value))
    }
}

impl<T: ?Sized> std::ops::Deref for ServiceProp<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl<T: ?Sized> PartialEq for ServiceProp<T> {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self.0.as_ref(), other.0.as_ref())
    }
}

impl<T: ?Sized> Clone for ServiceProp<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }

    fn clone_from(&mut self, source: &Self) {
        self.0.clone_from(&source.0)
    }
}
