// Crate-internal.
// ---

pub(crate) mod data {
    pub(crate) mod datasources {
        pub(crate) mod cache_datasource;
        pub(crate) mod open_exchange_rates_datasource;
    }
    pub(crate) mod models {
        pub(crate) mod open_exchange_rates_model;
    }
    pub(crate) mod repositories {
        pub(crate) mod fx_repository_impl;
    }
}

pub(crate) mod domain {
    pub(crate) mod entities {
        pub(crate) mod rate_table;
    }
    pub(crate) mod repositories {
        pub(crate) mod fx_repository;
    }
}

// Public exports.
// ---

#[doc(hidden)]
#[allow(unused_imports)]
pub mod exports {
    // This mod represents how clients see the library, and can differ from the
    // internal structure.
    //
    // The contents of this mod are re-exported in the root of the crate.

    pub mod entities {
        pub use crate::domain::entities::rate_table::*;
    }
}
