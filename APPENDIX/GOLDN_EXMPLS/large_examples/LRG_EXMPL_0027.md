# CONFIGURATION LOADER (MODULES + INIT + RESULT + OPTION + PARSING)

### JSON-LD Metadata
```jsonld
{
  "@context": "https://schema.nxd.dev/example",
  "example_id": "nxd_large_07_config_loader",
  "category": "Runtime",
  "layer": "large",
  "features": ["init", "modules", "result", "option", "parsing"]
}
```


### NXD
```nxd
MODULE config.core

TYPE OPTION UNION { SOME(any), NONE }
TYPE RESULT UNION { OK(any), ERR(string) }

LET CFG SET MAP<string, string> {}

INIT:
    CFG["host"] SET "localhost"
    CFG["port"] SET "8080"

FUNC GET(KEY: string): OPTION:
    IF CFG HAS KEY:
        RETURN SOME(CFG[KEY])
    RETURN NONE

FUNC PARSE_INT(S: string): RESULT:
    IF S CONTAINS NON_DIGIT:
        RETURN ERR("not numeric")
    RETURN OK(S AS int)


MODULE config.service
IMPORT config.core

FUNC LOAD_PORT(): RESULT:
    MATCH GET("port"):
        CASE SOME(V): RETURN PARSE_INT(V)
        CASE NONE: RETURN ERR("missing port")


MODULE app.main
IMPORT config.service

FUNC MAIN():
    LET R SET LOAD_PORT()
    PRINTLN(R)
    RETURN NONE
```