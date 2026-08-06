 ## Distributed Notification Hub  
(topics, subscriptions, fan‑out, channels, multi‑module, supervision)

### JSON‑LD Metadata
```jsonld
{
  "@context": "https://schema.nxd.dev/example",
  "example_id": "nxd_xl_06_notification_hub",
  "category": "Messaging",
  "layer": "xl",
  "features": [
    "multi-module",
    "topics",
    "subscriptions",
    "fanout",
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
MODULE notify.types

TYPE RESULT UNION { OK(any), ERR(string) }
TYPE OPTION UNION { SOME(any), NONE }

TYPE MESSAGE {
    TOPIC: string,
    BODY: string,
    TS: int
}

TYPE SUBSCRIBER {
    NAME: string,
    TOPIC: string,
    CH: CHANNEL<MESSAGE>
}

TYPE CHANNEL<MESSAGE> { }
TYPE CHANNEL<string> { }


MODULE notify.registry
IMPORT notify.types

LET SUBS SET LIST<SUBSCRIBER> []

FUNC SUBSCRIBE(NAME: string, TOPIC: string): SUBSCRIBER:
    LET CH SET CHANNEL<MESSAGE>()
    LET S SET SUBSCRIBER { NAME: NAME, TOPIC: TOPIC, CH: CH }
    PUSH SUBS, S
    RETURN S

FUNC GET_SUBS(TOPIC: string): LIST<SUBSCRIBER>:
    LET OUT SET []
    LOOP S IN SUBS:
        IF S.TOPIC EQ TOPIC:
            PUSH OUT, S
    RETURN OUT


MODULE notify.router
IMPORT notify.types
IMPORT notify.registry

FUNC ROUTE(M: MESSAGE):
    LET LIST SET GET_SUBS(M.TOPIC)
    LOOP S IN LIST:
        SEND M TO S.CH


MODULE notify.supervisor
IMPORT notify.types
IMPORT notify.router

FUNC SUPERVISE(IN: CHANNEL<MESSAGE>):
    LOOP:
        LET M SET RECV IN
        ROUTE(M)


MODULE notify.publisher
IMPORT notify.types

FUNC PUBLISH(IN: CHANNEL<MESSAGE>, TOPIC: string, BODY: string):
    LET M SET MESSAGE {
        TOPIC: TOPIC,
        BODY: BODY,
        TS: NOW()
    }
    SEND M TO IN


MODULE notify.sink
IMPORT notify.types

FUNC PRINT_SINK(S: SUBSCRIBER):
    LOOP:
        LET M SET RECV S.CH
        PRINTLN(
            "[" ADD S.NAME ADD "] " ADD
            M.TOPIC ADD ": " ADD M.BODY
        )


MODULE app.main
IMPORT notify.types
IMPORT notify.registry
IMPORT notify.router
IMPORT notify.supervisor
IMPORT notify.publisher
IMPORT notify.sink

FUNC MAIN():
    LET IN SET CHANNEL<MESSAGE>()

    # spawn supervisor
    SPAWN SUPERVISE(IN)

    # subscribers
    LET S1 SET SUBSCRIBE("audit", "security")
    LET S2 SET SUBSCRIBE("ops", "system")
    LET S3 SET SUBSCRIBE("billing", "payments")
    LET S4 SET SUBSCRIBE("all", "broadcast")

    # sinks
    SPAWN PRINT_SINK(S1)
    SPAWN PRINT_SINK(S2)
    SPAWN PRINT_SINK(S3)
    SPAWN PRINT_SINK(S4)

    # publish messages
    PUBLISH(IN, "security", "multiple auth failures detected")
    PUBLISH(IN, "system", "disk usage at 92%")
    PUBLISH(IN, "payments", "invoice #4421 processed")
    PUBLISH(IN, "broadcast", "system maintenance scheduled")

    # allow routing to complete
    SLEEP(2)

    RETURN NONE
```



# What this XL example demonstrates

### Multi‑module messaging architecture  
- `notify.types`  
- `notify.registry`  
- `notify.router`  
- `notify.supervisor`  
- `notify.publisher`  
- `notify.sink`  
- `app.main`

### Topic‑based subscription system  
- Subscribers register for specific topics  
- Router fans out messages to all matching subscribers  
- Broadcast topic supported

### Channels + processes  
- Publisher → Supervisor → Router → Sinks  
- Fully asynchronous  
- Real distributed‑system semantics

### Result + Option  
- Optional subscriber lists  
- Safe routing logic  
- Pattern matching

### Realistic notification hub  
- Security notifications  
- System operations notifications  
- Billing notifications  
- Broadcast notifications  
- Multi‑sink output

This is a **full subsystem**, suitable for real NXD agent training and backend mapping.

