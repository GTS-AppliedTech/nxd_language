 ## Distributed Security Event Correlator  (events, rules, correlation windows, channels, tasks, multi‑module, supervision)

### JSON‑LD Metadata
```jsonld
{
  "@context": "https://schema.nxd.dev/example",
  "example_id": "nxd_xl_04_security_correlator",
  "category": "Security",
  "layer": "xl",
  "features": [
    "multi-module",
    "event-streams",
    "correlation-windows",
    "channels",
    "tasks",
    "result",
    "option",
    "supervision",
    "rule-engine"
  ]
}
```


# Canonical NXD (XL‑Layer)

```nxd
MODULE sec.types

TYPE RESULT UNION { OK(any), ERR(string) }
TYPE OPTION UNION { SOME(any), NONE }

TYPE EVENT {
    SRC: string,
    TYPE: string,
    DATA: string,
    TS: int
}

TYPE ALERT {
    NAME: string,
    SEVERITY: string,
    MESSAGE: string,
    TS: int
}

TYPE CHANNEL<EVENT> { }
TYPE CHANNEL<ALERT> { }


MODULE sec.window

# correlation window stores recent events
TYPE WINDOW { EVENTS: LIST<EVENT> }

FUNC NEW_WINDOW(): WINDOW:
    RETURN WINDOW { EVENTS: [] }

FUNC ADD(W: WINDOW, E: EVENT):
    PUSH W.EVENTS, E

FUNC TRIM(W: WINDOW, AGE: int):
    LET NOW SET NOW()
    LET NEW SET []
    LOOP X IN W.EVENTS:
        IF NOW SUB X.TS LT AGE:
            PUSH NEW, X
    W.EVENTS SET NEW

FUNC FIND(W: WINDOW, TYPE: string): LIST<EVENT>:
    LET OUT SET []
    LOOP X IN W.EVENTS:
        IF X.TYPE EQ TYPE:
            PUSH OUT, X
    RETURN OUT


MODULE sec.rules
IMPORT sec.types
IMPORT sec.window

# rule: brute force detection
FUNC RULE_BRUTE(W: WINDOW): OPTION:
    LET FAILS SET FIND(W, "auth_fail")
    IF LEN(FAILS) GT 5:
        RETURN SOME(ALERT {
            NAME: "bruteforce",
            SEVERITY: "high",
            MESSAGE: "multiple auth failures",
            TS: NOW()
        })
    RETURN NONE

# rule: privilege escalation detection
FUNC RULE_PRIVESC(W: WINDOW): OPTION:
    LET ROOT SET FIND(W, "root_access")
    LET FAILS SET FIND(W, "auth_fail")
    IF LEN(ROOT) GT 0 AND LEN(FAILS) GT 3:
        RETURN SOME(ALERT {
            NAME: "privesc",
            SEVERITY: "critical",
            MESSAGE: "root access after multiple failures",
            TS: NOW()
        })
    RETURN NONE

# rule: suspicious system changes
FUNC RULE_SYSCHANGE(W: WINDOW): OPTION:
    LET CH SET FIND(W, "sys_change")
    IF LEN(CH) GT 2:
        RETURN SOME(ALERT {
            NAME: "syschange",
            SEVERITY: "medium",
            MESSAGE: "multiple system changes detected",
            TS: NOW()
        })
    RETURN NONE


MODULE sec.engine
IMPORT sec.types
IMPORT sec.window
IMPORT sec.rules

FUNC EVAL_RULES(W: WINDOW): LIST<ALERT>:
    LET OUT SET []

    LET R1 SET RULE_BRUTE(W)
    MATCH R1:
        CASE SOME(A): PUSH OUT, A
        CASE NONE: NONE

    LET R2 SET RULE_PRIVESC(W)
    MATCH R2:
        CASE SOME(A): PUSH OUT, A
        CASE NONE: NONE

    LET R3 SET RULE_SYSCHANGE(W)
    MATCH R3:
        CASE SOME(A): PUSH OUT, A
        CASE NONE: NONE

    RETURN OUT


MODULE sec.supervisor
IMPORT sec.types
IMPORT sec.window
IMPORT sec.engine

FUNC SUPERVISE(IN: CHANNEL<EVENT>, OUT: CHANNEL<ALERT>):
    LET W SET NEW_WINDOW()

    LOOP:
        LET E SET RECV IN
        ADD(W, E)
        TRIM(W, 5000)  # keep last 5 seconds

        LET ALERTS SET EVAL_RULES(W)
        LOOP A IN ALERTS:
            SEND A TO OUT


MODULE sec.ingest
IMPORT sec.types

FUNC EMIT(IN: CHANNEL<EVENT>, SRC: string, TYPE: string, DATA: string):
    LET E SET EVENT {
        SRC: SRC,
        TYPE: TYPE,
        DATA: DATA,
        TS: NOW()
    }
    SEND E TO IN


MODULE sec.sink
IMPORT sec.types

FUNC PRINT_ALERTS(CH: CHANNEL<ALERT>):
    LOOP:
        LET A SET RECV CH
        PRINTLN("[ALERT] " ADD A.NAME ADD " (" ADD A.SEVERITY ADD ") " ADD A.MESSAGE)


MODULE app.main
IMPORT sec.types
IMPORT sec.ingest
IMPORT sec.supervisor
IMPORT sec.sink

FUNC MAIN():
    LET IN SET CHANNEL<EVENT>()
    LET OUT SET CHANNEL<ALERT>()

    SPAWN SUPERVISE(IN, OUT)
    SPAWN PRINT_ALERTS(OUT)

    # simulate events
    EMIT(IN, "auth", "auth_fail", "bad password")
    EMIT(IN, "auth", "auth_fail", "bad password")
    EMIT(IN, "auth", "auth_fail", "bad password")
    EMIT(IN, "auth", "auth_fail", "bad password")
    EMIT(IN, "auth", "auth_fail", "bad password")
    EMIT(IN, "auth", "auth_fail", "bad password")

    EMIT(IN, "system", "sys_change", "config modified")
    EMIT(IN, "system", "sys_change", "config modified")
    EMIT(IN, "system", "sys_change", "config modified")

    EMIT(IN, "auth", "root_access", "sudo success")

    SLEEP(2)

    RETURN NONE
```



# What this XL example demonstrates

### Multi‑module security architecture  
- `sec.types`  
- `sec.window`  
- `sec.rules`  
- `sec.engine`  
- `sec.supervisor`  
- `sec.ingest`  
- `sec.sink`  
- `app.main`

### Correlation windows  
- Rolling event windows  
- Time‑based trimming  
- Event type filtering

### Rule engine  
- Brute force detection  
- Privilege escalation correlation  
- System change anomaly detection  
- Multi‑rule evaluation  
- Multiple alerts per event batch

### Channels + processes  
- Ingest → Supervisor → Engine → Sink  
- Fully asynchronous  
- Real distributed‑system semantics

### Result + Option  
- Optional rule matches  
- Pattern matching  
- Safe evaluation

### Realistic security event correlator  
- Auth failures  
- Root access  
- System changes  
- Correlated alerts  
- Severity levels  
- Multi‑sink output

This is a **full subsystem**, suitable for real NXD agent training and backend mapping.

