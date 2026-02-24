use crate::service::holders::service_holder::ServiceHolder;
use crate::service::managers::service_store::ServiceStore;

pub trait ServiceRegistry<T>: ServiceStore<T> {
    fn register_service(&mut self, id: u16, service: T) -> Result<&dyn ServiceHolder<T>, String>;
    fn unregister_service(
        &mut self,
        id: u16,
        service: &T,
    ) -> Result<Vec<Box<dyn ServiceHolder<T>>>, String>;
    fn unregister_service_by_id(
        &mut self,
        id: u16,
    ) -> Result<Vec<Box<dyn ServiceHolder<T>>>, String>;
    fn unregister_service_by_impl(
        &mut self,
        service: &T,
    ) -> Result<Vec<Box<dyn ServiceHolder<T>>>, String>;
}
