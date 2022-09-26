use std::sync::mpsc::{channel, Receiver, RecvError, Sender};
use std::thread;
use std::time::Duration;
use crossterm::event;
use crate::inputs::{InputEvent};
use crate::inputs::key::Key;

pub struct Events {
    rx: Receiver<InputEvent>,
    _tx: Sender<InputEvent>,
}

impl Events {
    pub fn new(tick_rate: Duration) -> Events {
        let (tx, rx) = channel();
        let event_tx = tx.clone();
        thread::spawn(move || {
            loop {
                if event::poll(tick_rate).unwrap() {
                    if let crossterm::event::Event::Key(key) = event::read().unwrap() {
                        let key = Key::from(key);
                        event_tx.send(InputEvent::Input(key)).unwrap();
                    }

                    event_tx.send(InputEvent::Tick).unwrap();
                }
            }
        });

        Events {rx, _tx: tx}
    }

    pub fn next(&self) -> Result<InputEvent, RecvError> {
        self.rx.recv()
    }
}