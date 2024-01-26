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

and copy it to your bin directory to start using.

Or as one command:

```sh
git clone git@github.com:Zolwiastyl/git-helpe-rs.git &&
cargo build -r &&
cd ./target/release &&
chmod +x git-helpe-rs
```

<!-- TODO -->

[] use clap builder
[] add setting branch format
[] add branch formatting
[] add {b} in templates
[] add {[]} in templates
[] add piping output from git
[] add dry run
[] add copy flag
