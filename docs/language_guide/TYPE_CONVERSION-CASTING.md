---
{
  "@context": "https://nxdlang.org/schema",
  "doc_id": "LG004",
  "title": "",
  "description": "",
  "layer": "language guide",
  "category": "language guide",
  "keywords": [],
  "doc_version": "",
  "status": "active"
}
---


# LG004 Type Conversion & Casting Semantics

(Implicit conversions, explicit casts, numeric rules, failure behavior, AS/IS semantics)

NXD defines a strict, explicit, and predictable conversion model.
The goal is to ensure:

• semantic consistency across Nim, Elixir, and D
• no silent data loss
• no implicit widening or narrowing
• clear runtime failure behavior
• auditability for security agents


This chapter defines all conversion rules.


1. Core principles

1. No implicit conversions

NXD never performs implicit conversions between distinct types.

• int -> float is not implicit
• float -> int is not implicit
• int -> string is not implicit
• string -> int is not implicit
• enum -> string is not implicit
• none -> option is not implicit


All conversions must be explicit via [AS](ca://s?q=Explain_NXD_AS_cast).

2. Conversions may fail

Explicit casts may:

• succeed
• fail with a runtime error
• produce a RESULT ERR if used inside a safe conversion API


3. Conversions are semantic, not reinterpretation

NXD does not allow reinterpretation casts (bit-level reinterpretation).
All conversions are value-level transformations.


2. The `AS` operator (explicit cast)

Syntax

LET Y SET X AS float
LET N SET S AS int
LET T SET V AS string


Semantics

AS performs an explicit, potentially fallible conversion.

Failure behavior

If the conversion cannot be performed:

• inside normal code → runtime exception
• inside TRY/CATCH → caught exception
• inside conversion APIs → ERR("conversion failed")


Examples

int → float

Always succeeds.

10 AS float   # 10.0


float → int

Potentially lossy.

10.7 AS int   # truncation or error (defined below)


NXD must define one rule:

• truncate
• round
• error on fractional part


Recommended: error on fractional part for safety.

string → int

Fails if string is not numeric.

"42" AS int   # ok
"hello" AS int   # runtime error


int → string

Always succeeds.

enum → string

Always succeeds (returns the enum tag name).

none → option

Always succeeds:

none AS option<int>   # NONE


3. The `IS` operator (type check)

Syntax

IF X IS int:
IF V IS SERIALIZABLE:


Semantics

IS performs a type check only. It does not convert.

Behavior

• returns true if the value conforms to the type
• returns false otherwise
• never throws
• never converts


4. Allowed conversions table

From	To	Allowed	Notes	
int	float	yes	explicit only	
float	int	yes	explicit, may fail	
int	string	yes	explicit	
string	int	yes	explicit, may fail	
enum	string	yes	explicit	
string	enum	no	must use parser API	
none	option	yes	explicit	
option	T	no	must unwrap	
result	T	no	must unwrap	
any	any	yes	identity cast	


5. Numeric conversion rules

int → float

Always succeeds.

float → int

NXD must define one of three policies:

1. truncate
2. round
3. error if fractional part exists ← recommended


Recommended rule:

float → int fails if the value has a fractional component.

This prevents silent data loss.


6. String conversion rules

string → int

Allowed only if:

• string contains only digits
• optional leading + or -
• no whitespace unless explicitly allowed


Otherwise → runtime error.

int → string

Always succeeds.

enum → string

Always succeeds.

string → enum

Not allowed via AS.
Must use:

PARSE_ENUM<T>(string)


7. Option and Result conversions

none → option<T>

Legal:

none AS option<int>   # NONE


option<T> → T

Illegal:

LET X SET O AS int   # illegal


Must use:

UNWRAP_OR(O, DEFAULT)


result<T> → T

Illegal:

LET X SET R AS int   # illegal


Must use:

MATCH R:
    CASE OK(V): ...
    CASE ERR(E): ...


8. Trait-based conversions

Traits may define conversion functions:

TRAIT SERIALIZABLE {
    FUNC TO_STRING(X): string
}


If a type implements SERIALIZABLE, then:

X AS string


is legal only if the trait defines a conversion function.

Trait-based conversions are explicit, not implicit.


9. Backend mapping

Nim

• AS → explicit conversion functions (float(x), int(x), $x)
• IS → of or is checks
• failures → exceptions or Result depending on lowering


Elixir

• AS → conversion functions (String.to_integer, Float.to_string)
• IS → pattern matching or guards
• failures → exceptions


D

• AS → cast + conversion functions
• IS → is or template constraints
• failures → exceptions


10. Required new document

Yes—you must have a dedicated:

NXD Type Conversion & Casting Specification

This chapter is exactly that.
