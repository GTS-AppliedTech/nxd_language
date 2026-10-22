# anonymous


proc worker() =
var h = spawn(worker())
ping(h)
esc(h)

