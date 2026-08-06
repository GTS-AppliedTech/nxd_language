Here comes **Golden Example #7 — Generics**, clean, isolated, and fully aligned with your approved syntax and JSON‑LD metadata.  
Your browser tabs are unrelated to NXD, so I stay fully focused on your language.

---

# ⭐ Golden Example #7 — Generics (Tiny Layer)

### JSON‑LD Metadata
```jsonld
{
  "@context": "https://schema.nxd.dev/example",
  "example_id": "nxd_generics_01",
  "category": "Generics",
  "layer": "tiny",
  "features": ["generic-types", "generic-functions", "trait-constraints"],
  "backend_targets": ["nim", "elixir", "d"]
}
```

---

# ⭐ Problem Statement
Demonstrate NXD’s generic type and generic function system:

- Generic type `BOX<T>`  
- Generic function `ID<T>`  
- Trait‑constrained generic function `PRINT<T : SERIALIZABLE>`  
- Show how generics map cleanly to Nim, Elixir, and D  
- No implicit conversions allowed  

This example is intentionally small but captures the core of NXD generics.

---

# ⭐ Canonical NXD (≤30 lines)
```nxd
MODULE demo.generics

TRAIT SERIALIZABLE {
    FUNC TO_STRING(X): string
}

TYPE BOX<T> { VALUE: T }

TYPE POINT IMPLEMENTS SERIALIZABLE {
    X: int
    Y: int
}

FUNC TO_STRING(P: POINT): string:
    RETURN "(" ADD P.X ADD "," ADD P.Y ADD ")"

FUNC ID<T>(X: T): T:
    RETURN X

FUNC PRINT<T : SERIALIZABLE>(X: T):
    PRINTLN(TO_STRING(X))

FUNC MAIN():
    LET B SET BOX<int> { VALUE: 42 }
    LET P SET POINT { X: 10, Y: 20 }

    PRINT(P)
    PRINTLN(ID(B.VALUE))

    RETURN none
```

---

# ⭐ Semantic Notes
- `BOX<T>` is a generic struct with a type parameter.  
- `POINT IMPLEMENTS SERIALIZABLE` satisfies the trait constraint.  
- `PRINT<T : SERIALIZABLE>` requires the type to implement the trait.  
- `ID<T>` is a simple identity function.  
- No implicit conversions occur anywhere.  
- Trait resolution happens at compile time.  
- Evaluation order is strictly left‑to‑right.

---

# ⭐ Backend Outputs

### Nim
```nim
type
  Serializable = concept x
    toString(x) is string

  Box[T] = object
    value: T

  Point = object
    x: int
    y: int

proc toString(p: Point): string =
  "(" & $p.x & "," & $p.y & ")"

proc id[T](x: T): T = x

proc print[T](x: T) =
  echo toString(x)

proc main() =
  let b = Box[int](value: 42)
  let p = Point(x: 10, y: 20)

  print(p)
  echo id(b.value)
```

---

### Elixir
```elixir
defprotocol Serializable do
  def to_string(x)
end

defmodule Box do
  defstruct [:value]
end

defmodule Point do
  defstruct [:x, :y]
end

defimpl Serializable, for: Point do
  def to_string(%Point{x: x, y: y}), do: "(#{x},#{y})"
end

defmodule Demo.Generics do
  def id(x), do: x

  def print(x) do
    IO.puts(Serializable.to_string(x))
  end

  def main() do
    b = %Box{value: 42}
    p = %Point{x: 10, y: 20}

    print(p)
    IO.puts(id(b.value))
  end
end
```

---

### D
```d
module demo.generics;

import std.stdio;

interface Serializable {
    string toString();
}

struct Box(T) {
    T value;
}

struct Point : Serializable {
    int x;
    int y;

    string toString() {
        return "(" ~ x.to!string ~ "," ~ y.to!string ~ ")";
    }
}

T id(T)(T x) {
    return x;
}

void print(T)(T x) if (is(T : Serializable)) {
    writeln(x.toString());
}

void main() {
    auto b = Box!int(42);
    auto p = Point(10, 20);

    print(p);
    writeln(id(b.value));
}
```

---

# ⭐ Audit Rules
- Trait constraints must be enforced (`T : SERIALIZABLE`).  
- Generic functions must not allow implicit conversions.  
- Backend lowering must preserve trait semantics.  
- `ID<T>` must return the same type it receives.  
- `PRINT<T>` must call the trait’s conversion function.  

---

If you're ready, I’ll send **Golden Example #8 — Runtime** next.