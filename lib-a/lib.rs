#![allow(unused)]

use std::error::Error;
use statig::prelude::*;
use std::fmt::Debug;
use std::io::Write;
use std::ops::Deref;
use serde::Serialize;
use serde::Deserialize;
#[derive(Debug,Serialize, Deserialize ,Default)]
pub struct Blinky;

// The event that will be handled by the state machine.
#[derive(Debug)]
pub enum Event {
    TimerElapsed,
    ButtonPressed,
}

/// The `state_machine` procedural macro generates the `State` and `Superstate`
/// enums by parsing the function signatures with a `state`, `superstate` or
/// `action` attribute. It also implements the `statig::State` and
/// `statig::Superstate` traits.
#[state_machine(
    initial = "State::led_on()",
    state(derive(Debug, Serialize, Deserialize)),
    superstate(derive(Debug)),
    on_transition = "Self::on_transition",
    on_dispatch = "Self::on_dispatch"
)]
impl Blinky {
    /// The `#[state]` attibute marks this as a state handler.  By default the
    /// `event` argument will map to the event handler by the state machine.
    /// Every state must return a `Response<State>`.
    #[state(superstate = "blinking")]
    fn led_on(event: &Event) -> Response<State> {
        match event {
            // When we receive a `TimerElapsed` event we transition to the `led_off` state.
            Event::TimerElapsed => Transition(State::led_off()),
            // Other events are deferred to the superstate, in this case `blinking`.
            _ => Super,
        }
    }

    #[state(superstate = "blinking")]
    fn led_off(event: &Event) -> Response<State> {
        match event {
            Event::TimerElapsed => Transition(State::led_on()),
            _ => Super,
        }
    }

    /// The `#[superstate]` attribute marks this as a superstate handler.
    #[superstate]
    fn blinking(event: &Event) -> Response<State> {
        match event {
            Event::ButtonPressed => Transition(State::not_blinking()),
            _ => Super,
        }
    }

    #[state]
    fn not_blinking(event: &Event) -> Response<State> {
        match event {
            Event::ButtonPressed => Transition(State::led_on()),
            // Altough this state has no superstate, we can still defer the event which
            // will cause the event to be handled by an implicit `top` superstate.
            _ => Super,
        }
    }
}

impl Blinky {
    // The `on_transition` callback that will be called after every transition.
    fn on_transition(&mut self, source: &State, target: &State) {
        println!("transitioned from `{source:?}` to `{target:?}`");
    }

    fn on_dispatch(&mut self, state: StateOrSuperstate<Self>, event: &Event) {
        println!("dispatching `{event:?}` to `{state:?}`");
    }
}


pub fn really_complicated_code_a(a: u8, b: u8) -> Result<u8, Box<dyn Error>> {
    Ok(a + b)
}

pub fn test_blinky() {
    let start = std::time::Instant::now();

    let mut state_machine = Blinky::default().state_machine();

    state_machine.handle(&Event::TimerElapsed);
    state_machine.handle(&Event::ButtonPressed);
    state_machine.handle(&Event::TimerElapsed);
    state_machine.handle(&Event::ButtonPressed);
    let ser = serde_json::to_string(&state_machine).unwrap();


    // let serialize = Serialize::serialize(&state_machine, ()).unwrap();

    let end = std::time::Instant::now();

    println!("Duration: {:?}", end - start);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_01() {
        assert_eq!(2, really_complicated_code_a(1, 1))
    }
}

