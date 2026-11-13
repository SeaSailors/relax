// A macro a *macro expansion* makes exists only once the module that makes it is expanded: written
// definitions are prescanned, expansion-made ones are not, and module order decides which comes first.
// bnum 0.12.1 declares `mod bint;` before `mod doc;` and `bint`'s expansion calls `doc::cast_unsigned!`,
// which `link_doc_comment_method!` generates in `doc` (rustc resolves it: a macro path is resolved after
// the whole crate has expanded). Declared in the other order the same code never failed.
pub mod bint {
    use crate::doc;

    macro_rules! ilog {
        ($method: ident) => {
            #[doc = doc::$method!(I)]
            pub fn $method() -> &'static str { "fn body" }
        };
    }

    macro_rules! mod_impl {
        ($BInt: ident) => {
            ilog!(cast_unsigned);
        };
    }

    crate::macro_impl!(mod_impl);
}

pub mod doc {
    macro_rules! doc_comment {
        ($name: ident) => { concat!("doc of ", stringify!($name)) };
    }
    pub(crate) use doc_comment;

    macro_rules! link_doc_comment_method {
        ($($name: ident), *) => {
            $(
                macro_rules! $name {
                    ($sign: ident) => { doc::doc_comment!($sign) };
                }
                pub(crate) use $name;
            )*
        }
    }
    pub(crate) use link_doc_comment_method;

    crate::doc::link_doc_comment_method! { cast_unsigned }
}

macro_rules! macro_impl {
    ($name: ident) => { $name!(BInt); };
}
pub(crate) use macro_impl;

#[test]
fn an_earlier_module_may_call_a_later_modules_generated_macro() {
    assert_eq!(bint::cast_unsigned(), "fn body");
}
