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

### ST001

NXD Sample:

```nxd
MODULE TEST
```

Result: 

PASS

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

### ST002

NXD Sample:

```nxd
MODULE TEST

FUNC MAIN():
    RETURN 1
```    

Result:

PASS

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

### ST003

NXD Sample:

```nxd
MODULE TEST

FUNC MAIN():
    RETURN 1 ADD 2
```    

Result:

PASS

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

### ST004

NXD Sample:

(st001_ir.json)
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

### ST005 (micro test)

NXD Sample:

(st002_ir.json)
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

### ST006 (micro test)

NXD Sample:

(st003_ir.json)
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

### ST007 (micro test)

NXD Sample:

(st007.nxd)
```nxd
MODULE TEST

FUNC MAIN()
    LET X SET 1

    PRINTLN(X)
```    

Result:

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

### ST008

NXD Sample:

```nxd
MODULE TEST

FUNC MAIN():
    LET X SET 1
    LET Y SET 2

    PRINTLN(X ADD Y)
```

Result:

PASS

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

### ST009 (micro test)

NXD Sample:

```nxd
MODULE TEST
    FUNC MAIN():
    RETURN 1
```    

Result:

Expected Output:

Actual Output:

Notes:

---

### ST010 (tiny test)

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

FAIL

Failure Stage:

Parser / Field Access

Expected Output:

Actual Output:

Notes:

The pipeline reached function call parsing for `PRINTLN(P.NAME)`. Parsing failed because member access expressions such as `P.NAME` are not currently represented in the parser/AST/IR contract. The parser successfully recognized the outer call but expected `RPAREN` after parsing `P`, then encountered `NAME`.

