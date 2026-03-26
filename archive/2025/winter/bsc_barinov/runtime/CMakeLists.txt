set(SOURCES
  src/Exceptions.cpp
  src/runtime.cpp
)
 
add_library(sgtruntime SHARED ${SOURCES})
target_include_directories(sgtruntime PRIVATE include)