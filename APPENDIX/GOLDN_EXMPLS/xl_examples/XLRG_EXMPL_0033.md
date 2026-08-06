## Distributed Logging Mesh (Routers, Filters, Sinks, Channels, Supervision, Multi‑Module)

### JSON‑LD Metadata
```jsonld
{
  "@context": "https://schema.nxd.dev/example",
  "example_id": "nxd_xl_03_logging_mesh",
  "category": "Logging",
  "layer": "xl",
  "features": [
    "multi-module",
    "routing",
    "filters",
    "channels",
    "processes",
    "supervision",
    "result",
    "option",
    "init"
  ]
}
```



# Canonical NXD (XL‑Layer)

```nxd
MODULE log.types

TYPE RESULT UNION { OK(any), ERR(string) }
TYPE OPTION UNION { SOME(any), NONE }

TYPE LOG {
    LEVEL: string,
    SOURCE: string,
    MESSAGE: string,
    TS: int
}

TYPE CHANNEL<LOG> { }
TYPE CHANNEL<string> { }


MODULE log.filter

FUNC MATCH_LEVEL(L: LOG, LVL: string): bool:
    RETURN L.LEVEL EQ LVL

FUNC MATCH_SOURCE(L: LOG, SRC: string): bool:
    RETURN L.SOURCE EQ SRC

FUNC MATCH_CONTAINS(L: LOG, SUB: string): bool:
    RETURN L.MESSAGE CONTAINS SUB


MODULE log.router
IMPORT log.types
IMPORT log.filter

TYPE ROUTE {
    NAME: string,
    LEVEL: OPTION,
    SOURCE: OPTION,
    CONTAINS: OPTION,
    OUT: CHANNEL<LOG>
}

LET ROUTES SET LIST<ROUTE> []

FUNC ADD_ROUTE(NAME: string, LVL: OPTION, SRC: OPTION, SUB: OPTION, OUT: CHANNEL<LOG>):
    LET R SET ROUTE { NAME: NAME, LEVEL: LVL, SOURCE: SRC, CONTAINS: SUB, OUT: OUT }
    PUSH ROUTES, R

FUNC ROUTE_LOG(L: LOG):
    LOOP R IN ROUTES:
        LET PASS SET true

        MATCH R.LEVEL:
            CASE SOME(LVL):
                IF NOT MATCH_LEVEL(L, LVL):
                    PASS SET false
            CASE NONE:
                PASS SET PASS

        MATCH R.SOURCE:
            CASE SOME(SRC):
                IF NOT MATCH_SOURCE(L, SRC):
                    PASS SET false
            CASE NONE:
                PASS SET PASS

        MATCH R.CONTAINS:
            CASE SOME(SUB):
                IF NOT MATCH_CONTAINS(L, SUB):
                    PASS SET false
            CASE NONE:
                PASS SET PASS

        IF PASS:
            SEND L TO R.OUT


MODULE log.sink
IMPORT log.types

FUNC PRINT_SINK(CH: CHANNEL<LOG>, NAME: string):
    LOOP:
        LET L SET RECV CH
        PRINTLN("[" ADD NAME ADD "] " ADD L.LEVEL ADD " " ADD L.SOURCE ADD ": " ADD L.MESSAGE)


MODULE log.supervisor
IMPORT log.types
IMPORT log.router

FUNC SUPERVISE(IN: CHANNEL<LOG>):
    LOOP:
        LET L SET RECV IN
        ROUTE_LOG(L)


MODULE log.ingest
IMPORT log.types

FUNC EMIT(IN: CHANNEL<LOG>, LEVEL: string, SRC: string, MSG: string):
    LET TS SET NOW()
    LET L SET LOG { LEVEL: LEVEL, SOURCE: SRC, MESSAGE: MSG, TS: TS }
    SEND L TO IN


MODULE app.main
IMPORT log.types
IMPORT log.router
IMPORT log.sink
IMPORT log.supervisor
IMPORT log.ingest

FUNC MAIN():
    LET IN SET CHANNEL<LOG>()

    # sinks
    LET ERR_CH SET CHANNEL<LOG>()
    LET AUTH_CH SET CHANNEL<LOG>()
    LET SYS_CH SET CHANNEL<LOG>()
    LET ALL_CH SET CHANNEL<LOG>()

    # spawn sinks
    SPAWN PRINT_SINK(ERR_CH, "errors")
    SPAWN PRINT_SINK(AUTH_CH, "auth")
    SPAWN PRINT_SINK(SYS_CH, "system")
    SPAWN PRINT_SINK(ALL_CH, "all")

    # router supervisor
    SPAWN SUPERVISE(IN)

    # routes
    ADD_ROUTE(
        "error-route",
        SOME("error"),
        NONE,
        NONE,
        ERR_CH
    )

    ADD_ROUTE(
        "auth-route",
        NONE,
        SOME("auth"),
        NONE,
        AUTH_CH
    )

    ADD_ROUTE(
        "system-route",
        NONE,
        SOME("system"),
        SOME("critical"),
        SYS_CH
    )

    ADD_ROUTE(
        "all-route",
        NONE,
        NONE,
        NONE,
        ALL_CH
    )

    # emit logs
    EMIT(IN, "info", "system", "startup complete")
    EMIT(IN, "error", "system", "disk failure")
    EMIT(IN, "info", "auth", "login success")
    EMIT(IN, "warn", "auth", "multiple login attempts")
    EMIT(IN, "info", "system", "critical temperature spike")

    # allow routing to complete
    SLEEP(2)

    RETURN NONE
```



# What this XL example demonstrates

### Multi‑module distributed logging architecture  
- `log.types`  
- `log.filter`  
- `log.router`  
- `log.sink`  
- `log.supervisor`  
- `log.ingest`  
- `app.main`

### Dynamic routing rules  
- Level‑based  
- Source‑based  
- Substring‑based  
- Combined filters  
- Multiple sinks per log

### Channels + processes  
- Ingest → Supervisor → Router → Sinks  
- Fully asynchronous  
- Fan‑out routing

### Result + Option  
- Optional filters  
- Pattern matching  
- Safe routing logic

### Realistic distributed logging mesh  
- Error sink  
- Auth sink  
- System sink  
- Catch‑all sink  
- Supervisor process  
- Multiple concurrent sinks

This is a **full subsystem**, suitable for real NXD agent training and backend mapping.

