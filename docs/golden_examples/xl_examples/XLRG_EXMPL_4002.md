# DISTRIBUTED CONFIGURATION SERVICE (MULTI-MODULE, VERSIONING, WATCHERS, CHANNELS, TASKS, RESULT, OPTION)

### JSON-LD Metadata
```jsonld
{
  "@context": "https://schema.nxd.dev/example",
  "example_id": "nxd_xl_02_config_service",
  "category": "Configuration",
  "layer": "xl",
  "features": [
    "multi-module",
    "versioning",
    "watchers",
    "channels",
    "tasks",
    "result",
    "option",
    "init",
    "supervision"
  ]
}
```


### NXD
```nxd
MODULE config.types

TYPE RESULT UNION { OK(any), ERR(string) }
TYPE OPTION UNION { SOME(any), NONE }

TYPE VERSION { MAJOR: int, MINOR: int }
TYPE CONFIG { KEY: string, VALUE: string, VER: VERSION }

TYPE CHANNEL<CONFIG> { }
TYPE CHANNEL<string> { }   # watcher notifications


MODULE config.store
IMPORT config.types

LET STORE SET MAP<string, CONFIG> {}

FUNC VERSION_INC(V: VERSION): VERSION:
    RETURN VERSION { MAJOR: V.MAJOR, MINOR: V.MINOR ADD 1 }

FUNC PUT(KEY: string, VALUE: string): RESULT:
    IF STORE HAS KEY:
        LET OLD SET STORE[KEY]
        LET NEW_VER SET VERSION_INC(OLD.VER)
        LET NEW SET CONFIG { KEY: KEY, VALUE: VALUE, VER: NEW_VER }
        STORE[KEY] SET NEW
        RETURN OK(NEW)
    OTHERWISE:
        LET NEW SET CONFIG {
            KEY: KEY,
            VALUE: VALUE,
            VER: VERSION { MAJOR: 1, MINOR: 0 }
        }
        STORE[KEY] SET NEW
        RETURN OK(NEW)

FUNC GET(KEY: string): OPTION:
    IF STORE HAS KEY:
        RETURN SOME(STORE[KEY])
    RETURN NONE

FUNC DELETE(KEY: string): RESULT:
    IF STORE HAS KEY:
        REMOVE STORE[KEY]
        RETURN OK("deleted")
    RETURN ERR("missing key")

FUNC DUMP():
    LOOP K IN KEYS(STORE):
        LET C SET STORE[K]
        PRINTLN(C.KEY ADD "=" ADD C.VALUE ADD " (v" ADD C.VER.MAJOR ADD "." ADD C.VER.MINOR ADD ")")
    RETURN NONE


MODULE config.watch
IMPORT config.types
IMPORT config.store

# watchers subscribe to key changes
TYPE WATCHER { KEY: string, CH: CHANNEL<string> }

LET WATCHERS SET LIST<WATCHER> []

FUNC SUBSCRIBE(KEY: string): WATCHER:
    LET CH SET CHANNEL<string>()
    LET W SET WATCHER { KEY: KEY, CH: CH }
    PUSH WATCHERS, W
    RETURN W

FUNC NOTIFY(KEY: string, MSG: string):
    LOOP W IN WATCHERS:
        IF W.KEY EQ KEY:
            SEND MSG TO W.CH


MODULE config.supervisor
IMPORT config.types
IMPORT config.store
IMPORT config.watch

FUNC SUPERVISE(IN: CHANNEL<CONFIG>):
    LOOP:
        LET C SET RECV IN
        LET R SET PUT(C.KEY, C.VALUE)

        MATCH R:
            CASE OK(NEW):
                LET MSG SET "updated " ADD NEW.KEY ADD " -> " ADD NEW.VALUE
                NOTIFY(NEW.KEY, MSG)
            CASE ERR(E):
                PRINTLN("error updating config: " ADD E)


MODULE api.client
IMPORT config.types

FUNC BUILD(KEY: string, VALUE: string): CONFIG:
    RETURN CONFIG {
        KEY: KEY,
        VALUE: VALUE,
        VER: VERSION { MAJOR: 0, MINOR: 0 }
    }


MODULE app.main
IMPORT config.types
IMPORT config.store
IMPORT config.watch
IMPORT config.supervisor
IMPORT api.client

FUNC MAIN():
    LET IN SET CHANNEL<CONFIG>()

    SPAWN SUPERVISE(IN)

    # watchers
    LET W1 SET SUBSCRIBE("host")
    LET W2 SET SUBSCRIBE("port")

    # simulate clients
    LET C1 SET BUILD("host", "localhost")
    LET C2 SET BUILD("port", "8080")
    LET C3 SET BUILD("host", "127.0.0.1")
    LET C4 SET BUILD("port", "9090")

    SEND C1 TO IN
    SEND C2 TO IN
    SEND C3 TO IN
    SEND C4 TO IN

    # watcher listeners
    SPAWN fn():
        LOOP:
            LET M SET RECV W1.CH
            PRINTLN("[watch host] " ADD M)

    SPAWN fn():
        LOOP:
            LET M SET RECV W2.CH
            PRINTLN("[watch port] " ADD M)

    # allow events to flow
    SLEEP(2)

    DUMP()

    RETURN NONE
```


What this XL example demonstrates
#  Multi‑module architecture
config.types

config.store

config.watch

config.supervisor

api.client

app.main

# Versioning system
Major/minor version increments

Automatic version bump on update

# Watchers
Subscribe to key changes

Receive notifications via channels

Multiple watchers per key

# Supervisor process
Receives config updates

Applies them

Notifies watchers

Handles errors

# Channels + processes + tasks
Real distributed‑system semantics

Event propagation

Asynchronous updates

# Result + Option
Safe error handling

Safe lookup

Pattern matching

# Full end‑to‑end workflow
Client → Supervisor → Store → Watchers → Dump

This is a full subsystem, not a toy example.