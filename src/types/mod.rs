#![allow(dead_code, unused_imports, unused_variables, non_snake_case, unused_must_use)]

use super::failure;
use super::protobuf::repeated::RepeatedField;

use super::sc2_protobuf::protos;
use super::UnitIDs;

mod common;
pub use self::common::*;

mod raw;
pub use self::raw::*;

mod data;
pub use self::data::*;

mod error;
pub use self::error::*;

mod ui;
pub use self::ui::*;

mod spatial;
pub use self::spatial::*;

mod debug;
pub use self::debug::*;

pub trait RequestMessage<T, U>: ToProtobuf<T>
where
    Self::Reply: FromProtobuf<U>,
{
    type Reply;
}

pub trait FromU32: Sized {
    fn from_u32(val: u32) -> Option<Self>;
}

pub trait FromProtobuf<T>: Sized {
    fn from_protobuf(t: T) -> Result<Self, failure::Error>;
}

trait ToProtoSimple {} // marker trait

macro_rules! ProtoSelf {
    ($t:ty) => {
        impl ToProtobuf < $t > for $t { fn into_protobuf(self) -> $t { self }}
        impl ToProtoSimple for $t {}
        impl FromProtobuf < $t > for $t {
            fn from_protobuf(t:$t) -> Result<Self, failure::Error> { Ok(t) }
        }
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

impl Unit {
    pub fn is_worker(&self) -> bool {
        super::utils::is_worker(UnitIDs::from_u32(self.unit_type).unwrap())
    }
    pub fn is_idle(&self) -> bool {
        self.orders.len() == 0
    }

    pub fn unit_type(&self) -> UnitIDs {
        UnitIDs::from_u32(self.unit_type).expect("Unknown unit type id")
    }
    pub fn is_visible(&self) -> bool {
        self.display_type == DisplayType::Visible
    }
}


#[derive(Debug, ToProtobuf, FromProtobuf)]
//#[ProtoType = "LocalMap"]
pub struct LocalMap {
    pub MapPath: String,
}

#[derive(Clone, PartialEq, Eq, Debug, Hash, ToProtobuf, FromProtobuf)]
pub enum PlayerType {
    Participant = 1,
    Computer = 2,
    Observer = 3,
}


#[derive(Clone, PartialEq, Eq, Debug, Hash, ToProtobuf, FromProtobuf)]
pub enum Difficulty {
    VeryEasy = 1,
    Easy = 2,
    Medium = 3,
    MediumHard = 4,
    Hard = 5,
    Harder = 6,
    VeryHard = 7,
    CheatVision = 8,
    CheatMoney = 9,
    CheatInsane = 10,
}

#[derive(Debug, ToProtobuf, FromProtobuf)]
pub struct PlayerSetup {
    #[Get]
    pub field_type: PlayerType,
    #[Get]
    pub race: Race,
    #[Get]
    pub difficulty: Difficulty,
}


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
    Race(Race),
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


#[derive(Debug, FromProtobuf)]
pub struct PlayerInfo {
    #[Get]
    pub player_id: u32,
    #[Get]
    pub field_type: PlayerType,
    #[Get]
    pub race_requested: Race,
    /// Only populated for your player or when watching replay
    #[Get]
    pub race_actual: Option<Race>,
    #[Get]
    pub difficulty: Difficulty,
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
pub struct ResponseStep {}

#[derive(Debug, ToProtobuf)]
pub struct RequestStep {
    /// Number of game loops to simulate for the next frame
    pub count: u32,
}

#[derive(Debug, ToProtobuf)]
pub struct RequestAction {
    pub actions: Vec<Action>
}

#[derive(Debug, ToProtobuf)]
pub struct RequestDebug {
    pub debug: Vec<DebugCommand>
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
    Action(RequestAction),
    Debug(RequestDebug),
}


#[derive(Clone, PartialEq, Eq, Debug, Hash, ToProtobuf, FromProtobuf)]
#[allow(non_camel_case_types)]
pub enum ResponseCreateGame_Error {
    MissingMap = 1,
    InvalidMapPath = 2,
    InvalidMapData = 3,
    InvalidMapName = 4,
    InvalidMapHandle = 5,
    MissingPlayerSetup = 6,
    InvalidPlayerSetup = 7,
    MultiplayerUnsupported = 8,
}

#[derive(Debug, ToProtobuf, FromProtobuf)]
pub struct ResponseCreateGame {
    #[Get]
    pub error: Option<ResponseCreateGame_Error>,
    pub error_details: Option<String>,
}


#[derive(Clone, PartialEq, Eq, Debug, Hash, ToProtobuf, FromProtobuf)]
#[allow(non_camel_case_types)]
pub enum ResponseJoinGame_Error {
    MissingParticipation = 1,
    InvalidObservedPlayerId = 2,
    MissingOptions = 3,
    MissingPorts = 4,
    GameFull = 5,
    LaunchError = 6,
    FeatureUnsupported = 7,
    NoSpaceForUser = 8,
    MapDoesNotExist = 9,
    CannotOpenMap = 10,
    ChecksumError = 11,
    NetworkError = 12,
    OtherError = 13,
}


#[derive(Debug, ToProtobuf, FromProtobuf)]
pub struct ResponseJoinGame {
    #[Get]
    pub player_id: u32,

    #[Get]
    pub error: Option<ResponseJoinGame_Error>,
    pub error_details: Option<String>,
}


#[derive(Debug, FromProtobuf)]
pub struct ResponseAvailableMaps {
    pub local_map_paths: Vec<String>,
    pub battlenet_map_names: Vec<String>,
}

#[derive(Debug, ToProtobuf, FromProtobuf)]
pub struct Action {
    /// Populated if Raw interface is enabled
    pub action_raw: Option<ActionRaw>,
    /// Populated if Feature Layer interface is enabled
    pub action_feature_layer: Option<ActionSpatial>,
    /// Not implemented. Populated if Render interface is enabled
    pub action_render: Option<ActionSpatial>,
    /// Populated if Feature Layer or Render interface is enabled
    pub action_ui: Option<ActionUI>,
    /// Chat messages as a player typing into the chat channel
    pub chat: Vec<ActionChat>,
}

#[derive(Clone, PartialEq, Eq, Debug, Hash, ToProtobuf, FromProtobuf)]
#[allow(non_camel_case_types)]
pub enum ActionChat_Channel {
    Broadcast = 1,
    Team = 2,
}

#[derive(Debug, ToProtobuf, FromProtobuf)]
pub struct ActionChat {
    #[Get]
    pub channel: ActionChat_Channel,
    pub message: String,
}


#[derive(Clone, PartialEq, Eq, Debug, Hash, FromProtobuf)]
#[ProtoType(Result)]
pub enum ProtoResult {
    Victory = 1,
    Defeat = 2,
    Tie = 3,
    Undecided = 4,
}


#[derive(Debug, FromProtobuf)]
pub struct ActionError {
    /// Only populated when using raw interface
    #[Get]
    pub unit_tag: Option<u64>,
    #[Get]
    pub ability_id: u64,
    #[Get]
    pub result: ActionResult,
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


#[derive(Clone, PartialEq, Eq, Debug, Hash, FromProtobuf)]
pub enum Alert {
    NuclearLaunchDetected = 1,
    NydusWormDetected = 2,
}

#[derive(Debug, FromProtobuf)]
pub struct Observation {
    #[Get]
    pub game_loop: u32,
    pub player_common: PlayerCommon,
    pub alerts: Vec<Alert>,
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
    pub result: ProtoResult,
}


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
    pub fn get_my_units<'a>(&'a self) -> impl Iterator<Item = &'a Unit> {
        self.units.iter().filter(|u| u.alliance == Alliance::Selff)
    }
    pub fn get_my_idle_units<'a>(&'a self) -> impl Iterator<Item = &'a Unit> {
        self.get_my_units().filter(|u| u.is_idle())
    }

    pub fn find_by_tag<'a>(&'a self, tag: u64) -> Option<&'a Unit> {
        self.units.iter().find(|u| u.tag == tag)
    }

    pub fn find_by_type<'a>(&'a self, ty: UnitIDs) -> impl Iterator<Item = &'a Unit> {
        self.units.iter().filter(move |u| u.unit_type == ty as u32)
    }
}

#[derive(Debug, Eq, PartialEq, FromProtobuf)]
pub enum Status {
    /// Game has been launched and is not yet doing anything
    #[name = "launched"]
    Launched = 1,
    /// Create game has been called, and the host is awaiting players
    #[name = "init_game"]
    InitGame = 2,
    /// In a single or multiplayer game
    #[name = "in_game"]
    InGame = 3,
    /// In a replay
    #[name = "in_replay"]
    InReplay = 4,
    /// Game has ended, can still request game info, but ready for a new game
    #[name = "ended"]
    Ended = 5,
    /// Application is shutting down
    #[name = "quit"]
    Quit = 6,
    /// Should not happen
    #[name = "unknown"]
    Unknown = 99,
}


#[derive(Debug, FromProtobuf)]
pub struct ResponseDebug {}

#[derive(Debug, FromProtobuf)]
pub struct ResponseAction {
    pub result: Vec<ActionResult>,
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
    Debug(ResponseDebug),
    Action(ResponseAction),
}

#[derive(Debug, FromProtobuf)]
pub struct Response {
    #[OneOf]
    pub response: ResponseEnum,
    pub error: Vec<String>,
    #[Get]
    pub status: Option<Status>,
}
