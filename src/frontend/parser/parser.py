from dataclasses import fields
from os import name
from turtle import left

from src.frontend.lexer.scanner import lex
from src.frontend.ast.nodes import *

class ParserError(Exception):
    def __init__(self, code, message, line, col):
        super().__init__(message)
        self.code = code
        self.message = message
        self.line = line
        self.col = col

class WarningDiagnostic:
    def __init__(self, line, col, message):
        self.line = line
        self.col = col
        self.message = message

class Parser:
    def __init__(self, src: str):
        self.tokens = lex(src)
        self.pos = 0
        self.indent_stack = [0]

    def peek(self):
        if self.pos >= len(self.tokens):
            return ("EOF", "")
        return self.tokens[self.pos]
    
    def _next_is(self, kind):
        if self.pos + 1 >= len(self.tokens):
            return False
        return self.tokens[self.pos + 1][0] == kind

    def eat(self, kind=None, val=None):
        tok = self.peek()

        if kind and tok[0] != kind:
            raise ParserError(
                "NXD-P1001",
                f"Expected {kind}, got {tok[0]}",
                tok[2],
                tok[3]
           )

        if val and tok[1] != val:
            raise ParserError(
                "NXD-P1002",
                f"Expected {val}, got {tok[1]}",
                tok[2],
                tok[3]
            )

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
        # Skip leading blank lines
        while self.at("NEWLINE"):
            self.eat("NEWLINE")
        # MODULE is optional.
        if self.at("KEYWORD", "MODULE"):
            return self.parse_module()

        imports = []
        body = []

        while not self.at("EOF"):
            if self.at("NEWLINE"):
                self.eat("NEWLINE")
                continue

            if self.at("KEYWORD", "IMPORT"):
                imports.append(self.parse_import())
                continue

            body.append(self.parse_top_level())

        return ASTModule(
            name="ANONYMOUS",
            imports=imports,
            body=body,
    )

    def parse_module(self):
        self.eat("KEYWORD", "MODULE")
        name = self.eat("IDENT")[1]
        imports = []
        body = []

        while not self.at("EOF"):
            if self.at("NEWLINE"):
                self.eat("NEWLINE")
                continue
            if self.at("KEYWORD", "IMPORT"):
                imports.append(self.parse_import())
                continue
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
        if tok[0] == "KEYWORD" and tok[1] == "TRAIT":
            return self.parse_trait_decl()
        if tok[0] == "KEYWORD" and tok[1] == "IMPL":
            return self.parse_impl_decl()
        if tok[0] == "KEYWORD" and tok[1] == "IMPORT":
            return self.parse_import()
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

        if self.at("KEYWORD", "STRUCT"):
            self.eat("KEYWORD", "STRUCT")
            return self.parse_struct_type(name)

        tok = self.peek()

        raise ParserError(
            f"Expected STRUCT, ENUM, UNION, or TRAIT, got {tok[0]} {tok[1]}",
            tok[2],
            tok[3]
        )
    def parse_impl_decl(self):
        impl_column = self.peek()[3]

        self.eat("KEYWORD", "IMPL")
        trait_name = self.eat("IDENT")[1]
        self.eat("KEYWORD", "FOR")
        target_type = self.eat("IDENT")[1]
        self.eat("COLON")

        while self.at("NEWLINE"):
            self.eat("NEWLINE")

        methods = []

        while (
            self.at("KEYWORD", "FUNC")
            and self.peek()[3] > impl_column
        ):
            method_column = self.peek()[3]

            self.eat("KEYWORD", "FUNC")
            method_name = self.eat("IDENT")[1]
            self.eat("LPAREN")

            params = []

            if not self.at("RPAREN"):
                params.append(self.eat("IDENT")[1])

                while self.at("COMMA"):
                    self.eat("COMMA")
                    params.append(self.eat("IDENT")[1])

            self.eat("RPAREN")

            return_type = None

            if self.at("COLON"):
                self.eat("COLON")

                if not self.at("NEWLINE"):
                    return_type = self.parse_type_ref()

            if self.at("NEWLINE"):
                self.eat("NEWLINE")

            body = []

            while not self.at("EOF"):
                while self.at("NEWLINE"):
                    self.eat("NEWLINE")

                if self.at("EOF"):
                    break

                if self.peek()[3] <= method_column:
                    break

                body.append(self.parse_statement())

            methods.append(
                ASTFunction(
                    name=method_name,
                    params=params,
                    return_type=return_type,
                    body=body,
                )
            )

        return ASTImpl(
            trait_name=trait_name,
            target_type=target_type,
            methods=methods,
        )

    def parse_trait_decl(self):
        self.eat("KEYWORD", "TRAIT")
        name = self.eat("IDENT")[1]

        return self.parse_trait_type(name)

    def parse_struct_type(self, name):
        self.eat("COLON")

        while self.at("NEWLINE"):
            self.eat("NEWLINE")

        fields = []

        while self.at("IDENT") and self._next_is("COLON"):
            fname = self.eat("IDENT")[1]
            self.eat("COLON")
            ty = self.parse_type_ref()

            if self.at("COMMA"):
                self.eat("COMMA")

            fields.append(ASTField(name=fname, ty=ty))

            while self.at("NEWLINE"):
                self.eat("NEWLINE")

        return ASTStruct(name=name, fields=fields)

    def parse_enum_type(self, name):
        self.eat("COLON")

        while self.at("NEWLINE"):
            self.eat("NEWLINE")

        variants = []

        while self.at("IDENT"):
            variants.append(self.eat("IDENT")[1])

            if self.at("COMMA"):
                self.eat("COMMA")

            while self.at("NEWLINE"):
                self.eat("NEWLINE")

        return ASTEnum(name=name, variants=variants)

    def parse_union_type(self, name):
        self.eat("COLON")
        while self.at("NEWLINE"):
            self.eat("NEWLINE")
        variants = []
        while self.at("IDENT"):
            kind = self.eat("IDENT")[1]
            self.eat("LBRACE")
            fields = []
            while self.at("NEWLINE"):
                self.eat("NEWLINE")
            while not self.at("RBRACE"):
                if self.at("NEWLINE"):
                    self.eat("NEWLINE")
                    continue
                fname = self.eat("IDENT")[1]
                self.eat("COLON")
                ty = self.parse_type_ref()
                fields.append(
                    ASTField(
                        name=fname,
                        ty=ty,
                    )
                )

                if self.at("COMMA"):
                    self.eat("COMMA")
            self.eat("RBRACE")
            variants.append(
                ASTUnionVariant(
                    kind=kind,
                    fields=fields,
                )
            )

            while self.at("NEWLINE"):
                self.eat("NEWLINE")

        return ASTUnion(
            name=name,
            variants=variants,
        )
    
    def parse_trait_type(self, name):
        self.eat("COLON")

        while self.at("NEWLINE"):
            self.eat("NEWLINE")
        methods = []
        while self.at("KEYWORD", "FUNC"):
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
            ret = None
            if self.at("COLON"):
                self.eat("COLON")
                ret = self.parse_type_ref()
            methods.append(
                ASTFunctionSignature(
                    name=mname,
                    params=params,
                    return_type=ret,
                )
            )
            while self.at("NEWLINE"):
                self.eat("NEWLINE")
        return ASTTrait(
            name=name,
            methods=methods,
        )

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
        stmts = []

        while (
            not self.at("EOF")
            and not self.at("KEYWORD", "ELSE")
            and not self.at("KEYWORD", "CASE")
            and not self.at("KEYWORD", "OTHERWISE")
            and not self.at("KEYWORD", "CATCH")
            and not self.at("KEYWORD", "FINALLY")

            and not self.at("KEYWORD", "FUNC")
            and not self.at("KEYWORD", "IMPORT")
            and not self.at("KEYWORD", "TYPE")
            and not self.at("KEYWORD", "ENUM")
            and not self.at("KEYWORD", "STRUCT")
            and not self.at("KEYWORD", "UNION")
            and not self.at("KEYWORD", "TRAIT")
            and not self.at("KEYWORD", "IMPL")
        ):
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

            if kw == "TRY":
                return self.parse_try()

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

        if tok[0] == "OP" and tok[1] == "MOVE":
            return self.parse_move()
        if tok[0] == "OP" and tok[1] == "CLONE":
            return self.parse_clone()

    # Fallback: expression statement
        expr = self.parse_expr()
        return expr

    def parse_clone(self):
        token = self.eat("OP")

        if token[1] != "CLONE":
            raise ParserError(
                f"Expected CLONE, got {token[0]} {token[1]}"
            )

        if not self.at("IDENT"):
            raise ParserError(
                f"Expected source identifier after CLONE, "
                f"got {self.peek()[0]} {self.peek()[1]}"
            )

        source = ASTVar(name=self.eat("IDENT")[1])

        if not (self.at("IDENT") and self.peek()[1] == "TO"):
            raise ParserError(
                f"Expected TO after CLONE source, "
                f"got {self.peek()[0]} {self.peek()[1]}"
            )

        self.eat("IDENT")

        if not self.at("IDENT"):
            raise ParserError(
                f"Expected target identifier after TO, "
                f"got {self.peek()[0]} {self.peek()[1]}"
            )

        target = ASTVar(name=self.eat("IDENT")[1])

        return ASTClone(
            source=source,
            target=target,
        )

    def parse_move(self):
        self.eat("OP")      # MOVE

        source = self.parse_primary()

        if not (self.at("IDENT") and self.peek()[1] == "TO"):
            raise SyntaxError("Expected TO")

        self.eat("IDENT")   # TO

        target = self.parse_primary()

        return ASTMove(source=source, target=target)

    def parse_let(self):
        self.eat("KEYWORD", "LET")
        ident_tok = self.eat("IDENT")
        name = ident_tok[1]
        self.eat("KEYWORD", "SET")
        value = self.parse_expr()
        return ASTLet(name=name, value=value, line=ident_tok[2], col=ident_tok[3])

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

    def parse_try(self):
        self.eat("KEYWORD", "TRY")
        self.eat("COLON")

        try_body = self.parse_block()

        catch_body = []
        finally_body = []

        if self.at("KEYWORD", "CATCH"):
            self.eat("KEYWORD", "CATCH")
            self.eat("COLON")
            catch_body = self.parse_block()

        if self.at("KEYWORD", "FINALLY"):
            self.eat("KEYWORD", "FINALLY")
            self.eat("COLON")
            finally_body = self.parse_block()

        return ASTTry(
            try_body=try_body,
            catch_body=catch_body,
            finally_body=finally_body,
    )

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

        while self.at("OP") and self.peek()[1] in (
            "EQ", "NEQ", "GT", "LT", "GTE", "LTE", "AS", "IS"
        ):
            op = self.eat("OP")[1]

            if op in ("IS", "AS"):
                if self.at("LOWTYPE"):
                    right = ASTVar(self.eat("LOWTYPE")[1])
                else:
                    raise SyntaxError(
                        f"Expected type after {op}, got {self.peek()[0]} {self.peek()[1]}"
                    )
            else:
                right = self.parse_add()

            print("OP:", op)
            print("NEXT:", self.peek())

            left = ASTBinary(
                kind=op,
                left=left,
                right=right
            )

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
        if self.at("OP") and self.peek()[1] in ("NOT", "BORROW", "SUB"):
            op = self.eat("OP")[1]
            expr = self.parse_primary()
            return ASTUnary(kind=op, expr=expr)
        return self.parse_primary()

    def parse_primary(self):
        tok = self.peek()

        if tok[0] in ("NUMBER", "STRING"):
            return self.parse_literal()
        if tok[0] == "LOWNAME" and tok[1] in ("true", "false", "none"):
            return self.parse_literal()
        if tok[0] == "LBRACK":
            return self.parse_list_literal()
        if tok[0] == "LBRACE":
            return self.parse_map_literal()
        if tok[0] == "FN":
            return self.parse_lambda()
        if tok[0] == "IDENT":
            # Could be a call, typed object initializer, or variable.
            if self._next_is("LPAREN"):
                return self.parse_call_expr()
            if self._next_is("LBRACE"):
                return self.parse_object_init()

            ident_tok = self.eat("IDENT")

            return ASTVar(
                name=ident_tok[1],
                line=ident_tok[2],
                col=ident_tok[3]
            )
        raise ParserError(
        f"Unexpected token in primary: {tok[1]}",
            tok[2],
            tok[3]
)
    def parse_literal(self):
        tok = self.peek()
        if tok[0] == "NUMBER":
            v = float(self.eat("NUMBER")[1]) if "." in tok[1] else int(self.eat("NUMBER")[1])
            return ASTLiteral(value=v)
        if tok[0] == "STRING":
            s = self.eat("STRING")[1][1:-1]
            return ASTLiteral(value=s)
        if tok[0] == "LOWNAME":
            value = self.eat("LOWNAME")[1]
            if value == "true":
                return ASTLiteral(value=True)
            if value == "false":
                return ASTLiteral(value=False)
            if value == "none":
                return ASTLiteral(value=None)
        tok = self.peek()

        raise ParserError(
            f"Literal expected, got {tok[0]} {tok[1]}",
            tok[2],
            tok[3]
        )
    def parse_list_literal(self):
        self.eat("LBRACK")
        items = []
        while not self.at("RBRACK"):
            items.append(self.parse_expr())
            if self.at("COMMA"):
                self.eat("COMMA")
        self.eat("RBRACK")
        return ASTLiteral(value=items)

    def parse_object_init(self):
        type_name = self.eat("IDENT")[1]
        map_literal = self.parse_map_literal()

        return ASTObjectInit(
            type_name=type_name,
            fields=map_literal.value
        )

    def parse_map_literal(self):
        self.eat("LBRACE")
        entries = {}
        while not self.at("RBRACE"):
            while self.at("NEWLINE"):
                self.eat("NEWLINE")
                if self.at("RBRACE"):
                 break
        # Map/object keys should be simple identifiers for now
            if self.at("IDENT"):
                key = self.eat("IDENT")[1]
            elif self.at("LOWNAME"):
                key = self.eat("LOWNAME")[1]
            elif self.at("STRING"):
                key = self.eat("STRING")[1][1:-1]
            else:
                tok = self.peek()

                raise ParserError(
                    f"Expected map key, got {tok[0]} {tok[1]}",
                    tok[2],
                    tok[3]
                )
            self.eat("COLON")
            val = self.parse_expr()
            entries[key] = val
            if self.at("COMMA"):
                self.eat("COMMA")
            while self.at("NEWLINE"):
                self.eat("NEWLINE")
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
def parse(src: str):
    parser = Parser(src)
    module = parser.parse_program()

    ast_types = []
    ast_functions = []
    ast_statements = []

    for node in module.body:
        if isinstance(node, (ASTStruct, ASTEnum, ASTUnion, ASTTrait, ASTImpl)):
            ast_types.append(node)
        elif isinstance(node, ASTFunction):
            ast_functions.append(node)
        else:
            ast_statements.append(node)

    return module, ast_types, ast_functions


