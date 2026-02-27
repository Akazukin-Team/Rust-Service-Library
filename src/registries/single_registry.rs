use crate::holders::service_holder::ServiceHolder;
use crate::managers::service_store::ServiceStore;
use crate::registries::multi_registry::MultiServiceRegistry;
use crate::registries::registry::ServiceRegistry;

pub struct SingleServiceRegistry<T> {
    delegate: MultiServiceRegistry<T>,
}

impl<T> ServiceStore<T> for SingleServiceRegistry<T> {
    fn get_all_services(&self) -> Vec<&T> {
        self.delegate.get_all_services()
    }

    fn get_all_holders(&self) -> Vec<&dyn ServiceHolder<T>> {
        self.delegate.get_all_holders()
    }

    fn contains_service(&self, id: u16, service: &T) -> bool {
        self.delegate.contains_service(id, service)
    }

    fn contains_service_by_impl(&self, service: &T) -> bool {
        self.delegate.contains_service_by_impl(service)
    }

    fn contains_service_by_id(&self, id: u16) -> bool {
        self.delegate.contains_service_by_id(id)
    }
}

impl<T: 'static> ServiceRegistry<T> for SingleServiceRegistry<T> {
    fn register_service(&mut self, id: u16, service: T) -> Result<&dyn ServiceHolder<T>, String> {
        if self.contains_service_by_id(id) {
            return Err(format!("Service (Id:{}) already exists", id));
        }

        self.delegate.register_service(id, service)
    }

    fn unregister_service(
        &mut self,
        id: u16,
        service: &T,
    ) -> Result<Vec<Box<dyn ServiceHolder<T>>>, String> {
        self.delegate.unregister_service(id, service)
    }

    fn unregister_service_by_id(
        &mut self,
        id: u16,
    ) -> Result<Vec<Box<dyn ServiceHolder<T>>>, String> {
        self.delegate.unregister_service_by_id(id)
    }

    fn unregister_service_by_impl(
        &mut self,
        service: &T,
    ) -> Result<Vec<Box<dyn ServiceHolder<T>>>, String> {
        self.delegate.unregister_service_by_impl(service)
    }
}
