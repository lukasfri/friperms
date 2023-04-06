use friperms::{kv_list_set, DifferenceInPlace, KVListWithWildcardSet, UnionInPlace};

#[test]
fn test_add() {
    let tree_1 = kv_list_set! {
      1 => KVListWithWildcardSet {
        rest_list: kv_list_set! {},
        wildcard_exceptions: kv_list_set! {},
        wildcard_value: Box::new(kv_list_set! {
          15 => true,
        })
      }
    };

    let tree_2 = kv_list_set! {
      1 => kv_list_set! {
        5 => kv_list_set! {
          15 => true,
          5 => true,
        },
      },
    };

    let mut tree_1_minus_2 = tree_1.clone();
    tree_1_minus_2.difference_in_place(&tree_2);
    {
        let result = kv_list_set! {
          1 => KVListWithWildcardSet {
            rest_list: kv_list_set! {},
            wildcard_exceptions: kv_list_set! {
              5 => kv_list_set! {
                15 => true,
              },
            },
            wildcard_value: Box::new(kv_list_set! {
              15 => true,
            }),
          }
        };

        assert_eq!(tree_1_minus_2, result);
    }

    tree_1_minus_2.union_in_place(&tree_2);
    //Does not equal tree_1 because 1.5.5 has been added.
    assert_ne!(tree_1, tree_1_minus_2);
}
