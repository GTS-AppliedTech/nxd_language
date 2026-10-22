# test


type
  RESULT = object
    case kind: ResultKind
    of SUCCESS:
      value: int
    of FAILURE:
      message: string

proc main() =
echo("UNION_DECLARED")

