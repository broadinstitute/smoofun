mod basic;

use std::collections::BTreeMap;
use crate::basic::var::Var;

pub(crate) static NO_VARS: Vec<Var> = vec!();

pub trait SmooFun {
    fn vars(&self) -> &[Var];
    fn apply(&self, args: &[f64]) -> f64;
    fn copy(&self) -> Box<dyn SmooFun>;
    fn bind(&self, vars: &[Var], args: &[f64]) -> Box<dyn SmooFun>;
    fn derivative(&self, var: &Var) -> Box<dyn SmooFun>;
    fn notation(&self) -> String;
    fn composed_notation(&self, map: BTreeMap<Var, String>) -> String;
}

