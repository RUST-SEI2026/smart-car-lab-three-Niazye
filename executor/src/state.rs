use crate::action::Action;

pub trait Assembler {
    fn assemble(&self, cmd: char) -> Vec<Action> {
        match cmd {
            'M' => self.move_assemble(),
            'L' => self.turn_left_assemble(),
            'R' => self.turn_right_assemble(),
            _ => Vec::new(),
        }
    }
    fn move_assemble(&self) -> Vec<Action>;
    fn turn_left_assemble(&self) -> Vec<Action>;
    fn turn_right_assemble(&self) -> Vec<Action>;
}

#[derive(Default, Copy, Clone)]
pub(crate) struct State {
    is_reverse: bool,
    is_fast: bool,
}

impl Assembler for State {
    fn move_assemble(&self) -> Vec<Action> {
        let mut actions = Vec::new();
        actions.push(Action::Forward(if self.is_reverse { -1 } else { 1 }));
        if self.is_fast {
            actions.push(Action::Forward(if self.is_reverse { -1 } else { 1 }));
        }
        actions
    }

    fn turn_left_assemble(&self) -> Vec<Action> {
        let mut actions = Vec::new();
        if self.is_fast {
            actions.push(Action::Forward(if self.is_reverse { -1 } else { 1 }));
        }
        actions.push(if self.is_reverse {
            Action::TurnRight
        } else {
            Action::TurnLeft
        });
        actions
    }

    fn turn_right_assemble(&self) -> Vec<Action> {
        let mut actions = Vec::new();
        if self.is_fast {
            actions.push(Action::Forward(if self.is_reverse { -1 } else { 1 }));
        }
        actions.push(if self.is_reverse {
            Action::TurnLeft
        } else {
            Action::TurnRight
        });
        actions
    }
}

impl State {
    pub(crate) fn toggle_reverse(&mut self) {
        self.is_reverse = !self.is_reverse;
    }

    pub(crate) fn toggle_fast(&mut self) {
        self.is_fast = !self.is_fast;
    }
}
