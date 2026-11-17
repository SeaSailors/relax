//! Three shapes a Rust 2021 crate writes that rustc accepts and `relax test` refused -- all met in
//! `relax test sandsea-api`, in the dependencies' own type checking.

/// A block `use` alias hides a type parameter of the same name: an item in a body block sits in a
/// rib inner to the item's own generics (rustc_resolve late.rs), so `F` in the body is the struct.
/// polars-core 0.55.2 `chunked_array/ops/arity.rs:582` writes `use StatisticsFlags as F;` beside
/// `F: FnMut(..)` and then `F::IS_SORTED_ANY` (arity.rs:590); relax resolved the *parameter* and
/// reported `E0599: no function or associated item named `IS_SORTED_ANY` found for type parameter
/// `T2``.
pub fn alias_hides_param<T, F: FnMut(u8) -> u8>(_op: F) -> u32 {
    use crate::inner::Flags as F;
    let _ = core::marker::PhantomData::<T>;
    F::IS_SORTED_ANY
}

/// A block `struct` item hides a parameter the same way, and the shadowed name is a type in a
/// written type as well as the head of a type-relative path.
pub fn block_struct_hides_param<T, U: Clone>(u: U) -> u32 {
    struct U;
    impl U {
        const VALUE: u32 = 9;
    }
    let _ = u;
    let _v: U = U;
    U::VALUE
}

/// `expr as &[u8]`: a cast from a reference to a reference is a coercion, and the coercion is what
/// decides a pointee still open in the operand's type. datafusion-functions-aggregate 55.2.0
/// `min_max/min_max_bytes.rs:262` writes `builder.append_value(s.as_ref() as &[u8])` where
/// `s: &Vec<u8>`, `Vec<u8>` has two `AsRef` impls (`AsRef<[u8]>`, `AsRef<Vec<u8>>`) and the cast
/// picks one; without the coercion relax reported `E0283: type annotations needed: cannot satisfy
/// `Vec<u8>: AsRef<_>``.
pub fn cast_decides_as_ref(v: &Vec<u8>) -> &[u8] {
    v.as_ref() as &[u8]
}

/// core 1.70's `default_iters` impls (`impl<I: Default> Default for Cloned<I>` and its six
/// siblings) - polars-plan 0.55.2 `plans/optimizer/sortedness.rs:553` writes
/// `map_or(Default::default(), |x| x.0.iter().cloned())`, whose default value is a
/// `Cloned<slice::Iter<Sorted>>`; relax's std model lacked them and reported `E0277: the trait
/// bound `Cloned<Iter<Sorted>>: Default` is not satisfied`.
pub fn default_of_an_adapter(items: &std::sync::Arc<[u8]>) -> Vec<u8> {
    let none: Option<&std::sync::Arc<[u8]>> = None;
    let same: Option<&std::sync::Arc<[u8]>> = Some(items);
    same.map_or(Default::default(), |x| x.iter().cloned())
        .chain(none.map_or(Default::default(), |x| x.iter().cloned()))
        .chain(std::iter::once(0u8))
        .collect()
}

pub mod inner {
    pub struct Flags;

    impl Flags {
        pub const IS_SORTED_ANY: u32 = 0x03;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn a_block_use_alias_hides_a_type_parameter() {
        let f = |x: u8| x;
        assert_eq!(super::alias_hides_param::<u8, _>(f), 0x03);
    }

    #[test]
    fn a_block_struct_hides_a_type_parameter() {
        assert_eq!(super::block_struct_hides_param::<u8, u8>(1), 9);
    }

    #[test]
    fn a_cast_to_a_slice_decides_a_methods_open_pointee() {
        let v: Vec<u8> = vec![1, 2];
        assert_eq!(super::cast_decides_as_ref(&v), &[1u8, 2][..]);
    }

    #[test]
    fn a_cloned_iterator_defaults_to_an_empty_one() {
        let items: std::sync::Arc<[u8]> = std::sync::Arc::from(vec![7u8, 8]);
        assert_eq!(super::default_of_an_adapter(&items), vec![7u8, 8, 0]);
    }
}
