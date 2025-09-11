# Terminology Agent System - Bachelor Thesis

This thesis describes TAS (Terminology Agent System), a system based on the
architectural blackboard pattern and Generative AI (GenAI), integrating Large
Language Models (LLMs). TAS uses an event based system to coordinate various
Knowledge Sources (KS), which are responsible for text extraction, text normal-
ization, and definition generation. Its modular design enables the extension for
new algorithms and external data sources.

## Getting started

This project was built using python 3.11. First, install the projects dependencies using poetry:

```shell
pip install poetry
poetry install
```

The system requires a valid OpenAI API key passed as environment variable:

```shell
export OPENAI_API_KEY=<your-api-key>
```

TAS contains a simple fastapi interface. In order to start the server, run the following commands:

```shell
pip install "fastapi[standard]"
fastapi dev src/main.py
```

## Tests

In order to reproduce the test results from Appendix B, start the server and run the following commands:

```shell
python -m unittest tests/test_definition.py
python -m unittest tests/test_extract.py
python -m unittest -v tests.test_integration.TestIntegrationTerminology.testExtractDomainTerminology_LLM
python -m unittest -v tests.test_integration.TestIntegrationTerminology.testExtractDomainTerminology
```

In order to run the performance evaluation, run the `test_performance.ipynb` jupyter notebook.