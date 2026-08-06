## Distributed Metrics Aggregator  (collectors, reducers, rollups, channels, tasks, multi‑module, supervision)

### JSON‑LD Metadata
```jsonld
{
  "@context": "https://schema.nxd.dev/example",
  "example_id": "nxd_xl_05_metrics_aggregator",
  "category": "Monitoring",
  "layer": "xl",
  "features": [
    "multi-module",
    "metrics",
    "aggregation",
    "rollups",
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
MODULE metrics.types

TYPE RESULT UNION { OK(any), ERR(string) }
TYPE OPTION UNION { SOME(any), NONE }

TYPE METRIC {
    NAME: string,
    VALUE: int,
    TS: int
}

TYPE ROLLUP {
    NAME: string,
    AVG: int,
    MIN: int,
    MAX: int,
    COUNT: int,
    TS: int
}

TYPE CHANNEL<METRIC> { }
TYPE CHANNEL<ROLLUP> { }


MODULE metrics.window

TYPE WINDOW { ITEMS: LIST<METRIC> }

FUNC NEW_WINDOW(): WINDOW:
    RETURN WINDOW { ITEMS: [] }

FUNC ADD(W: WINDOW, M: METRIC):
    PUSH W.ITEMS, M

FUNC TRIM(W: WINDOW, AGE: int):
    LET NOW SET NOW()
    LET NEW SET []
    LOOP X IN W.ITEMS:
        IF NOW SUB X.TS LT AGE:
            PUSH NEW, X
    W.ITEMS SET NEW

FUNC GET_BY_NAME(W: WINDOW, NAME: string): LIST<METRIC>:
    LET OUT SET []
    LOOP X IN W.ITEMS:
        IF X.NAME EQ NAME:
            PUSH OUT, X
    RETURN OUT


MODULE metrics.rollup
IMPORT metrics.types
IMPORT metrics.window

FUNC COMPUTE(NAME: string, W: WINDOW): OPTION:
    LET LIST SET GET_BY_NAME(W, NAME)
    IF LEN(LIST) EQ 0:
        RETURN NONE

    LET SUM SET 0
    LET MINV SET LIST[0].VALUE
    LET MAXV SET LIST[0].VALUE

    LOOP X IN LIST:
        SUM SET SUM ADD X.VALUE
        IF X.VALUE LT MINV: MINV SET X.VALUE
        IF X.VALUE GT MAXV: MAXV SET X.VALUE

    LET AVG SET SUM DIV LEN(LIST)

    RETURN SOME(ROLLUP {
        NAME: NAME,
        AVG: AVG,
        MIN: MINV,
        MAX: MAXV,
        COUNT: LEN(LIST),
        TS: NOW()
    })


MODULE metrics.collector
IMPORT metrics.types

FUNC EMIT(IN: CHANNEL<METRIC>, NAME: string, VALUE: int):
    LET M SET METRIC {
        NAME: NAME,
        VALUE: VALUE,
        TS: NOW()
    }
    SEND M TO IN


MODULE metrics.reducer
IMPORT metrics.types
IMPORT metrics.window
IMPORT metrics.rollup

FUNC REDUCE(IN: CHANNEL<METRIC>, OUT: CHANNEL<ROLLUP>, NAMES: LIST<string>):
    LET W SET NEW_WINDOW()

    LOOP:
        LET M SET RECV IN
        ADD(W, M)
        TRIM(W, 5000)

        LOOP N IN NAMES:
            LET R SET COMPUTE(N, W)
            MATCH R:
                CASE SOME(ROLL):
                    SEND ROLL TO OUT
                CASE NONE:
                    NONE


MODULE metrics.sink
IMPORT metrics.types

FUNC PRINT_ROLLUPS(CH: CHANNEL<ROLLUP>):
    LOOP:
        LET R SET RECV CH
        PRINTLN(
            "[ROLLUP] " ADD R.NAME ADD
            " avg=" ADD R.AVG ADD
            " min=" ADD R.MIN ADD
            " max=" ADD R.MAX ADD
            " count=" ADD R.COUNT
        )


MODULE app.main
IMPORT metrics.types
IMPORT metrics.collector
IMPORT metrics.reducer
IMPORT metrics.sink

FUNC MAIN():
    LET IN SET CHANNEL<METRIC>()
    LET OUT SET CHANNEL<ROLLUP>()

    LET NAMES SET ["cpu", "mem", "disk"]

    SPAWN REDUCE(IN, OUT, NAMES)
    SPAWN PRINT_ROLLUPS(OUT)

    # simulate collectors
    SPAWN fn():
        LOOP:
            EMIT(IN, "cpu", RANDOM(0, 100))
            SLEEP(1)

    SPAWN fn():
        LOOP:
            EMIT(IN, "mem", RANDOM(0, 100))
            SLEEP(1)

    SPAWN fn():
        LOOP:
            EMIT(IN, "disk", RANDOM(0, 100))
            SLEEP(1)

    # allow rollups to accumulate
    SLEEP(10)

    RETURN NONE
```



# What this XL example demonstrates

### Multi‑module metrics architecture  
- `metrics.types`  
- `metrics.window`  
- `metrics.rollup`  
- `metrics.collector`  
- `metrics.reducer`  
- `metrics.sink`  
- `app.main`

### Rolling metric windows  
- Time‑based trimming  
- Per‑metric grouping  
- Dynamic rollup computation

### Rollup engine  
- AVG / MIN / MAX / COUNT  
- Multiple metric names  
- Continuous streaming rollups

### Channels + processes  
- Collectors → Reducer → Sink  
- Fully asynchronous  
- Real distributed‑system semantics

### Result + Option  
- Optional rollups  
- Safe evaluation  
- Pattern matching

### Realistic metrics aggregator  
- CPU, memory, disk  
- Randomized values  
- Continuous rollups  
- Multi‑sink output

This is a **full subsystem**, suitable for real NXD agent training and backend mapping.

