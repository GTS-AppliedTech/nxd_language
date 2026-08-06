# DISTRIBUTED JOB ORCHESTRATION (MULTI-SERVICE, MULTI-MODULE, SUPERVISION)

### JSON-LD Metadata
```jsonld
{
  "@context": "https://schema.nxd.dev/example",
  "example_id": "nxd_xl_01_distributed_jobs",
  "category": "Orchestration",
  "layer": "xl",
  "features": [
    "multi-module",
    "services",
    "channels",
    "processes",
    "tasks",
    "result",
    "supervision",
    "logging"
  ]
}
```


### NXD
```nxd
MODULE core.types

TYPE RESULT UNION { OK(any), ERR(string) }
TYPE OPTION UNION { SOME(any), NONE }

TYPE JOB {
    ID: int
    NAME: string
    PAYLOAD: string
}

TYPE JOB_STATUS {
    ID: int
    STATE: string
    MESSAGE: string
}

TYPE CHANNEL<JOB> { }
TYPE CHANNEL<JOB_STATUS> { }


MODULE core.registry
IMPORT core.types

LET JOBS SET MAP<int, JOB> {}
LET STATUSES SET MAP<int, JOB_STATUS> {}

FUNC REGISTER(J: JOB):
    JOBS[J.ID] SET J
    STATUSES[J.ID] SET JOB_STATUS { ID: J.ID, STATE: "queued", MESSAGE: "" }

FUNC UPDATE_STATUS(ID: int, STATE: string, MSG: string):
    IF STATUSES HAS ID:
        LET S SET STATUSES[ID]
        LET NEW SET JOB_STATUS { ID: S.ID, STATE: STATE, MESSAGE: MSG }
        STATUSES[ID] SET NEW

FUNC GET_STATUS(ID: int): OPTION:
    IF STATUSES HAS ID:
        RETURN SOME(STATUSES[ID])
    RETURN NONE

FUNC DUMP_STATUSES():
    LOOP K IN KEYS(STATUSES):
        LET S SET STATUSES[K]
        PRINTLN("job " ADD S.ID ADD " -> " ADD S.STATE ADD " (" ADD S.MESSAGE ADD ")")
    RETURN NONE


MODULE core.logger

FUNC LOG(MSG: string):
    PRINTLN("[LOG] " ADD MSG)

FUNC ERROR(MSG: string):
    PRINTLN("[ERR] " ADD MSG)


MODULE worker.exec
IMPORT core.types
IMPORT core.registry
IMPORT core.logger

FUNC PROCESS_JOB(J: JOB): RESULT:
    LOG("processing job " ADD J.ID ADD " (" ADD J.NAME ADD ")")

    IF J.PAYLOAD EQ "":
        UPDATE_STATUS(J.ID, "failed", "empty payload")
        ERROR("job " ADD J.ID ADD " failed: empty payload")
        RETURN ERR("empty payload")

    IF J.PAYLOAD CONTAINS "fail":
        UPDATE_STATUS(J.ID, "failed", "payload requested failure")
        ERROR("job " ADD J.ID ADD " failed: payload requested failure")
        RETURN ERR("payload failure")

    UPDATE_STATUS(J.ID, "running", "")
    LOG("job " ADD J.ID ADD " running")

    # simulate work
    SLEEP(1)

    UPDATE_STATUS(J.ID, "done", "ok")
    LOG("job " ADD J.ID ADD " done")

    RETURN OK("done")


MODULE worker.supervisor
IMPORT core.types
IMPORT core.registry
IMPORT core.logger
IMPORT worker.exec

FUNC SUPERVISE(IN: CHANNEL<JOB>, OUT: CHANNEL<JOB_STATUS>):
    LOOP:
        LET J SET RECV IN
        REGISTER(J)

        LET T SET TASK(fn() => PROCESS_JOB(J))

        LET R SET TRY AWAIT T CATCH E:
            ERROR("worker crashed for job " ADD J.ID ADD ": " ADD E)
            UPDATE_STATUS(J.ID, "crashed", E)
            RETURN ERR("crashed: " ADD E)

        MATCH R:
            CASE OK(_):
                LET S SET JOB_STATUS { ID: J.ID, STATE: "done", MESSAGE: "ok" }
                SEND S TO OUT
            CASE ERR(E):
                LET S SET JOB_STATUS { ID: J.ID, STATE: "failed", MESSAGE: E }
                SEND S TO OUT


MODULE api.submit
IMPORT core.types
IMPORT core.logger

FUNC BUILD_JOB(ID: int, NAME: string, PAYLOAD: string): JOB:
    RETURN JOB { ID: ID, NAME: NAME, PAYLOAD: PAYLOAD }

FUNC SUBMIT(JOBS: CHANNEL<JOB>, ID: int, NAME: string, PAYLOAD: string):
    LET J SET BUILD_JOB(ID, NAME, PAYLOAD)
    LOG("submitting job " ADD ID ADD " (" ADD NAME ADD ")")
    SEND J TO JOBS


MODULE api.query
IMPORT core.types
IMPORT core.registry

FUNC STATUS(ID: int): RESULT:
    MATCH GET_STATUS(ID):
        CASE SOME(S):
            RETURN OK(S)
        CASE NONE:
            RETURN ERR("unknown job")


MODULE app.main
IMPORT core.types
IMPORT core.registry
IMPORT core.logger
IMPORT worker.supervisor
IMPORT api.submit
IMPORT api.query

FUNC MAIN():
    LET JOBS SET CHANNEL<JOB>()
    LET STAT_CH SET CHANNEL<JOB_STATUS>()

    SPAWN SUPERVISE(JOBS, STAT_CH)

    SUBMIT(JOBS, 1, "backup", "run backup")
    SUBMIT(JOBS, 2, "cleanup", "run cleanup")
    SUBMIT(JOBS, 3, "test-fail", "please fail")

    LOOP I FROM 1 TO 3:
        LET S SET RECV STAT_CH
        PRINTLN("status event: job " ADD S.ID ADD " -> " ADD S.STATE ADD " (" ADD S.MESSAGE ADD ")")

    DUMP_STATUSES()

    PRINTLN(STATUS(1))
    PRINTLN(STATUS(2))
    PRINTLN(STATUS(3))
    PRINTLN(STATUS(999))

    RETURN NONE
```