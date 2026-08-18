use serde::Deserialize;
use std::fmt;
// ===============================
// IR MODULE
// ===============================

#[derive(Clone, Debug, Deserialize)]
pub struct IRModule {
    pub name: String,
    pub imports: Vec<IRImport>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct IRImport {
    pub path: String,
    pub alias: Option<String>,
}


// ===============================
// IR TYPES
// ===============================

#[derive(Clone, Debug, Deserialize)]
pub enum IRTypeDecl {
    Struct(IRStruct),
    Enum(IREnum),
    Union(IRUnion),
    Trait(IRTrait),
}

#[derive(Clone, Debug, Deserialize)]
pub struct IRStruct {
    pub name: String,
    pub fields: Vec<IRField>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct IRField {
    pub name: String,
    pub ty: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct IREnum {
    pub name: String,
    pub variants: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct IRUnion {
    pub name: String,
    pub variants: Vec<IRUnionVariant>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct IRUnionVariant {
    pub kind: String,
    pub fields: Vec<IRField>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct IRTrait {
    pub name: String,
    pub methods: Vec<IRFunctionSignature>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct IRFunctionSignature {
    pub name: String,
    pub params: Vec<IRParam>,
    pub return_type: String,
}


// ===============================
// IR IMPLEMENTATIONS (IMPL)
// ===============================

#[derive(Clone, Debug, Deserialize)]
pub struct IRImpl {
    pub trait_name: String,
    pub target_type: String,
    pub methods: Vec<IRFunction>,
}


// ===============================
// IR FUNCTIONS
// ===============================

#[derive(Clone, Debug, Deserialize)]
pub struct IRFunction {
    pub name: String,
    pub params: Vec<IRParam>,
    pub return_type: Option<String>,
    pub body: Vec<IRStatement>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct IRParam {
    pub name: String,
    pub ty: String,
}


// ===============================
// IR STATEMENTS
// ===============================

#[derive(Clone, Debug, Deserialize)]
pub enum IRStatement {
    Let { name: String, value: IRExpr },
    Const { name: String, value: IRExpr },
    Return(IRExpr),
    Loop(Vec<IRStatement>),
    If(IRIf),
    Match(IRMatch),
    Expr(IRExpr),
}


// ===============================
// IR CONTROL FLOW
// ===============================

#[derive(Clone, Debug, Deserialize)]
pub struct IRIf {
    pub condition: IRExpr,
    pub then_branch: Vec<IRStatement>,
    pub else_branch: Vec<IRStatement>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct IRMatch {
    pub scrutinee: IRExpr,
    pub arms: Vec<IRMatchArm>,
    pub otherwise: Option<Vec<IRStatement>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct IRMatchArm {
    pub pattern: String,
    pub body: Vec<IRStatement>,
}

// ===============================
// IR EXPRESSIONS
// ===============================

#[derive(Clone, Debug, Deserialize)]
pub enum IRExpr {
    Literal(IRLiteral),
    Binary(Box<IRBinaryOp>),
    Unary(Box<IRUnaryOp>),
    Call { func: String, args: Vec<IRExpr> },
    Var(String),
    Pipeline { value: Box<IRExpr>, func: String },
}


// ===============================
// IR OPERATORS
// ===============================

#[derive(Clone, Debug, Deserialize)]
pub struct IRBinaryOp {
    pub kind: String,     // ADD, SUB, MUL, DIV, EQ, GT, etc.
    pub left: Box<IRExpr>,
    pub right: Box<IRExpr>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct IRUnaryOp {
    pub kind: String,     // NOT
    pub expr: Box<IRExpr>,
}
// ===============================
// IR LITERALS
// ===============================

#[derive(Clone, Debug, Deserialize)]
pub enum IRLiteral {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    None,
    List(Vec<IRLiteral>),
}
impl fmt::Display for IRLiteral {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IRLiteral::Int(v) => write!(f, "{}", v),
            IRLiteral::Float(v) => write!(f, "{}", v),
            IRLiteral::String(v) => write!(f, "\"{}\"", v),
            IRLiteral::Bool(v) => write!(f, "{}", v),
            IRLiteral::None => write!(f, "nil"),
            IRLiteral::List(items) => {
                let rendered: Vec<String> = items
                    .iter()
                    .map(|item| item.to_string())
                    .collect();

                write!(f, "[{}]", rendered.join(", "))
            }
        }
    }
}

impl fmt::Display for IRExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IRExpr::Literal(lit) => {
                write!(f, "{}", lit)
            }

            IRExpr::Var(name) => {
                write!(f, "{}", name.to_lowercase())
            }

            IRExpr::Binary(op) => {
                let op_str = match op.kind.as_str() {
                    "ADD" => "+",
                    "SUB" => "-",
                    "MUL" => "*",
                    "DIV" => "/",
                    "MOD" => "mod",
                    "EQ" => "==",
                    "NEQ" => "!=",
                    "GT" => ">",
                    "LT" => "<",
                    "GTE" => ">=",
                    "LTE" => "<=",
                    "AND" => "and",
                    "OR" => "or",
                    other => other,
                };

                write!(f, "{} {} {}", op.left, op_str, op.right)
            }

            IRExpr::Unary (op) => {
                let op_str = match op.kind.as_str() {
                    "NOT" => "not",
                    other => other,
                };

                write!(f, "{} {}", op_str, op.expr)
            }

            IRExpr::Call { func, args } => {
                let rendered_args: Vec<String> = args
                    .iter()
                    .map(|arg| arg.to_string())
                    .collect();

                write!(
                    f,
                    "{}({})",
                    func.to_lowercase(),
                    rendered_args.join(", ")
                )
            }

            IRExpr::Pipeline { value, func } => {
                write!(f, "{} |> {}", value, func.to_lowercase())
            }
        }
    }
}