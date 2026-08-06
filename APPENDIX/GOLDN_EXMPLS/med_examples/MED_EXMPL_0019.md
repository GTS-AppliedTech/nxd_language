# COMPILER PLUGIN: ENFORCE EXPLICIT AS ON NUMERIC OPS

### JSON-LD Metadata
```jsonld
{
  "@context": "https://schema.nxd.dev/example",
  "example_id": "nxd_medium_09_plugin_cast_policy",
  "category": "CompilerPlugin",
  "layer": "medium",
  "features": ["plugin", "casts", "diagnostics"]
}
```


### NXD
```nxd
MODULE plugin.cast_policy

PLUGIN REQUIRE_EXPLICIT_CAST:
    ON AST(BINARY_OP):
        IF OP.NAME EQ "ADD" OR OP.NAME EQ "SUB":
            IF OP.LEFT.TYPE NEQ OP.RIGHT.TYPE:
                DIAGNOSTIC E7002 "numeric ops require explicit AS cast"
    END

FUNC CALC():
    LET A SET 10
    LET B SET 3.5
    LET C SET A ADD B AS int   # legal with explicit AS
    RETURN C

FUNC MAIN():
    RETURN CALC()
```