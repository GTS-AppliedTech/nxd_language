Here comes **Golden Example #9 — Compiler Plugin**, clean, isolated, and fully aligned with your approved syntax and JSON‑LD metadata.  
Your browser tabs are unrelated to NXD, so I stay fully focused on your language.

---

# ⭐ Golden Example #9 — Compiler Plugin (Tiny Layer)

### JSON‑LD Metadata
```jsonld
{
  "@context": "https://schema.nxd.dev/example",
  "example_id": "nxd_plugin_01",
  "category": "CompilerPlugin",
  "layer": "tiny",
  "features": ["plugin", "compile-time-transform", "ast-hook", "diagnostics"],
  "backend_targets": ["nim", "elixir", "d"]
}
```

---

# ⭐ Problem Statement
Demonstrate NXD’s **compiler plugin** mechanism:

- Plugins run at **compile time**  
- They inspect or transform AST nodes  
- They can emit diagnostics  
- They can enforce rules (e.g., forbid certain patterns)  
- They do **not** execute at runtime  
- They are backend‑agnostic  

This example shows a plugin that **forbids subtraction** (`SUB`) and emits a diagnostic if detected.

---

# ⭐ Canonical NXD (≤30 lines)
```nxd
MODULE demo.plugin

PLUGIN FORBID_SUBTRACTION:
    ON AST(OPERATOR):
        IF OP.NAME EQ "SUB":
            DIAGNOSTIC E7001 "subtraction is forbidden"
    END

FUNC CALC():
    LET A SET 10 SUB 3   # plugin will reject this
    RETURN A

FUNC MAIN():
    RETURN CALC()
```

---

# ⭐ Semantic Notes
- `PLUGIN NAME:` declares a compile‑time plugin.  
- `ON AST(OPERATOR)` hooks into operator nodes during compilation.  
- `DIAGNOSTIC E7001` emits a structured compile‑time error.  
- Plugins cannot mutate runtime values — only AST.  
- Plugins run **before lowering** to Nim/Elixir/D.  
- This plugin forbids the `SUB` operator entirely.  
- `CALC()` will fail compilation due to the plugin.  

---

# ⭐ Backend Outputs  
Plugins do **not** lower to backend code.  
Instead, they prevent backend code generation when rules are violated.

Below are the backend equivalents **if the plugin did NOT reject the code** (for illustration only).

### Nim
```nim
proc calc(): int =
  let a = 10 - 3
  return a

proc main() =
  echo calc()
```

### Elixir
```elixir
defmodule Demo.Plugin do
  def calc() do
    a = 10 - 3
    a
  end

  def main() do
    IO.puts(calc())
  end
end
```

### D
```d
module demo.plugin;

import std.stdio;

int calc() {
    int a = 10 - 3;
    return a;
}

void main() {
    writeln(calc());
}
```

---

# ⭐ Audit Rules
- Plugins must run at compile time only.  
- Plugins must not modify runtime semantics.  
- AST hooks must match node categories (`OPERATOR`, `CALL`, `STRUCT`, etc.).  
- Diagnostics must use structured codes (`E7001`).  
- Backend lowering must halt if plugin emits an error.  
- No implicit conversions allowed in plugin logic.  

---

If you're ready, I’ll send **Golden Example #10 — Multi‑Module** next.