Muxed
=====
[![Build Status](https://travis-ci.org/brianp/muxed.png?branch=master)](https://travis-ci.org/brianp/muxed)

## Another TMUX project manager

Currently this project is _not stable_ and in active development (May 11, 2016). This means it compiles on Rustlang stable and can be used to manage projects in the "happy path."

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
$ touch ~/.muxed/project.yml
```

### 3. Copy a simple config in to your project file.

```yaml
root: "~/"
windows:
  - editor:
      layout: "main-vertical"
      panes: ["vim", "guard"]
  - server: "bundle exec rails s"
  - logs: "tail -f log/development.log"
```

### 4. Compile & run Muxed for your project.
Here is where you call your project, with the same name as the config file created in step 2.

```bash
$ ./muxed project
```

#### With Cargo:

```bash
$ cargo run project
```
