# anonymous


proc fetch() =
return 42

proc main() =
var task = fetch()
var x = await(task)
echo(x)

