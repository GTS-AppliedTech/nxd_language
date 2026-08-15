---
{
  "@context": "https://nxdlang.org/schema",
  "doc_id": "LG003",
  "title": "",
  "description": "",
  "layer": "language guide",
  "category": "language guide",
  "keywords": [],
  "doc_version": "",
  "status": "active"
}
---

# LG003 GRAMMER STRUCTURE

1. Program Structure

A program is composed of:

• module declarations
• import statements
• type declarations
• function declarations
• constant declarations
• executable statements


Grammar

PROGRAM ::= MODULE_DECL* IMPORT_DECL* TOP_LEVEL_DECL* STATEMENT*


NXD allows both declarative and executable top‑level code (like Nim and Elixir).


2. Module Declaration

MODULE_DECL ::= "MODULE" IDENTIFIER


Modules define namespaces and compilation units.

Example:

MODULE MATH


3. Import Declaration

IMPORT_DECL ::= "IMPORT" IDENTIFIER ( "AS" IDENTIFIER )?


Example:

IMPORT UTIL AS U


4. Type Declaration

NXD supports:

• STRUCT
• ENUM
• UNION
• TRAIT
• IMPL


Grammar

TYPE_DECL ::= "TYPE" IDENTIFIER TYPE_BODY
TYPE_BODY ::= STRUCT_DECL | ENUM_DECL | UNION_DECL | TRAIT_DECL


Example:

TYPE PERSON { NAME: string, AGE: int }


5. Function Declaration

FUNC_DECL ::= "FUNC" IDENTIFIER "(" PARAM_LIST? ")" BLOCK
PARAM_LIST ::= IDENTIFIER ( "," IDENTIFIER )*


Example:

FUNC ADD(X, Y):
    RETURN X ADD Y


6. Statements

NXD supports:

• LET / CONST
• RETURN
• LOOP
• IF / ELSE
• MATCH / CASE
• SPAWN / SEND / RECV
• TRY / CATCH / FINALLY


Grammar

STATEMENT ::= LET_STMT
            | CONST_STMT
            | RETURN_STMT
            | LOOP_STMT
            | IF_STMT
            | MATCH_STMT
            | EXPR_STMT


7. Expressions

Expressions are the core of NXD.

Grammar

EXPR ::= LOGIC_EXPR
LOGIC_EXPR ::= COMP_EXPR ( ( "AND" | "OR" ) COMP_EXPR )*
COMP_EXPR ::= ADD_EXPR ( ( "EQ" | "NEQ" | "GT" | "LT" | "GTE" | "LTE" ) ADD_EXPR )*
ADD_EXPR ::= MUL_EXPR ( ( "ADD" | "SUB" ) MUL_EXPR )*
MUL_EXPR ::= UNARY_EXPR ( ( "MUL" | "DIV" | "MOD" ) UNARY_EXPR )*
UNARY_EXPR ::= ( "NOT" | "MOVE" | "CLONE" | "BORROW" )? PRIMARY
PRIMARY ::= IDENTIFIER
          | LITERAL
          | CALL_EXPR
          | LIST_LITERAL
          | MAP_LITERAL
          | LAMBDA_LITERAL


This is clean, predictable, and easy to transpile.


8. Function Calls

CALL_EXPR ::= IDENTIFIER "(" ARG_LIST? ")"
ARG_LIST ::= EXPR ( "," EXPR )*


Example:

ADD(X, Y)


9. Pattern Matching

MATCH_STMT ::= "MATCH" EXPR ":" CASE_BLOCK+
CASE_BLOCK ::= "CASE" PATTERN ":" BLOCK
PATTERN ::= LITERAL | IDENTIFIER | STRUCT_PATTERN | LIST_PATTERN


Example:

MATCH N:
    CASE 0:
        RETURN 1
    CASE _:
        RETURN N MUL FACTORIAL(N SUB 1)


10. Concurrency Grammar

SPAWN_STMT ::= "SPAWN" CALL_EXPR
SEND_STMT ::= "SEND" EXPR "TO" EXPR
RECV_STMT ::= "RECV" IDENTIFIER
AWAIT_STMT ::= "AWAIT" EXPR


This maps directly into Elixir’s BEAM semantics.


11. Error Handling Grammar

TRY_STMT ::= "TRY" BLOCK CATCH_BLOCK* FINALLY_BLOCK?
CATCH_BLOCK ::= "CATCH" IDENTIFIER ":" BLOCK
FINALLY_BLOCK ::= "FINALLY" ":" BLOCK


12. Block Structure

NXD uses colon + indentation (like Nim and Python).

BLOCK ::= ":" NEWLINE INDENT STATEMENT* DEDENT


Example:

IF X GT 10:
    RETURN X


Visual Example of NXD Grammar in Action
MODULE MATH

FUNC FACTORIAL(N):
    MATCH N:
        CASE 0:
            RETURN 1
        OTHERWISE:
            RETURN N MUL FACTORIAL(N SUB 1)

