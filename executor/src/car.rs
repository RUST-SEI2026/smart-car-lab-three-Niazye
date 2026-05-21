pub trait Car {
    fn with_pose(pose: Pose) -> Self;
    fn execute(&mut self, cmds: &str);
    fn query(&self) -> Pose;
}

pub struct SportCar {
    state: State,
}
