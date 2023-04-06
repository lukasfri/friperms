# FriPerms

## What is FriPerms?

FriPerms is a framework for creating typed permission models for whatever system you could phatom. The set system that it is based on can technically be used for other things as well.

FriPerms is inspired by Minecraft Bukkits permission system, which are strings separated by dots, like "admin.ban", or "admin.\*" for all admin perms. FriPerms takes a much more generic approach, letting anything be a set that can be unioned, differenced, intersected & more.

```rust
extern crate friperms;
use friperms::{
    kv_list_set, DifferenceInPlace, DisjunctiveUnionInPlace, HasSubset, Intersection,
    IntersectionInPlace, KVListSet, Set, UnionInPlace,
};

#[derive(
    Set,
    UnionInPlace,
    DifferenceInPlace,
    IntersectionInPlace,
    DisjunctiveUnionInPlace,
    PartialEq,
    Clone,
    Debug,
)]
struct UserPerms {
    account_access: bool,
    mod_powers_over: KVListSet<String, bool>,
}

let user_1 = UserPerms {
    account_access: true,
    mod_powers_over: kv_list_set! {
      "duck".to_string() => true,
    },
};

//User has account access
assert!(user_1.has_subset(&UserPerms {
    account_access: true,
    ..Set::empty()
}));

//But user does not have mod powers over "frog".
assert!(!user_1.has_subset(&UserPerms {
    mod_powers_over: kv_list_set! {
      "frog".to_string() => true,
    },
    ..Set::empty()
}));

//User does however have mod powers over "duck".
assert!(user_1.has_subset(&UserPerms {
    mod_powers_over: kv_list_set! {
      "duck".to_string() => true,
    },
    ..Set::empty()
}));
```

## Why does it have to be so complicated with unions and such?

Because a big part of my requirements for a functional permission system were: typed sets, different data types for differing requirements, wildcard permissions (ie for every key, certain values must be true), inheritance/composition and consistence, meaning if you have A, difference it with B, then union it again with A, a should be the exact same. The data-structure must be identical.

This is done by properly defining the mathamatical equivalences as recursively called traits. Since the system is defined in data structs, _serde_ can be used to compactly serialize, store away cached versions of different permission trees, and then combine them however you want.

The implementation for how these should be used it left open to the user, but some [examples](./examples) are provided.
