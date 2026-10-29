fn main() -> std::process::ExitCode {
    liknon_validator::run_cli_with_version(env!("CARGO_PKG_VERSION"))
}
