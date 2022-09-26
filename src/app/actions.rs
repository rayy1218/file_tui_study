use std::collections::HashMap;
use std::fmt;
use std::fmt::Display;
use std::slice::Iter;
use crate::inputs;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Action {
    Quit,
    Increment,
    Decrement,
    Select,
    Back,
    Forward,
    ToggleLog,
    ToggleHelp,
}

impl Action {
    pub fn iterator() -> Iter<'static, Action> {
        static ACTIONS: [Action; 8] = [
            Action::Quit,
            Action::Increment,
            Action::Decrement,
            Action::Select,
            Action::Back,
            Action::Forward,
            Action::ToggleLog,
            Action::ToggleHelp,
        ];
        ACTIONS.iter()
    }

    pub fn keys(&self) -> &[inputs::key::Key] {
        match self {
            Action::Quit => &[inputs::key::Key::Ctrl('c'), inputs::key::Key::Char('q')],
            Action::Increment => &[inputs::key::Key::Down],
            Action::Decrement => &[inputs::key::Key::Up],
            Action::Select => &[inputs::key::Key::Enter],
            Action::Back => &[inputs::key::Key::Left],
            Action::Forward => &[inputs::key::Key::Right],
            Action::ToggleLog => &[inputs::key::Key::Char('D')],
            Action::ToggleHelp => &[inputs::key::Key::Char('?')],
        }
    }
}

impl Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match self {
            Action::Quit => "Quit",
            Action::Increment => "Select Next",
            Action::Decrement => "Select Previous",
            Action::Select => "Select",
            Action::Back => "Cursor Go To Parent",
            Action::Forward => "Cursor Go To Selected Directory",
            Action::ToggleLog => "Toggle Log",
            Action::ToggleHelp => "Toggle Help",
        };
        write!(f, "{}", str)
    }
}

#[derive(Default, Debug, Clone)]
pub struct Actions(Vec<Action>);

impl Actions {
    pub fn find(&self, key: inputs::key::Key) -> Option<&Action> {
        Action::iterator()
            .filter(|action| self.0.contains(action))
            .find(|action| action.keys().contains(&key))
    }

    pub fn actions(&self) -> &[Action] {
        self.0.as_slice()
    }
}

impl From<Vec<Action>> for Actions {
    fn from(actions: Vec<Action>) -> Self {
        let mut map: HashMap<inputs::key::Key, Vec<Action>> = HashMap::new();
        for action in actions.iter() {
            for key in action.keys().iter() {
                match map.get_mut(key) {
                    Some(vec) => vec.push(*action),
                    None => {
                        map.insert(*key, vec![*action]);
                    }
                }
            }
        }
        let errors = map
            .iter()
            .filter(|(_, actions)| actions.len() > 1) // at least two actions share same shortcut
            .map(|(key, actions)| {
                let actions = actions
                    .iter()
                    .map(Action::to_string)
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("Conflict key {} with actions {}", key, actions)
            })
            .collect::<Vec<_>>();
        if !errors.is_empty() {
            panic!("{}", errors.join("; "))
        }

        // Ok, we can create contextual actions
        Self(actions)
    }
}