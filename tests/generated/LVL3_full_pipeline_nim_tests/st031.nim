# test


proc main() =
  let state = "READY"
case state:
  of "READY":
    println("GO")
  of "WAIT":
    println("HOLD")
  else:
    println("UNKNOWN")

