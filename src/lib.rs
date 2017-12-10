#![feature(conservative_impl_trait)]
#![allow(dead_code, unused_imports, unused_variables, unused_must_use)]

// used in gen/*.rs
#[macro_use]
extern crate lazy_static;

extern crate ws_sync;

extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate sc2_pb_derive;

extern crate protobuf;

use protobuf::core::Message;

extern crate sc2_protobuf;

#[macro_use]
extern crate failure;


use failure::{Error, ResultExt};

extern crate url;

use url::Url;

use std::sync::mpsc::{Sender, Receiver};
use std::thread;
use std::process;
use std::path::Path;
use std::sync::atomic::AtomicBool;
use std::time::Duration;

pub mod types;
mod gen;

pub use gen::*;

pub mod utils;


use types::ToProtobuf;


#[allow(non_snake_case)]
pub mod GameState {
    /// For states that allow us to send Requests via websockets
    #[doc(hidden)]
    pub trait StateConnected {}

    pub trait AllowsGameInfo: StateConnected {}

    pub trait AllowsObservation: StateConnected {}

    pub trait AllowsStep: StateConnected {}

    pub trait AllowsGameData: StateConnected {}

    pub trait AllowsQuery: StateConnected {}

    /// The starting state
    pub struct Unlaunched;

    pub struct Launched;

    impl StateConnected for Launched {}

    pub struct Connected;

    impl StateConnected for Connected {}

    pub struct InitGame;

    impl StateConnected for InitGame {}

    pub struct InGame;

    impl StateConnected for InGame {}

    impl AllowsGameInfo for InGame {}

    impl AllowsObservation for InGame {}

    impl AllowsStep for InGame {}

    impl AllowsGameData for InGame {}

    impl AllowsQuery for InGame {}

    pub struct Ended;

    impl StateConnected for Ended {}

    impl AllowsGameInfo for Ended {}

    impl AllowsObservation for Ended {}

    impl AllowsGameData for Ended {}

    impl AllowsQuery for Ended {}

    pub struct InReplay;

    impl StateConnected for InReplay {}

    impl AllowsGameInfo for InReplay {}

    impl AllowsObservation for InReplay {}

    impl AllowsGameData for InReplay {}

    impl AllowsQuery for InReplay {}
}


pub struct Coordinator<State> {
    ws_socket: Option<ws_sync::WebSocket>,
    _state: std::marker::PhantomData<State>,
}

impl<State> Coordinator<State>
    where
        State: GameState::StateConnected,
{
    fn get_request(&mut self, req: types::Request) -> Result<types::Response, Error> {
        use types::FromProtobuf;

        if let Some(ref mut sock) = self.ws_socket {
            let pb = req.into_protobuf();
            let bytes = pb.write_to_bytes()?;
            //println!("Sending {} bytes...", bytes.len());
            sock.send(bytes)?;

            match sock.read()? {
                ws_sync::Message::Text(s) => panic!("Unexpected string message {}", s),
                ws_sync::Message::Binary(data) => {
                    let mut resp = sc2_protobuf::protos::Response::new();
                    resp.merge_from_bytes(&data)?;

                    let resp = types::Response::from_protobuf(resp)?;
                    if resp.error.len() > 0 {
                        return Err(format_err!("Response contained errors: {:?}", resp.error));
                    }

                    return Ok(resp);
                }
            }
        } else {
            panic!("Tried to send request, but Coordinator isn't connected!");
        }
    }

    pub fn list_available_maps(&mut self) -> Result<types::ResponseAvailableMaps, Error> {
        let req = types::Request::AvailableMaps(types::RequestAvailableMaps {});
        let resp = self.get_request(req)?;

        if let types::ResponseEnum::AvailableMaps(r) = resp.response {
            return Ok(r);
        } else {
            return Err(format_err!("Unexpected response!"));
        }
    }
    fn req_inner<I, T>(&mut self, req: I) -> Result<T, Error>
        where types::ResponseEnum: types::Unpack<T>,
              types::Request: From<I> {
        use types::Unpack;

        let req: types::Request = From::from(req);
        let resp = self.get_request(req)?;

        if resp.error.len() > 0 {
            return Err(format_err!("Response contained errors: {:?}", resp.error));
        }

        if let Some(typ) = resp.response.unpack() {
            return Ok(typ);
        } else {
            return Err(format_err!("Unexpected response!"));
        }
    }
}

impl Coordinator<GameState::Unlaunched> {
    pub fn new() -> Coordinator<GameState::Unlaunched> {
        Coordinator {
            ws_socket: None,
            _state: std::marker::PhantomData,
        }
    }
}


impl Coordinator<GameState::Unlaunched> {
    pub fn launch(self) -> Result<Coordinator<GameState::Launched>, Error> {
        use std::str::FromStr;

        let scpath = Path::new(
            r#"Z:\Program Files (x86)\StarCraft II\Versions\\Base60321\SC2_x64.exe"#,
        );

        let mut cmd = process::Command::new(&scpath)
            .current_dir(r#"z:\Program Files (x86)\StarCraft II\Support64"#)
            .args(
                &[
                    "-listen",
                    "127.0.0.1",
                    "-port",
                    "8167",
                    "-displayMode",
                    "0",
                    "-windowwidth",
                    "1024",
                    "-windowheight",
                    "768",
                    "-windowx",
                    "100",
                    "-windowxy",
                    "100",
                ],
            )
            .spawn()?;

        // spawn a new thread to manage this child process
        let hand = thread::spawn(move || cmd.wait().expect("Failed to spawn SC2_x64.exe"));

        std::thread::sleep(std::time::Duration::from_secs(10));

        let url = Url::from_str("ws://127.0.0.1:8167/sc2api").unwrap();
        let sock = ws_sync::WebSocket::connect_with_retry(&url, std::time::Duration::from_secs(1));


        Ok(Coordinator {
            ws_socket: Some(sock),
            _state: std::marker::PhantomData,
        })
    }
}


impl Coordinator<GameState::Launched> {
    /// Connect to an already running starcraft2 instance
    pub fn connect(self, url: Url) -> Result<Coordinator<GameState::Connected>, Error> {
        let sock = ws_sync::WebSocket::connect(&url).unwrap();

        unimplemented!()
    }


    pub fn create_game(
        mut self,
        req: types::RequestCreateGame,
    ) -> Result<Coordinator<GameState::InitGame>, Error> {
        let resp = self.get_request(types::Request::CreateGame(req))?;

        if resp.status != Some(types::Status::InitGame) {
            return Err(format_err!("Game state is not correct: {:?}", resp.status));
        }

        if let types::ResponseEnum::CreateGame(r) = resp.response {
            if let Some(game_error) = r.error {
                return Err(format_err!(
                    "Game error: {:?} {:?}",
                    game_error,
                    r.error_details
                ));
            }
            return Ok(Coordinator {
                ws_socket: self.ws_socket,
                _state: std::marker::PhantomData,
            });
        } else {
            return Err(format_err!("Unexpected response type"));
        }
    }
    pub fn join_game(
        self,
        req: types::RequestJoinGame,
    ) -> Result<Coordinator<GameState::InGame>, Error> {
        unimplemented!()
    }
    pub fn start_replay(
        self,
        req: types::RequestJoinGame,
    ) -> Result<Coordinator<GameState::InReplay>, Error> {
        unimplemented!()
    }
}

impl Coordinator<GameState::InitGame> {
    pub fn join_game(
        mut self,
        req: types::RequestJoinGame,
    ) -> Result<Coordinator<GameState::InGame>, Error> {
        let resp = self.get_request(types::Request::JoinGame(req))?;

        if resp.status != Some(types::Status::InGame) {
            return Err(format_err!("Game state is not correct: {:?}", resp.status));
        }

        if let types::ResponseEnum::JoinGame(r) = resp.response {
            if let Some(game_error) = r.error {
                return Err(format_err!(
                    "Game error: {:?} {:?}",
                    game_error,
                    r.error_details
                ));
            }
            return Ok(
                Coordinator {
                    ws_socket: self.ws_socket,
                    _state: std::marker::PhantomData
                }
            );
        } else {
            return Err(format_err!("Unexpected response type"));
        }
    }
}


impl<State> Coordinator<State>
    where
        State: GameState::AllowsGameInfo,
{
    pub fn game_info<T>(&mut self) -> Result<types::ResponseGameInfo, Error>
        where types::RequestGameInfo: From<T> {
        self.req_inner(types::RequestGameInfo{})
    }}

impl<State> Coordinator<State>
    where
        State: GameState::AllowsObservation,
{
    pub fn observation<T>(&mut self, t: T) -> Result<types::ResponseObservation, Error>
        where types::RequestObservation: From<T> {
        let req: types::RequestObservation = From::from(t);
        self.req_inner(req)
    }
}

impl<State> Coordinator<State>
    where
        State: GameState::AllowsStep,
{
    pub fn step<T>(&mut self, t: T) -> Result<types::ResponseStep, Error>
        where types::RequestStep: From<T> {
        let req: types::RequestStep = From::from(t);
        self.req_inner(req)
    }}

impl<State> Coordinator<State>
    where
        State: GameState::AllowsGameData,
{
    pub fn game_data<T>(&mut self, t: T) -> Result<types::ResponseData, Error>
        where types::RequestData: From<T> {
        let req: types::RequestData = From::from(t);
        self.req_inner(req)
    }}

impl<State> Coordinator<State>
    where
        State: GameState::AllowsQuery,
{
    pub fn query<T>(&mut self, t: T) -> Result<types::ResponseQuery, Error>
        where types::RequestQuery: From<T> {
        let req: types::RequestQuery = From::from(t);
        self.req_inner(req)
    }}

impl Coordinator<GameState::InGame> {
    pub fn debug<T>(&mut self, t: T) -> Result<types::ResponseDebug, Error>
        where types::RequestDebug: From<T> {
        let req: types::RequestDebug = From::from(t);
        self.req_inner(req)
    }
    pub fn action<T>(&mut self, t: T) -> Result<types::ResponseAction, Error>
        where types::RequestAction: From<T> {
        let req: types::RequestAction = From::from(t);
        self.req_inner(req)
    }
    //TODO leave_game
    //TODO quick_save
    //TODO quick_lock
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
