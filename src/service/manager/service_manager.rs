use crate::service::manager::service_store::ServiceStore;
use crate::service::registry::registry::ServiceRegistry;

pub trait ServiceManager<T>: ServiceStore<T> {
    fn get_registry(&mut self) -> &mut Box<dyn ServiceRegistry<T>>;
}
