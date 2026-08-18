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

## Pass 2 - Python Frontend + Partial Rust Backend (No Semantics Run) + Nim Backend 

Goal:
Verify NXD source can travel through:

Scanner → Parser → AST → Lowering → IR JSON → Rust Loader → IR Root → Nim Backend

---

| Test | Feature | Status | Failure Stage | Notes |
|--------|---------|---------|---------|---------|
| ST004 | Module Only | PENDING |  |  |
| ST005 | Function Return Literal | PENDING |  |  |
| ST006 | Binary Expression | PENDING |  |  |

---

## Findings (Micro Test)

### ST004

NXD Sample:

```nxd
MODULE TEST
```

Result:

Expected Output:

Actual Output:

Notes:

---

### ST005 (micro test)

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

### ST006 (micro test)

NXD Sample:

```nxd
MODULE TEST
    FUNC MAIN():
    RETURN 1 ADD 2
```    

Result:

Expected Output:

Actual Output:

Notes:

---

## Pass 3 - Python Frontend + Partial Rust Backend + Nim Backend (Full Run)

Goal:
Verify NXD source can travel through:

Scanner → Parser → AST → Lowering → IR JSON → Rust Loader → IR Root → Semantics → Nim Backend

---

| Test | Feature | Status | Failure Stage | Notes |
|--------|---------|---------|---------|---------|
| ST007 | Add One | PENDING | | |
| ST008 | Factorial | PENDING | | |
| ST009 | If Else | PENDING | | |

---

## Findings (Micro Test)

### ST007

NXD Sample:

```nxd
MODULE TEST
```

Result:

Expected Output:

Actual Output:

Notes:

---

### ST008 (micro test)

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

### ST009 (micro test)

NXD Sample:

```nxd
MODULE TEST
    FUNC MAIN():
    RETURN 1 ADD 2
```    

Result:

Expected Output:

Actual Output:

Notes:


