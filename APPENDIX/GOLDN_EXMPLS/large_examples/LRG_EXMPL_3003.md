# KEY-VALUE DATABASE (MAPS + PERSISTANCE SIMULATION + RESULT + OPTION)

### JSON-LD Metadata
```jsonld
{
  "@context": "https://schema.nxd.dev/example",
  "example_id": "nxd_large_03_kv_database",
  "category": "Storage",
  "layer": "large",
  "features": ["maps", "result", "option", "modules", "mutation"]
}
```


### NXD
```nxd
MODULE db.core

TYPE OPTION UNION { SOME(any), NONE }
TYPE RESULT UNION { OK(any), ERR(string) }

LET STORE SET MAP<string, string> {}

FUNC PUT(KEY: string, VAL: string): RESULT:
    STORE[KEY] SET VAL
    RETURN OK("stored")

FUNC GET(KEY: string): OPTION:
    IF STORE HAS KEY:
        RETURN SOME(STORE[KEY])
    RETURN NONE

FUNC DELETE(KEY: string): RESULT:
    IF STORE HAS KEY:
        REMOVE STORE[KEY]
        RETURN OK("deleted")
    RETURN ERR("missing key")


MODULE db.admin
IMPORT db.core

FUNC DUMP():
    LOOP K IN KEYS(STORE):
        PRINTLN(K ADD "=" ADD STORE[K])
    RETURN NONE


MODULE app.main
IMPORT db.core
IMPORT db.admin

FUNC MAIN():
    PRINTLN(PUT("host", "localhost"))
    PRINTLN(PUT("port", "8080"))

    MATCH GET("host"):
        CASE SOME(V): PRINTLN("host=" ADD V)
        CASE NONE: PRINTLN("missing host")

    PRINTLN(DELETE("port"))
    PRINTLN(DELETE("port"))

    DUMP()

    RETURN NONE
```