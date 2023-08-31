use std::collections::BTreeMap;
use crate::{NO_VARS, SmooFun, Var};
use crate::basic::zero::Zero;

#[derive(Copy, Clone)]
pub struct Cons {
    value: f64
}

impl Cons {
    pub fn new(value: f64) -> Cons { Cons { value }}
}

impl SmooFun for Cons {
    fn vars(&self) -> &[Var] { &NO_VARS }
    fn apply(&self, _args: &[f64]) -> f64 { self.value }
    fn copy(&self) -> Box<dyn SmooFun> { Box::new(*self) }
    fn bind(&self, _vars: &[Var], _args: &[f64]) -> Box<dyn SmooFun> { Box::new(*self) }
    fn derivative(&self, _var: &Var) -> Box<dyn SmooFun> { Box::<Zero>::default() }
    fn notation(&self) -> String { self.value.to_string() }
    fn composed_notation(&self, _map: BTreeMap<Var, String>) -> String { self.value.to_string() }
}