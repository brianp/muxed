Muxed
=====
[![Build Status](https://travis-ci.org/brianp/muxed.png?branch=master)](https://travis-ci.org/brianp/muxed)

## Another TMUX project manager

Currently this project is _not stable_ and in active development (May, 2016). This means it compiles on Rustlang stable and can be used to manage projects in the "happy path."

What's the happy path mean? You must meet all requirements and use simple configs.

Work on stability, setup, and resilience are planned for the immediate future.

## Setup

### 1. Create a new `.muxed` project directory.

```bash
$ mkdir ~/.muxed/
```

### 2. Create a new yaml config for your project.
The config file name should match your project name, and will be used to call on the project in step 4.

```bash
$ touch ~/.muxed/my_project.yml
```

### 3. Copy a simple config in to your project config file.

Example config:
```yaml
root: "~/"
windows:
  - editor:
      layout: "main-vertical"
      panes: ["vi", "ls -alh"]
  - processes: "ls /proc"
  - logs: "tail -f /var/log/dmesg"
```

This will create a new tmux session with three windows named *editor*,
*processes* and *logs*. By default your view will be on the last window opened,
which in this case is *logs*. The first window will have two panes split
vertically, the left will have the editor *vi* running and the right will have a
shell listing of your current working directory.

A one liner for creating the above config:
```bash
$ printf 'root: "~/"\nwindows:\n  - editor:\n      layout: "main-vertical"\n      panes: ["vi", "ls -alh"]\n  - processes: "ls /proc"\n  - logs: "tail -f /var/log/dmesg"' > ~/.muxed/my_project.yml
```

## Installation

### Download:

See the [releases](https://github.com/brianp/muxed/releases) page for muxed packages. Download and untar the package as desired.
Make sure the `muxed` binary is somewhere in your `$PATH`. I generally move the binary in to `/usr/local/bin`.

```bash
$ tar -xvzf muxed-VERSION-SYSTEM.tar.gz
x muxed
$ mv muxed /usr/local/bin
$ muxed --help
$ muxed my_project
```

### From source:

Have rust stable (or nightly at the risk of it not working) installed.
Clone this repo. Then run cargo to build the source, and again use cargo to run the app.

```bash
$ git clone git@github.com:brianp/muxed.git
$ cargo build
$ cargo run -- --help
$ cargo run my_project
```

## Usage Options

```shell
USAGE:
    muxed [FLAGS] [OPTIONS] <PROJECT_NAME>

FLAGS:
    -d               If you want to create a muxed session without connecting to it
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -p <PROJECT_DIR>        The directory your project config files live in. Defaults to ~/.muxed/

ARGS:
    <PROJECT_NAME>    The name of your poject to open
```

## Inspiration
This project has been inspired by the work done on the [tmuxinator](https://github.com/tmuxinator/tmuxinator) project. Check it out for a `ruby` based tmux session management solution.

## Copyright
Copyright (c) 2014-2016 Brian Pearce. See [LICENSE](https://github.com/brianp/muxed/blob/master/LICENSE) for further details.
