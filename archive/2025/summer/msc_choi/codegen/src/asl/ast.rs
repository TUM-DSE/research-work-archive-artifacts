#![allow(unused)]

use crate::asl::CodegenError;
use crate::unwrap_node_data;
use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use strum_macros::{Display, EnumString};

#[derive(Debug, Clone, Deserialize)]
pub struct AstNode {
    pub node_type: NodeType,
    pub node_subtype: NodeSubtype,
    pub node_data: Option<Box<NodeData>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum NodeType {
    Decls,
    Decl,
    Stmt,
    Expr,
    LExpr,
    Type,
    Binop,
    Unop,
    Index,
    Formal,
    Encoding,
    IField,
    Opcode,
    SElsif,
    EElsif,
    Alt,
    Pat,
    Direction,
    Catcher,
    Slice,
    DecoderCase,
    DecoderSlice,
    DecoderAlt,
    DecoderPattern,
    DecoderBody,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum NodeSubtype {
    // Decls:
    Decls,

    // Decl:
    BuiltinType,
    Forward,
    Record,
    Typedef,
    Enum,  // shared with Index node_type
    Var,   // shared with Expr and LExpr node_type
    Const, // shared with Pat node_type
    BuiltinFunction,
    FunType,
    FunDefn,
    ProcType,
    ProcDefn,
    VarGetterType,
    VarGetterDefn,
    ArrayGetterType,
    ArrayGetterDefn,
    VarSetterType,
    VarSetterDefn,
    ArraySetterType,
    ArraySetterDefn,
    InstructionDefn,
    DecoderDefn,
    Operator1,
    Operator2,
    NewEventDefn,
    EventClause,
    NewMapDefn,
    MapClause,
    Config,

    // Stmt:
    VarDeclsNoInit,
    VarDecl,
    ConstDecl,
    Assign,
    FunReturn,
    ProcReturn,
    Assert,
    Unpred,
    ConstrainedUnpred,
    ImpDef, // shared with Expr node_type
    Undefined,
    ExceptionTaken,
    DepUnpred,
    DepImpDef,
    DepUndefined,
    See,
    Throw,
    DecodeExecute,
    TCall,
    If,   // shared with Expr node_type
    Case, // shared with DecoderCase node_type
    For,
    While,
    Repeat,
    Try,

    // Expr:
    Binop,
    Unop,
    Field,  // shared with LExpr and IField node_type
    Fields, // shared with LExpr node_type
    Slices, // shared with LExpr node_type
    In,     // shared with Formal node_type
    Parens,
    Tuple, // shared with LExpr, Type, and Pat node_type
    Unknown,
    TApply,
    Array,  // shared with LExpr and Type node_type
    LitInt, // shared with Pat node_type
    LitHex, // shared with Pat node_type
    LitReal,
    LitBits, // shared with Pat node_type
    LitMask, // shared with Pat node_type
    LitString,

    // LExpr:
    Wildcard, // shared with DecoderPattern node_type
    BitTuple,
    Write,
    ReadWrite,

    // Type:
    Constructor,
    Bits, // shared with Opcode and DecoderPattern node_type
    App,
    OfExpr,
    Register,

    // Binop:
    Eq,
    NtEq,
    Gt,
    GtEq,
    Lt,
    LtEq,
    Plus,
    Minus,
    Multiply,
    Divide,
    Power,
    Quot,
    Rem,
    Div,
    Mod,
    ShiftL,
    ShiftR,
    BoolAnd,
    BoolOr,
    BoolIff,
    BoolImplies,
    BitOr,
    BitEor,
    BitAnd,
    Append,
    Concat, // shared with DecoderSlice node_type

    // Unop:
    Negate,
    BoolNot,
    BitsNot,

    // Index:
    Range, // shared with Pat node_type

    // Formal:
    InOut,

    // Encoding:
    Block,

    // Opcode:
    Mask, // shared with DecoderPattern node_type

    // SElsif, EElsif:
    Cond,

    // Alt:
    Alt, // shared with DecoderAlt node_type

    // Pat:
    Set,
    Single, // shared with Slice node_type

    // Direction:
    Up,
    Down,

    // Catcher:
    Guarded,

    // Slice:
    HiLo,
    LoWd,

    // DecoderSlice:
    Slice,
    FieldName,

    // DecoderPattern:
    Not,

    // DecoderBody:
    UNPRED,
    UNALLOC,
    NOP,
    Encoding,
    Decoder,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "serde_tag")]
pub enum NodeData {
    Decls(DeclsData),

    DeclBuiltinType(DeclBuiltinTypeData),
    DeclForward(DeclForwardData),
    DeclRecord(DeclRecordData),
    DeclTypedef(DeclTypedefData),
    DeclEnum(DeclEnumData),
    DeclVar(DeclVarData),
    DeclConst(DeclConstData),
    DeclBuiltinFunction(DeclBuiltinFunctionData),
    DeclFunType(DeclFunTypeData),
    DeclFunDefn(DeclFunDefnData),
    DeclProcType(DeclProcTypeData),
    DeclProcDefn(DeclProcDefnData),
    DeclVarGetterType(DeclVarGetterTypeData),
    DeclVarGetterDefn(DeclVarGetterDefnData),
    DeclArrayGetterType(DeclArrayGetterTypeData),
    DeclArrayGetterDefn(DeclArrayGetterDefnData),
    DeclVarSetterType(DeclVarSetterTypeData),
    DeclVarSetterDefn(DeclVarSetterDefnData),
    DeclArraySetterType(DeclArraySetterTypeData),
    DeclArraySetterDefn(DeclArraySetterDefnData),
    DeclInstructionDefn(DeclInstructionDefnData),
    DeclDecoderDefn(DeclDecoderDefnData),
    DeclOperator1(DeclOperator1Data),
    DeclOperator2(DeclOperator2Data),
    DeclNewEventDefn(DeclNewEventDefnData),
    DeclEventClause(DeclEventClauseData),
    DeclNewMapDefn(DeclNewMapDefnData),
    DeclMapClause(DeclMapClauseData),
    DeclConfig(DeclConfigData),

    StmtVarDeclsNoInit(StmtVarDeclsNoInitData),
    StmtVarDecl(StmtVarDeclData),
    StmtConstDecl(StmtConstDeclData),
    StmtAssign(StmtAssignData),
    StmtFunReturn(StmtFunReturnData),
    StmtProcReturn(StmtProcReturnData),
    StmtAssert(StmtAssertData),
    StmtUnpred(StmtUnpredData),
    StmtConstrainedUnpred(StmtConstrainedUnpredData),
    StmtImpDef(StmtImpDefData),
    StmtUndefined(StmtUndefinedData),
    StmtExceptionTaken(StmtExceptionTakenData),
    StmtDepUnpred(StmtDepUnpredData),
    StmtDepImpDef(StmtDepImpDefData),
    StmtDepUndefined(StmtDepUndefinedData),
    StmtSee(StmtSeeData),
    StmtThrow(StmtThrowData),
    StmtDecodeExecute(StmtDecodeExecuteData),
    StmtTCall(StmtTCallData),
    StmtIf(StmtIfData),
    StmtCase(StmtCaseData),
    StmtFor(StmtForData),
    StmtWhile(StmtWhileData),
    StmtRepeat(StmtRepeatData),
    StmtTry(StmtTryData),

    ExprIf(ExprIfData),
    ExprBinop(ExprBinopData),
    ExprUnop(ExprUnopData),
    ExprField(ExprFieldData),
    ExprFields(ExprFieldsData),
    ExprSlices(ExprSlicesData),
    ExprIn(ExprInData),
    ExprVar(ExprVarData),
    ExprParens(ExprParensData),
    ExprTuple(ExprTupleData),
    ExprUnknown(ExprUnknownData),
    ExprImpDef(ExprImpDefData),
    ExprTApply(ExprTApplyData),
    ExprArray(ExprArrayData),
    ExprLitInt(ExprLitIntData),
    ExprLitHex(ExprLitHexData),
    ExprLitReal(ExprLitRealData),
    ExprLitBits(ExprLitBitsData),
    ExprLitMask(ExprLitMaskData),
    ExprLitString(ExprLitStringData),

    LExprVar(LExprVarData),
    LExprField(LExprFieldData),
    LExprFields(LExprFieldsData),
    LExprSlices(LExprSlicesData),
    LExprBitTuple(LExprBitTupleData),
    LExprTuple(LExprTupleData),
    LExprArray(LExprArrayData),
    LExprWrite(LExprWriteData),
    LExprReadWrite(LExprReadWriteData),

    TypeConstructor(TypeConstructorData),
    TypeBits(TypeBitsData),
    TypeApp(TypeAppData),
    TypeOfExpr(TypeOfExprData),
    TypeRegister(TypeRegisterData),
    TypeArray(TypeArrayData),
    TypeTuple(TypeTupleData),

    IndexEnum(IndexEnumData),
    IndexRange(IndexRangeData),

    FormalIn(FormalInData),
    FormalInOut(FormalInOutData),

    EncodingBlock(EncodingBlockData),

    IField(IFieldData),

    OpcodeBits(OpcodeBitsData),
    OpcodeMask(OpcodeMaskData),

    SElsifCond(SElsifCondData),
    EElsifCond(EElsifCondData),

    Alt(AltData),

    PatLitInt(PatLitIntData),
    PatLitHex(PatLitHexData),
    PatLitBits(PatLitBitsData),
    PatLitMask(PatLitMaskData),
    PatConst(PatConstData),
    PatTuple(PatTupleData),
    PatSet(PatSetData),
    PatRange(PatRangeData),
    PatSingle(PatSingleData),

    CatcherGuarded(CatcherGuardedData),

    SliceSingle(SliceSingleData),
    SliceHiLo(SliceHiLoData),
    SliceLoWd(SliceLoWdData),

    DecoderCase(DecoderCaseData),

    DecoderSlice(DecoderSliceData),
    DecoderSliceFieldName(DecoderSliceFieldNameData),
    DecoderSliceConcat(DecoderSliceConcatData),

    DecoderAlt(DecoderAltData),

    DecoderPatternBits(DecoderPatternBitsData),
    DecoderPatternMask(DecoderPatternMaskData),
    DecoderPatternWildcard(DecoderPatternWildcardData),
    DecoderPatternNot(DecoderPatternNotData),

    DecoderBodyUNPRED(DecoderBodyUNPREDData),
    DecoderBodyUNALLOC(DecoderBodyUNALLOCData),
    DecoderBodyNOP(DecoderBodyNOPData),
    DecoderBodyEncoding(DecoderBodyEncodingData),
    DecoderBodyDecoder(DecoderBodyDecoderData),
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeclsData {
    pub decls: Vec<AstNode>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeclBuiltinTypeData {
    pub ident: String,
    pub l: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeclForwardData {
    pub ident: String,
    pub l: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeclRecordData {
    pub ident: String,
    pub fields: Vec<TypedIdentifier>,
    pub l: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeclTypedefData {
    pub ident: String,
    pub ty: AstNode,
    pub l: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeclEnumData {
    pub ident: String,
    pub idents: Vec<String>,
    pub l: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeclVarData {
    pub ty: AstNode,
    pub ident: String,
    pub l: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeclConstData {
    pub ty: AstNode,
    pub ident: String,
    pub expr: AstNode,
    pub l: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeclBuiltinFunctionData {
    pub ty: AstNode,
    pub ident: String,
    pub params: Vec<TypedIdentifier>,
    pub l: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeclFunTypeData {
    pub ty: AstNode,
    pub ident: String,
    pub params: Vec<TypedIdentifier>,
    pub l: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeclFunDefnData {
    pub ty: AstNode,
    pub ident: String,
    pub params: Vec<TypedIdentifier>,
    pub stmts: Vec<AstNode>,
    pub l: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeclProcTypeData {
    pub ident: String,
    pub params: Vec<TypedIdentifier>,
    pub l: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeclProcDefnData {
    pub ident: String,
    pub params: Vec<TypedIdentifier>,
    pub stmts: Vec<AstNode>,
    pub l: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeclVarGetterTypeData {
    pub ty: AstNode,
    pub ident: String,
    pub l: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeclVarGetterDefnData {
    pub ty: AstNode,
    pub ident: String,
    pub stmts: Vec<AstNode>,
    pub l: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeclArrayGetterTypeData {
    pub ty: AstNode,
    pub ident: String,
    pub params: Vec<TypedIdentifier>,
    pub l: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeclArrayGetterDefnData {
    pub ty: AstNode,
    pub ident: String,
    pub params: Vec<TypedIdentifier>,
    pub stmts: Vec<AstNode>,
    pub l: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeclVarSetterTypeData {
    pub ident1: String,
    pub ty: AstNode,
    pub ident2: String,
    pub l: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeclVarSetterDefnData {
    pub ident1: String,
    pub ty: AstNode,
    pub ident2: String,
    pub stmts: Vec<AstNode>,
    pub l: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeclArraySetterTypeData {
    pub ident1: String,
    pub params: Vec<AstNode>,
    pub ty: AstNode,
    pub ident2: String,
    pub l: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeclArraySetterDefnData {
    pub ident1: String,
    pub params: Vec<AstNode>,
    pub ty: AstNode,
    pub ident2: String,
    pub stmts: Vec<AstNode>,
    pub l: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeclInstructionDefnData {
    pub ident: String,
    pub encodings: Vec<AstNode>,
    pub opt_stmts: Option<Vec<AstNode>>,
    pub bool: bool,
    pub stmts: Vec<AstNode>,
    pub l: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeclDecoderDefnData {
    pub ident: String,
    pub decode_case: AstNode,
    pub l: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeclOperator1Data {
    pub unop: AstNode,
    pub idents: Vec<String>,
    pub l: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeclOperator2Data {
    pub binop: AstNode,
    pub idents: Vec<String>,
    pub l: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeclNewEventDefnData {
    pub ident: String,
    pub params: Vec<TypedIdentifier>,
    pub l: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeclEventClauseData {
    pub ident: String,
    pub stmts: Vec<AstNode>,
    pub l: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeclNewMapDefnData {
    pub ty: AstNode,
    pub ident: String,
    pub params: Vec<TypedIdentifier>,
    pub stmts: Vec<AstNode>,
    pub l: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeclMapClauseData {
    pub ident: String,
    pub mapfields: Vec<AstNode>,
    pub opt_expr: Option<AstNode>,
    pub stmts: Vec<AstNode>,
    pub l: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeclConfigData {
    pub ty: AstNode,
    pub ident: String,
    pub expr: AstNode,
    pub l: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StmtVarDeclsNoInitData {
    pub ty: AstNode,
    pub idents: Vec<String>,
    pub l: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StmtVarDeclData {
    pub ty: AstNode,
    pub ident: String,
    pub expr: AstNode,
    pub l: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StmtConstDeclData {
    pub ty: AstNode,
    pub ident: String,
    pub expr: AstNode,
    pub l: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StmtAssignData {
    pub l_expr: AstNode,
    pub expr: AstNode,
    pub l: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StmtFunReturnData {
    pub expr: AstNode,
    pub l: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StmtProcReturnData {
    pub l: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StmtAssertData {
    pub expr: AstNode,
    pub l: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StmtUnpredData {
    pub l: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StmtConstrainedUnpredData {
    pub l: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StmtImpDefData {
    pub ident: String,
    pub l: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StmtUndefinedData {
    pub l: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StmtExceptionTakenData {
    pub l: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StmtDepUnpredData {
    pub l: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StmtDepImpDefData {
    pub string_lit: String,
    pub l: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StmtDepUndefinedData {
    pub l: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StmtSeeData {
    pub expr: AstNode,
    pub l: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StmtThrowData {
    pub ident: String,
    pub l: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StmtDecodeExecuteData {
    pub ident: String,
    pub expr: AstNode,
    pub l: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StmtTCallData {
    pub ident: String,
    pub exprs1: Vec<AstNode>,
    pub exprs2: Vec<AstNode>,
    pub l: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StmtIfData {
    pub expr: AstNode,
    pub stmts1: Vec<AstNode>,
    pub s_elsifs: Vec<AstNode>,
    pub stmts2: Vec<AstNode>,
    pub l: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StmtCaseData {
    pub expr: AstNode,
    pub alts: Vec<AstNode>,
    pub opt_stmts: Option<Vec<AstNode>>,
    pub l: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StmtForData {
    pub ident: String,
    pub expr1: AstNode,
    pub direction: AstNode,
    pub expr2: AstNode,
    pub stmts: Vec<AstNode>,
    pub l: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StmtWhileData {
    pub expr: AstNode,
    pub stmts: Vec<AstNode>,
    pub l: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StmtRepeatData {
    pub stmts: Vec<AstNode>,
    pub expr: AstNode,
    pub l: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StmtTryData {
    pub stmts: Vec<AstNode>,
    pub ident: String,
    pub catchers: Vec<AstNode>,
    pub opt_stmts: Option<Vec<AstNode>>,
    pub l: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ExprIfData {
    pub if_cond_expr: AstNode,
    pub if_body_expr: AstNode,
    pub e_elsifs: Vec<AstNode>,
    pub else_body_expr: AstNode,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ExprBinopData {
    pub l_expr: AstNode,
    pub binop: AstNode,
    pub r_expr: AstNode,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ExprUnopData {
    pub unop: AstNode,
    pub expr: AstNode,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ExprFieldData {
    pub expr: AstNode,
    pub ident: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ExprFieldsData {
    pub expr: AstNode,
    pub idents: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ExprSlicesData {
    pub expr: AstNode,
    pub slices: Vec<AstNode>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ExprInData {
    pub expr: AstNode,
    pub pattern: AstNode,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ExprVarData {
    pub ident: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ExprParensData {
    pub expr: AstNode,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ExprTupleData {
    pub exprs: Vec<AstNode>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ExprUnknownData {
    pub ty: AstNode,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ExprImpDefData {
    pub ty: AstNode,
    pub opt_string_lit: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ExprTApplyData {
    pub ident: String,
    pub exprs1: Vec<AstNode>,
    pub exprs2: Vec<AstNode>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ExprArrayData {
    pub expr1: AstNode,
    pub expr2: AstNode,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ExprLitIntData {
    pub int_lit: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ExprLitHexData {
    pub hex_lit: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ExprLitRealData {
    pub real_lit: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ExprLitBitsData {
    pub bits_lit: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ExprLitMaskData {
    pub mask_lit: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ExprLitStringData {
    pub string_lit: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LExprVarData {
    pub ident: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LExprFieldData {
    pub l_expr: AstNode,
    pub ident: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LExprFieldsData {
    pub l_expr: AstNode,
    pub idents: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LExprSlicesData {
    pub l_expr: AstNode,
    pub slices: Vec<AstNode>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LExprBitTupleData {
    pub l_exprs: Vec<AstNode>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LExprTupleData {
    pub l_exprs: Vec<AstNode>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LExprArrayData {
    pub l_expr: AstNode,
    pub expr: AstNode,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LExprWriteData {
    pub ident: String,
    pub exprs1: Vec<AstNode>,
    pub exprs2: Vec<AstNode>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LExprReadWriteData {
    pub ident1: String,
    pub ident2: String,
    pub exprs1: Vec<AstNode>,
    pub exprs2: Vec<AstNode>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TypeConstructorData {
    pub ident: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TypeBitsData {
    pub expr: AstNode,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TypeAppData {
    pub ident: String,
    pub exprs: Vec<AstNode>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TypeOfExprData {
    pub expr: AstNode,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TypeRegisterData {
    pub int_lit: String,
    pub fields: Vec<RegisterField>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TypeArrayData {
    pub ix_type: AstNode,
    pub ty: AstNode,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TypeTupleData {
    pub tys: Vec<AstNode>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct IndexEnumData {
    pub ident: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct IndexRangeData {
    pub expr1: AstNode,
    pub expr2: AstNode,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FormalInData {
    pub ty: AstNode,
    pub ident: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FormalInOutData {
    pub ty: AstNode,
    pub ident: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EncodingBlockData {
    pub ident1: String,
    pub ident2: String,
    pub instr_fields: Vec<AstNode>,
    pub opcode_value: AstNode,
    pub expr: AstNode,
    pub unpredictables: Vec<Unpredictable>,
    pub stmts: Vec<AstNode>,
    pub l: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct IFieldData {
    pub ident: String,
    pub int1: i32,
    pub int2: i32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct OpcodeBitsData {
    pub bits_lit: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct OpcodeMaskData {
    pub bits_lit: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SElsifCondData {
    pub expr: AstNode,
    pub stmts: Vec<AstNode>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EElsifCondData {
    pub expr1: AstNode,
    pub expr2: AstNode,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AltData {
    pub patterns: Vec<AstNode>,
    pub opt_expr: Option<AstNode>,
    pub stmts: Vec<AstNode>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PatLitIntData {
    pub int_lit: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PatLitHexData {
    pub hex_lit: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PatLitBitsData {
    pub bits_lit: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PatLitMaskData {
    pub mask_lit: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PatConstData {
    pub ident: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PatTupleData {
    pub patterns: Vec<AstNode>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PatSetData {
    pub patterns: Vec<AstNode>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PatRangeData {
    pub expr1: Vec<AstNode>,
    pub expr2: Vec<AstNode>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PatSingleData {
    pub expr: AstNode,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CatcherGuardedData {
    pub expr: AstNode,
    pub stmts: Vec<AstNode>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SliceSingleData {
    pub expr: AstNode,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SliceHiLoData {
    pub hi_expr: AstNode,
    pub lo_expr: AstNode,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SliceLoWdData {
    pub lo_expr: AstNode,
    pub wd_expr: AstNode,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DecoderCaseData {
    pub decode_slices: Vec<AstNode>,
    pub decode_alts: Vec<AstNode>,
    pub l: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DecoderSliceData {
    pub int1: i32,
    pub int2: i32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DecoderSliceFieldNameData {
    pub ident: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DecoderSliceConcatData {
    pub idents: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DecoderAltData {
    pub decode_patterns: Vec<AstNode>,
    pub decode_body: AstNode,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DecoderPatternBitsData {
    pub bits_lit: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DecoderPatternMaskData {
    pub mask_lit: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DecoderPatternWildcardData {
    pub ident: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DecoderPatternNotData {
    pub decode_pattern: AstNode,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DecoderBodyUNPREDData {
    pub l: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DecoderBodyUNALLOCData {
    pub l: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DecoderBodyNOPData {
    pub l: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DecoderBodyEncodingData {
    pub ident: String,
    pub l: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DecoderBodyDecoderData {
    pub instr_fields: Vec<AstNode>,
    pub decode_case: AstNode,
    pub l: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TypedIdentifier {
    pub ty: AstNode,
    pub ident: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Unpredictable {
    pub int: i32,
    pub bits_list: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RegisterField {
    pub slices: Vec<AstNode>,
    pub ident: String,
}

#[derive(Debug, Display, Hash, Eq, PartialEq, Copy, Clone, Ord, PartialOrd, EnumString)]
pub enum InstructionSet {
    #[strum(serialize = "A64")]
    A64,
    #[strum(serialize = "A32")]
    A32,
    #[strum(serialize = "T32")]
    T32,
    #[strum(serialize = "T16")]
    T16,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Ord, PartialOrd)]
pub struct OperandMetadata {
    pub name: String,
    pub type_name: String,
}

#[macro_export]
macro_rules! unwrap_node_data {
    ($node:expr, $variant:path) => {
        match $node.node_data.as_deref().unwrap() {
            $variant(data) => data,
            _ => panic!(
                "Mismatched node type: expected {}, got type={:?} subtype={:?}",
                stringify!($variant),
                $node.node_type,
                $node.node_subtype,
            ),
        }
    };
}

pub struct AstAnalyzer {
    decoder_defs: Vec<AstNode>,
    instruction_defs: Vec<AstNode>,
    functions: Vec<AstNode>,
    procedures: Vec<AstNode>,
    opcodes_and_operands: BTreeMap<InstructionSet, BTreeMap<String, Vec<OperandMetadata>>>,
    enums: BTreeMap<String, Vec<String>>,
    enum_types: BTreeSet<String>,
    enum_variants: BTreeMap<String, String>,
    constants: BTreeMap<String, (AstNode, AstNode)>,
    records: BTreeMap<String, Vec<TypedIdentifier>>,
}

impl AstAnalyzer {
    pub fn new(file_path: &str, supported_opcodes: BTreeSet<String>) -> Result<Self, CodegenError> {
        let tree = Self::deserialize_json_to_ast(file_path)?;

        // Self::visualize_ast(&tree, 0);

        let mut decoder_defs = Vec::new();
        let mut instruction_defs = Vec::new();
        let mut functions = Vec::new();
        let mut procedures = Vec::new();
        let mut opcodes_and_operands: BTreeMap<InstructionSet, BTreeMap<String, Vec<OperandMetadata>>> = BTreeMap::new();
        let mut enums = BTreeMap::new();
        let mut enum_types = BTreeSet::new();
        let mut enum_variants = BTreeMap::new();
        let mut constants = BTreeMap::new();
        let mut records = BTreeMap::new();

        if let Some(boxed_data) = tree.node_data {
            if let NodeData::Decls(decls) = *boxed_data {
                let mut unique_instruction_defn_names = BTreeSet::new();
                let mut unique_function_names = BTreeSet::new();
                let mut unique_procedure_names = BTreeSet::new();

                for decl in decls.decls {
                    match decl.node_subtype {
                        NodeSubtype::DecoderDefn => decoder_defs.push(decl),
                        NodeSubtype::InstructionDefn => {
                            let instruction_data = unwrap_node_data!(decl, NodeData::DeclInstructionDefn);
                            let instruction_name = &instruction_data.ident;
                            if !unique_instruction_defn_names.contains(instruction_name) {
                                unique_instruction_defn_names.insert(instruction_name.to_string());
                                let mut arches = BTreeSet::new();
                                let mut instruction_operands = BTreeSet::new();
                                let supported = supported_opcodes.contains(instruction_name);
                                for encoding in instruction_data.encodings.iter() {
                                    let encoding_data = unwrap_node_data!(encoding, NodeData::EncodingBlock);
                                    let instruction_set = &encoding_data.ident2;
                                    let arch = instruction_set
                                        .parse::<InstructionSet>()
                                        .map_err(|_| CodegenError::UnknownInstructionSet(instruction_set.to_string()))?;
                                    let current_operands = Self::extract_operands_from_decode_block(&encoding_data.stmts)?;
                                    if instruction_operands.is_empty() {
                                        instruction_operands.extend(current_operands);
                                    } else {
                                        instruction_operands.retain(|operand| current_operands.contains(operand));
                                    }
                                    arches.insert(arch);
                                }
                                if let Some(postdecode_stmts) = &instruction_data.opt_stmts {
                                    instruction_operands.extend(Self::extract_operands_from_decode_block(&postdecode_stmts)?);
                                }
                                if !supported {
                                    // only collect destination registers as operands for unsupported instructions
                                    instruction_operands.retain(|metadata| ["d", "n", "dn", "s", "t", "t2"].contains(&metadata.name.as_str()));
                                }
                                for arch in arches {
                                    opcodes_and_operands
                                        .entry(arch)
                                        .or_insert_with(BTreeMap::new)
                                        .insert(instruction_name.to_string(), instruction_operands.clone().into_iter().collect());
                                }
                                instruction_defs.push(decl);
                            }
                        }
                        NodeSubtype::BuiltinFunction => {
                            let data = unwrap_node_data!(decl, NodeData::DeclBuiltinFunction);
                            let name = &data.ident;
                            if !unique_function_names.contains(name) {
                                unique_function_names.insert(name.to_string());
                                functions.push(decl);
                            }
                        }
                        NodeSubtype::FunDefn => {
                            let data = unwrap_node_data!(decl, NodeData::DeclFunDefn);
                            let name = &data.ident;
                            if !unique_function_names.contains(name) {
                                unique_function_names.insert(name.to_string());
                                functions.push(decl);
                            }
                        }
                        NodeSubtype::ProcDefn => {
                            let data = unwrap_node_data!(decl, NodeData::DeclProcDefn);
                            let name = &data.ident;
                            if !unique_procedure_names.contains(name) {
                                unique_procedure_names.insert(name.to_string());
                                procedures.push(decl);
                            }
                        }
                        NodeSubtype::ArrayGetterDefn => {
                            let data = unwrap_node_data!(decl, NodeData::DeclArrayGetterDefn);
                            let name = &data.ident;
                            if !unique_function_names.contains(name) {
                                unique_function_names.insert(name.to_string());
                                functions.push(decl);
                            }
                        }
                        NodeSubtype::ArraySetterDefn => {
                            let data = unwrap_node_data!(decl, NodeData::DeclArraySetterDefn);
                            let name = &data.ident1;
                            if !unique_function_names.contains(name) {
                                unique_function_names.insert(name.to_string());
                                procedures.push(decl);
                            }
                        }
                        NodeSubtype::VarGetterDefn => {
                            let data = unwrap_node_data!(decl, NodeData::DeclVarGetterDefn);
                            let name = &data.ident;
                            if !unique_function_names.contains(name) {
                                unique_function_names.insert(name.to_string());
                                functions.push(decl);
                            }
                        }
                        NodeSubtype::Enum => {
                            let data = unwrap_node_data!(decl, NodeData::DeclEnum);
                            let name = &data.ident;
                            if !enums.contains_key(name) {
                                enums.insert(name.to_string(), data.idents.clone());
                                let enum_type = &data.ident;
                                enum_types.insert(enum_type.to_string());
                                for enum_variant in data.idents.iter() {
                                    enum_variants.insert(enum_variant.to_string(), enum_type.to_string());
                                }
                            }
                        }
                        NodeSubtype::Const => {
                            let data = unwrap_node_data!(decl, NodeData::DeclConst);

                            if !constants.contains_key(&data.ident) {
                                constants.insert(data.ident.clone(), (data.ty.clone(), data.expr.clone()));
                            }
                        }
                        NodeSubtype::Record => {
                            let data = unwrap_node_data!(decl, NodeData::DeclRecord);

                            if !records.contains_key(&data.ident) {
                                records.insert(data.ident.clone(), data.fields.clone());
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        if decoder_defs.is_empty() || instruction_defs.is_empty() || functions.is_empty() || procedures.is_empty() {
            return Err(CodegenError::MissingAstNodes);
        }

        Ok(Self {
            decoder_defs,
            instruction_defs,
            functions,
            procedures,
            opcodes_and_operands,
            enums,
            enum_types,
            enum_variants,
            constants,
            records,
        })
    }

    fn deserialize_json_to_ast(file_path: &str) -> Result<AstNode, CodegenError> {
        let ast_json = fs::read_to_string(file_path).map_err(|e| CodegenError::IoError(format!("Failed to read JSON file: {}", e)))?;
        let ast_tree: AstNode = serde_json::from_str(&ast_json).map_err(CodegenError::JsonParseError)?;
        Ok(ast_tree)
    }

    fn extract_operands_from_decode_block(stmts: &Vec<AstNode>) -> Result<BTreeSet<OperandMetadata>, CodegenError> {
        let mut instruction_operands = BTreeSet::new();

        for stmt_node in stmts.iter() {
            match stmt_node.node_subtype {
                NodeSubtype::VarDeclsNoInit => {
                    let stmt_data = unwrap_node_data!(stmt_node, NodeData::StmtVarDeclsNoInit);
                    let ty_node = &stmt_data.ty;
                    let ty_name = match ty_node.node_subtype {
                        NodeSubtype::Constructor => {
                            let data = unwrap_node_data!(ty_node, NodeData::TypeConstructor);
                            data.ident.clone()
                        }
                        NodeSubtype::Bits => "bits".to_string(),
                        NodeSubtype::App | NodeSubtype::OfExpr | NodeSubtype::Register | NodeSubtype::Array | NodeSubtype::Tuple => {
                            return Err(CodegenError::NotImplemented(file!(), line!()))
                        }
                        _ => return Err(CodegenError::InvalidNodeType(ty_node.node_type, ty_node.node_subtype)),
                    };
                    for variable_name in stmt_data.idents.iter() {
                        instruction_operands.insert(OperandMetadata {
                            name: variable_name.to_string(),
                            type_name: ty_name.clone(),
                        });
                    }
                }
                NodeSubtype::VarDecl => {
                    let stmt_data = unwrap_node_data!(stmt_node, NodeData::StmtVarDecl);
                    let ty_node = &stmt_data.ty;
                    let ty_name = match ty_node.node_subtype {
                        NodeSubtype::Constructor => {
                            let data = unwrap_node_data!(ty_node, NodeData::TypeConstructor);
                            data.ident.clone()
                        }
                        NodeSubtype::Bits => "bits".to_string(),
                        NodeSubtype::App | NodeSubtype::OfExpr | NodeSubtype::Register | NodeSubtype::Array | NodeSubtype::Tuple => {
                            return Err(CodegenError::NotImplemented(file!(), line!()))
                        }
                        _ => return Err(CodegenError::InvalidNodeType(ty_node.node_type, ty_node.node_subtype)),
                    };
                    let variable_name = &stmt_data.ident;
                    instruction_operands.insert(OperandMetadata {
                        name: variable_name.to_string(),
                        type_name: ty_name.clone(),
                    });
                }
                _ => {}
            }
        }

        Ok(instruction_operands)
    }

    pub fn get_opcode_and_operands(&self, arch: InstructionSet) -> Result<&BTreeMap<String, Vec<OperandMetadata>>, CodegenError> {
        self.opcodes_and_operands
            .get(&arch)
            .ok_or_else(|| CodegenError::MissingOperandAnalysis(arch))
    }

    pub fn get_enums(&self) -> Result<&BTreeMap<String, Vec<String>>, CodegenError> {
        Ok(&self.enums)
    }

    pub fn get_enum_types_and_variants(&self) -> Result<(&BTreeSet<String>, &BTreeMap<String, String>), CodegenError> {
        Ok((&self.enum_types, &self.enum_variants))
    }

    pub fn get_constants(&self) -> Result<&BTreeMap<String, (AstNode, AstNode)>, CodegenError> {
        Ok(&self.constants)
    }

    pub fn get_records(&self) -> Result<&BTreeMap<String, Vec<TypedIdentifier>>, CodegenError> {
        Ok(&self.records)
    }

    pub fn get_constant_names(&self) -> Result<BTreeSet<String>, CodegenError> {
        Ok(self.constants.keys().cloned().collect())
    }

    pub fn get_decoder_case(&self, arch: InstructionSet) -> Result<&AstNode, CodegenError> {
        self.decoder_defs
            .iter()
            .find_map(|decoder| {
                if let Some(NodeData::DeclDecoderDefn(data)) = decoder.node_data.as_deref() {
                    if data.ident == arch.to_string() {
                        return Some(&data.decode_case);
                    }
                }
                None
            })
            .ok_or_else(|| CodegenError::MissingDecoderDef(arch))
    }

    // pub fn get_instruction_def_by_encoding_name(&self, encoding_name: &str) -> Result<&AstNode, CodegenError> {
    //     self.instruction_defs
    //         .iter()
    //         .find(|instruction_def| {
    //             unwrap_node_data!(instruction_def, NodeData::DeclInstructionDefn)?
    //                 .ok()
    //                 .map_or(false, |instruction_data| {
    //                     instruction_data.encodings.iter().any(|encoding_block| {
    //                         unwrap_node_data!(encoding_block, NodeData::EncodingBlock)?
    //                             .ok()
    //                             .map_or(false, |encoding_data| encoding_data.ident1 == encoding_name)
    //                     })
    //                 })
    //         })
    //         .ok_or_else(|| CodegenError::EncodingNotFound(encoding_name.to_string()))
    // }
    pub fn get_instruction_def_by_encoding_name(&self, encoding_name: &str) -> Result<&AstNode, CodegenError> {
        self.instruction_defs
            .iter()
            .find(|instruction_def| {
                let instruction_data = unwrap_node_data!(instruction_def, NodeData::DeclInstructionDefn);
                instruction_data.encodings.iter().any(|encoding_block| {
                    let encoding_data = unwrap_node_data!(encoding_block, NodeData::EncodingBlock);
                    encoding_data.ident1 == encoding_name
                })
            })
            .ok_or_else(|| CodegenError::EncodingNotFound(encoding_name.to_string()))
    }

    // pub fn get_instruction_def_by_instruction_name(&self, instruction_name: &str) -> Result<&AstNode, CodegenError> {
    //     self.instruction_defs
    //         .iter()
    //         .find(|instruction_def| {
    //             unwrap_node_data!(instruction_def, NodeData::DeclInstructionDefn)
    //                 .ok()
    //                 .map_or(false, |instruction_data| instruction_data.ident == instruction_name)
    //         })
    //         .ok_or_else(|| CodegenError::InstructionNotFound(instruction_name.to_string()))
    // }
    pub fn get_instruction_def_by_instruction_name(&self, instruction_name: &str) -> Result<&AstNode, CodegenError> {
        self.instruction_defs
            .iter()
            .find(|instruction_def| {
                let instruction_data = unwrap_node_data!(instruction_def, NodeData::DeclInstructionDefn);
                instruction_data.ident == instruction_name
            })
            .ok_or_else(|| CodegenError::InstructionNotFound(instruction_name.to_string()))
    }

    pub fn get_function(&self, ident: &str) -> Result<&AstNode, CodegenError> {
        for function in &self.functions {
            match function.node_subtype {
                NodeSubtype::BuiltinFunction => {
                    let data = unwrap_node_data!(function, NodeData::DeclBuiltinFunction);
                    if data.ident == ident {
                        return Ok(function);
                    }
                }
                NodeSubtype::FunDefn => {
                    let data = unwrap_node_data!(function, NodeData::DeclFunDefn);
                    if data.ident == ident {
                        return Ok(function);
                    }
                }
                NodeSubtype::ArrayGetterDefn => {
                    let data = unwrap_node_data!(function, NodeData::DeclArrayGetterDefn);
                    if data.ident == ident {
                        return Ok(function);
                    }
                }
                NodeSubtype::VarGetterDefn => {
                    let data = unwrap_node_data!(function, NodeData::DeclVarGetterDefn);
                    if data.ident == ident {
                        return Ok(function);
                    }
                }
                _ => panic!("Unexpected node type: type={:?} subtype={:?}", function.node_type, function.node_subtype),
            }
        }

        Err(CodegenError::FunctionNotFound(ident.to_string()))
    }

    pub fn get_procedure(&self, ident: &str) -> Result<&AstNode, CodegenError> {
        for procedure in &self.procedures {
            match procedure.node_subtype {
                NodeSubtype::ProcDefn => {
                    let data = unwrap_node_data!(procedure, NodeData::DeclProcDefn);
                    if data.ident == ident {
                        return Ok(procedure);
                    }
                }
                NodeSubtype::ArraySetterDefn => {
                    let data = unwrap_node_data!(procedure, NodeData::DeclArraySetterDefn);
                    if data.ident1 == ident {
                        return Ok(procedure);
                    }
                }
                _ => panic!(
                    "Unexpected node type: type={:?} subtype={:?}",
                    procedure.node_type, procedure.node_subtype
                ),
            }
        }

        Err(CodegenError::ProcedureNotFound(ident.to_string()))
    }

    pub fn normalize_ident(ident: &str) -> String {
        let forbidden_keywords = ["match"].iter().map(|&s| s.to_string()).collect::<BTreeSet<String>>();

        if forbidden_keywords.contains(ident) {
            format!("{}_", ident)
        } else {
            ident.replace('.', "_")
        }
    }

    pub fn visualize_ast(tree: &AstNode, indent: usize) {
        let node_label = format!("{:?}_{:?}", tree.node_type, tree.node_subtype);

        if let Some(data) = &tree.node_data.as_deref() {
            match data {
                NodeData::Decls(ref data) => {
                    println!("{:indent$}{}", "", node_label, indent = indent);
                    for decl in &data.decls {
                        Self::visualize_ast(&decl, indent + 4);
                    }
                }
                NodeData::DeclBuiltinType(ref data) => {
                    println!("{:indent$}{:<20} {}", "", node_label, &data.ident, indent = indent);
                }
                NodeData::DeclForward(ref data) => {
                    println!("{:indent$}{:<20} {}", "", node_label, &data.ident, indent = indent);
                }
                NodeData::DeclRecord(ref data) => {
                    println!("{:indent$}{:<20} {}", "", node_label, &data.ident, indent = indent);
                }
                NodeData::DeclTypedef(ref data) => {
                    println!("{:indent$}{:<20} {}", "", node_label, &data.ident, indent = indent);
                }
                NodeData::DeclEnum(ref data) => {
                    println!("{:indent$}{:<20} {}", "", node_label, &data.ident, indent = indent);
                }
                NodeData::DeclVar(ref data) => {
                    println!("{:indent$}{:<20} {}", "", node_label, &data.ident, indent = indent);
                }
                NodeData::DeclConst(ref data) => {
                    println!("{:indent$}{:<20} {}", "", node_label, &data.ident, indent = indent);
                }
                NodeData::DeclBuiltinFunction(ref data) => {
                    println!("{:indent$}{:<20} {}", "", node_label, &data.ident, indent = indent);
                }
                NodeData::DeclFunType(ref data) => {
                    println!("{:indent$}{:<20} {}", "", node_label, &data.ident, indent = indent);
                }
                NodeData::DeclFunDefn(ref data) => {
                    println!("{:indent$}{:<20} {}", "", node_label, &data.ident, indent = indent);
                }
                NodeData::DeclProcType(ref data) => {
                    println!("{:indent$}{:<20} {}", "", node_label, &data.ident, indent = indent);
                }
                NodeData::DeclProcDefn(ref data) => {
                    println!("{:indent$}{:<20} {}", "", node_label, &data.ident, indent = indent);
                }
                NodeData::DeclVarGetterType(ref data) => {
                    println!("{:indent$}{:<20} {}", "", node_label, &data.ident, indent = indent);
                }
                NodeData::DeclVarGetterDefn(ref data) => {
                    println!("{:indent$}{:<20} {}", "", node_label, &data.ident, indent = indent);
                }
                NodeData::DeclArrayGetterType(ref data) => {
                    println!("{:indent$}{:<20} {}", "", node_label, &data.ident, indent = indent);
                }
                NodeData::DeclArrayGetterDefn(ref data) => {
                    println!("{:indent$}{:<20} {}", "", node_label, &data.ident, indent = indent);
                }
                NodeData::DeclVarSetterType(ref data) => {
                    println!("{:indent$}{:<20} {}", "", node_label, &data.ident1, indent = indent);
                }
                NodeData::DeclVarSetterDefn(ref data) => {
                    println!("{:indent$}{:<20} {}", "", node_label, &data.ident1, indent = indent);
                }
                NodeData::DeclArraySetterType(ref data) => {
                    println!("{:indent$}{:<20} {}", "", node_label, &data.ident1, indent = indent);
                }
                NodeData::DeclArraySetterDefn(ref data) => {
                    println!("{:indent$}{:<20} {}", "", node_label, &data.ident1, indent = indent);
                }
                NodeData::DeclInstructionDefn(ref data) => {
                    println!("{:indent$}{:<20} {}", "", node_label, &data.ident, indent = indent);
                }
                NodeData::DeclDecoderDefn(ref data) => {
                    println!("{:indent$}{:<20} {}", "", node_label, &data.ident, indent = indent);
                }
                NodeData::DeclOperator1(ref data) => {
                    println!("{:indent$}{:<20} {}", "", node_label, &data.idents.join(", "), indent = indent);
                }
                NodeData::DeclOperator2(ref data) => {
                    println!("{:indent$}{:<20} {}", "", node_label, &data.idents.join(", "), indent = indent);
                }
                NodeData::DeclNewEventDefn(ref data) => {
                    println!("{:indent$}{:<20} {}", "", node_label, &data.ident, indent = indent);
                }
                NodeData::DeclEventClause(ref data) => {
                    println!("{:indent$}{:<20} {}", "", node_label, &data.ident, indent = indent);
                }
                NodeData::DeclNewMapDefn(ref data) => {
                    println!("{:indent$}{:<20} {}", "", node_label, &data.ident, indent = indent);
                }
                NodeData::DeclMapClause(ref data) => {
                    println!("{:indent$}{:<20} {}", "", node_label, &data.ident, indent = indent);
                }
                NodeData::DeclConfig(ref data) => {
                    println!("{:indent$}{:<20} {}", "", node_label, &data.ident, indent = indent);
                }
                _ => (),
            }
        }
    }
}
