use crate::{Set, operations::*};

macro_rules! impl_set_tuple {
    ($($ty:ident),*) => {
        impl<$($ty: Set),*> Set for ($($ty,)*) {
            type Empty = ($($ty::Empty,)*);

            fn is_empty(&self) -> bool {
                #[allow(non_snake_case)]
                let ($($ty,)+) = self;

                $($ty.is_empty())&&*
            }

            fn empty() -> Self::Empty {
                ($($ty::empty(),)*)
            }
        }
    };
}

macro_rules! impl_op_tuple {
    ($op:ident, $func_name:ident, $(($ty:ident, $rest_ty:ident)),*) => {
        impl<$($ty: $op<$rest_ty>, $rest_ty),*> $op<($($rest_ty,)*)> for ($($ty,)*) {
            type Output = ($($ty::Output,)*);

            fn $func_name(self, other: ($($rest_ty,)*)) -> Self::Output {
                #[allow(non_snake_case)]
                let ($($ty,)+) = self;
                #[allow(non_snake_case)]
                let ($($rest_ty,)+) = other;


                ($($ty.$func_name($rest_ty),)*)
            }
        }
    };
}

macro_rules! impl_op_assign_tuple {
    ($op:ident, $func_name:ident, $(($ty:ident, $rest_ty:ident)),+) => {
        impl<$($ty: $op<$rest_ty>, $rest_ty),*> $op<($($rest_ty,)*)> for ($($ty,)*) {
            fn $func_name(&mut self, other: ($($rest_ty,)*)) {
                #[allow(non_snake_case)]
                let ($($ty,)+) = self;
                #[allow(non_snake_case)]
                let ($($rest_ty,)+) = other;

                $($ty.$func_name($rest_ty);)*
            }
        }
    };
}
macro_rules! impl_op_ref_tuple {
    ($op:ident, $func_name:ident, $(($ty:ident, $rest_ty:ident)),*) => {
        impl<'a, $($ty: $op<&'a $rest_ty>, $rest_ty),*> $op<&'a ($($rest_ty,)*)> for ($($ty,)*) {
            type Output = ($(<$ty as $op<&'a $rest_ty>>::Output,)*);

            fn $func_name(self, other: &'a ($($rest_ty,)*)) -> Self::Output {
                #[allow(non_snake_case)]
                let ($($ty,)+) = self;
                #[allow(non_snake_case)]
                let ($($rest_ty,)+) = other;


                ($($ty.$func_name($rest_ty),)*)
            }
        }
    };
}

macro_rules! impl_op_assign_ref_tuple {
    ($op:ident, $func_name:ident, $(($ty:ident, $rest_ty:ident)),+) => {
        impl<'a, $($ty: $op<&'a $rest_ty>, $rest_ty),*> $op<&'a ($($rest_ty,)*)> for ($($ty,)*) {
            fn $func_name(&mut self, other: &'a ($($rest_ty,)*)) {
                #[allow(non_snake_case)]
                let ($($ty,)+) = self;
                #[allow(non_snake_case)]
                let ($($rest_ty,)+) = other;

                $($ty.$func_name($rest_ty);)*
            }
        }
    };
}

macro_rules! impl_subset_of_tuple {
    ($(($ty:ident, $rest_ty:ident)),+) => {
        impl<$($ty: $crate::comparisons::SubsetOf<$rest_ty>, $rest_ty),*> $crate::comparisons::SubsetOf<($($rest_ty,)*)> for ($($ty,)*) {
            fn subset_of(&self, other: &($($rest_ty,)*)) -> bool {
                #[allow(non_snake_case)]
                let ($($ty,)+) = self;
                #[allow(non_snake_case)]
                let ($($rest_ty,)+) = other;

                $($ty.subset_of($rest_ty))&&*
            }
        }
    };
}

macro_rules! impl_tuples {
    (($first:ident, $first_rest:ident) $(, ($rest:ident, $rest_rest:ident))*) => {
        impl_set_tuple!($first $(, $rest)*);
        impl_op_tuple!(Union, union, ($first, $first_rest) $(, ($rest, $rest_rest))*);
        impl_op_assign_tuple!(UnionAssign, union_assign, ($first, $first_rest) $(, ($rest, $rest_rest))*);
        impl_op_tuple!(Intersection, intersection, ($first, $first_rest) $(, ($rest, $rest_rest))*);
        impl_op_assign_tuple!(IntersectionAssign, intersection_assign, ($first, $first_rest) $(, ($rest, $rest_rest))*);
        impl_op_tuple!(Difference, difference, ($first, $first_rest) $(, ($rest, $rest_rest))*);
        impl_op_assign_tuple!(DifferenceAssign, difference_assign, ($first, $first_rest) $(, ($rest, $rest_rest))*);
        impl_op_tuple!(DisjunctiveUnion, disjunctive_union, ($first, $first_rest) $(, ($rest, $rest_rest))*);
        impl_op_assign_tuple!(DisjunctiveUnionAssign, disjunctive_union_assign, ($first, $first_rest) $(, ($rest, $rest_rest))*);
        impl_op_ref_tuple!(Union, union, ($first, $first_rest) $(, ($rest, $rest_rest))*);
        impl_op_assign_ref_tuple!(UnionAssign, union_assign, ($first, $first_rest) $(, ($rest, $rest_rest))*);
        impl_op_ref_tuple!(Intersection, intersection, ($first, $first_rest) $(, ($rest, $rest_rest))*);
        impl_op_assign_ref_tuple!(IntersectionAssign, intersection_assign, ($first, $first_rest) $(, ($rest, $rest_rest))*);
        impl_op_ref_tuple!(Difference, difference, ($first, $first_rest) $(, ($rest, $rest_rest))*);
        impl_op_assign_ref_tuple!(DifferenceAssign, difference_assign, ($first, $first_rest) $(, ($rest, $rest_rest))*);
        impl_op_ref_tuple!(DisjunctiveUnion, disjunctive_union, ($first, $first_rest) $(, ($rest, $rest_rest))*);
        impl_op_assign_ref_tuple!(DisjunctiveUnionAssign, disjunctive_union_assign, ($first, $first_rest) $(, ($rest, $rest_rest))*);
        impl_subset_of_tuple!(($first, $first_rest) $(, ($rest, $rest_rest))*);
        impl_tuples!($(($rest, $rest_rest)),*);
    };
    () => {};
}

impl_tuples!(
    (T1, R1),
    (T2, R2),
    (T3, R3),
    (T4, R4),
    (T5, R5),
    (T6, R6),
    (T7, R7),
    (T8, R8),
    (T9, R9),
    (T10, R10),
    (T11, R11),
    (T12, R12),
    (T13, R13),
    (T14, R14),
    (T15, R15),
    (T16, R16)
);

#[cfg(test)]
mod tests {
    use crate::operations::{Difference, Intersection, Union};
    use core::fmt::Debug;
    use rstest::rstest;

    #[rstest]
    #[case((true, false, false, true), (false, true, false, true), (true, true, false, true))]
    fn union_list_tests<T, U, V>(#[case] a: T, #[case] b: U, #[case] c: V)
    where
        T: Union<U, Output = V>,
        V: PartialEq + Debug,
    {
        assert_eq!(a.union(b), c);
    }

    #[rstest]
    #[case((true, false, false, true), (false, true, false, true), (true, false, false, false))]
    fn difference_list_tests<T, U, V>(#[case] a: T, #[case] b: U, #[case] c: V)
    where
        T: Difference<U, Output = V>,
        V: PartialEq + Debug,
    {
        assert_eq!(a.difference(b), c);
    }

    #[rstest]
    #[case((true, false, false, true), (false, true, false, true), (false, false, false, true))]
    fn intersection_list_tests<T, U, V>(#[case] a: T, #[case] b: U, #[case] c: V)
    where
        T: Intersection<U, Output = V>,
        V: PartialEq + Debug,
    {
        assert_eq!(a.intersection(b), c);
    }
}
