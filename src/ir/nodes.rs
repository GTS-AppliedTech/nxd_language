// ===============================
// IR MODULE
// ===============================

#[derive(Clone, Debug)]
pub struct IRModule {
    pub name: String,
    pub imports: Vec<IRImport>,
}

#[derive(Clone, Debug)]
pub struct IRImport {
    pub path: String,
    pub alias: Option<String>,
}


// ===============================
// IR TYPES
// ===============================

#[derive(Clone, Debug)]
pub enum IRTypeDecl {
    Struct(IRStruct),
    Enum(IREnum),
    Union(IRUnion),
    Trait(IRTrait),
}

#[derive(Clone, Debug)]
pub struct IRStruct {
    pub name: String,
    pub fields: Vec<IRField>,
}

#[derive(Clone, Debug)]
pub struct IRField {
    pub name: String,
    pub ty: String,
}

#[derive(Clone, Debug)]
pub struct IREnum {
    pub name: String,
    pub variants: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct IRUnion {
    pub name: String,
    pub variants: Vec<IRUnionVariant>,
}

#[derive(Clone, Debug)]
pub struct IRUnionVariant {
    pub kind: String,
    pub fields: Vec<IRField>,
}

#[derive(Clone, Debug)]
pub struct IRTrait {
    pub name: String,
    pub methods: Vec<IRFunctionSignature>,
}

#[derive(Clone, Debug)]
pub struct IRFunctionSignature {
    pub name: String,
    pub params: Vec<IRParam>,
    pub return_type: String,
}


// ===============================
// IR IMPLEMENTATIONS (IMPL)
// ===============================

#[derive(Clone, Debug)]
pub struct IRImpl {
    pub trait_name: String,
    pub target_type: String,
    pub methods: Vec<IRFunction>,
}


// ===============================
// IR FUNCTIONS
// ===============================

#[derive(Clone, Debug)]
pub struct IRFunction {
    pub name: String,
    pub params: Vec<IRParam>,
    pub return_type: Option<String>,
    pub body: Vec<IRStatement>,
}

#[derive(Clone, Debug)]
pub struct IRParam {
    pub name: String,
    pub ty: String,
}


// ===============================
// IR STATEMENTS
// ===============================

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
pub struct IRIf {
    pub condition: IRExpr,
    pub then_branch: Vec<IRStatement>,
    pub else_branch: Vec<IRStatement>,
}

#[derive(Clone, Debug)]
pub struct IRMatch {
    pub scrutinee: IRExpr,
    pub arms: Vec<IRMatchArm>,
    pub otherwise: Option<Vec<IRStatement>>,
}

#[derive(Clone, Debug)]
pub struct IRMatchArm {
    pub pattern: String,
    pub body: Vec<IRStatement>,
}


// ===============================
// IR EXPRESSIONS
// ===============================

#[derive(Clone, Debug)]
pub enum IRExpr {
    Literal(IRLiteral),
    Binary(IRBinaryOp),
    Unary(IRUnaryOp),
    Call { func: String, args: Vec<IRExpr> },
    Var(String),
    Pipeline { value: Box<IRExpr>, func: String },
}


// ===============================
// IR OPERATORS
// ===============================

#[derive(Clone, Debug)]
pub struct IRBinaryOp {
    pub kind: String,     // ADD, SUB, MUL, DIV, EQ, GT, etc.
    pub left: IRExpr,
    pub right: IRExpr,
}

#[derive(Clone, Debug)]
pub struct IRUnaryOp {
    pub kind: String,     // NOT
    pub expr: IRExpr,
}


// ===============================
// IR LITERALS
// ===============================

#[derive(Clone, Debug)]
pub enum IRLiteral {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    None,
    List(Vec<IRLiteral>),
}
