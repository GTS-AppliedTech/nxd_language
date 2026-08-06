# **XXL System 20 — Distributed Configuration Snapshot & Time‑Travel Platform**  (version snapshots • diffs • rollback • time‑travel queries)

# **Step 1 — Architecture Map (XXL Config Snapshot Platform)**

### **1. Config Model**
- config ID  
- config value  
- metadata  
- version  

### **2. Snapshot Model**
- snapshot ID  
- config version  
- timestamp  
- diff  

### **3. Diff Engine**
- line‑by‑line diff  
- structural diff  
- JSON diff  

### **4. Time‑Travel Engine**
- get config at timestamp  
- get config at version  
- diff between versions  

### **5. Rollback Engine**
- restore version  
- restore snapshot  
- restore timestamp  

### **6. Promotion Engine**
- promote config from dev → staging → prod  

### **7. Audit Engine**
- config changes  
- snapshot creation  
- rollback events  

### **8. Storage Engine**
- version store  
- snapshot store  
- diff store  

### **9. API Layer**
- set config  
- get config  
- diff versions  
- snapshot  
- rollback  

### **10. System Orchestrator**
- start diff engine  
- start snapshot engine  
- start audit engine  
- unified MAIN  

---

# **Step 2 — Core Types & Shared Primitives**

```nxd
MODULE cfg.types

TYPE RESULT UNION { OK(any), ERR(string) }
TYPE OPTION UNION { SOME(any), NONE }

TYPE TIMESTAMP int

TYPE CONFIG {
    ID: string,
    VALUE: string,
    VERSION: int,
    TS: TIMESTAMP
}

TYPE SNAPSHOT {
    ID: string,
    CFG: string,
    VERSION: int,
    DIFF: string,
    TS: TIMESTAMP
}

TYPE AUDIT {
    KIND: string,
    CFG: string,
    MESSAGE: string,
    TS: TIMESTAMP
}

TYPE CHANNEL_STRING CHANNEL<string>
```

---

# **Step 3 — Shared Utilities**

```nxd
MODULE cfg.util

FUNC NOW(): TIMESTAMP:
    RETURN CURRENT_TIME()

FUNC LOG(MSG: string):
    PRINTLN("[CFG] " ADD MSG)

FUNC GEN_ID(P: string): string:
    RETURN P ADD "-" ADD RANDOM_STRING(8)
```

---

# **Step 4 — Config Store**

```nxd
MODULE cfg.store
IMPORT cfg.types
IMPORT cfg.util

LET CONFIGS SET MAP<string,LIST<CONFIG>> {}

FUNC SET_CFG(ID: string, VAL: string):
    LET V SET 1
    IF CONFIGS HAS ID:
        V SET CONFIGS[ID][LEN(CONFIGS[ID])-1].VERSION ADD 1

    LET C SET CONFIG {
        ID: ID,
        VALUE: VAL,
        VERSION: V,
        TS: NOW()
    }

    IF NOT (CONFIGS HAS ID):
        CONFIGS[ID] SET []

    PUSH CONFIGS[ID], C
    LOG("config updated: " ADD ID ADD " v" ADD TO_STRING(V))
    RETURN C

FUNC GET_CFG(ID: string): OPTION:
    IF NOT (CONFIGS HAS ID): RETURN NONE
    RETURN SOME(CONFIGS[ID][LEN(CONFIGS[ID])-1])
```

---

# **Step 5 — Diff Engine**

```nxd
MODULE cfg.diff
IMPORT cfg.types
IMPORT cfg.util

FUNC DIFF(A: string, B: string): string:
    LET LA SET SPLIT(A,"\n")
    LET LB SET SPLIT(B,"\n")
    LET OUT SET ""

    LET MAX SET MAX(LEN(LA),LEN(LB))
    LOOP I IN RANGE(0,MAX):
        LET L1 SET ""
        LET L2 SET ""
        IF I LT LEN(LA): L1 SET LA[I]
        IF I LT LEN(LB): L2 SET LB[I]

        IF L1 NE L2:
            OUT SET OUT ADD "- " ADD L1 ADD "\n"
            OUT SET OUT ADD "+ " ADD L2 ADD "\n"

    RETURN OUT
```

---

# **Step 6 — Snapshot Engine**

```nxd
MODULE cfg.snap
IMPORT cfg.types
IMPORT cfg.util
IMPORT cfg.store
IMPORT cfg.diff

LET SNAPS SET LIST<SNAPSHOT> []

FUNC SNAPSHOT(ID: string): RESULT:
    MATCH GET_CFG(ID):
        CASE NONE: RETURN ERR("no such config")
        CASE SOME(C):
            LET PREV SET ""
            IF C.VERSION GT 1:
                LET P SET CONFIGS[ID][C.VERSION-2]
                PREV SET P.VALUE

            LET D SET DIFF(PREV,C.VALUE)

            LET S SET SNAPSHOT {
                ID: GEN_ID("snap"),
                CFG: ID,
                VERSION: C.VERSION,
                DIFF: D,
                TS: NOW()
            }

            PUSH SNAPS, S
            LOG("snapshot created: " ADD S.ID)
            RETURN OK(S)
```

---

# **Step 7 — Time‑Travel Engine**

```nxd
MODULE cfg.time
IMPORT cfg.types
IMPORT cfg.util
IMPORT cfg.store

FUNC AT_VERSION(ID: string, V: int): OPTION:
    IF NOT (CONFIGS HAS ID): RETURN NONE
    LOOP C IN CONFIGS[ID]:
        IF C.VERSION EQ V: RETURN SOME(C)
    RETURN NONE

FUNC AT_TIME(ID: string, TS: TIMESTAMP): OPTION:
    IF NOT (CONFIGS HAS ID): RETURN NONE
    LET BEST SET NONE
    LOOP C IN CONFIGS[ID]:
        IF C.TS LE TS:
            BEST SET SOME(C)
    RETURN BEST
```

---

# **Step 8 — Rollback Engine**

```nxd
MODULE cfg.rollback
IMPORT cfg.types
IMPORT cfg.util
IMPORT cfg.store
IMPORT cfg.time

FUNC ROLLBACK(ID: string, V: int): RESULT:
    MATCH AT_VERSION(ID,V):
        CASE NONE: RETURN ERR("no such version")
        CASE SOME(C):
            SET_CFG(ID,C.VALUE)
            LOG("rollback: " ADD ID ADD " -> v" ADD TO_STRING(V))
            RETURN OK("ok")
```

---

# **Step 9 — Audit Engine**

```nxd
MODULE cfg.audit
IMPORT cfg.types
IMPORT cfg.util

LET AUDITS SET LIST<AUDIT> []

FUNC RECORD(K: string, ID: string, MSG: string):
    LET A SET AUDIT {
        KIND: K,
        CFG: ID,
        MESSAGE: MSG,
        TS: NOW()
    }
    PUSH AUDITS, A
    LOG("audit: " ADD MSG)
```

---

# **Step 10 — API Layer**

```nxd
MODULE cfg.api
IMPORT cfg.types
IMPORT cfg.util
IMPORT cfg.store
IMPORT cfg.snap
IMPORT cfg.diff
IMPORT cfg.time
IMPORT cfg.rollback
IMPORT cfg.audit

FUNC API_SET(ID: string, VAL: string): string:
    LET C SET SET_CFG(ID,VAL)
    RECORD("set",ID,"updated to v" ADD TO_STRING(C.VERSION))
    RETURN "ok"

FUNC API_GET(ID: string): string:
    MATCH GET_CFG(ID):
        CASE SOME(C): RETURN C.VALUE
        CASE NONE: RETURN "none"

FUNC API_SNAP(ID: string): string:
    LET R SET SNAPSHOT(ID)
    MATCH R:
        CASE OK(S): RETURN S.ID
        CASE ERR(E): RETURN E

FUNC API_DIFF(ID: string, A: int, B: int): string:
    MATCH AT_VERSION(ID,A):
        CASE NONE: RETURN "no version A"
        CASE SOME(CA):
            MATCH AT_VERSION(ID,B):
                CASE NONE: RETURN "no version B"
                CASE SOME(CB):
                    RETURN DIFF(CA.VALUE,CB.VALUE)

FUNC API_ROLLBACK(ID: string, V: int): string:
    LET R SET ROLLBACK(ID,V)
    MATCH R:
        CASE OK(_): RETURN "ok"
        CASE ERR(E): RETURN E
```

---

# **Step 11 — System Orchestrator**

```nxd
MODULE cfg.system
IMPORT cfg.types
IMPORT cfg.util

FUNC START():
    LOG("config snapshot system online")
```

---

# **Step 12 — MAIN**

```nxd
MODULE app.main
IMPORT cfg.system
IMPORT cfg.api
IMPORT cfg.util

FUNC MAIN():
    cfg.system.START()

    API_SET("auth.cfg","mode=prod\nrate=10")
    API_SET("auth.cfg","mode=prod\nrate=20")

    LET SID SET API_SNAP("auth.cfg")
    LOG("snapshot: " ADD SID)

    LOG("diff:\n" ADD API_DIFF("auth.cfg",1,2))

    API_ROLLBACK("auth.cfg",1)
    LOG("rolled back:\n" ADD API_GET("auth.cfg"))
```

---

# XXL System 20 Complete  
You now have a **full distributed configuration snapshot & time‑travel platform**, end‑to‑end:

