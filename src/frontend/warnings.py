from src.frontend.ast.nodes import *

def collect_usage(node, declared, used):

    if isinstance(node, ASTModule):
        for item in node.body:
            collect_usage(item, declared, used)

    elif isinstance(node, ASTFunction):
        for stmt in node.body:
            collect_usage(stmt, declared, used)

    elif isinstance(node, ASTLet):
        declared[node.name] = node
        collect_usage(node.value, declared, used)

    elif isinstance(node, ASTVar):
        used.add(node.name)

    elif isinstance(node, ASTConst):
        declared[node.name] = node
        collect_usage(node.value, declared, used)

    elif isinstance(node, ASTBinary):
        collect_usage(node.left, declared, used)
        collect_usage(node.right, declared, used)

    elif isinstance(node, ASTUnary):
        collect_usage(node.expr, declared, used)

    elif isinstance(node, ASTCall):
        for arg in node.args:
            collect_usage(arg, declared, used)

    elif isinstance(node, ASTReturn):
        collect_usage(node.value, declared, used)

    elif isinstance(node, ASTIf):
        collect_usage(node.condition, declared, used)

        for stmt in node.then_branch:
            collect_usage(stmt, declared, used)

        for stmt in node.else_branch:
            collect_usage(stmt, declared, used)


def find_unused_variables(module):


    declared = {}
    used = set()

    collect_usage(module, declared, used)

    warnings = []

    for name, node in declared.items():

        if name not in used:

            warnings.append(
                {
                    "code": "01",
                    "message": (f"NXD-W2001: Variable '{name}' declared but never used."),
                    "line": node.line,
                    "col": node.col,
                    "severity": "warning"
                    
                }
            )


    return warnings

def find_unreachable_code(module):

    warnings = []

    for item in module.body:

        if not isinstance(item, ASTFunction):
            continue

        found_return = False

        for stmt in item.body:

            if found_return:

                warnings.append(
                    {
                        "code": "NXD-W2002",
                        "message": (f"NXD-W2002: Unreachable code."),
                        "line": stmt.line,
                        "col": stmt.col,
                        "severity": "warning"
                    }
                )

            if isinstance(stmt, ASTReturn):
                found_return = True

    return warnings

def find_shadowed_bindings(module):
    warnings = []

    def visit_statements(statements, outer_names=None):
        if outer_names is None:
            outer_names = set()

        current_names = set()

        for stmt in statements:
            if isinstance(stmt, (ASTLet, ASTConst)):
                name = stmt.name

                if name in current_names or name in outer_names:
                    warnings.append(
                        {
                            "code": "NXD-W2003",
                            "message": (f"NXD-W2003: Binding '{name}' shadows a previous binding."),
                            "line": stmt.line,
                            "col": stmt.col,
                        }
                    )

                current_names.add(name)

            if isinstance(stmt, ASTFunction):
                visit_statements(
                    stmt.body,
                    outer_names | current_names,
                )

            elif isinstance(stmt, ASTIf):
                visible_names = outer_names | current_names

                visit_statements(
                    stmt.then_branch,
                    visible_names,
                )

                visit_statements(
                    stmt.else_branch,
                    visible_names,
                )

            elif isinstance(stmt, ASTLoop):
                visit_statements(
                    stmt.body,
                    outer_names | current_names,
                )

    if isinstance(module, ASTModule):
        visit_statements(module.body)

    return warnings

def find_unused_imports(module):
    warnings = []



    return warnings