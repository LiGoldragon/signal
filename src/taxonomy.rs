//! Behavior over the generated cross-component taxonomy.

use crate::generated::{AuthorizedObjectInterest, AuthorizedObjectReference};

/// Tests whether an authorized object is selected by a shared interest.
pub trait InterestMatchable {
    fn matches_interest(&self, interest: &AuthorizedObjectInterest) -> bool;
}

impl InterestMatchable for AuthorizedObjectReference {
    fn matches_interest(&self, interest: &AuthorizedObjectInterest) -> bool {
        match interest {
            AuthorizedObjectInterest::AnyAuthorizedObject => true,
            AuthorizedObjectInterest::Component(component_kind) => {
                self.component_kind == *component_kind
            }
            AuthorizedObjectInterest::ObjectKind(authorized_object_kind) => {
                self.authorized_object_kind == *authorized_object_kind
            }
            AuthorizedObjectInterest::ComponentObject(component_object_interest) => {
                self.component_kind == component_object_interest.component_kind
                    && self.authorized_object_kind
                        == component_object_interest.authorized_object_kind
            }
        }
    }
}
