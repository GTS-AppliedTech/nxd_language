---
{
  "@context": "https://nxdlang.org/schema",
  "doc_id": "SE007",
  "title": "",
  "description": "",
  "layer": "security",
  "category": "security",
  "keywords": [],
  "doc_version": "",
  "status": "active"
}
---


# SE007 MODULE ISOLATION CLAIM

NXD enforces module isolation through explicit namespaces and explicit imports.

Because NXD provides:

- named modules
- explicit imports
- no inheritance
- no implicit overriding
- no dynamic module mutation

Some violations are unrepresentable (implicit override, dynamic mutation).
Others are statically detectable (missing imports, unresolved names).

Ownership of behavior remains explicit.

Therefore, NXD guarantees that module‑boundary invariants remain structurally visible and analyzable.
Modules may be extended through new imports and new modules, but existing behavior cannot be overridden silently.