#![allow(unused)]

pub mod parser {
    pub mod adapters {
        pub mod and;
        //pub mod repeat;
        //pub mod conversion;
        pub mod attach;
        pub mod flat_map;
        pub mod ignore;
        pub mod map;
        pub mod map_err;
        pub mod non_terminal;
        pub mod opt;
        pub mod or;
    }
    pub(crate) mod prelude;
    pub mod sources;
    pub mod traits;
    pub(crate) mod util;
}
pub mod data {
    pub mod adapters {
        pub mod effect;
        pub mod option;
        pub mod pure;
        pub mod result;
        pub mod unit;
    }
    pub(crate) mod prelude;
    pub mod traits;
}
pub mod stream {
    pub mod adapters;
    pub mod traits;
}
pub mod prelude {
    // TODO
}
