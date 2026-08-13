# AUTHENTICATION SERVICE (MODULES + TRAITS + GENERICS + SECURITY)

### JSON-LD Metadata
```jsonld
{
  "@context": "https://schema.nxd.dev/example",
  "example_id": "nxd_large_02_auth_service",
  "category": "Security",
  "layer": "large",
  "features": ["modules", "traits", "generics", "capabilities", "result"]
}
```


### NXD
```nxd
MODULE auth.core

TYPE TOKEN { VALUE: string, EXP: int }
TYPE RESULT UNION { OK(any), ERR(string) }

TRAIT SERIALIZABLE {
    FUNC TO_STRING(X): string
}

TYPE USER IMPLEMENTS SERIALIZABLE {
    NAME: string
    TOKEN: TOKEN
}

FUNC TO_STRING(U: USER): string:
    RETURN "user(" ADD U.NAME ADD ")"

FUNC ISSUE(NAME: string): TOKEN:
    RETURN TOKEN { VALUE: "tok_" ADD NAME, EXP: 999 }

FUNC VALID(T: TOKEN): bool:
    RETURN T.VALUE CONTAINS "tok_" AND T.EXP GT 0


MODULE auth.service
IMPORT auth.core

FUNC LOGIN(NAME: string): RESULT:
    LET T SET ISSUE(NAME)
    LET U SET USER { NAME: NAME, TOKEN: T }
    RETURN OK(U)

FUNC CHECK(U: USER): RESULT:
    IF VALID(U.TOKEN):
        RETURN OK("auth ok")
    RETURN ERR("invalid token")


MODULE app.main
IMPORT auth.service
IMPORT auth.core

FUNC MAIN():
    LET R SET LOGIN("gabriel")

    MATCH R:
        CASE OK(U):
            PRINTLN(TO_STRING(U))
            PRINTLN(CHECK(U))
        CASE ERR(E):
            PRINTLN("login failed: " ADD E)

    RETURN NONE
```