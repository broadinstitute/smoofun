use std::collections::BTreeMap;
use crate::{NO_VARS, SmooFun, Var};

#[derive(Default, Copy, Clone)]
pub struct Zero {}

impl SmooFun for Zero {
    fn vars(&self) -> &[Var] { &NO_VARS }
    fn apply(&self, _args: &[f64]) -> f64 { 0.0 }
    fn copy(&self) -> Box<dyn SmooFun> { Box::new(*self) }
    fn bind(&self, _vars: &[Var], _args: &[f64]) -> Box<dyn SmooFun> { Box::<Zero>::default() }
    fn derivative(&self, _var: &Var) -> Box<dyn SmooFun> { Box::<Zero>::default() }
    fn notation(&self) -> String { "0".to_string() }
    fn composed_notation(&self, _map: BTreeMap<Var, String>) -> String { "0".to_string() }
}