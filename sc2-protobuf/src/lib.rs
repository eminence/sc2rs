extern crate protobuf;

pub mod protos {
    mod common;
    mod data;
    mod debug;
    mod error;
    mod query;
    mod raw;
    mod sc2api;
    mod spatial;
    mod score;
    mod ui;

    pub use self::common::*;
    pub use self::data::*;
    pub use self::debug::*;
    pub use self::error::*;
    pub use self::query::*;
    pub use self::raw::*;
    pub use self::sc2api::*;
    pub use self::spatial::*;
    pub use self::score::*;
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
