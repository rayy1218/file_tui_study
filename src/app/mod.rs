use log::{debug};
use actions::Action;
use crate::app::state::AppState::Initialized;
use crate::app::state::File;

use self::actions::Actions;
use self::state::AppState;

pub mod actions;
pub mod state;
pub mod ui;

#[derive(Debug, PartialEq, Eq)]
pub enum AppReturn {
    Exit,
    Continue,
}

/// The main application, containing the state
pub struct App {
    actions: Actions,
    /// State
    state: AppState,
}

impl App {
    pub fn new() -> Self {
        let actions = vec![
            Action::Quit,
            Action::Increment,
            Action::Decrement,
            Action::Select,
            Action::Back,
            Action::Forward,
            Action::ToggleLog,
            Action::ToggleHelp,
        ].into();
        let state = AppState::initialized();
        Self { actions, state }
    }

    pub fn do_action(&mut self, key: crate::inputs::key::Key) -> AppReturn {
        if let Some(action) = self.actions.find(key) {
            debug!("Run action [{:?}]", action);
            match action {
                Action::Quit => AppReturn::Exit,
                Action::Decrement => {
                    match &mut self.state {
                        Initialized { current_list, .. } => {
                            current_list.previous();
                        }
                        _ => {}
                    }
                    AppReturn::Continue
                },
                Action::Increment => {
                    match &mut self.state {
                        Initialized { current_list, .. } => {
                            current_list.next();
                        }
                        _ => {}
                    }
                    AppReturn::Continue
                },
                Action::Select => {
                    match &mut self.state {
                        Initialized { current_list, .. } => {
                            let selected:&File = current_list.items.get(current_list.index()).unwrap();
                            debug!("Select Item: {}", selected.name.to_str().unwrap());
                        }
                        _ => {}
                    }

                    AppReturn::Continue
                },
                Action::Back => {
                    self.state.read_parent();

                    AppReturn::Continue
                },
                Action::Forward => {
                    self.state.read_dir();

                    AppReturn::Continue
                }
                Action::ToggleLog => {
                    self.state.toggle_log();

                    AppReturn::Continue
                }
                Action::ToggleHelp => {
                    self.state.toggle_help();

                    AppReturn::Continue
                }
            }
        }
        else {
            AppReturn::Continue
        }
    }

    pub fn update_on_tick(&mut self) -> AppReturn {
        AppReturn::Continue
    }

    pub fn state(&self) -> &AppState {
        &self.state
    }
}