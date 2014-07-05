#![crate_id = "conduit-persistent"]

extern crate conduit;
extern crate middleware = "conduit-middleware";
extern crate sync;

use conduit::Request;
use middleware::Middleware;

use sync::{Arc, RWLock};
use std::fmt::Show;
use std::any::{Any, AnyMutRefExt};
use std::collections::HashMap;

pub type BoundAny = Box<Any + Send + Share>;
pub type Shared<T> = Arc<RWLock<T>>;
type PersistentStore = HashMap<String, Shared<BoundAny>>;

pub struct Persisted {
    key: String,
    data: Shared<BoundAny>
}

impl Persisted {
    pub fn new(key: String, default: BoundAny) -> Persisted {
        Persisted {
            key: key,
            data: Arc::new(RWLock::new(default))
        }
    }
}

fn new_store() -> PersistentStore { HashMap::new() }

impl Middleware for Persisted {
    fn before(&self, req: &mut Request) -> Result<(), Box<Show>> {
        let store = req.mut_extensions()
            .find_or_insert_with("persistent.store",
                                 |_| box new_store() as Box<Any>);
        store.as_mut::<PersistentStore>().unwrap().insert(
            self.key.clone(),
            self.data.clone()
        );
        Ok(())
    }
}

