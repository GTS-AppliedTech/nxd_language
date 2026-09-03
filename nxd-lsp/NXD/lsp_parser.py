import sys

from src.frontend.parser.parser import parse, ParserError

source = sys.stdin.read()

try:
    parse(source)

    print("OK")

except ParserError as e:
    print(
        f"{e.line}|{e.col}|{e.message}"
    )