---
{
  "@context": "https://nxdlang.org/schema",
  "doc_id": "CP004",
  "title": "CP004 Compiler Diagnostics",
  "description": "LSP and compiler diagnostic code library",
  "layer": "compiler",
  "category": "compiler",
  "keywords": ["diagnostics", "warnings", "errors", "codes"],
  "doc_version": "1.0",
  "status": "active"
}
---

# CP004 COMPILER DIAGNOSTICS - CODES & SUBCODES

### **COMPILER SPECIFIC ERROR CODES**

- NXD-LSP0001 Rust semantic compiler has not been built
- NXD-LSP0002 Rust semantic validation failed
- NXD-LSP0003 Invalid semantic diagnostic response: {error}


### **PARSER CODES**

##### *ERROR CODES:*

- NXD-P1001 Expected {kind}, got {}
- NXD-P1002 Expected {val}, got {}
- NXD-P1003 Expected 'STRUCT', 'ENUM', 'UNION', or 'TRAIT', got {}
- NXD-P1004 Expected 'CLONE', got {}
- NXD-P1005 Expected source identifier after 'CLONE', got {} 
- NXD-P1006 Expected 'TO' after 'CLONE' source, got {}
- NXD-P1007 Expected target identifier after 'TO', got {}
- NXD-P1008 Expected token in primary
- NXD-P1009-01  Literal expected, got {}
- NXD-P1009-02  Invalid integer literal: {} 
- NXD-P1009-03  Invalid float literal: {} | Float literal exceeds finite range: {}
- NXD-P1009-04  Invalid list literal, expected ',' or `'}'`, got {} 
- NXD-P1009-05  Invalid map literal, expected ',' or `'}'`, got {} 
- NXD-P1009-06  Invalid lambda literal, expected ',' or `')'`, got {} 
- NXD-P1009-07  Invalid string literal: {} **(Reserved)**
- NXD-P1009-08  Invalid boolean literal: {} **(Reserved)**
- NXD-P1009-09  Invalid none literal: {} **(Reserved)**
- NXD-P1010 Expected map key, got {}
- NXD-P1011 Expected 'TO' after 'MOVE' source, got {}
- NXD-P1012 Expected type after 'AS/IS', got {}
- NXD-P1013 Function name must be immediately followed by `'('`
- NXD-P1014 Expected an indented block
- NXD-P1015 Unexpected indentation

### *WARNING CODES:*

- NXD-W2001 Variable '<name>' declared but never used 
- NXD-W2002 Unreachable code
- NXD-W2003 Variable shadows existing binding
- NXD-W2004 *(RESERVED UNUSED 'IMPORT')*
- NXD-W2005 Parameter '{}' declared but never used
- NXD-W2006 FUNC '{}' declared but never used.
- NXD-W2007 Condition is always '{}'.
- NXD-W2008 Duplicate/redundant MATCH arm '{}'.
- NXD-W2009 Non-exhaustive 'MATCH'.
- NXD-W2010 'CONST' '{}' declared but never used.


### **SEMANTICS CODES**

##### *ERROR CODES:*

- NXD-S3001 Undefined symbol
- NXD-S3002 Type mismatch
- NXD-S3003 Trait not implemented*** *(See note)*
- NXD-S3004 Invalid cast
- NXD-S3005-01 'AWAIT' Outside 'ASYNC' function
- NXD-S3005-02 'AWAIT' non awaitable function
- NXD-S3005-03 Invalid 'SPAWN' target
- NXD-S3005-04 Invalid 'SEND' target
- NXD-S3005-05 Invalid 'RECV' source
- NXD-S3005-06 'ASYNC' Context violation
- NXD-S3005-07 Invalid 'PING' target
- NXD-S3005-08 Invalid 'ESC' target
- NXD-S3005-09 Invalid 'TIMEOUT' target
- NXD-S3005-10 Invalid 'SYNC' target
- NXD-S3005-11 Invalid 'CHANNEL' declaration
- NXD-S3005-12 Invalid 'TASK' target
- NXD-S3006 *(RESERVED FOR 'SYNC' AS NON-ASYNC)*
- NXD-S3007 *(RESERVED FOR 'CHANNEL' AS NON-ASYNC)*
- NXD-S3008 Cannot use '{}' after it has been moved


***Note NXD-S3003: Trait not implemented:
*Trait declarations and implementations are preserved through the AST and IR and are* *available to semantic analysis through the trait registry.*
*Trait conformance lookup infrastructure exists; however, no currently implemented language* *construct requires trait conformance during semantic analysis.*
*This diagnostic remains reserved until a trait requirement, such as generic trait* *constraints, requires enforcement.*

