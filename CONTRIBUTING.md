# IDE setup
We are currently using unstable rustfmt features in our rustfmt.toml, which isn't supported through rustfmt stable natively, as rustfmt v1 stable only supports unstable options via the command line. As such we are currently using a small wrapper around rustfmt, called `rustfmt-unstable`, which essentially passes the arguments in the config file via the cli.

To be consistent in ci & in our local development experience, we also overwrote the rustfmt command for rust-analyzer in both vscode and helix.
If you are using one of these editors you will need to install rustfmt-unstable via `cargo install rustfmt-unstable` to get formatting working.

# Maintainer documentation
This section of the docs is only relevant to the maintainers of this project. Feel free to skip if you are just a contributor.

## Publishing to a registry
When publishing this workspace to a registry we have to consider that we have intra-workspace dependencies.
As such sadly publishing a new version isn't _quite_ as easy as bumping one version number and running cargo publish.

The following steps are usually required:
1. Bump the main version number in the root `Cargo.toml` (the `version = "..."` key at the top of the file). Make sure to maintain semver guarantees while doing that.
2. Update the intra workspace dependencies. Since we use workspace dependencies for all of them, this is self-contained in the `Cargo.toml` file at the workspace root:
    1. Go down to the `[workspace.dependencies]` section. There you'll find the workspace packages as workspace dependencies using `path = "..."` style references.
    2. Bump the version of all the workspace packages to the same version you have choosen in step 1. If they don't have a `version = "..."` key already, feel free to add them in **addition** to the `path = "..."` key. That will ensure that we still use path dependencies while developing locally and only use the version when publishing (cargo supports that).
    3. If you are publishing to a registry other than your system's default one (which is usually `crates.io`) you'll also need to add a `registry = "..."` key along the version key from the step above. Make sure to not push this (commit it only locally to satisfy cargo), as the  CI won't find your locally configured registry and as such will fail to build.
3. Now we can finally use `cargo publish --workspace` to publish the workspace packages. Make sure that you include the `--registry` argument if you plan to publish to another registry than your systems default one.

## Publishing v1.0.0
Once we publish any of this to crates.io we need to replace some placeholder links with actual URLs to docs.rs. These placeholders are not yet valid because the packages don't exist in the registry. To find them, search for `]: #` in the code.

# Tests
We enable the `unwrap_used` and `expect_used` lints workspace-wide (see `Cargo.toml`), which would normally forbid `.unwrap()` and `.expect()`. We deliberately relax them inside tests via the `allow-unwrap-in-tests`, `allow-expect-in-tests`, and `allow-panic-in-tests` options in `clippy.toml`.

As such, prefer `.unwrap()`/`.expect()` in `#[test]` functions over returning a `Result` and using `?`. This keeps tests consistent across the code base, and means a failure points at the exact call that failed: a propagated `?` instead reports the error at the end of the test and doesn't identify which of several fallible calls went wrong.

We still use assertions such as `assert!(result.is_err())` or `matches!(result, Err(..))` when the failure is itself the behaviour under test; the guidance above is only about errors that are part of a test's setup.

Error messages are user-facing prose, so avoid asserting their exact wording (including capitalization or substrings) in tests; cosmetic rewording shouldn't break a test. Prefer structural checks instead — e.g. that a wrapper forwards the wrapped error's message, or that two variants produce distinguishable messages. More generally, only assert `Display` output that is part of a type's contract (such as a `Bitstring` rendering as `"10110"`), not incidental formatting.
