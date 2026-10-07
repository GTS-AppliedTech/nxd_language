# nim_scope_lifetime


proc create_value() =
var value = "temporary"
echo(value)

proc main() =
create_value()
echo("complete")

