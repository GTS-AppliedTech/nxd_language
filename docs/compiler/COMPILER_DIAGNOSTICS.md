---
{
  "@context": "https://nxdlang.org/schema",
  "doc_id": "CP004",
  "title": "CP004 Compiler Diagnostics",
  "description": "LSP diagnostic code library",
  "layer": "compiler",
  "category": "compiler",
  "keywords": [diagnostics, warnings, errors, codes],
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
- NXD-P1009 Literal expected, got {}
- NXD-P1010 Expected map key, got {}

##### *WARNING CODES:*

- NXD-W2001 Variable '<name>' declared but never used 
- NXD-W2003 Variable shadows existing binding


### **SEMANTICS CODES**

##### *ERROR CODES:*

- NXD-S3001 Undefined symbol
- NXD-S3002 Type mismatch
- NXD-S3003 Trait not implemented
- NXD-S3004 Invalid cast
- NXD-S3005-01 'AWAIT' Outside 'ASYNC' function
- NXD-S3005-02 'AWAIT' non awaitable function
- NXD-S3005-03 Invalid 'SPAWN' target
- NXD-S3005-04 Invalid 'SEND' target
- NXD-S3005-05 Invalid 'RECV' source
- NXD-S3005-06 'ASYNC' Context violation
- NXD-S3005-07 Invalid 'PING' target

##### *WARNING CODES:*
 
- NXD-W2002 Unreachable code
