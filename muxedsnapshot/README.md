Muxed Snapshot
=====
[![Build Status](https://travis-ci.org/brianp/muxedsnapshot.png?branch=master)](https://travis-ci.org/brianp/muxedsnapshot)

## A TMUX session codifier for Muxed

This is a tool to generate project configuration files for [Muxed](https://github.com/brianp/muxed). The template will be generated off a currently running TMUX session. This will include:

 - Session name
 - Windows
  - Names
  - Layouts
  - Active Window
  - Panes
    - Current paths
    - Running processes
    - Active Pane

Execute muxedsnapshot and generate a file in `~/.muxed/` by running:
```bash
$ muxedsnapshot -n <NEW_PROJECT_NAME> -t <TARGET_SESSION>
```

Or via the `Muxed` subcommand:
```bash
$ muxed snapshot -n <NEW_PROJECT_NAME> -t <TARGET_SESSION>
```

## Installation

### Download:

See the [releases](https://github.com/brianp/muxedsnapshot/releases) page for muxedsnapshot packages. Download and untar the package as desired.
Make sure the `muxedsnapshot` binary is somewhere in your `$PATH`. I generally move the binary in to `/usr/local/bin`.

```bash
$ tar -xvzf muxedsnapshot-VERSION-SYSTEM.tar.gz
x muxedsnapshot
$ mv muxedsnapshot /usr/local/bin
$ muxedsnapshot --help
$ muxedsnapshot -n <NEW_PROJECT_NAME> -t <TARGET_SESSION>
```

### From source:

Have rust stable (or nightly at the risk of it not working) installed.
Clone this repo. Then run cargo to build the source, and again use cargo to run the app.

```bash
$ git clone git@github.com:brianp/muxedsnapshot.git
$ cargo build
$ cargo run -- --help
$ cargo run -- -n <NEW_PROJECT_NAME> -t <TARGET_SESSION>
```

## Usage Options

```shell
USAGE:
    muxedsnapshot [FLAGS] [OPTIONS] -n <NEW_PROJECT_NAME> -t <SESSION>

FLAGS:
    -f, --force      Overwrite existing file if one exists
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -n <NEW_PROJECT_NAME>        The name of your new project file to create
    -p <PROJECT_DIR>             The directory your project config files should live in. Defaults to ~/.muxed/
    -t <SESSION>                 The name of the TMUX session to codify
```

## Copyright
Copyright (c) 2014-2017 Brian Pearce. See [LICENSE](https://github.com/brianp/muxedsnapshot/blob/master/LICENSE) for further details.
