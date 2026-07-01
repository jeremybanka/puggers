use std::process::ExitCode;

fn main() -> ExitCode {
    match puggers::run_from_env() {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("{message}");
            ExitCode::FAILURE
        }
    }
}
