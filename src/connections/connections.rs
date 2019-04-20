use std::collections::HashMap;
use std::collections::hash_map::IterMut;

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

    pub fn react(&mut self, reaction: Reaction<T>) -> Option<&mut T> {
        use Reaction::*;
        match reaction {
            Value(reactor) => {
                self.reactors.insert(reactor.token(), reactor);
                None
            }
            Event(event) => {
                if let Some(reactor) = self.reactors.get_mut(&event.token()) {
                    reactor.stream_mut().react(event.into());
                    Some(reactor)
                } else {
                    None
                }
            }
            Continue => None,
        }
    }
}

