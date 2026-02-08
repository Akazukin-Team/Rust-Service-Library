use crate::service::holder::service_holder::{ServiceHolder, ServiceHolderImpl};
use crate::service::managers::service_store::ServiceStore;
use crate::service::registries::registry::ServiceRegistry;
use std::ptr::eq;
use std::sync::Mutex;

pub struct MultiServiceRegistry<T> {
    holders: Vec<Box<dyn ServiceHolder<T>>>,
    safe_rc: Mutex<()>,
}

impl<T> MultiServiceRegistry<T> {
    pub fn new() -> Self {
        Self {
            holders: Vec::new(),
            safe_rc: Mutex::new(()),
        }
    }
}

impl<T> Default for MultiServiceRegistry<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: 'static> ServiceStore<T> for MultiServiceRegistry<T> {
    fn get_all_services(&self) -> Vec<&T> {
        self.get_all_holders()
            .iter()
            .map(|e| e.get_impl())
            .collect()
    }

    fn get_all_holders(&self) -> Vec<&dyn ServiceHolder<T>> {
        self.holders.iter().map(|e| &**e).collect()
    }

    fn contains_service(&self, id: u16, service: &T) -> bool {
        self.holders
            .iter()
            .any(|e| e.get_id() == id && eq(e.get_impl(), service))
    }

    fn contains_service_by_impl(&self, service: &T) -> bool {
        self.holders.iter().any(|e| eq(e.get_impl(), service))
    }

    fn contains_service_by_id(&self, id: u16) -> bool {
        self.holders.iter().any(|e| e.get_id() == id)
    }
}

impl<T: 'static> ServiceRegistry<T> for MultiServiceRegistry<T> {
    fn register_service(&mut self, id: u16, service: T) -> Result<&dyn ServiceHolder<T>, String> {
        let _lock = self.safe_rc.lock();
        self.holders
            .push(Box::new(ServiceHolderImpl::new(id, service)));
        Ok(self.holders.last().map(|e| &**e).unwrap())
    }

    fn unregister_service(
        &mut self,
        id: u16,
        service: &T,
    ) -> Result<Vec<Box<dyn ServiceHolder<T>>>, String> {
        let res: Vec<_> = self
            .holders
            .extract_if(0.., |e| e.get_id() == id && eq(e.get_impl(), service))
            .collect();

        if res.is_empty() {
            Err(format!("Service (Id:{}, Impl:{}) not found", id, "Some"))
        } else {
            Ok(res)
        }
    }

    fn unregister_service_by_id(
        &mut self,
        id: u16,
    ) -> Result<Vec<Box<dyn ServiceHolder<T>>>, String> {
        let res: Vec<_> = self.holders.extract_if(0.., |e| e.get_id() != id).collect();

        if res.is_empty() {
            Err(format!("Service (Id:{}) not found", id))
        } else {
            Ok(res)
        }
    }

    fn unregister_service_by_impl(
        &mut self,
        service: &T,
    ) -> Result<Vec<Box<dyn ServiceHolder<T>>>, String> {
        let res: Vec<_> = self
            .holders
            .extract_if(0.., |e| !eq(e.get_impl(), service))
            .collect();

        if res.is_empty() {
            Err(format!("Service (Impl:{}) not found", "Some"))
        } else {
            Ok(res)
        }
    }
}
