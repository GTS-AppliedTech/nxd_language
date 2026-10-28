# NXD Language

##### *Official Visual Studio Code language support for the NXD programming language.*

The NXD extension provides source-language support, syntax highlighting, diagnostics, and LSP-based semantic validation for .nxd files.

NXD is developed as its **own** language first. Backend targets are implementation targets and do **not** define NXD language semantics.
Features
### Language Support

    .nxd file recognition
    .nxd source file icons
    Syntax highlighting
    NXD keyword highlighting
    Type highlighting
    String highlighting
    Numeric literal highlighting
    Comment highlighting

### Diagnostics

NXD provides parser, warning, and semantic diagnostics through its Rust frontend and LSP validator.

**Current diagnostic families include:**

    NXD-P1xxx — parser diagnostics
    NXD-W2xxx — compiler warnings
    NXD-S3xxx — semantic diagnostics
    NXD-LSPxxxx — LSP/compiler integration diagnostics

**Async / Concurrency Diagnostics**

The current *NXD-S3005* family validates NXD async and concurrency semantics, including:

    AWAIT
    SPAWN
    SEND
    RECV
    ASYNC context
    PING
    ESC
    TIMEOUT
    SYNC
    CHANNEL
    TASK

NXD distinguishes between asynchronous execution operations and concurrency primitives that may cross the synchronous/asynchronous boundary.

**Current semantic work includes:**

    Task[T]
    ProcessHandle[T]
    ReceiveOperation[T]
    Channel[T]
    Inside-out validation of nested concurrency expressions
    Direct-target validation for guarded operations
    Async context enforcement
    Task lifecycle and synchronization behavior

### LSP Support

The extension integrates with the Rust-based NXD semantic validator to provide diagnostics for .nxd source files.

**Current packaged validator platforms:**

    Linux x64
    Windows x64

**Planned platform support:**

    macOS Intel / x64
    macOS Apple Silicon / ARM64

Async / Concurrency Model

NXD defines concurrency at the language level rather than adopting the concurrency model of an individual backend.

Current concurrency concepts include:
Process

An NXD process represents an independently executing concurrent unit with language-defined process semantics.
Task

A TASK represents managed asynchronous work.

Tasks participate in async-context validation and can compose with valid asynchronous operations.

Nested concurrency expressions are validated inside-out, so each operation remains responsible for its own semantic rules and diagnostics.
Channel

A CHANNEL represents typed communication infrastructure.

Channel declaration is not inherently restricted to an ASYNC FUNC. This allows communication infrastructure to be established separately from the asynchronous operations that use it.

Active operations such as SEND and RECV retain their own context and target/source requirements.
Sync

SYNC represents an explicit synchronization boundary.

Unlike AWAIT, SYNC is not inherently restricted to an ASYNC FUNC.

The synchronization model is intentionally distinct from AWAIT:

    AWAIT is completion-oriented.
    SYNC is synchronization/state-oriented.

Additional general synchronization semantics remain under development.
Optional Async Guards and Controllers

NXD also defines optional control semantics:

    PING — prerequisite / handshake validation for direct SEND or RECV operations
    ESC — cancellation/abort control for eligible asynchronous work
    TIMEOUT — bounded execution/retry lifetime for eligible asynchronous work

PING, ESC, and TIMEOUT are optional. They are not valid against every concurrency construct.

For example, PING validates its immediate target and is restricted to SEND or RECV. Wrapping an incompatible operation does not make it a valid PING target.
Backend Philosophy

NXD language semantics are independent of backend implementation details.

**NXD may target multiple backend languages, including:**

    Nim
    D
    Elixir

Backend-native concurrency primitives are implementation mechanisms, not definitions of NXD behavior.

Semantic validity is determined by NXD before backend lowering. Where a target cannot directly represent required NXD behavior, the backend/runtime layer is responsible for preserving the NXD semantics.
Platform Support
Platform	LSP Validator
Linux x64	Supported
Windows x64	Supported
macOS Intel / x64	Planned
macOS Apple Silicon / ARM64	Planned
Current Development

### Current NXD compiler and tooling work is focused on:

    Completing async/concurrency semantic validation
    Expanding TASK, CHANNEL, and SYNC behavior
    Refining PING, ESC, and TIMEOUT compatibility
    Expanding async/concurrency test coverage
    Completing Channel[T] integration
    Refining synchronization rules
    Adding structured Literal diagnostic subcodes
    Maintaining compiler/LSP diagnostic parity

### Roadmap

    macOS Intel LSP validator binary
    macOS Apple Silicon LSP validator binary
    Hover documentation
    Go To Definition
    Generic Constraints (NXD-S3003)
    Async/concurrency runtime completion
    Literal diagnostic subcodes
    Expanded semantic diagnostics
    Continued test-corpus expansion and normalization
    .nxd tooling and configuration file icon
    .nxd generated-artifact file icon

#### Version
**Current extension version: v0.2.15**