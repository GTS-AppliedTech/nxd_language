# Expected to Fail

### ST101 (micro failure)

NXD Sample:

```nxd
module TEST
```

Result:

FAIL

Expected Output:

FAIL - Keyword capitalization violation

Expected Rejection Stage:

[X] Lexer
[X] Parser
[ ] AST
[ ] IR
[ ] Semantics
[ ] Backend

Actual Output:

N/A

Notes: Intentional Parser failure. no IR JSON created. NXD requires the MODULE keyword to be fully uppercase.
The lexer classified "module" as LOWNAME rather than the MODULE keyword.
The parser rejected the first token when beginning module parsing. 


---


### ST102 (micro failure)

NXD Sample:

```nxd
MODULE TEST

FUNC MAIN():
    Return 1
```    

Result:

FAIL

Expected Output:

FAIL - keyword capitalization violation

Expected Rejection Stage:

[X] Lexer
[X] Parser
[ ] AST
[ ] IR
[ ] Semantics
[ ] Backend

Actual Output:

N/A

Notes:

intentional Parser failure. no IR JSON generated. The lexer/parser appears to separate or consume the leading uppercase R, resulting in the remaining token "eturn".

---


### ST103 (micro failure)

NXD Sample:

```nxd
MODULE TEST

FUNC MAIN ():
    RETURN 1 ADD 2
```

Result:

PASS

Expected Output:

FAIL -

Expected Rejection Stage:

[ ] Lexer
[ ] Parser
[ ] AST
[ ] IR
[ ] Semantics
[ ] Backend

Actual Output:

N/A

Notes:

intentional Parser failure. no IR JSON generated 

---


### ST104 (micro failure)

NXD Sample:

```nxd
MODULE test
```

Result:

FAIL

Expected Output:

FAIL - Capitalization violation

Expected Rejection Stage:

[X] Lexer
[X] Parser
[ ] AST
[ ] IR
[ ] Semantics
[ ] Backend

Actual Output:

N/A

Notes:

intentional Parser failure. no IR JSON generated. After the MODULE keyword, NXD expects an IDENT token.
The lexer classified "test" as LOWNAME rather than IDENT.

---


### ST105 (micro failure)

NXD Sample:

```nxd
MODULE TEST

FUNC MAIN():
RETURN 1
```

Result:

PASS

Expected Output:

FAIL -

Expected Rejection Stage:

[ ] Lexer
[ ] Parser
[ ] AST
[ ] IR
[ ] Semantics
[ ] Backend

Actual Output:

N/A

Notes:

intentional Parser failure. no IR JSON generated 

---


### ST106 (micro failure)

NXD Sample:

```nxd
MODULE TEST

FUNC MAIN();
    RETURN 1 ADD 2
    
```

Result:

PASS

Expected Output:

FAIL - 

Expected Rejection Stage:

[ ] Lexer
[ ] Parser
[ ] AST
[ ] IR
[ ] Semantics
[ ] Backend


Actual Output:



Notes:

intentional Parser failure. no IR JSON generated - incorrect punctuation

---

### ST107 (micro failure)

NXD Sample:

```nxd
MODULE TEST

FUNC MAIN()
    LET X SET 1
    
    echo(X)
```

Result:

FAIL

Expected Output:

FAIL - Capitalization Violation

Expected Rejection Stage:

[ ] Lexer
[X] Parser
[ ] AST
[ ] IR
[ ] Semantics
[ ] Backend

Actual Output:

N/A

Notes:

intentional Parser failure. no IR JSON generated. NXD capitalization rules require uppercase language constructs.
The lexer classified "echo" as LOWNAME.
The parser rejected the construct before IR generation.

---


### ST108 (micro failure)

NXD Sample:

```nxd
MODULE TEST

FUNC MAIN():
    LET X SET 1
    LET Y SET 2

    ECHO(X ADD Y)
```

Result:

FAIL

Expected Output:

FAIL - Undefined symbol

Expected Rejection Stage:

[ ] Lexer
[ ] Parser
[ ] AST
[ ] IR
[X] Semantics
[ ] Backend

Actual Output:

N/A

Notes:

intentional Semantics failure. ECHO() is syntactically valid and parses as a function reference.
The parser accepted the construct and IR was generated successfully.
Semantic analysis attempted to resolve ECHO and failed:
UndefinedSymbol { name: "ECHO" }

---


### ST109 (micro failure)

NXD Sample:

```nxd
MODULE TEST

FUNC MAIN()
    LET VALID SET TRUE

    IF VALID:
        PRINTLN("True")
```

Result:

FAIL

Expected Output:

FAIL - Undefined symbol

Expected Rejection Stage:

[ ] Lexer
[ ] Parser
[ ] AST
[ ] IR
[X] Semantics
[ ] Backend

Actual Output:



Notes:

intentional Semantics failure. NXD requires lowercase boolean literals.
TRUE is interpreted as an identifier rather than a boolean literal.
String literals are exempt from capitalization rules.
Example:
TRUE -> invalid symbol
"True" -> valid string literal
Compiler Result:
UndefinedSymbol { name: "TRUE" }

---


### ST111 (micro failure)

NXD Sample:

```nxd
MODULE TEST

MAIN FUNC()
    LET VALID SET false

    IF VALID:
        PRINTLN("PASS")
    ELSE:
        PRINTLN("FAIL")
```

Result:

FAIL

Expected Output:

FAIL - Function declaration order violation

Expected Rejection Stage:

[ ] Lexer
[X] Parser
[ ] AST
[ ] IR
[ ] Semantics
[ ] Backend

Actual Output:

N/A

Notes:

intentional Parser failure. no IR JSON generated. NXD requires the function declaration keyword FUNC to appear before the function identifier.

---


### ST112 (micro failure)

NXD Sample:

```nxd
MODULE TEST

FUNC MAIN()
    CONST LIMIT SET 10
    LET X SET -5

    PRINTLN(LIMIT)
    PRINTLN(X)
```

Result:

PASS

Expected Output:

FAIL -

Expected Rejection Stage:

[ ] Lexer
[ ] Parser
[ ] AST
[ ] IR
[ ] Semantics
[ ] Backend

Actual Output:

N/A

Notes:

intentional Parser failure. no IR JSON generated - invalid variable 
---


### ST116 (micro failure)

NXD Sample:

```nxd
MODULE TEST

FUNC MAIN()
    LET COUNT SET 0

    LOOP:
        PRINTLN(COUNTX)
```

Result:

FAIL

Expected Output:

FAIL -

Expected Rejection Stage:

[ ] Lexer
[ ] Parser
[ ] AST
[ ] IR
[X] Semantics
[ ] Backend

Actual Output:



Notes:

intentional Semantics failure. COUNT was declared and valid.
COUNTX was never declared.
Parser accepted the identifier reference and IR was successfully generated.
Semantic analysis failed with:
UndefinedSymbol { name: "COUNTX" }

---


### ST117 (micro failure)

NXD Sample:

```nxd
MODULE TEST

FUNC MAIN()
    LET X SET 2

    MATCH X:
        CASE 1:
            PRINTLN(ONE)

        CASE 2:
            PRINTLN("TWO")

        OTHERWISE:
            PRINTLN("OTHER")
```

Result:

PASS

Expected Output:

FAIL -

Expected Rejection Stage:

[ ] Lexer
[ ] Parser
[ ] AST
[ ] IR
[ ] Semantics
[ ] Backend

Actual Output:

N/A

Notes:

intentional Parser failure. no IR JSON generated - invalid case type invalid string

---


### ST121 (micro failure)

NXD Sample:

```nxd
MODULE TEST

FUNC MAIN()
    CONST MSG SET: "NXD TEST"

    PRINTLN(MSG)
```

Result:

FAIL

Expected Output:

FAIL - Invalid expression

Expected Rejection Stage:

[ ] Lexer
[X] Parser
[ ] AST
[ ] IR
[ ] Semantics
[ ] Backend

Actual Output:

N/A

Notes:

intentional Pa/rser failure. no IR JSON generated. While parsing a CONST declaration value, the parser encountered a colon token.
A colon is not a valid primary expression and compilation stopped.

---


### ST123 (micro failure)

NXD Sample:

```nxd
MODULE TEST

FUNC MAIN()
    CONST NAME SET "gabriel"
    LET AGE SET 42
    LET SCORE SET 99.5
    LET ACTIVE SET TRUE

    PRINTLN(NAME)
    PRINTLN(AGE)
    PRINTLN(SCORE)
    PRINTLN(ACTIVE)
```

Result:

FAIL

Expected Output:

FAIL -

Expected Rejection Stage:

[ ] Lexer
[ ] Parser
[ ] AST
[ ] IR
[X] Semantics
[ ] Backend

Actual Output:

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
          "Const": {
            "name": "NAME",
            "value": {
              "Literal": {
                "String": "gabriel"
              }
            }
          }
        },
        {
          "Let": {
            "name": "AGE",
            "value": {
              "Literal": {
                "Int": 42
              }
            }
          }
        },
        {
          "Let": {
            "name": "SCORE",
            "value": {
              "Literal": {
                "Float": 99.5
              }
            }
          }
        },
        {
          "Let": {
            "name": "ACTIVE",
            "value": {
              "Literal": {
                "Bool": true
              }
            }
          }
        },
        {
          "Expr": {
            "Call": {
              "func": "PRINTLN",
              "args": [
                {
                  "Var": "NAME"
                }
              ]
            }
          }
        },
        {
          "Expr": {
            "Call": {
              "func": "PRINTLN",
              "args": [
                {
                  "Var": "AGE"
                }
              ]
            }
          }
        },
        {
          "Expr": {
            "Call": {
              "func": "PRINTLN",
              "args": [
                {
                  "Var": "SCORE"
                }
              ]
            }
          }
        },
        {
          "Expr": {
            "Call": {
              "func": "PRINTLN",
              "args": [
                {
                  "Var": "ACTIVE"
                }
              ]
            }
          }
        }
      ]
    }
  ],
  "statements": []
}

Notes:

intentional semantics failure. The lexer and parser accepted `TRUE` as an identifier rather than recognizing it as a Boolean literal. AST construction, IR lowering, and JSON generation completed successfully.
The Rust semantic analyzer attempted to resolve `TRUE` as a declared symbol and rejected the program with:
`UndefinedSymbol { name: "TRUE" }` All NXD boolean values are lowercase.

---


### ST124 (micro failure)

NXD Sample:

```nxd
MODULE TEST

FUNC MAIN()
    CONST PI SET 3.14
    CONST RATE SET 2.0

    println(PI MUL RATE)
    println(PI DIV RATE)
```

Result:

FAIL

Expected Output:

FAIL - Keyword capitalization violation

Expected Rejection Stage:

[X] Lexer
[X] Parser
[ ] AST
[ ] IR
[ ] Semantics
[ ] Backend

Actual Output:

N/A

Notes:

intentional Parser failure. no IR JSON generated - NXD requires PRINTLN() as the valid output keyword.
The parser received the token 'println' classified as LOWNAME and rejected it as an invalid primary expression.

---


### ST125 (micro failure)

NXD Sample:

```nxd
MODULE TEST

fn MAIN()
    CONST A SET 10
    CONST B SET 3
    CONST C SET 0

    IF A MOD B NEQ C OR C EQ 0:
        PRINTLN("MOD_OR_PASS")
```

Result:

FAIL

Expected Output:

FAIL - Malformed lambda expression

Expected Rejection Stage:

[ ] Lexer
[X] Parser
[ ] AST
[ ] IR
[X] Semantics
[ ] Backend

Actual Output:

N/A

Notes:

intentional Parser failure. no IR JSON generated. Parser entered lambda parsing mode (because "fn" is used to start a lambda expression)and expected an opening parenthesis.
Encountered identifier "MAIN" instead. 
NXD has 3 function calls: "FUNC" = keyword, "FN" = ordinary identifier/name, and "fn" = lambda/mathematics construct.
---


### ST126 (micro failure)

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

PASS

Expected Output:

FAIL - 

Expected Rejection Stage:

[ ] Lexer
[ ] Parser
[ ] AST
[ ] IR
[ ] Semantics
[ ] Backend

Actual Output:

N/A

Notes:

intentional Parser failure. no IR JSON generated - incorrect indentation

---


### PT127 (micro failure)

NXD Sample:

```nxd
MODULE TEST

FUNC MAIN()
    CONST READY SET false

    IF NOT READY:
        RETURN null
```

Result:

FAIL

Expected Output:

FAIL - Illegal litteral

Expected Rejection Stage:

[ ] Lexer
[X] Parser
[ ] AST
[ ] IR
[X] Semantics
[ ] Backend

Actual Output:

N/A

Notes:

intentional Parser failure. no IR JSON generated. Valid NXD literal "none" was replaced with unsupported token "null".
Lexer classified "null" as LOWNAME.
Parser rejected token while parsing RETURN expression. NXD doesnt use the word "null" instead it uses the word "none".

---


### PT130 Intentional Failure

NXD Sample:

```nxd
MODULE TEST:

FUNC MAIN()
    CONST COUNT SET 1

    LOOP:
        PRINTLN(COUNT ADD 1)
```

Result:

FAIL

Expected Output:

FAIL - Invalid expressiopn syntax

Expected Rejection Stage:

[ ] Lexer
[X] Parser
[ ] AST
[ ] IR
[ ] Semantics
[ ] Backend

Actual Output:

N/A

Notes:

intentional Parser failure. no IR JSON generated. Parser encountered a colon while parsing a primary expression.
A colon is not a valid primary expression token in this context.

---


### PT131 Intentional Failure

NXD Sample:

```nxd
MODULE TEST

FUNC MAIN() {
    CONST STATE SET "READY"

    MATCH STATE:
        CASE "READY":
            PRINTLN("GO")

        CASE "WAIT":
            PRINTLN("HOLD")

        OTHERWISE:
            PRINTLN("UNKNOWN")
}
```

Result:

FAIL

Expected Output:

FAIL - Malformed map litteral

Expected Rejection Stage:

[ ] Lexer
[X] Parser
[ ] AST
[ ] IR
[ ] Semantics
[ ] Backend

Actual Output:

N/A

Notes:

intentional Parser failure. no IR JSON generated. Parser entered map literal parsing and expected a valid map key.
Encountered CONST keyword instead.

---


### ST133 (micro failure)

NXD Sample:

```nxd
MODULE TEST

TYPE PERSON STRUCT:
    NAME: string,
    AGE: int

FUNC MAIN()
    PRINTLN("STRUCT_DECLARED")
```

Result:

PASS

Expected Output:

FAIL -

Expected Rejection Stage:

[ ] Lexer
[ ] Parser
[ ] AST
[ ] IR
[ ] Semantics
[ ] Backend

Actual Output:



Notes:



---

### PT134 (micro failure)

NXD Sample:

```nxd
MODULE TEST

TYPE STATUS ENUM
    READY
    WAITING
    FAILED

FUNC MAIN()
    PRINTLN("ENUM_DECLARED")
```

Result:

FAIL - Parser

Expected Rejection Stage:

[ ] Lexer
[ ] Parser
[ ] AST
[ ] IR
[ ] Semantics
[ ] Backend

Actual Output:

N/A

Notes:

intentional Parser failure. no IR JSON generated. Parser expected a colon in the ENUM definition but encountered a newline instead.

---


### PT137 Intentional Failure

NXD Sample:

```nxd
MODULE TEST

FUNC MAIN()
    CONST VALUE LET 10

    PRINTLN(CLONE VALUE)
    PRINTLN(BORROW VALUE)
    PRINTLN(MOVE VALUE)

Result:

FAIL - Parser

Expected Output:

FAIL - Malformed constant declaration

Expected Rejection Stage:
[ ] Lexer
[X] Parser
[ ] AST
[ ] IR
[ ] Semantics
[ ] Backend

Actual Output:

N/A

Notes:

Intentional parser failure. No IR JSON created. Parser expected SET keyword while processing CONST statement.
Encountered LET keyword instead.

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


