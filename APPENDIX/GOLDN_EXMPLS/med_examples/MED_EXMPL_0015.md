# RESULT +OPTION COMPOSITION (SAFE LOOKUP + PARSE)

### JSON-LD Metadata
```jsonld
{
  "@context": "https://schema.nxd.dev/example",
  "example_id": "nxd_medium_05_result_option_composition",
  "category": "ErrorHandling",
  "layer": "medium",
  "features": ["result", "option", "match", "composition"]
}
```

### NXD
```nxd
MODULE error.composition

TYPE OPTION UNION { SOME(any), NONE }
TYPE RESULT UNION { OK(any), ERR(string) }

LET CONFIG SET MAP<string, string> {
    "port": "8080",
    "host": "localhost"
}

FUNC GET_CONFIG(KEY: string): OPTION:
    IF CONFIG HAS KEY:
        RETURN SOME(CONFIG[KEY])
    RETURN NONE

FUNC PARSE_INT(S: string): RESULT:
    IF S CONTAINS NON_DIGIT:
        RETURN ERR("not numeric")
    RETURN OK(S AS int)

FUNC GET_PORT(): RESULT:
    MATCH GET_CONFIG("port"):
        CASE SOME(V):
            RETURN PARSE_INT(V)
        CASE NONE:
            RETURN ERR("missing port")

FUNC MAIN():
    LET R SET GET_PORT()
    MATCH R:
        CASE OK(P):
            PRINTLN("port: " ADD P)
        CASE ERR(E):
            PRINTLN("error: " ADD E)
    RETURN NONE
```