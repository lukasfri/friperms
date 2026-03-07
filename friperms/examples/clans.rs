use std::collections::HashMap;

use friperms::{kv_list_set, Set, SubsetOf};
use friperms_derive::{
    DifferenceAssign, DisjunctiveUnionAssign, IntersectionAssign, Set, UnionAssign,
};

#[derive(
    Debug,
    Clone,
    Set,
    UnionAssign,
    DifferenceAssign,
    IntersectionAssign,
    DisjunctiveUnionAssign,
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
            clans: kv_list_set! {
                clan_name => ClanPerms {
                    owner: true,
                    ..Set::empty()
                }
            },
            ..Set::empty()
        };

        comparer.subset_of(self)
    }
}

fn main() {
    use friperms::{kv_list_set, Set};

    let user_perms = UserPerms {
        account_access: true,
        theming: ThemingPerms {
            can_have_dark_mode: false,
            allowed_themes: kv_list_set! {
                "default_theme".to_string() => true
            },
        },
        clans: kv_list_set! {
            "redwood".to_string() => ClanPerms {
                owner: true,
                ..Set::empty()
            }
        },
    };

    assert!(user_perms.is_owner_of_clan("redwood".to_string()));

    println!("User is owner of redwood clan");
}
