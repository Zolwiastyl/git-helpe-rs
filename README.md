# git-helpe-rs

CLI helper for formatting branches and commits

## Why

Probably when you code you have some requirements about branch name and commit format.
I want to make it easy for myself, maybe someone finds themself in the same situation.

## What

This cli provides you with following commands:

- `show` - show current config
- `set-prefix k value` - sets key-value pair for your branch prefix
- `b k` - uses what you have in your clipboard and ads your prefix to it and checkouts to new branch
- `set-commit "Your own template with {}; That will be interpolated on each {}"` - this sets format for your commits
- `b c here "are values with which your template will be interpolated"` - by this you will make a commit with given format
- `set-branch "Same as for commit, but doesn't yet work"` - tbd
- `delete k` - deletes key from branch prefixes

## How

Get the git repo, build it with:

`cargo build -r`

Go to

`cd ./target/release/`

run:

`chmod +x git-helpe-rs`

and copy it to your bin directory to start using or make alias for it.

Or as one command:

```sh
git clone git@github.com:Zolwiastyl/git-helpe-rs.git &&
cargo build -r &&
cd ./target/release &&
chmod +x git-helpe-rs
```

<!-- TODO -->

[x] use clap builder
[x] piping output from git
[x] add variants for template in gitconfig
[x] adding clipboard commands
[x] setting branch template
[x] branch formatting with template

[x] copy flag and dry run
      [x] commit
      [x] branch:
            [x] prefix
            [x] template

[ ] {b} in templates
[ ] {[]} in templates
[ ] autocompletion
[ ] publish

ADD VARIANTS FOR TEMPLATE
  set-branch-format  Set template that can be used when switching branches
  set-commit         Set template for commit formatting

  set-branch-prefix  Set prefix for checkout using clipboard contents
        key value

  bp                 Check you out to a branch using your clipboard contents
        prefixKey

  bf                 Check out to a branch based on template
  -k use exact template
  c                  Commit using one of templates
    -a use autocomplete values
    -k use exact template
    -b use number from branch
        template-key ..interpolate-values

  show               Show current config in plain JSON
  help               Print this message or the help of the given subcommand(s)
