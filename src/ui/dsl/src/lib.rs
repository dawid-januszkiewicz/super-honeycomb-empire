extern crate hashbrown;
extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, punctuated::Punctuated, parse::Parse, parse::ParseStream, Ident, Token};
use std::collections::HashSet;
// use hashbrown::HashMap;

#[proc_macro]
pub fn pda(input: TokenStream) -> TokenStream {
    let transitions = parse_macro_input!(input as Transitions);

    let mut state_set = HashSet::new();
    let mut input_set = HashSet::new();

    let mut transition_entries = Vec::new();

    for transition in transitions.transitions {
        let mut current_state: Option<Ident> = None;
        let mut prev_state: Option<Ident> = None;
        
        for (i, next_state) in transition.states.iter().enumerate() {
            state_set.insert(next_state.clone());
            let input = Ident::new(&format!("To{}", next_state), next_state.span());
            input_set.insert(input.clone());

            if i == 0 {
                current_state = Some(next_state.clone());
                continue
            }

            let prev_state_option = if i == 1 {
                quote! { None }
            } else {
                quote! { Some(State::#prev_state) }
            };

            if let Some(state) = current_state.clone() {
                transition_entries.push(quote! {
                    transitions.insert(
                        pda::TransitionCondition(State::#state, Input::#input, #prev_state_option),
                        pda::TransitionAction(State::#next_state, pda::Action::Push(State::#state))
                    );
                    transitions.insert(
                        pda::TransitionCondition(State::#next_state, Input::Back, Some(State::#state)),
                        pda::TransitionAction(State::#state, pda::Action::Pop)
                    );
                });
                // if let Some(prev) = prev_state {
                //     transition_entries.push(quote! {
                //         transitions.insert(
                //             crate::TransitionCondition(State::#next_state, Input::Back, Some(State::#state)),
                //             crate::TransitionAction(State::#state, crate::Action::Pop)
                //         );
                //     });
                // };
            };

            // if i == 1 {
            //     transition_entries.push(quote! {
            //         transitions.insert(
            //             crate::TransitionCondition(State::#state, Input::#input, None),
            //             crate::TransitionAction(State::#next_state, crate::Action::Push(State::#state))
            //         );
            //         transitions.insert(
            //             crate::TransitionCondition(State::#state, Input::Back, None),
            //             crate::TransitionAction(State::#prev_state, crate::Action::Pop)
            //         );
            //     });
            // } else {
            //     transition_entries.push(quote! {
            //         transitions.insert(
            //             crate::TransitionCondition(State::#state, Input::#input, Some(State::#prev_state)),
            //             crate::TransitionAction(State::#next_state, crate::Action::Push(State::#state))
            //         );
            //         transitions.insert(
            //             crate::TransitionCondition(State::#state, Input::Back, Some(State::#prev_state)),
            //             crate::TransitionAction(State::#prev_state, crate::Action::Pop)
            //         );
            //     });
            // }

            prev_state = current_state;
            current_state = Some(next_state.clone());
        }
    }

    let state_enum_variants: Vec<_> = state_set.iter().map(|state| {
        quote! { #state }
    }).collect();

    let input_enum_variants: Vec<_> = input_set.iter().map(|input| {
        quote! { #input }
    }).collect();

    let expanded = quote! {
        #[derive(Hash, PartialEq, Eq, Clone, Debug)]
        pub enum State {
            #(#state_enum_variants,)*
        }

        #[derive(Hash, PartialEq, Eq, Clone, Debug)]
        pub enum Input {
            #(#input_enum_variants,)*
            Back,
        }

        pub fn generate_transitions() -> hashbrown::HashMap<pda::TransitionCondition<State, Input, State>, pda::TransitionAction<State, State>> {
            let mut transitions = hashbrown::HashMap::new();
            #(#transition_entries)*
            transitions
        }
    };

    TokenStream::from(expanded)
}

struct Transition {
    states: Punctuated<Ident, Token![=>]>,
}

impl Parse for Transition {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let states = Punctuated::parse_separated_nonempty(input)?;
        Ok(Transition { states })
    }
}

struct Transitions {
    transitions: Punctuated<Transition, Token![,]>,
}

impl Parse for Transitions {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let transitions = Punctuated::parse_terminated(input)?;
        Ok(Transitions { transitions })
    }
}
