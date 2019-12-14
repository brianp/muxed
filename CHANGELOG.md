<a name="0.8.0">The one that adds Edit and upgrades all the Dependencies</a>
## 0.8.0 (2019-12-14)

#### Features

*   Add github issue template ([f805c103](https://github.com/brianp/muxed/commit/f805c103c3abba16b55e77946bf5d17bb3a46e72))
* **Edit:**  The edit subcommand ([43ebe607](https://github.com/brianp/muxed/commit/43ebe607616da784d277cf27a79635a71ff6426c), closes [#6](https://github.com/brianp/muxed/issues/6))

#### Improvements

* **New:**  Expand the new command's arguments to accept -F ([9f1f04e5](https://github.com/brianp/muxed/commit/9f1f04e5ef6c3878d6e251bfe93a799d6d9430ea))

#### Fixes

* **New:**  Don't panic when file already exists ([9f1f04e5](https://github.com/brianp/muxed/commit/9f1f04e5ef6c3878d6e251bfe93a799d6d9430ea))

#### Documentation

* **Readme:**  Update usage ([051f83c2](https://github.com/brianp/muxed/commit/051f83c227bb4131f933ec8c67af5dace670ca6e))

#### Refactor

*   Refactor file naming in tests ([c104abd5](https://github.com/brianp/muxed/commit/c104abd5fc0cfa76259652625c195b2bbcaa8bb6))
*   Don't compile untested crates ([9999778b](https://github.com/brianp/muxed/commit/9999778ba8492efbd37306bfc022ea9fdeb50bcd))
*   Utilize same template creation code ([46ecae5b](https://github.com/brianp/muxed/commit/46ecae5b88338260e8c3e7e1e228012f16fd19e8))
*   Fixup un-used result warning ([101bf984](https://github.com/brianp/muxed/commit/101bf9844bd3b7037d271ed2838f0d7336fde164))
* **Muxed:**  Upgrade docopt 0.7.0 -> 1.1.0 ([d0429ef3](https://github.com/brianp/muxed/commit/d0429ef3f785a6898f082d1a3558a2ae025abaca))
* **Common:**
  *  Upgrade rand 0.3.15 -> 0.7.2 ([ef2b3dd0](https://github.com/brianp/muxed/commit/ef2b3dd099f84145dcfe23f1823e4807d4ecf038))
  *  Upgrade dirs 1.0.5 -> 2.0.2 ([90f60ed9](https://github.com/brianp/muxed/commit/90f60ed9a21786f62f34e1f4606d0cb45794a8cd))
* **Edit:**
  *  Upgrade libc 0.2.21 -> 0.2.66 ([672430fc](https://github.com/brianp/muxed/commit/672430fc7550059ed0a750bc3103c226db26c664))
  *  Upgrade dirs 1.0.5 -> 2.0.2 ([07422d76](https://github.com/brianp/muxed/commit/07422d7622de2050300e35cc298074796bf01048))
* **Load:**
  *  Upgrade yaml-rust 0.3.2 -> 0.4.3 ([31476536](https://github.com/brianp/muxed/commit/31476536ae8067ec74fb5a3c731a8b08478c18fe))
  *  Upgrade libc 0.2.21 -> 0.2.66 ([37562a42](https://github.com/brianp/muxed/commit/37562a42f8068d761fc45e71c64d130bc198f0e0))
  *  Upgrade rand 0.3.15 -> 0.7.2 ([95392372](https://github.com/brianp/muxed/commit/9539237241cb74ca7dcda7811592d5dd4d458d40))
  *  Upgrade dirs 1.0.5 -> 2.0.2 ([958a382b](https://github.com/brianp/muxed/commit/958a382bc938548599914ef475a8fddd6233d7fb))
* **Snapshot:**
  *  Upgrade serde 0.8.23->1.0.103 ([46f1e872](https://github.com/brianp/muxed/commit/46f1e87200f2a72cc239a2733d1ae2ebb44a36f2))
  *  Upgrade regex 0.2.1 -> 1.3.1 ([2824b0ad](https://github.com/brianp/muxed/commit/2824b0adfcaff1adbe0cad80e4fbf3b4271076fa))
* **Docker:**  Rename and fixup docker files ([d6bec594](https://github.com/brianp/muxed/commit/d6bec594f435e108c96585f66c3f332582ec822d))
* **First Run:**  Use paths for first run check ([98fecbd2](https://github.com/brianp/muxed/commit/98fecbd20c891af5d780ba95384cbdaae0b7f8e1))
* **Project Paths:**  Use std::default for Args ([9a0ec6dd](https://github.com/brianp/muxed/commit/9a0ec6dd786889d99ec3c563d256c63d38181137))

<a name="0.7.1">The one that removes debug lines</a>
### 0.7.1 (2019-12-05)

#### Documentation

  *  Update installation methods ([06266d1d](06266d1d))

#### Bug Fixes

  *  Remove debug lines being printed during execution

#### Features

  *  Adds the debug flag ([20ee37e3](20ee37e3))

<a name="0.7.0">Modernizing the repo and fixing bugs</a>
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
