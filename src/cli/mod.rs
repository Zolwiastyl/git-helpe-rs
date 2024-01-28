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
}

pub struct UseTemplate {
    pub key: String,
    pub interpolate_values: Vec<String>,
    pub use_autocomplete: bool,
}

pub struct CommitSubcommandFlags {
    pub use_branch_number: bool,
}

pub struct CommitOperationArguments {
    pub use_template: UseTemplate,
    pub flags: CommitSubcommandFlags,
}

pub enum OperationWithArguments {
    Commit(CommitOperationArguments),
    BranchFromClipboard(CheckoutToPrefix),
    BranchFromTemplate(UseTemplate),
    SetCommitFormat(SetFormat),
    SetBranchFormat(SetFormat),
    SetBranchPrefix(SetFormat),
    SetClipboardCommand(String),
    Show,
}

pub struct ParsedArguments {
    pub operation_with_arguments: OperationWithArguments,
    pub path_to_config: PathBuf,
}
