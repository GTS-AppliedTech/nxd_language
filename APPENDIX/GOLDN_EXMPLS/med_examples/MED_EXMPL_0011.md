# USER SERVICE (CRUD + RESULT + OPTION)

{
  "@context": "https://schema.nxd.dev/example",
  "example_id": "nxd_medium_01_user_service",
  "category": "CRUD",
  "layer": "medium",
  "features": ["structs", "maps", "option", "result", "error-handling"]
}

### NXD
```nxd
MODULE service.user

TYPE USER { ID: int, NAME: string }

TYPE OPTION UNION { SOME(any), NONE }
TYPE RESULT UNION { OK(any), ERR(string) }

LET USERS SET MAP<int, USER> {}

FUNC CREATE_USER(ID: int, NAME: string): RESULT:
    IF USERS HAS ID:
        RETURN ERR("user exists")
    LET U SET USER { ID: ID, NAME: NAME }
    USERS[ID] SET U
    RETURN OK(U)

FUNC GET_USER(ID: int): OPTION:
    IF USERS HAS ID:
        RETURN SOME(USERS[ID])
    RETURN NONE

FUNC UPDATE_USER_NAME(ID: int, NAME: string): RESULT:
    MATCH GET_USER(ID):
        CASE SOME(U):
            LET NEW SET USER { ID: U.ID, NAME: NAME }
            USERS[ID] SET NEW
            RETURN OK(NEW)
        CASE NONE:
            RETURN ERR("not found")

FUNC DELETE_USER(ID: int): RESULT:
    IF USERS HAS ID:
        REMOVE USERS[ID]
        RETURN OK("deleted")
    RETURN ERR("not found")

FUNC MAIN():
    PRINTLN(CREATE_USER(1, "gabriel"))
    PRINTLN(UPDATE_USER_NAME(1, "gts"))
    PRINTLN(GET_USER(1))
    PRINTLN(DELETE_USER(1))
    PRINTLN(GET_USER(1))
    RETURN NONE
```