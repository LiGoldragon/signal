//! Place-based lifetime annotations.
//!
//! Three origin forms:
//!   - PlaceRef: origin at a simple place (parameter or local)
//!   - PlacePath: origin at a field-path rooted in a place
//!   - PlaceUnion: union of origins (borrow came from any of these) —
//!     references other Origin records by content hash via OriginId.

use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use serde::{Deserialize, Serialize};

use crate::names::{FieldName, OriginId, PlaceName};

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Origin {
    PlaceRef {
        place: PlaceName,
    },
    PlacePath {
        place: PlaceName,
        fields: Vec<FieldName>,
    },
    /// Borrow originated at any of these places. Origins referenced
    /// by content hash to avoid embedded recursion.
    PlaceUnion {
        origins: Vec<OriginId>,
    },
}
