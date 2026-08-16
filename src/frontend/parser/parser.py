from frontend.ast.nodes import *
from frontend.lexer.scanner import lex

class Parser:
    def __init__(self, source: str):
        self.tokens = lex(source)
        self.pos = 0

    def peek(self):
        return self.tokens[self.pos] if self.pos < len(self.tokens) else ("EOF", "")

    def eat(self, kind=None):
        tok = self.peek()
        if kind and tok[0] != kind:
            raise Exception(f"Expected {kind}, got {tok}")
        self.pos += 1
        return tok

    # ============================
    # MODULE
    # ============================

    def parse_module(self):
        self.eat("IDENT")  # MODULE
        name = self.eat("NAME")[1]
        imports = []
        body = []

        # TODO: parse imports, types, functions, statements

        return ASTModule(name=name, imports=imports, body=body)

    # ============================
    # EXPRESSIONS
    # ============================

    def parse_expr(self):
        tok = self.peek()

        if tok[0] == "NUMBER":
            self.eat("NUMBER")
            return ASTLiteral(int(tok[1]))

        if tok[0] == "STRING":
            self.eat("STRING")
            return ASTLiteral(tok[1][1:-1])

        if tok[0] == "NAME":
            name = self.eat("NAME")[1]
            return ASTVar(name)

        raise Exception("Expression parsing not implemented yet")
