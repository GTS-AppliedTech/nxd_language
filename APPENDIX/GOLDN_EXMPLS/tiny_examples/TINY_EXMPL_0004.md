Here comes **Golden Example #4 — Ownership**, clean, isolated, and fully aligned with the syntax you approved.  
Your open tabs are unrelated to NXD, so I ignore them and stay focused on your language.

---

# ⭐ Golden Example #4 — Ownership (Tiny Layer)

### JSON‑LD Metadata
```jsonld
{
  "@context": "https://schema.nxd.dev/example",
  "example_id": "nxd_ownership_01",
  "category": "Ownership",
  "layer": "tiny",
  "features": ["move", "borrow", "clone", "immutability", "value-transfer"],
  "backend_targets": ["nim", "elixir", "d"]
}
```

---

# ⭐ Problem Statement
Demonstrate NXD’s ownership semantics:

- `MOVE` transfers ownership  
- `BORROW` creates an immutable borrow  
- `CLONE` creates a deep copy  
- Show how LET/CONST interact with ownership  
- Show backend lowering behavior  

This example is intentionally small but captures the core ownership model.

---

# ⭐ Canonical NXD (≤30 lines)
```nxd
MODULE demo.ownership

TYPE POINT { X: int, Y: int }

FUNC MAIN():
    LET A SET POINT { X: 10, Y: 20 }

    # Immutable borrow
    LET B SET BORROW A
    PRINTLN(B.X)

    # Deep clone
    LET C SET CLONE A
    C.X SET 99

    # Ownership move
    LET D SET MOVE A

    PRINTLN(D.X)
    PRINTLN(D.Y)

    RETURN none
```

---

# ⭐ Semantic Notes
- `BORROW A` creates an immutable borrow; multiple borrows allowed.  
- `CLONE A` creates a deep copy; mutations to `C` do not affect `A`.  
- `MOVE A` transfers ownership; after this, `A` is considered “moved-from.”  
- NXD does **not** enforce Rust‑style borrow checking; these are semantic hints.  
- Backends lower MOVE/BORROW/CLONE differently:  
  - Nim → ARC/ORC hints  
  - Elixir → immutable heap (borrow = alias)  
  - D → RAII + copy/move semantics  

---

# ⭐ Backend Outputs

### Nim
```nim
type Point = object
  x: int
  y: int

proc main() =
  var a = Point(x: 10, y: 20)

  let b = a  # borrow = alias (immutable by convention)
  echo b.x

  var c = a  # clone = copy
  c.x = 99

  var d = a  # move = alias (semantic)
  echo d.x
  echo d.y
```

---

### Elixir
```elixir
defmodule Demo.Ownership do
  defstruct [:x, :y]

  def main() do
    a = %Demo.Ownership{x: 10, y: 20}

    b = a   # borrow = alias (immutable heap)
    IO.puts(b.x)

    c = %{a | x: 99}  # clone = copy-on-write

    d = a   # move = alias (semantic only)
    IO.puts(d.x)
    IO.puts(d.y)
  end
end
```

---

### D
```d
module demo.ownership;

import std.stdio;

struct Point {
    int x;
    int y;
}

void main() {
    Point a = Point(10, 20);

    auto b = a; // borrow = copy (immutable by convention)
    writeln(b.x);

    auto c = a; // clone = copy
    c.x = 99;

    auto d = a; // move = alias (semantic)
    writeln(d.x);
    writeln(d.y);
}
```

---

# ⭐ Audit Rules
- `BORROW` must not allow mutation of the borrowed value.  
- `CLONE` must produce a deep copy.  
- `MOVE` must invalidate the original binding semantically.  
- Backend lowering must preserve immutability guarantees.  
- No implicit conversions allowed.  

---

If you're ready, I’ll send **Golden Example #5 — Security** next.