#!/bin/bash

../llama.cpp/llama-bench -m ../models/Meta-Llama-3-8B-Instruct-GGUF/Meta-Llama-3-8B-Instruct.Q2_K.gguf -p 0 -n 64,128,256,512,1024 -o json -b 256,512,1024  -ngl 40,80,160 -ctk f16 -ctv f16  -nkvo 0,1 -fa 0,1 --progress > llama_Q2_f16kv.json
