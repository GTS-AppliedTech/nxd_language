# anonymous


proc worker() =
return 42

proc main() =
var handle = spawn(worker())
var result = await(handle)
echo(result)

