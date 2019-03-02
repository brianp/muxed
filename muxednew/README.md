Muxed New
=====
[![Build Status](https://travis-ci.org/brianp/muxednew.png?branch=master)](https://travis-ci.org/brianp/muxednew)

## A Muxed Template Generator

This is a helper tool to generate project configuration files for [Muxed](https://github.com/brianp/muxed). The template will be generated with inline docs explaining available features and options.

Execute muxednew and generate a file in `~/.muxed/` by running:

```bash
$ muxednew <PROJECT_NAME>
```

Or via the `Muxed` subcommand:

```bash
$ muxed new <PROJECT_NAME>
```

## Installation

### Download:

See the [releases](https://github.com/brianp/muxednew/releases) page for muxednew packages. Download and untar the package as desired.
Make sure the `muxednew` binary is somewhere in your `$PATH`. I generally move the binary in to `/usr/local/bin`.

```bash
$ tar -xvzf muxednew-VERSION-SYSTEM.tar.gz
x muxednew
$ mv muxednew /usr/local/bin
$ muxednew --help
$ muxednew my_project
```

### From source:

Have rust stable (or nightly at the risk of it not working) installed.
Clone this repo. Then run cargo to build the source, and again use cargo to run the app.

```bash
$ git clone git@github.com:brianp/muxednew.git
$ cargo build
$ cargo run -- --help
$ cargo run my_project
```

## Usage Options

```shell
USAGE:
    muxednew [FLAGS] [OPTIONS] <PROJECT_NAME>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -p <PROJECT_DIR>        The directory your project config files live in. Defaults to ~/.muxed/

ARGS:
    <PROJECT_NAME>    The name of your poject to open
```

## Copyright
Copyright (c) 2014-2017 Brian Pearce. See [LICENSE](https://github.com/brianp/muxednew/blob/master/LICENSE) for further details.
