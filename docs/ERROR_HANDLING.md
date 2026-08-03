ERROR_HANDLING.md

NXD Error Handling Specification

NXD defines a unified, semantic error model that works consistently across Nim, Elixir, and D.
The user sees one system, while backends map it to their native mechanisms.

NXD error handling is built on three pillars:

• Result types — for recoverable errors
• Option types — for nullable/optional values
• Exceptions — for exceptional or unrecoverable conditions
• Try/Catch/Finally — structured error control flow


1. Error categories

NXD defines three categories of errors:

1. Recoverable errors

Represented using the RESULT type:

TYPE RESULT UNION { OK(any), ERR(string) }


Used for:

• validation failures
• IO errors
• user‑level recoverable conditions
• domain errors


2. Optional absence

Represented using the OPTION type:

TYPE OPTION UNION { SOME(any), NONE }


Used for:

• missing values
• optional fields
• nullable semantics


3. Exceptional errors

Represented using THROW and TRY/CATCH.

Used for:

• unexpected conditions
• invariants
• runtime failures
• system errors


2. Result type semantics

Construction

OK(V)
ERR(MSG)


Consumption

Pattern matching:

MATCH R:
    CASE OK(V):
        ...
    CASE ERR(E):
        ...


Helpers

• IS_OK(R)
• IS_ERR(R)
• UNWRAP_OR(R, DEFAULT)
• MAP_OK(R, FN)


Philosophy

Recoverable errors should use RESULT.
Exceptions should be reserved for truly exceptional conditions.


3. Option type semantics

Construction

SOME(V)
NONE


Consumption

MATCH O:
    CASE SOME(V):
        ...
    CASE NONE:
        ...


Helpers

• IS_SOME(O)
• IS_NONE(O)
• UNWRAP_OR(O, DEFAULT)


Philosophy

Use OPTION when absence is expected and not an error.


4. Exceptions

NXD supports exceptions for exceptional conditions.

Throwing

THROW "Something went wrong"


Catching

TRY:
    ...
CATCH E:
    ...
FINALLY:
    ...


Semantics

• THROW immediately unwinds the current call stack.
• CATCH binds the error message or error object.
• FINALLY always executes.


Philosophy

Exceptions are for unexpected or unrecoverable conditions.
They should not be used for normal control flow.


5. Backend mapping

Nim

• RESULT → Result[T]
• OPTION → Option[T] or ref T
• THROW → raise newException
• TRY/CATCH → try/except/finally


Elixir

• RESULT → {:ok, v} / {:error, msg}
• OPTION → {:some, v} / :none or nil
• THROW → raise
• TRY/CATCH → try/rescue/after


D

• RESULT → Result!T
• OPTION → Nullable!T
• THROW → throw new Exception
• TRY/CATCH → try/catch/finally


Backends preserve semantics even if their native mechanisms differ.


6. Error propagation rules

Result propagation

Functions returning RESULT must propagate errors explicitly:

FUNC LOAD_FILE(PATH):
    LET R SET READ(PATH)
    MATCH R:
        CASE OK(DATA):
            RETURN OK(DATA)
        CASE ERR(E):
            RETURN ERR(E)


Option propagation

FUNC FIRST(LIST):
    IF LIST IS EMPTY:
        RETURN NONE
    OTHERWISE:
        RETURN SOME(LIST[0])


Exception propagation

Exceptions propagate automatically unless caught.


7. Concurrency error rules

Process errors

If a process throws an exception:

• It terminates.
• Its parent may receive a PROCESS_ERR message (future spec).
• Tasks wrapping processes convert exceptions into ERR.


Task errors

If a task fails:

LET T SET TASK(FN)
LET R SET AWAIT T


R becomes:

ERR("task failed: ...")


Channel errors

Sending to a closed channel:

• Raises an exception.


Receiving from a closed channel:

• Returns ERR("channel closed").


8. Error safety guarantees

NXD guarantees:

• No silent failures.
• No implicit null dereferencing.
• No implicit exception swallowing.
• All error paths are visible in IR for audit agents.
• Unsafe blocks must declare error behavior explicitly.


9. Example: unified error handling

NXD

FUNC DIVIDE(X, Y):
    IF Y EQ 0:
        RETURN ERR("division by zero")
    OTHERWISE:
        RETURN OK(X DIV Y)

FUNC SAFE_DIVIDE(X, Y):
    LET R SET DIVIDE(X, Y)
    MATCH R:
        CASE OK(V):
            RETURN V
        CASE ERR(E):
            THROW E


Nim

proc divide(x, y: int): Result[int] =
  if y == 0:
    err("division by zero")
  else:
    ok(x div y)

proc safeDivide(x, y: int): int =
  let r = divide(x, y)
  if r.isOk:
    r.ok
  else:
    raise newException(ValueError, r.err)


Elixir

def divide(x, 0), do: {:error, "division by zero"}
def divide(x, y), do: {:ok, div(x, y)}

def safe_divide(x, y) do
  case divide(x, y) do
    {:ok, v} -> v
    {:error, e} -> raise e
  end
end


D

Result!int divide(int x, int y) {
    if (y == 0) return err!int("division by zero");
    return ok!int(x / y);
}

int safeDivide(int x, int y) {
    auto r = divide(x, y);
    if (r.isOk) return r.ok;
    throw new Exception(r.err);
}

 Summary

NXD error handling is:

• Unified across all backends
• Semantic, not backend‑specific
• Explicit, with clear control flow
• Typed, via RESULT and OPTION
• Structured, via TRY/CATCH/FINALLY
• Safe, with no silent failures
• Auditable, visible in IR for agents


This system is the backbone of NXD’s reliability and security guarantees.
