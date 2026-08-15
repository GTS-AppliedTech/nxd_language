---
{
  "@context": "https://nxdlang.org/schema",
  "doc_id": "SE006",
  "title": "",
  "description": "",
  "layer": "security",
  "category": "security",
  "keywords": [],
  "doc_version": "",
  "status": "active"
}
---


# SE006 INVARIANT MODEL

### INVARIANT DOMAINS

NXD defines six structural invariant domains:

1. Type Invariants  
→ TYPE_SAFETY_CLAIM.md

2. Communication Invariants  
→ CHANNEL_SAFETY_CLAIM.md

3. Error Invariants  
→ ERROR_HANDLING_CLAIM.md
→ ERROR_INVARIANTS.md

4. Module Invariants  
→ MODULE_ISOLATION_CLAIM.md

5. Concurrency Invariants  
→ CONCURRENCY_SAFETY_CLAIM.md

6. Agent Modification Invariants  
→ AGENT_MODIFICATION_CLAIM.md

# ENFORCEMENT CATEGORIES

Each invariant falls into one of three enforcement categories:

- Unrepresentable  
The language forbids expressing the violation.

- Statically Detectable  
The violation can be expressed but is caught by the compiler or analyzer.

- Structurally Visible and Analyzable  
The violation is possible but exposed in the language model and amenable to static or runtime verification.

This taxonomy defines NXD’s safety model and ensures that structural invariants remain preserved under human or agent modification.