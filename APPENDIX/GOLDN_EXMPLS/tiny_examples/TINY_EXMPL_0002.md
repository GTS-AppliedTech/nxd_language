

# Golden Example #2 — Concurrency (Tiny Layer)

### JSON‑LD Metadata
```jsonld
{
  "@context": "https://schema.nxd.dev/example",
  "example_id": "nxd_concurrency_01",
  "category": "Concurrency",
  "layer": "tiny",
  "features": ["process", "task", "channel", "spawn", "send", "recv", "await"],
  "backend_targets": ["nim", "elixir", "d"]
}
```



# Problem Statement
Demonstrate NXD’s core concurrency primitives:

- `SPAWN` to create a process  
- `CHANNEL<T>` for typed message passing  
- `SEND` and `RECV` for communication  
- `TASK(FN)` and `AWAIT` for asynchronous work  

This example shows a spawned worker process sending a computed value back to the main process through a channel.



# Canonical NXD (≤30 lines)
```nxd
MODULE demo.concurrency

TYPE CHANNEL<int> { }

FUNC WORK(CH):
    LET VALUE SET 42
    SEND VALUE TO CH
    RETURN none

FUNC MAIN():
    LET CH SET CHANNEL<int>()
    SPAWN WORK(CH)

    LET RESULT SET RECV CH
    PRINTLN(RESULT)

    LET T SET TASK(fn() => 100 ADD 23)
    LET OUT SET AWAIT T
    PRINTLN(OUT)

    RETURN none
```



# Semantic Notes
- `CHANNEL<int>()` creates a typed channel.  
- `SPAWN WORK(CH)` creates a new process running `WORK`.  
- `SEND VALUE TO CH` enqueues a message in FIFO order.  
- `RECV CH` blocks until a message arrives.  
- `TASK(fn() => ...)` creates a managed asynchronous task.  
- `AWAIT T` returns the task’s result in completion order.  
- No shared mutable state is used — pure message passing.  
- All evaluation is left‑to‑right, per operational semantics.



# Backend Outputs

### Nim
```nim
import asyncdispatch

proc work(ch: Channel[int]) =
  let value = 42
  ch.send(value)

proc main() =
  let ch = newChannel[int]()
  spawn work(ch)
  let result = ch.recv()
  echo result

  let t = asyncProc:
    return 100 + 23

  let out = waitFor t()
  echo out
```



### Elixir
```elixir
defmodule Demo.Concurrency do
  def work(ch) do
    send(ch, 42)
  end

  def main() do
    ch = self()
    spawn(fn -> work(ch) end)

    receive do
      result -> IO.puts(result)
    end

    task = Task.async(fn -> 100 + 23 end)
    out = Task.await(task)
    IO.puts(out)
  end
end
```



### D
```d
module demo.concurrency;

import std.concurrency;
import std.stdio;

void work(Tid ch) {
    send(ch, 42);
}

void main() {
    auto ch = thisTid;
    spawn(&work, ch);

    int result = receiveOnly!int();
    writeln(result);

    auto t = async!(() => 100 + 23);
    auto out = t.get();
    writeln(out);
}
```



# Audit Rules
- `SPAWN` must create an isolated process.  
- `SEND`/`RECV` must preserve FIFO ordering.  
- Channels must enforce type correctness (`int`).  
- `TASK` must return a value and be awaitable.  
- Backend lowering must preserve message‑passing semantics.  
- No implicit conversions allowed in any message.  

