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

#[derive(Debug, Copy, Clone, ToProtobuf, FromProtobuf)]
pub struct PointI {
    #[Get]
    pub x: i32,
    #[Get]
    pub y: i32,
}

impl PointI {
    pub fn distance_between(&self, other: &PointI) -> f32 {
        // sqrt of the sums of the differences squared
        let dx = ((self.x - other.x) as f32).powi(2);
        let dy = ((self.y - other.y) as f32).powi(2);
        f32::sqrt(dx + dy)
    }
}

#[derive(Debug, Copy, Clone, ToProtobuf, FromProtobuf)]
pub struct RectangleI {
    pub p0: PointI,
    pub p1: PointI,
}

#[derive(Debug, Copy, Clone, ToProtobuf, FromProtobuf)]
/// Point on the game board, 0..222
///
/// Note: bottom left of the screen is 0,0
pub struct Point2D {
    #[Get]
    pub x: f32,
    #[Get]
    pub y: f32,
}

impl Point2D {
    pub fn distance_between(&self, other: &Point2D) -> f32 {
        // sqrt of the sums of the differences squared
        let dx = (self.x - other.x).powi(2);
        let dy = (self.y - other.y).powi(2);
        f32::sqrt(dx + dy)
    }
}

#[derive(Debug, Copy, Clone, ToProtobuf, FromProtobuf)]
pub struct Point {
    #[Get]
    pub x: f32,
    #[Get]
    pub y: f32,
    #[Get]
    pub z: f32,
}

impl Point {
    pub fn distance_between(&self, other: &Point) -> f32 {
        // sqrt of the sums of the differences squared
        let dx = (self.x - other.x).powi(2);
        let dy = (self.y - other.y).powi(2);
        let dz = (self.z - other.z).powi(2);
        f32::sqrt(dx + dy + dz)
    }
    pub fn to_2d(&self) -> Point2D {
        Point2D{ x: self.x, y: self.y }
    }
}

#[derive(Debug, Copy, Clone, ToProtobuf, FromProtobuf)]
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
