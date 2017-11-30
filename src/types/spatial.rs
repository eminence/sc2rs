
use sc2_protobuf::protos;
use failure;
use super::{ToProtobuf, FromProtobuf};

use super::common::*;

#[derive(Debug, FromProtobuf)]
pub struct ObservationFeatureLayer {
    renders: Option<FeatureLayers>,
    minimap_renders: Option<FeatureLayersMinimap>
}
#[derive(Debug, FromProtobuf)]
pub struct FeatureLayers {
    height_map: ImageData,
    visibility_map: ImageData,
    creep: ImageData,
    power: ImageData,
    player_id: ImageData,
    unit_type: ImageData,
    selected: ImageData,
    unit_hit_points: ImageData,
    unit_hit_points_ratio: ImageData,
    unit_energy: ImageData,
    unit_energy_ratio: ImageData,
    unit_shields: ImageData,
    unit_shields_ratio: ImageData,
    player_relative: ImageData,
    unit_density_aa: ImageData,
    unit_density: ImageData,
    effects: ImageData
}


#[derive(Debug, FromProtobuf)]
pub struct FeatureLayersMinimap {
    height_map: ImageData,
    visibility_map: ImageData,
    creep: ImageData,
    camera: ImageData,
    player_id: ImageData,
    player_relative: ImageData,
    selected: ImageData,
    unit_type: Option<ImageData>
}

#[derive(Debug, FromProtobuf)]
pub struct ObservationRender {
    map: ImageData,
    minimap: ImageData
}

#[derive(Debug, ToProtobuf, FromProtobuf)]
pub enum ActionSpatial {
    UnitCommand(ActionSpatialUnitCommand),
    CameraMove(ActionSpatialCameraMove),
    UnitSelectionPoint(ActionSpatialUnitSelectionPoint),
    UnitSelectionRect(ActionSpatialUnitSelectionRect)
}

#[derive(Debug, FromProtobuf, ToProtobuf)]
#[AttachedTo(ActionSpatialUnitCommand)]
pub enum ActionSpatialUnitCommandEnum {
    TargetScreenCoord(PointI),
    TargetMinimapCoord(PointI)
}

#[derive(Debug, FromProtobuf, ToProtobuf)]
pub struct ActionSpatialUnitCommand {
    #[Get] ability_id: i32,
    #[OneOf] target: ActionSpatialUnitCommandEnum,
    #[Get] queue_command: bool
}

#[derive(Debug, FromProtobuf, ToProtobuf)]
pub struct ActionSpatialCameraMove {
    center_minimap: PointI
}

#[derive(Debug, ToProtobuf, FromProtobuf)]
pub enum ActionSpatialUnitSelectionPoint_Type {
    Select = 1,
    Toggle = 2,
    AllType = 3,
    AddAllType = 4
}

#[derive(Debug, ToProtobuf, FromProtobuf)]
pub struct ActionSpatialUnitSelectionPoint {
    selection_screen_coord: PointI,
    #[Get] field_type: ActionSpatialUnitSelectionPoint_Type,

}

#[derive(Debug, ToProtobuf, FromProtobuf)]
pub struct ActionSpatialUnitSelectionRect {
    selection_screen_coord: Vec<RectangleI>,
    #[Get] selection_add: bool
}