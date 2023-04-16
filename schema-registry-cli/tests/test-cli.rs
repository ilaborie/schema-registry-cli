use trycmd::TestCases;

#[test]
fn should_readme_ok() {
    TestCases::new().case("README.md");
}
