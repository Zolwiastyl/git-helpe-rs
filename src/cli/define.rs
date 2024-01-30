use clap::{Arg, ArgAction, Command};

pub fn build_cli_commands() -> Command {
    Command::new("")
        .arg(Arg::new("config").required(false).short('c'))
        .subcommand(
            Command::new("set-branch-prefix")
                .arg(Arg::new("key").required(false))
                .arg(Arg::new("prefix").required(false))
                .about("Set prefix for checkout using clipboard contents")
                .after_help(
                    "Set branch prefix under given key so then you can use it when\n\
                running \n\
                git-helpe-rs bp <key> \n\
                to checkout to a branch name based on contents of your clipboard\n\
                ",
                ),
        )
        .subcommand(
            Command::new("set-branch-template")
                .arg(Arg::new("key").required(true))
                .arg(Arg::new("template").required(true))
                .after_help(
                    "Set branch template under given key so then you can use it when running \n\
                    git-helpe-rs bf ...args_to_interpolate \n\
                    branch template will be interpolated where {} occurs. \n\
                    For example \n\
                    git-helpe-rs set-branch-template fu feature-{}/utils-{} \n\
                    git-helpe-rs bf fu 123 'new cli for stuff automation' \n\
                    will checkout in the following way: \n\
                    git checkout -b feature-123/utils-new-cli-for-stuff-automation \n\
                    you try anything funky with args to interpolate on your own \n\
                    ",
                )
                .about("Set template that can be used when switching branches"),
        )
        .subcommand(
            Command::new("bp")
                .arg(Arg::new("prefix").required(true))
                .about("Check you out to a branch using your clipboard contents")
                .after_help(
                    "Will use contents of your clipboard to checkout you to\n\
                    branch with prefix that's under the given key. \n\
                     Valid clipboard contents when calling this command looks like this: \n\
                    git checkout -b name-of-your-branch \n\
                    If there is a valid content in your clipboard \n\
                    after running following commands: \n\n\
                    git-helpe-rs set-branch-prefix f 'feature/' \n\
                    git-helpe-rs bp \n\n\
                    you will be checked out like this \n\
                    git checkout -b feature/name-of-your-branch
                    ",
                )
                .add_copy_flag(),
        )
        .subcommand(
            Command::new("bt")
                .arg(Arg::new("interpolate_values").required(true).num_args(0..))
                .arg(Arg::new("key").short('k').required(false).help(
                    "Specify which template you want to use \n\
                    if you omit this param default template will be used",
                ))
                .about("Check out to a branch based on template")
                .add_copy_flag(),
        )
        // ========== COMMIT-RELATED COMMANDS ========== //
        .subcommand(
            Command::new("set-commit")
                .arg(Arg::new("template").required(true).help(
                    "Template has places to interpolate marked with {}, [], {b} \n\n\
                    When you provide: \n\
                    '[{}] - {}' \n\
                    as your commit template, you will be able to run \n\
                    commit command with following args: \n\
                    git-helpe-rs commit 123 'fix gpu issues' \n\
                    and this commit message will be added: \n\
                    git commit -m \'[123] - fix gpu issues\' \n\n\
                    If you provide [] in your template you will have to \n\
                    set autocomplete value with set-auto-complete \n\
                    with the same number of args as number of [] \n\
                    in provided template \n\n\
                    if you provide {b} in your template you should \n\
                     - have some number in your branch \n\
                     - run git-helpers-c with -b flag \n\
                    ",
                ))
                .arg(Arg::new("key").short('k').required(false).help(
                    "
                You can provide key to have different templates at hand: \n\
                git-helpe-rs commit 123 'fix gpu issues' -k sec \n\
                will use template that was set with \n\
                git-helpe-rs set-commit '{} - {}' -k sec \n\
                Otherwise it will be saved as your default \n\
                ",
                ))
                .about("Set template for commit formatting")
                .after_help(
                    "Sets commit format. \n\
                    You can use {{}} for places to interpolate \n\
                    and {[]} for places to autocomplete. \n\
                    and {b} as places to autocomplete from number in branch. \n\
                    ",
                ),
        )
        .subcommand(
            Command::new("set-auto-complete")
                .about("set value that will be used to autocomplete commit template")
                .arg(Arg::new("auto_complete_value").required(true).num_args(0..)),
        )
        .subcommand(
            Command::new("c")
                .arg(Arg::new("interpolate_values").required(true).num_args(0..))
                .arg(
                    Arg::new("auto-complete")
                        .short('a')
                        .action(ArgAction::SetTrue)
                        .help("Should use autocomplete values for commit message"),
                )
                .arg(Arg::new("key").short('k').required(false).help(
                    "
                    Specify which template you want to use \n\
                    if you omit this param default template will be used \n\
                    ",
                ))
                .arg(
                    Arg::new("infer-number-from-branch")
                        .short('b')
                        .action(ArgAction::SetTrue)
                        .help(
                            "Will try to catch any number from branch name \n\
                    and use it as a value for {b} in your template \n\
                    if your template has no {b} it will panic \n\
                    ",
                        ),
                )
                .about("Commit using one of templates")
                .add_copy_flag(),
        )
        // ============== OTHERS ============== //
        .subcommand(Command::new("show").about("Show current config in plain JSON"))
        .subcommand(
            Command::new("set-clipboard-command")
                .about("[WIP] Set command which will be run when doing taking value from clipboard")
                .arg(Arg::new("program_name").help(
                    "On default it's written to use pbpaste as \
                command for taking branch from clipboard, but if you want \
                you can use command of your own choice
               please don't use it for now 
                ",
                )),
        )
}

trait AddCopyFlag {
    fn add_copy_flag(&self) -> Self;
}

impl AddCopyFlag for Command {
    fn add_copy_flag(&self) -> Self {
        self.arg(Arg::new("copy-flag").short('x').help(
            "instead of executing command pass it to clipboard \n \n
        you can always configure command used for passing to clipboard",
        ))
    }
}
