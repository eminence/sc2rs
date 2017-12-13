use sc2_protobuf::protos;
use failure;
use super::{ToProtobuf, FromProtobuf};

use super::common::*;
use super::error::*;

#[derive(Debug, ToProtobuf)]
/// Return type [ResponseQuery](struct.ResponseQuery.html).
pub struct RequestQuery {
    pub pathing: Vec<RequestQueryPathing>,
    pub abilities: Vec<RequestQueryAvailableAbilities>,
    pub placements: Vec<RequestQueryBuildingPlacement>,
    /// Ignores requirements like food, minerals and so on.
    pub ignore_resource_requirements: bool,
}

#[derive(Debug, FromProtobuf)]
pub struct ResponseQuery {
    pub pathing: Vec<ResponseQueryPathing>,
    pub abilities: Vec<ResponseQueryAvailableAbilities>,
    pub placements: Vec<ResponseQueryBuildingPlacement>,
}

#[derive(Debug, ToProtobuf)]
#[AttachedTo(RequestQueryPathing)]
#[allow(non_camel_case_types)]
pub enum RequestQueryPathing_start {
    StartPos(Point2D),
    #[Set]
    UnitTag(u64),
}

#[derive(Debug, ToProtobuf)]
pub struct RequestQueryPathing {
    #[OneOf]
    pub start: RequestQueryPathing_start,
    pub end_pos: Point2D,
}

#[derive(Debug, FromProtobuf)]
pub struct ResponseQueryPathing {
    /// 0 if no path exists
    #[Get]
    pub distance: f32,
}

#[derive(Debug, ToProtobuf)]
pub struct RequestQueryAvailableAbilities {
    pub unit_tag: u64,
}

#[derive(Debug, FromProtobuf)]
pub struct ResponseQueryAvailableAbilities {
    pub abilities: Vec<AvailableAbility>,
    #[Get]
    pub unit_tag: u64,
    #[Get]
    pub unit_type_id: u32,
}

#[derive(Debug, ToProtobuf)]
pub struct RequestQueryBuildingPlacement {
    pub ability_id: i32,
    pub target_pos: Point2D,

    pub placing_unit_tag: Option<u64>,
}

#[derive(Debug, FromProtobuf)]
pub struct ResponseQueryBuildingPlacement {
    #[Get]
    pub result: ActionResult,
}
