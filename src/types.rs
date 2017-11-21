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

#[derive(ToProtobuf, FromProtobuf)]
//#[ProtoType = "LocalMap"]
pub struct LocalMap {
    pub MapPath: String,
}

//impl FromProtobuf<protos::LocalMap> for LocalMap {
//    fn from_protobuf(mut t: protos::LocalMap) -> Result<Self, failure::Error> {
//        if t.has_map_path() {
//            Ok(LocalMap { MapPath: t.take_map_path() })
//        } else {
//            Err(format_err!("No map path"))
//        }
//    }
//}

#[derive(ToProtobuf, FromProtobuf)]
pub struct PlayerSetup {
    #[Get]
    field_type: protos::PlayerType,
    #[Get]
    race: protos::Race,
    #[Get]
    difficulty: protos::Difficulty,
}

#[derive(ToProtobuf, FromProtobuf)]
#[AttachedTo(RequestCreateGame)]
pub enum RequestMap {
    LocalMap(LocalMap),
    BattlenetMapName(String)
}

#[derive(ToProtobuf, FromProtobuf)]
pub struct RequestCreateGame {
    #[OneOf]
    map: RequestMap,
    player_setup: Vec<PlayerSetup>,
    #[Get]
    disable_fog: Option<bool>,
    #[Get]
    random_seed: Option<u32>,
    #[Get]
    realtime: Option<bool>,
}


#[derive(ToProtobuf, FromProtobuf)]
#[AttachedTo(RequestJoinGame)]
pub enum Participation {
    #[Get]
    Race(protos::Race),
    #[Get]
    ObservedPlayerId(u32),
}

#[derive(ToProtobuf, FromProtobuf)]
pub struct InterfaceOptions {
    #[Get]
    raw: Option<bool>,
    #[Get]
    score: Option<bool>,
    // TODO feature_layer
    // TODO render
}


#[derive(ToProtobuf, FromProtobuf)]
pub struct RequestJoinGame {
    #[OneOf]
    participation: Participation,
    options: InterfaceOptions,
    // TODO server_ports
    // TODO client_ports
    // TODO shared_port
}


#[derive(ToProtobuf,FromProtobuf)]
pub enum Request {
    CreateGame(RequestCreateGame),
    JoinGame(RequestJoinGame),
}


#[derive(ToProtobuf, FromProtobuf)]
pub struct ResponseCreateGame {
    #[Get]
    error: Option<protos::ResponseCreateGame_Error>,
    error_details: Option<String>,
}

//impl FromProtobuf<protos::ResponseCreateGame> for ResponseCreateGame {
//    fn from_protobuf(mut t: protos::ResponseCreateGame) -> Result<Self, failure::Error> {
//        Ok(ResponseCreateGame {
//            error: if t.has_error() {
//                Some(t.get_error())
//            } else {
//                None
//            },
//            error_details: if t.has_error_details() {
//                Some(t.take_error_details())
//            } else {
//                None
//            },
//        })
//    }
//}

//impl Send for Response;

#[derive(ToProtobuf, FromProtobuf)]
pub enum Response {
    CreateGame(ResponseCreateGame),
}

//impl FromProtobuf<protos::Response> for Response {
//    fn from_protobuf(mut t: protos::Response) -> Result<Response, failure::Error> {
//        if t.has_create_game() {
//            let x = t.take_create_game();
//            Ok(Response::CreateGame(ResponseCreateGame::from_protobuf(x)?))
//        } else {
//            Err(format_err!("No known responses"))
//        }
//    }
//}
