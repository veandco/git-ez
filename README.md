# git-ez

git-ez is a git commit tool that uses emojis! Currently only supports the Linux platform.

The conventions and emojis used here are mostly inspired by the beautiful-looking repository [vue-i18n](https://github.com/kazupon/vue-i18n)!

## Features

+ Single standalone binary that can be copied around
+ Multi-user commit using a single user account based on SSH IP address
+ Conventional commit style commits with emojis

## Build

You can build this tool by running:

```
cargo build --release
```

## Installation

You can install this tool by copying it into a path that your shell can find it. For example:

```
cp target/release/git-ez $HOME/.local/bin
```

## Usage

You can use this tool by running:

```
git ez
```

as an alternative to

```
git commit
```
