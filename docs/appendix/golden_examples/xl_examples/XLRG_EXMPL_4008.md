## Distributed Access Control Engine  (policy evaluation, roles, capabilities, channels, multi‑module, supervision)

### JSON‑LD Metadata
```jsonld
{
  "@context": "https://schema.nxd.dev/example",
  "example_id": "nxd_xl_08_access_control_engine",
  "category": "Security",
  "layer": "xl",
  "features": [
    "multi-module",
    "rbac",
    "abac",
    "capabilities",
    "policy-evaluation",
    "channels",
    "processes",
    "tasks",
    "result",
    "option",
    "supervision"
  ]
}
```



# Canonical NXD (XL‑Layer)

```nxd
MODULE access.types

TYPE RESULT UNION { OK(any), ERR(string) }
TYPE OPTION UNION { SOME(any), NONE }

TYPE ROLE { NAME: string }
TYPE CAP { NAME: string }

TYPE USER {
    NAME: string,
    ROLES: LIST<ROLE>,
    CAPS: LIST<CAP>
}

TYPE RESOURCE {
    NAME: string,
    OWNER: string,
    SENSITIVITY: string
}

TYPE REQUEST {
    USER: USER,
    ACTION: string,
    RES: RESOURCE,
    TS: int
}

TYPE DECISION {
    ALLOW: bool,
    REASON: string,
    TS: int
}

TYPE CHANNEL<REQUEST> { }
TYPE CHANNEL<DECISION> { }
```



## MODULE: Role Registry (RBAC)

```nxd
MODULE access.roles
IMPORT access.types

LET ROLE_DB SET MAP<string, LIST<CAP>> {}

FUNC DEFINE_ROLE(NAME: string, CAPS: LIST<CAP]):
    ROLE_DB[NAME] SET CAPS

FUNC GET_CAPS(R: ROLE): OPTION:
    IF ROLE_DB HAS R.NAME:
        RETURN SOME(ROLE_DB[R.NAME])
    RETURN NONE
```



## MODULE: Capability Engine (ABAC)

```nxd
MODULE access.capabilities
IMPORT access.types

FUNC HAS_CAP(U: USER, NAME: string): bool:
    LOOP C IN U.CAPS:
        IF C.NAME EQ NAME:
            RETURN true
    RETURN false

FUNC HAS_ROLE_CAP(U: USER, NAME: string): bool:
    LOOP R IN U.ROLES:
        MATCH GET_CAPS(R):
            CASE SOME(CLIST):
                LOOP C IN CLIST:
                    IF C.NAME EQ NAME:
                        RETURN true
            CASE NONE:
                NONE
    RETURN false

FUNC CHECK_CAP(U: USER, NAME: string): bool:
    RETURN HAS_CAP(U, NAME) OR HAS_ROLE_CAP(U, NAME)
```



## MODULE: Policy Engine

```nxd
MODULE access.policy
IMPORT access.types
IMPORT access.capabilities

FUNC EVAL_POLICY(REQ: REQUEST): DECISION:
    LET U SET REQ.USER
    LET R SET REQ.RES
    LET A SET REQ.ACTION

    # owner always allowed
    IF U.NAME EQ R.OWNER:
        RETURN DECISION { ALLOW: true, REASON: "owner", TS: NOW() }

    # sensitivity rules
    IF R.SENSITIVITY EQ "high":
        IF NOT CHECK_CAP(U, "access_high"):
            RETURN DECISION { ALLOW: false, REASON: "missing cap: access_high", TS: NOW() }

    IF R.SENSITIVITY EQ "medium":
        IF NOT CHECK_CAP(U, "access_medium"):
            RETURN DECISION { ALLOW: false, REASON: "missing cap: access_medium", TS: NOW() }

    # action rules
    IF A EQ "delete":
        IF NOT CHECK_CAP(U, "delete"):
            RETURN DECISION { ALLOW: false, REASON: "missing cap: delete", TS: NOW() }

    IF A EQ "write":
        IF NOT CHECK_CAP(U, "write"):
            RETURN DECISION { ALLOW: false, REASON: "missing cap: write", TS: NOW() }

    # default allow
    RETURN DECISION { ALLOW: true, REASON: "default allow", TS: NOW() }
```



## MODULE: Supervisor

```nxd
MODULE access.supervisor
IMPORT access.types
IMPORT access.policy

FUNC SUPERVISE(IN: CHANNEL<REQUEST>, OUT: CHANNEL<DECISION>):
    LOOP:
        LET REQ SET RECV IN
        LET DEC SET EVAL_POLICY(REQ)
        SEND DEC TO OUT
```



## MODULE: Client

```nxd
MODULE access.client
IMPORT access.types

FUNC MAKE_REQ(U: USER, ACTION: string, RES: RESOURCE): REQUEST:
    RETURN REQUEST {
        USER: U,
        ACTION: ACTION,
        RES: RES,
        TS: NOW()
    }

FUNC SEND_REQ(IN: CHANNEL<REQUEST>, U: USER, ACTION: string, RES: RESOURCE):
    LET R SET MAKE_REQ(U, ACTION, RES)
    SEND R TO IN
```



## MODULE: Sink

```nxd
MODULE access.sink
IMPORT access.types

FUNC PRINT_DECISIONS(CH: CHANNEL<DECISION>):
    LOOP:
        LET D SET RECV CH
        IF D.ALLOW:
            PRINTLN("[ALLOW] " ADD D.REASON)
        OTHERWISE:
            PRINTLN("[DENY] " ADD D.REASON)
```



## MODULE: App Main

```nxd
MODULE app.main
IMPORT access.types
IMPORT access.roles
IMPORT access.capabilities
IMPORT access.policy
IMPORT access.supervisor
IMPORT access.client
IMPORT access.sink

FUNC MAIN():
    LET IN SET CHANNEL<REQUEST>()
    LET OUT SET CHANNEL<DECISION>()

    SPAWN SUPERVISE(IN, OUT)
    SPAWN PRINT_DECISIONS(OUT)

    # define roles
    DEFINE_ROLE("admin", [CAP { NAME: "delete" }, CAP { NAME: "write" }, CAP { NAME: "access_high" }])
    DEFINE_ROLE("editor", [CAP { NAME: "write" }, CAP { NAME: "access_medium" }])
    DEFINE_ROLE("viewer", [CAP { NAME: "access_medium" }])

    # users
    LET U1 SET USER { NAME: "gabriel", ROLES: [ROLE { NAME: "admin" }], CAPS: [] }
    LET U2 SET USER { NAME: "alex", ROLES: [ROLE { NAME: "viewer" }], CAPS: [] }
    LET U3 SET USER { NAME: "sam", ROLES: [], CAPS: [CAP { NAME: "write" }] }

    # resources
    LET R1 SET RESOURCE { NAME: "config.yaml", OWNER: "gabriel", SENSITIVITY: "high" }
    LET R2 SET RESOURCE { NAME: "notes.txt", OWNER: "alex", SENSITIVITY: "medium" }

    # requests
    SEND_REQ(IN, U1, "delete", R1)
    SEND_REQ(IN, U2, "write", R2)
    SEND_REQ(IN, U3, "write", R1)
    SEND_REQ(IN, U2, "delete", R1)

    SLEEP(2)

    RETURN NONE
```



# What this XL example demonstrates

### Multi‑module access control architecture  
- RBAC (roles → capabilities)  
- ABAC (user attributes → capabilities)  
- Policy engine  
- Supervisor  
- Client  
- Sink  
- Full request/decision pipeline

### Capability evaluation  
- Direct user capabilities  
- Role‑derived capabilities  
- Combined evaluation

### Policy rules  
- Owner override  
- Sensitivity levels  
- Action‑based permissions  
- Default allow fallback

### Channels + processes  
- Client → Supervisor → Policy Engine → Sink  
- Fully asynchronous  
- Real distributed‑system semantics

### Result + Option  
- Optional capability lookup  
- Safe evaluation  
- Pattern matching

### Realistic access control engine  
- Admin, editor, viewer roles  
- High/medium sensitivity resources  
- Delete/write actions  
- Multi‑user, multi‑resource evaluation  
- Deterministic decisions

This is a **full subsystem**, suitable for real NXD agent training and backend mapping.

