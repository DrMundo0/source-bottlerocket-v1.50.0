use migration_helpers::common_migrations::{AddPrefixSuffixMigration, PrefixSuffix};
use migration_helpers::{migrate, Result};
use std::process;

// We added new settings container-runtime-plugins
fn run() -> Result<()> {
    migrate(AddPrefixSuffixMigration(vec![
        PrefixSuffix {
            prefix: "settings.host-containers",
            suffix: "command",
        },
        PrefixSuffix {
            prefix: "settings.bootstrap-containers",
            suffix: "command",
        },
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
