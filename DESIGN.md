# `cargo new` specification

## Conventions

Each normative line item has a stable identifier.
Before each line item is `_*_` which links to an HTML anchor named for the identifier at that exact line item.
Identifiers are lowercase,
start with `r-`,
include a short kebab-case phrase for each parent header separated by `.`,
and end with a short kebab-case summary of the line item.

`SHALL`, `SHOULD`, and `MAY` are used in their normal specification sense.

## Output path

<a id="r-path.required"></a>
[_*_](#r-path.required)
The `new` subcommand SHALL require a positional `PATH` argument naming the destination package directory.

<a id="r-path.relative-to-current-dir"></a>
[_*_](#r-path.relative-to-current-dir)
A relative `PATH` argument SHALL be interpreted relative to the process working directory.

<a id="r-path.fail-if-exists"></a>
[_*_](#r-path.fail-if-exists)
`cargo new` SHALL fail if the destination path already exists.

<a id="r-path.warn-invalid-path"></a>
[_*_](#r-path.warn-invalid-path)
`cargo new` SHALL warn if the destination path cannot be added to `PATH`, like with `std::env::join_paths`.

## Output kind selection

<a id="r-kind.bin"></a>
[_*_](#r-kind.bin)
If `--bin` is specified, `cargo new` SHALL create a package with a binary build target.

<a id="r-kind.lib"></a>
[_*_](#r-kind.lib)
If `--lib` is specified, `cargo new` SHALL create a package with a library build target.

<a id="r-kind.default"></a>
[_*_](#r-kind.default)
If no output kind is specified,
`cargo new` SHALL default to binary package behavior.

<a id="r-kind.conflict"></a>
[_*_](#r-kind.conflict)
`cargo new` SHALL fail if more than one output kind is specified.

## Version control integration

<a id="r-vcs.optional"></a>
[_*_](#r-vcs.optional)
The `new` subcommand SHALL accept an option `--vcs <VCS>` to select a version control mode.

<a id="r-vcs.support"></a>
[_*_](#r-vcs.support)
The `--vcs` option SHALL accept only `git`, `hg`, `pijul`, `fossil`, or `none`.

<a id="r-vcs.default"></a>
[_*_](#r-vcs.default)
The VCS mode SHALL default to `git` unless the destination would be tracked by an existing supported repository,
in which case it SHALL default to `none`.

<a id="r-vcs.none"></a>
[_*_](#r-vcs.none)
If `--vcs none` is supplied, `cargo new` SHALL not initialize a version-control repository.

<a id="r-vcs.ignore"></a>
[_*_](#r-vcs.ignore)
When a repository mode is selected, `cargo new` SHALL create the ignore file appropriate to that repository type.

<a id="r-vcs.ignore-target"></a>
[_*_](#r-vcs.ignore-target)
Ignore-file content SHALL include at least the package build output directory.

## Name

<a id="r-name.optional"></a>
[_*_](#r-name.optional)
The `new` subcommand SHALL accept an option `--name <NAME>` to use as the package name.

<a id="r-name.default"></a>
[_*_](#r-name.default)
The package name SHALL default to the final path component when `--name` is not supplied.

<a id="r-name.fail-invalid-package-names"></a>
[_*_](#r-name.fail-invalid-package-names)
`cargo new` SHALL reject invalid Cargo package names.

<a id="r-name.fail-keywords"></a>
[_*_](#r-name.fail-keywords)
`cargo new` SHALL reject Rust keywords as package names.

<a id="r-name.fail-artifact"></a>
[_*_](#r-name.fail-artifact)
`cargo new` SHALL reject binary build-target names that conflict with Cargo artifact-directory names.

<a id="r-name.warn-artifact"></a>
[_*_](#r-name.warn-artifact)
`cargo new` SHALL warn for non-binary build-target names that conflict with Cargo artifact-directory names.

<a id="r-name.fail-test"></a>
[_*_](#r-name.fail-test)
`cargo new` SHALL reject package names that conflict with Rust’s built-in `test` library.

<a id="r-name.warn-sysroot"></a>
[_*_](#r-name.warn-sysroot)
`cargo new` SHOULD warn for names that are legal but conflict with standard-library crates.

<a id="r-name.fail-windows-reserved"></a>
[_*_](#r-name.fail-windows-reserved)
`cargo new` SHALL reject names that are reserved on Windows when run on Windows.

<a id="r-name.warn-windows-reserved"></a>
[_*_](#r-name.warn-windows-reserved)
`cargo new` SHALL warn for names that are reserved on Windows when run on non-Windows systems.

<a id="r-name.warn-case"></a>
[_*_](#r-name.warn-case)
`cargo new` SHALL warn for names that are neither snake_case nor kebab-case.

<a id="r-name.warn-non-ascii"></a>
[_*_](#r-name.warn-non-ascii)
`cargo new` SHALL warn for names that are not ASCII.

## Package output

<a id="r-package.create-cargo-toml"></a>
[_*_](#r-package.create-cargo-toml)
`cargo new` SHALL create a `Cargo.toml` file in the destination package directory.

<a id="r-package.status"></a>
[_*_](#r-package.status)
`cargo new` SHALL print a "creation" status message describing the package being created.

<a id="r-package.name"></a>
[_*_](#r-package.name)
The generated manifest SHALL set `package.name` to the name argument.

<a id="r-package.version"></a>
[_*_](#r-package.version)
The generated manifest SHALL set `package.version` to `0.1.0`.

<a id="r-package.edition-option"></a>
[_*_](#r-package.edition-option)
If `--edition` is supplied, the generated manifest SHALL use that for `package.edition`.

<a id="r-package.edition-default"></a>
[_*_](#r-package.edition-default)
If `--edition` is not supplied, the generated manifest SHALL use the latest stable edition for `package.edition`.

<a id="r-package.edition-valid"></a>
[_*_](#r-package.edition-valid)
The `--edition` option SHALL accept only supported Rust edition values.

<a id="r-package.dependencies"></a>
[_*_](#r-package.dependencies)
The generated manifest SHALL include an empty `[dependencies]` table.

<a id="r-package.registry"></a>
[_*_](#r-package.registry)
If `--registry` is supplied, the generated manifest SHALL use that for `package.publish`.

<a id="r-package.workspace-detect"></a>
[_*_](#r-package.workspace-detect)
If the destination package is created under an ancestor workspace root and not excluded, `cargo new` SHOULD attempt workspace-aware manifest behavior.

<a id="r-package.workspace-invalid"></a>
[_*_](#r-package.workspace-invalid)
If a potential ancestor workspace manifest is invalid, `cargo new` SHOULD warn instead of erroring out.

<a id="r-package.workspace-members"></a>
[_*_](#r-package.workspace-members)
When an ancestor workspace is detected, `cargo new` SHOULD add the new package to `workspace.members`.

<a id="r-package.workspace-status"></a>
[_*_](#r-package.workspace-status)
`cargo new` SHALL print an "addition" status message describing the package being added as a workspace member.

<a id="r-package.workspace.package"></a>
[_*_](#r-package.workspace.package)
When added as a workspace member, `cargo new` SHALL inherit `[workspace.package]` keys in the generated manifest.

<a id="r-package.workspace.lints"></a>
[_*_](#r-package.workspace.lints)
When added as a workspace member, `cargo new` SHALL inherit `[workspace.lints]` in the generated manifest.

## Build-target output

<a id="r-target.bin"></a>
[_*_](#r-target.bin)
A binary build-target SHALL generate `src/main.rs`.

<a id="r-target.bin-compile"></a>
[_*_](#r-target.bin-compile)
A generated binary entrypoint SHALL be a compilable Rust program.

<a id="r-target.bin-main"></a>
[_*_](#r-target.bin-main)
Generated source content for a binary package SHALL include a minimal `main` function.

<a id="r-target.lib"></a>
[_*_](#r-target.lib)
A library build-target SHALL generate `src/lib.rs`.

<a id="r-target.lib-compile"></a>
[_*_](#r-target.lib-compile)
A generated library source file SHALL be a compilable Rust library source file.

<a id="r-target.lib-fn"></a>
[_*_](#r-target.lib-fn)
Generated source content for a library package SHALL include a minimal library function and a passing unit test.

<a id="r-target.rustfmt"></a>
[_*_](#r-target.rustfmt)
`cargo new` SHOULD format newly created Rust source files with `rustfmt` when that formatter is available.

## Exit behavior

<a id="r-exit.documentation"></a>
[_*_](#r-exit.documentation)
After successful creation, `cargo new` SHALL print a note pointing users at Cargo manifest documentation.

<a id="r-exit.files"></a>
[_*_](#r-exit.files)
A failure SHALL NOT leave partially generated output when the failure occurs.

<a id="r-exit.success-code"></a>
[_*_](#r-exit.success-code)
On successful package creation, `cargo new` SHALL exit successfully.

<a id="r-exit.cli-failure-code"></a>
[_*_](#r-exit.cli-failure-code)
On CLI validation failure, `cargo new` SHALL exit with a `1` exit code.

<a id="r-exit.op-failure-code"></a>
[_*_](#r-exit.op-failure-code)
On operation failure, `cargo new` SHALL exit with a `101` exit code.
