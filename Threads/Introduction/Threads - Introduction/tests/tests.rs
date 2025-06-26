use task_threads_intro::intro;

#[test]
fn test_intro() {
    assert_eq!(
        intro(),
        "I'm ready to build a concurrent ticket management system!"
    );
}
