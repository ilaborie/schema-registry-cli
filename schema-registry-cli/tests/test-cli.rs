use trycmd::TestCases;

#[test]
fn should_readme_ok() {
    TestCases::new().case("README.md");
}

#[test]
fn should_run_commands() {
    TestCases::new().case("tests/cmd/*.toml");
}

#[test]
fn should_run_command() {
    TestCases::new().case("tests/cmd/subject-register-json.toml");
}
