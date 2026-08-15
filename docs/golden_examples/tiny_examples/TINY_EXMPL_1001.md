


## Golden Example #1 — CRUD (Create/Read/Update/Delete)

### JSON‑LD Metadata
```jsonld
{
  "@context": "https://schema.nxd.dev/example",
  "example_id": "nxd_crud_01",
  "category": "CRUD",
  "layer": "tiny",
  "features": ["structs", "mutation", "value-equality"],
  "backend_targets": ["nim", "elixir", "d"]
}
```



## Problem Statement
Demonstrate basic CRUD operations on a `PERSON` struct using NXD’s `LET`, `SET`, field mutation, and value equality semantics.



## Canonical NXD (≤30 lines)
```nxd
MODULE demo.crud

TYPE PERSON { NAME: string, AGE: int }

FUNC MAIN():
    # Create
    LET P SET PERSON { NAME: "gabriel", AGE: 30 }

    # Read
    PRINTLN(P.NAME)
    PRINTLN(P.AGE)

    # Update
    P.AGE SET 31

    # Delete (simulate by setting to none)
    LET P2 SET none

    # Equality check
    IF P IS PERSON:
        PRINTLN("valid person")

    RETURN none
```

---

## Semantic Notes
- `LET P SET PERSON {...}` creates a mutable binding.  
- Struct fields are mutable even under `CONST`, but here `LET` is used.  
- `P.AGE SET 31` uses NXD’s mutation semantics.  
- `none` is the canonical “deleted” placeholder.  
- `IS` checks type, not value.  
- No implicit conversions occur anywhere.  
- All literals follow lowercase rules.



## Backend Outputs

### Nim
```nim
type Person = object
  name: string
  age: int

proc main() =
  var p = Person(name: "gabriel", age: 30)
  echo p.name
  echo p.age
  p.age = 31
  var p2: Person
  # p2 = nil not allowed; simulate delete differently
  echo "valid person"
```

### Elixir
```elixir
defmodule Demo.Crud do
  defstruct [:name, :age]

  def main() do
    p = %Demo.Crud{name: "gabriel", age: 30}
    IO.puts(p.name)
    IO.puts(p.age)
    p = %{p | age: 31}
    p2 = nil
    IO.puts("valid person")
  end
end
```

### D
```d
module demo.crud;

struct Person {
    string name;
    int age;
}

void main() {
    Person p = Person("gabriel", 30);
    writeln(p.name);
    writeln(p.age);
    p.age = 31;
    Person* p2 = null;
    writeln("valid person");
}
```



## Audit Rules
- Struct mutation must follow LET/CONST semantics.  
- No implicit conversions allowed.  
- Type checks must use `IS`.  
- Backend outputs must preserve value semantics.  
- Delete simulation must use `none` or backend‑appropriate null.  

