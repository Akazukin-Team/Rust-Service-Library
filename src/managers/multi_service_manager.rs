use crate::holders::service_holder::ServiceHolder;
use crate::managers::service_manager::ServiceManager;
use crate::managers::service_store::ServiceStore;
use crate::registries::registry::ServiceRegistry;

pub trait MultiServiceManager<T>: ServiceManager<T> {
    fn get_services_by_id(&self, id: u16) -> Vec<&T>;
    fn get_holders_by_id(&self, id: u16) -> Vec<&dyn ServiceHolder<T>>;
}

pub struct MultiServiceManagerImpl<T> {
    registry: Box<dyn ServiceRegistry<T>>,
}

impl<T> MultiServiceManagerImpl<T> {
    pub fn new(registry: Box<dyn ServiceRegistry<T>>) -> Self {
        Self { registry }
    }

    pub fn get_registry(&self) -> &dyn ServiceRegistry<T> {
        &*self.registry
    }
}

impl<T> ServiceStore<T> for MultiServiceManagerImpl<T> {
    fn get_all_services(&self) -> Vec<&T> {
        self.registry.get_all_services()
    }

    fn get_all_holders(&self) -> Vec<&dyn ServiceHolder<T>> {
        self.registry.get_all_holders()
    }

    fn contains_service(&self, id: u16, service: &T) -> bool {
        self.registry.contains_service(id, service)
    }

    fn contains_service_by_impl(&self, service: &T) -> bool {
        self.registry.contains_service_by_impl(service)
    }

    fn contains_service_by_id(&self, id: u16) -> bool {
        self.registry.contains_service_by_id(id)
    }
}

impl<T> ServiceManager<T> for MultiServiceManagerImpl<T> {
    fn get_registry(&mut self) -> &mut Box<dyn ServiceRegistry<T>> {
        &mut self.registry
    }
}

impl<T> MultiServiceManager<T> for MultiServiceManagerImpl<T> {
    fn get_services_by_id(&self, id: u16) -> Vec<&T> {
        self.registry
            .get_all_holders()
            .iter()
            .filter(|e| e.get_id() == id)
            .map(|e| e.get_impl())
            .collect()
    }

    fn get_holders_by_id(&self, id: u16) -> Vec<&dyn ServiceHolder<T>> {
        self.registry
            .get_all_holders()
            .iter()
            .filter(|&e| e.get_id() == id)
            .copied()
            .collect()
    }
}
