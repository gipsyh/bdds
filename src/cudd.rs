use crate::{Bdd, BddManager};
use cudd::Cudd;

impl Bdd for cudd::Bdd {
    fn is_constant(&self, val: bool) -> bool {
        self.is_constant(val)
    }

    fn if_then_else(&self, _then: &Self, _else: &Self) -> Self {
        self.if_then_else(_then, _else)
    }

    fn next_state(&self) -> Self {
        self.next_state()
    }

    fn pre_image(&self, trans: &Self) -> Self {
        self.pre_image(trans)
    }

    fn post_image(&self, trans: &Self) -> Self {
        self.post_image(trans)
    }
}

impl BddManager for Cudd {
    type Bdd = cudd::Bdd;

    fn new() -> Self {
        Self::new()
    }

    fn new_with_capacity(capacity: usize) -> Self {
        todo!()
    }

    fn constant(&self, val: bool) -> Self::Bdd {
        self.constant(val)
    }

    fn ith_var(&self, var: usize) -> Self::Bdd {
        self.ith_var(var)
    }
}
