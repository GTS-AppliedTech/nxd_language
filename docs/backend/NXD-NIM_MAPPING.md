---
{
  "@context": "https://nxdlang.org/schema",
  "doc_id": "BE004",
  "title": "",
  "description": "",
  "layer": "backend",
  "category": "backend",
  "keywords": [],
  "doc_version": "",
  "status": "active"
}
---

# BE004 MAPPING NIM


This is the first backend in the multi‑backend compiler, and Nim is the easiest target because:

• Nim supports macros
• Nim supports templates
• Nim supports compile‑time execution
• Nim supports both functional and imperative styles
• Nim’s syntax is indentation‑based like NXD
• Nim’s type system is flexible enough to host NXD’s semantics


Below is the full mapping system, structured for clarity and compiler implementation.


NXD → Nim Mapping Rules

These rules operate after IR lowering, meaning you map IR → Nim, not AST → Nim.


1️ Module Mapping

IRModule → Nim module

MODULE MATH


becomes:

# math.nim


Rules:

• NXD module name → Nim filename (lowercase)
• NXD EXPORT → Nim export pragma
• NXD IMPORT → Nim import


Example:

NXD:

IMPORT UTIL AS U


Nim:

import util as u


2️ Type Mapping

NXD types map cleanly into Nim types.

STRUCT → object

NXD:

TYPE PERSON { NAME: string, AGE: int }


Nim:

type
  Person = object
    name: string
    age: int


ENUM → enum

NXD:

TYPE COLOR ENUM { RED, GREEN, BLUE }


Nim:

type
  Color = enum
    RED, GREEN, BLUE


UNION → variant object or `distinct`

NXD:

TYPE RESULT UNION { OK(string), ERR(int) }


Nim:

type
  Result = object
    case kind: ResultKind
    of rkOk:
      ok: string
    of rkErr:
      err: int


TRAIT → Nim concept

NXD:

TRAIT SERIALIZABLE { FUNC TO_STRING(X): string }


Nim:

type
  Serializable = concept x
    toString(x) is string


IMPL → Nim proc implementation

NXD:

IMPL SERIALIZABLE FOR PERSON:
    FUNC TO_STRING(P): string:
        RETURN "person"


Nim:

proc toString(p: Person): string =
  "person"


3️ Function Mapping

NXD functions map directly into Nim procs.

NXD:

FUNC ADD(X, Y):
    RETURN X ADD Y


Nim:

proc add(x, y: int): int =
  return x + y


Rules:

• FUNC NAME(PARAMS) → proc name(params)
• NXD capitalized identifiers → Nim lowercase
• NXD operators → Nim operators (see operator mapping below)
• NXD blocks → Nim indentation blocks


4️ Statement Mapping

LET → var

NXD:

LET X SET 10


Nim:

var x = 10


CONST → let

NXD:

CONST MAX SET 100


Nim:

let max = 100


RETURN → return

NXD:

RETURN X


Nim:

return x


LOOP → while true

NXD:

LOOP:
    ...


Nim:

while true:
  ...


5️ Control Flow Mapping

IF / ELSE

NXD:

IF X GT 10:
    RETURN X
ELSE:
    RETURN 0


Nim:

if x > 10:
  return x
else:
  return 0


MATCH → case

NXD:

MATCH N:
    CASE 0:
        RETURN 1
    OTHERWISE:
        RETURN N MUL FACTORIAL(N SUB 1)


Nim:

case n:
  of 0:
    return 1
  else:
    return n * factorial(n - 1)


6️ Operator Mapping

NXD operators → Nim operators:

NXD	Nim	
ADD	+	
SUB	-	
MUL	*	
DIV	/	
MOD	mod	
EQ	==	
NEQ	!=	
GT	>	
LT	<	
GTE	>=	
LTE	<=	
AND	and	
OR	or	
NOT	not	


Ownership operators:

NXD	Nim	
MOVE	(no-op or =)	
CLONE	deepCopy()	
BORROW	(reference)	


Pipeline operators:

NXD:

X PIPE F


Nim:

F(x)


7️ Expression Mapping

Binary expressions

NXD:

X ADD Y


Nim:

x + y


Unary expressions

NXD:

NOT X


Nim:

not x


Function calls

NXD:

ADD(X, Y)


Nim:

add(x, y)


8️ Literal Mapping

NXD literals → Nim literals:

NXD	Nim	
10	10	
3.14	3.14	
“hello”	“hello”	
true	true	
false	false	
none	nil	


List:

[1, 2, 3] → @[1, 2, 3]


Map:

{ "a": 1 } → {"a": 1}


9️ Concurrency Mapping

NXD concurrency → Nim async/await or threads.

SPAWN

NXD:

SPAWN WORK()


Nim:

spawn work()


SEND / RECV

NXD:

SEND MSG TO CH
RECV X


Nim:

ch.send(msg)
x = ch.recv()


AWAIT

NXD:

AWAIT TASK


Nim:

await task


10 Error Handling Mapping

NXD:

TRY:
    ...
CATCH E:
    ...
FINALLY:
    ...


Nim:

try:
  ...
except Exception as e:
  ...
finally:
  ...


⭐ Visual Example: Full NXD → Nim Mapping

MODULE MATH

FUNC FACTORIAL(N):
    MATCH N:
        CASE 0:
            RETURN 1
        OTHERWISE:
            RETURN N MUL FACTORIAL(N SUB 1)


Becomes:

# math.nim

proc factorial(n: int): int =
  case n:
    of 0:
      return 1
    else:
      return n * factorial(n - 1)


Summary

NXD → Nim mapping is:

• clean
• predictable
• lossless
• semantically aligned
• easy to implement
• perfect for your multi‑backend compiler

