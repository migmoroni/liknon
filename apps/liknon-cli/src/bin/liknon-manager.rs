fn main() -> std::process::ExitCode {
    liknon_manager::run_cli_with_version(env!("CARGO_PKG_VERSION"))
}
