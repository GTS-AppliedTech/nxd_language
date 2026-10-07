# test


proc main() =
  let state = "READY"
case state:
  of "READY":
    echo("GO")
  of "WAIT":
    echo("HOLD")
  else:
    echo("UNKNOWN")

