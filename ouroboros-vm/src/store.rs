use std::collections::HashMap;

use crate::Func;

#[async_trait::async_trait]
pub trait Store {
    async fn insert_func(&mut self, func: Func) -> anyhow::Result<()>;
    async fn get_func(&self, name: &str) -> anyhow::Result<Option<Func>>;
}

pub struct InMemoryStore {
    funcs: HashMap<String, Func>,
}

impl InMemoryStore {
    pub fn new() -> Self {
        Self {
            funcs: HashMap::new(),
        }
    }
}

#[async_trait::async_trait]
impl Store for InMemoryStore {
    async fn insert_func(&mut self, func: Func) -> anyhow::Result<()> {
        self.funcs.insert(func.name.clone(), func); // FIXME: Should we allow overriding previously deployed functions?
        Ok(())
    }

    async fn get_func(&self, name: &str) -> anyhow::Result<Option<Func>> {
        Ok(self.funcs.get(name).cloned())
    }
}
