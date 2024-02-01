use crate::cli::DryRunAndCopyFlag;

pub enum RunMode {
    Normal,
    DryRun,
    DryRunAndCopy,
    Copy,
}

pub fn get_run_mode_from_options(flags: DryRunAndCopyFlag) -> RunMode {
    if flags.copy {
        if flags.dry_run {
            RunMode::DryRunAndCopy
        } else {
            RunMode::Copy
        }
    } else {
        if flags.dry_run {
            RunMode::DryRun
        } else {
            RunMode::Normal
        }
    }
}
