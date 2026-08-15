---
{
  "@context": "https://nxdlang.org/schema",
  "doc_id": "SE003",
  "title": "",
  "description": "",
  "layer": "security",
  "category": "security",
  "keywords": [],
  "doc_version": "",
  "status": "active"
}
---


# SE003 Concurrency Safety Claim 

NXD eliminates shared‑memory concurrency hazards by design.
Concurrency is expressed exclusively through actor isolation and typed message passing.

Because NXD provides no shared mutable memory, no lock primitives, no mutexes, no semaphores, no atomic synchronization constructs, and no blocking critical sections, race conditions and lock‑based deadlocks are unrepresentable within the language.

NXD does not eliminate protocol‑level deadlocks, such as circular waits between actors.
These are message‑level liveness failures, not shared‑memory deadlocks, and arise from protocol design rather than memory contention.

Protocol‑level liveness failures remain expressible because they are properties of actor interaction rather than memory access; however, they are represented entirely through explicit message‑flow constructs and are therefore amenable to static analysis and runtime verification.

Because all blocking points (RECV), message types, and communication edges are explicit, message‑level deadlocks and liveness failures are structurally visible and can be analyzed through:
- message‑flow graphs
- dependency cycles
- actor wait‑graphs
- protocol assertions
- static analyzers

Therefore, NXD eliminates shared‑memory concurrency hazards while making remaining message‑level coordination issues detectable, analyzable, and correctable, rather than implicit or hidden.