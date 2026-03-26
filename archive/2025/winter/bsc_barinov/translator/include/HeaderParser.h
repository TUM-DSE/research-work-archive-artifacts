//===-- IncludedFileInfo.h -------------------------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//
//
// Taken from MCTOLL project and adopted to sgtranslator's needs
//
//===----------------------------------------------------------------------===//

#ifndef HEADERPARSER_H
#define HEADERPARSER_H

#include "llvm/IR/Function.h"
#include "llvm/IR/Module.h"
#include <map>
#include <set>
#include <string>

class IncludedFileInfo {
  IncludedFileInfo() {};
  ~IncludedFileInfo() {};

public:
  struct FunctionRetAndArgs {
    std::string ReturnType;
    std::vector<std::string> Arguments;
    bool IsVariadic;
  };

  static llvm::Function *CreateFunction(const std::string &CFuncName,
                                        llvm::Module *M);

  static std::map<std::string, IncludedFileInfo::FunctionRetAndArgs>
      ExternalFunctions;

  static std::set<std::string> ExternalVariables;

  static bool getExternalFunctionPrototype(const std::vector<std::string> &FileNames);

  static bool isExternalVariable(std::string Name);
};

/* Goes over all dynamic call stubs in the module and inserts a call to external function instead. */
void patchDynamicCalls(llvm::Module* Mod, const std::vector<std::string>& DynamicLibHeaders);

#endif