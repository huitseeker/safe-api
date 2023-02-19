mod sponge_instance;

#[test]
fn illegal_api_uses() {
    let t = trybuild::TestCases::new();
    t.compile_fail("src/unit_tests/compilation/*.rs");
}
