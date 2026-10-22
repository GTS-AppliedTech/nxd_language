# anonymous


type
  PRINTABLE = concept x
    print(x) is void
  USER = object
    name: STRING

proc main() =
var x = USER(NAME: "gabby")

