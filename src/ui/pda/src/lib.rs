#![allow(non_camel_case_types)]

extern crate hashbrown;

use hashbrown::HashMap;

use std::collections::HashSet;
use std::hash::Hash;
use std::cmp::Eq;
use std::cmp::PartialEq;

#[derive(Debug)]
pub enum Action<Γ> {
    Push(Γ),
    Pop,
    PopAndPush(Γ),
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct TransitionCondition<Q, Σ, Γ>(pub Q, pub Σ, pub Option<Γ>);
#[derive(Hash, PartialEq, Eq)]
pub struct TransitionConditionRef<'a, Q, Σ, Γ>(&'a Q, Σ, Option<&'a Γ>);
#[derive(Debug)]
pub struct TransitionAction<Q, Γ>(pub Q, pub Action<Γ>);

pub type δ<Q, Σ, Γ> = hashbrown::HashMap<
    TransitionCondition<Q, Σ, Γ>, 
    TransitionAction<Q, Γ>,
>;

impl<'a, 'b, Q, Σ, Γ> hashbrown::Equivalent<TransitionCondition<Q, Σ, Γ>> for TransitionConditionRef<'a, Q, Σ, Γ>
where Q: PartialEq, Σ: PartialEq, Γ: PartialEq,{
    fn equivalent(&self, key: &TransitionCondition<Q, Σ, Γ>) -> bool {
        self.0 == &key.0 && self.1 == key.1 && self.2 == key.2.as_ref()
    }
}

/// Q is a finite set of states
/// Σ is the input alphabet
/// Γ is the stack alphabet
/// δ is the transition function
/// https://en.wikipedia.org/wiki/Pushdown_automaton#Formal_definition
pub struct PushdownAutomaton<Q, Σ, Γ> {
    stack: Vec<*const Γ>,
    start_state: *const Q,
    final_states: HashSet<*const Q>,
    transitions: δ<Q, Σ, Γ>,
}

impl<Q, Σ, Γ> PushdownAutomaton<Q, Σ, Γ>
where
    Q: Eq + Hash,
    Σ: Eq + Hash,
    Γ: Eq + Hash,
{
    pub fn new(
        start_state: &Q,
        final_states: HashSet<&Q>,
        transitions: δ<Q, Σ, Γ>,
    ) -> Self {
        let stack = Vec::new();
        let final_states: HashSet<*const Q> = unsafe { core::mem::transmute(final_states) };
        Self {
            stack,
            start_state,
            final_states,
            transitions,
        }
    }

    pub fn transition(&mut self, input_symbol: Σ) -> Result<(), &'static str> {
        let stack_top =self.stack.pop();
        let stack_top_ptr: Option<&Γ> = unsafe { stack_top.map(|r| &*r) };
        let start_state = unsafe { &*self.start_state as &Q };
        let key = TransitionConditionRef(start_state, input_symbol, stack_top_ptr);

        match self.transitions.get(&key) {
            Some(TransitionAction(next_state, Action::Push(symbol))) => {
                if let Some(value) = stack_top {
                    self.stack.push(value as *const Γ); // Push back the original top
                }
                self.stack.push(symbol as *const Γ);
                self.start_state = next_state;
            }
            Some(TransitionAction(next_state, Action::Pop)) => {
                self.start_state = next_state;
            }
            Some(TransitionAction(next_state, Action::PopAndPush(symbol))) => {
                self.stack.push(symbol as *const Γ);
                self.start_state = next_state;
            }
            None => return Err("Error: Transition not defined"),
        };
        Ok(())
    }
    pub fn get_state(&self) -> &Q {
        unsafe { &*self.start_state as &Q }
    }
    pub fn get_stack(&self) -> Vec<&Γ> {
        let mut stack = Vec::with_capacity(self.stack.len());
        unsafe {        
            stack.set_len(self.stack.len());
    
            core::ptr::copy_nonoverlapping(
                self.stack.as_ptr() as *const &Γ,
                stack.as_mut_ptr(),
                self.stack.len(),
            );
    
            stack
        }
    }
}
