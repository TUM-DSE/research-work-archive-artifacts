//===-- IncludedFileInfo.cpp -----------------------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//
//
// This file was taken from LLVM-MCTOLL project and modified for sgtranslator's
// needs. It is used to parse headers of dynamic libraries.
//
//===----------------------------------------------------------------------===//

#include "HeaderParser.h"
#include "clang/AST/ASTConsumer.h"
#include "clang/AST/RecursiveASTVisitor.h"
#include "clang/Frontend/CompilerInstance.h"
#include "clang/Tooling/CompilationDatabase.h"
#include "clang/Tooling/Tooling.h"
#include "llvm/IR/LLVMContext.h"
#include "llvm/Support/Debug.h"
#include "llvm/Support/Path.h"
#include "llvm/Support/raw_ostream.h"

#include <clang-c/Index.h>

#include <memory>
#include <string>
#include "llvm/IR/Constants.h"
#include "llvm/IR/IRBuilder.h"
#include "llvm/IR/Instructions.h"

#define DEBUG_TYPE "prototypes"

// NOTE: Not using namespace clang to highlight the fact that certain types such
// as Type being used in this file are from clang namespace and not from llvm
// namespace.

using namespace llvm;

std::map<std::string, IncludedFileInfo::FunctionRetAndArgs>
    IncludedFileInfo::ExternalFunctions;

std::set<std::string> IncludedFileInfo::ExternalVariables;

// FuncDeclVisitor

class FuncDeclVisitor : public clang::RecursiveASTVisitor<FuncDeclVisitor> {
  clang::ASTContext &Context;

public:
  FuncDeclVisitor(clang::ASTContext &Context) : Context(Context) {
#ifndef NDEBUG
    llvm::setCurrentDebugType(DEBUG_TYPE);
#endif
  }

  bool VisitFunctionDecl(clang::FunctionDecl *FuncDecl) {
    IncludedFileInfo::FunctionRetAndArgs Entry;
    clang::QualType RetTy = FuncDecl->getDeclaredReturnType();
    Entry.ReturnType =
        getUnqualifiedTypeString(RetTy, FuncDecl->getASTContext());
    for (auto *Param : FuncDecl->parameters()) {
      clang::QualType ParamTy = Param->getOriginalType();
      std::string ParamTyStr =
          getUnqualifiedTypeString(ParamTy, FuncDecl->getASTContext());
      Entry.Arguments.push_back(ParamTyStr);
    }
    Entry.IsVariadic = FuncDecl->isVariadic();
    // TODO: Raising binaries compiled from C++ sources is not yet supported. C
    // does not support function overloading. So, for now, trivially check for
    // function name to detect duplicate function prototype specification. Need
    // to update this check to include argument types when support to raise C++
    // binary is added.
    if (IncludedFileInfo::ExternalFunctions.find(
            FuncDecl->getQualifiedNameAsString()) !=
        IncludedFileInfo::ExternalFunctions.end()) {
      LLVM_DEBUG(dbgs() << FuncDecl->getQualifiedNameAsString()
                        << " : Ignoring duplicate entry at "
                        << FuncDecl->getLocation().printToString(
                               Context.getSourceManager())
                        << "\n");
    } else {
      IncludedFileInfo::ExternalFunctions.insert(
          std::pair<std::string, IncludedFileInfo::FunctionRetAndArgs>(
              FuncDecl->getQualifiedNameAsString(), Entry));
      LLVM_DEBUG(dbgs() << FuncDecl->getQualifiedNameAsString()
                        << " : Added entry found at "
                        << FuncDecl->getLocation().printToString(
                               Context.getSourceManager())
                        << "\n");
    }
    return true;
  }

private:
  std::string getUnqualifiedTypeString(clang::QualType &QTy,
                                       clang::ASTContext &ASTCtx) {
    std::string PointerStr;
    std::string UnQTyStr;
    clang::SplitQualType SplitCurQTy = QTy.split();
    // Get unqualified, de-sugared type
    const clang::Type *CurUnQTy = SplitCurQTy.Ty->getUnqualifiedDesugaredType();
    while (true) {
      if (CurUnQTy->isPointerType()) {
        PointerStr.append("*");
        // Get unqualified, de-sugared pointee type
        CurUnQTy = CurUnQTy->getPointeeType()
                       .split()
                       .Ty->getUnqualifiedDesugaredType();
      } else
        break;
    }

    // Construct type string corresponding to the buitl-in type
    if (CurUnQTy->isBuiltinType()) {
      const clang::BuiltinType *BltInTy = CurUnQTy->getAs<clang::BuiltinType>();
      if (BltInTy->isInteger()) {
        auto FieldInfo = ASTCtx.getTypeInfo(CurUnQTy);
        uint64_t TypeWidth = FieldInfo.Width;
        assert((TypeWidth == 64 || TypeWidth == 32 || TypeWidth == 16 ||
                TypeWidth == 8) &&
               "Unexpected builtin type width encountered");
        UnQTyStr.append("i" + std::to_string(TypeWidth));
      } else {
        switch (BltInTy->getKind()) {
        case clang::BuiltinType::Kind::Float:
          UnQTyStr.append("float");
          break;
        case clang::BuiltinType::Kind::Double:
          UnQTyStr.append("double");
          break;
        case clang::BuiltinType::Kind::LongDouble:
          UnQTyStr.append("ldouble");
          break;
        case clang::BuiltinType::Kind::Void:
          UnQTyStr.append("void");
          break;
        default:
          assert(false && "Unhandled builtin type found in include file");
        }
      }
      // Append any pointer qualifiers
      UnQTyStr.append(PointerStr);
    } else {
      // If it is not a builtin type consider it to be an int64 type
      UnQTyStr.append("i64").append(PointerStr);
    }
    return UnQTyStr;
  }
};

class FuncDeclFinder : public clang::ASTConsumer {
  FuncDeclVisitor Visitor;

public:
  FuncDeclFinder(clang::ASTContext &Context) : Visitor(Context) {}

  void HandleTranslationUnit(clang::ASTContext &Context) final {
    auto Decls = Context.getTranslationUnitDecl()->decls();
    for (auto &Decl : Decls) {
      if (Decl->isFunctionOrFunctionTemplate() && Decl->isFirstDecl()) {
        clang::FunctionDecl *FuncDecl = Decl->getAsFunction();
        LLVM_DEBUG(dbgs() << FuncDecl->getQualifiedNameAsString() << " : Visit "
                          << FuncDecl->getLocation().printToString(
                                 Context.getSourceManager())
                          << "\n");
        Visitor.TraverseFunctionDecl(FuncDecl);
      } else if (Decl->getKind() == clang::Decl::Kind::Var) {
        auto *VarDecl = dyn_cast<clang::VarDecl>(Decl);
        IncludedFileInfo::ExternalVariables.insert(VarDecl->getQualifiedNameAsString());
      }
    }
  }
};

class FuncDeclFindingAction : public clang::ASTFrontendAction {
public:
  std::unique_ptr<clang::ASTConsumer>
  CreateASTConsumer(clang::CompilerInstance &CI,
                    clang::StringRef InFile) final {
    return std::unique_ptr<clang::ASTConsumer>(
        new FuncDeclFinder(CI.getASTContext()));
  }
};

/// Get the data type corresponding to type string. These correspond to type
/// strings generated in IncludedFileInfo.cpp upon parsing user specified
/// include files with external function prototypes.
Type *getDataType(LLVMContext &C, const std::string &TypeStr) {
  Type *RetTy = nullptr;
  if (TypeStr.find_first_of("*") != std::string::npos) {
    RetTy = PointerType::get(C, 0);
  } else if (TypeStr.starts_with("void"))
    RetTy = Type::getVoidTy(C);
  else if (TypeStr.starts_with("i1"))
    RetTy = Type::getInt1Ty(C);
  else if (TypeStr.starts_with("i8"))
    RetTy = Type::getInt8Ty(C);
  else if (TypeStr.starts_with("i16"))
    RetTy = Type::getInt16Ty(C);
  else if (TypeStr.starts_with("i32"))
    RetTy = Type::getInt32Ty(C);
  else if (TypeStr.starts_with("i64"))
    RetTy = Type::getInt64Ty(C);
  else if (TypeStr.starts_with("float"))
    RetTy = Type::getFloatTy(C);
  else if (TypeStr.starts_with("double"))
    RetTy = Type::getDoubleTy(C);
  assert((RetTy != nullptr) && "Invalid data type string!");
  return RetTy;
}

// Construct and return a Function* corresponding to a known external function
Function *IncludedFileInfo::CreateFunction(const std::string &CFuncName,
                                           Module *M) {
  assert(M != nullptr);
  Function *Func = M->getFunction(CFuncName);
  if (Func != nullptr) {
    return Func;
  }

  auto Iter = IncludedFileInfo::ExternalFunctions.find(CFuncName);
  if (Iter == IncludedFileInfo::ExternalFunctions.end()) {
    errs() << "External function " << CFuncName << " not found in header files! Specify path to "
              "the appropriate header file in config.json\n";
    std::exit(1);
  }

  const IncludedFileInfo::FunctionRetAndArgs &RetAndArgs = Iter->second;
  /* We only support void and int for now. */

  LLVMContext &C = M->getContext();
  Type *RetType = getDataType(C, RetAndArgs.ReturnType);
  std::vector<Type *> ArgVec;
  for (StringRef Arg : RetAndArgs.Arguments) {
    Type *ArgType = getDataType(C, Arg.str());
    ArgVec.push_back(ArgType);
  }

  ArrayRef<Type *> Args(ArgVec);
  if (llvm::FunctionType *FuncType =
          FunctionType::get(RetType, Args, RetAndArgs.IsVariadic)) {
    FunctionCallee FunCallee = M->getOrInsertFunction(CFuncName, FuncType);
    assert(isa<Function>(FunCallee.getCallee()) && "Expect Function");
    Func = reinterpret_cast<Function *>(FunCallee.getCallee());
    Func->setCallingConv(CallingConv::C);
    Func->setDSOLocal(true);
    return Func;
  }

  errs() << "Failed to construct external function's type for : "
         << CFuncName.data() << "\n";
  return nullptr;
}

bool IncludedFileInfo::getExternalFunctionPrototype(const std::vector<std::string> &FileNames) {
  // Collect unique parent directories to use as -I include paths.
  // These strings must outlive ArgPtrVec below.
  std::vector<std::string> IncludeArgs;
  {
    std::set<std::string> Seen;
    for (const auto &File : FileNames) {
      llvm::StringRef Dir = llvm::sys::path::parent_path(File);
      if (!Dir.empty() && Seen.insert(Dir.str()).second)
        IncludeArgs.push_back("-I" + Dir.str());
    }
  }

  std::vector<const char *> ArgPtrVec;
  ArgPtrVec.push_back("parse-header-files");
  ArgPtrVec.push_back("--");

  for (const auto &Arg : IncludeArgs)
    ArgPtrVec.push_back(Arg.c_str());

  if (llvm::DebugFlag) {
    ArgPtrVec.push_back("-v");
  }

  auto *ToolArgv = ArgPtrVec.data();
  int ArgSz = ArgPtrVec.size();

  std::string ErrorMessage;
  std::unique_ptr<clang::tooling::CompilationDatabase> Compilations =
      clang::tooling::FixedCompilationDatabase::loadFromCommandLine(
          ArgSz, ToolArgv, ErrorMessage);
  if (!ErrorMessage.empty()) {
    llvm::errs() << ErrorMessage.append("\n");
  }

  clang::tooling::ClangTool Tool(*Compilations, FileNames);
  int Success = Tool.run(
      clang::tooling::newFrontendActionFactory<FuncDeclFindingAction>().get());
  switch (Success) {
  case 0:
    break;
  default:
    dbgs() << "Error\n";
  }

  return true;
}

bool IncludedFileInfo::isExternalVariable(std::string Name) {
  // If there is a suffix like stdout@@GLIBC_2.2.5, remove it to check
  // if the symbol is defined in a user-passed header file
  auto NameEnd = Name.find("@@");
  if (NameEnd != std::string::npos) {
    Name = Name.substr(0, NameEnd);
  }
  // Declare external global variables as external and don't initalize them
  return IncludedFileInfo::ExternalVariables.find(Name) !=
         IncludedFileInfo::ExternalVariables.end();
}

void patchDynamicCalls(llvm::Module *Mod, const std::vector<std::string> &DynamicLibHeaders) {
  IncludedFileInfo::getExternalFunctionPrototype(DynamicLibHeaders);

  LLVMContext &Ctx = Mod->getContext();

  // Build the register struct type once per context using a named type so
  // repeated calls to patchDynamicCalls don't redo the lookup work.
  // { { i1, i1, i1, i1 }, [31 x i64], i64, i64, i64, i64, [32 x i2048], [16 x i256], i256 }
  Type *I1Ty   = Type::getInt1Ty(Ctx);
  Type *I32Ty  = Type::getInt32Ty(Ctx);
  Type *I64Ty  = Type::getInt64Ty(Ctx);
  Type *I2048Ty = Type::getIntNTy(Ctx, 2048);
  Type *I256Ty  = Type::getIntNTy(Ctx, 256);
  StructType *RegStructTy = StructType::getTypeByName(Ctx, "AirliftRegState");
  if (!RegStructTy) {
    StructType *FlagsTy  = StructType::get(Ctx, {I1Ty, I1Ty, I1Ty, I1Ty});
    ArrayType  *GpRegsTy = ArrayType::get(I64Ty, 31);
    ArrayType  *FpRegsTy = ArrayType::get(I2048Ty, 32);
    ArrayType  *VecRegsTy = ArrayType::get(I256Ty, 16);
    RegStructTy = StructType::create(
        Ctx, {FlagsTy, GpRegsTy, I64Ty, I64Ty, I64Ty, I64Ty, FpRegsTy, VecRegsTy, I256Ty},
        "AirliftRegState");
  }

  // Collect (CallInst*, first-arg-of-outer-function) pairs first to avoid
  // iterator invalidation during replacement.
  std::vector<std::pair<CallInst *, Value *>> ToReplace;

  for (Function &F : *Mod) {
    if (F.isDeclaration() || F.getName() == "airlift_init_state" || F.getName() == "main") {
      continue;
    }
    outs() << F.getName() << "\n";
    assert(F.arg_size() >= 1 && "Lifted function must have register struct pointer as first argument");
    Value *RegStructPtr = F.getArg(0);

    for (BasicBlock &BB : F) {
      for (Instruction &I : BB) {
        if (auto *CI = dyn_cast<CallInst>(&I)) {
          Function *Callee = CI->getCalledFunction();
          if (Callee && Callee->getName() == "airlift_dyncall_helper") {
            ToReplace.emplace_back(CI, RegStructPtr);
          }
        }
      }
    }
  }

  for (auto &[CI, RegStructPtr] : ToReplace) {
    IRBuilder<> Builder(CI); // inserts before CI

    // Extract the target function name from the first argument (@dyncall_xxx)
    Value *FirstArg = CI->getArgOperand(0);
    auto *GV = dyn_cast<GlobalVariable>(FirstArg);
    assert(GV != nullptr && GV->hasInitializer());
    auto *CDA = dyn_cast<ConstantDataArray>(GV->getInitializer());
    assert(CDA != nullptr && CDA->isCString());
    StringRef FuncName = CDA->getAsCString();

    Function *DynFun = IncludedFileInfo::CreateFunction(FuncName.str(), Mod);
    assert(DynFun != nullptr);
    FunctionType *FTy = DynFun->getFunctionType();

    if (FTy->isVarArg()) {
      llvm::errs() << "patchDynamicCalls: variadic function '" << FuncName
                   << "' is not supported\n";
      std::exit(1);
    }

    // For each parameter, load gpregs[N] from the register struct and cast.
    std::vector<Value *> Args;
    for (unsigned N = 0; N < FTy->getNumParams(); ++N) {
      // GEP: &RegStruct->gpregs[N]  (indices: 0=deref, 1=gpregs field, N=element)
      Value *Indices[] = {ConstantInt::get(I32Ty, 0), ConstantInt::get(I32Ty, 1),
                          ConstantInt::get(I32Ty, N)};
      Value *GEP = Builder.CreateGEP(RegStructTy, RegStructPtr, Indices, "reg_ptr");
      Value *Loaded = Builder.CreateLoad(I64Ty, GEP, "reg_val");

      Type *ParamTy = FTy->getParamType(N);
      Value *Casted;
      if (ParamTy == I64Ty) {
        Casted = Loaded;
      } else if (ParamTy->isPointerTy()) {
        Casted = Builder.CreateIntToPtr(Loaded, ParamTy, "cast");
      } else if (ParamTy->isIntegerTy()) {
        Casted = Builder.CreateTrunc(Loaded, ParamTy, "cast");
      } else {
        Casted = Loaded; // fallback: pass i64 as-is
      }
      Args.push_back(Casted);
    }

    // Replace the helper call with the real typed call.
    CallInst *NewCall = Builder.CreateCall(FTy, DynFun, Args, "");

    // Store the return value into gpregs[0] (x0).
    Type *RetTy = FTy->getReturnType();
    if (!RetTy->isVoidTy()) {
      Value *RetAsI64;
      if (RetTy == I64Ty) {
        RetAsI64 = NewCall;
      } else if (RetTy->isPointerTy()) {
        RetAsI64 = Builder.CreatePtrToInt(NewCall, I64Ty, "ret_i64");
      } else if (RetTy->isIntegerTy()) {
        RetAsI64 = Builder.CreateZExt(NewCall, I64Ty, "ret_i64");
      } else {
        RetAsI64 = Builder.CreateZExt(NewCall, I64Ty, "ret_i64"); // fallback
      }

      // GEP: &RegStruct->gpregs[0]
      Value *RetIndices[] = {ConstantInt::get(I32Ty, 0), ConstantInt::get(I32Ty, 1),
                             ConstantInt::get(I32Ty, 0)};
      Value *RetSlot =
          Builder.CreateGEP(RegStructTy, RegStructPtr, RetIndices, "ret_slot");
      Builder.CreateStore(RetAsI64, RetSlot);
    }

    CI->eraseFromParent();
  }
}

#undef DEBUG_TYPE