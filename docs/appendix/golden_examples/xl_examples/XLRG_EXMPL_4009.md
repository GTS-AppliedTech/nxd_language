## Distributed Threat Intelligence Pipeline  (ingest, normalize, enrich, correlate, channels, multi‑module, supervision)

### JSON‑LD Metadata
```jsonld
{
  "@context": "https://schema.nxd.dev/example",
  "example_id": "nxd_xl_09_threat_intel_pipeline",
  "category": "Security",
  "layer": "xl",
  "features": [
    "multi-module",
    "ingest",
    "normalization",
    "enrichment",
    "correlation",
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
MODULE ti.types

TYPE RESULT UNION { OK(any), ERR(string) }
TYPE OPTION UNION { SOME(any), NONE }

TYPE RAW_EVENT {
    SRC: string,
    PAYLOAD: string,
    TS: int
}

TYPE NORM_EVENT {
    SRC: string,
    TYPE: string,
    VALUE: string,
    TS: int
}

TYPE ENRICHED_EVENT {
    SRC: string,
    TYPE: string,
    VALUE: string,
    GEO: string,
    ASN: string,
    TS: int
}

TYPE ALERT {
    NAME: string,
    SEVERITY: string,
    MESSAGE: string,
    TS: int
}

TYPE CHANNEL<RAW_EVENT> { }
TYPE CHANNEL<NORM_EVENT> { }
TYPE CHANNEL<ENRICHED_EVENT> { }
TYPE CHANNEL<ALERT> { }
```



## MODULE: Ingest

```nxd
MODULE ti.ingest
IMPORT ti.types

FUNC EMIT(IN: CHANNEL<RAW_EVENT>, SRC: string, PAYLOAD: string):
    LET E SET RAW_EVENT {
        SRC: SRC,
        PAYLOAD: PAYLOAD,
        TS: NOW()
    }
    SEND E TO IN
```



## MODULE: Normalizer

```nxd
MODULE ti.normalize
IMPORT ti.types

FUNC PARSE(P: string): OPTION:
    # simple pattern: "type:value"
    IF NOT (P CONTAINS ":"):
        RETURN NONE

    LET PARTS SET SPLIT(P, ":")
    RETURN SOME({ TYPE: PARTS[0], VALUE: PARTS[1] })

FUNC NORMALIZE(IN: CHANNEL<RAW_EVENT>, OUT: CHANNEL<NORM_EVENT>):
    LOOP:
        LET R SET RECV IN
        LET P SET PARSE(R.PAYLOAD)

        MATCH P:
            CASE SOME(D):
                LET N SET NORM_EVENT {
                    SRC: R.SRC,
                    TYPE: D.TYPE,
                    VALUE: D.VALUE,
                    TS: R.TS
                }
                SEND N TO OUT
            CASE NONE:
                NONE
```



## MODULE: Enrichment

```nxd
MODULE ti.enrich
IMPORT ti.types

FUNC GEOLOOKUP(V: string): string:
    # fake geo lookup
    IF V CONTAINS "192.168":
        RETURN "internal"
    RETURN "unknown"

FUNC ASNLOOKUP(V: string): string:
    # fake ASN lookup
    IF V CONTAINS "192.168":
        RETURN "AS-LOCAL"
    RETURN "AS-EXT"

FUNC ENRICH(IN: CHANNEL<NORM_EVENT>, OUT: CHANNEL<ENRICHED_EVENT>):
    LOOP:
        LET N SET RECV IN
        LET G SET GEOLOOKUP(N.VALUE)
        LET A SET ASNLOOKUP(N.VALUE)

        LET E SET ENRICHED_EVENT {
            SRC: N.SRC,
            TYPE: N.TYPE,
            VALUE: N.VALUE,
            GEO: G,
            ASN: A,
            TS: N.TS
        }

        SEND E TO OUT
```



## MODULE: Correlation Window

```nxd
MODULE ti.window
IMPORT ti.types

TYPE WINDOW { ITEMS: LIST<ENRICHED_EVENT> }

FUNC NEW_WINDOW(): WINDOW:
    RETURN WINDOW { ITEMS: [] }

FUNC ADD(W: WINDOW, E: ENRICHED_EVENT):
    PUSH W.ITEMS, E

FUNC TRIM(W: WINDOW, AGE: int):
    LET NOW SET NOW()
    LET NEW SET []
    LOOP X IN W.ITEMS:
        IF NOW SUB X.TS LT AGE:
            PUSH NEW, X
    W.ITEMS SET NEW

FUNC FIND(W: WINDOW, TYPE: string): LIST<ENRICHED_EVENT>:
    LET OUT SET []
    LOOP X IN W.ITEMS:
        IF X.TYPE EQ TYPE:
            PUSH OUT, X
    RETURN OUT
```



## MODULE: Correlation Rules

```nxd
MODULE ti.rules
IMPORT ti.types
IMPORT ti.window

FUNC RULE_PORTSCAN(W: WINDOW): OPTION:
    LET HITS SET FIND(W, "port")
    IF LEN(HITS) GT 10:
        RETURN SOME(ALERT {
            NAME: "portscan",
            SEVERITY: "high",
            MESSAGE: "multiple port hits detected",
            TS: NOW()
        })
    RETURN NONE

FUNC RULE_INTERNAL_ANOMALY(W: WINDOW): OPTION:
    LET INT SET []
    LOOP X IN W.ITEMS:
        IF X.GEO EQ "internal":
            PUSH INT, X

    IF LEN(INT) GT 5:
        RETURN SOME(ALERT {
            NAME: "internal-anomaly",
            SEVERITY: "medium",
            MESSAGE: "excessive internal activity",
            TS: NOW()
        })

    RETURN NONE

FUNC RULE_ASN_MISMATCH(W: WINDOW): OPTION:
    LET EXT SET []
    LOOP X IN W.ITEMS:
        IF X.ASN EQ "AS-EXT":
            PUSH EXT, X

    IF LEN(EXT) GT 3:
        RETURN SOME(ALERT {
            NAME: "asn-mismatch",
            SEVERITY: "critical",
            MESSAGE: "external ASN activity detected",
            TS: NOW()
        })

    RETURN NONE
```



## MODULE: Correlator

```nxd
MODULE ti.correlate
IMPORT ti.types
IMPORT ti.window
IMPORT ti.rules

FUNC CORRELATE(IN: CHANNEL<ENRICHED_EVENT>, OUT: CHANNEL<ALERT>):
    LET W SET NEW_WINDOW()

    LOOP:
        LET E SET RECV IN
        ADD(W, E)
        TRIM(W, 5000)

        LET A1 SET RULE_PORTSCAN(W)
        LET A2 SET RULE_INTERNAL_ANOMALY(W)
        LET A3 SET RULE_ASN_MISMATCH(W)

        MATCH A1:
            CASE SOME(A): SEND A TO OUT
            CASE NONE: NONE

        MATCH A2:
            CASE SOME(A): SEND A TO OUT
            CASE NONE: NONE

        MATCH A3:
            CASE SOME(A): SEND A TO OUT
            CASE NONE: NONE
```



## MODULE: Sink

```nxd
MODULE ti.sink
IMPORT ti.types

FUNC PRINT_ALERTS(CH: CHANNEL<ALERT>):
    LOOP:
        LET A SET RECV CH
        PRINTLN("[ALERT] " ADD A.NAME ADD " (" ADD A.SEVERITY ADD ") " ADD A.MESSAGE)
```



## MODULE: App Main

```nxd
MODULE app.main
IMPORT ti.types
IMPORT ti.ingest
IMPORT ti.normalize
IMPORT ti.enrich
IMPORT ti.correlate
IMPORT ti.sink

FUNC MAIN():
    LET RAW SET CHANNEL<RAW_EVENT>()
    LET NORM SET CHANNEL<NORM_EVENT>()
    LET ENR SET CHANNEL<ENRICHED_EVENT>()
    LET OUT SET CHANNEL<ALERT>()

    SPAWN NORMALIZE(RAW, NORM)
    SPAWN ENRICH(NORM, ENR)
    SPAWN CORRELATE(ENR, OUT)
    SPAWN PRINT_ALERTS(OUT)

    # simulate threat intel events
    LOOP I FROM 1 TO 15:
        EMIT(RAW, "scanner", "port:192.168.1." ADD I)
        SLEEP(0.1)

    LOOP J FROM 1 TO 8:
        EMIT(RAW, "internal", "sys:192.168.1." ADD J)
        SLEEP(0.1)

    LOOP K FROM 1 TO 5:
        EMIT(RAW, "external", "port:8.8.8." ADD K)
        SLEEP(0.1)

    SLEEP(3)

    RETURN NONE
```



# What this XL example demonstrates

### Multi‑module threat intelligence architecture  
- Ingest  
- Normalize  
- Enrich  
- Window  
- Rules  
- Correlator  
- Sink  
- App main

### Full pipeline  
RAW → NORM → ENRICHED → CORRELATED → ALERTS

### Enrichment  
- Geo lookup  
- ASN lookup  
- Type/value extraction

### Correlation rules  
- Portscan detection  
- Internal anomaly detection  
- ASN mismatch detection  
- Multi‑rule evaluation

### Channels + processes  
- Fully asynchronous  
- Real distributed‑system semantics

### Result + Option  
- Optional rule matches  
- Safe evaluation  
- Pattern matching

This is a **full subsystem**, suitable for real NXD agent training and backend mapping.

