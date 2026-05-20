use super::state::State;
use crate::Pose;

pub struct Executor {
    pose: Pose,
    state: State,
}

impl Executor {
    pub fn with_pose(pose: Pose) -> Self {
        Executor {
            pose,
            state: State::default(),
        }
    }

    pub fn execute(&mut self, cmds: &str) {
        for cmd in cmds.chars() {
            match cmd {
                'B' => self.state.be_reverse(),
                'F' => self.state.be_fast(),
                _ => {
                    let astions = self.state.assemble(cmd);
                    for action in astions {
                        action.perform(&mut self.pose)
                    }
                }
            }
        }
    }

    pub fn query(&self) -> Pose {
        self.pose
    }
}
