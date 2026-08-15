---
{
  "@context": "https://nxdlang.org/schema",
  "doc_id": "",
  "title": "",
  "description": "",
  "layer": "",
  "category": "",
  "keywords": [],
  "doc_version": "",
  "status": "active"
}
---

# CB001 COOKBOOK_APPENDIX


## APPENDIX A

A.1 Result module

NXD surface

MODULE RESULT

TYPE RESULT UNION { OK(any), ERR(string) }

FUNC OK(V):
    RETURN OK(V)

FUNC ERR(MSG):
    RETURN ERR(MSG)

FUNC IS_OK(R):
    MATCH R:
        CASE OK(_):
            RETURN true
        OTHERWISE:
            RETURN false

FUNC IS_ERR(R):
    MATCH R:
        CASE ERR(_):
            RETURN true
        OTHERWISE:
            RETURN false

FUNC UNWRAP_OR(R, DEFAULT):
    MATCH R:
        CASE OK(V):
            RETURN V
        CASE ERR(_):
            RETURN DEFAULT

FUNC MAP_OK(R, FN):
    MATCH R:
        CASE OK(V):
            RETURN OK(FN(V))
        CASE ERR(MSG):
            RETURN ERR(MSG)



Nim mapping

Idiomatic Nim uses Result[T]:

type
  Result[T] = object
    isOk: bool
    ok: T
    err: string

proc ok[T](v: T): Result[T] =
  Result[T](isOk: true, ok: v, err: "")

proc err[T](msg: string): Result[T] =
  Result[T](isOk: false, err: msg)

proc isOk[T](r: Result[T]): bool =
  r.isOk

proc isErr[T](r: Result[T]): bool =
  not r.isOk

proc unwrapOr[T](r: Result[T], default: T): T =
  if r.isOk: r.ok else: default

proc mapOk[T, U](r: Result[T], fn: proc(x: T): U): Result[U] =
  if r.isOk:
    ok(fn(r.ok))
  else:
    err[U](r.err)



Elixir mapping

Use tagged tuples:

defmodule Result do
  def ok(v), do: {:ok, v}
  def err(msg), do: {:error, msg}

  def is_ok({:ok, _}), do: true
  def is_ok(_),        do: false

  def is_err({:error, _}), do: true
  def is_err(_),           do: false

  def unwrap_or({:ok, v}, _default), do: v
  def unwrap_or({:error, _}, default), do: default

  def map_ok({:ok, v}, fn), do: {:ok, fn.(v)}
  def map_ok({:error, msg}, _fn), do: {:error, msg}
end



D mapping

Use a templated Result:

module result;

struct Result(T) {
    bool isOk;
    T ok;
    string err;
}

Result!T ok(T)(T v) {
    return Result!T(true, v, "");
}

Result!T err(T)(string msg) {
    Result!T r;
    r.isOk = false;
    r.err = msg;
    return r;
}

bool isOk(T)(Result!T r) {
    return r.isOk;
}

bool isErr(T)(Result!T r) {
    return !r.isOk;
}

T unwrapOr(T)(Result!T r, T defaultValue) {
    return r.isOk ? r.ok : defaultValue;
}

Result!U mapOk(T, U)(Result!T r, U function(T) fn) {
    if (r.isOk) {
        return ok!U(fn(r.ok));
    } else {
        return err!U(r.err);
    }
}



## A.2 Concurrent module

##NXD surface

MODULE CONCURRENT

TYPE CHANNEL(any)

FUNC MAKE_CHANNEL():
    RETURN CHANNEL()

FUNC SPAWN(FN):
    SPAWN FN()

FUNC SEND(CH, MSG):
    SEND MSG TO CH

FUNC RECV(CH):
    LET V SET RECV CH
    RETURN V

FUNC TASK(FN):
    RETURN SPAWN FN()

FUNC AWAIT(T):
    RETURN AWAIT T



Nim mapping

Use asyncdispatch or channels:

import asyncdispatch, channels

type
  Channel[T] = Channel[T]  # from channels module

proc makeChannel[T](): Channel[T] =
  newChannel[T]()

proc spawn*(fn: proc() {.async.}): Future[void] =
  asyncCheck fn()

proc send[T](ch: Channel[T], msg: T) =
  ch.send(msg)

proc recv[T](ch: Channel[T]): T =
  ch.recv()

proc task*(fn: proc() {.async.}): Future[void] =
  spawn(fn)

proc await*[T](f: Future[T]): T =
  waitFor f



Elixir mapping

Processes + message passing:

defmodule Concurrent do
  def make_channel do
    self()
  end

  def spawn(fn) do
    spawn(fn)
  end

  def send(ch, msg) do
    send(ch, msg)
  end

  def recv(_ch \\ self()) do
    receive do
      msg -> msg
    end
  end

  def task(fn) do
    Task.async(fn)
  end

  def await(task) do
    Task.await(task)
  end
end



D mapping

Threads + channels (e.g., std.concurrency):

module concurrent;

import std.concurrency;
import core.thread;

alias Channel(T) = Tid; // simplistic: a thread id as channel

Channel!T makeChannel(T)() {
    // in practice you'd spawn a worker that receives messages
    return thisTid;
}

void send(T)(Channel!T ch, T msg) {
    send(ch, msg);
}

T recv(T)() {
    auto m = receiveOnly!T();
    return m;
}

Thread spawn(void function() fn) {
    auto t = new Thread(fn);
    t.start();
    return t;
}

// task/await could wrap std.concurrency or futures libs








## Appendix B — Dynamic Trait Objects (`BOX<TRAIT>`)

Dynamic trait objects allow NXD to support runtime polymorphism in addition to compile‑time trait constraints.
They behave like interface objects or protocol dispatch in backends.



## B.1 Concept

A dynamic trait object is a runtime value that stores:

• a reference to a concrete value
• a vtable containing implementations of the trait’s functions


This enables:

• heterogeneous collections
• runtime dispatch
• plugin architectures
• dynamic capability wrappers



## B.2 Syntax

Trait definition

TRAIT SERIALIZABLE {
    FUNC TO_STRING(X): string
}


Type implementing trait

TYPE USER IMPLEMENTS SERIALIZABLE:
    NAME: string

FUNC TO_STRING(U: USER): string:
    RETURN "USER(" ADD U.NAME ADD ")"


Dynamic trait object creation

LET B SET BOX<SERIALIZABLE>(U)


Dynamic dispatch

FUNC LOG_OBJ(X: BOX<SERIALIZABLE>):
    PRINTLN(X.TO_STRING())



## B.3 Semantics

B.3.1 Type erasure

BOX<SERIALIZABLE> hides the concrete type.
Only trait methods are accessible.

B.3.2 Lifetime

A BOX<T> owns the underlying value unless constructed with BORROW.

B.3.3 Borrowed trait objects

LET B SET BOX<SERIALIZABLE>(BORROW U)


Borrowed trait objects do not own the underlying value.

B.3.4 Equality

Trait objects use identity equality, not value equality.



## B.4 Backend mapping

Nim

• Lowered to ref object with proc table
• Dynamic dispatch via manually constructed vtables


Elixir

• Lowered to protocol dispatch
• Struct stored directly; protocol resolution at runtime


D

• Lowered to interface + class or struct wrapper
• Vtable dispatch native



## B.5 Limitations

• Trait objects cannot be serialized unless the trait defines a serialization method.
• Trait objects cannot be sent through channels unless explicitly allowed.



##Appendix C — Ownership Patterns

NXD uses a lightweight ownership model compatible with Nim ARC/ORC, Elixir GC, and D RAII/GC.



## C.1 Owned Values

Owned values are created normally:

LET BUF SET MAKE_BUFFER()


The creator owns the value.



## C.2 Borrowed Values

Borrowing creates a non-owning reference:

USE_BUFFER(BORROW BUF)


Rules:

• Borrow is immutable
• Multiple borrows allowed
• Borrow does not extend lifetime beyond owner



## C.3 Move Semantics

Move transfers ownership:

LET BUF2 SET MOVE BUF


After move:

• BUF becomes invalid
• BUF2 owns the value



## C.4 Ownership Across Processes

Ownership transfer across processes must be explicit:

SEND MOVE BUF TO CH


Borrowing across processes is allowed but discouraged unless safe.



## Appendix D — Security Patterns

NXD’s security model is capability‑based.
These patterns show how to structure secure code.



## D.1 Capability‑Guarded API

Every sensitive function requires a capability:

FUNC READ_SECURE(PATH: string, CAP: CAP_FS_READ): string:
    RETURN READ_FILE(PATH)



## D.2 Least Privilege Process

Give each process only the capabilities it needs:

SPAWN WORKER(CAP_FS_READ)
SPAWN LOGGER(CAP_LOG_WRITE)



## D.3 Capability Delegation

Explicit delegation:

SEND CAP TO CH


Only allowed if capability is delegable.



## D.4 Capability Revocation

REVOKE(CAP)


Revocation is global for that capability instance.



## Appendix E — Package Design Patterns

These patterns help structure NXD packages for clarity and portability.



## E.1 Core + Backend Adapters

mypkg/
  core/
    types.nxd
    logic.nxd
  nim/
    runtime_shim.nxd
  elixir/
    runtime_shim.nxd
  d/
    runtime_shim.nxd


Rules:

• Core contains pure NXD logic
• Backend folders contain runtime shims
• No backend conditionals in core code



## E.2 Facade Pattern

Expose a stable API:

MODULE mypkg.api
IMPORT mypkg.core
IMPORT mypkg.nim


Backend differences hidden behind the facade.



##E.3 Capability‑Scoped Packages

Packages declare required capabilities:

[capabilities]
requires = ["FS_READ", "NET_CONNECT"]



## Appendix F — Backend Portability Patterns

These patterns ensure code runs consistently across Nim, Elixir, and D.



## F.1 Portability Profile

Each package declares its portability requirements:

[portability]
requires = ["RESULT", "OPTION", "CHANNEL"]



## F.2 Portable Subset

To maximize portability:

• Avoid UNSAFE
• Avoid backend‑specific concurrency primitives
• Use:• RESULT
• OPTION
• channels
• tasks
• traits



## F.3 Backend Capability Matrix

Feature	Nim	Elixir	D	
RESULT	native	emulated	native	
OPTION	native	emulated	native	
MOVE	ARC/ORC	hint	RAII/hint	
BORROW	hint	N/A	hint	
CHANNEL	native	wrapped	native	
UNSAFE	full	limited	full	
TRAITS	concepts	protocols	interfaces/templates	
INIT	static	@on_load	static this()	
PROCESS	threads	BEAM processes	threads/fibers	
TASK	futures	Task	std.concurrency	







## Appendix G — Cross‑Backend Examples

This appendix demonstrates how core NXD abstractions lower into Nim, Elixir, and D.
It is not normative; it is illustrative.



## G.1 Futures / Tasks Examples

(NXD TASK / AWAIT → Nim futures, Elixir Task, D std.concurrency)

NXD

FUNC WORK(X: int): int:
    RETURN X MUL 2

FUNC MAIN():
    LET T SET TASK(WORK, 21)
    LET R SET AWAIT T
    MATCH R:
        CASE OK(V): PRINTLN(V)
        CASE ERR(E): PRINTLN("error: " ADD E)



Nim

import asyncdispatch

proc work(x: int): Future[int] {.async.} =
  return x * 2

proc main() =
  let t = work(21)
  let r = waitFor t
  echo r



Elixir

def work(x), do: x * 2

def main() do
  t = Task.async(fn -> work(21) end)
  r = Task.await(t)
  IO.puts(r)
end



D

import std.concurrency;

int work(int x) {
    return x * 2;
}

void main() {
    auto tid = spawn(&work, 21);
    auto r = receiveOnly!int;
    writeln(r);
}



## G.2 Collections Examples

(LIST, MAP, mutation, iteration)

NXD

FUNC MAIN():
    LET L SET [1,2,3]
    L.PUSH(4)

    LET M SET { "x": 10, "y": 20 }
    M["z"] SET 30

    FOR V IN L:
        PRINTLN(V)

    FOR K,V IN M:
        PRINTLN(K ADD ":" ADD V)



Nim

var L = @[1,2,3]
L.add(4)

var M = {"x": 10, "y": 20}.toTable
M["z"] = 30

for v in L:
  echo v

for k,v in M:
  echo k, ":", v



Elixir

def main() do
  l = [1,2,3] ++ [4]
  m = %{"x" => 10, "y" => 20} |> Map.put("z", 30)

  Enum.each(l, &IO.puts/1)

  Enum.each(m, fn {k,v} ->
    IO.puts("#{k}:#{v}")
  end)
end



D

import std.stdio;
import std.array;
import std.algorithm;

void main() {
    auto L = [1,2,3];
    L ~= 4;

    int[string] M;
    M["x"] = 10;
    M["y"] = 20;
    M["z"] = 30;

    foreach(v; L)
        writeln(v);

    foreach(k,v; M)
        writeln(k, ":", v);
}



## G.3 Type Examples

(Structs, traits, generics, constraints)

NXD

TRAIT SERIALIZABLE {
    FUNC TO_STRING(X): string
}

TYPE POINT IMPLEMENTS SERIALIZABLE:
    X: int
    Y: int

FUNC TO_STRING(P: POINT): string:
    RETURN "(" ADD P.X ADD "," ADD P.Y ADD ")"

FUNC PRINT<T : SERIALIZABLE>(X: T):
    PRINTLN(TO_STRING(X))



Nim

type
  Point = object
    x, y: int

proc toString(p: Point): string =
  "(" & $p.x & "," & $p.y & ")"

proc print[T](x: T) =
  echo toString(x)



Elixir

defprotocol Serializable do
  def to_string(x)
end

defmodule Point do
  defstruct [:x, :y]
end

defimpl Serializable, for: Point do
  def to_string(%Point{x: x, y: y}), do: "(#{x},#{y})"
end

def print(x), do: IO.puts(Serializable.to_string(x))



D

interface Serializable {
    string toString();
}

struct Point {
    int x;
    int y;
    string toString() {
        return "(" ~ x.to!string ~ "," ~ y.to!string ~ ")";
    }
}

void print(T)(T x) if (is(T : Serializable)) {
    writeln(x.toString());
}



## G.4 Capability Examples

(Creation, delegation, revocation, use)

NXD

FUNC READ_SECURE(PATH: string, CAP: CAP_FS_READ): string:
    RETURN READ_FILE(PATH)

FUNC MAIN():
    LET CAP SET FS_READ_CAP()

    LET T SET TASK(READ_SECURE, "data.txt", CAP)
    LET R SET AWAIT T

    MATCH R:
        CASE OK(V): PRINTLN(V)
        CASE ERR(E): PRINTLN("error: " ADD E)

    REVOKE(CAP)



Nim

(Capabilities lowered to explicit parameters + runtime checks)

type FsReadCap = object

proc readSecure(path: string, cap: FsReadCap): string =
  readFile(path)

proc main() =
  let cap = FsReadCap()
  let t = async(readSecure("data.txt", cap))
  let r = waitFor t
  echo r



Elixir

(Capabilities lowered to explicit arguments + pattern checks)

def read_secure(path, :fs_read_cap) do
  File.read!(path)
end

def main() do
  cap = :fs_read_cap
  t = Task.async(fn -> read_secure("data.txt", cap) end)
  r = Task.await(t)
  IO.puts(r)
end



D

(Capabilities lowered to structs + runtime validation)

struct FsReadCap {}

string readSecure(string path, FsReadCap cap) {
    return import(path);
}

void main() {
    auto cap = FsReadCap();
    auto tid = spawn(&readSecure, "data.txt", cap);
    auto r = receiveOnly!string;
    writeln(r);
}



## Appendix G.5 — Channel lifecycle examples

(Creation, send/recv, closing, draining, errors)

NXD

FUNC MAIN():
    LET CH SET MAKE_CHANNEL<int>(CAPACITY 2)

    # producer
    SPAWN:
        SEND 1 TO CH
        SEND 2 TO CH
        SEND 3 TO CH   # error: channel full
        CLOSE CH

    # consumer
    LET A SET RECV CH   # 1
    LET B SET RECV CH   # 2
    LET C SET RECV CH   # error: channel closed and empty



Nim

import asyncdispatch, channels

proc producer(ch: Channel[int]) {.async.} =
  ch.send(1)
  ch.send(2)
  # third send may block or error depending on impl
  ch.close()

proc consumer(ch: Channel[int]) {.async.} =
  try:
    let a = await ch.recv()
    let b = await ch.recv()
    let c = await ch.recv()  # error after close+empty
  except ChannelClosedError:
    echo "channel closed"

proc main() =
  let ch = newChannel[int](2)
  asyncCheck producer(ch)
  asyncCheck consumer(ch)
  runForever()



Elixir

defmodule Chan do
  use GenServer

  def start_link(cap \\ 2), do: GenServer.start_link(__MODULE__, {[], cap})

  def send(pid, v), do: GenServer.call(pid, {:send, v})
  def recv(pid), do: GenServer.call(pid, :recv)
  def close(pid), do: GenServer.cast(pid, :close)

  def init({buf, cap}), do: {:ok, %{buf: buf, cap: cap, closed: false}}

  def handle_call({:send, v}, _from, %{closed: true} = s),
    do: {:reply, {:error, :closed}, s}

  def handle_call({:send, v}, _from, %{buf: buf, cap: cap} = s) when length(buf) < cap,
    do: {:reply, :ok, %{s | buf: buf ++ [v]}}

  def handle_call({:send, _}, _from, s),
    do: {:reply, {:error, :full}, s}

  def handle_call(:recv, _from, %{buf: [h | t]} = s),
    do: {:reply, {:ok, h}, %{s | buf: t}}

  def handle_call(:recv, _from, %{buf: [], closed: true} = s),
    do: {:reply, {:error, :closed}, s}

  def handle_call(:recv, _from, s),
    do: {:reply, {:error, :empty}, s}

  def handle_cast(:close, s), do: {:noreply, %{s | closed: true}}
end

def main() do
  {:ok, ch} = Chan.start_link(2)

  Task.start(fn ->
    Chan.send(ch, 1)
    Chan.send(ch, 2)
    Chan.send(ch, 3)   # {:error, :full}
    Chan.close(ch)
  end)

  IO.inspect Chan.recv(ch)  # {:ok, 1}
  IO.inspect Chan.recv(ch)  # {:ok, 2}
  IO.inspect Chan.recv(ch)  # {:error, :closed}
end



D

import std.stdio;
import std.concurrency;
import std.container;

struct Channel(T) {
    Array!T buf;
    size_t cap;
    bool closed;

    this(size_t c) { cap = c; }

    void send(T v) {
        if (closed) throw new Exception("channel closed");
        if (buf.length >= cap) throw new Exception("channel full");
        buf.insertBack(v);
    }

    T recv() {
        if (buf.length > 0) {
            auto v = buf.front;
            buf.removeFront();
            return v;
        }
        if (closed) throw new Exception("channel closed");
        throw new Exception("channel empty");
    }

    void close() { closed = true; }
}

void producer(Tid consumer, Channel!int ch) {
    ch.send(1);
    ch.send(2);
    try ch.send(3); catch (Exception e) writeln(e.msg);
    ch.close();
    consumer.send(ch);
}

void consumer() {
    auto ch = receiveOnly!(Channel!int);
    writeln(ch.recv()); // 1
    writeln(ch.recv()); // 2
    try writeln(ch.recv()); catch (Exception e) writeln(e.msg);
}

void main() {
    auto c = spawn(&consumer);
    auto ch = Channel!int(2);
    auto p = spawn(&producer, c, ch);
}



## Appendix G.6 — Trait object examples

(Static traits now, dynamic trait objects as future pattern)

NXD — static trait use (current spec)

TRAIT SERIALIZABLE {
    FUNC TO_STRING(X): string
}

TYPE USER IMPLEMENTS SERIALIZABLE:
    NAME: string

FUNC TO_STRING(U: USER): string:
    RETURN "USER(" ADD U.NAME ADD ")"

FUNC LOG<T : SERIALIZABLE>(X: T):
    PRINTLN(TO_STRING(X))

FUNC MAIN():
    LET U SET USER("gabriel")
    LOG(U)



Nim — static trait‑like pattern

type
  User = object
    name: string

proc toString(u: User): string =
  "USER(" & u.name & ")"

proc log[T](x: T) =
  echo toString(x)

proc main() =
  let u = User(name: "gabriel")
  log(u)



Elixir — protocol‑based static dispatch

defprotocol Serializable do
  def to_string(x)
end

defmodule User do
  defstruct [:name]
end

defimpl Serializable, for: User do
  def to_string(%User{name: name}), do: "USER(#{name})"
end

def log(x), do: IO.puts(Serializable.to_string(x))

def main() do
  u = %User{name: "gabriel"}
  log(u)
end



D — interface‑based trait object (future NXD pattern)

If you later allow BOX<SERIALIZABLE> as a dynamic trait object:

TRAIT SERIALIZABLE {
    FUNC TO_STRING(X): string
}

TYPE USER IMPLEMENTS SERIALIZABLE:
    NAME: string

FUNC TO_STRING(U: USER): string:
    RETURN "USER(" ADD U.NAME ADD ")"

FUNC LOG_OBJ(X: BOX<SERIALIZABLE>):
    PRINTLN(X.TO_STRING())

FUNC MAIN():
    LET U SET USER("gabriel")
    LET B SET BOX<SERIALIZABLE>(U)
    LOG_OBJ(B)


D lowering:

interface Serializable {
    string toString();
}

class User : Serializable {
    string name;
    this(string n) { name = n; }
    string toString() { return "USER(" ~ name ~ ")"; }
}

void logObj(Serializable x) {
    writeln(x.toString());
}

void main() {
    auto u = new User("gabriel");
    logObj(u);
}





## Appendix H - Error Handling Patterns




## Appendix I - Actor/Process Design Patterns





## Appendix J - State Machines





## Appendix K - Memory Safe Patterns





## Appendix L - Compiler Construction Examples






## Appendix M - Capability Security Cookbook




## Appendix N - Backend Differences Cookbook




## Appendix O - Pitfalls




## Appendix P - "Real Programs"




