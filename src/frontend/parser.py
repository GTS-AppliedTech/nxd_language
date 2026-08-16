from frontend.lexer import lex
from frontend.ast.nodes import *

class Parser:
    def __init__(self, src: str):
        self.tokens = lex(src)
        self.pos = 0
        self.indent_stack = [0]

    def peek(self):
        return self.tokens[self.pos]

    def eat(self, kind=None, val=None):
        tok = self.peek()
        if kind and tok[0] != kind:
            raise Exception(f"Expected {kind}, got {tok}")
        if val and tok[1] != val:
            raise Exception(f"Expected {val}, got {tok}")
        self.pos += 1
        return tok

    def at(self, kind, val=None):
        tok = self.peek()
        if tok[0] != kind:
            return False
        if val and tok[1] != val:
            return False
        return True

    # ---------- top level ----------

    def parse_program(self):
        module = self.parse_module()
        # for now, everything else hangs off module.body
        return module

    def parse_module(self):
        self.eat("KEYWORD", "MODULE")
        name = self.eat("IDENT")[1]
        imports = []
        body = []
        while self.at("KEYWORD", "IMPORT"):
            imports.append(self.parse_import())
        while not self.at("EOF"):
            body.append(self.parse_top_level())
        return ASTModule(name=name, imports=imports, body=body)

    def parse_import(self):
        self.eat("KEYWORD", "IMPORT")
        path = self.eat("IDENT")[1]
        alias = None
        if self.at("KEYWORD", "AS"):
            self.eat("KEYWORD", "AS")
            alias = self.eat("IDENT")[1]
        return ASTImport(path=path, alias=alias)

    def parse_top_level(self):
        tok = self.peek()
        if tok[0] == "KEYWORD" and tok[1] == "TYPE":
            return self.parse_type_decl()
        if tok[0] == "KEYWORD" and tok[1] == "FUNC":
            return self.parse_func_decl()
        # allow top-level statements
        return self.parse_statement()

    # ---------- types ----------

    def parse_type_decl(self):
        self.eat("KEYWORD", "TYPE")
        name = self.eat("IDENT")[1]
        if self.at("KEYWORD", "ENUM"):
            self.eat("KEYWORD", "ENUM")
            return self.parse_enum_type(name)
        if self.at("KEYWORD", "UNION"):
            self.eat("KEYWORD", "UNION")
            return self.parse_union_type(name)
        if self.at("KEYWORD", "TRAIT"):
            self.eat("KEYWORD", "TRAIT")
            return self.parse_trait_type(name)
        # default: struct with brace body
        return self.parse_struct_type(name)

    def parse_struct_type(self, name):
        self.eat("LBRACE")
        fields = []
        while not self.at("RBRACE"):
            fname = self.eat("IDENT")[1]
            self.eat("COLON")
            ty = self.parse_type_ref()
            if self.at("COMMA"):
                self.eat("COMMA")
            fields.append(ASTField(name=fname, ty=ty))
        self.eat("RBRACE")
        return ASTStruct(name=name, fields=fields)

    def parse_enum_type(self, name):
        self.eat("LBRACE")
        variants = []
        while not self.at("RBRACE"):
            v = self.eat("IDENT")[1]
            variants.append(v)
            if self.at("COMMA"):
                self.eat("COMMA")
        self.eat("RBRACE")
        return ASTEnum(name=name, variants=variants)

    def parse_union_type(self, name):
        self.eat("LBRACE")
        variants = []
        while not self.at("RBRACE"):
            kind = self.eat("IDENT")[1]
            self.eat("LPAREN")
            fields = []
            while not self.at("RPAREN"):
                fname = self.eat("IDENT")[1]
                self.eat("COLON")
                ty = self.parse_type_ref()
                if self.at("COMMA"):
                    self.eat("COMMA")
                fields.append(ASTField(name=fname, ty=ty))
            self.eat("RPAREN")
            if self.at("COMMA"):
                self.eat("COMMA")
            variants.append(ASTUnionVariant(kind=kind, fields=fields))
        self.eat("RBRACE")
        return ASTUnion(name=name, variants=variants)

    def parse_trait_type(self, name):
        self.eat("LBRACE")
        methods = []
        while not self.at("RBRACE"):
            self.eat("KEYWORD", "FUNC")
            mname = self.eat("IDENT")[1]
            self.eat("LPAREN")
            params = []
            if not self.at("RPAREN"):
                params.append(self.eat("IDENT")[1])
                while self.at("COMMA"):
                    self.eat("COMMA")
                    params.append(self.eat("IDENT")[1])
            self.eat("RPAREN")
            self.eat("COLON")
            ret = self.parse_type_ref()
            methods.append(ASTFunctionSignature(name=mname, params=params, return_type=ret))
        self.eat("RBRACE")
        return ASTTrait(name=name, methods=methods)

    def parse_type_ref(self):
        if self.peek()[0] == "LOWTYPE":
            return self.eat("LOWTYPE")[1]
        return self.eat("IDENT")[1]

    # ---------- functions ----------

    def parse_func_decl(self):
        self.eat("KEYWORD", "FUNC")
        name = self.eat("IDENT")[1]
        self.eat("LPAREN")
        params = []
        if not self.at("RPAREN"):
            params.append(self.parse_param())
            while self.at("COMMA"):
                self.eat("COMMA")
                params.append(self.parse_param())
        self.eat("RPAREN")
        ret = None
        if self.at("COLON"):
            self.eat("COLON")
            # either type or start of block
            if not self.at("NEWLINE"):
                ret = self.parse_type_ref()
        self.expect_block_colon()
        body = self.parse_block()
        return ASTFunction(name=name, params=[p[0] for p in params], return_type=ret, body=body)

    def parse_param(self):
        name = self.eat("IDENT")[1]
        if self.at("COLON"):
            self.eat("COLON")
            ty = self.parse_type_ref()
            return (name, ty)
        return (name, "any")

    # ---------- blocks & indentation ----------

    def expect_block_colon(self):
        # already consumed ':' in func; for IF/MATCH/etc we ensure colon then newline
        if self.at("COLON"):
            self.eat("COLON")
        if self.at("NEWLINE"):
            self.eat("NEWLINE")

    def parse_block(self):
        # simplified: read until blank line or dedent; for now, just read statements until keyword that closes
        stmts = []
        while not self.at("EOF") and not self.at("KEYWORD", "ELSE") and not self.at("KEYWORD", "CASE") and not self.at("KEYWORD", "OTHERWISE"):
            if self.at("NEWLINE"):
                self.eat("NEWLINE")
                continue
            stmts.append(self.parse_statement())
        return stmts

    # ---------- statements ----------

    def parse_statement(self):
        tok = self.peek()
        if tok[0] == "KEYWORD":
            kw = tok[1]
            if kw == "LET":
                return self.parse_let()
            if kw == "CONST":
                return self.parse_const()
            if kw == "RETURN":
                return self.parse_return()
            if kw == "LOOP":
                return self.parse_loop()
            if kw == "IF":
                return self.parse_if()
            if kw == "MATCH":
                return self.parse_match()
            if kw == "SPAWN":
                return self.parse_spawn()
            if kw == "SEND":
                return self.parse_send()
            if kw == "RECV":
                return self.parse_recv()
            if kw == "AWAIT":
                return self.parse_await_stmt()
        # fallback: expression statement
        expr = self.parse_expr()
        return expr

    def parse_let(self):
        self.eat("KEYWORD", "LET")
        name = self.eat("IDENT")[1]
        self.eat("KEYWORD", "SET")
        value = self.parse_expr()
        return ASTLet(name=name, value=value)

    def parse_const(self):
        self.eat("KEYWORD", "CONST")
        name = self.eat("IDENT")[1]
        self.eat("KEYWORD", "SET")
        value = self.parse_expr()
        return ASTConst(name=name, value=value)

    def parse_return(self):
        self.eat("KEYWORD", "RETURN")
        value = self.parse_expr()
        return ASTReturn(value=value)

    def parse_loop(self):
        self.eat("KEYWORD", "LOOP")
        self.expect_block_colon()
        body = self.parse_block()
        return ASTLoop(body=body)

    def parse_if(self):
        self.eat("KEYWORD", "IF")
        cond = self.parse_expr()
        self.expect_block_colon()
        then_branch = self.parse_block()
        else_branch = []
        if self.at("KEYWORD", "ELSE"):
            self.eat("KEYWORD", "ELSE")
            self.expect_block_colon()
            else_branch = self.parse_block()
        return ASTIf(condition=cond, then_branch=then_branch, else_branch=else_branch)

    def parse_match(self):
        self.eat("KEYWORD", "MATCH")
        scrutinee = self.parse_expr()
        self.expect_block_colon()
        arms = []
        otherwise = None
        while self.at("KEYWORD", "CASE"):
            arms.append(self.parse_case_block())
        if self.at("KEYWORD", "OTHERWISE"):
            self.eat("KEYWORD", "OTHERWISE")
            self.expect_block_colon()
            otherwise = self.parse_block()
        return ASTMatch(scrutinee=scrutinee, arms=arms, otherwise=otherwise)

    def parse_case_block(self):
        self.eat("KEYWORD", "CASE")
        pattern = self.parse_pattern()
        self.expect_block_colon()
        body = self.parse_block()
        return ASTMatchArm(pattern=pattern, body=body)

    def parse_pattern(self):
        # TODO: struct/list patterns; for now, literal or identifier
        if self.peek()[0] == "NUMBER" or self.peek()[0] == "STRING":
            lit = self.parse_literal()
            return lit.value
        return self.eat("IDENT")[1]

    def parse_spawn(self):
        self.eat("KEYWORD", "SPAWN")
        call = self.parse_call_expr()
        return ASTExpr(call)  # or dedicated ASTSpawn

    def parse_send(self):
        self.eat("KEYWORD", "SEND")
        msg = self.parse_expr()
        self.eat("KEYWORD", "TO")
        target = self.parse_expr()
        # TODO: dedicated AST node
        return ASTExpr(ASTCall(func="SEND", args=[msg, target]))

    def parse_recv(self):
        self.eat("KEYWORD", "RECV")
        ch = self.eat("IDENT")[1]
        # LET V SET RECV CH handled at statement level
        return ASTCall(func="RECV", args=[ASTVar(name=ch)])

    def parse_await_stmt(self):
        self.eat("KEYWORD", "AWAIT")
        expr = self.parse_expr()
        return ASTExpr(ASTCall(func="AWAIT", args=[expr]))

    # ---------- expressions ----------

    def parse_expr(self):
        return self.parse_logic()

    def parse_logic(self):
        left = self.parse_comp()
        while self.at("OP") and self.peek()[1] in ("AND", "OR"):
            op = self.eat("OP")[1]
            right = self.parse_comp()
            left = ASTBinary(kind=op, left=left, right=right)
        return left

    def parse_comp(self):
        left = self.parse_add()
        while self.at("OP") and self.peek()[1] in ("EQ", "NEQ", "GT", "LT", "GTE", "LTE", "AS", "IS"):
            op = self.eat("OP")[1]
            right = self.parse_add()
            left = ASTBinary(kind=op, left=left, right=right)
        return left

    def parse_add(self):
        left = self.parse_mul()
        while self.at("OP") and self.peek()[1] in ("ADD", "SUB"):
            op = self.eat("OP")[1]
            right = self.parse_mul()
            left = ASTBinary(kind=op, left=left, right=right)
        return left

    def parse_mul(self):
        left = self.parse_unary()
        while self.at("OP") and self.peek()[1] in ("MUL", "DIV", "MOD"):
            op = self.eat("OP")[1]
            right = self.parse_unary()
            left = ASTBinary(kind=op, left=left, right=right)
        return left

    def parse_unary(self):
        if self.at("OP") and self.peek()[1] in ("NOT", "MOVE", "CLONE", "BORROW"):
            op = self.eat("OP")[1]
            expr = self.parse_primary()
            return ASTUnary(kind=op, expr=expr)
        return self.parse_primary()

    def parse_primary(self):
        tok = self.peek()
        if tok[0] == "NUMBER" or tok[0] == "STRING":
            return self.parse_literal()
        if tok[0] == "LBRACK":
            return self.parse_list_literal()
        if tok[0] == "LBRACE":
            return self.parse_map_literal()
        if tok[0] == "FN":
            return self.parse_lambda()
        if tok[0] == "IDENT":
            # could be var or call
            if self._next_is("LPAREN"):
                return self.parse_call_expr()
            return ASTVar(name=self.eat("IDENT")[1])
        raise Exception(f"Unexpected token in primary: {tok}")

    def parse_literal(self):
        tok = self.peek()
        if tok[0] == "NUMBER":
            v = float(self.eat("NUMBER")[1]) if "." in tok[1] else int(self.eat("NUMBER")[1])
            return ASTLiteral(value=v)
        if tok[0] == "STRING":
            s = self.eat("STRING")[1][1:-1]
            return ASTLiteral(value=s)
        raise Exception("Literal expected")

    def parse_list_literal(self):
        self.eat("LBRACK")
        items = []
        while not self.at("RBRACK"):
            items.append(self.parse_expr())
            if self.at("COMMA"):
                self.eat("COMMA")
        self.eat("RBRACK")
        return ASTLiteral(value=[items])

    def parse_map_literal(self):
        self.eat("LBRACE")
        entries = {}
        while not self.at("RBRACE"):
            key = self.parse_expr()
            self.eat("COLON")
            val = self.parse_expr()
            if self.at("COMMA"):
                self.eat("COMMA")
            entries[key] = val
        self.eat("RBRACE")
        return ASTLiteral(value=entries)

    def parse_lambda(self):
        self.eat("FN")
        self.eat("LPAREN")
        params = []
        if not self.at("RPAREN"):
            params.append(self.eat("IDENT")[1])
            while self.at("COMMA"):
                self.eat("COMMA")
                params.append(self.eat("IDENT")[1])
        self.eat("RPAREN")
        self.eat("ARROW")
        body = self.parse_expr()
        # represent as ASTCall to a synthetic lambda or dedicated node
        return ASTLiteral(value=("lambda", params, body))

    def parse_call_expr(self):
        name = self.eat("IDENT")[1]
        self.eat("LPAREN")
        args = []
        if not self.at("RPAREN"):
            args.append(self.parse_expr())
            while self.at("COMMA"):
                self.eat("COMMA")
                args.append(self.parse_expr())
        self.eat("RPAREN")
        return ASTCall(func=name, args=args)

    def _next_is(self, kind):
        if self.pos + 1 >= len(self.tokens):
            return False
        return self.tokens[self.pos + 1][0] == kind
