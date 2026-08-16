from dataclasses import dataclass
from typing import List, Optional, Union

# ============================
# AST MODULE
# ============================

@dataclass
class ASTModule:
    name: str
    imports: List["ASTImport"]
    body: List["ASTNode"]


@dataclass
class ASTImport:
    path: str
    alias: Optional[str]


# ============================
# AST TYPES
# ============================

@dataclass
class ASTStruct:
    name: str
    fields: List["ASTField"]


@dataclass
class ASTField:
    name: str
    ty: str


@dataclass
class ASTEnum:
    name: str
    variants: List[str]


@dataclass
class ASTUnion:
    name: str
    variants: List["ASTUnionVariant"]


@dataclass
class ASTUnionVariant:
    kind: str
    fields: List[ASTField]


@dataclass
class ASTTrait:
    name: str
    methods: List["ASTFunctionSignature"]


@dataclass
class ASTFunctionSignature:
    name: str
    params: List[str]
    return_type: str


# ============================
# AST IMPLEMENTATIONS
# ============================

@dataclass
class ASTImpl:
    trait_name: str
    target_type: str
    methods: List["ASTFunction"]


# ============================
# AST FUNCTIONS
# ============================

@dataclass
class ASTFunction:
    name: str
    params: List[str]
    return_type: Optional[str]
    body: List["ASTStatement"]


# ============================
# AST STATEMENTS
# ============================

@dataclass
class ASTLet:
    name: str
    value: "ASTExpr"


@dataclass
class ASTConst:
    name: str
    value: "ASTExpr"


@dataclass
class ASTReturn:
    value: "ASTExpr"


@dataclass
class ASTLoop:
    body: List["ASTStatement"]


@dataclass
class ASTIf:
    condition: "ASTExpr"
    then_branch: List["ASTStatement"]
    else_branch: List["ASTStatement"]


@dataclass
class ASTMatch:
    scrutinee: "ASTExpr"
    arms: List["ASTMatchArm"]
    otherwise: Optional[List["ASTStatement"]]


@dataclass
class ASTMatchArm:
    pattern: str
    body: List["ASTStatement"]


ASTStatement = Union[
    ASTLet, ASTConst, ASTReturn, ASTLoop, ASTIf, ASTMatch, "ASTExpr"
]


# ============================
# AST EXPRESSIONS
# ============================

@dataclass
class ASTLiteral:
    value: Union[int, float, str, bool, None, list]


@dataclass
class ASTBinary:
    kind: str
    left: "ASTExpr"
    right: "ASTExpr"


@dataclass
class ASTUnary:
    kind: str
    expr: "ASTExpr"


@dataclass
class ASTCall:
    func: str
    args: List["ASTExpr"]


@dataclass
class ASTVar:
    name: str


@dataclass
class ASTPipeline:
    value: "ASTExpr"
    func: str


ASTExpr = Union[
    ASTLiteral, ASTBinary, ASTUnary, ASTCall, ASTVar, ASTPipeline
]

ASTNode = Union[
    ASTStruct, ASTEnum, ASTUnion, ASTTrait, ASTImpl, ASTFunction,
    ASTStatement
]
