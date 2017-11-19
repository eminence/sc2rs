=== Implementing ToProtobuf<LocalMap> for LocalMap
=== impl ToProtobuf < protos::LocalMap > for LocalMap { fn into_protobuf ( self ) -> protos::LocalMap { let mut pb = protos::LocalMap :: new ( ) ; pb . set_map_path ( self . MapPath . into_protobuf ( ) ) ; unimplemented ! ( ) } }
=== Implementing helper functions for Participation
=== impl Participation { fn set_fields ( & self , pb : & mut RequestJoinGame ) { } }
=== Implementing ToProtobuf<InterfaceOptions> for InterfaceOptions
=== impl ToProtobuf < protos::InterfaceOptions > for InterfaceOptions { fn into_protobuf ( self ) -> protos::InterfaceOptions { let mut pb = protos::InterfaceOptions :: new ( ) ; if let Some ( b ) = self . raw { pb . set_raw ( b ) ; } if let Some ( b ) = self . score { pb . set_score ( b ) ; } unimplemented ! ( ) } }
=== Implementing ToProtobuf<RequestJoinGame> for RequestJoinGame
=== impl ToProtobuf < protos::RequestJoinGame > for RequestJoinGame { fn into_protobuf ( self ) -> protos::RequestJoinGame { let mut pb = protos::RequestJoinGame :: new ( ) ; self . participation . set_fields ( & mut pb ) ; pb . set_options ( self . options . into_protobuf ( ) ) ; unimplemented ! ( ) } }
#![feature(prelude_import)]
#![no_std]
#![feature(const_atomic_bool_new)]
#![allow(dead_code, unused_imports, unused_variables, unused_must_use)]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std as std;


#[macro_use]
extern crate sc2_pb_derive;

extern crate protobuf;

use protobuf::core::Message;

extern crate sc2_protobuf;
extern crate ws;

use ws::util::Token;

#[macro_use]
extern crate failure;
#[macro_use]
extern crate failure_derive;

use failure::{Error, ResultExt};

extern crate url;

use url::Url;

use std::sync::mpsc::{Sender, Receiver};
use std::thread;
use std::process;
use std::path::Path;
use std::sync::atomic::AtomicBool;
use std::time::Duration;

pub mod types {




















    //receiver: Option<Receiver<types::Request>>






    // spawn a new thread to manage this child process


    // ws::connect will create a new event loop and run it to completion, so this needs to be in a new thread
    // let mut builder = builder; // move



    // create a channel to receive












    // try to parse message as a reply
    //                    break;
    //                    break;
    // timeout


    // set a timeout to try to read a request from our channel



    //  -listen 127.0.0.1 -port 8167 -displayMode 0 -windowwidth 1024 -windowheight 768 -windowx 100 -windowy 200

    /*
    Sending: create_game {
      local_map {
        map_path: "Z:\\devel\\s2client-api\\maps\\Ladder/(2)Bel\'ShirVestigeLE (Void).SC2Map"
      }
      player_setup {
        type: Participant
        race: Terran
        difficulty: Easy
      }
      player_setup {
        type: Computer
        race: Zerg
        difficulty: Easy
      }
      realtime: false
    }




    Sending: b




    */




    #![allow(dead_code,
             unused_imports,
             unused_variables,
             non_snake_case,
             unused_must_use)]
    use super::failure;
    use super::protobuf::repeated::RepeatedField;
    use super::sc2_protobuf::protos;
    pub trait FromProtobuf<T>: Sized {
        fn from_protobuf(t: T)
        -> Result<Self, failure::Error>;
    }
    pub trait ToProtobuf<T> {
        fn into_protobuf(self)
        -> T;
    }
    impl ToProtobuf<String> for String {
        fn into_protobuf(self) -> String { self }
    }
    impl <T, U> ToProtobuf<RepeatedField<T>> for Vec<U> where U: ToProtobuf<T>
     {
        fn into_protobuf(self) -> RepeatedField<T> {
            let newv = self.into_iter().map(|e| e.into_protobuf()).collect();
            RepeatedField::from_vec(newv)
        }
    }
    pub struct LocalMap {
        pub MapPath: String,
    }
    impl ToProtobuf<protos::LocalMap> for LocalMap {
        fn into_protobuf(self) -> protos::LocalMap {
            let mut pb = protos::LocalMap::new();
            pb.set_map_path(self.MapPath.into_protobuf());
            {
                ::rt::begin_panic("not yet implemented",
                                  &("src\\types.rs", 28u32, 9u32))
            }
        }
    }
    impl FromProtobuf<protos::LocalMap> for LocalMap {
        fn from_protobuf(mut t: protos::LocalMap)
         -> Result<Self, failure::Error> {
            if t.has_map_path() {
                Ok(LocalMap{MapPath: t.take_map_path(),})
            } else {
                Err(::err_msg(::fmt::format(::std::fmt::Arguments::new_v1(&["No map path"],
                                                                          &match ()
                                                                               {
                                                                               ()
                                                                               =>
                                                                               [],
                                                                           }))))
            }
        }
    }
    pub struct PlayerSetup {
        player_type: protos::PlayerType,
        race: protos::Race,
        difficulty: protos::Difficulty,
    }
    impl ToProtobuf<protos::PlayerSetup> for PlayerSetup {
        fn into_protobuf(self) -> protos::PlayerSetup {
            let mut pb = protos::PlayerSetup::new();
            pb.set_field_type(self.player_type);
            pb.set_race(self.race);
            pb.set_difficulty(self.difficulty);
            pb
        }
    }
    pub struct RequestCreateGame {
        map: protos::RequestCreateGame_oneof_Map,
        player_setup: Vec<PlayerSetup>,
        disable_fog: Option<bool>,
        random_seed: Option<u32>,
        realtime: Option<bool>,
    }
    impl ToProtobuf<protos::RequestCreateGame> for RequestCreateGame {
        fn into_protobuf(self) -> protos::RequestCreateGame {
            let mut pb = protos::RequestCreateGame::new();
            pb.set_player_setup(self.player_setup.into_protobuf());
            if let Some(x) = self.disable_fog { pb.set_disable_fog(x); };
            if let Some(x) = self.random_seed { pb.set_random_seed(x); };
            if let Some(x) = self.realtime { pb.set_realtime(x); };
            pb
        }
    }
    #[AttachedTo(RequestJoinGame)]
    pub enum Participation { Race(protos::Race), ObservedPlayerId(u32), }
    impl Participation {
        fn set_fields(&self, pb: &mut RequestJoinGame) { }
    }
    pub struct InterfaceOptions {
        raw: Option<bool>,
        score: Option<bool>,
    }
    impl ToProtobuf<protos::InterfaceOptions> for InterfaceOptions {
        fn into_protobuf(self) -> protos::InterfaceOptions {
            let mut pb = protos::InterfaceOptions::new();
            if let Some(b) = self.raw { pb.set_raw(b); }
            if let Some(b) = self.score { pb.set_score(b); }
            {
                ::rt::begin_panic("not yet implemented",
                                  &("src\\types.rs", 105u32, 9u32))
            }
        }
    }
    pub struct RequestJoinGame {
        #[OneOf]
        participation: Participation,
        options: InterfaceOptions,
    }
    impl ToProtobuf<protos::RequestJoinGame> for RequestJoinGame {
        fn into_protobuf(self) -> protos::RequestJoinGame {
            let mut pb = protos::RequestJoinGame::new();
            self.participation.set_fields(&mut pb);
            pb.set_options(self.options.into_protobuf());
            {
                ::rt::begin_panic("not yet implemented",
                                  &("src\\types.rs", 114u32, 9u32))
            }
        }
    }
    pub enum Request {
        CreateGame(RequestCreateGame),
        JoinGame(RequestJoinGame),
    }
    impl ToProtobuf<protos::Request> for Request {
        fn into_protobuf(self) -> protos::Request {
            let mut req = protos::Request::new();
            match self {
                Request::CreateGame(rcg) => {
                    let rcg_pb: protos::RequestCreateGame =
                        rcg.into_protobuf();
                    req.set_create_game(rcg_pb);
                }
                Request::JoinGame(rjg) => { }
            }
            req
        }
    }
    pub struct ResponseCreateGame {
        error: Option<protos::ResponseCreateGame_Error>,
        details: Option<String>,
    }
    impl FromProtobuf<protos::ResponseCreateGame> for ResponseCreateGame {
        fn from_protobuf(mut t: protos::ResponseCreateGame)
         -> Result<Self, failure::Error> {
            Ok(ResponseCreateGame{error:
                                      if t.has_error() {
                                          Some(t.get_error())
                                      } else { None },
                                  details:
                                      if t.has_error_details() {
                                          Some(t.take_error_details())
                                      } else { None },})
        }
    }
    pub enum Response { Noop, CreateGame(ResponseCreateGame), }
    impl FromProtobuf<protos::Response> for Response {
        fn from_protobuf(mut t: protos::Response)
         -> Result<Response, failure::Error> {
            if t.has_create_game() {
                let x = t.take_create_game();
                Ok(Response::CreateGame(ResponseCreateGame::from_protobuf(x)?))
            } else {
                Err(::err_msg(::fmt::format(::std::fmt::Arguments::new_v1(&["No known responses"],
                                                                          &match ()
                                                                               {
                                                                               ()
                                                                               =>
                                                                               [],
                                                                           }))))
            }
        }
    }
}
use types::ToProtobuf;
const TIMEOUT_TOKEN: Token = Token(1);
static KEEPRUNNING: AtomicBool = AtomicBool::new(true);
#[allow(non_snake_case)]
pub mod GameState {
    /// The starting state
    pub struct Unlaunched;
    pub struct Launched;
    pub struct InitGame;
    pub struct InGame;
    pub struct Ended;
    pub struct InReplay;
}
pub struct Coordinator<State> {
    sc2_join_handle: Option<thread::JoinHandle<process::ExitStatus>>,
    sc2_receiver: Option<Receiver<types::Response>>,
    sc2_sender: Option<Sender<types::Request>>,
    _state: std::marker::PhantomData<State>,
}
impl Coordinator<GameState::Unlaunched> {
    pub fn new() -> Coordinator<GameState::Unlaunched> {
        Coordinator{sc2_join_handle: None,
                    sc2_receiver: None,
                    sc2_sender: None,
                    _state: std::marker::PhantomData,}
    }
}
/// This is a factory that will only be able to tell you when you're connected.
struct ConnectOnlyWSFactory {
}
struct ConnectOnlyHandler {
    ws_sender: ws::Sender,
}
impl ws::Handler for ConnectOnlyHandler {
    fn on_open(&mut self, shake: ws::Handshake) -> ws::Result<()> {
        KEEPRUNNING.store(false, std::sync::atomic::Ordering::Release);
        self.ws_sender.shutdown();
        Ok(())
    }
}
impl ws::Factory for ConnectOnlyWSFactory {
    type
    Handler
    =
    ConnectOnlyHandler;
    fn connection_made(&mut self, ws_sender: ws::Sender) -> Self::Handler {
        ConnectOnlyHandler{ws_sender,}
    }
}
struct WSHandlerFactory {
    sender: Sender<types::Response>,
    receiver: Option<Receiver<types::Request>>,
}
impl WSHandlerFactory {
    fn new(s: Sender<types::Response>, r: Receiver<types::Request>)
     -> WSHandlerFactory {
        WSHandlerFactory{sender: s, receiver: Some(r),}
    }
}
impl ws::Factory for WSHandlerFactory {
    type
    Handler
    =
    WSHandler;
    fn connection_made(&mut self, ws_socket: ws::Sender) -> Self::Handler {
        if let Some(r) = self.receiver.take() {
            WSHandler{ws_socket, sender: self.sender.clone(), receiver: r,}
        } else {
            {
                ::rt::begin_panic("Unable to handle simultaneous connections",
                                  &("src\\lib.rs", 122u32, 12u32))
            }
        }
    }
    fn connection_lost(&mut self, hand: Self::Handler) {
        self.receiver = Some(hand.receiver);
    }
}
impl Coordinator<GameState::Unlaunched> {
    pub fn launch(self) -> Result<Coordinator<GameState::Launched>, Error> {
        let scpath =
            Path::new(r#"Z:\Program Files (x86)\StarCraft II\Versions\Base59587\SC2_x64.exe"#);
        let mut cmd =
            process::Command::new(&scpath).current_dir(r#"z:\Program Files (x86)\StarCraft II\Support64"#).args(&["-listen",
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
                                                                                                                  "100"]).spawn()?;
        let hand =
            thread::spawn(move ||
                              {
                                  cmd.wait().expect("Failed to spawn SC2_x64.exe")
                              });
        let wait_thread =
            thread::spawn(move ||
                              {
                                  let url =
                                      Url::parse("ws://127.0.0.1:8167/sc2api").unwrap();
                                  while KEEPRUNNING.load(std::sync::atomic::Ordering::Relaxed)
                                        {
                                      thread::sleep(std::time::Duration::from_secs(2));
                                      let mut builder =
                                          ws::Builder::new().build(ConnectOnlyWSFactory{}).unwrap();
                                      builder.connect(url.clone()).unwrap();
                                      builder.run();
                                  }
                                  ::io::_print(::std::fmt::Arguments::new_v1(&["Done with connect-only loop\n"],
                                                                             &match ()
                                                                                  {
                                                                                  ()
                                                                                  =>
                                                                                  [],
                                                                              }))
                              });
        ::io::_print(::std::fmt::Arguments::new_v1(&["Trying to make websocket connection\n"],
                                                   &match () { () => [], }));
        ::io::_print(::std::fmt::Arguments::new_v1(&["Waiting for websocket connection...\n"],
                                                   &match () { () => [], }));
        wait_thread.join();
        ::io::_print(::std::fmt::Arguments::new_v1(&["websocket connection open!\n"],
                                                   &match () { () => [], }));
        let (sc2_sender, remote_sc2_receiver) = std::sync::mpsc::channel();
        let (remote_sc2_sender, sc2_receiver) = std::sync::mpsc::channel();
        let builder =
            ws::Builder::new().build(WSHandlerFactory::new(remote_sc2_sender,
                                                           remote_sc2_receiver)).unwrap();
        Ok(Coordinator{sc2_join_handle: Some(hand),
                       sc2_receiver: Some(sc2_receiver),
                       sc2_sender: Some(sc2_sender),
                       _state: std::marker::PhantomData,})
    }
}
impl Coordinator<GameState::Launched> {
    pub fn create_game<T: Into<sc2_protobuf::protos::RequestCreateGame>>(self,
                                                                         req:
                                                                             T)
     -> Coordinator<GameState::InitGame> {
        let reqgame = req.into();
        let mut req = sc2_protobuf::protos::Request::new();
        req.set_create_game(reqgame);
        {
            ::rt::begin_panic("not yet implemented",
                              &("src\\lib.rs", 194u32, 8u32))
        }
    }
    pub fn join_game(self) -> Coordinator<GameState::InGame> {
        {
            ::rt::begin_panic("not yet implemented",
                              &("src\\lib.rs", 197u32, 8u32))
        }
    }
    pub fn start_replay(self) -> Coordinator<GameState::InReplay> {
        {
            ::rt::begin_panic("not yet implemented",
                              &("src\\lib.rs", 200u32, 8u32))
        }
    }
}
impl Coordinator<GameState::InitGame> {
    pub fn join_game(self) -> Coordinator<GameState::InGame> {
        {
            ::rt::begin_panic("not yet implemented",
                              &("src\\lib.rs", 206u32, 8u32))
        }
    }
}
pub struct WSHandler {
    /// The websocket that can be used to send data to starcraft
    ws_socket: ws::Sender,
    /// After decoding the protobuf message, it's sent down this channel
    sender: Sender<types::Response>,
    receiver: Receiver<types::Request>,
}
impl ws::Handler for WSHandler {
    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
        use types::FromProtobuf;
        match msg {
            ws::Message::Binary(bin_vec) => {
                let mut resp = sc2_protobuf::protos::Response::new();
                if resp.merge_from_bytes(&bin_vec).is_ok() {
                    match types::Response::from_protobuf(resp) {
                        Ok(r) => self.sender.send(r).expect("send"),
                        Err(e) => {
                            ::io::_print(::std::fmt::Arguments::new_v1(&["Failed to construct a Response: ",
                                                                         "\n"],
                                                                       &match (&e,)
                                                                            {
                                                                            (__arg0,)
                                                                            =>
                                                                            [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                         ::std::fmt::Display::fmt)],
                                                                        }));
                        }
                    }
                }
            }
            ws::Message::Text(s) => {
                ::io::_print(::std::fmt::Arguments::new_v1(&["Unexpected Text frame: ",
                                                             "\n"],
                                                           &match (&s,) {
                                                                (__arg0,) =>
                                                                [::std::fmt::ArgumentV1::new(__arg0,
                                                                                             ::std::fmt::Display::fmt)],
                                                            }));
            }
        }
        Ok(())
    }
    fn on_timeout(&mut self, event: Token) -> ws::Result<()> {
        match self.receiver.recv_timeout(Duration::from_secs(1)) {
            Ok(req) => {
                let pb = req.into_protobuf();
                let bytes = pb.write_to_bytes().unwrap();
                ::io::_print(::std::fmt::Arguments::new_v1(&["Sending ",
                                                             " bytes...\n"],
                                                           &match (&bytes.len(),)
                                                                {
                                                                (__arg0,) =>
                                                                [::std::fmt::ArgumentV1::new(__arg0,
                                                                                             ::std::fmt::Display::fmt)],
                                                            }));
                self.ws_socket.send(ws::Message::binary(bytes)).unwrap();
            }
            Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => {
                ::io::_eprint(::std::fmt::Arguments::new_v1(&["Request receiver is unexpectedly disconnected"],
                                                            &match () {
                                                                 () => [],
                                                             }));
            }
            Err(_) => { }
        }
        self.ws_socket.timeout(100, TIMEOUT_TOKEN);
        Ok(())
    }
    fn on_open(&mut self, shake: ws::Handshake) -> ws::Result<()> {
        self.ws_socket.timeout(10, TIMEOUT_TOKEN);
        Ok(())
    }
}
