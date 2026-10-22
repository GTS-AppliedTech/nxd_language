from src.frontend.ir.nodes import *

class IRValidationError(Exception):
    pass


def validate_ir(ir_root):
    validate_module(ir_root.module)
    validate_types(ir_root.types)
    validate_functions(ir_root.functions)


def validate_module(module):
    if not isinstance(module.name, str):
        raise IRValidationError(f"Module name must be string, got {module.name!r}")

    for imp in module.imports:
        if not isinstance(imp.path, str):
            raise IRValidationError(f"Import path must be string, got {imp.path!r}")


def validate_types(types):
    for t in types:
        if isinstance(t, IRTypeDecl.Struct):
            validate_struct(t.struct)
        elif isinstance(t, IRTypeDecl.Enum):
            validate_enum(t.enum)
        elif isinstance(t, IRTypeDecl.Union):
            validate_union(t.union)
        elif isinstance(t, IRTypeDecl.Trait):
            validate_trait(t.trait)


def validate_struct(s):
    for f in s.fields:
        if not isinstance(f.name, str):
            raise IRValidationError("Struct field name must be string")
        if not isinstance(f.ty, str):
            raise IRValidationError("Struct field type must be string")


def validate_enum(e):
    for v in e.variants:
        if not isinstance(v, str):
            raise IRValidationError("Enum variant must be string")


def validate_union(u):
    for v in u.variants:
        if not isinstance(v.kind, str):
            raise IRValidationError("Union variant kind must be string")


def validate_trait(t):
    for m in t.methods:
        if not isinstance(m.name, str):
            raise IRValidationError("Trait method name must be string")


def validate_functions(funcs):
    for f in funcs:
        for p in f.params:
            if not isinstance(p.name, str):
                raise IRValidationError("Function parameter name must be string")
            if not isinstance(p.ty, str):
                raise IRValidationError("Function parameter type must be string")
