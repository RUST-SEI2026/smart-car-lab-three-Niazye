use executor::{Executor, Pose};

#[test]
fn should_return_x_minus_1_given_command_is_bm_and_facing_is_e() {
    // given
    let original_pose = Pose::new(0, 0, 'E');
    let mut executor = Executor::with_pose(original_pose);

    // when
    executor.execute("BM");

    // then
    let expected_pose = Pose::new(-1, 0, 'E');
    assert_eq!(expected_pose, executor.query());
}

#[test]
fn should_return_s_given_command_is_bl_and_facing_is_e() {
    // given
    let original_pose = Pose::new(0, 0, 'E');
    let mut executor = Executor::with_pose(original_pose);

    // when
    executor.execute("BL");

    // then
    let expected_pose = Pose::new(0, 0, 'S');
    assert_eq!(expected_pose, executor.query());
}

#[test]
fn should_return_n_given_command_is_br_and_facing_is_e() {
    // given
    let original_pose = Pose::new(0, 0, 'E');
    let mut executor = Executor::with_pose(original_pose);

    // when
    executor.execute("BR");

    // then
    let expected_pose = Pose::new(0, 0, 'N');
    assert_eq!(expected_pose, executor.query());
}

#[test]
fn should_return_y_plus_1_given_command_is_bbm_and_facing_is_n() {
    // given
    let original_pose = Pose::new(0, 0, 'N');
    let mut executor = Executor::with_pose(original_pose);

    // when
    executor.execute("BBM");

    // then
    let expected_pose = Pose::new(0, 1, 'N');
    assert_eq!(expected_pose, executor.query());
}
