# Neural-Network-Based Lightweight Predecoder for Speculative Pauli Frame Updates

**Author:** Andrei Dolmatov\
**Advisor:** Aleksandra Świerkowska\
**Host institution:** Chair of Computer Systems, Technical University of Munich (TUM)\
**Degree-awarding institution:** Catholic University of Eichstätt-Ingolstadt (KU)\
**Research type:** Bachelor's thesis\
**Year:** 2026

## Overview

This project investigates whether a lightweight neural predecoder can reduce
classical-decoder workload while preserving logical-flip prediction accuracy
and limiting execution time and memory use.

A graph neural network processes a detector–error graph and predicts the flip
of one logical observable. A calibrated confidence rule accepts selected
predictions and delegates the remaining shots to a classical decoder. The
evaluation compares fixed and fresh training schedules, structured channel
pruning, and mixed-precision inference.

The experiments use recorded distance-five, X-basis surface-code data from
Google's Sycamore experiments, with 53 samples and histories of 5, 9, and
25 rounds. Each round-count study evaluates 12 model variants on 397,500
held-out shots.

The prototype implements complete-shot prediction and delegation in software.
Connecting its output to a live controller and applying Pauli-frame updates
requires a controller interface. The reported classical comparison uses saved
decoder predictions. Avoided calls measure workload reduction, while an
end-to-end speedup requires live classical and hybrid timing measurements.

## Reproduction status of this checkout

The local source snapshot used to prepare this README is incomplete for
training. The existing runner imports the following files, which are absent:

```text
core/GCN/GCN.py
core/GCN/calibration.py
core/GCN/collection.py
core/GCN/data_loader.py
core/GCN/evaluate.py
core/GCN/graph_data.py
core/GCN/metrics.py
core/GCN/reference.py
core/GCN/run_io.py
```

Restore the complete benchmark implementation from the working project before
running training or benchmark report export. The standalone reference-comparison
script is independent of these modules and can process completed study artifacts.

The bundled `benchmark_config.py` also contains older values for fresh-shot
count and baseline learning rate. The reproduction settings below explicitly
replace them. The current `docker/Dockerfile` selects TensorFlow 2.21.0, while
the thesis measurements used 2.20.0. Its requirements file contains invalid
single-equals version pins and omits dependencies needed by the benchmark.
The installation commands below specify the environment directly.

## Requirements

The recorded benchmark environment used:

| Component | Version or setting |
| --- | --- |
| Python | 3.11.13 |
| TensorFlow / Keras | TensorFlow 2.20.0 with Keras |
| NumPy | 1.26.4 |
| Platform | Linux in Docker |
| GPU | NVIDIA H200 NVL |
| TensorFlow CUDA build | 12.5.1 |
| cuDNN | Major version 9 |

Other Python dependencies include Stim, SciPy, Matplotlib, scikit-learn, and
psutil. PyMatching supports the optional live matching-decoder path.

GPU training requires a compatible NVIDIA driver and a TensorFlow installation
that detects the GPU. The reference-comparison stage uses NumPy, SciPy, and
Matplotlib and runs on a CPU by default. It requires the completed prediction
artifacts and corresponding dataset reference files.

Original recordings and completed benchmark runs must be supplied separately.
Matching these versions alone does not guarantee identical timing. The saved
reports do not identify every driver, compiler, and numerical setting.

## Installation

Run commands from the directory containing `run_benchmark.py`. For a Linux
NVIDIA GPU environment with Python 3.11:

```bash
python3.11 -m venv .venv
source .venv/bin/activate
python -m pip install --upgrade pip
python -m pip install "tensorflow[and-cuda]==2.20.0" "numpy==1.26.4" \
  "stim==1.16.0" scipy matplotlib scikit-learn psutil "pymatching>=2.1,<3"
python -c "import tensorflow as tf; print(tf.config.list_physical_devices('GPU'))"
```

TensorFlow documents this GPU installation approach in its
[installation guide](https://www.tensorflow.org/install/pip).
The command installs an environment for running the workflow. Retain the
original experiment environment when comparing runtime measurements.

For reference comparison only, activate a Python environment and install:

```bash
python -m pip install -r reference_comparison_requirements.txt
```

No root-level `requirements.txt` is included in this checkout.

## Dataset preparation

Use the surface-code collection associated with
[Sivak, Newman and Klimov (2024)](https://doi.org/10.1103/PhysRevLett.133.150603).
The study uses recorded experimental shots and their supplied labels.

Set `data_root` to the directory directly containing `sample_00` through
`sample_52`. For example, if the collection is stored in
`data/sycamore_surface_code_d3_d5`, use that path as `data_root`.

Each sample must contain the following files for the selected round count:

```text
data/
├── sample_00/
│   └── d5_at_q6_5/
│       └── X/
│           ├── r05/
│           │   ├── circuit_ideal.stim
│           │   ├── detection_events.b8
│           │   ├── obs_flips_actual.b8
│           │   ├── dems/
│           │   │   └── correlations.dem
│           │   └── obs_flips_predicted/
│           │       └── dem_correlations/
│           │           └── correlated_matching.b8
│           ├── r09/  [same required files]
│           └── r25/  [same required files]
├── sample_01/
└── ... sample_52/
```

The first four files support neural training and evaluation. The saved
`correlated_matching.b8` predictions are additionally required for the default
classical-reference comparison. Preserve sample names and shot order so that
predictions can be aligned to the same ground-truth labels.

In Docker, `data_root` must name a path visible inside the container. For
example, use `/work/data` if that directory contains the mounted samples.

## Running the project

The benchmark reads `benchmark_config.py`. The launcher takes no command-line
configuration arguments. Relative data and output paths resolve from the
project directory containing the launcher.

For the thesis settings, add these overrides after the existing `CONFIG`
dictionary, or update the corresponding entries directly:

```python
CONFIG.update({
    "data_root": "data",
    "configuration": "d5_at_q6_5/X/r05",
    "output_root": "experiments/sycamore_benchmark_r05",
    "expected_samples": 53,
    "action": "all",
    "resume": True,
    "max_hours": None,
    "fresh_shots_per_epoch": 120_000,
    "fixed_training_shots": 120_000,
})
CONFIG["model"]["learning_rate"] = 0.0001
```

Keep the other shared settings as follows:

| Setting | Thesis configuration |
| --- | --- |
| Training schedules | `fresh`, `fixed` |
| Training batch sizes | 64, 128 |
| Initialization and split seeds | 73 |
| Baseline | 13 features, 96 hidden channels, 3 residual GCN blocks, dense head 128/64/1 |
| Dropout | 0.1 |
| Baseline training | At most 100 epochs |
| Pruned model | 64 hidden channels, dense head 64/32/1 |
| Pruning fine-tuning | At most 20 epochs, learning rate 0.0001 |
| Early stopping | Validation loss, patience 15, minimum improvement 0.00001 |
| Compression eligibility | Validation BCE increase at most 0.01 relative to the direct parent |
| Mixed precision | Export from the eligible pruned model, with no additional training |
| Confidence calibration | Temperature scaling, 1% accepted-error target, alpha 0.05, minimum 20 accepted selection shots |
| Bootstrap | 2,000 replicates |
| Latency | GPU, host inputs, batch 1, 50 warm-up calls, 500 timed calls per sample |
| Timing inputs | 32 distinct shots per sample |
| Throughput | Batch 8, five repetitions |
| Original benchmark reference settings | `references = {}`, `live_dem = None` |

Start the configured study:

```bash
python run_benchmark.py
```

Repeat for the other round counts using a separate output directory for each:

| Round count | `configuration` | `output_root` |
| --- | --- | --- |
| 5 | `d5_at_q6_5/X/r05` | `experiments/sycamore_benchmark_r05` |
| 9 | `d5_at_q6_5/X/r09` | `experiments/sycamore_benchmark_r09` |
| 25 | `d5_at_q6_5/X/r25` | `experiments/sycamore_benchmark_r25` |

Fixed training reuses 120,000 shots. Fresh training consumes unused recorded
shots, up to 120,000 per epoch. With 75,000 shots per sample, its training pool
contains 3,180,000 shots, allowing 26 full epochs and a final 60,000-shot epoch.
Both schedules use early stopping. Pruning fine-tuning reuses a fixed subset.

`resume = True` skips completed stages. An interrupted training stage restarts
from its stage initialization or parent model, rather than continuing at its
last completed epoch. Changing scientific parameters requires a new
`output_root`. `max_hours` limits one invocation, and `None` disables that limit.
It does not guarantee that the entire study finishes within a particular time.

## Reproducing the evaluation

### Training, calibration, and testing

With `action = "all"`, the runner trains the baselines, creates compressed
candidates, evaluates validation eligibility, calibrates confidence rules,
freezes eligible models and gates, evaluates test predictions, benchmarks
execution, and exports reports.

The partition within every sample is 80% training, 5% validation, 5%
calibration, and 10% testing. Within a round-count study, variants share the
test identifiers. The 198,750 calibration shots are divided equally between
temperature fitting and threshold selection.

Other actions use the same launch command:

| `action` | Purpose |
| --- | --- |
| `develop` | Train and calibrate, stopping before final test evaluation |
| `final` | Finish evaluation and benchmarking of an already developed study |
| `report` | Regenerate exports from completed test and benchmark artifacts |

The output directory contains `manifest.json`, `final_selection.json`,
per-model artifacts, and a `results/` directory. Model directories retain
configurations, graphs, weights, partitions, calibration data, test predictions,
and completed timing reports. Keep the complete study directory when copying
results to another machine.

### Classical-reference and threshold comparison

Edit `reference_comparison_config.py` to point to all completed studies:

```python
CONFIG["data_root"] = "data"
CONFIG["studies"] = {
    "r05": "experiments/sycamore_benchmark_r05",
    "r09": "experiments/sycamore_benchmark_r09",
    "r25": "experiments/sycamore_benchmark_r25",
}
CONFIG["output_root"] = "reference_comparisons"
CONFIG["references"] = {
    "correlated_matching":
        "obs_flips_predicted/dem_correlations/correlated_matching.b8",
}
```

Then run:

```bash
python run_reference_comparison.py
```

This stage reuses saved neural probabilities, fitted temperatures, and
classical predictions. It aligns records by sample and shot identifier and
checks artifact hashes. It requires no retraining or repeated latency benchmark.

The configured thresholds compare accepted coverage, overall hybrid accuracy,
accuracy of both decoders on the same accepted subset, introduced errors,
corrected errors, and uncertainty bounds. The original fitted gate remains a
separate policy. The threshold sweep is exploratory and does not replace that
gate or establish a deployment threshold from test outcomes.

See [REFERENCE_COMPARISON.md](REFERENCE_COMPARISON.md) for configuration details,
input checks, and statistical assumptions.

### Metrics, figures, and tables

Prediction metrics include accuracy, logical error rate per complete shot,
binary cross-entropy, and ROC AUC. Resource measurements include median, P95,
and P99 latency, batched throughput, exported parameter bytes, resident
parameter bytes, process RSS, and TensorFlow allocator peaks.

To regenerate benchmark exports, keep the original study configuration,
set `CONFIG["action"] = "report"`, and run:

```bash
python run_benchmark.py
```

Outputs under the study's `results/` directory include:

- `predictions.csv`, `variant_summary.csv`, and `paired_comparisons.csv`
- `configuration.csv`, `graphs.csv`, `gates.csv`, `resources.csv`, and `latency.csv`
- `per_sample_predictions.csv`, `per_sample_gates.csv`, and `per_sample_latency.csv`
- `*_table.tex`, `diagnostic_figures.tex`, diagnostic PDFs, and `sources.json`

Running `run_reference_comparison.py` also regenerates comparison figures and
tables in a new timestamped directory under `reference_comparisons/`, including:

- `comparison.json` and `comparison.csv`
- `per_acquisition.csv`, whose original filename is retained for compatibility
- PNG/PDF figures under `plots/`
- LaTeX tables under `tables/`

These scripts generate evaluation exports. The authored thesis layout and
presentation-specific figures belong to the separate manuscript and
presentation sources.

## Interpretation of the results

The highest observed neural accuracy is 62.32% at five rounds for the pruned
fixed-data, batch-64 model. Correlated matching reaches 82.11% on the same
test shots. All 36 fitted confidence policies delegate every shot at the
original 1% accepted-error target.

Compression reduces model-weight storage, while single-shot GPU latency
changes little after pruning. The exploratory threshold comparisons quantify
accuracy tradeoffs and avoided calls. They do not establish reliable workload
reduction with preserved accuracy or an overall decoding speedup.

The study uses one initialization seed and holds out shots from samples also
represented in training. The supplied DEMs remain fixed. Further experiments
are needed to assess new samples, other code configurations, and live runtime.

## Thesis and presentation

**Title:** Neural-Network-Based Lightweight Predecoder for Speculative Pauli Frame Updates

**Research archive:** [TUM-DSE research work archive](https://github.com/TUM-DSE/research-work-archive)

## Original repository

[dabokva/quant](https://github.com/dabokva/quant)

## License

This project is licensed under the [MIT License](LICENCE).

Dataset access and reuse remain subject to the dataset's own terms.
