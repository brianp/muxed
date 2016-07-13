Muxed
=====
[![Build Status](https://travis-ci.org/brianp/muxed.png?branch=master)](https://travis-ci.org/brianp/muxed)

## Another TMUX project manager

Currently this project is _semi-stable_ and in active development (July, 2016).
Muxed is usable with simple project configs, and testable with configs you may already have (see the `-p` option.)
Drop muxed in to your `$PATH` and take it for a spin. Happy to receive feature requests or bug reports!

## Muxed Tools

**muxed**: Provides the functionality for opening and parsing your project configs to
launch a TMUX session.

**muxednew**: Provides a template generator to create new project files. See the `new`
subcommand in [Usage](#usage-options).

## Installation

### Download:

See the [releases](https://github.com/brianp/muxed/releases) page for muxed packages.
Download and untar the package as desired. The muxed-complete package contains
both `muxed` and `muxednew`. The stand alone muxed package
only contains the `muxed` bin.
Make sure the `muxed` and `muxednew` bins are somewhere in your `$PATH`. I
generally move the bins in to `/usr/local/bin`.

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

## Setup

### 1. Create a new project file.

If this is your first run, muxed will create the `~/.muxed/` directory for you.

```bash
$ muxed new my_project
Looks like this is your first time here. Muxed could't find the configuration directory: `/home/vagrant/.muxed`
Creating that now 👌

✌ The template file my_project.yml has been written to /home/vagrant/.muxed
Happy tmuxing!
```

The generated template will look like this (but with some inline docs):
```yaml
root: "~/"
windows:
  - editor:
      layout: "main-vertical"
      panes: ["vi", "ls -alh"]
  - processes: "ls /proc"
  - logs: "tail -f /var/log/dmesg"
```

This config will create a new tmux session with three windows named *editor*,
*processes* and *logs*. By default your view will be on the first window opened,
on the first pane, which in this case is *vi* in the *editor* window. The first window will have
two panes split vertically, the left will have the editor *vi* running and the
right will have a shell listing of your current working directory.


### 2. Edit your template
Now you can use your favourite editor and make changes to the config as desired.

```bash
$ $EDITOR ~/.muxed/my_project.yml
```

### 3. Open TMUX with your muxed config
```bash
$ muxed my_project
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

SUBCOMMANDS:
    new    The name of your poject to create
```

## Inspiration
This project has been inspired by the work done on the [tmuxinator](https://github.com/tmuxinator/tmuxinator) project. Check it out for a `ruby` based tmux session management solution.

## Copyright
Copyright (c) 2014-2016 Brian Pearce. See [LICENSE](https://github.com/brianp/muxed/blob/master/LICENSE) for further details.
