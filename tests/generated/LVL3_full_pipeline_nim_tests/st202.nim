# nim_moive_invalid_reuse


proc main() =
  var source = "NXD"
  var target = move(source)
  println(source)

