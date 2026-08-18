from src.frontend.parser.parser import parse
from src.frontend.ir.lowering import lower_module, lower_types, lower_function
from src.frontend.ir.validate import validate_ir
from src.frontend.ir.nodes import * # plus others
import json

def compile_to_ir_json(src: str, out_path: str):
    ast_module, ast_types, ast_functions = parse(src)

    ir_module = lower_module(ast_module)
    ir_types = lower_types(ast_types)
    ir_functions = [lower_function(f) for f in ast_functions]

ir_root = {
    "module": serialize_module(ir_module),
    "types": [serialize_type(t) for t in ir_types],
    "traits": [],
    "impls": [],
    "functions": [serialize_function(f) for f in ir_functions],
    "statements": [],
}
    # optional: validate IR structure before writing
    # validate_ir(ir_root_like_object)  # or validate on Rust side only

with open(out_path, "w") as f:
        json.dump(ir_root, f, indent=2)


# --- serialization helpers (Python → JSON) ---

def serialize_module(m: IRModule):
    return {
        "name": m.name,
        "imports": [
            {"path": imp.path, "alias": imp.alias}
            for imp in m.imports
        ],
    }

def serialize_type(t):
    # You can tag kinds explicitly
    if isinstance(t, IRTypeDecl.Struct):
        return {"kind": "struct", "name": t.struct.name,
                "fields": [{"name": f.name, "ty": f.ty} for f in t.struct.fields]}
    if isinstance(t, IRTypeDecl.Enum):
        return {"kind": "enum", "name": t.enum.name,
                "variants": list(t.enum.variants)}
    if isinstance(t, IRTypeDecl.Union):
        return {"kind": "union", "name": t.union.name,
                "variants": [
                    {
                        "kind": v.kind,
                        "fields": [{"name": f.name, "ty": f.ty} for f in v.fields],
                    }
                    for v in t.union.variants
                ]}
    if isinstance(t, IRTypeDecl.Trait):
        return {"kind": "trait", "name": t.trait.name,
                "methods": [
                    {
                        "name": m.name,
                        "params": [{"name": p.name, "ty": p.ty} for p in m.params],
                        "return_type": m.return_type,
                    }
                    for m in t.trait.methods
                ]}

def serialize_function(f):
    return {
        "name": f.name,
        "params": [{"name": p.name, "ty": p.ty} for p in f.params],
        "return_type": f.return_type,
        "body": serialize_statements(f.body),
    }

def serialize_statements(stmts):
    out = []
    for s in stmts:
        if isinstance(s, IRStatement.Let):
            out.append({"kind": "let", "name": s.name, "value": serialize_expr(s.value)})
        elif isinstance(s, IRStatement.Const):
            out.append({"kind": "const", "name": s.name, "value": serialize_expr(s.value)})
        elif isinstance(s, IRStatement.Return):
            out.append({"kind": "return", "value": serialize_expr(s.value)})
        elif isinstance(s, IRStatement.Loop):
            out.append({"kind": "loop", "body": serialize_statements(s.body)})
        elif isinstance(s, IRStatement.If):
            out.append({
                "kind": "if",
                "condition": serialize_expr(s.ifnode.condition),
                "then": serialize_statements(s.ifnode.then_branch),
                "else": serialize_statements(s.ifnode.else_branch) if s.ifnode.else_branch else None,
            })
        elif isinstance(s, IRStatement.Match):
            out.append({
                "kind": "match",
                "scrutinee": serialize_expr(s.matchnode.scrutinee),
                "arms": [
                    {
                        "pattern": arm.pattern,
                        "body": serialize_statements(arm.body),
                    }
                    for arm in s.matchnode.arms
                ],
                "otherwise": serialize_statements(s.matchnode.otherwise)
                if s.matchnode.otherwise else None,
            })
        elif isinstance(s, IRStatement.Expr):
            out.append({"kind": "expr", "expr": serialize_expr(s.expr)})
    return out

def serialize_expr(e):
    if isinstance(e, IRExpr.Literal):
        return {"kind": "literal", "value": e.literal.value}
    if isinstance(e, IRExpr.Binary):
        return {
            "kind": "binary",
            "op": e.op.kind,
            "left": serialize_expr(e.op.left),
            "right": serialize_expr(e.op.right),
        }
    if isinstance(e, IRExpr.Unary):
        return {
            "kind": "unary",
            "op": e.op.kind,
            "expr": serialize_expr(e.op.expr),
        }
    if isinstance(e, IRExpr.Call):
        return {
            "kind": "call",
            "func": e.func,
            "args": [serialize_expr(a) for a in e.args],
        }
    if isinstance(e, IRExpr.Var):
        return {"kind": "var", "name": e.name}
    if isinstance(e, IRExpr.Pipeline):
        return {
            "kind": "pipeline",
            "value": serialize_expr(e.value),
            "func": e.func,
        }
