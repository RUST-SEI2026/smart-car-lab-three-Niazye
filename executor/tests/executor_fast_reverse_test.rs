use executor::{Executor, Pose};

#[test]
fn should_return_x_minus_2_given_command_is_fbm_and_facing_is_e() {
    // given
    let original_pose = Pose::new(0, 0, 'E');
    let mut executor = Executor::with_pose(original_pose);

    // when
    executor.execute("FBM");

    // then
    let expected_pose = Pose::new(-2, 0, 'E');
    assert_eq!(expected_pose, executor.query());
}

#[test]
fn should_return_s_and_x_minus_1_given_command_is_fbl_and_facing_is_e() {
    // given
    let original_pose = Pose::new(0, 0, 'E');
    let mut executor = Executor::with_pose(original_pose);

    // when
    executor.execute("FBL");

    // then
    let expected_pose = Pose::new(-1, 0, 'S');
    assert_eq!(expected_pose, executor.query());
}

#[test]
fn should_return_n_and_x_minus_1_given_command_is_fbr_and_facing_is_e() {
    // given
    let original_pose = Pose::new(0, 0, 'E');
    let mut executor = Executor::with_pose(original_pose);

    // when
    executor.execute("FBR");

    // then
    let expected_pose = Pose::new(-1, 0, 'N');
    assert_eq!(expected_pose, executor.query());
}
