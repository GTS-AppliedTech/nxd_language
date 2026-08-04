BUILD_SYSTEM_SPECS.md


## NXD build system specification

This defines how NXD projects are built, tested, and released across Nim, Elixir, and D, while presenting a single, coherent interface to the user.



1. Core commands

NXD exposes three primary CLI commands:

• nxd build
Purpose: Compile the project to all configured backends (or a selected one).
• nxd test
Purpose: Run the project’s test suite across backends.
• nxd release
Purpose: Produce optimized, deployable artifacts (binaries, releases, packages).


Each command operates on the project manifest and module graph.



2. Project manifest

Every NXD project has a project.nxd (or nxd.toml) manifest:

name = "my_app"
version = "0.1.0"
license = "MIT"

[backends]
nim = true
elixir = true
d = true

[dependencies]
geometry = ">=1.0.0"
security = "^2.0.0"


Semantics:

• [backends] controls which targets are built.
• [dependencies] references NXD packages (not raw Nim/Mix/Dub deps).
• The build system generates backend‑specific manifests (Nimble, Mix, Dub) automatically.




3. Build pipeline

For each backend, nxd build runs:

1. Source collection• Read .nxd files according to module/package graph.

2. Compilation• Lexer → Parser → Typed AST → IR → Backend code (Nim/Elixir/D).

3. Backend integration• Generate backend project files:• Nim: *.nim, nimble manifest
• Elixir: *.ex, mix.exs
• D: *.d, dub.json


4. Backend build invocation• Call nim c, mix compile, dub build (configurable).



Artifacts:

• Nim: binaries or libraries.
• Elixir: compiled BEAM files / releases.
• D: binaries or libraries.




4. Test pipeline

nxd test:

• Discovers test modules (e.g., tests/* or MODULE my_app.tests.*).
• Compiles them through the same pipeline.
• Generates backend‑specific test harnesses:• Nim: unittest blocks or custom runner.
• Elixir: ExUnit.
• D: unittest blocks or custom runner.



Runs tests per backend and aggregates results into a unified report.



5. Release pipeline

nxd release:

• Builds with optimization flags per backend.
• Produces:• Nim: optimized binary / library.
• Elixir: release (via Mix).
• D: optimized binary / library.

• Optionally bundles:• config files
• runtime shims
• security policies
• version metadata



Release profiles can be defined in the manifest:

[release]
profile = "prod"




6. Cross‑backend consistency rules

The build system enforces:

• Same NXD source → same IR → consistent semantics across backends.
• No backend‑specific code paths in NXD (no #if nim, etc.).
• Backend differences are handled only in generated code and runtime shims.


If a backend cannot support a feature, the build system must:

• fail clearly, or
• require explicit feature flags in the manifest.



7. Example workflow

# initialize project
nxd init my_app

# build for all backends
nxd build

# build only for Nim
nxd build --backend nim

# run tests
nxd test

# create release
nxd release



In short: the NXD build system gives you one set of commands and one manifest, and turns that into three coherent backend projects—while preserving the semantic guarantees you’ve defined everywhere else.
