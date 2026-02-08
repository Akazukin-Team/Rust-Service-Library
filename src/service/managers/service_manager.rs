use crate::service::managers::service_store::ServiceStore;
use crate::service::registries::registry::ServiceRegistry;

pub trait ServiceManager<T>: ServiceStore<T> {
    fn get_registry(&mut self) -> &mut Box<dyn ServiceRegistry<T>>;
}
