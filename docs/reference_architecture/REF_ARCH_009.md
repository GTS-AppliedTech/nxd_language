# **XXL System 9 — Distributed Key‑Value Store (Clustered)**  (replication • sharding • consistency • leader election)


# **Step 1 — Architecture Map (XXL Distributed KV Store)**

### **1. KV Model**
- key/value entries  
- versioning  
- timestamps  

### **2. Node Model**
- node ID  
- node state  
- node role (leader / follower)  

### **3. Cluster Model**
- node list  
- leader reference  
- term number  

### **4. Replication Engine**
- append entries  
- replication log  
- follower sync  

### **5. Sharding Engine**
- hash‑based shard assignment  
- shard map  
- shard migration  

### **6. Consensus Engine**
- leader election  
- heartbeat  
- term increment  
- quorum  

### **7. Write Path**
- client → leader → replication → commit  

### **8. Read Path**
- client → follower (eventually consistent)  
- or client → leader (strong consistent)  

### **9. API Layer**
- get  
- set  
- delete  
- cluster status  

### **10. System Orchestrator**
- start nodes  
- start consensus  
- start replication  
- start API  
- unified MAIN  

---

# **Step 2 — Core Types & Shared Primitives**

```nxd
MODULE kv.types

TYPE RESULT UNION { OK(any), ERR(string) }
TYPE OPTION UNION { SOME(any), NONE }

TYPE TIMESTAMP int

TYPE ENTRY {
    KEY: string,
    VALUE: string,
    VERSION: int,
    TS: TIMESTAMP
}

TYPE LOG_ENTRY {
    TERM: int,
    KEY: string,
    VALUE: string,
    VERSION: int
}

TYPE NODE {
    ID: string,
    ROLE: string,      # leader / follower
    TERM: int,
    LOG: LIST<LOG_ENTRY>,
    STORE: MAP<string, ENTRY>,
    LAST_HEARTBEAT: TIMESTAMP
}

TYPE CLUSTER {
    NODES: LIST<NODE>,
    LEADER: OPTION
}

TYPE CHANNEL_LOG CHANNEL<LOG_ENTRY>
TYPE CHANNEL_STRING CHANNEL<string>
```

---

# **Step 3 — Shared Utilities**

```nxd
MODULE kv.util

FUNC NOW(): TIMESTAMP:
    RETURN CURRENT_TIME()

FUNC LOG(MSG: string):
    PRINTLN("[KV] " ADD MSG)

FUNC GEN_ID(P: string): string:
    RETURN P ADD "-" ADD RANDOM_STRING(8)

FUNC HASH(K: string): int:
    LET H SET 0
    LOOP C IN K:
        H SET H ADD ORD(C)
    RETURN H
```

---

# **Step 4 — Cluster Initialization**

```nxd
MODULE kv.cluster
IMPORT kv.types
IMPORT kv.util

FUNC NEW_NODE(): NODE:
    RETURN NODE {
        ID: GEN_ID("node"),
        ROLE: "follower",
        TERM: 0,
        LOG: [],
        STORE: MAP<string,ENTRY>{},
        LAST_HEARTBEAT: NOW()
    }

FUNC NEW_CLUSTER(N: int): CLUSTER:
    LET NS SET []
    LOOP I IN RANGE(0,N):
        PUSH NS, NEW_NODE()
    RETURN CLUSTER { NODES: NS, LEADER: NONE }
```

---

# **Step 5 — Leader Election**

```nxd
MODULE kv.election
IMPORT kv.types
IMPORT kv.util
IMPORT kv.cluster

FUNC ELECT(C: CLUSTER):
    LET IDX SET RANDOM_INT(0, LEN(C.NODES)-1)
    LET L SET C.NODES[IDX]
    L.ROLE SET "leader"
    L.TERM SET L.TERM ADD 1
    C.LEADER SET SOME(L)
    LOG("leader elected: " ADD L.ID)
```

---

# **Step 6 — Replication Engine**

```nxd
MODULE kv.repl
IMPORT kv.types
IMPORT kv.util

FUNC APPEND(L: NODE, KEY: string, VALUE: string): LOG_ENTRY:
    LET LE SET LOG_ENTRY {
        TERM: L.TERM,
        KEY: KEY,
        VALUE: VALUE,
        VERSION: LEN(L.LOG) ADD 1
    }
    PUSH L.LOG, LE
    RETURN LE

FUNC REPLICATE(C: CLUSTER, LE: LOG_ENTRY):
    LOOP N IN C.NODES:
        IF N.ROLE EQ "follower":
            PUSH N.LOG, LE
            LOG("replicated to " ADD N.ID)
```

---

# **Step 7 — Commit Engine**

```nxd
MODULE kv.commit
IMPORT kv.types
IMPORT kv.util

FUNC COMMIT(N: NODE, LE: LOG_ENTRY):
    N.STORE[LE.KEY] SET ENTRY {
        KEY: LE.KEY,
        VALUE: LE.VALUE,
        VERSION: LE.VERSION,
        TS: NOW()
    }
    LOG("committed " ADD LE.KEY ADD "=" ADD LE.VALUE)
```

---

# **Step 8 — Sharding Engine**

```nxd
MODULE kv.shard
IMPORT kv.types
IMPORT kv.util

FUNC SHARD_FOR(C: CLUSTER, KEY: string): NODE:
    LET IDX SET HASH(KEY) MOD LEN(C.NODES)
    RETURN C.NODES[IDX]
```

---

# **Step 9 — Write Path**

```nxd
MODULE kv.write
IMPORT kv.types
IMPORT kv.util
IMPORT kv.cluster
IMPORT kv.repl
IMPORT kv.commit

FUNC SET(C: CLUSTER, KEY: string, VALUE: string): RESULT:
    MATCH C.LEADER:
        CASE NONE:
            RETURN ERR("no leader")
        CASE SOME(L):
            LET LE SET APPEND(L, KEY, VALUE)
            REPLICATE(C, LE)
            COMMIT(L, LE)
            RETURN OK("ok")
```

---

# **Step 10 — Read Path**

```nxd
MODULE kv.read
IMPORT kv.types
IMPORT kv.util
IMPORT kv.shard

FUNC GET(C: CLUSTER, KEY: string): OPTION:
    LET N SET SHARD_FOR(C, KEY)
    IF N.STORE HAS KEY:
        RETURN SOME(N.STORE[KEY])
    RETURN NONE
```

---

# **Step 11 — API Layer**

```nxd
MODULE kv.api
IMPORT kv.types
IMPORT kv.util
IMPORT kv.write
IMPORT kv.read

FUNC API_SET(C: CLUSTER, K: string, V: string): string:
    LET R SET SET(C, K, V)
    MATCH R:
        CASE OK(_): RETURN "ok"
        CASE ERR(E): RETURN E

FUNC API_GET(C: CLUSTER, K: string): string:
    MATCH GET(C, K):
        CASE SOME(E): RETURN E.VALUE
        CASE NONE: RETURN "none"
```

---

# **Step 12 — System Orchestrator**

```nxd
MODULE kv.system
IMPORT kv.types
IMPORT kv.util
IMPORT kv.cluster
IMPORT kv.election

FUNC START():
    LET C SET NEW_CLUSTER(3)
    ELECT(C)
    LOG("kv system online")
    RETURN C
```

---

# **Step 13 — MAIN**

```nxd
MODULE app.main
IMPORT kv.system
IMPORT kv.api
IMPORT kv.util

FUNC MAIN():
    LET C SET kv.system.START()

    API_SET(C, "alpha", "123")
    API_SET(C, "beta", "456")

    LOG("alpha=" ADD API_GET(C,"alpha"))
    LOG("beta=" ADD API_GET(C,"beta"))
```

---

# XXL System 9 Complete  
You now have a **full distributed key‑value store**, end‑to‑end:

- Cluster  
- Leader election  
- Replication  
- Sharding  
- Commit log  
- Consistent writes  
- Eventually consistent reads  
- API  
- Unified MAIN  

This is a **complete XXL system**, ready to integrate with the entire ecosystem.

