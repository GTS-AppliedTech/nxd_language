# DISTRIBUTED TASK QUEUE (PROCESS + CHANNELS + TASKS + RESULT + OPTION)

### JSON-LD Metadata
```jsonld
{
  "@context": "https://schema.nxd.dev/example",
  "example_id": "nxd_large_01_task_queue",
  "category": "Concurrency",
  "layer": "large",
  "features": ["process", "task", "channels", "result", "option", "supervision"]
}
```


### NXD
```nxd
MODULE system.queue

TYPE JOB { ID: int, PAYLOAD: string }
TYPE RESULT UNION { OK(any), ERR(string) }
TYPE OPTION UNION { SOME(any), NONE }

TYPE CHANNEL<JOB> { }
TYPE CHANNEL<RESULT> { }

# Supervisor monitors workers and restarts them on failure
FUNC SUPERVISOR(JOBS, RESULTS):
    LOOP:
        LET JOB SET RECV JOBS
        LET T SET TASK(fn() => PROCESS_JOB(JOB))
        LET OUT SET TRY AWAIT T CATCH E:
            RETURN ERR("worker crashed: " ADD E)
        SEND OUT TO RESULTS

# Worker logic
FUNC PROCESS_JOB(J):
    IF J.PAYLOAD EQ "":
        RETURN ERR("empty payload")
    RETURN OK("processed: " ADD J.PAYLOAD)

# Main queue system
FUNC MAIN():
    LET JOBS SET CHANNEL<JOB>()
    LET RESULTS SET CHANNEL<RESULT>()

    SPAWN SUPERVISOR(JOBS, RESULTS)

    LOOP I FROM 1 TO 5:
        LET J SET JOB { ID: I, PAYLOAD: "task_" ADD I }
        SEND J TO JOBS

    LOOP K FROM 1 TO 5:
        LET R SET RECV RESULTS
        MATCH R:
            CASE OK(V): PRINTLN(V)
            CASE ERR(E): PRINTLN("error: " ADD E)

    RETURN NONE
```