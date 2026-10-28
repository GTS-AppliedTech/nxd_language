# Change Log

All notable changes to the NXD Visual Studio Code extension are documented in this file.

This project follows the general structure recommended by Keep a Changelog.

---

**[v0.2.15] 10.28.2026**
### Added

    Cross-platform LSP validator support for:
        Linux x64
        Windows x64

    Platform-specific validator binary selection for supported extension environments.
    Expanded semantic diagnostics for NXD async and concurrency behavior.

    Async/concurrency diagnostic subcodes under NXD-S3005, including validation for:
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
    Semantic support for Task[T], ProcessHandle[T], ReceiveOperation[T], and ongoing Channel[T] support.
    Parser support for newer concurrency constructs including TASK, SYNC, and CHANNEL.

### Changed

    Expanded async/concurrency validation to distinguish operation-specific failures from general ASYNC context violations.
    TASK now participates in ASYNC context validation and has dedicated invalid-target diagnostics.
    SYNC is treated as a synchronization boundary that may be used from synchronous or asynchronous contexts.
    CHANNEL declarations may exist outside an ASYNC FUNC, allowing communication infrastructure to be established independently of active asynchronous execution.
    PING validation is restricted to direct SEND and RECV targets.
    Async/concurrency expressions use inside-out semantic validation so nested operations retain ownership of their own diagnostics.
    Continued development of ESC and TIMEOUT compatibility with managed asynchronous tasks.

### Platform Support

**Currently packaged validator binaries:**

    Linux x64
    Windows x64

**Planned:**

    macOS Intel / x64
    macOS Apple Silicon / ARM64

**In Progress**

    Completion and refinement of NXD async/concurrency runtime semantics.
    Channel[T] integration with message-passing operations.
    Additional ESC, TIMEOUT, and TASK compatibility validation.
    Continued expansion of the semantic test corpus.
    Synchronization semantics beyond the async-specific diagnostic family.
    Literal diagnostic subcodes.

---

**[v0.2.6]**
**Added**

    Initial published release of the NXD Visual Studio Code extension.