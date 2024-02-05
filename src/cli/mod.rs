use std::path::PathBuf;

pub mod define;

pub mod map_to_operation;

pub enum Operation {
    Commit,
    BranchFromClipboard,
    BranchFromTemplate,
    SetCommitFormat,
    SetBranchFormat,
    SetBranchPrefix,
    Show,
}

pub struct SetFormat {
    pub key: String,
    pub value: String,
}

pub struct CheckoutToPrefix {
    pub prefix_key: String,
    pub copy: bool,
    pub dry_run: bool,
}

pub struct DryRunAndCopyFlag {
    pub dry_run: bool,
    pub copy: bool,
}

pub struct UseTemplate {
    pub key: String,
    pub interpolate_values: Vec<String>,
    pub use_autocomplete: bool,
    pub copy: bool,
    pub dry_run: bool,
}

pub struct CommitSubcommandFlags {
    pub use_branch_number: bool,
    pub copy: bool,
    pub dry_run: bool,
}

pub struct CommitOperationArguments {
    pub use_template: UseTemplate,
    pub flags: CommitSubcommandFlags,
}
pub struct SetClipboardCommands {
    pub copy: String,
    pub paste: String,
}

pub enum OperationWithArguments {
    Commit(CommitOperationArguments),
    BranchFromClipboard(CheckoutToPrefix),
    BranchFromTemplate(UseTemplate),
    SetCommitFormat(SetFormat),
    SetBranchFormat(SetFormat),
    SetBranchPrefix(SetFormat),
    SetClipboardCommands(SetClipboardCommands),
    Show,
    GenerateAutocompletionScript(PathBuf),
}

pub struct ParsedArguments {
    pub operation_with_arguments: OperationWithArguments,
    pub path_to_config: PathBuf,
}
