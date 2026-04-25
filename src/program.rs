//! Top-level executable program.

use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use serde::{Deserialize, Serialize};

use crate::module::Import;
use crate::names::{ConstId, ModuleName};

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Program {
    pub name: ModuleName,
    pub imports: Vec<Import>,
    pub consts: Vec<ConstId>,
}
