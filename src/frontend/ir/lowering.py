from ir.nodes import *

def lower_module(ast):
    return IRModule(
        name=ast.name,
        imports=[lower_import(i) for i in ast.imports]
    )

def lower_import(ast):
    return IRImport(path=ast.path, alias=ast.alias)

def lower_types(ast_nodes):
    out = []
    for node in ast_nodes:
        if isinstance(node, ASTStruct):
            out.append(IRTypeDecl.Struct(lower_struct(node)))
        elif isinstance(node, ASTEnum):
            out.append(IRTypeDecl.Enum(lower_enum(node)))
        elif isinstance(node, ASTUnion):
            out.append(IRTypeDecl.Union(lower_union(node)))
        elif isinstance(node, ASTTrait):
            out.append(IRTypeDecl.Trait(lower_trait(node)))
    return out

def lower_struct(ast):
    return IRStruct(
        name=ast.name,
        fields=[IRField(f.name, f.ty) for f in ast.fields]
    )

def lower_enum(ast):
    return IREnum(name=ast.name, variants=ast.variants)

def lower_union(ast):
    return IRUnion(
        name=ast.name,
        variants=[
            IRUnionVariant(kind=v.kind, fields=[IRField(f.name, f.ty) for f in v.fields])
            for v in ast.variants
        ]
    )

def lower_trait(ast):
    return IRTrait(
        name=ast.name,
        methods=[
            IRFunctionSignature(
                name=m.name,
                params=[IRParam(p, "any") for p in m.params],
                return_type=m.return_type
            )
            for m in ast.methods
        ]
    )

def lower_function(ast):
    return IRFunction(
        name=ast.name,
        params=[IRParam(p, "any") for p in ast.params],
        return_type=ast.return_type,
        body=[lower_statement(s) for s in ast.body]
    )

def lower_statement(ast):
    if isinstance(ast, ASTLet):
        return IRStatement.Let(name=ast.name, value=lower_expr(ast.value))
    if isinstance(ast, ASTConst):
        return IRStatement.Const(name=ast.name, value=lower_expr(ast.value))
    if isinstance(ast, ASTReturn):
        return IRStatement.Return(lower_expr(ast.value))
    if isinstance(ast, ASTLoop):
        return IRStatement.Loop([lower_statement(s) for s in ast.body])
    if isinstance(ast, ASTIf):
        return IRStatement.If(IRIf(
            condition=lower_expr(ast.condition),
            then_branch=[lower_statement(s) for s in ast.then_branch],
            else_branch=[lower_statement(s) for s in ast.else_branch],
        ))
    if isinstance(ast, ASTMatch):
        return IRStatement.Match(IRMatch(
            scrutinee=lower_expr(ast.scrutinee),
            arms=[
                IRMatchArm(pattern=a.pattern, body=[lower_statement(s) for s in a.body])
                for a in ast.arms
            ],
            otherwise=[lower_statement(s) for s in ast.otherwise] if ast.otherwise else None
        ))
    if isinstance(ast, ASTExpr):
        return IRStatement.Expr(lower_expr(ast))

    raise Exception("Unknown AST statement")

def lower_expr(ast):
    if isinstance(ast, ASTLiteral):
        return IRExpr.Literal(lower_literal(ast))
    if isinstance(ast, ASTBinary):
        return IRExpr.Binary(IRBinaryOp(kind=ast.kind, left=lower_expr(ast.left), right=lower_expr(ast.right)))
    if isinstance(ast, ASTUnary):
        return IRExpr.Unary(IRUnaryOp(kind=ast.kind, expr=lower_expr(ast.expr)))
    if isinstance(ast, ASTCall):
        return IRExpr.Call(func=ast.func, args=[lower_expr(a) for a in ast.args])
    if isinstance(ast, ASTVar):
        return IRExpr.Var(ast.name)
    if isinstance(ast, ASTPipeline):
        return IRExpr.Pipeline(value=lower_expr(ast.value), func=ast.func)

    raise Exception("Unknown AST expression")

def lower_literal(ast):
    return IRLiteral(ast.value)
