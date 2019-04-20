use std::collections::hash_map::IterMut;
use std::collections::HashMap;

use sonr::net::stream::StreamRef;
use sonr::prelude::*;

pub struct Connections<T> {
    reactors: HashMap<Token, T>,
}

impl<T> Connections<T> {
    pub fn new() -> Self {
        Self {
            reactors: HashMap::new(),
        }
    }

    pub fn remove(&mut self, token: Token) -> Option<T> {
        self.reactors.remove(&token)
    }

    pub fn iter_mut<'a>(&'a mut self) -> IterMut<'a, Token, T> {
        self.reactors.iter_mut()
    }
}

impl<T: StreamRef> Connections<T> {
    pub fn insert(&mut self, reactor: T) {
        self.reactors.insert(reactor.token(), reactor);
    }

    /// Returns a reference and isn't a real reactor
    pub fn inner_react(&mut self, reaction: Reaction<T>) -> Reaction<&mut T> {
        use Reaction::*;
        match reaction {
            Value(reactor) => {
                self.reactors.insert(reactor.token(), reactor);
                Continue
            }
            Event(event) => {
                if let Some(reactor) = self.reactors.get_mut(&event.token()) {
                    reactor.stream_mut().react(event.into());
                    Value(reactor)
                } else {
                    event.into()
                }
            }
            Continue => Continue,
        }
    }
}
