<a name="0.7.1"></a>
### 0.7.1 (2019-12-05)

#### Documentation

  *  Update installation methods ([06266d1d](06266d1d))

#### Bug Fixes

  *  Remove debug lines being printed during execution

#### Features

  *  Adds the debug flag ([20ee37e3](20ee37e3))

<a name="0.7.0"></a>
## 0.7.0 (2019-11-28)

#### Bug Fixes

  *  Use pane-base-index properly ([4b9ee44a](4b9ee44a))
  *  Fix root directory for new sessions ([d52cba96](d52cba96))

#### Features

  *  Add support for window paths ([f52438fc](f52438fc))

#### Improvements

  *  Begin trait based Command system ([65223007](65223007))
  *  Set default path on the session ([4460f026](4460f026))
  *  Major internal refactoring of type
  *  Restructured code into workspaces

#### Documentation

  *  Version bump to 0.7.0 ([19b44ef5](19b44ef5))
  *  Fixup minor errors ([2a035b27](2a035b27))
  *  Move root docs back to project root ([8f78742a](8f78742a))
  *  Update the usage ([07b9618f](07b9618f))


<a name="0.6.0"></a>
## 0.6.0 (2017-03-08)

#### Documentation

* **README:**
  *  No longer claim to be unstable ([0aae01fc](0aae01fc))
  *  Change shell samples to `shell` md ([01ad036f](01ad036f))
  *  Docs around muxednew ([05ee2222](05ee2222))
* **changelog:**  Add perf and imp to the changelog ([f5aca741](f5aca741))
* **clog.toml:**  Make changelogs easier ([682a53b6](682a53b6))
* **tmux/mod.rs#call:**  Update the docs for call ([68164956](68164956))

#### Features

* **Pre:**  Allow the Pre config option ([4d2bf1b8](4d2bf1b8))
* **issue-17:**  Allow pre to accept arrays ([fbe8ebaf](fbe8ebaf))

#### Improvements

* **Command::Pre:**  Add the Pre command ([7ef112b3](7ef112b3))
* **optparse:**  Swap clap for docopt ([065f8e1a](065f8e1a))

#### Bug Fixes

* **Attach:**  Attach to named sessions with spaces ([c5a5020d](c5a5020d))
* **issue-24:**  Allow pre_window config option ([05703e3f](05703e3f), closes [#24](24))

<a name="v0.5.0"></a>
## v0.5.0
Feature:
 - Added `new` SubCommand for generating new project files.

Bug:
 - Silence un-needed output.

Refactored:
 - Expressions in project/parser.rs to utilize `if let`.
 - Removed the linker from ./cargo/config. It had conflicts when actually
   building on osx.
 - When help is displayed.

<a name="v0.4.0"></a>
## v0.4.0 (2016-06-19)
Enhancement:
 - Increase the principle of least surprise by creating the project config
   directory during the first run, if the directory doesn't exist.

<a name="v0.3.5"></a>
## v0.3.5 (2016-06-18)
Add a fix for focusing on the top most first window when the session is attached.

<a name="v0.3.4"></a>
## v0.3.4 (2016-06-16)
Fixes a bug and now supports directories with spaces in the name.

<a name="v0.3.2"></a>
## v0.3.2 (2016-06-05)
Fixed a bug where blank values quotes values ex: "" created a send keys command for execution of no command. Instead let blank values opt out of the command completely.

<a name="v0.3.1"></a>
## v0.3.1 (2016-05-30)
Better error messaging when config file is not found.
The message was only showing the default muxed path even if the user ad specified their own project path.

Just changing the available installtion options for releases.

<a name="v0.3.0"></a>
## v0.3.0 (2016-05-30)
Substantial changes how windows manage default directories internally.
Big change on how the session and first window are created.

Cleanup warnings for unused code and paths.

<a name="v0.2.3"></a>
## v0.2.3 (2016-05-29)
This includes the bug fix for already active sessions.

<a name="v0.2.2"></a>
## v0.2.2 (2016-05-27)
Changes:
 - Bug fixes for windows and panes without execution commands. Where the value
   is None vs blank ("").
 - Fix for unnamed windows.

<a name="v0.2.1"></a>
## v0.2.1 (2016-05-26)
Fixes for better logging and minor bugs found by significant test coverage increase.

<a name="v0.2.0"></a>
## v0.2.0 (2016-05-07)
Significant changes have been made in the way commands are now built, and what commands can be built.
Major changes in the parsing of files and how windows are treated.

 - Added multiple commands now. Commands represent the actions taken on tmux and
   the language corresponds better.
 - Stopped opening sessions with a named window. This made it hard to treat all
   windows the same.

<a name="v0.1.0"></a>
## v0.1.0 (2016-04-30)
Bumping to 0.1.0 as this build works as desired for a simple config running the happy path.
