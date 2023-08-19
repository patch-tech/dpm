# Development

## Developing

Workflow is the standard Cargo-based workflow: `cargo build`, `cargo test`, etc. The [Cargo book](https://doc.rust-lang.org/stable/cargo/) is a great reference.

If `proto/dpm_agent.proto` is modified, run `make` to regenerate all the static `dpm-agent` client stubs.

*NOTE*: The Python stub in
`/static/python/src/backends/dpm_agent/dpm_agent_pb2_grpc.py` is generated with
absolute imports, so revert the change below before you commit.
```diff
diff --git a/static/python/src/backends/dpm_agent/dpm_agent_pb2_grpc.py b/static/python/src/backends/dpm_agent/dpm_agent_pb2_grpc.py
index 11c4989..d7119e1 100644
--- a/static/python/src/backends/dpm_agent/dpm_agent_pb2_grpc.py
+++ b/static/python/src/backends/dpm_agent/dpm_agent_pb2_grpc.py
@@ -2,9 +2,7 @@
 """Client and server classes corresponding to protobuf-defined services."""
 import grpc

-# NB: This import must remain relative!
-# Any change to an absolute import should be rejected.
-from . import dpm_agent_pb2 as dpm__agent__pb2
+import dpm_agent_pb2 as dpm__agent__pb2


 class DpmAgentStub(object):
```

## Repo layout

This repo is a Cargo [package](https://doc.rust-lang.org/cargo/appendix/glossary.html#package) containing 2 [targets](https://doc.rust-lang.org/cargo/reference/cargo-targets.html):
- src/bin/dpm.rs, a binary target that is dpm itself
- src/lib.rs, a library target whose sole use is generating docs via `cargo doc`

This and other layout choices follow [Rain's Rust CLI recommendations](https://rust-cli-recommendations.sunshowers.io/). Thanks, @sunshowers!

## Publishing

To create a new release, follow these steps:

1. Examine the changes in the "Unreleased" section of [the changelog](./CHANGELOG.md) and determine the [SemVer](https://semver.org/spec/v2.0.0.html) severity of the release: major, minor, or patch. The rest of these instructions refer to this new version as vX.Y.Z.
2. Create a "release PR": A PR that:
   1. Updates the `version` field in the [package's manifest](./Cargo.toml) to be `"X.Y.Z"`.
   2. Resets the changelog. Do this by duplicating the "Unreleased" section, re-titling the lower duplicate to be `X.Y.Z - <today's date>`, removing empty sections from the new version entry, and removing the items from the sections of "Unreleased".
3. Merge the release PR.
4. Create the GitHub release.
   1. Go to https://github.com/patch-tech/dpm/releases/new, type "vX.Y.Z" in the "Choose a tag" input and select "Create new tag ... on publish".
   2. Title the release `vX.Y.Z`.
   3. In the text area, copy/paste the contents of the vX.Y.Z section of the changelog.
   4. Press **Publish release**.
5. Create and merge a PR to update [the dpm Homebrew formula](https://github.com/patch-tech/homebrew-tap/blob/main/Formula/dpm.rb). In particular, update the `tag` and `revision` values to be the tag and SHA of the commit associated with the release created in the previous step.

That's it! After completing step 4 [a GitHub workflow](https://github.com/patch-tech/dpm/blob/main/.github/workflows/release.yml) will be triggered that builds release assets and attaches them to the just-created release. This asynchronous work normally takes about 15 minutes to complete.
