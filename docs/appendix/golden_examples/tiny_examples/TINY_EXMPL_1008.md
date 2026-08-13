Here comes **Golden Example #8 — Runtime**, clean, isolated, and fully aligned with your approved syntax and JSON‑LD metadata.  
Your browser tabs are unrelated to NXD, so I stay fully focused on your language.

---

# ⭐ Golden Example #8 — Runtime (Tiny Layer)

### JSON‑LD Metadata
```jsonld
{
  "@context": "https://schema.nxd.dev/example",
  "example_id": "nxd_runtime_01",
  "category": "Runtime",
  "layer": "tiny",
  "features": ["init-block", "module-loading", "left-to-right-eval", "cyclic-imports", "execution-order"],
  "backend_targets": ["nim", "elixir", "d"]
}
```

---

# ⭐ Problem Statement
Demonstrate NXD’s runtime semantics:

- `INIT` blocks execute once at startup  
- Execution order follows module dependency topology  
- Left‑to‑right evaluation is guaranteed  
- Show a simple multi‑module interaction  
- No cyclic INIT dependencies allowed  

This example uses two modules:  
`runtime.core` and `runtime.app`.

---

# ⭐ Canonical NXD (≤30 lines)
```nxd
MODULE runtime.core

INIT:
    PRINTLN("core init")

FUNC ADD(X, Y):
    RETURN X ADD Y


MODULE runtime.app
IMPORT runtime.core

INIT:
    PRINTLN("app init")

FUNC MAIN():
    LET A SET ADD(10, 20)
    LET B SET ADD(A, 5)
    PRINTLN(B)
    RETURN none
```

---

# ⭐ Semantic Notes
- `INIT` blocks run **once**, before any function calls.  
- Module initialization order is determined by import topology:  
  - `runtime.core` initializes first  
  - `runtime.app` initializes second  
- Left‑to‑right evaluation ensures `ADD(A, 5)` evaluates `A` first.  
- No cyclic INIT dependencies exist here.  
- Backend lowering must preserve initialization order semantics.

---

# ⭐ Backend Outputs

### Nim
```nim
# runtime/core.nim
static:
  echo "core init"

proc add(x, y: int): int =
  x + y

# runtime/app.nim
import runtime/core

static:
  echo "app init"

proc main() =
  let a = add(10, 20)
  let b = add(a, 5)
  echo b
```

---

### Elixir
```elixir
# runtime/core.ex
defmodule Runtime.Core do
  @on_load :init
  def init(), do: IO.puts("core init")

  def add(x, y), do: x + y
end

# runtime/app.ex
defmodule Runtime.App do
  @on_load :init
  def init(), do: IO.puts("app init")

  def main() do
    a = Runtime.Core.add(10, 20)
    b = Runtime.Core.add(a, 5)
    IO.puts(b)
  end
end
```

---

### D
```d
// runtime/core.d
module runtime.core;

static this() {
    writeln("core init");
}

int add(int x, int y) {
    return x + y;
}

// runtime/app.d
module runtime.app;

import runtime.core;

static this() {
    writeln("app init");
}

void main() {
    int a = add(10, 20);
    int b = add(a, 5);
    writeln(b);
}
```

---

# ⭐ Audit Rules
- INIT blocks must execute in topological order.  
- No cyclic INIT dependencies allowed.  
- Left‑to‑right evaluation must be preserved.  
- Backend lowering must preserve initialization semantics.  
- No implicit conversions allowed in arithmetic.  

---

If you're ready, I’ll send **Golden Example #9 — Compiler Plugin** next.