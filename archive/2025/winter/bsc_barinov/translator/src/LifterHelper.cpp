#include "LifterHelper.h" 
#include <iostream>

/* Provided by liblifter, used to lift an assembly file to LLVM IR */
extern "C" char* airlift_lift(const char* assembly_path);

/* Provided by liblifter, used to free the resulting */
extern "C" void airlift_free_string(char* string_ptr);

/* Lift */
std::string liftAArch64(const std::string& AssemblyPath) {
  char* AirliftResult = airlift_lift(AssemblyPath.c_str());

  if (AirliftResult == nullptr) {
    std::cerr << "Lifter failed to lift module, terminating translator\n";
    std::exit(1);
  }

  /* Copy from Lifted buffer */
  std::string Result{AirliftResult};

  /* Free the buffer provided by rust */
  airlift_free_string(AirliftResult);
  return Result;
}