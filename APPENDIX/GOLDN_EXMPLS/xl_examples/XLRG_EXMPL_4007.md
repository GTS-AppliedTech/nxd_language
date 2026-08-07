# XL‑Layer Example 7  
## Distributed Storage Replicator  (replication streams, consistency, channels, multi‑module, supervision, conflict resolution)

### JSON‑LD Metadata
```jsonld
{
  "@context": "https://schema.nxd.dev/example",
  "example_id": "nxd_xl_07_storage_replicator",
  "category": "Storage",
  "layer": "xl",
  "features": [
    "multi-module",
    "replication",
    "consistency",
    "channels",
    "processes",
    "tasks",
    "result",
    "option",
    "supervision",
    "conflict-resolution"
  ]
}
```



# Canonical NXD (XL‑Layer)

```nxd
MODULE store.types

TYPE RESULT UNION { OK(any), ERR(string) }
TYPE OPTION UNION { SOME(any), NONE }

TYPE RECORD {
    KEY: string,
    VALUE: string,
    VER: int,
    TS: int
}

TYPE CHANGE {
    SRC: string,
    REC: RECORD
}

TYPE CHANNEL<CHANGE> { }
TYPE CHANNEL<string> { }


MODULE store.local
IMPORT store.types

LET DB SET MAP<string, RECORD> {}

FUNC GET(KEY: string): OPTION:
    IF DB HAS KEY:
        RETURN SOME(DB[KEY])
    RETURN NONE

FUNC PUT(R: RECORD):
    DB[R.KEY] SET R

FUNC VERSION_INC(R: RECORD): RECORD:
    RETURN RECORD {
        KEY: R.KEY,
        VALUE: R.VALUE,
        VER: R.VER ADD 1,
        TS: NOW()
    }

FUNC APPLY_LOCAL(KEY: string, VALUE: string): CHANGE:
    LET TS SET NOW()

    MATCH GET(KEY):
        CASE SOME(OLD):
            LET NEW SET RECORD {
                KEY: KEY,
                VALUE: VALUE,
                VER: OLD.VER ADD 1,
                TS: TS
            }
            PUT(NEW)
            RETURN CHANGE { SRC: "local", REC: NEW }

        CASE NONE:
            LET NEW SET RECORD {
                KEY: KEY,
                VALUE: VALUE,
                VER: 1,
                TS: TS
            }
            PUT(NEW)
            RETURN CHANGE { SRC: "local", REC: NEW }


MODULE store.conflict
IMPORT store.types

FUNC RESOLVE(A: RECORD, B: RECORD): RECORD:
    # last-write-wins with version tie-break
    IF A.VER GT B.VER:
        RETURN A
    IF B.VER GT A.VER:
        RETURN B

    # version equal → timestamp wins
    IF A.TS GT B.TS:
        RETURN A
    RETURN B


MODULE store.replicator
IMPORT store.types
IMPORT store.local
IMPORT store.conflict

FUNC REPLICATE(IN: CHANNEL<CHANGE>, OUT: CHANNEL<CHANGE>, NODE: string):
    LOOP:
        LET C SET RECV IN
        LET R SET C.REC

        MATCH GET(R.KEY):
            CASE SOME(LOCAL):
                LET WIN SET RESOLVE(LOCAL, R)
                PUT(WIN)
                SEND CHANGE { SRC: NODE, REC: WIN } TO OUT

            CASE NONE:
                PUT(R)
                SEND CHANGE { SRC: NODE, REC: R } TO OUT


MODULE store.supervisor
IMPORT store.types
IMPORT store.replicator

FUNC SUPERVISE(IN: CHANNEL<CHANGE>, OUT: CHANNEL<CHANGE>, NODE: string):
    SPAWN REPLICATE(IN, OUT, NODE)


MODULE store.client
IMPORT store.types
IMPORT store.local

FUNC WRITE(OUT: CHANNEL<CHANGE>, KEY: string, VALUE: string):
    LET C SET APPLY_LOCAL(KEY, VALUE)
    SEND C TO OUT


MODULE store.debug
IMPORT store.types
IMPORT store.local

FUNC DUMP():
    PRINTLN("=== DB STATE ===")
    LOOP K IN KEYS(DB):
        LET R SET DB[K]
        PRINTLN(
            R.KEY ADD "=" ADD R.VALUE ADD
            " (v" ADD R.VER ADD ", ts=" ADD R.TS ADD ")"
        )


MODULE app.main
IMPORT store.types
IMPORT store.local
IMPORT store.replicator
IMPORT store.supervisor
IMPORT store.client
IMPORT store.debug

FUNC MAIN():
    # channels between nodes
    LET A_OUT SET CHANNEL<CHANGE>()
    LET B_OUT SET CHANNEL<CHANGE>()

    LET A_IN SET CHANNEL<CHANGE>()
    LET B_IN SET CHANNEL<CHANGE>()

    # supervisors for nodes A and B
    SPAWN SUPERVISE(A_IN, A_OUT, "A")
    SPAWN SUPERVISE(B_IN, B_OUT, "B")

    # cross-wire replication streams
    SPAWN fn():
        LOOP:
            LET C SET RECV A_OUT
            SEND C TO B_IN

    SPAWN fn():
        LOOP:
            LET C SET RECV B_OUT
            SEND C TO A_IN

    # simulate writes on node A
    WRITE(A_IN, "host", "10.0.0.1")
    WRITE(A_IN, "port", "8080")

    # simulate writes on node B
    WRITE(B_IN, "host", "10.0.0.2")
    WRITE(B_IN, "mode", "active")

    # conflict scenario: both nodes write same key
    WRITE(A_IN, "host", "10.0.0.3")
    WRITE(B_IN, "host", "10.0.0.4")

    # allow replication to settle
    SLEEP(3)

    DUMP()

    RETURN NONE
```



# What this XL example demonstrates

### Multi‑module distributed storage architecture  
- `store.types`  
- `store.local`  
- `store.conflict`  
- `store.replicator`  
- `store.supervisor`  
- `store.client`  
- `store.debug`  
- `app.main`

### Replication streams  
- Node A → Node B  
- Node B → Node A  
- Continuous bidirectional replication

### Conflict resolution  
- Version‑based  
- Timestamp‑based  
- Last‑write‑wins  
- Deterministic resolution

### Channels + processes  
- Write → Local apply → Change → Replicator → Remote apply  
- Fully asynchronous  
- Real distributed‑system semantics

### Result + Option  
- Optional local record lookup  
- Safe conflict resolution  
- Pattern matching

### Realistic storage replicator  
- Multi‑node  
- Multi‑stream  
- Conflict scenarios  
- Final state dump  
- Deterministic convergence

This is a **full subsystem**, suitable for real NXD agent training and backend mapping.

