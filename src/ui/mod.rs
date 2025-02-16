use core::panic;
use std::collections::HashSet;
use dsl::pda;
use hashbrown::HashMap;
// use iced_macroquad::iced::raw::Element;
// use pda::*;
use pda::PushdownAutomaton;
use strum::IntoEnumIterator;

use crate::game::{Game, VictoryCondition};
use crate::mquad::Assets;
use crate::network::{ChatClient, ChatMsg, ChatServer, Client, Component, NullEndpoint, Server};
use crate::rules::Ruleset;
use crate::world::Player;
use crate::{next_frame, vec2, Vec2, FONT};

use iced_macroquad::{Interface};
use iced_macroquad::iced::{Element, Length, Theme};
use iced_macroquad::iced::{font, font::Font};
use iced_macroquad::iced::widget::{Button, Checkbox, Column, Container, Renderer, Row, Text};
use iced_macroquad::iced::widget::{button, row, column, text, center, checkbox, text_input, scrollable};

use macroquad::prelude::*;

// use macroquad::ui::{hash, root_ui};
// use macroquad::ui::widgets;
// use macroquad::ui::Skin;
// use macroquad::ui::widgets::Group;
// use macroquad::prelude::Color;
// use macroquad::prelude::RectOffset;
// use macroquad::prelude::load_ttf_font;

// use iced::{
//     Element, Application, Settings as IcedSettings, Length
// };

// use macroquad::prelude::*;

// use iced_macroquad::iced;
// extern crate glow;
// use iced_glow::{Renderer, Application, Settings};

// struct UI;

// impl Application for UI {
//     type Executor = iced_native::executor::Null;
//     type Message = (); // No messages for now
//     type Theme = iced_native::theme::Theme;
//     type Flags = ();

//     fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
//         (Self, Command::none())
//     }

//     fn title(&self) -> String {
//         "Iced in Macroquad".to_string()
//     }

//     fn update(&mut self, _message: Self::Message, _clipboard: &mut dyn Clipboard) -> Command<Self::Message> {
//         Command::none()
//     }

//     fn view(&self) -> iced_native::Element<'_, Self::Message, Renderer> {
//         use iced_native::widget::{column, text};
//         column![text("Hello, Iced in Macroquad!")].into()
//     }
// }

const FONT_HANDLE: Font = Font::with_name("Iceberg");

pda! {
    Main => Single => Game,
            Single => Map,

    Main => Multi => Lobby => Game,
                     Lobby => Map,

    Main => Settings,
}

#[derive(Debug, Clone)]
enum EndpointType {
    Null,
    Client,
    Server,
}

#[derive(Debug, Clone)]
enum Message {
    Transition(Input),
    IpAddressChanged(String),
    ChatMessageChanged(String),
    SendChatMessage,
    TransitionAndSetEndpoint(Input, EndpointType),
    VictoryConditionNext,
    VictoryConditionPrev,
    FogOfWarToggled,
    Exit,
}

// #[derive(Default)]
pub struct App {
    menu: PushdownAutomaton<State, Input, State>,
    ip_address: String,
    endpoint: Endpoint,
    chat_message: String,
    players: Vec<Player>,
    victory_condition: VictoryCondition,
    fog_of_war: bool,
    exit: bool,
}

pub enum Endpoint {
    Client(Client),
    Server(Server),
    Offline(),
}

impl Endpoint {
    fn get_chatlog(&self) -> Vec<&ChatMsg> {
        match self {
            Endpoint::Client(c) => c.chatlog.iter().collect(),
            Endpoint::Server(s) => s.chatlog.iter().collect(),
            Endpoint::Offline() => panic!(),
        }
    }
}

impl Endpoint {
    fn send_chat_message(&mut self, msg: String) {
        match self {
            Endpoint::Client(c) => c.send_chat_message(msg),
            Endpoint::Server(s) => s.send_chat_message(msg),
            Endpoint::Offline() => panic!(),
        };
    }
}

impl From<App> for Ruleset {
    fn from(app: App) -> Self {
        Self::default(app.victory_condition, &app.players)
    }
}

fn update(app: &mut App, message: Message) {
    match message {
        Message::Transition(input) => {
            app.menu.transition(input);
        }
        Message::IpAddressChanged(addr) => {
            app.ip_address = addr;
        }
        Message::ChatMessageChanged(msg) => {
            println!("chat message changed!!!: `{}`", msg);
            app.chat_message = msg
        }
        Message::SendChatMessage => {
            let msg = std::mem::take(&mut app.chat_message);
            app.endpoint.send_chat_message(msg);
        }
        Message::TransitionAndSetEndpoint(input, endpoint) => {
            match app.endpoint {
                Endpoint::Client(_) | Endpoint::Server(_) => return,
                _ => {}
            }
            // let component = T::empty();
            app.endpoint = match endpoint {
                EndpointType::Null => todo!(), //Endpoint::Offline(NullEndpoint::new(None)),
                EndpointType::Client => Endpoint::Client(Client::new(&app.ip_address).unwrap()),
                EndpointType::Server => Endpoint::Server(Server::new(&app.ip_address).unwrap()),
            };
            app.menu.transition(input);
        }
        Message::VictoryConditionNext => {
            let i = VictoryCondition::iter().position(|vc| vc == app.victory_condition).unwrap();
            app.victory_condition = VictoryCondition::iter().cycle().nth(i+1).unwrap();
        }
        Message::VictoryConditionPrev => {
            let i = VictoryCondition::iter().rev().position(|vc| vc == app.victory_condition).unwrap();
            app.victory_condition = VictoryCondition::iter().rev().cycle().nth(i+1).unwrap();
            //app.victory_condition = VictoryCondition::iter().rev().cycle().skip_while(|vc| *vc == app.victory_condition).next().unwrap();
            // let n = VictoryCondition::iter().skip_while(|vc| *vc == app.victory_condition).count();
            // let n = if n == 0 {VictoryCondition::iter().len()} else {n - 1};
            // app.victory_condition = VictoryCondition::iter().skip(n).next().unwrap();
        }
        Message::FogOfWarToggled => {
            app.fog_of_war = !app.fog_of_war;
        }
        Message::Exit => {
            app.exit = true;
        }
    }
}

impl App {
    fn new() -> Self {
        let transitions = generate_transitions();
        let start_state = &State::Main;
        let final_states = HashSet::new();
        let mut pda = PushdownAutomaton::new(start_state, final_states, transitions);
        Self {
            menu: pda,
            ip_address: "127.0.0.1:8000".into(),
            endpoint: Endpoint::Offline(),//Box::new(crate::network::NullEndpoint::new(Game::empty())),
            chat_message: "".into(),
            players: vec![],
            victory_condition: VictoryCondition::Elimination,
            fog_of_war: false,
            exit: false,
        }
    }
}

fn get_main_button(display_text: &str, message: Message) -> Button<Message> {
    Button::new(Text::new(display_text).size(64).center().font(FONT_HANDLE))
        .on_press(message)
        .width(Length::Fixed(400.))
}

pub async fn main_menu(assets: &mut Assets) -> (bool, App) {
    let mut state = App::new();
    let mut interface = Interface::<Message>::new();
    // interface.set_theme(iced::Theme::Oxocarbon);
    let mut messages = Vec::new();

    font::load(vec![FONT.into()]);

    while !state.exit {
        // poll
        if is_key_pressed(KeyCode::Escape) {
            messages.push(Message::Transition(Input::Back));
        }

        for message in messages.drain(..) {
            update(&mut state, message);
        }

        clear_background(LIGHTGRAY);

        let ui = match state.menu.get_state() {
            State::Main => center(column!()
                .push(get_main_button("Play", Message::Transition(Input::ToSingle)))
                .push(get_main_button("Multiplayer", Message::Transition(Input::ToMulti)))
                .push(get_main_button("Settings", Message::Transition(Input::ToSettings)))
                .push(get_main_button("Exit", Message::Exit))
                .spacing(20)
            ).into(),

            State::Single => center(column!()
                .push(button(text("Choose Map").size(64).font(FONT_HANDLE).center()).on_press(Message::Transition(Input::ToMap)).width(900).height(900))
                .push(checkbox("Fog of War", state.fog_of_war).on_toggle(|_| Message::FogOfWarToggled).font(FONT_HANDLE).text_size(64).size(64))
                .push(row!(
                    text("Victory Condition:").size(64).font(FONT_HANDLE),
                    button(text("<").size(64).font(FONT_HANDLE)).on_press(Message::VictoryConditionPrev),
                    text(state.victory_condition.to_string()).size(64).font(FONT_HANDLE),
                    button(text(">").size(64).font(FONT_HANDLE)).on_press(Message::VictoryConditionNext),
                ).spacing(20))
                .push(get_main_button("Play", Message::TransitionAndSetEndpoint(Input::ToGame, EndpointType::Null)))
                .spacing(20)
            ).into(), //.center(Length::Fill)

            State::Multi => center(column!()
                .push(text_input(&format!("Address: {}", "127.0.0.1:8000"), &state.ip_address)
                        .size(64)
                        .font(FONT_HANDLE)
                        .width(Length::Fixed(820.))
                        .on_input(Message::IpAddressChanged))
                .push(row!()
                    .push(get_main_button("Join", Message::TransitionAndSetEndpoint(Input::ToLobby, EndpointType::Client)))
                    .push(get_main_button("Host", Message::TransitionAndSetEndpoint(Input::ToLobby, EndpointType::Server)))
                    .spacing(20)
                )
                .spacing(20)
                ).into(),

            State::Lobby => {
                let chatlog: Vec<&ChatMsg> = state.endpoint.get_chatlog();//state.endpoint.get_chatlog();
                let chatlog_: Vec<String> = chatlog.into_iter().map(|msg| msg.to_string()).collect();
                let slog: Vec<Text> = chatlog_.into_iter().map(|msg| Text::new(msg.clone()).into()).collect();
                let elog = slog.into_iter().map(|t| <Text<'_, Theme, Renderer> as Into<Element<Message, Theme>>>::into(t));
                // let elog = slog.iter().map(|t| t.into());

                let chat_column = Column::with_children(
                    // chatlog.iter().map(|msg| Element::from(Text::from(msg.to_string().as_str()))).collect::<Vec<Element<_, _>>>()
                    elog
                    // todo!()
                );
            center(column!()
                .push(scrollable(chat_column))
                .push(text_input("press ENTER to send", &state.chat_message)
                        .on_input(Message::ChatMessageChanged)
                        .on_submit(Message::SendChatMessage))
                
            ).into()},

            State::Game => {
                break
            }
            _ => {panic!("menu panic");}
        };

        interface.view(&mut messages, ui);

        next_frame().await
    }

    (state.exit, state)
}

use rust_fsm::*;


pub fn test_ui() {
    pda! {
        Main => Single => Game,
        Main => Multi => Host => Game,
        Main => Multi => Join => Lobby,
        Main => Settings,
    }

    let transitions = generate_transitions();
    println!("{:?}", transitions);

    let start_state = &State::Main;
    let final_states = HashSet::new();
    let mut pda = PushdownAutomaton::new(start_state, final_states, transitions);


    // Test transitions
    println!("0 {:?}", pda.get_state());
    assert_eq!(pda.transition(Input::ToSingle), Ok(()));
    println!("1 {:?}", pda.get_state());
    assert_eq!(pda.transition(Input::ToGame), Ok(()));
    println!("2 {:?}", pda.get_state());
    assert_eq!(pda.transition(Input::Back), Ok(()));
    println!("3 {:?}", pda.get_state());
    assert_eq!(pda.transition(Input::Back), Ok(()));
    println!("4 {:?}", pda.get_state());
    assert_eq!(pda.transition(Input::ToMulti), Ok(()));
    println!("5 {:?}", pda.get_state());
    assert_eq!(pda.transition(Input::ToHost), Ok(()));
    println!("5 {:?}", pda.get_state());
    assert_eq!(pda.transition(Input::ToGame), Ok(()));
    println!("7 {:?}", pda.get_state());
    assert_eq!(pda.transition(Input::Back), Ok(()));
    println!("8 {:?}", pda.get_state());
    assert_eq!(pda.transition(Input::Back), Ok(()));
    println!("9 {:?}", pda.get_state());
    // assert_eq!(pda.transition(Input::ToSettings), Ok(()));
    // println!("10 {:?}", pda.get_state());
}
