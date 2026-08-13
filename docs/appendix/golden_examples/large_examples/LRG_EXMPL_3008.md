# JOB SCHEDULER (TASKS + CHANNELS + TIMERS + RESULT)

### JSON-LD Metadata
```jsonld
{
  "@context": "https://schema.nxd.dev/example",
  "example_id": "nxd_large_08_scheduler",
  "category": "Concurrency",
  "layer": "large",
  "features": ["tasks", "channels", "timers", "result"]
}
```


### NXD
```nxd
MODULE sched.core

TYPE JOB { NAME: string, DELAY: int }
TYPE RESULT UNION { OK(any), ERR(string) }
TYPE CHANNEL<JOB> { }

FUNC RUN_JOB(J):
    PRINTLN("running: " ADD J.NAME)
    RETURN OK("done")

FUNC SCHEDULER(CH):
    LOOP:
        LET J SET RECV CH
        SLEEP(J.DELAY)
        LET T SET TASK(fn() => RUN_JOB(J))
        LET OUT SET AWAIT T
        PRINTLN(OUT)


MODULE app.main
IMPORT sched.core

FUNC MAIN():
    LET CH SET CHANNEL<JOB>()

    SPAWN SCHEDULER(CH)

    SEND JOB { NAME: "backup", DELAY: 1 } TO CH
    SEND JOB { NAME: "cleanup", DELAY: 2 } TO CH

    RETURN NONE
```