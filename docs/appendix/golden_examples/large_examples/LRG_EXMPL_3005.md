# COMPILER LINTER PLUGIN (AST HOOKS + DIAGNOSTICS + MULTI-RULE)

### JSON-LD Metadata
```jsonld
{
  "@context": "https://schema.nxd.dev/example",
  "example_id": "nxd_large_05_linter",
  "category": "CompilerPlugin",
  "layer": "large",
  "features": ["plugin", "ast", "diagnostics", "rules"]
}
```


### NXD
```nxd
MODULE lint.rules

PLUGIN LINTER:
    ON AST(VAR_DECL):
        IF DECL.NAME EQ "tmp":
            DIAGNOSTIC W2001 "avoid tmp variable names"

    ON AST(FUNC_CALL):
        IF CALL.NAME EQ "DEBUG":
            DIAGNOSTIC W3001 "debug call in production"

    ON AST(BINARY_OP):
        IF OP.NAME EQ "DIV" AND OP.RIGHT EQ 0:
            DIAGNOSTIC E8001 "division by zero"
    END


MODULE app.main
IMPORT lint.rules

FUNC MAIN():
    LET tmp SET 10   # warning
    LET A SET 10 DIV 0   # error
    DEBUG("test")    # warning
    RETURN NONE
```