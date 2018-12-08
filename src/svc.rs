macro_rules! register_service {
    ($trait:ident) => {
        impl $trait for crate::store::Hub {}
    };
}

pub(crate) mod files;
pub(crate) mod tree;
