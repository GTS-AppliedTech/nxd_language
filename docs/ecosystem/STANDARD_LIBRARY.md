# STANDARD_LIBRARY.md


# High‑level layout

Module	Purpose	
CORE	primitives, math, collections	
STRING	text utilities	
LIST	list ops, higher‑order functions	
MAP	maps, dictionaries	
OPTION	some/none handling	
RESULT	ok/err flow	
CONCURRENT	processes, channels, tasks	
IO	basic input/output	
TIME	time, duration, scheduling	
UTIL	misc helpers	



1. `CORE`

• Types: int, float, bool, string, none.
• Math: ADD, SUB, MUL, DIV, MOD, ABS, MIN, MAX.
• Comparison: EQ, NEQ, GT, LT, GTE, LTE.


Example:

MODULE CORE

FUNC ABS(X):
    IF X LT 0:
        RETURN 0 SUB X
    OTHERWISE:
        RETURN X



2. `STRING`

• Funcs: TRIM, SPLIT, JOIN, UPPER, LOWER, REPLACE, CONTAINS.


FUNC TRIM(S): ...
FUNC SPLIT(S, SEP): ...



3. `LIST`

• Core: LEN, PUSH, POP, HEAD, TAIL.
• Functional: MAP, FILTER, REDUCE.


FUNC MAP(L, FN): ...
FUNC FILTER(L, FN): ...



4. `MAP`

• Ops: GET, SET, HAS, REMOVE, KEYS, VALUES.


FUNC GET(M, KEY): ...
FUNC SET(M, KEY, VALUE): ...



5. `OPTION`

• Type:


TYPE OPTION UNION { SOME(any), NONE }


• Helpers: IS_SOME, IS_NONE, UNWRAP_OR.



6. `RESULT`

• Type:


TYPE RESULT UNION { OK(any), ERR(string) }


• Helpers: IS_OK, IS_ERR, MAP_OK, MAP_ERR.



7. `CONCURRENT`

• Primitives: SPAWN, SEND, RECV, AWAIT, CHANNEL.


TYPE CHANNEL { ... }

FUNC SPAWN(FN): ...
FUNC SEND(CH, MSG): ...
FUNC RECV(CH): ...


Backends:

• Nim: async/threads.
• Elixir: processes/channels.
• D: threads/fibers.



8. `IO`

• Funcs: PRINT, PRINTLN, READ_LINE.


FUNC PRINTLN(S): ...



9. `TIME`

• Types: DURATION, INSTANT.
• Funcs: NOW, SLEEP, AFTER.



10. `UTIL`

• Helpers: DEBUG, ASSERT, RANGE, ID.



This standard library gives NXD:

• a coherent core,
• functional tools,
• concurrency primitives,
• error/option handling,
• and a clean surface for agents to document and teach.

