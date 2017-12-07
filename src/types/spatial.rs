use sc2_protobuf::protos;
use failure;
use super::{ToProtobuf, FromProtobuf};

use super::common::*;

#[derive(Debug, FromProtobuf)]
pub struct ObservationFeatureLayer {
    pub renders: Option<FeatureLayers>,
    pub minimap_renders: Option<FeatureLayersMinimap>,
}

#[derive(Debug, FromProtobuf)]
pub struct FeatureLayers {
    pub height_map: ImageData,
    pub visibility_map: ImageData,
    pub creep: ImageData,
    pub power: ImageData,
    pub player_id: ImageData,
    pub unit_type: ImageData,
    pub selected: ImageData,
    pub unit_hit_points: ImageData,
    pub unit_hit_points_ratio: ImageData,
    pub unit_energy: ImageData,
    pub unit_energy_ratio: ImageData,
    pub unit_shields: ImageData,
    pub unit_shields_ratio: ImageData,
    pub player_relative: ImageData,
    pub unit_density_aa: ImageData,
    pub unit_density: ImageData,
    pub effects: ImageData,
}


#[derive(Debug, FromProtobuf)]
pub struct FeatureLayersMinimap {
    pub height_map: ImageData,
    pub visibility_map: ImageData,
    pub creep: ImageData,
    pub camera: ImageData,
    pub player_id: ImageData,
    pub player_relative: ImageData,
    pub selected: ImageData,
    pub unit_type: Option<ImageData>,
}

#[derive(Debug, FromProtobuf)]
pub struct ObservationRender {
    pub map: ImageData,
    pub minimap: ImageData,
}

#[derive(Debug, ToProtobuf, FromProtobuf)]
pub enum ActionSpatial {
    UnitCommand(ActionSpatialUnitCommand),
    CameraMove(ActionSpatialCameraMove),
    UnitSelectionPoint(ActionSpatialUnitSelectionPoint),
    UnitSelectionRect(ActionSpatialUnitSelectionRect),
}

#[derive(Debug, FromProtobuf, ToProtobuf)]
#[AttachedTo(ActionSpatialUnitCommand)]
pub enum ActionSpatialUnitCommandEnum {
    TargetScreenCoord(PointI),
    TargetMinimapCoord(PointI),
}

#[derive(Debug, FromProtobuf, ToProtobuf)]
pub struct ActionSpatialUnitCommand {
    #[Get]
    pub ability_id: i32,
    #[OneOf]
    pub target: ActionSpatialUnitCommandEnum,
    #[Get]
    pub queue_command: bool,
}

#[derive(Debug, FromProtobuf, ToProtobuf)]
pub struct ActionSpatialCameraMove {
    pub center_minimap: PointI,
}

#[derive(Debug, ToProtobuf, FromProtobuf)]
#[allow(non_camel_case_types)]
pub enum ActionSpatialUnitSelectionPoint_Type {
    Select = 1,
    Toggle = 2,
    AllType = 3,
    AddAllType = 4,
}

#[derive(Debug, ToProtobuf, FromProtobuf)]
pub struct ActionSpatialUnitSelectionPoint {
    pub selection_screen_coord: PointI,
    #[Get]
    pub field_type: ActionSpatialUnitSelectionPoint_Type,
}

#[derive(Debug, ToProtobuf, FromProtobuf)]
pub struct ActionSpatialUnitSelectionRect {
    pub selection_screen_coord: Vec<RectangleI>,
    #[Get]
    pub selection_add: bool,
}
