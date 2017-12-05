

use sc2_protobuf::protos;
use failure;
use super::{ToProtobuf, FromProtobuf};

use super::common::*;




#[derive(Debug, FromProtobuf)]
pub struct StartRaw {
    pub map_size: Size2DI,
    pub pathing_grid: ImageData,
    pub terrain_height: ImageData,
    pub placement_grid: ImageData,
    pub playable_area: RectangleI,
    pub start_locations: Vec<Point2D>,
}



#[derive(Debug, FromProtobuf)]
pub struct ObservationRaw {
    pub player: PlayerRaw,
    pub units: Vec<Unit>,
    /// Fog of war, creep and so on. Board stuff that changes per frame
    pub map_state: MapState,
    pub event: Event,
    pub effects: Vec<Effect>,
}


#[derive(Debug, FromProtobuf)]
pub struct PowerSource {
    pub pos: Point,
    #[Get]
    pub radius: f32,
    #[Get]
    pub tag: u64,
}


#[derive(Debug, FromProtobuf)]
pub struct PlayerRaw {
    pub power_sources: Vec<PowerSource>,
    pub camera: Point,
    pub upgrade_ids: Vec<u32>,
}

#[derive(Debug, FromProtobuf)]
#[AttachedTo(UnitOrder)]
pub enum UnitOrderTarget {
    TargetWorldSpacePos(Point),
    #[Get]
    TargetUnitTag(u64),
}

#[derive(Debug, FromProtobuf)]
pub struct UnitOrder {
    #[Get]
    pub ability_id: u32,
    #[OneOf]
    pub target: Option<UnitOrderTarget>,
    /// Progress of train abilities.  Range 0.0 to 1.0
    #[Get]
    pub progress: f32,
}



#[derive(Clone, PartialEq, Eq, Debug, Hash, FromProtobuf)]
pub enum DisplayType {
    Visible = 1,
    Snapshot = 2,
    Hidden = 3,
}



#[derive(Clone, PartialEq, Eq, Debug, Hash, FromProtobuf)]
pub enum Alliance {
    Selff = 1,
    Ally = 2,
    Neutral = 3,
    Enemy = 4,
}

#[derive(Clone, PartialEq, Eq, Debug, Hash, FromProtobuf)]
pub enum CloakState {
    Cloaked = 1,
    CloakedDetected = 2,
    NotCloaked = 3,
}


#[derive(Debug, FromProtobuf)]
pub struct PassengerUnit {
    #[Get]
    pub tag: u64,
    #[Get]
    pub health: f32,
    #[Get]
    pub health_max: f32,
    #[Get]
    pub shield: f32,
    #[Get]
    pub shield_max: f32,
    #[Get]
    pub energy: f32,
    #[Get]
    pub energy_max: f32,
    #[Get]
    pub unit_type: u32,
}


#[derive(Debug, FromProtobuf)]
pub struct Unit {
    #[Get]
    pub display_type: DisplayType,
    #[Get]
    pub alliance: Alliance,
    /// Unique identifier for a unix
    #[Get]
    pub tag: u64,
    #[Get]
    pub unit_type: u32,
    #[Get]
    pub owner: i32,

    pub pos: Point,
    #[Get]
    pub facing: f32,
    #[Get]
    pub radius: f32,
    /// Range 0.0 to 1.0
    #[Get]
    pub build_progress: f32,
    #[Get]
    pub cloak: CloakState,

    #[Get]
    pub detect_range: f32,
    #[Get]
    pub radar_range: f32,

    #[Get]
    pub is_selected: bool,
    /// Visible and within the camera frustrum
    #[Get]
    pub is_on_screen: bool,
    /// Detected by sensor tower
    #[Get]
    pub is_blip: bool,
    #[Get]
    pub is_powered: bool,

    // Not populated for snapshots:
    #[Get]
    pub health: Option<f32>,
    #[Get]
    pub health_max: Option<f32>,
    #[Get]
    pub shield: Option<f32>,
    #[Get]
    pub shield_max: Option<f32>,
    #[Get]
    pub energy: Option<f32>,
    #[Get]
    pub energy_max: Option<f32>,
    #[Get]
    pub mineral_contents: Option<i32>,
    #[Get]
    pub vespene_contents: Option<i32>,
    #[Get]
    pub is_flying: Option<bool>,
    #[Get]
    pub is_burrowed: Option<bool>,

    // Not populated for enemies:
    pub orders: Vec<UnitOrder>,
    #[Get]
    pub add_on_tag: Option<u64>,
    pub passengers: Vec<PassengerUnit>,
    #[Get]
    pub cargo_space_taken: Option<i32>,
    #[Get]
    pub cargo_space_max: Option<i32>,
    pub buff_ids: Vec<u32>,
    #[Get]
    pub assigned_harvesters: Option<i32>,
    #[Get]
    pub ideal_harvesters: Option<i32>,
    #[Get]
    pub weapon_cooldown: Option<f32>,
    #[Get]
    pub engaged_target_tag: Option<u64>,
}




#[derive(Debug, FromProtobuf)]
pub struct MapState {
    /// 1 byte visibility layer
    pub visibility: ImageData,
    /// 1 byte creep layer
    pub creep: ImageData,
}




#[derive(Debug, FromProtobuf)]
pub struct Event {
    pub dead_units: Vec<u64>,
}




#[derive(Debug, FromProtobuf)]
pub struct Effect {
    #[Get]
    pub effect_id: u32,
    /// Effect may impact multiple locations.
    ///
    /// For example: Lurker attack
    pub pos: Vec<Point2D>,
}



#[derive(Debug, ToProtobuf, FromProtobuf)]
pub enum ActionRaw {
    UnitCommand(ActionRawUnitCommand),
    CameraMove(ActionRawCameraMove),
    ToggleAutocast(ActionRawToggleAutocast),
}



#[derive(Debug, ToProtobuf, FromProtobuf)]
#[AttachedTo(ActionRawUnitCommand)]
pub enum ActionRawUnitCommandTargetEnum {
    TargetWorldSpacePos(Point2D),
    #[Get]
    TargetUnitTag(u64),
}


#[derive(Debug, ToProtobuf, FromProtobuf)]
pub struct ActionRawUnitCommand {
    #[Get]
    pub ability_id: i32,
    #[OneOf]
    pub target: Option<ActionRawUnitCommandTargetEnum>,
    #[Set]
    pub unit_tags: Vec<u64>,
    #[Get]
    pub queue_command: bool,
}


#[derive(Debug, ToProtobuf, FromProtobuf)]
pub struct ActionRawCameraMove {
    pub center_world_space: Point,
}


#[derive(Debug, ToProtobuf, FromProtobuf)]
pub struct ActionRawToggleAutocast {
    #[Get]
    pub ability_id: i32,
    #[Set]
    pub unit_tags: Vec<u64>,
}
