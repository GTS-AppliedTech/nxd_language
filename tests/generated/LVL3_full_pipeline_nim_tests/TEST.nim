# anonymous


proc worker() =
return 42

proc main() =
var task = worker()
var result = await(task)

