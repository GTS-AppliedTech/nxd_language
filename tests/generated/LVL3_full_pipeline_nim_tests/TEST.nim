# anonymous


proc worker() =
return 42

proc main() =
var handle = spawn(worker())
timeout(handle, 5000)

