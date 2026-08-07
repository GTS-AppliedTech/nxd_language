ERROR_INVARIANTS.md

### NXD ERROR INVARIATS

NXD’s error invariants are:

1. All functions that may fail return a RESULT.

2. All optional values are represented as OPTION.

3. All RESULT and OPTION values must be matched exhaustively.

4. No implicit success or failure paths exist.

5. No exception‑based control flow exists.

6. All error propagation is explicit.

7. All error values are typed and structured.

8. Non‑exhaustive handling of RESULT and OPTION values is a compiler or analyzer failure.

This explicitly ties each invariant to its enforcement mechanism.