LEXICAL_SPECS.md

1. Lexical Overview

This document defines NXD lexical syntax for identifiers, keywords, operators, and literals.
It preserves existing normative definitions without introducing new syntax, keywords, operators, literals, or naming rules.

2. Capitalization Rules

NXD lexical constructs follow a capitalization rule:

• Identifiers are uppercase only.
• Keywords are uppercase only.
• Operators are uppercase only.
• Literals are lowercase only.

Identifiers may not contain lowercase letters or mixed case.
Numeric, string, boolean, collection, function literal, and type literal values are lowercase.

3. Identifiers

3.1 Naming Rules

Identifiers may contain:

• uppercase letters A–Z
• digits 0–9
• underscore _

Identifiers must begin with an uppercase letter.
Identifiers cannot begin with digits or underscores.

Identifiers are always fully capitalized.

Valid examples:

X
MY_VAR
PERSON_AGE
MAP2D

Invalid examples:

1X
_AGE
__TEMP
user
Total_Sum
Factorial

3.2 Reserved Names

Identifiers that match keywords are not allowed.

3.3 Scope of Identifiers

Identifiers follow lexical scoping.

• Inner scopes shadow outer scopes.
• Module identifiers define namespaces.
• Type identifiers define type scopes.
• Function identifiers define local scopes.

Example:

FUNC TEST():
    LET X SET 10
    LOOP:
        LET X SET 20

3.4 Identifier Categories

Identifiers may represent:

• variable identifiers
• function identifiers
• type identifiers
• module identifiers
• constant identifiers
• trait identifiers
• generic identifiers

3.5 Mutability and Identifiers

Identifier names do not encode mutability.
NXD uses keywords for mutability:

• LET for mutable binding
• CONST for immutable binding

Example:

LET X SET 10
CONST MAX_VALUE SET 100

4. Keywords

NXD keywords are uppercase reserved words used for declarations, control flow, pattern matching, type system constructs, concurrency, error handling, memory/resource rules, and compile-time features.

4.1 Keywords List

Core Declarations:

• LET
• CONST
• FUNC
• TYPE
• MODULE
• IMPORT
• EXPORT
• ALIAS

Control Flow:

• IF
• ELSE
• MATCH
• CASE
• LOOP
• BREAK
• CONTINUE
• RETURN

Pattern Matching:

• MATCH
• CASE
• WHEN
• OTHERWISE

Type System:

• TYPE
• ENUM
• STRUCT
• UNION
• GENERIC
• TRAIT
• IMPL
• AS
• IS

Concurrency:

• SPAWN
• AWAIT
• ASYNC
• SEND
• RECV
• SYNC

Error Handling:

• TRY
• CATCH
• THROW
• RAISE
• FINALLY

Memory & Resource:

• NEW
• FREE
• MOVE
• CLONE
• BORROW

Compilation & Meta:

• MACRO
• INLINE
• COMPILE
• EXTERN
• TARGET

4.2 Keyword Categories

Keywords are organized by their role in the language.

• Core declarations define binding and program structure.
• Control flow keywords manage execution paths.
• Pattern matching keywords define match constructs.
• Type system keywords define types and type relationships.
• Concurrency keywords define async and message passing constructs.
• Error handling keywords define exception and catch semantics.
• Memory/resource keywords define ownership and allocation operations.
• Compilation/meta keywords define compile-time behavior and targets.

5. Operators

5.1 Arithmetic Operators

• ADD — addition
• SUB — subtraction
• MUL — multiplication
• DIV — division
• MOD — modulo

5.2 Comparison Operators

• EQ — equal
• NEQ — not equal
• GT — greater than
• LT — less than
• GTE — greater or equal
• LTE — less or equal

5.3 Logical Operators

• AND
• OR
• NOT

5.4 Assignment Operators

• SET — basic assignment
• SETADD — +=
• SETSUB — -=
• SETMUL — *=
• SETDIV — /=

5.5 Special Operators

Pattern operators:

• MATCHES — pattern match
• IN — membership
• HAS — structural presence
• PIPECASE — pattern pipeline

Pipeline operators:

• PIPE — forward pipeline
• PIPEMAP — pipeline map
• PIPEFILTER — pipeline filter

Concurrency operators:

• SEND — send message
• RECV — receive message
• AWAIT — wait for async result
• SPAWN — spawn process/task

Memory & ownership operators:

• NEW — allocate
• FREE — deallocate
• MOVE — transfer ownership
• CLONE — deep copy
• BORROW — temporary reference

Type operators:

• AS — type cast
• IS — type check
• OF — type membership
• GEN — generic instantiation

Meta & compile-time operators:

• MACRO — compile-time macro
• INLINE — inline hint
• COMPILE — compile-time block
• EXTERN — foreign function interface
• TARGET — specify Nim/Elixir/D output

6. Literals

6.1 Integer Literals

Numeric literals are lowercase.
Integer literals include decimal, binary, and hexadecimal forms.

Valid examples:

0
1
42
0b1010
0xff

6.2 Floating-Point Literals

Floating-point literals include decimal fractions and scientific notation.

Valid examples:

3.14
1e10

6.3 Numeric Literal Rules

• No capitalization is allowed in numeric literals.
• No underscores are allowed inside numeric literals.
• Negative numbers are formed with the SUB operator: SUB 5.
• No implicit type suffixes are allowed.

6.4 String Literals

• Strings are enclosed in double quotes.
• Escape sequences are allowed: \n, \t, \", \\
• Unicode is allowed.
• Case inside strings is not enforced.

Valid examples:

"hello world"
"nxdlanguage"
"example"

6.5 Boolean Literals

Boolean values are lowercase:

true
false

6.6 Null/None Literal

NXD uses the lowercase literal:

none

6.7 Collection Literals

Lists are written with square brackets:

[1, 2, 3]
["a", "b", "c"]

Maps are written with braces and key/value punctuation:

{"a": 1, "b": 2}

Collection rules:

• Collections are lowercase punctuation constructs.
• Keys inside maps may be lowercase or strings.
• Identifiers inside collections remain capitalized.

6.8 Function Literal

NXD supports lowercase lambda literals:

fn(X) => X MUL 2

Function literal rules:

• fn is lowercase.
• Parameters follow identifier rules.
• The body uses capitalized operators.

Valid example:

LET DOUBLE SET fn(X) => X MUL 2

6.9 Type Literal

Primitive type literals are lowercase:

int
float
string
bool
none

User-defined types follow identifier rules:

TYPE PERSON { NAME: string, AGE: int }

7. Reserved Words

The following words are reserved and may not be used as identifiers:

LET, CONST, FUNC, TYPE, MODULE, IMPORT, EXPORT, ALIAS,
IF, ELSE, MATCH, CASE, LOOP, BREAK, CONTINUE, RETURN,
WHEN, OTHERWISE,
ENUM, STRUCT, UNION, GENERIC, TRAIT, IMPL, AS, IS,
SPAWN, AWAIT, ASYNC, SEND, RECV, SYNC,
TRY, CATCH, THROW, RAISE, FINALLY,
NEW, FREE, MOVE, CLONE, BORROW,
MACRO, INLINE, COMPILE, EXTERN, TARGET,
fn, true, false, none

8. Examples

Identifier examples:

Valid:
X
MY_VAR
PERSON_AGE
MAP2D

Invalid:
1X
_AGE
__TEMP
user
Total_Sum

Identifier example with lexical scope:

FUNC TEST():
    LET X SET 10
    LOOP:
        LET X SET 20

Operator examples:

FUNC PROCESS_DATA(X):
    LET CLEANED SET PIPEMAP(X, TRIM)
    LET FILTERED SET PIPEFILTER(CLEANED, IS_VALID)
    RETURN FILTERED

Function literal example:

LET DOUBLE SET fn(X) => X MUL 2

Type literal example:

TYPE PERSON { NAME: string, AGE: int }

Literal examples:

0
1
42
3.14
1e10
0b1010
0xff
"hello world"
true
false
none
[1, 2, 3]
{"a": 1, "b": 2}

9. Potential Specification Conflicts

No conflicting definitions were detected within this document.
