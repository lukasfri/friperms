use std::collections::HashMap;

use finit::comparisons::{SubsetOf, SetEq};
use finit::operations::{
    DifferenceAssign, DisjunctiveUnionAssign, IntersectionAssign, UnionAssign,
};
use finit::{Set};
use maplit::hashmap;

#[derive(
    Debug,
    Clone,
    Set,
    UnionAssign,
    DifferenceAssign,
    IntersectionAssign,
    DisjunctiveUnionAssign,
    SetEq,
    SubsetOf,
    PartialEq,
)]
pub struct ThemingPerms {
    can_have_dark_mode: bool,
    allowed_themes: HashMap<String, bool>,
}

#[derive(
    Debug,
    Clone,
    Set,
    UnionAssign,
    DifferenceAssign,
    IntersectionAssign,
    DisjunctiveUnionAssign,
    SetEq,
    SubsetOf,
    PartialEq,
)]
pub struct ClanPerms {
    kick: bool,
    ban: bool,
    owner: bool,
}

#[derive(
    Debug,
    Clone,
    Set,
    UnionAssign,
    DifferenceAssign,
    IntersectionAssign,
    DisjunctiveUnionAssign,
    SetEq,
    SubsetOf,
    PartialEq,
)]
pub struct UserPerms {
    theming: ThemingPerms,
    clans: HashMap<String, ClanPerms>,
    account_access: bool,
}

impl UserPerms {
    fn is_owner_of_clan(&self, clan_name: String) -> bool {
        let comparer = UserPerms {
            clans: hashmap! {
                clan_name => ClanPerms {
                    owner: true,
                    ..<ClanPerms as Set>::empty()
                }
            },
            ..<UserPerms as Set>::empty()
        };

        comparer.subset_of(self)
    }
}

fn main() {
    use finit::Set;

    let user_perms = UserPerms {
        account_access: true,
        theming: ThemingPerms {
            can_have_dark_mode: false,
            allowed_themes: hashmap! {
                "default_theme".to_string() => true
            },
        },
        clans: hashmap! {
            "redwood".to_string() => ClanPerms {
                owner: true,
                ..<ClanPerms as Set>::empty()
            }
        },
    };

    assert!(user_perms.is_owner_of_clan("redwood".to_string()));

    println!("User is owner of redwood clan");
}
