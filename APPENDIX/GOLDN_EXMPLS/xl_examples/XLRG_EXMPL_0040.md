## Security Orchestration & Automated Response (SOAR)  
(playbooks, triggers, actions, automation, channels, multi‑module, supervision)

### JSON‑LD Metadata
```jsonld
{
  "@context": "https://schema.nxd.dev/example",
  "example_id": "nxd_xl_10_soar_engine",
  "category": "Security",
  "layer": "xl",
  "features": [
    "multi-module",
    "playbooks",
    "automation",
    "actions",
    "triggers",
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
MODULE soar.types

TYPE RESULT UNION { OK(any), ERR(string) }
TYPE OPTION UNION { SOME(any), NONE }

TYPE ALERT {
    NAME: string,
    SEVERITY: string,
    MESSAGE: string,
    SRC: string,
    TS: int
}

TYPE ACTION {
    NAME: string,
    PARAM: string
}

TYPE PLAYBOOK {
    NAME: string,
    TRIGGER: string,
    ACTIONS: LIST<ACTION>
}

TYPE CHANNEL<ALERT> { }
TYPE CHANNEL<string> { }
```



## MODULE: Playbook Registry

```nxd
MODULE soar.playbooks
IMPORT soar.types

LET PB_DB SET LIST<PLAYBOOK> []

FUNC REGISTER(PB: PLAYBOOK):
    PUSH PB_DB, PB

FUNC MATCH_PLAYBOOKS(AL: ALERT): LIST<PLAYBOOK>:
    LET OUT SET []
    LOOP PB IN PB_DB:
        IF PB.TRIGGER EQ AL.NAME:
            PUSH OUT, PB
    RETURN OUT
```



## MODULE: Action Engine

```nxd
MODULE soar.actions
IMPORT soar.types

FUNC EXEC_ACTION(A: ACTION): RESULT:
    IF A.NAME EQ "block_ip":
        PRINTLN("[ACTION] blocking IP " ADD A.PARAM)
        RETURN OK("blocked " ADD A.PARAM)

    IF A.NAME EQ "disable_user":
        PRINTLN("[ACTION] disabling user " ADD A.PARAM)
        RETURN OK("disabled " ADD A.PARAM)

    IF A.NAME EQ "notify_team":
        PRINTLN("[ACTION] notifying team: " ADD A.PARAM)
        RETURN OK("notified " ADD A.PARAM)

    RETURN ERR("unknown action: " ADD A.NAME)
```



## MODULE: Orchestrator

```nxd
MODULE soar.orchestrator
IMPORT soar.types
IMPORT soar.playbooks
IMPORT soar.actions

FUNC RUN_PLAYBOOK(PB: PLAYBOOK, AL: ALERT):
    PRINTLN("[PLAYBOOK] running " ADD PB.NAME)

    LOOP A IN PB.ACTIONS:
        LET R SET EXEC_ACTION(A)
        MATCH R:
            CASE OK(MSG):
                PRINTLN("[PLAYBOOK] " ADD MSG)
            CASE ERR(E):
                PRINTLN("[PLAYBOOK ERROR] " ADD E)

FUNC ORCHESTRATE(IN: CHANNEL<ALERT>):
    LOOP:
        LET AL SET RECV IN
        LET PBS SET MATCH_PLAYBOOKS(AL)

        LOOP PB IN PBS:
            RUN_PLAYBOOK(PB, AL)
```



## MODULE: Supervisor

```nxd
MODULE soar.supervisor
IMPORT soar.types
IMPORT soar.orchestrator

FUNC SUPERVISE(IN: CHANNEL<ALERT>):
    SPAWN ORCHESTRATE(IN)
```



## MODULE: Sink (Alert Source)

```nxd
MODULE soar.sink
IMPORT soar.types

FUNC EMIT(IN: CHANNEL<ALERT>, NAME: string, SEVERITY: string, MESSAGE: string, SRC: string):
    LET A SET ALERT {
        NAME: NAME,
        SEVERITY: SEVERITY,
        MESSAGE: MESSAGE,
        SRC: SRC,
        TS: NOW()
    }
    SEND A TO IN
```



## MODULE: App Main

```nxd
MODULE app.main
IMPORT soar.types
IMPORT soar.playbooks
IMPORT soar.actions
IMPORT soar.orchestrator
IMPORT soar.supervisor
IMPORT soar.sink

FUNC MAIN():
    LET IN SET CHANNEL<ALERT>()

    # supervisor
    SPAWN SUPERVISE(IN)

    # register playbooks
    REGISTER(PLAYBOOK {
        NAME: "block-malicious-ip",
        TRIGGER: "portscan",
        ACTIONS: [
            ACTION { NAME: "block_ip", PARAM: "192.168.1.55" },
            ACTION { NAME: "notify_team", PARAM: "SOC" }
        ]
    })

    REGISTER(PLAYBOOK {
        NAME: "disable-compromised-user",
        TRIGGER: "privesc",
        ACTIONS: [
            ACTION { NAME: "disable_user", PARAM: "sam" },
            ACTION { NAME: "notify_team", PARAM: "IR" }
        ]
    })

    REGISTER(PLAYBOOK {
        NAME: "internal-anomaly-response",
        TRIGGER: "internal-anomaly",
        ACTIONS: [
            ACTION { NAME: "notify_team", PARAM: "Ops" }
        ]
    })

    # simulate alerts
    EMIT(IN, "portscan", "high", "multiple ports hit", "scanner")
    EMIT(IN, "privesc", "critical", "root access after failures", "auth")
    EMIT(IN, "internal-anomaly", "medium", "excessive internal activity", "system")

    SLEEP(3)

    RETURN NONE
```



# What this XL example demonstrates

### Multi‑module SOAR architecture  
- Playbook registry  
- Action engine  
- Orchestrator  
- Supervisor  
- Alert sink  
- App main

### Playbook system  
- Trigger → Actions  
- Multiple actions per playbook  
- Multiple playbooks per alert  
- Deterministic matching

### Action engine  
- block_ip  
- disable_user  
- notify_team  
- extensible action model

### Channels + processes  
- Alerts → Supervisor → Orchestrator → Actions  
- Fully asynchronous  
- Real distributed‑system semantics

### Result + Option  
- Safe action execution  
- Error handling  
- Pattern matching

### Realistic SOAR subsystem  
- Portscan response  
- Privilege escalation response  
- Internal anomaly response  
- Multi‑action automation  
- SOC/IR/Ops notifications

This completes the **first 10 XL‑layer NXD examples** — a full suite of production‑grade subsystems.

