---
{
  "@context": "https://nxdlang.org/schema",
  "doc_id": "SE010",
  "title": "",
  "description": "",
  "layer": "security",
  "category": "security",
  "keywords": [],
  "doc_version": "",
  "status": "active"
}
---


# SE010 Type Safety Claim 

NXD enforces type safety through explicit type declarations, closed union types, and the absence of implicit conversions.
All values must conform to declared types, and all functions specify explicit input and output types.

Because NXD provides:

- explicit record types
- closed union types
- no implicit null
- no implicit type coercion
- no structural subtyping
- no inheritance‑based polymorphism

Some violations are unrepresentable (e.g., implicit null, implicit coercion).
Other violations are statically detectable through type checking and analysis.

Therefore, NXD guarantees that all well‑typed programs preserve declared type invariants.
Violations of declared type invariants are either unrepresentable within the language or statically detectable, manifesting as compile‑time or analysis‑time failures rather than runtime corruption.