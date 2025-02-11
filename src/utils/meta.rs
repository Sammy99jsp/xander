//!
//! Runtime metadata for runtime tracking.
//!

use std::ops::Deref;

pub trait Meta<Table>: Deref<Target = Table> {
    fn meta(&self) -> &'static Table;
}

pub(crate) use macros::meta_trait;

mod macros {
    macro_rules! meta_trait {
        ($macro_ident: ident, $tr: ty, $meta_ty: ty) => {
            macro $macro_ident($st: ident, $meta: expr) {
                #[derive(Debug, Clone, Copy)]
                pub struct $st;

                impl $crate::utils::Meta<$meta_ty> for $st {
                    fn meta(&self) -> &'static $meta_ty {
                        <Self as $tr>::META
                    }
                }

                impl $tr for $st {
                    const META: &'static $meta_ty = &$meta;
                }

                impl std::ops::Deref for $st {
                    type Target = $meta_ty;

                    fn deref(&self) -> &Self::Target {
                        <Self as $tr>::META
                    }
                }
            }
        };
    }

    pub(crate) use meta_trait;
}
