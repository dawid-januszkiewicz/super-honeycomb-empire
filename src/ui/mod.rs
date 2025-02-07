use core::panic;
use std::collections::HashSet;
use dsl::pda;
use hashbrown::HashMap;
// use pda::*;
use pda::PushdownAutomaton;
use strum::IntoEnumIterator;

use crate::game::VictoryCondition;
use crate::rules::Ruleset;
use crate::world::Player;
use crate::{next_frame, vec2, Vec2, FONT};

use iced_macroquad::{Interface};
use iced_macroquad::iced::{Length, Theme};
use iced_macroquad::iced::{font, font::Font};
use iced_macroquad::iced::widget::{Button, Row, Column, Text, Container, Checkbox};
use iced_macroquad::iced::widget::{button, row, column, text, center, checkbox};

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
    Map => Single,
    Main => Multi => Host => Game,
    Main => Multi => Join => Lobby,
    Main => Settings,
}

#[derive(Debug, Clone)]
enum Message {
    Transition(Input),
    VictoryConditionNext,
    VictoryConditionPrev,
    FogOfWarToggled,
    Exit,
}

// #[derive(Default)]
pub struct App {
    menu: PushdownAutomaton<State, Input, State>,
    players: Vec<Player>,
    victory_condition: VictoryCondition,
    fog_of_war: bool,
    exit: bool,
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

pub async fn main_menu() -> (bool, App) {
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
                .push(get_main_button("Play", Message::Transition(Input::ToGame)))
                .spacing(20)
            ).into(), //.center(Length::Fill)

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
