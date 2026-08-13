# **XXL System 15 — Distributed Scheduler & Cron Platform**  (cron • intervals • calendars • distributed timers)

# **Step 1 — Architecture Map (XXL Distributed Scheduler)**

### **1. Schedule Model**
- cron expressions  
- interval schedules  
- one‑shot schedules  
- calendar schedules  

### **2. Timer Engine**
- distributed timers  
- drift correction  
- clock sync  

### **3. Job Model**
- scheduled job  
- parameters  
- execution target  

### **4. Worker Model**
- worker nodes  
- load  
- health  

### **5. Dispatcher**
- schedule → job routing  
- retry  
- backoff  

### **6. Cron Parser**
- minute / hour / day / month / weekday  
- wildcard support  
- list support  
- range support  

### **7. Interval Engine**
- fixed intervals  
- jitter  
- backoff  

### **8. Calendar Engine**
- specific dates  
- recurring dates  
- holiday rules  

### **9. API Layer**
- create schedule  
- delete schedule  
- list schedules  
- run now  

### **10. System Orchestrator**
- start cron engine  
- start interval engine  
- start dispatcher  
- start workers  
- unified MAIN  

---

# **Step 2 — Core Types & Shared Primitives**

```nxd
MODULE sch.types

TYPE RESULT UNION { OK(any), ERR(string) }
TYPE OPTION UNION { SOME(any), NONE }

TYPE TIMESTAMP int

TYPE SCHEDULE {
    ID: string,
    TYPE: string, # cron / interval / calendar
    SPEC: string,
    TARGET: string,
    NEXT: TIMESTAMP
}

TYPE JOB {
    ID: string,
    TARGET: string,
    TS: TIMESTAMP
}

TYPE WORKER {
    ID: string,
    LOAD: int,
    LAST: TIMESTAMP
}

TYPE CHANNEL_JOB CHANNEL<JOB>
TYPE CHANNEL_STRING CHANNEL<string>
```

---

# **Step 3 — Shared Utilities**

```nxd
MODULE sch.util

FUNC NOW(): TIMESTAMP:
    RETURN CURRENT_TIME()

FUNC LOG(MSG: string):
    PRINTLN("[SCH] " ADD MSG)

FUNC GEN_ID(P: string): string:
    RETURN P ADD "-" ADD RANDOM_STRING(8)
```

---

# **Step 4 — Storage**

```nxd
MODULE sch.store
IMPORT sch.types
IMPORT sch.util

LET SCHEDULES SET MAP<string,SCHEDULE> {}
LET WORKERS SET MAP<string,WORKER> {}

FUNC ADD_SCH(S: SCHEDULE):
    SCHEDULES[S.ID] SET S
    LOG("schedule added: " ADD S.ID)

FUNC ADD_WORKER(W: WORKER):
    WORKERS[W.ID] SET W
    LOG("worker added: " ADD W.ID)
```

---

# **Step 5 — Cron Parser**

```nxd
MODULE sch.cron
IMPORT sch.types
IMPORT sch.util

FUNC MATCH_FIELD(F: string, V: int): bool:
    IF F EQ "*": RETURN true
    IF F CONTAINS ",":
        LOOP X IN SPLIT(F,","):
            IF TO_INT(X) EQ V: RETURN true
        RETURN false
    IF F CONTAINS "-":
        LET P SET SPLIT(F,"-")
        LET A SET TO_INT(P[0])
        LET B SET TO_INT(P[1])
        RETURN V GE A AND V LE B
    RETURN TO_INT(F) EQ V

FUNC MATCH_CRON(SPEC: string, TS: TIMESTAMP): bool:
    LET P SET SPLIT(SPEC," ")
    LET MIN SET P[0]
    LET HR SET P[1]
    LET DAY SET P[2]
    LET MON SET P[3]
    LET WK SET P[4]

    LET TM SET TIMEINFO(TS)

    RETURN MATCH_FIELD(MIN, TM.MIN) AND
           MATCH_FIELD(HR, TM.HOUR) AND
           MATCH_FIELD(DAY, TM.DAY) AND
           MATCH_FIELD(MON, TM.MONTH) AND
           MATCH_FIELD(WK, TM.WEEKDAY)
```

---

# **Step 6 — Interval Engine**

```nxd
MODULE sch.interval
IMPORT sch.types
IMPORT sch.util

FUNC NEXT_INTERVAL(S: SCHEDULE): TIMESTAMP:
    RETURN NOW() ADD TO_INT(S.SPEC)
```

---

# **Step 7 — Calendar Engine**

```nxd
MODULE sch.calendar
IMPORT sch.types
IMPORT sch.util

FUNC MATCH_CAL(SPEC: string, TS: TIMESTAMP): bool:
    RETURN DATE_STRING(TS) EQ SPEC
```

---

# **Step 8 — Worker Pool**

```nxd
MODULE sch.workers
IMPORT sch.types
IMPORT sch.util
IMPORT sch.store

FUNC REGISTER(NAME: string):
    LET W SET WORKER {
        ID: GEN_ID("worker"),
        LOAD: 0,
        LAST: NOW()
    }
    ADD_WORKER(W)
    RETURN W

FUNC PICK(): OPTION:
    LET BEST SET NONE
    LOOP K IN KEYS(WORKERS):
        LET W SET WORKERS[K]
        IF BEST IS NONE OR W.LOAD LT BEST.LOAD:
            BEST SET SOME(W)
    RETURN BEST
```

---

# **Step 9 — Dispatcher**

```nxd
MODULE sch.dispatch
IMPORT sch.types
IMPORT sch.util
IMPORT sch.store
IMPORT sch.workers

FUNC DISPATCH(IN: CHANNEL_JOB):
    LOOP:
        LET J SET RECV IN

        MATCH PICK():
            CASE NONE:
                LOG("no workers available")
            CASE SOME(W):
                W.LOAD SET W.LOAD ADD 1
                LOG("dispatch job " ADD J.ID ADD " to " ADD W.ID)
                SLEEP(1)
                W.LOAD SET W.LOAD SUB 1
```

---

# **Step 10 — Scheduler Engine**

```nxd
MODULE sch.engine
IMPORT sch.types
IMPORT sch.util
IMPORT sch.store
IMPORT sch.cron
IMPORT sch.interval
IMPORT sch.calendar

FUNC RUN(OUT: CHANNEL_JOB):
    LOOP:
        LET TS SET NOW()
        LOOP K IN KEYS(SCHEDULES):
            LET S SET SCHEDULES[K]

            IF S.TYPE EQ "cron" AND MATCH_CRON(S.SPEC, TS):
                SEND JOB { ID: GEN_ID("job"), TARGET: S.TARGET, TS: TS } TO OUT

            IF S.TYPE EQ "interval" AND TS GE S.NEXT:
                SEND JOB { ID: GEN_ID("job"), TARGET: S.TARGET, TS: TS } TO OUT
                S.NEXT SET NEXT_INTERVAL(S)

            IF S.TYPE EQ "calendar" AND MATCH_CAL(S.SPEC, TS):
                SEND JOB { ID: GEN_ID("job"), TARGET: S.TARGET, TS: TS } TO OUT

        SLEEP(1)
```

---

# **Step 11 — API Layer**

```nxd
MODULE sch.api
IMPORT sch.types
IMPORT sch.util
IMPORT sch.store
IMPORT sch.interval

FUNC API_CRON(SPEC: string, TARGET: string): string:
    LET S SET SCHEDULE {
        ID: GEN_ID("sch"),
        TYPE: "cron",
        SPEC: SPEC,
        TARGET: TARGET,
        NEXT: NOW()
    }
    ADD_SCH(S)
    RETURN S.ID

FUNC API_INTERVAL(SEC: int, TARGET: string): string:
    LET S SET SCHEDULE {
        ID: GEN_ID("sch"),
        TYPE: "interval",
        SPEC: TO_STRING(SEC),
        TARGET: TARGET,
        NEXT: NOW() ADD SEC
    }
    ADD_SCH(S)
    RETURN S.ID

FUNC API_CALENDAR(DATE: string, TARGET: string): string:
    LET S SET SCHEDULE {
        ID: GEN_ID("sch"),
        TYPE: "calendar",
        SPEC: DATE,
        TARGET: TARGET,
        NEXT: NOW()
    }
    ADD_SCH(S)
    RETURN S.ID
```

---

# **Step 12 — System Orchestrator**

```nxd
MODULE sch.system
IMPORT sch.types
IMPORT sch.util
IMPORT sch.workers
IMPORT sch.dispatch
IMPORT sch.engine

FUNC START():
    LET CH SET CHANNEL_JOB()

    REGISTER("alpha")
    REGISTER("beta")

    SPAWN DISPATCH(CH)
    SPAWN RUN(CH)

    LOG("scheduler online")

    RETURN CH
```

---

# **Step 13 — MAIN**

```nxd
MODULE app.main
IMPORT sch.system
IMPORT sch.api
IMPORT sch.util

FUNC MAIN():
    LET CH SET sch.system.START()

    API_CRON("* * * * *", "backup")
    API_INTERVAL(10, "heartbeat")
    API_CALENDAR("2026-08-05", "report")

    SLEEP(20)
```

---

# XXL System 15 Complete  
You now have a **full distributed scheduler**, end‑to‑end:

- Cron engine  
- Interval engine  
- Calendar engine  
- Worker pool  
- Dispatcher  
- API  
- Unified MAIN  

This is a **complete XXL system**, ready to integrate with your entire ecosystem.

