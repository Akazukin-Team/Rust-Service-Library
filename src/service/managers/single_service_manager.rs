use crate::service::holder::service_holder::ServiceHolder;
use crate::service::managers::multi_service_manager::{
    MultiServiceManager, MultiServiceManagerImpl,
};
use crate::service::managers::service_manager::ServiceManager;
use crate::service::managers::service_store::ServiceStore;
use crate::service::registries::registry::ServiceRegistry;

pub trait SingleServiceManager<T>: ServiceManager<T> {
    fn get_service_by_id(&self, id: u16) -> Option<&T>;
    fn get_holder_by_id(&self, id: u16) -> Option<&dyn ServiceHolder<T>>;
}

pub struct SingleServiceManagerImpl<T> {
    base: Box<dyn MultiServiceManager<T>>,
}

impl<T: 'static> SingleServiceManagerImpl<T> {
    pub fn new(registry: Box<dyn ServiceRegistry<T>>) -> Self {
        Self {
            base: Box::new(MultiServiceManagerImpl::new(registry)),
        }
    }

    pub fn get_registry(&mut self) -> &mut Box<dyn ServiceRegistry<T>> {
        self.base.get_registry()
    }
}

impl<T> ServiceStore<T> for SingleServiceManagerImpl<T> {
    fn get_all_services(&self) -> Vec<&T> {
        self.base.get_all_services()
    }

    fn get_all_holders(&self) -> Vec<&dyn ServiceHolder<T>> {
        self.base.get_all_holders()
    }

    fn contains_service(&self, id: u16, service: &T) -> bool {
        self.base.contains_service(id, service)
    }

    fn contains_service_by_impl(&self, service: &T) -> bool {
        self.base.contains_service_by_impl(service)
    }

    fn contains_service_by_id(&self, id: u16) -> bool {
        self.base.contains_service_by_id(id)
    }
}

impl<T> ServiceManager<T> for SingleServiceManagerImpl<T> {
    fn get_registry(&mut self) -> &mut Box<dyn ServiceRegistry<T>> {
        self.base.get_registry()
    }
}

impl<T> SingleServiceManager<T> for SingleServiceManagerImpl<T> {
    fn get_service_by_id(&self, id: u16) -> Option<&T> {
        self.base.get_services_by_id(id).iter().copied().next()
    }

    fn get_holder_by_id(&self, id: u16) -> Option<&dyn ServiceHolder<T>> {
        self.base.get_holders_by_id(id).iter().copied().next()
    }
}
