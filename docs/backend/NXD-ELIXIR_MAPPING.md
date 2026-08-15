---
{
  "@context": "https://nxdlang.org/schema",
  "doc_id": "BE003",
  "title": "",
  "description": "",
  "layer": "backend",
  "category": "backend",
  "keywords": [],
  "doc_version": "",
  "status": "active"
}
---

# BE003 MAPPING ELIXIR

1. Modules

NXD:

MODULE MATH


Elixir:

defmodule Math do
end


• NXD MODULE NAME → defmodule Name do ... end
• File name: math.ex


2. Imports / aliases

NXD:

IMPORT UTIL AS U


Elixir:

alias Util, as: U


• IMPORT → alias or import depending on semantics.


3. Types

NXD TYPE mostly maps to Elixir structs or plain maps.

Struct:

NXD:

TYPE PERSON { NAME: string, AGE: int }


Elixir:

defmodule Person do
  defstruct [:name, :age]
end


Enums/unions often map to tagged tuples or atoms:

NXD:

TYPE RESULT UNION { OK(string), ERR(int) }


Elixir:

# {:ok, value} | {:error, reason}


4. Functions

NXD:

FUNC ADD(X, Y):
    RETURN X ADD Y


Elixir:

def add(x, y) do
  x + y
end


• FUNC NAME(...) → def name(...) do ... end
• Capitalized identifiers → lowercase Elixir variables.


5. Control flow

IF / ELSE

NXD:

IF X GT 10:
    RETURN X
ELSE:
    RETURN 0


Elixir:

if x > 10 do
  x
else
  0
end


MATCH / CASE → case:

NXD:

MATCH N:
    CASE 0:
        RETURN 1
    OTHERWISE:
        RETURN N MUL FACTORIAL(N SUB 1)


Elixir:

case n do
  0 ->
    1

  _ ->
    n * factorial(n - 1)
end


6. Operators

NXD	Elixir	
ADD	+	
SUB	-	
MUL	*	
DIV	/	
MOD	rem	
EQ	==	
NEQ	!=	
GT	>	
LT	<	
GTE	>=	
LTE	<=	
AND	and	
OR	or	
NOT	not	


Expressions lower from IR into normal Elixir infix.


7. Literals

• true / false → true / false
• none → nil
• Lists: [1, 2, 3] → [1, 2, 3]
• Maps: { "a": 1 } → %{"a" => 1}


8. Concurrency

This is where NXD maps directly to BEAM semantics.

SPAWN

NXD:

SPAWN WORK()


Elixir:

spawn(fn -> work() end)


SEND / RECV

NXD:

SEND MSG TO PID
RECV X


Elixir:

send(pid, msg)

receive do
  x -> x
end


AWAIT (for async tasks):

NXD:

AWAIT TASK


Elixir:

Task.await(task)


9. Error handling

NXD:

TRY:
    ...
CATCH E:
    ...


Elixir:

try do
  ...
rescue
  e -> ...
end


Or pattern‑matching on {:ok, _} | {:error, _} if you model errors that way.


10. Full example

NXD:

MODULE MATH

FUNC FACTORIAL(N):
    MATCH N:
        CASE 0:
            RETURN 1
        OTHERWISE:
            RETURN N MUL FACTORIAL(N SUB 1)


Elixir:

defmodule Math do
  def factorial(0), do: 1

  def factorial(n) do
    n * factorial(n - 1)
  end
end


Use case wise: NXD becomes a high‑level, concurrency‑aware language that can drop straight into Elixir’s ecosystem while keeping a consistent syntax across Nim and D.
