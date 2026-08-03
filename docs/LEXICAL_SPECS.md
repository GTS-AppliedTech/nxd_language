LEXICAL_SPECS.md

Below is the complete, structured NXD identifier specification — clean, compiler‑friendly, AI‑friendly, and fully aligned with your capitalization rule.


NXD Identifier Rules (Capitalized)

1. Identifier Character Set

Identifiers may contain:

• A–Z (uppercase letters only)
• 0–9 (digits)
• _ (underscore)


This gives you predictable, easy‑to‑tokenize identifiers.

Examples:

X
MY_VAR
PERSON_AGE
MAP2D


2. Identifiers MUST begin with a letter

Identifiers cannot begin with digits or underscores.

Valid:

X1
DATA_3
MAP2D


Invalid:

1X
_AGE
__TEMP


This prevents ambiguity with numeric literals and operators.


3. Identifiers are ALWAYS capitalized

This is your signature rule.

Identifiers must be:

• fully capitalized
• no lowercase letters allowed
• no mixed case


Valid:

USER
TOTAL_SUM
FACTORIAL


Invalid:

user
Total_Sum
Factorial


This gives NXD a unique visual identity and makes parsing trivial.


4. Identifier Categories

Identifiers fall into these categories:

• Variable identifiers
• Function identifiers
• Type identifiers
• Module identifiers
• Constant identifiers
• Trait identifiers
• Generic identifiers


All follow the same capitalization rule.


5. Identifier Naming Conventions

To keep NXD readable and consistent:

• Use _ to separate words
• Avoid long identifiers
• Prefer descriptive names
• Use ALL_CAPS for everything


Examples:

TOTAL_COUNT
USER_NAME
PROCESS_DATA


This is similar to SQL’s uppercase style but applied to a programming language.


6. Mutability Rules

NXD uses keyword‑based mutability, not identifier‑based mutability.

• LET → mutable
• CONST → immutable


Identifiers themselves do not encode mutability.

Example:

LET X SET 10
CONST MAX_VALUE SET 100


7. Scope Rules

Identifiers follow lexical scoping:

• inner scopes shadow outer scopes
• module identifiers define namespaces
• type identifiers define type scopes
• function identifiers define local scopes


Example:

FUNC TEST():
    LET X SET 10
    LOOP:
        LET X SET 20   # shadows outer X


8. Reserved Identifiers

Identifiers that match keywords are not allowed.

Invalid:

LET LET SET 10
FUNC RETURN SET 5


This prevents ambiguity.


Visual Example of NXD Identifier Rules

MODULE MATH

FUNC FACTORIAL(N):
    IF N EQ 0:
        RETURN 1
    ELSE:
        RETURN N MUL FACTORIAL(N SUB 1)


Identifiers: MODULE, MATH, FUNC, FACTORIAL, N, RETURN
All capitalized.
Literals (0, 1) remain lowercase.


Why these identifier rules are perfect for NXD

• Easy for agents to parse
• Easy for compilers to tokenize
• Zero ambiguity with literals
• Strong visual identity
• Consistent across Nim, Elixir, and D mappings
• Perfect for documentation generation
• Perfect for AST and IR design


NXD Keyword List (Capitalized)


Core Declarations

• LET — variable binding
• CONST — immutable binding
• FUNC — function definition
• TYPE — type definition
• MODULE — module definition
• IMPORT — import external modules
• EXPORT — expose module items
• ALIAS — type or module alias


Control Flow

• IF
• ELSE
• MATCH — pattern matching
• CASE — match branch
• LOOP — general loop
• BREAK
• CONTINUE
• RETURN


Pattern Matching

• MATCH
• CASE
• WHEN — conditional pattern
• OTHERWISE — default case


This aligns well with Elixir’s pattern‑matching semantics and Nim’s case expressions.


Type System

• TYPE
• ENUM
• STRUCT
• UNION
• GENERIC
• TRAIT — interface/behavior
• IMPL — trait implementation
• AS — type cast
• IS — type check


These map cleanly into D’s templates, Nim’s generics, and Elixir’s protocols.


Concurrency

• SPAWN — create a process/task
• AWAIT — async wait
• ASYNC — async block
• SEND — message passing
• RECV — receive message
• SYNC — synchronization primitive


These map directly into Elixir’s BEAM concurrency model.


Error Handling

• TRY
• CATCH
• THROW
• RAISE
• FINALLY


These map cleanly into Nim’s exceptions and D’s error model.


Memory & Resource Rules

• NEW — allocate
• FREE — deallocate
• MOVE — ownership transfer
• CLONE — deep copy
• BORROW — temporary reference


These give NXD a hybrid memory model that can map into D and Nim.


Compilation & Meta

• MACRO — compile‑time macro
• INLINE — inline hint
• COMPILE — compile‑time block
• EXTERN — foreign function interface
• TARGET — specify Nim/Elixir/D output


These are essential for a transpiling language.


• ALL operators are CAPITALIZED
• ALL literals and everything else remain lowercase
• Operators are grouped by purpose


This gives NXD a clean, readable, AI‑friendly, and compiler‑friendly foundation.


NXD Operator List (Capitalized)

Arithmetic Operators

• ADD — addition
• SUB — subtraction
• MUL — multiplication
• DIV — division
• MOD — modulo


These map cleanly into Nim (+ - * / mod), Elixir (+ - * / rem), and D (+ - * / %).


Assignment Operators

• SET — basic assignment
• SETADD — +=
• SETSUB — -=
• SETMUL — *=
• SETDIV — /=


Capitalizing assignment operators gives NXD a unique visual identity.


Comparison Operators

• EQ — equal
• NEQ — not equal
• GT — greater than
• LT — less than
• GTE — greater or equal
• LTE — less or equal


These map directly into Nim, Elixir, and D comparison semantics.


Logical Operators

• AND
• OR
• NOT


These are readable, capitalized, and easy for agents to parse.


Pattern Operators

• MATCHES — pattern match
• IN — membership
• HAS — structural presence
• PIPECASE — pattern pipeline


These map into Elixir’s pattern matching and Nim’s case expressions.


Pipeline Operators

• PIPE — forward pipeline
• PIPEMAP — pipeline map
• PIPEFILTER — pipeline filter


These give NXD functional expressiveness similar to Elixir’s |> but with capitalized clarity.


Concurrency Operators

• SEND — send message
• RECV — receive message
• AWAIT — wait for async result
• SPAWN — spawn process/task


These map directly into BEAM semantics.


Memory & Ownership Operators

• NEW — allocate
• FREE — deallocate
• MOVE — transfer ownership
• CLONE — deep copy
• BORROW — temporary reference


These give NXD a hybrid memory model that can map into D and Nim.


Type Operators

• AS — type cast
• IS — type check
• OF — type membership
• GEN — generic instantiation


These map into Nim generics, D templates, and Elixir protocols.


Meta & Compile-Time Operators

• MACRO — compile‑time macro
• INLINE — inline hint
• COMPILE — compile‑time block
• EXTERN — foreign function interface
• TARGET — specify Nim/Elixir/D output


These are essential for a transpiling language.


Visual Example of NXD Operators in Action

FUNC PROCESS_DATA(X):
    LET CLEANED SET PIPEMAP(X, TRIM)
    LET FILTERED SET PIPEFILTER(CLEANED, IS_VALID)
    RETURN FILTERED


Capitalized operators make the flow extremely readable.


Why this operator list works

• Capitalization rule is preserved
• Easy for agents to parse and generate
• Clear mapping into Nim, Elixir, and D
• Supports functional, procedural, and systems paradigms
• Gives NXD a unique visual identity
• Compiler‑friendly and IR‑friendly
• Perfect for multi‑agent reasoning


This operator list is strong enough to support NXD as a standalone language and as a transpiler.


NXD Literal Rules (All literals are lowercase)

NXD literals are always lowercase, which creates a clean visual contrast against your capitalized identifiers, keywords, and operators.

This rule makes NXD extremely easy to scan and parse — both for humans and agents.


Numeric Literals

Numeric literals are always lowercase and may include:

• integers
• floats
• scientific notation
• binary
• hex


Valid examples

0
1
42
3.14
1e10
0b1010
0xff


Rules

• No capitalization allowed
• No underscores inside numbers
• Negative numbers use the operator: SUB 5 (not -5)
• No implicit type suffixes (e.g., 42i32)


This keeps NXD consistent and easy to tokenize.


String Literals

Strings are lowercase unless they contain user data.

Valid examples

"hello world"
"nxdlanguage"
"example"


Rules

• Strings are enclosed in double quotes
• Escape sequences allowed: \n, \t, \", \\
• Unicode allowed
• Case inside strings is not enforced (strings represent raw data)


Boolean Literals

Boolean values are lowercase:

true
false


This aligns with Nim, Elixir, and D.


Null / None Literal

NXD uses:

none


Instead of null, nil, or None.

This gives NXD a unique identity while remaining easy to map.


Collection Literals

Collections are lowercase and use punctuation, not keywords.

Lists

[1, 2, 3]
["a", "b", "c"]


Maps

{ "a": 1, "b": 2 }


Rules

• Collections are lowercase
• Keys inside maps may be lowercase or strings
• Identifiers inside collections remain capitalized


Example:

[USER_ID, 10, "name"]


Function Literal / Lambda

NXD supports lowercase lambda literals:

fn(x) => x mul 2


Rules:

• fn is lowercase
• parameters follow identifier rules (capitalized)
• body uses capitalized operators


Example:

LET DOUBLE SET fn(X) => X MUL 2


Type Literal

NXD uses lowercase type literals for primitive types:

int
float
string
bool
none


But user-defined types follow identifier rules:

TYPE PERSON { NAME: string, AGE: int }


This creates a clean separation between:

• primitive types (lowercase)
• user types (capitalized)


🧠 Visual Example of NXD Literal Rules
MODULE MATH

FUNC FACTORIAL(N):
    IF N EQ 0:
        RETURN 1
    ELSE:
        RETURN N MUL FACTORIAL(N SUB 1)


Literals: 0, 1
Identifiers: MODULE, MATH, FUNC, FACTORIAL, N
Operators: EQ, MUL, SUB
Keywords: IF, RETURN, ELSE

Everything is visually distinct.


Why these literal rules are perfect for NXD

• Clear visual separation between literals and capitalized constructs
• Easy for agents to parse and generate
• Easy for compilers to tokenize
• Consistent across Nim, Elixir, and D mappings
• Supports functional, procedural, and systems paradigms
• Perfect for documentation and teaching agents
• Future‑proof for IR and transpiler design


This completes the lexical foundation of NXD.

