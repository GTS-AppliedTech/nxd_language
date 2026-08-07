ERROR_HNDL_CLAIM.md

### ERROR HANDLING CLAIM

NXD enforces explicit error handling through closed union types (RESULT, OPTION) and exhaustive pattern matching.

Because NXD provides:

- explicit error values (ERR)
- explicit success values (OK)
- explicit optional values (SOME, NONE)
- exhaustive MATCH semantics
- no exceptions
- no implicit error propagation
- no hidden failure paths

Some error‑handling violations are unrepresentable (e.g., implicit exception propagation).
Others are statically detectable (e.g., non‑exhaustive match).

Therefore, NXD guarantees that failure paths are represented explicitly in program structure.
Error conditions cannot be hidden behind exceptions, implicit propagation, or unchecked optional values.
Failure handling remains structurally visible, analyzable, and enforceable through static analysis.