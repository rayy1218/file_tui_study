use crate::inputs::key::Key;

pub mod key;
pub mod event;

pub enum InputEvent {
    Input(Key),
    Tick,
}