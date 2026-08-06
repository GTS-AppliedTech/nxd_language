# EVENT BUS (PUB/SUB + CHANNELS + PROCESSES + MULTI-MODULE)

### JSON-LD Metadata
```jsonld
{
  "@context": "https://schema.nxd.dev/example",
  "example_id": "nxd_large_04_event_bus",
  "category": "Concurrency",
  "layer": "large",
  "features": ["pubsub", "channels", "processes", "modules"]
}
```


### NXD
```nxd
MODULE bus.core

TYPE CHANNEL<string> { }

TYPE SUBSCRIBER { NAME: string, CH: CHANNEL<string> }

LET SUBS SET LIST<SUBSCRIBER> []

FUNC SUBSCRIBE(NAME: string): SUBSCRIBER:
    LET CH SET CHANNEL<string>()
    LET S SET SUBSCRIBER { NAME: NAME, CH: CH }
    PUSH SUBS, S
    RETURN S

FUNC PUBLISH(MSG: string):
    LOOP S IN SUBS:
        SEND MSG TO S.CH


MODULE bus.handlers
IMPORT bus.core

FUNC LOGGER(S: SUBSCRIBER):
    LOOP:
        LET M SET RECV S.CH
        PRINTLN("[" ADD S.NAME ADD "] " ADD M)

FUNC ALERT(S: SUBSCRIBER):
    LOOP:
        LET M SET RECV S.CH
        IF M CONTAINS "error":
            PRINTLN("ALERT: " ADD M)


MODULE app.main
IMPORT bus.core
IMPORT bus.handlers

FUNC MAIN():
    LET L SET SUBSCRIBE("logger")
    LET A SET SUBSCRIBE("alert")

    SPAWN LOGGER(L)
    SPAWN ALERT(A)

    PUBLISH("system started")
    PUBLISH("error: disk full")

    RETURN NONE
```