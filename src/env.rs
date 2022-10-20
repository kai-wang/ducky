use std::{collections::HashMap, thread::JoinHandle};
use crate::val::Val;


#[derive(Debug, PartialEq, Default)]
pub(crate) struct Env<'parent> {
    bindings: HashMap<String, Val>,
    parent: Option<&'parent Self>
}

impl<'p> Env<'p> {
    
    pub(crate) fn create_child(&'p self) -> Self {
        Self {
            bindings: HashMap::new(),
            parent: Some(self)
        }
    }

    pub(crate) fn store_bindings(&mut self, name: String, val: Val) {
        self.bindings.insert(name, val);
    }

    pub(crate) fn get_binding_value(&self, name: &str) -> Result<Val, String> {
        self.get_binding_value_inner(name)
            .ok_or_else(|| format!("binding with name '{}' does not exist", name))
    }

    fn get_binding_value_inner(&self, name: &str) -> Option<Val> {
        self.bindings
            .get(name)
            .cloned()
            .or_else(|| {
                self.parent.and_then(|parent| parent.get_binding_value_inner(name))
            })

    }
}