use codespan::{ self, ByteIndex };

pub type Span = codespan::Span<ByteIndex>;

#[derive(Debug, Clone, PartialEq)]
pub struct File{
    pub items: Vec<Item>,
    pub span: Span
}

#[derive(Debug, Clone, PartialEq)]
pub enum Item {
    Extern(FunctionDecl),
    Function(Function),
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDecl{
    pub ident: Ident,
    pub args: Vec<Indent>,
    pub span: Span
}

#[derive(Debug, Clone, PartialEq)]
pub struct Function{
    pub decl: FunctionDecl,
    pub body: Expr,
    pub span: Span
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr{
    Indent(Indent),
    Literial(Literial),
    FunctionCall(FunctionCall),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Indent{
    pub name: String,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Literial{
    pub value: f32,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionCall{
    pub indent: Indent,
    pub args: Vec<Indent>,
    pub span: Span,
}