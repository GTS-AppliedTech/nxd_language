Here comes **Golden Example #6 — Error Handling**, clean, isolated, and fully aligned with your approved syntax and JSON‑LD metadata.  
Your browser tabs are unrelated to NXD, so I stay fully focused on your language.

---

# ⭐ Golden Example #6 — Error Handling (Tiny Layer)

### JSON‑LD Metadata
```jsonld
{
  "@context": "https://schema.nxd.dev/example",
  "example_id": "nxd_error_01",
  "category": "ErrorHandling",
  "layer": "tiny",
  "features": ["try", "catch", "result", "raise", "pattern-matching"],
  "backend_targets": ["nim", "elixir", "d"]
}
```

---

# ⭐ Problem Statement
Demonstrate NXD’s structured error‑handling model:

- `TRY / CATCH` for exception‑style handling  
- `RESULT` union type for recoverable errors  
- `RAISE` for structured diagnostics  
- Pattern matching on `RESULT`  
- No implicit conversions between `ERR` and `OK`  

This example shows a safe integer parser that returns `RESULT`, and a caller that handles both exceptions and recoverable errors.

---

# ⭐ Canonical NXD (≤30 lines)
```nxd
MODULE demo.error

TYPE RESULT UNION { OK(any), ERR(string) }

FUNC PARSE_INT(S):
    IF S IS string:
        IF S CONTAINS NON_DIGIT:
            RETURN ERR("not numeric")
        RETURN OK(S AS int)
    RAISE E5002  # invalid cast

FUNC MAIN():
    LET R SET PARSE_INT("42")

    MATCH R:
        CASE OK(V):
            PRINTLN(V)
        CASE ERR(E):
            PRINTLN("error: " ADD E)

    LET OUT SET TRY PARSE_INT("hello") CATCH E:
        RETURN ERR("exception: " ADD E)

    PRINTLN(OUT)

    RETURN none
```

---

# ⭐ Semantic Notes
- `RESULT` is a tagged union with `OK` and `ERR`.  
- `PARSE_INT` returns `ERR` for non‑numeric strings.  
- `S AS int` uses explicit casting rules (may fail).  
- `RAISE E5002` triggers a structured diagnostic.  
- `MATCH` must be exhaustive for sum types.  
- `TRY/CATCH` handles exceptions, not `ERR` values.  
- No implicit conversion between `ERR` and exceptions.  
- Evaluation order is strictly left‑to‑right.

---

# ⭐ Backend Outputs

### Nim
```nim
type Result[T] = object
  ok: bool
  val: T
  err: string

proc parseInt(s: string): Result[int] =
  if not s.allCharsInSet({'0'..'9'}):
    return Result[int](ok: false, err: "not numeric")
  try:
    return Result[int](ok: true, val: parseInt(s))
  except:
    raise newException(ValueError, "E5002")

proc main() =
  let r = parseInt("42")
  if r.ok:
    echo r.val
  else:
    echo "error: ", r.err

  var out: Result[int]
  try:
    out = parseInt("hello")
  except ValueError:
    out = Result[int](ok: false, err: "exception")

  echo out
```

---

### Elixir
```elixir
defmodule Demo.Error do
  def parse_int(s) when is_binary(s) do
    if String.match?(s, ~r/^\d+$/) do
      {:ok, String.to_integer(s)}
    else
      {:error, "not numeric"}
    end
  end

  def parse_int(_), do: raise "E5002"

  def main() do
    r = parse_int("42")

    case r do
      {:ok, v} -> IO.puts(v)
      {:error, e} -> IO.puts("error: " <> e)
    end

    out =
      try do
        parse_int("hello")
      rescue
        _ -> {:error, "exception"}
      end

    IO.inspect(out)
  end
end
```

---

### D
```d
module demo.error;

import std.stdio;
import std.conv;

struct Result(T) {
    bool ok;
    T val;
    string err;
}

Result!int parseInt(string s) {
    import std.regex;
    if (!match(s, regex(`^\d+$`)))
        return Result!int(false, 0, "not numeric");

    try {
        return Result!int(true, to!int(s), "");
    } catch (Exception e) {
        throw new Exception("E5002");
    }
}

void main() {
    auto r = parseInt("42");
    if (r.ok)
        writeln(r.val);
    else
        writeln("error: ", r.err);

    Result!int out;
    try {
        out = parseInt("hello");
    } catch (Exception e) {
        out = Result!int(false, 0, "exception");
    }

    writeln(out);
}
```

---

# ⭐ Audit Rules
- `RESULT` must be pattern‑matched exhaustively.  
- `TRY/CATCH` must not swallow `ERR` values.  
- `RAISE` must produce structured diagnostics (`E5002`).  
- No implicit conversions allowed between exceptions and `ERR`.  
- Backend lowering must preserve sum‑type semantics.  

---

If you're ready, I’ll send **Golden Example #7 — Generics** next.