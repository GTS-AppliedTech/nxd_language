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
                    "line": node.line,
                    "col": node.col,
                    "message": f"Variable '{name}' declared but never used."
                }
            )


    return warnings