#![allow(dead_code, unused_imports, unused_variables, non_snake_case, unused_must_use)]

use super::failure;
use super::protobuf::repeated::RepeatedField;

use super::sc2_protobuf::protos;

pub trait RequestMessage<T, U>: ToProtobuf<T>
    where
        Self::Reply: FromProtobuf<U>,
{
    type Reply;
}

pub trait FromProtobuf<T>: Sized {
    fn from_protobuf(t: T) -> Result<Self, failure::Error>;
}

macro_rules! ProtoSelf {
    ($t:ty) => {
        impl ToProtobuf < $t > for $t { fn into_protobuf(self) -> $t { self }}
        impl FromProtobuf < $t > for $t { fn from_protobuf(t:$t) -> Result<Self, failure::Error> { Ok(t) }}
    };
}

ProtoSelf!(String);
ProtoSelf!(bool);
ProtoSelf!(u64);
ProtoSelf!(u32);
ProtoSelf!(u8);
ProtoSelf!(i32);
ProtoSelf!(f32);


pub trait ToProtobuf<T> {
    fn into_protobuf(self) -> T;
}

impl<T, U> ToProtobuf<RepeatedField<T>> for Vec<U>
    where
        U: ToProtobuf<T>,
{
    fn into_protobuf(self) -> RepeatedField<T> {
        let newv = self.into_iter().map(|e| e.into_protobuf()).collect();
        RepeatedField::from_vec(newv)
    }
}

impl<T, U> FromProtobuf<Vec<T>> for Vec<U>
    where
        U: FromProtobuf<T>,
{
    fn from_protobuf(t: Vec<T>) -> Result<Self, failure::Error> {
        Ok(
            t.into_iter()
                .map(|e| FromProtobuf::from_protobuf(e).unwrap())
                .collect(),
        )
    }
}

impl<T, U> FromProtobuf<RepeatedField<T>> for Vec<U>
    where
        U: FromProtobuf<T>,
{
    fn from_protobuf(t: RepeatedField<T>) -> Result<Self, failure::Error> {
        Ok(
            t.into_vec()
                .into_iter()
                .map(|e| FromProtobuf::from_protobuf(e).unwrap())
                .collect(),
        )
    }
}

#[derive(Debug, ToProtobuf, FromProtobuf)]
//#[ProtoType = "LocalMap"]
pub struct LocalMap {
    pub MapPath: String,
}


#[derive(Debug, ToProtobuf, FromProtobuf)]
pub struct PlayerSetup {
    #[Get]
    pub field_type: protos::PlayerType,
    #[Get]
    pub race: protos::Race,
    #[Get]
    pub difficulty: protos::Difficulty,
}

pub use self::protos::{PlayerType, Race, Difficulty};

#[derive(Debug, ToProtobuf, FromProtobuf)]
#[AttachedTo(RequestCreateGame)]
pub enum RequestMap {
    /// Local .SC2Map file
    LocalMap(LocalMap),
    /// Map published to BattleNet
    BattlenetMapName(String),
}

#[derive(Debug, ToProtobuf, FromProtobuf)]
pub struct RequestCreateGame {
    #[OneOf]
    pub map: RequestMap,
    pub player_setup: Vec<PlayerSetup>,
    #[Get]
    pub disable_fog: bool,
    /// Sets the pseudo-random seed for the game
    #[Get]
    pub random_seed: Option<u32>,
    /// If set, the game plays in real time
    #[Get]
    pub realtime: bool,
}


#[derive(Debug, ToProtobuf, FromProtobuf)]
#[AttachedTo(RequestJoinGame)]
pub enum Participation {
    #[Get]
    Race(protos::Race),
    #[Get]
    ObservedPlayerId(u32),
}

#[derive(Debug, ToProtobuf, FromProtobuf)]
pub struct InterfaceOptions {
    #[Get]
    /// Enable the Raw interface?
    pub raw: bool,
    #[Get]
    /// Enable the Score interface?
    pub score: bool,
    /// Enable the Feature Layer interface?
    ///
    /// Set to `None` to disable.
    pub feature_layer: Option<SpatialCameraSetup>,
    // TODO render
}

#[derive(Debug, ToProtobuf, FromProtobuf)]
pub struct SpatialCameraSetup {
    #[Get]
    pub width: f32,
    pub resolution: Size2DI,
    pub minimap_resolution: Size2DI,
}


#[derive(Debug, ToProtobuf, FromProtobuf)]
pub struct RequestJoinGame {
    #[OneOf]
    pub participation: Participation,
    pub options: InterfaceOptions,
    // TODO server_ports
    // TODO client_ports
    // TODO shared_port
}

#[derive(Debug, ToProtobuf)]
pub struct RequestAvailableMaps {}

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
pub struct Size2DI {
    #[Get]
    pub x: i32,
    #[Get]
    pub y: i32,
}

#[derive(Debug, FromProtobuf)]
pub struct PointI {
    #[Get]
    pub x: i32,
    #[Get]
    pub y: i32,
}

#[derive(Debug, FromProtobuf)]
/// Point on the game board, 0..222
///
/// Note: bottom left of the screen is 0,0
pub struct Point2D {
    #[Get]
    pub x: f32,
    #[Get]
    pub y: f32,
}

#[derive(Debug, FromProtobuf)]
pub struct Point {
    #[Get]
    pub x: f32,
    #[Get]
    pub y: f32,
    #[Get]
    pub z: f32,
}

#[derive(Debug, FromProtobuf)]
pub struct RectangleI {
    pub p0: PointI,
    pub p1: PointI,
}

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
pub struct PlayerInfo {
    #[Get]
    pub player_id: u32,
    #[Get]
    pub field_type: protos::PlayerType,
    #[Get]
    pub race_requested: protos::Race,
    /// Only populated for your player or when watching replay
    #[Get]
    pub race_actual: Option<protos::Race>,
    #[Get]
    pub difficulty: protos::Difficulty,
}

#[derive(Debug, ToProtobuf)]
pub struct RequestGameInfo {}

#[derive(Debug, FromProtobuf)]
pub struct ResponseGameInfo {
    pub map_name: String,
    pub mod_names: Vec<String>,
    pub local_map_path: String,
    pub player_info: Vec<PlayerInfo>,

    /// Populated if Raw interface is enabled
    pub start_raw: Option<StartRaw>,
    pub options: Option<InterfaceOptions>,
}

#[derive(Debug, ToProtobuf)]
pub struct RequestObservation {
    pub disable_fog: bool,
}


#[derive(Debug, ToProtobuf, FromProtobuf)]
pub struct RequestData {
    #[Get]
    pub ability_id: bool,
    #[Get]
    pub unit_type_id: bool,
    #[Get]
    pub upgrade_id: bool,
    #[Get]
    pub buff_id: bool,
    #[Get]
    pub effect_id: bool,
}

#[derive(Debug, FromProtobuf)]
pub struct ResponseData {
    pub abilities: Vec<AbilityData>,
    pub units: Vec<UnitTypeData>,
    pub upgrades: Vec<UpgradeData>,
    pub buffs: Vec<BuffData>,
    pub effects: Vec<EffectData>,
}

#[derive(Debug, FromProtobuf)]
pub struct AbilityData {
    /// Stable ID
    #[Get]
    pub ability_id: u32,

    /// Catalog name of the ability
    pub link_name: String,

    /// Catalog index of the ability
    #[Get]
    pub link_index: u32,

    /// Name used for the command card
    pub button_name: String,

    /// A human friendly name when the button name or link name isn't descriptive
    pub friendly_name: Option<String>,

    /// Hotkey
    pub hotkey: Option<String>,

    /// The ability id may be represented by the given more generic id
    #[Get]
    pub remaps_to_ability_id: u32,

    #[Get]
    pub target: protos::AbilityData_Target,

    /// If true, the ability may be used on this set of mods/map
    #[Get]
    pub available: bool,

    #[Get]
    pub allow_minimap: bool,
    #[Get]
    pub allow_autocast: bool,
    #[Get]
    pub is_building: bool,
    #[Get]
    pub footprint_radius: f32,
    #[Get]
    pub is_instant_placement: bool,
    #[Get]
    pub cast_range: f32,
}

#[derive(Debug, FromProtobuf)]
pub struct UnitTypeData {
    #[Get]
    pub unit_id: u32,
    pub name: String,
    #[Get]
    pub available: bool,
    #[Get]
    pub cargo_size: u32,
    #[Get]
    pub mineral_cost: u32,
    #[Get]
    pub vespene_cost: u32,
    #[Get]
    pub food_required: f32,
    #[Get]
    pub food_provided: f32,
    /// The ability that builds this unit
    #[Get]
    pub ability_id: u32,
    #[Get]
    pub race: protos::Race,
    #[Get]
    pub build_time: f32,
    #[Get]
    pub has_vespene: bool,
    #[Get]
    pub has_minerals: bool,
    /// Other units that satisfy the same tech requirement
    pub tech_alias: Vec<u32>,
    /// The morphed variant of this unit
    #[Get]
    pub unit_alias: u32,
    #[Get]
    pub tech_requirement: u32,
    #[Get]
    pub require_attached: bool,

    pub attributes: Vec<protos::Attribute>,
    #[Get]
    pub movement_speed: f32,
    #[Get]
    pub armor: f32,
    pub weapons: Vec<Weapon>,
}

#[derive(Debug, FromProtobuf)]
pub struct Weapon {
    #[Get]
    pub field_type: protos::Weapon_TargetType,
    #[Get]
    pub damage: f32,
    pub damage_bonus: Vec<DamageBonus>,
    /// Number of hits per attack
    ///
    /// e.g. Colossus has 2 beams
    #[Get]
    pub attacks: u32,
    #[Get]
    pub range: f32,
    #[Get]
    pub speed: f32,
}

#[derive(Debug, FromProtobuf)]
pub struct DamageBonus {
    #[Get]
    pub attribute: protos::Attribute,
    #[Get]
    pub bonus: f32,
}

#[derive(Debug, FromProtobuf)]
pub struct UpgradeData {
    #[Get]
    pub upgrade_id: u32,
    pub name: String,
    #[Get]
    pub mineral_cost: u32,
    #[Get]
    pub vespene_cost: u32,
    #[Get]
    pub research_time: f32,
    #[Get]
    pub ability_id: u32,
}

#[derive(Debug, FromProtobuf)]
pub struct BuffData {
    #[Get]
    pub buff_id: u32,
    pub name: String,
}

#[derive(Debug, FromProtobuf)]
pub struct EffectData {
    #[Get]
    pub effect_id: u32,
    pub name: String,
    pub friendly_name: String,
    #[Get]
    pub radius: f32,
}

#[derive(Debug, FromProtobuf)]
pub struct ResponseStep {}

#[derive(Debug, ToProtobuf)]
pub struct RequestStep {
    /// Number of game loops to simulate for the next frame
    pub count: u32,
}

#[derive(Debug, ToProtobuf)]
pub enum Request {
    CreateGame(RequestCreateGame),
    JoinGame(RequestJoinGame),
    AvailableMaps(RequestAvailableMaps),
    GameInfo(RequestGameInfo),
    Observation(RequestObservation),
    Data(RequestData),
    Step(RequestStep),
}


#[derive(Debug, ToProtobuf, FromProtobuf)]
pub struct ResponseCreateGame {
    #[Get]
    pub error: Option<protos::ResponseCreateGame_Error>,
    pub error_details: Option<String>,
}

#[derive(Debug, ToProtobuf, FromProtobuf)]
pub struct ResponseJoinGame {
    #[Get]
    pub player_id: u32,

    #[Get]
    pub error: Option<protos::ResponseJoinGame_Error>,
    pub error_details: Option<String>,
}


#[derive(Debug, FromProtobuf)]
pub struct ResponseAvailableMaps {
    pub local_map_paths: Vec<String>,
    pub battlenet_map_names: Vec<String>,
}

#[derive(Debug, FromProtobuf)]
pub struct Action {
    /// Populated if Raw interface is enabled
    pub action_raw: Option<ActionRaw>,
    // /// Populated if Feature Layer interface is enabled
    // pub action_feature_layer: Option<ActionSpatial>,
    // /// Not implemented. Populated if Render interface is enabled
    // pub action_render: Option<ActionSpatial>,
    // /// Populated if Feature Layer or Render interface is enabled
    // pub action_ui: Option<ActionUI>,
    /// Chat messages as a player typing into the chat channel
    pub chat: Vec<ActionChat>,
}


#[derive(Debug, FromProtobuf)]
pub struct ActionChat {
    #[Get]
    pub channel: protos::ActionChat_Channel,
    pub message: String,
}

#[derive(Debug, FromProtobuf)]
pub enum ActionRaw {
    UnitCommand(ActionRawUnitCommand),
    CameraMove(ActionRawCameraMove),
    ToggleAutocast(ActionRawToggleAutocast),
}

#[derive(Debug, FromProtobuf)]
#[AttachedTo(ActionRawUnitCommand)]
pub enum ActionRawUnitCommandTargetEnum {
    TargetWorldSpacePos(Point2D),
    #[Get]
    TargetUnitTag(u64),
}

#[derive(Debug, FromProtobuf)]
pub struct ActionRawUnitCommand {
    #[Get]
    pub ability_id: i32,
    #[OneOf]
    pub target: ActionRawUnitCommandTargetEnum,
    pub unit_tags: Vec<u64>,
    #[Get]
    pub queue_command: bool,
}

#[derive(Debug, FromProtobuf)]
pub struct ActionRawToggleAutocast {
    #[Get]
    pub ability_id: i32,
    pub unit_tags: Vec<u64>,
}

#[derive(Debug, FromProtobuf)]
pub struct ActionRawCameraMove {
    pub center_world_space: Point,
}

#[derive(Debug, FromProtobuf)]
pub struct ActionError {
    /// Only populated when using raw interface
    #[Get]
    pub unit_tag: Option<u64>,
    #[Get]
    pub ability_id: u64,
    #[Get]
    pub result: protos::ActionResult,
}

#[derive(Debug, FromProtobuf)]
pub struct PlayerCommon {
    #[Get]
    pub player_id: u32,
    #[Get]
    pub minerals: u32,
    #[Get]
    pub vespene: u32,
    #[Get]
    pub food_cap: u32,
    #[Get]
    pub food_used: u32,
    #[Get]
    pub food_army: u32,
    #[Get]
    pub food_workers: u32,
    #[Get]
    pub idle_worker_count: u32,
    #[Get]
    pub army_count: u32,
    #[Get]
    pub warp_gate_count: u32,
    #[Get]
    pub larva_count: u32,
}

#[derive(Debug, FromProtobuf)]
pub struct AvailableAbility {
    #[Get]
    pub ability_id: i32,
    #[Get]
    pub requires_point: bool,
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
pub struct Event {
    pub dead_units: Vec<u64>,
}

#[derive(Debug, FromProtobuf)]
pub struct MapState {
    /// 1 byte visibility layer
    pub visibility: ImageData,
    /// 1 byte creep layer
    pub creep: ImageData,
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

#[derive(Debug, FromProtobuf)]
pub struct PlayerRaw {
    pub power_sources: Vec<PowerSource>,
    pub camera: Point,
    pub upgrade_ids: Vec<u32>,
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
pub struct Unit {
    #[Get]
    pub display_type: protos::DisplayType,
    #[Get]
    pub alliance: protos::Alliance,
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
    pub cloak: protos::CloakState,

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
    pub target: UnitOrderTarget,
    /// Progress of train abilities.  Range 0.0 to 1.0
    #[Get]
    pub progress: f32,
}

#[derive(Debug, FromProtobuf)]
pub struct Observation {
    #[Get]
    pub game_loop: u32,
    pub player_common: PlayerCommon,
    pub alerts: Vec<protos::Alert>,
    /// Abilities available in the selection. Enabled if in this list, disabled otherwise
    pub abilities: Vec<AvailableAbility>,
    //TODO score: Score,
    pub raw_data: Option<ObservationRaw>,
    //TODO feature_layer_data
    //TODO render_data
    //TODO ui_data
}

#[derive(Debug, FromProtobuf)]
pub struct PlayerResult {
    #[Get]
    pub player_id: u32,
    #[Get]
    pub result: protos::Result,
}

use self::protos::Result as ProtoResult;


#[derive(Debug, FromProtobuf)]
pub struct ChatReceived {
    #[Get]
    pub player_id: i32,
    pub message: String,
}

#[derive(Debug, FromProtobuf)]
pub struct ResponseObservation {
    pub actions: Vec<Action>,
    pub action_errors: Vec<ActionError>,
    pub observation: Observation,
    pub player_result: Vec<PlayerResult>,
    pub chat: Vec<ChatReceived>,
}

impl ObservationRaw {
    pub fn get_my_units<'a>(&'a self) -> Vec<&'a Unit> {
        self.units.iter().filter(|u| u.alliance == protos::Alliance::Selff).collect()
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Status {
    /// Game has been launched and is not yet doing anything
    Launched = 1,
    /// Create game has been called, and the host is awaiting players
    InitGame = 2,
    /// In a single or multiplayer game
    InGame = 3,
    /// In a replay
    InReplay = 4,
    /// Game has ended, can still request game info, but ready for a new game
    Ended = 5,
    /// Application is shutting down
    Quit = 6,
    /// Should not happen
    Unknown = 99,
}

impl FromProtobuf<protos::Status> for Status {
    fn from_protobuf(t: protos::Status) -> Result<Self, failure::Error> {
        Ok(match t {
            protos::Status::launched => Status::Launched,
            protos::Status::init_game => Status::InitGame,
            protos::Status::in_game => Status::InGame,
            protos::Status::in_replay => Status::InReplay,
            protos::Status::ended => Status::Ended,
            protos::Status::quit => Status::Quit,
            protos::Status::unknown => Status::Unknown,
        })
    }
}


#[derive(Debug, FromProtobuf)]
#[AttachedTo(Response)]
pub enum ResponseEnum {
    CreateGame(ResponseCreateGame),
    AvailableMaps(ResponseAvailableMaps),
    JoinGame(ResponseJoinGame),
    GameInfo(ResponseGameInfo),
    Observation(ResponseObservation),
    Data(ResponseData),
    Step(ResponseStep),
}

#[derive(Debug, FromProtobuf)]
pub struct Response {
    #[OneOf]
    pub response: ResponseEnum,
    pub error: Vec<String>,
    #[Get]
    pub status: Option<Status>,
}
