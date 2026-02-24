pub trait ServiceHolder<T> {
    fn get_id(&self) -> u16;
    fn get_impl(&self) -> &T;
}

pub struct ServiceHolderImpl<T> {
    id: u16,
    r#impl: T,
}

impl<T> ServiceHolderImpl<T> {
    pub fn new(id: u16, impl_: T) -> Self {
        Self { id, r#impl: impl_ }
    }
}

impl<T> ServiceHolder<T> for ServiceHolderImpl<T> {
    fn get_id(&self) -> u16 {
        self.id
    }

    fn get_impl(&self) -> &T {
        &self.r#impl
    }
}
