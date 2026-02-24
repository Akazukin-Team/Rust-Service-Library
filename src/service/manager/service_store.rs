use crate::service::holder::service_holder::ServiceHolder;

pub trait ServiceStore<T> {
    fn get_all_services(&self) -> Vec<&T>;
    fn get_all_holders(&self) -> Vec<&dyn ServiceHolder<T>>;
    fn contains_service(&self, id: u16, service: &T) -> bool;
    fn contains_service_by_impl(&self, service: &T) -> bool;
    fn contains_service_by_id(&self, id: u16) -> bool;
}
