use std::collections::BTreeMap;
use std::fmt::{Debug, Display, Formatter};
use std::slice::from_ref;
use crate::basic::cons::Cons;
use crate::SmooFun;

#[derive(Eq, PartialEq, Clone, Ord, PartialOrd)]
pub struct Var {
    pub name: String,
}

impl Var {
    pub fn new(name: String) -> Var { Var { name } }
}

impl SmooFun for Var {
    fn vars(&self) -> &[Var] { from_ref(self) }
    fn apply(&self, args: &[f64]) -> f64 { args[0] }
    fn copy(&self) -> Box<dyn SmooFun> { Box::new(self.clone()) }

    fn bind(&self, vars: &[Var], args: &[f64]) -> Box<dyn SmooFun> {
        if let Some(i) = vars.iter().position(|var| var == self) {
            Box::new(Cons::new(args[i]))
        } else {
            self.copy()
        }
    }

    fn derivative(&self, var: &Var) -> Box<dyn SmooFun> {
        if self == var {
            Box::new(Cons::new(1.0))
        } else {
            Box::new(Cons::new(0.0))
        }
    }

    fn notation(&self) -> String { self.name.clone() }
    fn composed_notation(&self, map: BTreeMap<Var, String>) -> String {
        if let Some(string) = map.get(self) {
            string.clone()
        } else {
            self.name.clone()
        }
    }
}

impl Debug for Var {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Display for Var {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}


