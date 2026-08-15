

# Golden Example #3 — Channels (Tiny Layer)

### JSON‑LD Metadata
```jsonld
{
  "@context": "https://schema.nxd.dev/example",
  "example_id": "nxd_channels_01",
  "category": "Channels",
  "layer": "tiny",
  "features": ["channel", "send", "recv", "fifo-ordering", "process"],
  "backend_targets": ["nim", "elixir", "d"]
}
```



# Problem Statement
Demonstrate NXD’s typed channel semantics:

- Creating a channel  
- Sending multiple messages  
- Receiving them in FIFO order  
- Using a worker process to push values  
- Showing that channels enforce type correctness  

This example is intentionally small but captures the core behavior of NXD channels.



# Canonical NXD (≤30 lines)
```nxd
MODULE demo.channels

TYPE CHANNEL<int> { }

FUNC PRODUCER(CH):
    SEND 1 TO CH
    SEND 2 TO CH
    SEND 3 TO CH
    RETURN none

FUNC MAIN():
    LET CH SET CHANNEL<int>()
    SPAWN PRODUCER(CH)

    LET A SET RECV CH
    LET B SET RECV CH
    LET C SET RECV CH

    PRINTLN(A)
    PRINTLN(B)
    PRINTLN(C)

    RETURN none
```



# Semantic Notes
- `CHANNEL<int>()` creates a typed FIFO channel.  
- `SEND` enqueues messages in strict FIFO order (per operational semantics).  
- `RECV` blocks until a message is available.  
- `SPAWN PRODUCER(CH)` creates a new process that pushes values.  
- All messages must be `int` — type mismatch is a compile‑time error.  
- Evaluation order is left‑to‑right.  
- No shared mutable state is used; pure message passing.



# Backend Outputs

### Nim
```nim
import asyncdispatch

proc producer(ch: Channel[int]) =
  ch.send(1)
  ch.send(2)
  ch.send(3)

proc main() =
  let ch = newChannel[int]()
  spawn producer(ch)

  let a = ch.recv()
  let b = ch.recv()
  let c = ch.recv()

  echo a
  echo b
  echo c
```



### Elixir
```elixir
defmodule Demo.Channels do
  def producer(ch) do
    send(ch, 1)
    send(ch, 2)
    send(ch, 3)
  end

  def main() do
    ch = self()
    spawn(fn -> producer(ch) end)

    a = receive do x -> x end
    b = receive do x -> x end
    c = receive do x -> x end

    IO.puts(a)
    IO.puts(b)
    IO.puts(c)
  end
end
```



### D
```d
module demo.channels;

import std.concurrency;
import std.stdio;

void producer(Tid ch) {
    send(ch, 1);
    send(ch, 2);
    send(ch, 3);
}

void main() {
    auto ch = thisTid;
    spawn(&producer, ch);

    int a = receiveOnly!int();
    int b = receiveOnly!int();
    int c = receiveOnly!int();

    writeln(a);
    writeln(b);
    writeln(c);
}
```



# Audit Rules
- Channel type must be enforced (`int`).  
- FIFO ordering must be preserved across all backends.  
- `SEND`/`RECV` must map to backend message‑passing primitives.  
- No implicit conversions allowed in messages.  
- `SPAWN` must create an isolated process.  
