use migration_helpers::common_migrations::AddSettingsMigration;
use migration_helpers::{migrate, Result};
use std::process;

// We added new kubernetes settings to reserve pids for kubernetes and system components.
fn run() -> Result<()> {
    migrate(AddSettingsMigration(&[
        "settings.kubernetes.kube-reserved.pid",
        "settings.kubernetes.system-reserved.pid",
    ]))
}

// Returning a Result from main makes it print a Debug representation of the error, but with Snafu
// we have nice Display representations of the error, so we wrap "main" (run) and print any error.
// https://github.com/shepmaster/snafu/issues/110
fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
        process::exit(1);
    }
}
