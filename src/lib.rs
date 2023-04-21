mod cudd;
mod peabody;

use std::{
    fmt::Debug,
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not},
};

pub trait Bdd: Sized + PartialEq
where
    Self: Not<Output = Self>
        + BitAnd<Self, Output = Self>
        + BitAndAssign<Self>
        + BitOr<Self, Output = Self>
        + BitOrAssign<Self>
        + BitXor<Self, Output = Self>
        + BitXorAssign<Self>,
    for<'a> Self: BitAnd<&'a Self, Output = Self>
        + BitAndAssign<&'a Self>
        + BitOr<&'a Self, Output = Self>
        + BitOrAssign<&'a Self>
        + BitXor<&'a Self, Output = Self>
        + BitXorAssign<&'a Self>,
    for<'a, 'b> &'a Self: Not<Output = Self>
        + BitAnd<Self, Output = Self>
        + BitAnd<&'b Self, Output = Self>
        + BitOr<Self, Output = Self>
        + BitOr<&'b Self, Output = Self>
        + BitXor<Self, Output = Self>
        + BitXor<&'b Self, Output = Self>,
{
    fn is_constant(&self, val: bool) -> bool;

    fn if_then_else(&self, _then: &Self, _else: &Self) -> Self;

    fn next_state(&self) -> Self;

    fn pre_image(&self, trans: &Self) -> Self;

    fn post_image(&self, trans: &Self) -> Self;
}

pub trait BddManager: Sized + Clone + Debug
where
    for<'a, 'b> &'a Self::Bdd: Not<Output = Self::Bdd>
        + BitAnd<Self::Bdd, Output = Self::Bdd>
        + BitAnd<&'b Self::Bdd, Output = Self::Bdd>
        + BitOr<Self::Bdd, Output = Self::Bdd>
        + BitOr<&'b Self::Bdd, Output = Self::Bdd>
        + BitXor<Self::Bdd, Output = Self::Bdd>
        + BitXor<&'b Self::Bdd, Output = Self::Bdd>,
{
    type Bdd: Bdd;

    fn new() -> Self;

    fn new_with_capacity(capacity: usize) -> Self;

    fn constant(&self, val: bool) -> Self::Bdd;

    fn ith_var(&self, var: usize) -> Self::Bdd;
}
