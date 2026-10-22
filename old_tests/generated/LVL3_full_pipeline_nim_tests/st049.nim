# anonymous


proc worker() =
return 42

proc main() =
var handle = spawn(worker())
var msg = recv(handle)
esc(msg)

