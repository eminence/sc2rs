

use sc2_protobuf::protos;
use failure;
use super::{ToProtobuf, FromProtobuf};




#[derive(Debug, FromProtobuf)]
pub struct AvailableAbility {
    #[Get]
    pub ability_id: i32,
    #[Get]
    pub requires_point: bool,
}


#[derive(FromProtobuf)]
pub struct ImageData {
    #[Get]
    pub bits_per_pixel: i32,
    pub size: Size2DI,
    pub data: Vec<u8>,
}
// custom derive for Debug, so we don't have to show all the data
impl ::std::fmt::Debug for ImageData {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        write!(f, "<ImageData {:?} {} bpp>", self.size, self.bits_per_pixel)
    }
}
#[derive(Debug, ToProtobuf, FromProtobuf)]
pub struct PointI {
    #[Get]
    pub x: i32,
    #[Get]
    pub y: i32,
}

#[derive(Debug, ToProtobuf, FromProtobuf)]
pub struct RectangleI {
    pub p0: PointI,
    pub p1: PointI,
}

#[derive(Debug, ToProtobuf, FromProtobuf)]
/// Point on the game board, 0..222
///
/// Note: bottom left of the screen is 0,0
pub struct Point2D {
    #[Get]
    pub x: f32,
    #[Get]
    pub y: f32,
}



#[derive(Debug, Clone, ToProtobuf, FromProtobuf)]
pub struct Point {
    #[Get]
    pub x: f32,
    #[Get]
    pub y: f32,
    #[Get]
    pub z: f32,
}


#[derive(Debug, ToProtobuf, FromProtobuf)]
pub struct Size2DI {
    #[Get]
    pub x: i32,
    #[Get]
    pub y: i32,
}


#[derive(Clone, PartialEq, Eq, Debug, Hash, ToProtobuf, FromProtobuf, Serialize, Deserialize)]
pub enum Race {
    NoRace = 0,
    Terran = 1,
    Zerg = 2,
    Protoss = 3,
    Random = 4,
}
