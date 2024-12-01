#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.pass("tests/ui/basic.rs");
    t.pass("tests/ui/skip_docs.rs");
    t.compile_fail("tests/ui/invalid_struct.rs");
    t.compile_fail("tests/ui/missing_base.rs");
    t.compile_fail("tests/ui/wrong_base_type.rs");
}
