# Cargo Build System

The Rust Enhanced build system provides an interface for running Cargo. It can
show inline warning and error messages.  It also has a variety of ways of
configuring options for how Cargo is run.

See [Messages](messages.md) for settings to control how compiler messages are
displayed.

## Usage

When Sublime is set to use "Automatic" build system detection, it will choose
the build system based on the syntax of the currently active view.  If you
want to ensure the Rust Enhanced build system is used regardless of which file
is open, choose it via `Tools > Build System > RustEnhanced`.

The basic Sublime commands available are:

Command | Keyboard | Menu | Description
------- | -------- | ---- | -----------
Build | Ctrl-B / ⌘-B | Tools > Build | Runs the currently active build variant.
Build With... | Ctrl-Shift-B / ⌘-Shift-B | Tools > Build With... | Choose the build variant.
Cancel Build | Ctrl-Break / Ctrl-C | Tools > Cancel Build | Abort the currently running build.
Show Build Results | | Tools > Build Results > Show Build Results | Opens the output panel with build results.
Next Result | F4 | Tools > Build Results > Next Result | Go to the next warning/error message.
Previous Result | Shift-F4 | Tools > Build Results > Previous Result | Go to the previous warning/error message.

## Build Variants

When you select the RustEnhanced build system in Sublime, there are a few
variants that you can select with Tools > Build With...
(Ctrl-Shift-B / ⌘-Shift-B).  They are:

Variant | Command | Description
------- | ------- | -----------
(Default) | <code>cargo&nbsp;build</code> | Builds the project.
Automatic | | Automatically detect the command to run based on the currently active view (tests do `test`, binaries and examples do `run`, libraries do `build`, benches do `bench`).
Run | <code>cargo&nbsp;run</code> | Runs the binary.
Run (with args)... | <code>cargo&nbsp;run&nbsp;-&#8288;-&#8288;&nbsp;*args*</code> | Runs the binary with optional arguments you specify.
Test | <code>cargo&nbsp;test</code> | Runs unit and integration tests.
Test (with args)... | <code>cargo&nbsp;test&nbsp;-&#8288;-&#8288;&nbsp;*args*</code> | Runs the test with optional arguments you specify.
Bench | <code>cargo&nbsp;bench</code> | Runs benchmarks.
Clean | <code>cargo&nbsp;clean</code> | Removes all built files.
Document | <code>cargo&nbsp;doc</code> | Builds package documentation.
Clippy | <code>cargo&nbsp;clippy</code> | Runs [Clippy](https://github.com/Manishearth/rust-clippy).  Clippy must be installed, and currently requires the nightly toolchain.
Script | <code>cargo&nbsp;script&nbsp;$path</code> | Runs [Cargo Script](https://github.com/DanielKeep/cargo-script).  Cargo Script must be installed.  This is an addon that allows you to run a Rust source file like a script (without a Cargo.toml manifest).

## Cargo Settings

A variety of settings are available to customize how Cargo is run. These
settings can be set globally, per-Sublime project, per-Cargo package, for
specific build variants ("Build", "Run", "Test", etc.), or specific Cargo
targets (`--lib`, `--example foo`, etc.).

### Configure Command

To help you configure the Cargo build settings, run the `Rust: Configure Cargo
Build` command from Sublime's Command Palette (Ctrl-Shift-P / ⌘-Shift-P). This
will ask you a series of questions for the setting to configure.  It will
update your `.sublime-project` or `Users/RustEnhanced.sublime-settings` file
depending on which options you pick.  The first question is the setting you
want to update:

Setting | Description
------- | -----------
Target | Specify an explicit target (`--bin`, `--lib`, `--example`, etc.).  The "Automatic Detection" option will attempt to determine which target to use based on the current active view in Sublime (a test file will use `--test` or a binary will use `--bin`, etc.).
Profile | Determine whether or not the `--release` flag is used.
Target Triple | The `--target` option to specify a target triple (such as `x86_64-apple-darwin`).
Toolchain | Set the Rust toolchain to use (`nightly`, `beta`, etc.).
Features | Set the Cargo build features to use.
Environment Variables | Specify additional environment variables to set.
Extra Cargo Arguments | Extra arguments to include in the command line.
Default Package/Path | The default package to build, useful if you have a workspace with multiple Cargo packages.  See `Multiple Cargo Projects` below.

If you have multiple Cargo packages in your workspace, it will ask for the package to configure.

Caution: If you have not created a `sublime-project` file, then any changes
you make will be lost if you close the Sublime window.

### Settings

Cargo settings are stored in the `"cargo_build"` Sublime setting.  This can be
either in your `sublime-project` file or in
`Users/RustEnhanced.sublime-settings`.  `"cargo_build"` is an object with the
following keys:

Key | Description
--- | -----------
`"paths"` | Settings for specific Cargo packages.
`"default_path"` | The default Cargo package to build (useful for workspaces, see below).
`"variants"` | Settings per build variant.
`"defaults"` | Default settings used if not set per target or variant.

Paths should be an absolute path to the directory of a Cargo package, or the
path to a Rust source file (when used with `cargo script`).

`"paths"` is an object of path keys mapping to an object with the keys:

Path Key | Description
-------- | -----------
`"defaults"` | Default settings used if not set per target or variant.
`"targets"` | Settings per target (such as `"--lib"` or `"--bin foo"`).
`"variants"` | Settings per build variant.

An example of a `sublime-project` file:

```json
{
    "folders": [
        { "path": "." }
    ],
    "settings": {
        "cargo_build": {
            "paths": {
                "/path/to/package": {
                    "defaults": {
                        "release": false
                    },
                    "targets": {
                        "--example ex1": {
                            "extra_run_args": "-f file"
                        }
                    },
                    "variants": {
                        "bench": {
                            "toolchain": "nightly"
                        },
                        "clippy": {
                            "toolchain": "nightly"
                        }
                    }
                }
            },
            "default_path": "/path/to/package",
            "variants": {
                "run": {
                    "env": {
                        "RUST_BACKTRACE": "1"
                    }
                }
            },
            "defaults": {
                "release": true
            }
        }
    }
}
```

The available settings are:

Setting Name | Description
------------ | -----------
`target` | The Cargo target (such as `"--bin myprog"`).  Applies to `variants` only.  Can be `"auto"` (see "Automatic Detection" above).
`toolchain` | The Rust toolchain to use (such as `nightly` or `beta`).
`target_triple` | If set, uses the `--target` flag with the given value.
`release` | If true, uses the `--release` flag.
`features` | A string with a space separated list of features to pass to the `--features` flag.  Set to "ALL" to pass the `--all-features` flag.
`extra_cargo_args` | String of extra arguments passed to Cargo (before the `--` flags separator).
`extra_run_args` | String of extra arguments passed to Cargo (after the `--` flags separator).
`env` | Object of environment variables to add when running Cargo.
`working_dir` | The directory where to run Cargo.  If not specified, uses the value from `default_path`, otherwise attempts to detect from the active view, or displays a panel to choose a Cargo package.
`script_path` | Path to a `.rs` script, used by `cargo script` if you want to hard-code a specific script to run.
`no_default_features` | If True, sets the `--no-default-features` flag.

The extra args settings support standard Sublime variable expansion (see
[Build System
Variables](http://docs.sublimetext.info/en/latest/reference/build_systems/configuration.html#build-system-variables))

### Setting Precedence

The Cargo commands will generally use the most specific setting available.
The order they are searched are (first found value wins):

1. `.sublime-project` > Cargo Package > Cargo Target
2. `.sublime-project` > Cargo Package > Build Variant
3. `.sublime-project` > Cargo Package > Defaults
4. `.sublime-project` > Build Variant
5. `RustEnhanced.sublime-settings` > Build Variant
6. `.sublime-project` > Defaults
7. `RustEnhanced.sublime-settings` > Defaults

## Multiple Cargo Projects (Advanced)

You can have multiple Cargo projects in a single Sublime project (such as when
using Cargo workspaces, or if you simply have multiple projects in different
folders).

If you have multiple Cargo projects in your Sublime window, the build system
will use the currently active view to attempt to determine which project to
build.  Otherwise it will show an input panel to select a package.

You can set the `default_path` setting to always use a specific path.  It is specified at the same level as `paths` (see example above).  This can be set using the  `Rust: Configure Cargo Build` command.

## Custom Build Variants (Advanced)

You can define your own build system that takes advantage of the Cargo
settings.  This is useful if you want to quickly switch between different
configurations, or to add support for Cargo commands that are not already included.

The build variants are stored in your `.sublime-project` file.  To assist you
in configuring a build variant, there is a Sublime command called `"Rust:
Create New Cargo Build Variant"` which you can access from the Command
Palette.  It will ask a series of questions, and when it is done it will
automatically add the new build variant to your `.sublime-project` file.  Then
use the "Build With..." command (Ctrl-Shift-B / ⌘-Shift-B) to select and
execute your new variant.  The command will also copy over the stock build
variants so you do not need to switch between build systems.

You can manually edit your `.sublime-project` file to change the settings.  The settings described above are under the `"settings"` key.  Additionally, there is a `"command_info"` key which describes the features the command supports.  The available values are:

Setting Name | Default | Description
------------ | ------- | -----------
`allows_target` |  False | If True, the command accepts cargo filters for determining which target to build (`--lib`, `--bin foo`, `--example bar`, etc.).  Can also be a sequence of strings like `["bin", "example"]` to specify a subset of targets it supports.
`allows_target_triple` |  False | If True, the command accepts triples like `--target x86_64-apple-darwin`.
`allows_release` |  False | If True, allows `--release` flag.
`allows_features` |  False | If True, allows feature flags.
`allows_json` |  False | If True, allows `--message-format=json` flag.
`json_stop_pattern` | None | A regular expression matched against Cargo's output to detect when it should stop looking for JSON messages (used by `cargo run` to stop looking for JSON messages once compilation is finished).
`requires_manifest` | True | If True, the command must be run in a directory with a `Cargo.toml` manifest.
`requires_view_path` |  False | If True, then the active view must be a Rust source file, and the path to that file will be passed into Cargo (used mainly by `cargo script`).
`wants_run_args` |  False | If True, it will ask for extra args to pass to the executable (after the `--` flag separator).
