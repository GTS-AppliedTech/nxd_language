# anonymous


proc worker() =
return 42

proc main() =
var handle = spawn(worker())
if ping(handle):
    echo(true)
else:
    timeout(handle, 5000)
echo(false)

