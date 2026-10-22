use crate::diagnostics::span::SourceSpan;
use serde::Serialize;
use std::collections::HashMap;
use std::fmt;

// ===============================
// IR MODULE
// ===============================

#[derive(Clone, Debug, Serialize)]
pub struct IRRoot {
    pub module: IRModule,

    pub types: Vec<IRTypeDecl>,

    pub traits: Vec<IRTrait>,

    pub impls: Vec<IRImpl>,

    pub functions: Vec<IRFunction>,

    pub statements: Vec<IRStatement>,
}

#[derive(Clone, Debug, Serialize)]
pub struct IRModule {
    pub name: String,
    pub imports: Vec<IRImport>,
}

#[derive(Clone, Debug, Serialize)]
pub struct IRImport {
    pub path: String,
    pub alias: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct IRMove {
    pub source: IRExpr,
    pub target: IRExpr,
}

#[derive(Clone, Debug, Serialize)]
pub struct IRClone {
    pub source: IRExpr,
    pub target: IRExpr,
}

// ===============================
// IR TYPES
// ===============================

#[derive(Clone, Debug, Serialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum IRTypeDecl {
    Struct(IRStruct),
    Enum(IREnum),
    Union(IRUnion),
    Trait(IRTrait),
}

#[derive(Clone, Debug, Serialize)]
pub struct IRStruct {
    pub name: String,
    pub fields: Vec<IRField>,
}

#[derive(Clone, Debug, Serialize)]
pub struct IRField {
    pub name: String,
    pub ty: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct IREnum {
    pub name: String,
    pub variants: Vec<String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct IRUnion {
    pub name: String,
    pub variants: Vec<IRUnionVariant>,
}

#[derive(Clone, Debug, Serialize)]
pub struct IRUnionVariant {
    pub kind: String,
    pub fields: Vec<IRField>,
}

#[derive(Clone, Debug, Serialize)]
pub struct IRTrait {
    pub name: String,
    pub methods: Vec<IRFunctionSignature>,
}

#[derive(Clone, Debug, Serialize)]
pub struct IRFunctionSignature {
    pub name: String,
    pub params: Vec<IRParam>,
    pub return_type: Option<String>,
}

// ===============================
// IR IMPLEMENTATIONS (IMPL)
// ===============================

#[derive(Clone, Debug, Serialize)]
pub struct IRImpl {
    pub trait_name: String,
    pub target_type: String,
    pub methods: Vec<IRFunction>,
}

// ===============================
// IR FUNCTIONS
// ===============================

#[derive(Clone, Debug, Serialize)]
pub struct IRFunction {
    pub name: String,
    pub span: SourceSpan,
    pub params: Vec<IRParam>,
    pub return_type: Option<String>,
    pub is_async: bool,
    pub body: Vec<IRStatement>,
}

#[derive(Clone, Debug, Serialize)]
pub struct IRParam {
    pub name: String,
    pub ty: String,
}

// ===============================
// IR STATEMENTS
// ===============================

#[derive(Clone, Debug, Serialize)]
pub enum IRStatement {
    Let { name: String, value: IRExpr },
    Const { name: String, value: IRExpr },
    Return(IRExpr),
    Loop(Vec<IRStatement>),
    If(IRIf),
    Match(IRMatch),
    Try(IRTry),
    Expr(IRExpr),
    Move(IRMove),
    Clone(IRClone),
}

// ===============================
// IR CONTROL FLOW
// ===============================

#[derive(Clone, Debug, Serialize)]
pub struct IRIf {
    pub condition: IRExpr,
    pub then_branch: Vec<IRStatement>,
    pub else_branch: Vec<IRStatement>,
}

#[derive(Clone, Debug, Serialize)]
pub struct IRMatch {
    pub scrutinee: IRExpr,
    pub arms: Vec<IRMatchArm>,
    pub otherwise: Option<Vec<IRStatement>>,
}

#[derive(Clone, Debug, Serialize)]
pub struct IRMatchArm {
    pub pattern: String,
    pub body: Vec<IRStatement>,
}

#[derive(Clone, Debug, Serialize)]
pub struct IRTry {
    pub try_body: Vec<IRStatement>,
    pub catch_body: Vec<IRStatement>,
    pub finally_body: Vec<IRStatement>,
}

// ===============================
// IR EXPRESSIONS
// ===============================

#[derive(Clone, Debug, Serialize)]
pub struct IRVar {
    pub name: String,
    pub span: SourceSpan,
}

#[derive(Clone, Debug, Serialize)]
pub enum IRExpr {
    Literal(IRLiteral),
    Binary(Box<IRBinaryOp>),
    Unary(Box<IRUnaryOp>),
    Call { func: String, args: Vec<IRExpr> },
    Var(IRVar),
    Pipeline { value: Box<IRExpr>, func: String },
    ObjectInit(IRObjectInit),
}

// ===============================
// IR OPERATORS
// ===============================

#[derive(Clone, Debug, Serialize)]
pub struct IRBinaryOp {
    pub kind: String, // ADD, SUB, MUL, DIV, EQ, GT, etc.
    pub left: Box<IRExpr>,
    pub right: Box<IRExpr>,
}

#[derive(Clone, Debug, Serialize)]
pub struct IRObjectInit {
    pub type_name: String,
    pub fields: HashMap<String, IRExpr>,
}

#[derive(Clone, Debug, Serialize)]
pub struct IRUnaryOp {
    pub kind: String, // NOT
    pub expr: Box<IRExpr>,
}
// ===============================
// IR LITERALS
// ===============================

#[derive(Clone, Debug, Serialize)]
pub enum IRLiteral {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    None,
    List(Vec<IRExpr>),
    Map(HashMap<String, IRExpr>),
    Lambda {
        params: Vec<String>,
        body: Box<IRExpr>,
    },
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
                let rendered = items
                    .iter()
                    .map(|item| item.to_string())
                    .collect::<Vec<_>>();

                write!(f, "[{}]", rendered.join(", "))
            }

            IRLiteral::Map(entries) => {
                let mut fields = entries.iter().collect::<Vec<_>>();

                fields.sort_by(|left, right| left.0.cmp(right.0));

                let rendered = fields
                    .into_iter()
                    .map(|(name, value)| format!("{}: {}", name, value))
                    .collect::<Vec<_>>();

                write!(f, "{{{}}}", rendered.join(", "))
            }

            IRLiteral::Lambda { params, body } => {
                write!(f, "FN({}) -> {}", params.join(", "), body)
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

            IRExpr::Var(var) => {
                write!(f, "{}", var.name.to_lowercase())
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

            IRExpr::Unary(op) => {
                let op_str = match op.kind.as_str() {
                    "NOT" => "not",
                    other => other,
                };

                write!(f, "{} {}", op_str, op.expr)
            }

            IRExpr::Call { func, args } => {
                let rendered_args: Vec<String> = args.iter().map(|arg| arg.to_string()).collect();

                write!(f, "{}({})", func.to_lowercase(), rendered_args.join(", "))
            }

            IRExpr::Pipeline { value, func } => {
                write!(f, "{} |> {}", value, func.to_lowercase())
            }

            IRExpr::ObjectInit(init) => {
                let mut fields = init.fields.iter().collect::<Vec<_>>();
                fields.sort_by(|a, b| a.0.cmp(b.0));

                let fields = fields
                    .into_iter()
                    .map(|(name, value)| format!("{}: {}", name, value))
                    .collect::<Vec<_>>()
                    .join(", ");

                write!(f, "{}({})", init.type_name, fields)
            }
        }
    }
}
