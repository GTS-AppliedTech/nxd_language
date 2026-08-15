Here comes **Golden Example #5 — Security**, clean, isolated, and fully aligned with your approved syntax and JSON‑LD metadata.  
Your browser tabs are unrelated to NXD, so I ignore them and stay focused on your language.

---

# ⭐ Golden Example #5 — Security (Tiny Layer)

### JSON‑LD Metadata
```jsonld
{
  "@context": "https://schema.nxd.dev/example",
  "example_id": "nxd_security_01",
  "category": "Security",
  "layer": "tiny",
  "features": ["capabilities", "revocation", "restricted-functions", "error-handling"],
  "backend_targets": ["nim", "elixir", "d"]
}
```

---

# ⭐ Problem Statement
Demonstrate NXD’s capability‑based security model:

- Capabilities are explicit values passed to functions  
- Restricted operations require a capability  
- Capabilities can be revoked  
- Unauthorized use triggers a security error (`E4001`, `E4002`)  
- No implicit privilege escalation is allowed  

This example shows a simple “secure write” operation that requires a capability token.

---

# ⭐ Canonical NXD (≤30 lines)
```nxd
MODULE demo.security

TYPE WRITE_CAP { TOKEN: string }

FUNC SECURE_WRITE(CAP, DATA):
    IF CAP IS none:
        RAISE E4001  # capability revoked

    IF CAP.TOKEN NEQ "valid":
        RAISE E4002  # capability not delegable

    PRINTLN("writing: " ADD DATA)
    RETURN OK("done")

FUNC MAIN():
    LET CAP SET WRITE_CAP { TOKEN: "valid" }

    LET R1 SET SECURE_WRITE(CAP, "hello")

    # revoke capability
    LET CAP SET none

    LET R2 SET TRY SECURE_WRITE(CAP, "fail") CATCH E:
        RETURN ERR("security violation")

    PRINTLN(R1)
    PRINTLN(R2)

    RETURN none
```

---

# ⭐ Semantic Notes
- Capabilities are explicit values; no ambient authority.  
- `CAP IS none` checks revocation.  
- `TOKEN NEQ "valid"` simulates a permission check.  
- `RAISE` triggers structured diagnostics (`E4001`, `E4002`).  
- `TRY/CATCH` handles security violations safely.  
- No implicit conversions or privilege escalation.  
- All evaluation is left‑to‑right.

---

# ⭐ Backend Outputs

### Nim
```nim
type WriteCap = object
  token: string

proc secureWrite(cap: WriteCap, data: string): string =
  if cap.token == "":
    raise newException(ValueError, "E4001")
  if cap.token != "valid":
    raise newException(ValueError, "E4002")
  echo "writing: " & data
  return "ok"

proc main() =
  var cap = WriteCap(token: "valid")
  let r1 = secureWrite(cap, "hello")

  cap.token = ""  # revoke
  var r2: string
  try:
    r2 = secureWrite(cap, "fail")
  except ValueError:
    r2 = "security violation"

  echo r1
  echo r2
```

---

### Elixir
```elixir
defmodule Demo.Security do
  defstruct [:token]

  def secure_write(%Demo.Security{token: ""}, _data),
    do: raise "E4001"

  def secure_write(%Demo.Security{token: t}, _data) when t != "valid",
    do: raise "E4002"

  def secure_write(%Demo.Security{} = cap, data) do
    IO.puts("writing: " <> data)
    {:ok, "done"}
  end

  def main() do
    cap = %Demo.Security{token: "valid"}
    r1 = secure_write(cap, "hello")

    cap = %Demo.Security{token: ""} # revoke

    r2 =
      try do
        secure_write(cap, "fail")
      rescue
        _ -> {:error, "security violation"}
      end

    IO.inspect(r1)
    IO.inspect(r2)
  end
end
```

---

### D
```d
module demo.security;

import std.stdio;

struct WriteCap {
    string token;
}

string secureWrite(WriteCap cap, string data) {
    if (cap.token == "")
        throw new Exception("E4001");
    if (cap.token != "valid")
        throw new Exception("E4002");

    writeln("writing: ", data);
    return "ok";
}

void main() {
    WriteCap cap = WriteCap("valid");
    auto r1 = secureWrite(cap, "hello");

    cap.token = ""; // revoke

    string r2;
    try {
        r2 = secureWrite(cap, "fail");
    } catch (Exception e) {
        r2 = "security violation";
    }

    writeln(r1);
    writeln(r2);
}
```

---

# ⭐ Audit Rules
- Capability tokens must be explicit values.  
- Revocation must be represented as `none` or empty token.  
- Unauthorized access must raise `E4001` or `E4002`.  
- No implicit privilege escalation allowed.  
- Backend lowering must preserve capability semantics.  

---

If you're ready, I’ll send **Golden Example #6 — Error Handling** next.