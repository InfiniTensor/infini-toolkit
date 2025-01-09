#![cfg(infini)]
#![deny(warnings)]

#[macro_use]
#[allow(
    non_camel_case_types,
    clippy::useless_transmute,
    clippy::missing_safety_doc,
    clippy::ptr_offset_with_cast
)]
pub mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

    #[macro_export]
    macro_rules! infiniop {
        ($f:expr) => {{
            #[allow(unused_imports)]
            use $crate::bindings::*;
            #[allow(unused_unsafe, clippy::macro_metavars_in_unsafe)]
            let err = unsafe { $f };
            assert_eq!(err, infiniopStatus_t::STATUS_SUCCESS);
        }};
    }
}

/// 资源的原始形式的表示。通常来自底层库的定义。
pub trait AsRaw {
    /// 原始形式的类型。
    type Raw: Unpin + 'static;
    /// # Safety
    ///
    /// The caller must ensure that the returned item is dropped before the original item.
    unsafe fn as_raw(&self) -> Self::Raw;
}

mod data_layout;
mod descriptor;
mod handle;
mod tensor;

pub use descriptor::Descriptor;
pub use handle::Handle;
pub use tensor::Tensor;
