use executor::{Executor, Pose};

#[test]
fn should_return_x_plus_2_given_command_is_fm_and_facing_is_e() {
    // given
    let original_pose = Pose::new(0, 0, 'E');
    let mut executor = Executor::with_pose(original_pose);

    // when
    executor.execute("FM");

    // then
    let expected_pose = Pose::new(2, 0, 'E');
    assert_eq!(expected_pose, executor.query());
}

#[test]
fn should_return_n_and_x_plus_1_given_command_is_fl_and_facing_is_e() {
    // given
    let original_pose = Pose::new(0, 0, 'E');
    let mut executor = Executor::with_pose(original_pose);

    // when
    executor.execute("FL");

    // then
    let expected_pose = Pose::new(1, 0, 'N');
    assert_eq!(expected_pose, executor.query());
}

#[test]
fn should_return_s_and_x_plus_1_given_command_is_fr_and_facing_is_e() {
    // given
    let original_pose = Pose::new(0, 0, 'E');
    let mut executor = Executor::with_pose(original_pose);

    // when
    executor.execute("FR");

    // then
    let expected_pose = Pose::new(1, 0, 'S');
    assert_eq!(expected_pose, executor.query());
}

#[test]
fn should_return_y_plus_1_given_command_is_ffm_and_facing_is_n() {
    // given
    let original_pose = Pose::new(0, 0, 'N');
    let mut executor = Executor::with_pose(original_pose);

    // when
    executor.execute("FFM");

    // then
    let expected_pose = Pose::new(0, 1, 'N');
    assert_eq!(expected_pose, executor.query());
}
