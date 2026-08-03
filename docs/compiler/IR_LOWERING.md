IR_LOWERING.md

NXD IR Lowering Rules

Lowering converts AST → IR by removing syntactic sugar, enforcing semantics, and producing a normalized representation.

NXD’s IR is intentionally simpler than the AST, so lowering is mostly:

• flattening nested constructs
• normalizing expressions
• converting blocks into instruction lists
• resolving identifiers
• resolving patterns
• normalizing concurrency primitives
• normalizing type references
• removing syntactic sugar (like MATCH)
• enforcing evaluation order


1️ Program-Level Lowering

AST → IRModule

• PROGRAM_NODE lowers into a list of IRModule objects.
• Each MODULE_DECL_NODE becomes an IRModule.
• Imports become IRImport.
• Top-level declarations lower into IR functions or IR types.


Rule

PROGRAM_NODE.modules → IRModule*
PROGRAM_NODE.imports → IRImport*
PROGRAM_NODE.top_level → IRFunction* or IRType*


2️ Type Lowering

STRUCT, ENUM, UNION, TRAIT, IMPL

Each type declaration lowers into an IRType.

Rule

TYPE_DECL_NODE → IRType(name, kind, fields/variants/options)


Examples:

• STRUCT_DECL_NODE.fields → IRField*
• ENUM_DECL_NODE.variants → IREnumVariant*
• UNION_DECL_NODE.options → IRTypeRef*
• TRAIT_DECL_NODE.methods → IRFunctionSignature*
• IMPL_DECL_NODE.methods → IRFunction*


3️ Function Lowering

AST → IRFunction

Lowering a function:

FUNC_DECL_NODE → IRFunction


Rules:

• Parameters lower into IRParam.
• Body lowers into IRBlock.
• Return type inferred or explicit.


4️ Block Lowering

BLOCK_NODE → IRBlock

A block becomes a flat list of IR instructions.

BLOCK_NODE.statements → IRInstr*


Indentation disappears; only structure remains.


5️ Statement Lowering

Each statement lowers into one or more IR instructions.


LET / CONST

LET_STMT_NODE → IRLet(name, value)
CONST_STMT_NODE → IRConst(name, value)


RETURN

RETURN_STMT_NODE → IRReturn(value?)


LOOP

LOOP_STMT_NODE → IRLoop(body)


IF / ELSE

IF_STMT_NODE → IRIf(cond, then_block, else_block?)


Lowering rule:

• Lower condition into IRValue
• Lower both blocks into IRBlock


MATCH

MATCH is syntactic sugar. It lowers into IRMatch.

MATCH_STMT_NODE → IRMatch(target, cases)
CASE_BLOCK_NODE → IRMatchCase(pattern, body)


Pattern lowering rules:

• Literal → IRPatternLiteral
• Identifier → IRPatternVar
• Struct pattern → IRPatternStruct
• List pattern → IRPatternList


TRY / CATCH / FINALLY

TRY_STMT_NODE → IRTry(try_block, catches, finally_block?)
CATCH_BLOCK_NODE → IRCatch(error_name, body)


Concurrency

SPAWN

SPAWN_STMT_NODE → IRSpawn(call)


SEND

SEND_STMT_NODE → IRSend(message, target)


RECV

RECV_STMT_NODE → IRRecv(name)


AWAIT

AWAIT_STMT_NODE → IRAwait(expr)


6️ Expression Lowering

Expressions lower into IRValue nodes.


Identifiers

IDENT_EXPR_NODE → IRVar(name)


Literals

LITERAL_EXPR_NODE → IRLiteral(kind, value)


Unary Operators

UNARY_EXPR_NODE → IRUnaryOp(op, operand)


Binary Operators

BINARY_EXPR_NODE → IRBinaryOp(op, left, right)


Lowering rule:

• Lower left and right recursively
• Normalize operator names (ADD → add, EQ → eq, etc.)


Function Calls

CALL_EXPR_NODE → IRCall(func_name, args)


Arguments lower recursively.


Lists

LIST_LITERAL_NODE → IRList(elements)


Maps

MAP_LITERAL_NODE → IRMap(entries)


Lambdas

LAMBDA_EXPR_NODE → IRLambda(params, body)


7 Pattern Lowering

Patterns lower into IRPattern nodes.

Literal pattern

LITERAL_PATTERN_NODE → IRPatternLiteral(literal)


Identifier pattern

IDENT_PATTERN_NODE → IRPatternVar(name)


Struct pattern

STRUCT_PATTERN_NODE → IRPatternStruct(type_name, fields)


List pattern

LIST_PATTERN_NODE → IRPatternList(elements, rest?)


8 Type Reference Lowering

TYPE_REF_NODE → IRTypeRef(name, generics)


Generics lower recursively.


Visual Example: AST → IR Lowering

FUNC FACTORIAL(N):
    MATCH N:
        CASE 0:
            RETURN 1
        OTHERWISE:
            RETURN N MUL FACTORIAL(N SUB 1)


Lowers into:

IRFunction FACTORIAL:
  params: [N]
  body:
    IRMatch(
      target = IRVar("N"),
      cases = [
        IRMatchCase(
          pattern = IRPatternLiteral(0),
          body = IRBlock([ IRReturn(IRLiteral(1)) ])
        ),
        IRMatchCase(
          pattern = IRPatternVar("_"),
          body = IRBlock([
            IRReturn(
              IRBinaryOp(
                op="mul",
                left=IRVar("N"),
                right=IRCall("FACTORIAL", [
                  IRBinaryOp(op="sub", left=IRVar("N"), right=IRLiteral(1))
                ])
              )
            )
          ])
        )
      ]
    )


This is clean, normalized, and ready for Nim/Elixir/D transpilation.


Why these lowering rules are perfect

• They remove all syntactic sugar
• They normalize operators
• They flatten blocks
• They unify control flow
• They make MATCH universal
• They make concurrency explicit
• They make type references uniform
• They make NXD easy to transpile
• They make NXD easy to optimize
• They make NXD easy for agents to reason about

