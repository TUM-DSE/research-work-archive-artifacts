from typing import Callable, Literal
import matplotlib as mpl
import matplotlib.pyplot as plt
from pathlib import Path

from plot import parallel_non_contiguous
from plot import cas
from plot import non_contiguous
from plot import contiguous
from plot import malloc
from plot import contiguous_tagging

Experiments = Literal[
    "cas",
    "contiguous",
    "non_contiguous",
    "contiguous_tagging",
    "malloc",
    "parallel_non_contiguous",
]
PlottingFunction = Callable[[Path, str], None]

experiments: dict[str, PlottingFunction] = {
    "cas": lambda r, t: cas.plot(r, t),
    "contiguous": lambda r, t: contiguous.plot(r, t),
    "non_contiguous": lambda r, t: non_contiguous.plot(r, t),
    "contiguous_tagging": lambda r, t: contiguous_tagging.plot(r, t),
    "malloc": lambda r, t: malloc.plot(r, t),
    "parallel_non_contiguous": lambda r, t: parallel_non_contiguous.plot(r, t),
}


def experiment_choices() -> list[str]:
    return list(experiments.keys())


def plot(output_root: Path, experiment: Experiments, format: Literal["pdf", "png"]):
    rcParams = {
        "font.family": "serif",
        "font.size": 11,
        "pgf.rcfonts": False,
    }

    if format == "pdf":
        mpl.use("pdf")
        plt.rcParams["text.latex.preamble"] = r"\renewcommand{\mathdefault}[1][]{}"
        rcParams["pgf.texsystem"] = "pdflatex"

    mpl.rcParams.update(rcParams)
    experiments[experiment](output_root, format)
