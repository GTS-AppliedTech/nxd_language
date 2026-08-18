# IR node definitions for NXD compiler

class IRModule:
    def __init__(self, name, imports):
        self.name = name
        self.imports = imports


class IRImport:
    def __init__(self, path, alias=None):
        self.path = path
        self.alias = alias


# ---------- Types ----------

class IRTypeDecl:
    class Struct:
        def __init__(self, struct):
            self.struct = struct

    class Enum:
        def __init__(self, enum):
            self.enum = enum

    class Union:
        def __init__(self, union):
            self.union = union

    class Trait:
        def __init__(self, trait):
            self.trait = trait


class IRStruct:
    def __init__(self, name, fields):
        self.name = name
        self.fields = fields


class IRField:
    def __init__(self, name, ty):
        self.name = name
        self.ty = ty


class IREnum:
    def __init__(self, name, variants):
        self.name = name
        self.variants = variants


class IRUnion:
    def __init__(self, name, variants):
        self.name = name
        self.variants = variants


class IRUnionVariant:
    def __init__(self, kind, fields):
        self.kind = kind
        self.fields = fields


class IRTrait:
    def __init__(self, name, methods):
        self.name = name
        self.methods = methods


class IRFunctionSignature:
    def __init__(self, name, params, return_type):
        self.name = name
        self.params = params
        self.return_type = return_type


class IRParam:
    def __init__(self, name, ty):
        self.name = name
        self.ty = ty


# ---------- Functions ----------

class IRFunction:
    def __init__(self, name, params, return_type, body):
        self.name = name
        self.params = params
        self.return_type = return_type
        self.body = body


# ---------- Statements ----------

class IRStatement:
    class Let:
        def __init__(self, name, value):
            self.name = name
            self.value = value

    class Const:
        def __init__(self, name, value):
            self.name = name
            self.value = value

    class Return:
        def __init__(self, value):
            self.value = value

    class Loop:
        def __init__(self, body):
            self.body = body

    class If:
        def __init__(self, ifnode):
            self.ifnode = ifnode

    class Match:
        def __init__(self, matchnode):
            self.matchnode = matchnode

    class Expr:
        def __init__(self, expr):
            self.expr = expr


class IRIf:
    def __init__(self, condition, then_branch, else_branch):
        self.condition = condition
        self.then_branch = then_branch
        self.else_branch = else_branch


class IRMatch:
    def __init__(self, scrutinee, arms, otherwise):
        self.scrutinee = scrutinee
        self.arms = arms
        self.otherwise = otherwise


class IRMatchArm:
    def __init__(self, pattern, body):
        self.pattern = pattern
        self.body = body


# ---------- Expressions ----------

class IRExpr:
    class Literal:
        def __init__(self, literal):
            self.literal = literal

    class Binary:
        def __init__(self, op):
            self.op = op

    class Unary:
        def __init__(self, op):
            self.op = op

    class Call:
        def __init__(self, func, args):
            self.func = func
            self.args = args

    class Var:
        def __init__(self, name):
            self.name = name

    class Pipeline:
        def __init__(self, value, func):
            self.value = value
            self.func = func


class IRBinaryOp:
    def __init__(self, kind, left, right):
        self.kind = kind
        self.left = left
        self.right = right


class IRUnaryOp:
    def __init__(self, kind, expr):
        self.kind = kind
        self.expr = expr


# ---------- Literals ----------

class IRLiteral:
    def __init__(self, value):
        self.value = value
