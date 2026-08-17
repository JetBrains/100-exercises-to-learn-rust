use warmup_syntax::greeting;

#[test]
fn test_welcome() {
    assert_eq!(greeting(), "I'm ready to learn Rust!");
}
