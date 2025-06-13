# llm-os
Trustworthy on-device LLM inference framework.

### Benchmarks
We use llama-bench from llama.cpp for benchmarking. The main command looks like this:
`./llama-bench -m models/meta-llama-3.1-8b-instruct.f16.gguf -p 0 -n 2048,4096 -ngl 999 -o json --progress > output.json`

```
-m           model to be used
-p           benchmark for prompt processing, we disable it with zero since we are interested in text generation
-n           number of text tokens to generate. Each number will be executed as a different experiment
-ngl         number of layers to offload to GPU. Setting it high ensures all layers are loaded to GPU
-o           set output file format to json
--progress   shows the progress of the benchmark during execution
```
