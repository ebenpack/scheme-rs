use std::{cell::RefCell, collections::HashMap, rc::Rc};
use wasm_bindgen::prelude::*;

use crate::lisp_val::LispVal;

pub type Bindings = HashMap<String, LispVal>;

pub struct Environment {
    bindings: RefCell<Bindings>,
    parent: Option<Env>,
}

pub type Signal = Box<dyn FnMut(&mut Vec<LispVal>)>;

#[wasm_bindgen]
pub struct Port {
    _signal: Signal,
    data: Vec<LispVal>,
}

impl Port {
    pub fn push(&mut self, data: LispVal) {
        self.data.push(data)
    }
    pub fn signal(&mut self) {
        (self._signal)(&mut self.data)
    }
    pub fn flush(&mut self) -> Vec<LispVal> {
        self.data.drain(..).collect()
    }
}

#[derive(Clone)]
pub struct Ports(Rc<HashMap<String, Rc<RefCell<Port>>>>);

impl Ports {
    pub fn new(default_signal: Signal) -> Self {
        Ports(Rc::new(HashMap::from([(
            "default".to_string(),
            Rc::new(RefCell::new(Port {
                _signal: default_signal,
                data: vec![],
            })),
        )])))
    }
    pub fn get(&self, port: &str) -> Option<Rc<RefCell<Port>>> {
        self.0.get(port).cloned()
    }
    pub fn signal(&self, port: &str) {
        if let Some(port) = self.get(port) {
            let mut port = port.as_ref().borrow_mut();
            port.signal()
        }
    }
}

#[derive(Clone)]
pub struct Env {
    pub env: Rc<Environment>,
    pub ports: Ports,
    // TODO: ... input port, output port, events/callbacks for in/out
}

fn noop(_port: &mut Vec<LispVal>) {}

impl Default for Env {
    fn default() -> Self {
        Env {
            env: Rc::new(Environment {
                bindings: RefCell::new(HashMap::new()),
                parent: None,
            }),
            ports: Ports::new(Box::new(noop)),
        }
    }
}

impl Env {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_bindings(bindings: Bindings, ports: Ports) -> Self {
        Env {
            env: Rc::new(Environment {
                bindings: RefCell::new(bindings),
                parent: None,
            }),
            ports,
        }
    }

    pub fn push_frame(&self, bindings: Bindings) -> Self {
        let parent = Some(self.clone());
        let mut new_bindings = HashMap::new();
        for (key, value) in bindings {
            new_bindings.insert(key, value);
        }
        Env {
            env: Rc::new(Environment {
                bindings: RefCell::new(new_bindings),
                parent,
            }),
            ports: self.ports.clone(),
        }
    }

    pub fn lookup_local(&self, key: &str) -> Option<LispVal> {
        self.env.bindings.borrow().get(key).cloned()
    }

    pub fn is_bound_local(&self, key: &str) -> bool {
        self.lookup_local(key).is_some()
    }

    pub fn lookup(&self, key: &str) -> Option<LispVal> {
        let mut curr = &self.env;
        loop {
            if let Some(val) = curr.bindings.borrow().get(key) {
                return Some(val.clone());
            }
            match &curr.parent {
                Some(parent) => {
                    curr = &parent.env;
                }
                None => return None,
            }
        }
    }

    pub fn is_bound(&self, key: &str) -> bool {
        self.lookup(key).is_some()
    }

    pub fn bind(&self, key: &str, val: LispVal) {
        self.env.bindings.borrow_mut().insert(key.to_string(), val);
    }

    pub fn set_var(&self, key: &str, val: LispVal) -> bool {
        if self.is_bound_local(key) {
            self.bind(key, val);
            true
        } else if let Some(parent) = &self.env.parent {
            parent.set_var(key, val)
        } else {
            false
        }
    }
}
