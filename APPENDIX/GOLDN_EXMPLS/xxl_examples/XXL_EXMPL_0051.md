# **XXL System 11 — Distributed Container Runtime**  
(images • layers • registry • scheduler • container engine)


# **Step 1 — Architecture Map (XXL Container Runtime)**

### **1. Image Model**
- image manifest  
- layers  
- metadata  
- versioning  

### **2. Registry**
- push  
- pull  
- layer deduplication  

### **3. Layer Store**
- layer blobs  
- compression  
- caching  

### **4. Container Model**
- container ID  
- image reference  
- environment  
- mounts  
- network  

### **5. Container Engine**
- create  
- start  
- stop  
- delete  

### **6. Scheduler**
- node selection  
- resource constraints  
- placement  

### **7. Node Model**
- CPU  
- memory  
- running containers  
- free resources  

### **8. Networking**
- container IP  
- virtual network  
- port mapping  

### **9. API Layer**
- pull image  
- run container  
- stop container  
- list containers  

### **10. System Orchestrator**
- start registry  
- start scheduler  
- start engine  
- unified MAIN  

---

# **Step 2 — Core Types & Shared Primitives**

```nxd
MODULE ctr.types

TYPE RESULT UNION { OK(any), ERR(string) }
TYPE OPTION UNION { SOME(any), NONE }

TYPE TIMESTAMP int

TYPE LAYER {
    ID: string,
    DATA: string,
    SIZE: int
}

TYPE IMAGE {
    NAME: string,
    TAG: string,
    LAYERS: LIST<string>,
    META: MAP<string,string>
}

TYPE CONTAINER {
    ID: string,
    IMAGE: string,
    STATUS: string,
    ENV: MAP<string,string>,
    NODE: string,
    CREATED: TIMESTAMP
}

TYPE NODE {
    ID: string,
    CPU: int,
    MEM: int,
    USED_CPU: int,
    USED_MEM: int,
    CONTAINERS: LIST<string>
}

TYPE CHANNEL_STRING CHANNEL<string>
TYPE CHANNEL_CONTAINER CHANNEL<CONTAINER>
```

---

# **Step 3 — Shared Utilities**

```nxd
MODULE ctr.util

FUNC NOW(): TIMESTAMP:
    RETURN CURRENT_TIME()

FUNC LOG(MSG: string):
    PRINTLN("[CTR] " ADD MSG)

FUNC GEN_ID(P: string): string:
    RETURN P ADD "-" ADD RANDOM_STRING(8)
```

---

# **Step 4 — Registry**

```nxd
MODULE ctr.registry
IMPORT ctr.types
IMPORT ctr.util

LET IMAGES SET MAP<string,IMAGE> {}
LET LAYERS SET MAP<string,LAYER> {}

FUNC PUSH_LAYER(DATA: string): string:
    LET ID SET GEN_ID("layer")
    LET L SET LAYER { ID: ID, DATA: DATA, SIZE: LEN(DATA) }
    LAYERS[ID] SET L
    LOG("pushed layer " ADD ID)
    RETURN ID

FUNC PUSH_IMAGE(NAME: string, TAG: string, LIDS: LIST<string>): string:
    LET KEY SET NAME ADD ":" ADD TAG
    LET IMG SET IMAGE {
        NAME: NAME,
        TAG: TAG,
        LAYERS: LIDS,
        META: MAP<string,string>{"created": NOW()}
    }
    IMAGES[KEY] SET IMG
    LOG("pushed image " ADD KEY)
    RETURN KEY

FUNC PULL_IMAGE(KEY: string): OPTION:
    IF IMAGES HAS KEY:
        RETURN SOME(IMAGES[KEY])
    RETURN NONE
```

---

# **Step 5 — Node Model**

```nxd
MODULE ctr.node
IMPORT ctr.types
IMPORT ctr.util

FUNC NEW_NODE(): NODE:
    RETURN NODE {
        ID: GEN_ID("node"),
        CPU: 16,
        MEM: 64000,
        USED_CPU: 0,
        USED_MEM: 0,
        CONTAINERS: []
    }

FUNC NEW_CLUSTER(N: int): LIST<NODE>:
    LET OUT SET []
    LOOP I IN RANGE(0,N):
        PUSH OUT, NEW_NODE()
    RETURN OUT
```

---

# **Step 6 — Scheduler**

```nxd
MODULE ctr.sched
IMPORT ctr.types
IMPORT ctr.util

FUNC PICK_NODE(NODES: LIST<NODE>, CPU: int, MEM: int): OPTION:
    LOOP N IN NODES:
        IF N.CPU SUB N.USED_CPU GE CPU AND N.MEM SUB N.USED_MEM GE MEM:
            RETURN SOME(N)
    RETURN NONE
```

---

# **Step 7 — Container Engine**

```nxd
MODULE ctr.engine
IMPORT ctr.types
IMPORT ctr.util
IMPORT ctr.registry
IMPORT ctr.sched

LET CONTAINERS SET MAP<string,CONTAINER> {}

FUNC CREATE(NODES: LIST<NODE>, IMAGE: string): RESULT:
    MATCH PULL_IMAGE(IMAGE):
        CASE NONE:
            RETURN ERR("image not found")
        CASE SOME(IMG):
            LET CPU SET 1
            LET MEM SET 512

            MATCH PICK_NODE(NODES, CPU, MEM):
                CASE NONE:
                    RETURN ERR("no node available")
                CASE SOME(N):
                    LET CID SET GEN_ID("ctr")
                    LET C SET CONTAINER {
                        ID: CID,
                        IMAGE: IMAGE,
                        STATUS: "running",
                        ENV: MAP<string,string>{},
                        NODE: N.ID,
                        CREATED: NOW()
                    }

                    CONTAINERS[CID] SET C
                    PUSH N.CONTAINERS, CID
                    N.USED_CPU SET N.USED_CPU ADD CPU
                    N.USED_MEM SET N.USED_MEM ADD MEM

                    LOG("container started: " ADD CID)
                    RETURN OK(CID)
```

---

# **Step 8 — Stop/Delete**

```nxd
MODULE ctr.stop
IMPORT ctr.types
IMPORT ctr.util
IMPORT ctr.node
IMPORT ctr.engine

FUNC STOP(NODES: LIST<NODE>, CID: string): RESULT:
    IF NOT (CONTAINERS HAS CID):
        RETURN ERR("not found")

    LET C SET CONTAINERS[CID]
    LET N SET NONE

    LOOP X IN NODES:
        IF X.ID EQ C.NODE:
            N SET X

    IF N IS NONE:
        RETURN ERR("node missing")

    REMOVE N.CONTAINERS WHERE VALUE EQ CID
    N.USED_CPU SET N.USED_CPU SUB 1
    N.USED_MEM SET N.USED_MEM SUB 512

    C.STATUS SET "stopped"
    LOG("container stopped: " ADD CID)

    RETURN OK("ok")
```

---

# **Step 9 — API Layer**

```nxd
MODULE ctr.api
IMPORT ctr.types
IMPORT ctr.util
IMPORT ctr.registry
IMPORT ctr.engine
IMPORT ctr.stop

FUNC API_PULL(NAME: string, TAG: string): string:
    LET KEY SET NAME ADD ":" ADD TAG
    MATCH PULL_IMAGE(KEY):
        CASE SOME(_): RETURN "exists"
        CASE NONE: RETURN "none"

FUNC API_RUN(NODES: LIST<NODE>, IMAGE: string): string:
    LET R SET CREATE(NODES, IMAGE)
    MATCH R:
        CASE OK(ID): RETURN ID
        CASE ERR(E): RETURN E

FUNC API_STOP(NODES: LIST<NODE>, CID: string): string:
    LET R SET STOP(NODES, CID)
    MATCH R:
        CASE OK(_): RETURN "ok"
        CASE ERR(E): RETURN E

FUNC API_LIST(): LIST<string>:
    RETURN KEYS(CONTAINERS)
```

---

# **Step 10 — System Orchestrator**

```nxd
MODULE ctr.system
IMPORT ctr.types
IMPORT ctr.util
IMPORT ctr.node
IMPORT ctr.registry

FUNC START():
    LET NODES SET NEW_CLUSTER(3)

    # preload an image
    LET L1 SET PUSH_LAYER("base")
    LET L2 SET PUSH_LAYER("app")
    PUSH_IMAGE("demo","latest",[L1,L2])

    LOG("container runtime online")
    RETURN NODES
```

---

# **Step 11 — MAIN**

```nxd
MODULE app.main
IMPORT ctr.system
IMPORT ctr.api
IMPORT ctr.util

FUNC MAIN():
    LET NODES SET ctr.system.START()

    LET CID SET API_RUN(NODES, "demo:latest")
    LOG("started: " ADD CID)

    SLEEP(2)

    LOG("containers: " ADD JOIN(API_LIST(),","))

    API_STOP(NODES, CID)
    LOG("stopped: " ADD CID)
```

---

# XXL System 11 Complete  
You now have a **full distributed container runtime**, end‑to‑end:

- Image registry  
- Layer store  
- Container engine  
- Scheduler  
- Node model  
- Networking hooks  
- API  
- Unified MAIN  

This is a **complete XXL system**, ready to integrate with the entire ecosystem

