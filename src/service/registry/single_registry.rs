use crate::service::holder::service_holder::ServiceHolder;
use crate::service::manager::service_store::ServiceStore;
use crate::service::registry::multi_registry::MultiServiceRegistry;
use crate::service::registry::registry::ServiceRegistry;

pub struct SingleServiceRegistry<T> {
    registry: MultiServiceRegistry<T>,
}

impl<T: 'static> ServiceStore<T> for SingleServiceRegistry<T> {
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

impl<T: 'static> ServiceRegistry<T> for SingleServiceRegistry<T> {
    fn register_service(&mut self, id: u16, service: T) -> Result<&dyn ServiceHolder<T>, String> {
        if self.contains_service_by_id(id) {
            return Err(format!("Service (Id:{}) already exists", id));
        }

        self.registry.register_service(id, service)
    }

    fn unregister_service(
        &mut self,
        id: u16,
        service: &T,
    ) -> Result<Vec<Box<dyn ServiceHolder<T>>>, String> {
        self.registry.unregister_service(id, service)
    }

    fn unregister_service_by_id(
        &mut self,
        id: u16,
    ) -> Result<Vec<Box<dyn ServiceHolder<T>>>, String> {
        self.registry.unregister_service_by_id(id)
    }

    fn unregister_service_by_impl(
        &mut self,
        service: &T,
    ) -> Result<Vec<Box<dyn ServiceHolder<T>>>, String> {
        self.registry.unregister_service_by_impl(service)
    }
}
