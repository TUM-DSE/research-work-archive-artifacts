# Memory Tagging Extension

This tool and benchmark are designed to evaluate the performance overhead introduced with the usage of MTE.

## Usage

You can run the existing benchmark both locally and on a remote machine. The software requires certain dependencies to run:
- Makefile
- gcc (with support for `-march=armv8-5-a+memtag`)
- rsync

### Benchmarks

#### Locally

To run any experiment locally, use the following command:
```bash
./scripts/main.py --experiment <experiment> locally
```

#### Remote

To run the experiments on a remote machine, valid SSH keys must be established. The connection must allow allocating a TTY to execute and build the benchmark. Moreover, the remote machine must have the required software installed.

From the root directory of the repo, run any benchmark with the following command:
```bash
./scripts/main.py \
  --experiment <experiment> \
  remote \
  --remote-user <ssh-user> \
  --remote-host <ssh-host>
```

If an SSH jump host is needed, use the following command:
```bash
./scripts/main.py \
  --experiment <experiment> \
  remote \
  --remote-user <ssh-user> \
  --remote-host <ssh-host> \
  --remote-jump-user <ssh-jump-user> \
  --remote-jump-host <ssh-jump-host> \
  --remote-jump-port <ssh-jump-port>
```

### Additional Information

- By default, experiments are extracted from `/experiments/`. If your experiment is located in another directory, specify the base directory with `--base`.
- By default, all results of the experiments are placed into `/results`. To change this behavior, specify `--result`.
- When an experiment is executed, a context on the machine is created. This context is a temporary directory on the system. By default, after the execution of the benchmark, this directory is not cleaned up. To enable cleanup, specify `--cleanup`.

## Structure

The structure of the experiment follows this pattern:

- Experiments are placed inside the `--base` directory (default: `/experiments/`) and are referred to by the name of the experiment. For example, if we have an experiment named `my-experiment`, a directory inside `/experiments/my-experiment/` must exist.
- On execution of an experiment, the `--base` directory is synced to the context of the current run, then:
  - We run the Makefile inside this folder. This Makefile performs all steps required to run the experiment (like compiling and executing the actual experiment).
  - The results produced by the experiment are placed into `/experiments/my-experiment/results/`. These results are then synced back to the results `--result` directory (default `/results/my-experiment/`).
  - The results are processed.

## Adding Additional Experiments

To add a custom experiment called `my-experiment`, follow these steps. Assume that the standard `--base` directory (`/experiments/`) is used.

1. Create a new directory:
   ```bash
   cd <root-of-repo>/experiments/
   mkdir my-experiment
   ```
   
2. Add a Makefile inside the directory (this will be called to perform any necessary work). For example, use `make` to compile and run your experiment.

3. Add a Python script to evaluate the results of the benchmark.

4. Add the name of your experiment to the literals specified in this file:
   ```python
   Experiments = Literal[ ... "my-experiment" ]
   ```
   
5. To add your analysis step, include `"my-experiment"` in the relevant section. How you evaluate the results is up to you, but you must add a function call to the `experiments` dictionary.

Now you can run your experiment!
