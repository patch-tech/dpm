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

Publishing to a registry (presumably crates.io) is disabled until dpm is minimally functional.
