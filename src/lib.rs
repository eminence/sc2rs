#![feature(const_atomic_bool_new)]
#![allow(dead_code, unused_imports, unused_variables, unused_must_use)]


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

pub mod types;

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
    _state: std::marker::PhantomData<State>
}

impl Coordinator<GameState::Unlaunched> {
    pub fn new() -> Coordinator<GameState::Unlaunched> {
        Coordinator {
            sc2_join_handle: None,
            sc2_receiver: None,
            sc2_sender: None,
            _state: std::marker::PhantomData
        }
    }
}


/// This is a factory that will only be able to tell you when you're connected.
struct ConnectOnlyWSFactory {}


struct ConnectOnlyHandler {
    ws_sender: ws::Sender
}

impl ws::Handler for ConnectOnlyHandler {
    fn on_open(&mut self, shake: ws::Handshake) -> ws::Result<()> {
        KEEPRUNNING.store(false, std::sync::atomic::Ordering::Release);
        self.ws_sender.shutdown();
        Ok(())
    }
}

impl ws::Factory for ConnectOnlyWSFactory {
    type Handler = ConnectOnlyHandler;

    fn connection_made(&mut self, ws_sender: ws::Sender) -> Self::Handler {
        ConnectOnlyHandler { ws_sender }
    }
}

struct WSHandlerFactory {
    sender: Sender<types::Response>,
    receiver: Option<Receiver<types::Request>>,
    //receiver: Option<Receiver<types::Request>>
}

impl WSHandlerFactory {
    fn new(s: Sender<types::Response>, r: Receiver<types::Request>) -> WSHandlerFactory {
        WSHandlerFactory { sender: s, receiver: Some(r) }
    }
}

impl ws::Factory for WSHandlerFactory {
    type Handler = WSHandler;

    fn connection_made(&mut self, ws_socket: ws::Sender) -> Self::Handler {
        if let Some(r) = self.receiver.take() {
            WSHandler { ws_socket, sender: self.sender.clone(), receiver: r }
        } else {
            panic!("Unable to handle simultaneous connections")
        }
    }
    fn connection_lost(&mut self, hand: Self::Handler) {
        self.receiver = Some(hand.receiver);
    }
}

impl Coordinator<GameState::Unlaunched> {
    pub fn launch(self) -> Result<Coordinator<GameState::Launched>, Error> {
        let scpath = Path::new(r#"Z:\Program Files (x86)\StarCraft II\Versions\Base59587\SC2_x64.exe"#);

        let mut cmd = process::Command::new(&scpath)
            .current_dir(r#"z:\Program Files (x86)\StarCraft II\Support64"#)
            .args(&["-listen", "127.0.0.1",
                "-port", "8167",
                "-displayMode", "0",
                "-windowwidth", "1024",
                "-windowheight", "768",
                "-windowx", "100",
                "-windowxy", "100"
            ])
            .spawn()?;

        // spawn a new thread to manage this child process
        let hand = thread::spawn(move || {
            cmd.wait().expect("Failed to spawn SC2_x64.exe")
        });


        // ws::connect will create a new event loop and run it to completion, so this needs to be in a new thread
        let wait_thread = thread::spawn(move || {
            let url = Url::parse("ws://127.0.0.1:8167/sc2api").unwrap();
            // let mut builder = builder; // move
            while KEEPRUNNING.load(std::sync::atomic::Ordering::Relaxed) {
                thread::sleep(std::time::Duration::from_secs(2));
                let mut builder = ws::Builder::new().build(ConnectOnlyWSFactory {}).unwrap();
                builder.connect(url.clone()).unwrap();
                builder.run();
            }
            println!("Done with connect-only loop")
        });
        println!("Trying to make websocket connection");

        println!("Waiting for websocket connection...");
        wait_thread.join();
        println!("websocket connection open!");


        // create a channel to receive
        let (sc2_sender, remote_sc2_receiver) = std::sync::mpsc::channel();
        let (remote_sc2_sender, sc2_receiver) = std::sync::mpsc::channel();

        let builder = ws::Builder::new().build(WSHandlerFactory::new(remote_sc2_sender, remote_sc2_receiver)).unwrap();

        Ok(Coordinator {
            sc2_join_handle: Some(hand),
            sc2_receiver: Some(sc2_receiver),
            sc2_sender: Some(sc2_sender),
            _state: std::marker::PhantomData
        })
    }
}

impl Coordinator<GameState::Launched> {
    pub fn create_game<T: Into<sc2_protobuf::protos::RequestCreateGame>>(self, req: T) -> Coordinator<GameState::InitGame> {
        let reqgame = req.into();


        let mut req = sc2_protobuf::protos::Request::new();
        req.set_create_game(reqgame);

        unimplemented!()
    }
    pub fn join_game(self) -> Coordinator<GameState::InGame> {
        unimplemented!()
    }
    pub fn start_replay(self) -> Coordinator<GameState::InReplay> {
        unimplemented!()
    }
}

impl Coordinator<GameState::InitGame> {
    pub fn join_game(self) -> Coordinator<GameState::InGame> {
        unimplemented!()
    }
}

pub struct WSHandler {
    /// The websocket that can be used to send data to starcraft
    ws_socket: ws::Sender,

    /// After decoding the protobuf message, it's sent down this channel
    sender: Sender<types::Response>,
    receiver: Receiver<types::Request>
}


impl ws::Handler for WSHandler {
    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
        use types::FromProtobuf;

        // try to parse message as a reply
        match msg {
            ws::Message::Binary(bin_vec) => {
                let mut resp = sc2_protobuf::protos::Response::new();
                if resp.merge_from_bytes(&bin_vec).is_ok() {
                    match types::Response::from_protobuf(resp) {
                        Ok(r) => self.sender.send(r).expect("send"),
                        Err(e) => {
                            println!("Failed to construct a Response: {}", e);
                        }
                    }
                }
            }
            ws::Message::Text(s) => {
                println!("Unexpected Text frame: {}", s);
            }
        }
        Ok(())
    }
    fn on_timeout(&mut self, event: Token) -> ws::Result<()> {
        match self.receiver.recv_timeout(Duration::from_secs(1)) {
            Ok(req) => {
                let pb = req.into_protobuf();
                let bytes = pb.write_to_bytes().unwrap();
                println!("Sending {} bytes...", bytes.len());
                self.ws_socket.send(ws::Message::binary(bytes)).unwrap();
            }
            Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => {
                eprint!("Request receiver is unexpectedly disconnected");
                //                    break;
            }
            Err(_) => {
                //                    break;
                // timeout
            }
        }

        self.ws_socket.timeout(100, TIMEOUT_TOKEN);

        Ok(())
    }
    fn on_open(&mut self, shake: ws::Handshake) -> ws::Result<()> {
        self.ws_socket.timeout(10, TIMEOUT_TOKEN);
        // set a timeout to try to read a request from our channel

        Ok(())
    }
}


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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }


}