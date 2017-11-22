#![allow(dead_code, unused_imports, unused_variables, non_snake_case, unused_must_use)]

use super::failure;
use super::protobuf::repeated::RepeatedField;

use super::sc2_protobuf::protos;

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
ProtoSelf!(u32);


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

impl<T, U> FromProtobuf<RepeatedField<T>> for Vec<U>
    where U: FromProtobuf<T>
{
    fn from_protobuf(t: RepeatedField<T>) -> Result<Self, failure::Error> {
        Ok(t.into_vec().into_iter().map(|e| FromProtobuf::from_protobuf(e).unwrap()).collect())
    }
}

#[derive(Debug,ToProtobuf, FromProtobuf)]
//#[ProtoType = "LocalMap"]
pub struct LocalMap {
    pub MapPath: String,
}


#[derive(Debug,ToProtobuf, FromProtobuf)]
pub struct PlayerSetup {
    #[Get]
    field_type: protos::PlayerType,
    #[Get]
    race: protos::Race,
    #[Get]
    difficulty: protos::Difficulty,
}

#[derive(Debug,ToProtobuf, FromProtobuf)]
#[AttachedTo(RequestCreateGame)]
pub enum RequestMap {
    LocalMap(LocalMap),
    BattlenetMapName(String)
}

#[derive(Debug,ToProtobuf, FromProtobuf)]
pub struct RequestCreateGame {
    #[OneOf]
    pub map: RequestMap,
    pub player_setup: Vec<PlayerSetup>,
    #[Get]
    pub disable_fog: Option<bool>,
    #[Get]
    pub random_seed: Option<u32>,
    #[Get]
    pub realtime: Option<bool>,
}


#[derive(Debug,ToProtobuf, FromProtobuf)]
#[AttachedTo(RequestJoinGame)]
pub enum Participation {
    #[Get]
    Race(protos::Race),
    #[Get]
    ObservedPlayerId(u32),
}

#[derive(Debug,ToProtobuf, FromProtobuf)]
pub struct InterfaceOptions {
    #[Get]
    raw: Option<bool>,
    #[Get]
    score: Option<bool>,
    // TODO feature_layer
    // TODO render
}


#[derive(Debug,ToProtobuf, FromProtobuf)]
pub struct RequestJoinGame {
    #[OneOf]
    participation: Participation,
    options: InterfaceOptions,
    // TODO server_ports
    // TODO client_ports
    // TODO shared_port
}


#[derive(Debug,ToProtobuf,FromProtobuf)]
pub enum Request {
    CreateGame(RequestCreateGame),
    JoinGame(RequestJoinGame),
}


#[derive(Debug,ToProtobuf, FromProtobuf)]
pub struct ResponseCreateGame {
    #[Get]
    error: Option<protos::ResponseCreateGame_Error>,
    error_details: Option<String>,
}


#[derive(Debug,ToProtobuf, FromProtobuf)]
pub enum Response {
    CreateGame(ResponseCreateGame),
}

