#include "lld/Common/Driver.h"
#include "llvm/Analysis/CGSCCPassManager.h"
#include "llvm/Analysis/CallGraph.h"
#include "llvm/Analysis/LoopAnalysisManager.h"
#include "llvm/AsmParser/Parser.h"
#include "llvm/CodeGen/CommandFlags.h"
#include "llvm/IR/Constants.h"
#include "llvm/IR/DataLayout.h"
#include "llvm/IR/DebugInfo.h"
#include "llvm/IR/DerivedTypes.h"
#include "llvm/IR/GlobalValue.h"
#include "llvm/IR/GlobalVariable.h"
#include "llvm/IR/LLVMContext.h"
#include "llvm/IR/LLVMRemarkStreamer.h"
#include "llvm/IR/LegacyPassManager.h"
#include "llvm/IR/Module.h"
#include "llvm/IR/Verifier.h"
#include "llvm/IRReader/IRReader.h"
#include "llvm/MC/TargetRegistry.h"
#include "llvm/Passes/PassBuilder.h"
#include "llvm/Passes/PassPlugin.h"
#include "llvm/Support/ErrorHandling.h"
#include "llvm/Support/InitLLVM.h"
#include "llvm/Support/MemoryBuffer.h"
#include "llvm/Support/SourceMgr.h"
#include "llvm/Support/TargetSelect.h"
#include "llvm/Support/raw_ostream.h"
#include "llvm/Target/TargetMachine.h"
#include "llvm/Target/TargetOptions.h"
#include "llvm/TargetParser/Host.h"
#include "llvm/Transforms/IPO/WholeProgramDevirt.h"
#include "llvm/Transforms/Utils/Cloning.h"

#include <argparse.hpp>
#include <cstdio>
#include <fstream>
#include <iostream>
#include <memory>
#include <nlohmann/json.hpp>

#include <LifterHelper.h>
#include <HeaderParser.h>

using namespace llvm;

LLD_HAS_DRIVER(elf);

static std::vector<std::string> DynamicLibs;
static std::vector<std::string> DynamicLibHeaders;

/* patchModuleWithNewMain patches the translated module with a new main function that passes control to runtime */
void patchModuleWithNewMain(Module* Mod) {
  assert(Mod != nullptr);
  LLVMContext& Context = Mod->getContext();
  Function *NewStart =
      Function::Create(FunctionType::get(Type::getInt32Ty(Context), false),
                       GlobalValue::LinkageTypes::ExternalLinkage, "main", Mod);
  Function *AirLiftInit = Function::Create(
      FunctionType::get(Type::getInt32Ty(Context), false),
      GlobalValue::LinkageTypes::ExternalLinkage, "airlift_init", Mod);

  BasicBlock *Entry = BasicBlock::Create(Context, "entry", NewStart);
  Value *Result = CallInst::Create(AirLiftInit, {}, "", Entry);
  ReturnInst::Create(Context, Result, Entry);
}

static std::string OutputBasename;
static bool DumpAfterAll;
static bool PrintAfterAll;
static bool NoOpt;
static bool PreserveObj;

static std::string Crt1;
static std::string Crti;
static std::string Crtn;
static std::string Libc;
static std::string DynamicLinker;

static std::string RuntimePath;

void parseConfig(const std::string &configPath) {
  std::ifstream configFile(configPath);
  if (!configFile.is_open()) {
    std::cerr << "Error: Could not open config file: " << configPath
              << std::endl;
    exit(1);
  }

  nlohmann::json config;
  try {
    configFile >> config;
  } catch (const std::exception &E) {
    std::cerr << "Error: Failed to parse config file: " << E.what()
              << std::endl;
    exit(1);
  }

  try {
    Crt1 = config.at("crt1").get<std::string>();
    Crti = config.at("crti").get<std::string>();
    Crtn = config.at("crtn").get<std::string>();
    Libc = config.at("libc").get<std::string>();
    RuntimePath = config.at("runtime").get<std::string>();
    DynamicLinker = config.at("dynamiclinker").get<std::string>();
    DynamicLibs = config.at("dynamic_libs").get<std::vector<std::string>>();
    DynamicLibHeaders =
        config.at("dynamic_libs_headers").get<std::vector<std::string>>();

    static const std::vector<std::pair<std::string, std::string *>> Fields = {
        {"crt1", &Crt1},
        {"crti", &Crti},
        {"crtn", &Crtn},
        {"libc", &Libc},
        {"dynamic linker", &DynamicLinker},
        {"translator runtime", &RuntimePath},
    };

    for (const auto &Field : Fields) {
      if (Field.second->empty()) {
        std::string Msg = "Path to " + Field.first + " should be specified!";
        if (Field.first == "translator runtime") {
          Msg += " (Hint: By default, it is located in "
                 "./build/runtime/libsgtruntime.so)";
        }
        throw std::runtime_error(Msg);
      }
    }

  } catch (const std::exception &E) {
    std::cerr << E.what() << std::endl;
    exit(1);
  }
}

void parseArgs(int argc, char **argv) {
  argparse::ArgumentParser program(argv[0]);
  program.add_argument("input").help("Name of the input file");
  program.add_argument("-o", "--output").help("Output file name");
  program.add_argument("-d", "--dump-after-all")
      .help("Dump after all stages")
      .flag();
  program.add_argument("-p", "--print-after-all")
      .help("Print after all stages")
      .flag();
  program.add_argument("-c", "--config")
      .help("Path to config file (default: ./config.json)");
  program.add_argument("--no-opt").help("Skip the optimization stage").flag();
  program.add_argument("--preserve-obj")
      .help("Preserve the intermediate .o after linking")
      .flag();

  try {
    program.parse_args(argc, argv);
  } catch (const std::exception &E) {
    std::cerr << E.what() << std::endl;
    std::cerr << program << std::endl;
    exit(1);
  }

  std::string InputName = program.get<std::string>("input");
  if (auto OutputArg = program.present("-o")) {
    OutputBasename = *OutputArg;
  } else {
    OutputBasename = InputName.substr(0, InputName.find("."));
  }

  DumpAfterAll = program.get<bool>("-d");
  PrintAfterAll = program.get<bool>("-p");
  NoOpt = program.get<bool>("--no-opt");
  PreserveObj = program.get<bool>("--preserve-obj");

  std::string ConfigPath;
  if (auto ConfigArg = program.present("-c")) {
    ConfigPath = *ConfigArg;
  } else {
    ConfigPath = "./config.json";
  }
  parseConfig(ConfigPath);
}

int main(int argc, char **argv) {
  InitLLVM X{argc, argv};
  parseArgs(argc, argv);

  std::string IRString = liftAArch64(argv[1]);
  LLVMContext Context;
  SMDiagnostic Err;

  std::unique_ptr<MemoryBuffer> MemBuf = MemoryBuffer::getMemBuffer(IRString);
  std::unique_ptr<Module> Mod = parseIR(*MemBuf, Err, Context);

  if (!Mod) {
    Err.print(argv[0], errs());
    return 1;
  }

  /* Dump module before patching it with dynamic calls and new entry point */
  if (DumpAfterAll) {
    std::error_code ec;
    llvm::raw_fd_ostream file(std::format("{}_nopatch.ll", OutputBasename),
                              ec, llvm::sys::fs::OF_None);
    if (ec) {
      llvm::errs() << "Error opening file: " << ec.message() << "\n";
      return 1;
    }
    Mod->print(file, nullptr);
    file.close();
  }

  /* Insert a new entry point into the module */
  patchModuleWithNewMain(Mod.get());

  /* Parse all headers provided in the config */
  patchDynamicCalls(Mod.get(), DynamicLibHeaders);

  /* Dump before optimization */
  if (DumpAfterAll) {
      std::error_code ec;
      llvm::raw_fd_ostream file(std::format("{}_unoptimized.ll", OutputBasename),
                                ec, llvm::sys::fs::OF_None);
      if (ec) {
        llvm::errs() << "Error opening file: " << ec.message() << "\n";
        return 1;
      }
      Mod->print(file, nullptr);
      file.close();
  }

  if (PrintAfterAll) {
    std::cout << "---------------------------------------------------------\n"
              << std::endl;
    std::cout << "Before optimization:\n\n" << IRString << std::endl;
    std::cout << "---------------------------------------------------------\n"
              << std::endl;
  }


  if (llvm::verifyModule(*Mod, &llvm::errs())) {
    llvm::errs() << "Error: Module verification failed before optimization!\n";
    return 1; // Exit or handle the error
  }

  if (!NoOpt) {
    // Create the analysis managers.
    // These must be declared in this order so that they are destroyed in the
    // correct order due to inter-analysis-manager references.
    LoopAnalysisManager LAM;
    FunctionAnalysisManager FAM;
    CGSCCAnalysisManager CGAM;
    ModuleAnalysisManager MAM;

    // Create the new pass manager builder.
    // Take a look at the PassBuilder constructor parameters for more
    // customization, e.g. specifying a TargetMachine or various debugging
    // options.
    PassBuilder PB;

    // Register all the basic analyses with the managers.
    PB.registerModuleAnalyses(MAM);
    PB.registerCGSCCAnalyses(CGAM);
    PB.registerFunctionAnalyses(FAM);
    PB.registerLoopAnalyses(LAM);
    PB.crossRegisterProxies(LAM, FAM, CGAM, MAM);

    // Create the pass manager.
    // This one corresponds to a typical -O2 optimization pipeline.
    ModulePassManager MPM =
        PB.buildPerModuleDefaultPipeline(OptimizationLevel::O2);

    MPM.run(*Mod, MAM);

    // Optimize the IR!
    if (PrintAfterAll) {
      std::cout << "---------------------------------------------------------\n"
                << std::endl;
      std::cout << "After optimization:\n" << std::endl;
      Mod->dump();
      std::cout << "---------------------------------------------------------\n"
                << std::endl;
    }

    if (DumpAfterAll) {
      std::error_code ec;
      llvm::raw_fd_ostream file(std::format("{}_optimized.ll", OutputBasename),
                                ec, llvm::sys::fs::OF_None);
      if (ec) {
        llvm::errs() << "Error opening file: " << ec.message() << "\n";
        return 1;
      }
      Mod->print(file, nullptr);
      file.close();
    }
  }

  Function *F = Mod->getFunction("airlift_main");
  if (F == nullptr) {
    F = Mod->getFunction("airlift__start");
  }
  assert(F != nullptr && "Entry function should exist!");
  F->setName("airlift_entry");

  if (llvm::verifyModule(*Mod, &llvm::errs())) {
    llvm::errs() << "Error: Module verification failed before optimization!\n";
    return 1; // Exit or handle the error
  }

  /* Initialization of the target to compile to object file later */
  /* https://llvm.org/docs/tutorial/MyFirstLanguageFrontend/LangImpl08.html */

  std::string TargetTripleStr = sys::getDefaultTargetTriple();
  Triple TargetTriple = Triple(TargetTripleStr);
  InitializeAllTargetInfos();
  InitializeAllTargets();
  InitializeAllTargetMCs();
  InitializeAllAsmParsers();
  InitializeAllAsmPrinters();

  std::string Error;
  const Target *Target = TargetRegistry::lookupTarget(TargetTriple, Error);

  // Print an error and exit if we couldn't find the requested target.
  // This generally occurs if we've forgotten to initialise the
  // TargetRegistry or we have a bogus target triple.
  if (!Target) {
    errs() << Error;
    return 1;
  }

  /* Specify features (SSE, AVX...) */
  const char *CPU = "generic";
  const char *Features = "";

  TargetOptions opt;
  TargetMachine *TM = Target->createTargetMachine(TargetTriple, CPU, Features,
                                                  opt, Reloc::PIC_);

  Mod->setDataLayout(TM->createDataLayout());
  Mod->setTargetTriple(TargetTriple);

  std::string TranslatedObjectFileName =
      std::format("{}-x86.o", OutputBasename);
  std::error_code EC;
  raw_fd_ostream dest(TranslatedObjectFileName, EC, sys::fs::OF_None);

  legacy::PassManager pass;
  if (TM->addPassesToEmitFile(pass, dest, nullptr,
                              CodeGenFileType::ObjectFile)) {
    errs() << "TargetMachine can't emit a file of this type";
    return 1;
  }

  pass.run(*Mod);
  dest.flush();

  /* Linking with runtime and linker config from config file */
  std::string LinkedName = std::format("{}.x86", OutputBasename).c_str();

  // Build linker command with dynamic libraries
  std::vector<const char*> LinkerArgs = {
    "ld", "-dynamic-linker", DynamicLinker.c_str(),
    Crt1.c_str(), Crti.c_str(), RuntimePath.c_str(),
    TranslatedObjectFileName.c_str(), "-o", LinkedName.c_str()
  };
  
  // Add all dynamic libraries
  for (const auto& lib : DynamicLibs) {
    LinkerArgs.push_back(lib.c_str());
  }
  
  // Add libc and crtn at the end
  LinkerArgs.push_back(Libc.c_str());
  LinkerArgs.push_back(Crtn.c_str());

  lld::lldMain(
      ArrayRef<const char *>(LinkerArgs),
      llvm::outs(), llvm::errs(),
      ArrayRef<lld::DriverDef>({lld::Gnu, lld::elf::link}));

  if (!PreserveObj) {
    std::remove(TranslatedObjectFileName.c_str());
  }

  std::cout << "Success! Translated binary in " << LinkedName << std::endl;
}