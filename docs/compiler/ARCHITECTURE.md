---
{
  "@context": "https://nxdlang.org/schema",
  "doc_id": "CP001",
  "title": "",
  "description": "",
  "layer": "compiler",
  "category": "compiler",
  "keywords": [],
  "doc_version": "",
  "status": "active"
}
---

# CP001 ARCHITECTURE

Layer	Role	
Frontend	Parse NXD → AST	
Semantic	Analyze AST → typed/validated AST	
Lowering	AST → IR	
Optimization	IR → optimized IR	
Backend	IR → Nim / Elixir / D code	
Tooling/Agents	Docs, linting, refactors, audits	


1. Frontend (Lexer + Parser)

• Lexer:• Enforces your rules: capitalized identifiers/keywords/operators, lowercase literals.
• Emits tokens: IDENT, KEYWORD, OP, LITERAL, PUNCT, INDENT/DEDENT.

• Parser:• Builds the AST node list you defined: PROGRAM_NODE, MODULE_DECL_NODE, FUNC_DECL_NODE, IF_STMT_NODE, MATCH_STMT_NODE, etc.
• Uses indentation + : for block structure.


2. Semantic analysis

• Name resolution: link identifiers to declarations.
• Type checking: apply primitive types + user types + generics.
• Flow checks: unreachable code, missing returns, invalid patterns.
• Concurrency checks: valid SPAWN/SEND/RECV/AWAIT usage.
• Ownership/memory checks: MOVE/CLONE/BORROW rules.


Output: a typed AST ready for lowering.


3. IR lowering

• Use the IR lowering rules you defined:• Statements → IRInstr (IRLet, IRReturn, IRIf, IRMatch, IRSpawn, etc.).
• Expressions → IRValue (IRBinaryOp, IRCall, IRLiteral, IRVar, etc.).
• Patterns → IRPattern*.



Output: normalized IR with no syntactic sugar.


4. Optimization layer

• Simple passes first:• Constant folding (1 ADD 2 → 3).
• Dead code elimination.
• Inlined small functions.
• Pattern simplification.



Later: backend‑specific optimizations (e.g., Nim/D performance hints).


5. Backend layer (multi‑target)

• Nim backend:• IR → Nim procs, types, modules, case, try/except, async.

• Elixir backend:• IR → defmodule, def, case, spawn, send, receive, Task.await.

• D backend:• IR → module, struct, enum, switch, try/catch, threads/fibers.



Each backend is a pure IR→code generator.


6. Tooling & agents

• Doc generator: walks AST/IR → language spec, API docs, examples.
• Linting agent: inspects AST/IR → style, safety, best practices.
• Audit/pen‑test agents: operate on IR → concurrency, memory, security checks.
• Refactor agent: transforms AST → new AST (e.g., extract function, rename).


7. Compiler pipeline (end‑to‑end)

1. Source .nxd
2. Lexer → tokens
3. Parser → AST
4. Semantic analysis → typed AST
5. Lowering → IR
6. Optimization → optimized IR
7. Backend → Nim/Elixir/D code
8. Call target compiler (nim/elixir/dmd)

