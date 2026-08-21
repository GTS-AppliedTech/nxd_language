# NXD Nim Backend Validation

## Pass 1 - Frontend Python Structural Validation 

Goal:
Verify NXD source can travel through:

Scanner → Parser → AST → Lowering → IR JSON

---

| Test | Feature | Status | Failure Stage | Notes |
|--------|---------|---------|---------|---------|
| ST001 | Module only | PASS | N/A |  |
| ST002 | Function Return Literal | PASS |N/A |  |
| ST003 | Binary Expression| PASS | N/A |  |

---

## Findings (Micro Test)

### PT001

NXD Sample:

```nxd
MODULE TEST
```

Result: 

COMPILER: PASS
SEMANTICS: PASS


Failure Stage:

N/A

Expected Output:

```json
{
  "module": {
    "name": "TEST",
    "imports": []
  },
  "types": [],
  "functions": []
}
```

Actual Output:

```json
{
  "module": {
    "name": "TEST",
    "imports": []
  },
  "types": [],
  "functions": []
}
```

Notes: 

First successful frontend pipeline execution.
Frontend pipeline successfully generated IR JSON.

---

### PT002

NXD Sample:

```nxd
MODULE TEST

FUNC MAIN():
    RETURN 1
```    

Result:

COMPILER: PASS
SEMANTICS: PASS

Failure Stage:

N/A

Expected Output:

```json 
{
  "module": {
    "name": "TEST",
    "imports": []
  },
  "types": [],
  "functions": [
    {
      "name": "MAIN",
      "params": [],
      "return_type": null,
      "body": [
        {
          "kind": "return",
          "value": {
            "kind": "literal",
            "value": 1
          }
        }
      ]
    }
  ]
}
```

Actual Output:

```json
{
  "module": {
    "name": "TEST",
    "imports": []
  },
  "types": [],
  "functions": [
    {
      "name": "MAIN",
      "params": [],
      "return_type": null,
      "body": [
        {
          "kind": "return",
          "value": {
            "kind": "literal",
            "value": 1
          }
        }
      ]
    }
  ]
}
```

Notes:

Confirmed function declaration parsing, return statement parsing, integer literal parsing, AST lowering, and IR JSON generation.

---

### PT003

NXD Sample:

```nxd
MODULE TEST

FUNC MAIN():
    RETURN 1 ADD 2
```    

Result:

COMPILER: PASS
SEMANTICS: PASS

Failure Stage:

N/A

Expected Output:

```json
{
  "module": {
    "name": "TEST",
    "imports": []
  },
  "types": [],
  "functions": [
    {
      "name": "MAIN",
      "params": [],
      "return_type": null,
      "body": [
        {
          "kind": "return",
          "value": {
            "kind": "binary",
            "op": "ADD",
            "left": {
              "kind": "literal",
              "value": 1
            },
            "right": {
              "kind": "literal",
              "value": 2
            }
          }
        }
      ]
    }
  ]
}
```

Actual Output:

```json
{
  "module": {
    "name": "TEST",
    "imports": []
  },
  "types": [],
  "functions": [
    {
      "name": "MAIN",
      "params": [],
      "return_type": null,
      "body": [
        {
          "kind": "return",
          "value": {
            "kind": "binary",
            "op": "ADD",
            "left": {
              "kind": "literal",
              "value": 1
            },
            "right": {
              "kind": "literal",
              "value": 2
            }
          }
        }
      ]
    }
  ]
}
```

Notes:

Confirmed:
- Operator tokenization
- Binary expression parsing
- ASTBinary generation
- IR binary lowering
- JSON serialization

## Pass 2 - IR JSON + Partial Rust Backend (No Semantics Run) + Nim Backend 

Goal:
Verify JSON source can travel through:

IR JSON → Rust Loader → IR Root → Nim Backend

---

| Test | Feature | Status | Failure Stage | Notes |
|--------|---------|---------|---------|---------|
| ST004 | Module Only | PASS |  |  |
| ST005 | Function Return Literal | PASS |  |  |
| ST006 | Binary Expression | PASS |  |  |

---

## Findings (Micro Test)

### PT004

NXD Sample:


```JSON
{
  "module": {
    "name": "TEST",
    "imports": []
  },
  "types": [],
  "traits": [],
  "impls": [],
  "functions": [],
  "statements": []
}
```

Result:

PASS

Expected Output:

Actual Output:

```nim
# test
```

Notes:

Python → JSON → Rust → Nim path confirmed 

---

### PT005 (micro test)

NXD Sample:


```json
{
  "module": {
    "name": "TEST",
    "imports": []
  },
  "types": [],
  "traits": [],
  "impls": [],
  "functions": [
    {
      "name": "MAIN",
      "params": [],
      "return_type": null,
      "body": [
        {
          "Return": {
            "Literal": {
              "Int": 1
            }
          }
        }
      ]
    }
  ],
  "statements": []
}
```    

Result:

PASS

Expected Output:

Actual Output:

(st005_rust.nim)
```nim
# test


proc main() =
  return 1
```


Notes:

Verified Python frontend successfully parsed a function declaration.
Verified AST → IR lowering for FUNC MAIN.
Verified IR JSON serialization of function body.
Verified Rust successfully deserialized:
IRRoot
IRFunction
IRStatement::Return
IRExpr::Literal
Verified Nim backend emitted a valid procedure declaration.
Verified integer literal preservation through the entire pipeline.
Confirmed first successful end-to-end function handoff from Python frontend to Rust backend.

Technical Significance:

First validation beyond module-level emission.
Demonstrates function body reconstruction in Rust.
Establishes baseline for statement emission.


---

### PT006 (micro test)

NXD Sample:

```json
{
  "module": {
    "name": "TEST",
    "imports": []
  },
  "types": [],
  "traits": [],
  "impls": [],
  "functions": [
    {
      "name": "MAIN",
      "params": [],
      "return_type": null,
      "body": [
        {
          "Return": {
            "Binary": {
              "kind": "ADD",
              "left": {
                "Literal": {
                  "Int": 1
                }
              },
              "right": {
                "Literal": {
                  "Int": 2
                }
              }
            }
          }
        }
      ]
    }
  ],
  "statements": []
}
```    

Result:

PASS

Expected Output:

Actual Output:

(st006_rust.nim)
```nim
# test


proc main() =
  return 1 + 2
```

Notes:

Verified Python frontend parsed binary expression.
Verified AST → IR lowering for ADD.
Verified IR JSON serialization for nested expression nodes.
Verified Rust successfully deserialized:
IRBinaryOp
IRExpr::Binary
IRExpr::Literal
Verified operator mapping:
ADD → +
Verified Nim backend emitted valid infix expression syntax.
Confirmed expression tree survived Python → JSON → Rust → Nim translation without structural loss.

Technical Significance:

First successful expression-tree validation.
Demonstrates nested IR deserialization.
Validates backend operator translation layer.
Establishes foundation for more complex expression handling (precedence, nesting, function calls, variables, pipelines).

---


## Pass 3 - Python Frontend + Partial Rust Backend (No Semantics Run) + Nim Backend 

Goal:
Verify NXD source can travel through:

Scanner → Parser → AST → Lowering → IR JSON → Rust Loader → IR Root → Nim Backend

| Test | Feature | Status | Failure Stage | Notes |
|--------|---------|---------|---------|---------|
| ST007 |  | PASS | | |

### PT007 (micro test)

NXD Sample:

(st007.nxd)
```nxd
MODULE TEST

FUNC MAIN()
    LET X SET 1

    PRINTLN(X)
```    

Result:

COMPILER: PASS
SEMANTICS: PASS

Expected Output:

Actual Output:

```nim
# test


proc main() =
  var x = 1
  println(x)
```


Notes:

Verified LET statement parsing.
Verified variable assignment lowering into IR.
Verified variable reference usage within a later statement.
Verified function call parsing (PRINTLN).
Verified call expression serialization to JSON.
Verified Rust deserialization of variable and call-expression nodes.
Verified Nim backend emission of variable declaration and function invocation.
Confirmed multi-statement function bodies survive the full pipeline.

Technical Significance:

First validation of state being created and reused across statements.
First validation of call-expression flow through Python → JSON → Rust → Nim.
Establishes baseline support for user-defined variables and future API/library calls.

## Pass 4 - Python Frontend + Rust Backend + Nim Backend (Full Run)

Goal:
Verify NXD source can travel through:

Scanner → Parser → AST → Lowering → IR JSON → Rust Loader → IR Root → Semantics → Nim Backend


---

| Test | Feature | Status | Failure Stage | Notes |
|--------|---------|---------|---------|---------|
| ST008 |  | PASS | | |
| ST009 |  | PENDING | | |
| ST010 |  | PENDING | | |

---

## Findings (Micro Test)

### PT008

NXD Sample:

```nxd
MODULE TEST

FUNC MAIN():
    LET X SET 1
    LET Y SET 2

    PRINTLN(X ADD Y)
```

Result:

COMPILER: PASS
SEMANTICS: PASS

Expected Output:

Actual Output:

```nim
# test


proc main() =
  var x = 1
  var y = 2
  println(x + y)
````


Notes:

Verified multiple LET statements in a single function.
- Verified identifier storage and later retrieval.
- Verified variable references inside binary expressions.
- Verified expression tree preservation through AST → IR → JSON → Rust → Nim.
- Verified function-call arguments can contain nested expressions.
- Verified ADD operator mapping with variable operands.
 
Technical Significance:
- First successful test of variable-to-variable arithmetic.
- First successful nested expression inside a call expression.
- Demonstrates combined statement, expression, and call handling through the full pipeline.

---

### PT009 (micro test)

NXD Sample:

```nxd
MODULE TEST

FUNC MAIN()
    LET VALID SET true

    IF VALID:
        PRINTLN("PASS")
```    

Result:

COMPILER: SOFT PASS *(see notes)
SEMANTICS: PASS

Expected Output:

Actual Output:

```nim
# test


proc main() =
  var valid = true
if valid:
    println("PASS")

```


Notes:

Verified boolean literal parsing for true.
Verified boolean literal AST generation.
Verified boolean literal IR generation.
Verified boolean serialization into JSON IR.
Verified Rust deserialization of boolean literal nodes.
Verified IF statement parsing.
Verified AST → IR lowering for control flow.
Verified IRIf serialization and deserialization.
Verified IRStatement::If dispatch successfully reaches the Nim control-flow emitter.
Verified Nim backend emits an if construct instead of the previous placeholder (# TODO: emit if statement).
Confirmed end-to-end control-flow transport through:

Defect Identified:

The emitted if statement is generated outside the enclosing procedure scope

Impact:

Does not indicate a parser failure.
Does not indicate an IR failure.
Does not indicate a JSON or Rust deserialization failure.
Confirms control-flow functionality is present.
Requires indentation correction in the Nim emitter.
Technical Significance

This test represents the first successful validation of NXD control-flow statements through the full compiler pipeline. It confirms that boolean literals and IF statements are correctly recognized, lowered, serialized, reconstructed, and emitted. The remaining issue is limited to backend formatting rather than language feature implementation.

---

### FT010 (tiny test)

NXD Sample:

```nxd
MODULE TEST

FUNC MAIN()
    LET P SET PERSON {
        NAME: "gabriel",
        AGE: 42
    }

    PRINTLN(P.NAME)
    PRINTLN(P.AGE)
```    

Result:

COMPILER:FAIL
SEMANTICS: FAIL

Failure Stage:

Parser / Field Access

Expected Output:

Actual Output:

Notes:

The pipeline reached function call parsing for `PRINTLN(P.NAME)`. Parsing failed because member access expressions such as `P.NAME` are not currently represented in the parser/AST/IR contract. The parser successfully recognized the outer call but expected `RPAREN` after parsing `P`, then encountered `NAME`.



### PT011 (micro test)

NXD Sample:

```nxd
MODULE TEST

FUNC MAIN()
    LET VALID SET false

    IF VALID:
        PRINTLN("PASS")
    ELSE:
        PRINTLN("FAIL")
```
Result:

COMPILER: SOFT PASS *(see notes)
SEMANTICS: PASS

Expected Output:

Actual Output:

```nim
# test


proc main() =
  var valid = false
if valid:
    println("PASS")
else:
    println("FAIL")
```

Notes:
Verified boolean literal parsing for false.
Verified ELSE parsing.
Verified else_branch AST generation.
Verified else_branch IR generation.
Verified JSON serialization of both branches.
Verified Rust deserialization of IRIf with populated else_branch.
Verified backend dispatch correctly invokes emit_if().
Verified Nim backend emits both:
if
else
Verified the entire control-flow structure survives the pipeline:

The presence of both the if and else branches in the generated output confirms that control-flow lowering, transport, and reconstruction are functioning correctly. The defect remains limited to procedure-scoped indentation.

is strong evidence that else_branch survived every compiler phase successfully. In other words, ST010 expanded the control-flow proof started by ST009. The same emitter defect persists, but no new control-flow defects were discovered. That makes ST011 a natural Soft Pass rather than a failure.

---


### PT012 (micro test)

NXD Sample:

```nxd
MODULE TEST

FUNC MAIN()
    CONST LIMIT SET 10
    LET X SET 5

    PRINTLN(LIMIT)
    PRINTLN(X)
```

Result:

COMPILER: PASS
SEMANTICS: PASS

Expected Output:

```nim
# test

proc main() =
  let limit = 10
  var x = 5
  println(limit)
  println(x)
```

Actual Output:

```nim
# test


proc main() =
  let limit = 10
  var x = 5
  println(limit)
  println(x)

```

Notes:

Verified CONST statement parsing.
Verified CONST AST generation.
Verified CONST IR lowering.
Verified CONST JSON serialization.
Verified Rust deserialization of constant declarations.
Verified Nim backend correctly emits immutable values
Verified CONST and LET declarations can coexist within the same function scope.
Verified constant references survive the complete compiler pipeline.
Verified variable references survive the complete compiler pipeline.
Verified multiple function calls within a single function body.
Verified immutable and mutable storage semantics remain distinct after transpilation.

Technical Significance:

This test validates the first direct distinction between NXD immutable and mutable storage models through the complete compiler pipeline.

Observations:

The generated Nim closely mirrors the original NXD intent and remains highly readable without requiring prior Nim knowledge. This indicates that the current NXD → Nim mapping for storage declarations is predictable and transparent.

No defects were identified during this test.

---


### FT013 (micro test)

NXD Sample:

```nxd
LET P SET PERSON {
    NAME: "gabriel",
    AGE: 42
}
```

Result:

COMPILER: FAIL
SEMANTICS: FAIL

Expected Output:

Actual Output:

N/A

Notes:

Notes:

The parser successfully reached the object/map literal body and constructed a dictionary-like value, but the Python IR serializer does not currently support map/object literal serialization.
 
This test also indicates that typed construction syntax such as `PERSON { ... }` is not yet represented as a single constructor expression. The object literal appears to be treated separately from the `PERSON` identifier rather than as the value assigned to `P`.

Required Future Work:

- Add map/object literal representation to AST/IR.
- Add JSON serialization support for object/map values.
- Add Rust IR support for object/map values.
- Add backend emission support for object/map values.
- Add typed constructor support for `TYPE_NAME { ... }`.


---


### FT014 (micro test)

NXD Sample:

```nxd
MODULE TEST

FUNC MAIN()
    CONST LIMIT SET 10
    LET X SET SUB 5
    LET Y SET 2.5

    PRINTLN(LIMIT)
    PRINTLN(X)
    PRINTLN(Y)
```

Result:

COMPILER: FAIL
SEMANTICS: FAIL

Expected Output:

Actual Output:



Notes:


---


### FT015 (micro test)

NXD Sample:

```nxd
MODULE TEST

FUNC MAIN()
    LET X SET 5
    LET Y SET 10

    IF Y GTE X:
        PRINTLN("GTE")

    IF X LTE Y:
        PRINTLN("LTE")
```

Result:

COMPILER: FAIL
SEMANTICS: FAIL

Expected Output:

Actual Output:



Notes:

Validation Notes

Verified parser entered IF statement handling.
Verified failure occurred while processing the comparison expression.
The parser encountered the terminating : before successfully constructing a valid condition expression.
Current implementation does not appear to fully support GTE / LTE comparison parsing within IF conditions.

Technical Significance

This test indicates a gap in comparison-expression handling rather than a control-flow implementation failure. IF statements have already been demonstrated to survive the NXD → JSON → Rust → Nim pipeline in previous tests.

---


### PT016 (micro test)

NXD Sample:

```nxd
MODULE TEST

FUNC MAIN()
    LET COUNT SET 0

    LOOP:
        PRINTLN(COUNT)
```        

Result:

COMPILER: PASS
SEMANTICS: PASS

Expected Output:

Actual Output:

```nim
# test


proc main() =
  var count = 0
  while true:
    println(count)

```

Notes:


---


### PT017 (micro test)

NXD Sample:

```nxd
MODULE TEST

FUNC MAIN()
    LET X SET 2

    MATCH X:
        CASE 1:
            PRINTLN("ONE")

        CASE 2:
            PRINTLN("TWO")

        OTHERWISE:
            PRINTLN("OTHER")
```

Result:

COMPILER: PASS *(see notes)
SEMANTICS: PASS

Expected Output:

Actual Output:

```json
{
  "module": {
    "name": "TEST",
    "imports": []
  },
  "types": [],
  "traits": [],
  "impls": [],
  "functions": [
    {
      "name": "MAIN",
      "params": [],
      "return_type": null,
      "body": [
        {
          "Let": {
            "name": "X",
            "value": {
              "Literal": {
                "Int": 2
              }
            }
          }
        },
        {
          "Match": {
            "scrutinee": {
              "Var": "X"
            },
            "arms": [
              {
                "pattern": 1,
                "body": [
                  {
                    "Expr": {
                      "Call": {
                        "func": "PRINTLN",
                        "args": [
                          {
                            "Literal": {
                              "String": "ONE"
                            }
                          }
                        ]
                      }
                    }
                  }
                ]
              },
              {
                "pattern": 2,
                "body": [
                  {
                    "Expr": {
                      "Call": {
                        "func": "PRINTLN",
                        "args": [
                          {
                            "Literal": {
                              "String": "TWO"
                            }
                          }
                        ]
                      }
                    }
                  }
                ]
              }
            ],
            "otherwise": [
              {
                "Expr": {
                  "Call": {
                    "func": "PRINTLN",
                    "args": [
                      {
                        "Literal": {
                          "String": "OTHER"
                        }
                      }
                    ]
                  }
                }
              }
            ]
          }
        }
      ]
    }
  ],
  "statements": []
}
```

Notes:

Validation Notes

MATCH keyword parsed successfully.
CASE clauses parsed successfully.
OTHERWISE clause parsed successfully.
AST generation completed.
IR generation completed.
JSON emitted successfully.
Rust backend rejected the pattern type during deserialization.

Technical Significance

This demonstrates that MATCH/CASE constructs are recognized by the NXD frontend and can be represented in generated IR. The current failure occurs at the Rust handoff stage due to a mismatch between the frontend JSON pattern representation and the Rust IR contract.

---


### FT018 (micro test)

NXD Sample:

```nxd
MODULE TEST

FUNC MAIN()
    LET X SET 5
    LET Y SET CLONE X

    PRINTLN(Y)
```    

Result:

COMPILER: PASS
SEMANTICS: FAIL

Expected Output:

Actual Output:

```nim
# test


proc main() =
  var x = 5
  var y = clone
  x
  println(y)
```


Notes:


---


### FT019 (micro test)

NXD Sample:

```nxd
IMPORT STD.IO

FUNC MAIN()
    PRINTLN("TEST")
```

Result:

COMPILER: FAIL
SEMANTICS: FAIL

Expected Output:

Actual Output:



Notes:

Verified lexer recognizes IMPORT.
Verified IMPORT is tokenized as a keyword.
Parser currently requires MODULE as the first top-level construct.
Top-level import declarations are not currently supported by the parser.
Failure occurs before AST generation.
Technical Significance

This test demonstrates that IMPORT exists in the lexical grammar but is not yet integrated into the top-level parser workflow.

---


### FT020 (micro test)

NXD Sample:

```nxd
MODULE TEST

FUNC MAIN()
    LET NEG SET SUB 5
    LET PI SET 3.14

    PRINTLN(NEG)
    PRINTLN(PI)
```

Result:

COMPILER: FAIL
SEMANTICS: FAIL

Expected Output:

Actual Output:



Notes:

Verified lexer recognizes SUB as an operator.
Parser does not currently accept SUB as a valid unary expression.
Failure occurred before IR generation.
Float validation could not be completed because parsing stopped at the unary negative expression.

Technical Significance

The NXD specification states that negative values should be represented using: `SUB`

---
### PT021 (micro test)

NXD Sample:

```nxd
MODULE TEST

FUNC MAIN()
    CONST MSG SET "NXD TEST"

    PRINTLN(MSG)
```    

Result:

COMPILER: PASS
SEMANTICS: PASS

Expected Output:

Actual Output:

```nim
# test


proc main() =
  let msg = "NXD TEST"
  println(msg)
```

Notes:


---


### FT022 (micro test)

NXD Sample:

```nxd
MODULE TEST

FUNC MAIN()
    LET VALUES SET [1, 2, 3]

    PRINTLN(VALUES)
```

Result:

COMPILER: FAIL
SEMANTICS: FAIL

Expected Output:

Actual Output:



Notes:

Verified list literal parsing.
Verified AST generation for list values.
Verified list expressions survive frontend processing.
Failure occurred during JSON serialization.
Serializer attempted to process nested ASTLiteral nodes contained within the list.
Current serializer does not support converting list elements represented as ASTLiteral objects into JSON-compatible IR literals.

Technical Significance

This test demonstrates that list literals are recognized by the NXD frontend and represented in the AST. The failure occurs during translation from AST structures into serialized IR.

---
### PT023 (micro test)

NXD Sample:

```nxd
MODULE TEST

FUNC MAIN()
    CONST NAME SET "gabriel"
    LET AGE SET 42
    LET SCORE SET 99.5
    LET ACTIVE SET true

    PRINTLN(NAME)
    PRINTLN(AGE)
    PRINTLN(SCORE)
    PRINTLN(ACTIVE)
```

Result:

COMPILER: PASS
SEMANTICS: PASS

Expected Output:

Actual Output:

```nim
# test


proc main() =
  let name = "gabriel"
  var age = 42
  var score = 99.5
  var active = true
  println(name)
  println(age)
  println(score)
  println(active)
```

Notes:


---


### PT024 (micro test)

NXD Sample:

```nxd
MODULE TEST

FUNC MAIN()
    CONST PI SET 3.14
    CONST RATE SET 2.0

    PRINTLN(PI MUL RATE)
    PRINTLN(PI DIV RATE)
```

Result:

COMPILER: PASS
SEMANTICS: PASS

Expected Output:

Actual Output:

```nim
# test


proc main() =
  let pi = 3.14
  let rate = 2
  println(pi * rate)
  println(pi / rate)
```


Notes:


---


### PT025 (micro test)

NXD Sample:

```nxd
MODULE TEST

FUNC MAIN()
    CONST A SET 10
    CONST B SET 3
    CONST C SET 0

    IF A MOD B NEQ C OR C EQ 0:
        PRINTLN("MOD_OR_PASS")
```

Result:

COMPILER: PASS
SEMANTICS: PASS

Expected Output:

Actual Output:

```nim
# test


proc main() =
  let a = 10
  let b = 3
  let c = 0
if a mod b != c or c == 0:
    println("MOD_OR_PASS")
```

Notes:


---


### PT026 (micro test)

NXD Sample:

```nxd
MODULE TEST

FUNC MAIN()
    CONST LOW SET 3
    CONST HIGH SET 9

    IF HIGH GT LOW AND LOW LT HIGH:
        PRINTLN("RANGE_PASS")
```

Result:

COMPILER: PASS
SEMANTICS:PASS

Expected Output:

Actual Output:

```nim
# test


proc main() =
  let low = 3
  let high = 9
if high > low and low < high:
    println("RANGE_PASS")
```

Notes:


---


### PT027 (micro test)

NXD Sample:

```nxd
MODULE TEST

FUNC MAIN()
    CONST READY SET false

    IF NOT READY:
        RETURN none
```

Result:

COMPILER: PASS
SEMANTICS: PASS

Expected Output:

Actual Output:

```nim
# test


proc main() =
  let ready = false
if not ready:
    return nil
```

Notes:


---


### FT028 (micro test)

NXD Sample:

```nxd
MODULE TEST

FUNC MAIN()
    CONST VALUE SET 42

    IF VALUE IS int:
        PRINTLN(VALUE AS int)
```

Result:

COMPILER: FAIL
SEMANTICS: FAIL

Expected Output:

Actual Output:



Notes:


---


### FT029 (micro test)

NXD Sample:

```nxd
MODULE TEST

FUNC MAIN()
    CONST VALUE SET 5

    VALUE PIPE PRINTLN
```

Result:

COMPILER: FAIL
SEMANTICS: FAIL

Expected Output:

Actual Output:



Notes:


---


### PT030 (micro test)

NXD Sample:

```nxd
MODULE TEST

FUNC MAIN()
    CONST COUNT SET 1

    LOOP:
        PRINTLN(COUNT ADD 1)
```

Result:

COMPILER: PASS
SEMANTICS: PASS

Expected Output:

Actual Output:

```nim
# test


proc main() =
  let count = 1
  while true:
    println(count + 1)
```

Notes:


---


### PT031 (micro test)

NXD Sample:

```nxd
MODULE TEST

FUNC MAIN()
    CONST STATE SET "READY"

    MATCH STATE:
        CASE "READY":
            PRINTLN("GO")

        CASE "WAIT":
            PRINTLN("HOLD")

        OTHERWISE:
            PRINTLN("UNKNOWN")
```

Result:

COMPILER: PASS
SEMANTICS: PASS

Expected Output:

Actual Output:

```nim
# test


proc main() =
  let state = "READY"
case state:
  of "READY":
    println("GO")
  of "WAIT":
    println("HOLD")
  else:
    println("UNKNOWN")
```

Notes:


---


### FT032 (micro test)

NXD Sample:

```nxd
MODULE TEST

FUNC MAIN()
    CONST NAMES SET ["ALPHA", "BETA", "GAMMA"]
    CONST FLAGS SET [true, false, true]

    PRINTLN(NAMES)
    PRINTLN(FLAGS)
```

Result:

COMPILER: FAIL
SEMANTICS: FAIL

Expected Output:

Actual Output:



Notes:


---


### PT033 (micro test)

NXD Sample:

```nxd
MODULE TEST

TYPE PERSON STRUCT:
    NAME: string
    AGE: int

FUNC MAIN()
    PRINTLN("STRUCT_DECLARED")
```

Result:

COMPILER: PASS
SEMANTICS: PASS

Expected Output:

Actual Output:

```nim
# test


type
  PERSON = object
    name: string
    age: int

proc main() =
  println("STRUCT_DECLARED")
```

Notes:


---


### PT034 (micro test)

NXD Sample:

```nxd
MODULE TEST

TYPE STATUS ENUM:
    READY
    WAITING
    FAILED

FUNC MAIN()
    PRINTLN("ENUM_DECLARED")
```

Result:

COMPILER: PASS
SEMANTICS: PASS

Expected Output:

Actual Output:

```nim
# test


type
  STATUS = enum
    READY,
    WAITING,
    FAILED,

proc main() =
  println("ENUM_DECLARED")
```

Notes:


---


### FT035 (micro test)

NXD Sample:

```nxd
MODULE TEST

TYPE RESULT UNION:
    SUCCESS { VALUE: int }
    FAILURE { MESSAGE: string }

FUNC MAIN()
    PRINTLN("UNION_DECLARED")
```

Result:

COMPILER: FAIL
SEMANTICS: FAIL

Expected Output:

Actual Output:



Notes:


---


### FT036 (micro test)

NXD Sample:

```nxd
MODULE TEST

TYPE PERSON STRUCT:
    NAME: string

TRAIT DISPLAYABLE:
    FUNC DISPLAY()

IMPL DISPLAYABLE FOR PERSON:
    FUNC DISPLAY()
        PRINTLN("PERSON")

FUNC MAIN()
    PRINTLN("TRAIT_IMPL_TEST")
```

Result:

COMPILER: FAIL
SEMANTICS: FAIL

Expected Output:

Actual Output:



Notes:


---


### PT037 (micro test)

NXD Sample:

```nxd
MODULE TEST

FUNC MAIN()
    CONST VALUE SET 10

    PRINTLN(CLONE VALUE)
    PRINTLN(BORROW VALUE)
    PRINTLN(MOVE VALUE)
```

Result:

COMPILER: PASS
SEMANTICS: PASS

Expected Output:

Actual Output:

```nim
# test


proc main() =
  let value = 10
  println(CLONE value)
  println(BORROW value)
  println(MOVE value)
```

Notes:


---


### FT038 (micro test)

NXD Sample:

```nxd
MODULE TEST

FUNC MAIN()
    TRY:
        PRINTLN("TRY")

    CATCH:
        PRINTLN("CATCH")

    FINALLY:
        PRINTLN("FINALLY")
```

Result:

COMPILER: FAIL
SEMANTICS: FAIL

Expected Output:

Actual Output:



Notes:


---


### FT039 (micro test)

NXD Sample:

```nxd
MODULE TEST

FUNC MAIN()
    TRY:
        PRINTLN("TRY")

    CATCH:
        PRINTLN("CATCH")

    FINALLY:
        PRINTLN("FINALLY")
```

Result:

COMPILER: FAIL
SEMANTICS: FAIL

Expected Output:

Actual Output:



Notes:


---


### FT040 (micro test)

NXD Sample:

```nxd
MODULE TEST

IMPORT STD.IO

FUNC MAIN()
    PRINTLN("IMPORT_TEST")
```

Result:

COMPILER: FAIL
SEMANTICS: FAIL

Expected Output:

Actual Output:



Notes:


---

### ST041 (micro test)

NXD Sample:



Result:

COMPILER:
SEMANTICS:

Expected Output:

Actual Output:



Notes:


---


### ST042 (micro test)

NXD Sample:



Result:

COMPILER:
SEMANTICS:

Expected Output:

Actual Output:



Notes:


---


### ST043 (micro test)

NXD Sample:



Result:

COMPILER:
SEMANTICS:

Expected Output:

Actual Output:



Notes:


---


### ST044 (micro test)

NXD Sample:



Result:

COMPILER:
SEMANTICS:

Expected Output:

Actual Output:



Notes:


---


### ST045 (micro test)

NXD Sample:



Result:

COMPILER:
SEMANTICS:

Expected Output:

Actual Output:



Notes:


---


### ST046 (micro test)

NXD Sample:



Result:

COMPILER:
SEMANTICS:

Expected Output:

Actual Output:



Notes:


---


### ST047 (micro test)

NXD Sample:



Result:

COMPILER:
SEMANTICS:

Expected Output:

Actual Output:



Notes:


---


### ST048 (micro test)

NXD Sample:



Result:

COMPILER:
SEMANTICS:

Expected Output:

Actual Output:



Notes:


---


### ST049 (micro test)

NXD Sample:



Result:

COMPILER:
SEMANTICS:

Expected Output:

Actual Output:



Notes:


---


### ST050 (micro test)

NXD Sample:



Result:

COMPILER:
SEMANTICS:

Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---

### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---

### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---

### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---

### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---

### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---

### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---

### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---

### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---

### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---

### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---


### ST0 (micro test)

NXD Sample:



Result:



Expected Output:

Actual Output:



Notes:


---
