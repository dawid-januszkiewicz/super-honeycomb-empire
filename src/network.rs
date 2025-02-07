use rand::random;
use serde::Deserialize;
use serde::Serialize;
use strum::Display;

use crate::Assets;
use crate::Controller;
use crate::Cube;
use crate::Fog;
use crate::Game;
use crate::Editor;
use crate::Layout;
use crate::World;
use crate::Command;
use crate::Player;

use core::panic;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::format;
use std::io::BufRead;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Read;
use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;
use std::net::ToSocketAddrs;

pub trait Endpoint {
    fn poll(&mut self, layout: &mut Layout<f32>) -> bool;
    fn draw(&self, layout: &Layout<f32>, assets: &Assets, time: f32);
    fn update(self: Box<Self>) -> Box<dyn Endpoint>;
    // fn swap_app(self: Box<Self>) -> Box<dyn Endpoint>;
}

pub struct NullEndpoint<T: Component> {
    pub app: T,
}

impl NullEndpoint<Game> {
    pub fn new(app: Game) -> Self {
        Self {app}
    }
}

impl Endpoint for NullEndpoint<Game> {
    fn poll(&mut self, layout: &mut Layout<f32>) -> bool {
        self.app.poll(layout)
    }
    fn draw(&self, layout: &Layout<f32>, assets: &Assets, time: f32) {
        self.app.draw(layout, assets, time)
    }
    fn update(mut self: Box<Self>) -> Box<dyn Endpoint> {
        self.app.update();
        self
    }
}

// impl<T: Component + 'static> Endpoint for Client<T> {
//     fn poll(&mut self, layout: &mut Layout<f32>) -> bool {
//         self.app.poll(layout)
//     }
//     fn draw(&self, layout: &Layout<f32>, assets: &Assets, time: f32) {
//         self.app.draw(layout, assets, time)
//     }
//     fn update(&mut self) {
//         self.app.update()
//     }
//     fn swap_app(self: Box<Self>) -> Box<dyn Endpoint> {
//         let app = self.app.swap();
//         Box::new(Server::new(app, "127.0.0.1:8080").unwrap())
//     }
// }

impl Endpoint for Client<Game> {
    fn poll(&mut self, layout: &mut Layout<f32>) -> bool {
        crate::poll_inputs_client(self, layout)
    }
    fn draw(&self, layout: &Layout<f32>, assets: &Assets, time: f32) {
        self.app.draw(layout, assets, time)
    }
    fn update(mut self: Box<Self>) -> Box<dyn Endpoint> {
        // Force a player to skip a turn if he has no units to move or no action points left.
        let Some(current_player_index) = self.app.current_player_index() else {return self};
        let current_player = self.app.current_player().unwrap();

        let can_player_issue_a_command = self.app.world.can_player_issue_a_command(&current_player_index);
        if current_player.actions == 0 || !can_player_issue_a_command {
            self.app.next_turn();
        }

        match read_json_message_async(&self.stream) {
            Some(result) => {
                match result {
                    Ok(message) => {
                        self.handle_message(message);
                    },
                    Err(e) => {
                        panic!("{}", e);
                    }
                }
            }
            None => {},
        }
        self
    }
    // fn swap_app(self: Box<Self>) -> Box<dyn Endpoint> {
    //     let app = self.app.swap();
    //     Box::new(Server::new(app, "127.0.0.1:8080").unwrap())
    // }
}

// impl<T: Component + 'static> Endpoint for Server<T> {
//     fn poll(&mut self, layout: &mut Layout<f32>) -> bool {
//         self.app.poll(layout)
//     }
//     fn draw(&self, layout: &Layout<f32>, assets: &Assets, time: f32) {
//         self.app.draw(layout, assets, time)
//     }
//     fn update(&mut self) {
//         self.app.update()
//     }
//     fn swap_app(self: Box<Self>) -> Box<dyn Endpoint> {
//         let app = self.app.swap();
//         Box::new(Server::new(app, "127.0.0.1:8080").unwrap())
//     }
// }

impl Endpoint for Server<Game> {
    fn poll(&mut self, layout: &mut Layout<f32>) -> bool {
        // true
        self.app.poll(layout)
    }
    fn draw(&self, layout: &Layout<f32>, assets: &Assets, time: f32) {
        self.app.draw(layout, assets, time)
    }
    fn update(mut self: Box<Self>) -> Box<dyn Endpoint> {
        self = Box::new(self.handle_client().unwrap());
        self.app.update();
        self = Box::new(self.poll_current_stream());
        self//Box::new(self)
    }
    // fn swap_app(self: Box<Self>) -> Box<dyn Endpoint> {
    //     let app = self.app.swap();
    //     Box::new(Server::new(app, "127.0.0.1:8080").unwrap())
    // }
}

pub struct Client<T: Component> {
    pub app: T,
    pub stream: TcpStream,
}

impl Command {
    /// given a set of observers, work out which command segments are detected
    /// and turn those into separate subcommands.
    // pub fn get_observed_sections(&self, observers: &Fog) -> Vec<Command> {
    //     todo!()
    //                 // if (command.from in fog.keys() || command.to in fog.keys()) {

    //         // }
    //     // let command_path: 
    //     // let observed_route_positions = 
    // }
    pub fn get_observed_sections(&self, observers: Option<&Fog>) -> Vec<Command> {
        match observers {
            Some(fog) => {
                if (fog.contains_key(&self.from) || fog.contains_key(&self.to)) {
                    vec![self.clone()]
                } else {
                    vec![]
                }
            },
            None => vec![self.clone()]
        }
    }
}

// #[derive(Serialize, Deserialize)]
// enum ServerResponse {
//     ValidMove(World),
//     InvalidMove,
// }

// impl<T: Component> Client<T> {
//     pub fn new<A: std::net::ToSocketAddrs>(app: T, addr: A) -> Result<Self, std::io::Error> {
//         let mut stream = TcpStream::connect(addr)?;

//         let mut buffer = Vec::new();
//         stream.read_to_end(&mut buffer)?;
//         let world: World = serde_json::from_slice(&buffer).expect("Failed to deserialize response");

//         let self_ = Self{player: Player::new("default", None), app, stream};
//         self_.set_world(world);
//         Ok(self_)

//         //Ok(Self{player: Player::new("default", None), app, stream})
//         // if let Ok(stream) = TcpStream::connect(addr) {
//         //     println!("Connected to the server!");
//         // } else {
//         //     println!("Couldn't connect to server...");
//         // }
//     }
// }


impl Client<Game> {
    pub fn new<A: std::net::ToSocketAddrs + core::fmt::Display>(mut app: Game, addr: A) -> Result<Self, std::io::Error> {
        println!("Connecting to {}...", addr);
        let stream = TcpStream::connect(addr)?;
        println!("TCP connection established...");

        let n = random::<usize>() % 100;
        let player = Player::new(&format!("Player {}", n), Controller::Human);
        let message = Message::NewPlayer{starting_position: Cube::new(0, 0), player};
        write_json_message(&stream, &message).unwrap();
        println!("Player sent...");
        // TODO: This will leak player.selection - perhaps censor the field before sending it here?
        let Message::Initialise { turn, players, world } = read_json_message(&stream).unwrap() else {panic!()};
        app.players = players;
        let my_idx = app.players.len() - 1;
        app.players[my_idx].controller = Controller::Human;
        app.world = world;
        app.turn = turn;
        println!("Game state received...");

        stream.set_nonblocking(true)?;
        Ok(Self{app, stream})

        //Ok(Self{player: Player::new("default", None), app, stream})
        // if let Ok(stream) = TcpStream::connect(addr) {
        //     println!("Connected to the server!");
        // } else {
        //     println!("Couldn't connect to server...");
        // }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ServerResponseError;

impl core::fmt::Display for ServerResponseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl std::error::Error for ServerResponseError {}


impl Client<Game> {
    pub fn send_command(&mut self, command: Command) -> Result<Command, std::io::Error>{
        let message = Message::Command(command);
        write_json_message(&self.stream, &message);
        let Message::Command(command) = message else {panic!()};
        let Message::RevealFog(result) = read_json_message(&self.stream).unwrap() else {panic!()};
        match result {
            Ok(mut world) => self.app.world.extend(world.drain()),
            Err(e) => panic!("{}", e),
        }
        Ok(command)
    }
    pub fn listen_for_player_joins(&mut self) {
        let Some(Ok(Message::NewPlayer { starting_position, player })) = read_json_message_async(&self.stream) else {return};
        self.app.world.gen_capital_at_cube(self.app.players.len(), starting_position);
        self.app.players.push(player);
    }
}

// pub trait Component {
//     type Swap;
//     fn poll(&mut self, layout: &mut Layout<f32>) -> bool;
//     fn draw(&self, layout: &Layout<f32>, assets: &Assets, time: f32);
//     fn update(&mut self);
//     fn swap(self) -> Self::Swap; //impl Component;
// }
pub trait Component {
    fn poll(&mut self, layout: &mut Layout<f32>) -> bool;
    fn draw(&self, layout: &Layout<f32>, assets: &Assets, time: f32);
    fn update(&mut self);
    fn swap(self) -> impl Component;
}

pub struct Server<T: Component> {
    // game: Game,
    pub app: T,
    listener: TcpListener,
    streams: HashMap<usize, TcpStream>, // some players may not have a stream
}

impl<T: Component> Server<T> {
    pub fn new<A: std::net::ToSocketAddrs>(app: T, addr: A) -> Result<Server<T>, std::io::Error>{
        let listener = std::net::TcpListener::bind(addr)?;
        listener.set_nonblocking(true)?;
        let streams = HashMap::new();
        Ok(Self{app, listener, streams})
        // let listeners: Result<Vec<_>, _> = addrs.iter().map(|a| {std::net::TcpListener::bind(a)}).collect();
        // Ok(Self{game, listeners: listeners?})
    }
}

impl Server<Game> {
    fn handle_client(mut self) -> std::io::Result<Self> {
        for stream in self.listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    // add Player to Game.players
                    let Message::NewPlayer { player: received_player, ..} = read_json_message(&stream).unwrap() else{panic!()};
                    let player_idx = self.app.players.len();
                    let mut player = Player::new(&received_player.name, Controller::Remote);
                    self.app.players.push(player);
                    // generate a starting position for the player
                    let cubes_with_cities = &self.app.world.get_cubes_with_cities();
                    let starting_position = *&self.app.world.gen_random_capital(player_idx, &cubes_with_cities);

                    // send the visible starting area to incoming client
                    // compute the area
                    // let fog = self.app.player_fogs.get(&player_idx).unwrap();
                    // let observations = self.app.world.get_visible_subset(fog);
                    // write_json_message(&stream, &observations);

                    let message = Message::Initialise{turn: self.app.turn, players: self.app.players, world: self.app.world};
                    write_json_message(&stream, &message);
                    let Message::Initialise{players: mut p, world: w, ..} = message else {panic!()};
                    // self.app.players = p;
                    self.app.world = w;

                    player = p.pop().unwrap();
                    // broadcast to other players
                    let message = Message::NewPlayer{starting_position, player};
                    for (_, s) in &self.streams {
                        write_json_message(&s, &message);
                    }
                    let Message::NewPlayer{starting_position: _, player: pl} = message else {panic!()};
                    p.push(pl);
                    self.app.players = p;

                    stream.set_nonblocking(true)?;
                    //self.streams.push(stream);
                    self.streams.insert(player_idx, stream);
                    println!("{} joined", received_player.name);
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    // No more connections to accept right now
                    // break to prevent from blocking
                    break;
                }
                Err(e) => {
                    eprintln!("Failed to accept a connection: {:?}", e);
                }
            }
        }
        Ok(self)
    }

    fn poll_current_stream(mut self) -> Self {
        if self.streams.is_empty() {
            return self
        }
        let idx = self.app.current_player_index().unwrap();
        // println!("listening for player index {}", idx);
        let Some(stream) = self.streams.get(&idx) else {return self};//&self.streams[idx];
        // println!("got stream...");
        // let Ok(command): Result<Command, Box<dyn std::error::Error>> = read_json_message(stream) else {println!("bad command?"); return Ok(())};
        let Some(Ok(message)): Option<Result<Message, std::io::Error>> = read_json_message_async(stream) else {return self};
        
        match message {
            Message::Command(command) => {
                self.handle_command(command)
            },
            Message::SkipTurn => {
                self.app.current_player_mut().unwrap().skip_turn();

                for (_, s) in self.streams.iter() {
                    write_json_message(s, &Message::SkipTurn);
                };
                self
            }
            _ => {self},
        }
    }
    fn handle_command(mut self, command: Command) -> Self {
        let idx = self.app.current_player_index().unwrap();
        let Some(stream) = self.streams.get(&idx) else {return self};

        // play out the command step-by-step
        // at each step, check which players can observe the command (before executing the step)
        // for each player that observes the command, truncate the command up to where they stop observing
        // after the command has finished playing out, relay the potentially truncated command to all players that have observed some part of it
        // rn there are no steps, so the process is simplified. unit's position is leaked if a unit moves out of or into view, but this might eventually be represented (drawn), or the commands will be reworked to involve steps.

        // do not broadcast the same move to its sender
        // let mut n: Vec<usize> = (0..self.app.players.len()).collect();
        // n.remove(idx);
        let n = 0..self.app.players.len(); //.into_iter()

        let fogs = std::mem::take(&mut self.app.player_fogs);
        let observations = n.map(|i| fogs.get(&i)).map(|maybe_fog| command.get_observed_sections(maybe_fog));
        // execute the move
        self.app.execute_command(&command);

        // tell client move was ok
        // let message = Message::RevealFog(Ok::<World, ServerResponseError>(World::new()));
        // write_json_message(stream, &message);

        // send the observations to clients
        observations.enumerate().for_each(|(idx, obs)| {
            obs.into_iter().for_each(|command| {
                println!("sending {:?} to {}", command, idx);
                write_json_message(self.streams.get(&idx).unwrap(), &Message::Command(command));
            });
        });
        self.app.player_fogs = fogs;
        self
    }
}

/// Define a generic function to read and deserialize JSON messages
fn read_json_message(stream: &TcpStream) -> Result<Message, Box<dyn std::error::Error>> {
    let mut reader = BufReader::new(stream);
    let mut buffer = String::new();

    // Read one line from the stream
    reader.read_line(&mut buffer)?;

    // Deserialize the JSON message
    let response = serde_json::from_str(&buffer.trim_end())?;
    println!("received {}", response);

    Ok(response)
}

// fn read_json_message_async<T>(stream: &TcpStream) -> Option<std::io::Result<Message>>
// where
//     T: serde::de::DeserializeOwned,
// {
fn read_json_message_async(stream: &TcpStream) -> Option<std::io::Result<Message>> {
    // Create a BufReader on the stream
    let mut reader = BufReader::new(stream);
    let mut buffer = String::new();

    // Attempt to read from the stream
    match reader.read_line(&mut buffer) {
        Ok(0) => {
            println!("Connection closed");
            None
        },
        Ok(_) => {
            // Deserialize the JSON message
            match serde_json::from_str(&buffer.trim_end()) {
                Ok(response) => {
                    println!("received {}", response);
                    Some(Ok(response))
                },
                Err(e) => Some(Err(std::io::Error::new(std::io::ErrorKind::InvalidData, e))),
            }
        },
        Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
            // Would block: No data available right now
            None
        },
        Err(e) => {
            // Other errors
            println!("{}", e);
            Some(Err(e))
        }
    }
}

// fn read_json_message_async<T>(stream: &TcpStream) -> Option<std::io::Result<String>>
// where
//     T: serde::de::DeserializeOwned,
// {
//     // Create a BufReader on the stream
//     let mut reader = BufReader::new(stream);
//     let mut buffer = String::new();

//     // Attempt to read from the stream
//     match reader.read_line(&mut buffer) {
//         Ok(0) => {
//             // Connection closed
//             None
//         },
//         Ok(_) => {
//             // Return raw message
//             Some(Ok(buffer))
//         },
//         Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
//             // Would block: No data available right now
//             None
//         },
//         Err(e) => {
//             // Other errors
//             println!("{}", e);
//             Some(Err(e))
//         }
//     }
// }

// fn write_json_message<T: Serialize>(stream: &TcpStream, message: &Message) -> Result<(), Box<dyn std::error::Error>> {
pub fn write_json_message(stream: &TcpStream, message: &Message) -> Result<(), Box<dyn std::error::Error>> {
    println!("sending {}", message);
    let mut writer = BufWriter::new(stream);

    let serialized_message = serde_json::to_string(&message)?;

    writer.write_all(serialized_message.as_bytes())?;
    writer.write_all(b"\n")?;
    writer.flush()?;

    Ok(())
}

// fn write_json_message(stream: &TcpStream, message: Message) -> Result<&'a T, Box<dyn std::error::Error>> {
//     let mut writer = BufWriter::new(stream);

//     let serialized_message = serde_json::to_string(&message)?;

//     writer.write_all(serialized_message.as_bytes())?;
//     writer.write_all(b"\n")?;
//     writer.flush()?;

//     Ok(message)
// }

// #[derive(Serialize, Deserialize)]
// enum Message {
//     NewPlayer{
//         starting_position: Cube<i32>, 
//         player: Player
//     },
//     Initialise{
//         players: Vec<Player>,
//         world: World
//     },
//     RevealFog(World),
//     Command(Command),
//     Result(Result<(), ServerResponseError>),
//     SkipTurn,
//     Chat(usize, String)
// }

// performing an action on the client side sends over a Command or a SkipTurn
// the server replies with the same message which the client parses
// and only then executes it.
#[derive(Serialize, Deserialize)]
pub enum Message {
    NewPlayer {starting_position: Cube<i32>, player: Player},
    Initialise {turn: usize, players: Vec<Player>, world: World},
    Command(Command),
    RevealFog(Result<World, ServerResponseError>),
    SkipTurn,
    Chat {id: usize, message: String},
}

impl core::fmt::Display for Message {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Message::NewPlayer{..} => write!(f, "NewPlayer"),
            Message::Initialise{..} => write!(f, "Initialise"),
            Message::Command(_) => write!(f, "Command"),
            Message::RevealFog(_) => write!(f, "RevealFog"),
            Message::SkipTurn => write!(f, "SkipTurn"),
            Message::Chat{..} => write!(f, "Chat"),
        }
    }
}

impl Client<Game> {
    fn handle_message(&mut self, message: Message) {
        match message {
            Message::NewPlayer { starting_position, player } => {
                self.app.world.gen_capital_at_cube(self.app.players.len(), starting_position);
                self.app.players.push(player);
            },
            Message::Initialise { turn, players, world } => {},
            Message::Command(command) => {
                println!("executing command {:?}", command);
                self.app.execute_command(&command);
                // println!("clicking");
                // self.app.click(&command.from);
                // self.app.click(&command.to);
            },
            Message::RevealFog(result) => {
                match result {
                    Ok(mut world) => self.app.world.extend(world.drain()),
                    Err(e) => panic!("{}", e),
                }
            },
            Message::SkipTurn => {
                self.app.current_player_mut().unwrap().skip_turn();
            },
            Message::Chat { id, message } => {
                todo!()
            },
        }
    }
}

// fn process_message(message: String) {
//     let s = message.trim_end();

//     let types: vec![Box::new(World), Player];

//     // Attempt to deserialize the message into each type and process it
//     for &ty in &types {
//         match serde_json::from_str::<&dyn JsonDeserializable>(message) {
//             Ok(obj) => {
//                 obj.process_message();
//                 return;
//             },
//             Err(_) => continue,
//         }
//     }

//     // Try to deserialize into type T
//     if let Ok(t_msg) = serde_json::from_str::<T>(s) {
//         println!("Received T message: {:?}", t_msg);
//         return;
//     }

//     // Try to deserialize into type U
//     if let Ok(u_msg) = serde_json::from_str::<U>(s) {
//         println!("Received U message: {:?}", u_msg);
//         return;
//     }

//     // Handle case where the message could not be deserialized into any known type
//     println!("Received unknown message type: {}", s);
// }

// impl Game {
//     pub fn new_for_client(self, player_idx: usize) -> &Self {
//         let mut player_fogs = HashMap::new();
//         player_fogs.insert(player_idx, *self.player_fogs.get(&player_idx).unwrap());
//         &Game {
//             turn: self.turn,
//             players: vec![*self.players.get(player_idx).unwrap()],
//             world: self.world,
//             player_fogs: player_fogs,
//             rules: self.rules,
//         }
//     }
// }