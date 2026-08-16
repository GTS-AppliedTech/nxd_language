---
{
  "@context": "https://nxdlang.org/schema",
  "doc_id": "CP003",
  "title": "",
  "description": "",
  "layer": "compiler",
  "category": "compiler",
  "keywords": [],
  "doc_version": "",
  "status": "active"
}
---


# CP003 NODE LIST & IR SPECIFICATIONS



NXD AST node list


Program and modules

• PROGRAM_NODE:• children: MODULE_DECL*, IMPORT_DECL*, TOP_LEVEL_DECL*

• MODULE_DECL_NODE:• fields: name: IDENTIFIER

• IMPORT_DECL_NODE:• fields: module_name: IDENTIFIER, alias: IDENTIFIER?


Types and declarations

• TYPE_DECL_NODE:• fields: name: IDENTIFIER, body: TYPE_BODY_NODE

• STRUCT_DECL_NODE:• fields: fields: FIELD_NODE*

• ENUM_DECL_NODE:• fields: variants: ENUM_VARIANT_NODE*

• UNION_DECL_NODE:• fields: options: TYPE_REF_NODE*

• TRAIT_DECL_NODE:• fields: name: IDENTIFIER, methods: FUNC_SIG_NODE*

• IMPL_DECL_NODE:• fields: trait: IDENTIFIER, type: IDENTIFIER, methods: FUNC_DECL_NODE*

• FIELD_NODE:• fields: name: IDENTIFIER, type: TYPE_REF_NODE

• ENUM_VARIANT_NODE:• fields: name: IDENTIFIER, payload_type: TYPE_REF_NODE?

• TYPE_REF_NODE:• fields: name: IDENTIFIER or primitive, generics: TYPE_REF_NODE*


Functions and parameters

• FUNC_DECL_NODE:• fields: name: IDENTIFIER, params: PARAM_NODE*, return_type: TYPE_REF_NODE?, body: BLOCK_NODE

• PARAM_NODE:• fields: name: IDENTIFIER, type: TYPE_REF_NODE?


Statements

• LET_STMT_NODE:• fields: name: IDENTIFIER, value: EXPR_NODE

• CONST_STMT_NODE:• fields: name: IDENTIFIER, value: EXPR_NODE

• RETURN_STMT_NODE:• fields: value: EXPR_NODE?

• LOOP_STMT_NODE:• fields: body: BLOCK_NODE

• IF_STMT_NODE:• fields: condition: EXPR_NODE, then_block: BLOCK_NODE, else_block: BLOCK_NODE?

• MATCH_STMT_NODE:• fields: target: EXPR_NODE, cases: CASE_BLOCK_NODE*

• CASE_BLOCK_NODE:• fields: pattern: PATTERN_NODE, body: BLOCK_NODE

• TRY_STMT_NODE:• fields: try_block: BLOCK_NODE, catches: CATCH_BLOCK_NODE*, finally_block: BLOCK_NODE?

• CATCH_BLOCK_NODE:• fields: error_name: IDENTIFIER, body: BLOCK_NODE

• EXPR_STMT_NODE:• fields: expr: EXPR_NODE


Expressions

• EXPR_NODE: (abstract)
• BINARY_EXPR_NODE:• fields: op: OPERATOR, left: EXPR_NODE, right: EXPR_NODE

• UNARY_EXPR_NODE:• fields: op: OPERATOR, operand: EXPR_NODE

• IDENT_EXPR_NODE:• fields: name: IDENTIFIER

• LITERAL_EXPR_NODE:• fields: value: LITERAL_NODE

• CALL_EXPR_NODE:• fields: callee: IDENTIFIER, args: EXPR_NODE*

• LIST_LITERAL_NODE:• fields: elements: EXPR_NODE*

• MAP_LITERAL_NODE:• fields: entries: MAP_ENTRY_NODE*

• MAP_ENTRY_NODE:• fields: key: EXPR_NODE, value: EXPR_NODE

• LAMBDA_EXPR_NODE:• fields: params: PARAM_NODE*, body: EXPR_NODE or BLOCK_NODE


Patterns

• PATTERN_NODE: (abstract)
• LITERAL_PATTERN_NODE:• fields: literal: LITERAL_NODE

• IDENT_PATTERN_NODE:• fields: name: IDENTIFIER

• STRUCT_PATTERN_NODE:• fields: type: IDENTIFIER, fields: PATTERN_FIELD_NODE*

• PATTERN_FIELD_NODE:• fields: name: IDENTIFIER, pattern: PATTERN_NODE

• LIST_PATTERN_NODE:• fields: elements: PATTERN_NODE*, rest: IDENTIFIER?


Concurrency

• SPAWN_STMT_NODE:• fields: call: CALL_EXPR_NODE

• SEND_STMT_NODE:• fields: message: EXPR_NODE, target: EXPR_NODE

• RECV_STMT_NODE:• fields: name: IDENTIFIER

• AWAIT_STMT_NODE:• fields: expr: EXPR_NODE


Blocks

• BLOCK_NODE:• fields: statements: STMT_NODE*


NXD IR specification

Think of IR as “NXD’s internal truth”—what everything lowers into before you transpile to Nim, Elixir, or D.

IR goals

• Language‑neutral: no NXD syntax, only semantics
• Target‑friendly: easy to map to Nim/Elixir/D
• Analysis‑friendly: good for optimization and audit
• Simple: fewer node types than AST


IR layers

1. Module and symbol layer

• IRModule:• name
• functions: IRFunction*
• types: IRType*
• imports: IRImport*

• IRImport:• module_name
• alias?

• IRType:• name
• kind: struct | enum | union | trait
• fields / variants / options


2. Function layer

• IRFunction:• name
• params: IRParam*
• return_type: IRTypeRef?
• body: IRBlock

• IRParam:• name
• type: IRTypeRef?

• IRTypeRef:• name
• generics: IRTypeRef*


3. Control flow layer

• IRBlock:• instructions: IRInstr*

• IRInstr: (abstract)


Key instruction types:

• IRLet: name, value: IRValue
• IRConst: name, value: IRValue
• IRAssign: target: IRValue, value: IRValue
• IRReturn: value: IRValue?
• IRIf: cond: IRValue, then_block: IRBlock, else_block: IRBlock?
• IRLoop: body: IRBlock
• IRMatch: target: IRValue, cases: IRMatchCase*
• IRMatchCase: pattern: IRPattern, body: IRBlock
• IRTry: try_block, catches, finally_block?


4. Value layer

• IRValue: (abstract)


Key value types:

• IRVar: name
• IRLiteral: kind, value
• IRBinaryOp: op, left: IRValue, right: IRValue
• IRUnaryOp: op, operand: IRValue
• IRCall: func_name, args: IRValue*
• IRList: elements: IRValue*
• IRMap: entries: (IRValue, IRValue)*
• IRLambda: params: IRParam*, body: IRBlock or IRValue


5. Concurrency IR

• IRSpawn: call: IRCall
• IRSend: message: IRValue, target: IRValue
• IRRecv: name: IRVar
• IRAwait: expr: IRValue


These map directly to Elixir processes or Nim/D async primitives.


6. Pattern IR

• IRPatternLiteral: literal: IRLiteral
• IRPatternVar: name
• IRPatternStruct: type_name, fields: (name, IRPattern)*
• IRPatternList: elements: IRPattern*, rest?


How this IR helps you

• Gives you one internal representation for all backends
• Lets you write optimizations once
• Lets your audit and pen‑test agents reason at IR level
• Makes transpiling to Nim/Elixir/D a matter of mapping IR → target constructs

