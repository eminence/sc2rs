extern crate protobuf;

pub mod protos {
    mod common;

    pub use self::common::*;

    mod data;

    pub use self::data::*;

    mod debug;

    pub use self::debug::*;

    mod error;

    pub use self::error::*;

    mod query;

    pub use self::query::*;

    mod raw;

    pub use self::raw::*;

    mod sc2api;

    pub use self::sc2api::*;


    mod spatial;

    pub use self::spatial::*;

    mod score;

    pub use self::score::*;

    mod ui;

    pub use self::ui::*;
}
//pub use protos;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
