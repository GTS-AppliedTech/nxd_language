# nim_clone_independence


proc main() =
  var source = "NXD"
  var copy = deepCopy(source)
  println(source)
  println(copy)

