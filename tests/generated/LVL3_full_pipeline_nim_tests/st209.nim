# nim_scope_lifetime


proc create_value() =
  var value = "temporary"

proc main() =
  create_value()

