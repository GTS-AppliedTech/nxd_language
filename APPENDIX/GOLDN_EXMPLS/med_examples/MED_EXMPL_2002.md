# CONCURRANCY PIPELINE (PRODUCER -> WORKER -> SINK)

### JSON-LD Metadata
```jsonld
{
  "@context": "https://schema.nxd.dev/example",
  "example_id": "nxd_medium_02_concurrency_pipeline",
  "category": "Concurrency",
  "layer": "medium",
  "features": ["process", "channel", "pipeline", "spawn", "recv", "send"]
}
```


### NXD
```nxd
MODULE pipeline.concurrent

TYPE CHANNEL<int> { }

FUNC PRODUCER(CH):
    LOOP I FROM 1 TO 5:
        SEND I TO CH
    RETURN NONE

FUNC WORKER(IN, OUT):
    LOOP:
        LET V SET RECV IN
        LET R SET V MUL 2
        SEND R TO OUT
    RETURN NONE

FUNC SINK(CH):
    LOOP J FROM 1 TO 5:
        LET V SET RECV CH
        PRINTLN(V)
    RETURN NONE

FUNC MAIN():
    LET C1 SET CHANNEL<int>()
    LET C2 SET CHANNEL<int>()

    SPAWN PRODUCER(C1)
    SPAWN WORKER(C1, C2)
    SPAWN SINK(C2)

    RETURN NONE
```