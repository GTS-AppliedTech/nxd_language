# anonymous


proc worker() =
return 42

proc main() =
var handle = spawn(worker())
send(42, handle)

