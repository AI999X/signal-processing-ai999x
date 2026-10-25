# signal-processing-ai999x — offline, verified signal processing solver with step-by-step answers

[![tests](https://github.com/AI999X/signal-processing-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/signal-processing-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/signal-processing-ai999x)](https://pypi.org/project/signal-processing-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/signal-processing-ai999x)

**[Install](https://pypi.org/project/signal-processing-ai999x/)** · **[Capability guide](https://huggingface.co/AI999X/signal-processing-ai999x/tree/main/docs)** · **[Evaluation](https://huggingface.co/AI999X/signal-processing-ai999x/tree/main/evaluation)** · **[AI999X on Hugging Face](https://huggingface.co/AI999X)** · **[Website](https://ai999x.com)** · **[Support: Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi)**

License: Apache-2.0 · Author: Ibnul Jaif Farabi (creator and engineer) · Organization: AI999X · Version 1.0.0

Field: **Engineering** · Tasks: Question Answering, Token Classification, Text Classification, Sentence Similarity, Feature Extraction

signal-processing-ai999x is an offline, deterministic Python solver that answers signal processing questions with verified results from 74 certified capabilities, gives 334 documented answers identically on Windows and Linux, and refuses rather than guesses when an answer cannot be verified.

**signal-processing-ai999x** is a compact, deterministic signal processing engine for Python. It reads a question in plain English (or a typed call), picks a certified signal processing capability, computes the answer exactly, checks it, and returns the result with its steps, units, LaTeX and assumptions — or an honest refusal that says what is missing. It runs fully offline on any CPU, needs no GPU, no API key and no internet, and is built to give **zero wrong answers**: when it is not sure, it refuses instead of guessing.

[![Buy me a coffee](https://img.shields.io/badge/Buy%20me%20a%20coffee-support%20development-FFDD00?logo=buymeacoffee&logoColor=black)](https://buymeacoffee.com/ibnuljaiffarabi) ![License](https://img.shields.io/badge/license-Apache--2.0-blue) ![Python](https://img.shields.io/badge/python-3.10%20%7C%203.11%20%7C%203.12%20%7C%203.13%20%7C%203.14-blue) ![Offline](https://img.shields.io/badge/runs-offline%20on%20CPU-green)

## Highlights

- **Verified, not generated.** Every answer is computed by a certified signal processing capability and checked (substituted back, dimension-checked or compared with an independent computation) before it is returned.
- **Zero wrong answers by design.** Seven explicit outcomes — SOLVED, NEEDS_INPUT, AMBIGUOUS, INVALID_INPUT, OUT_OF_SCOPE, UNRECOGNIZED_FORMAT, INTERNAL_ERROR — never a silent guess. Every one of its 334 documented answers is reproduced exactly on every tested platform.
- **Plain-English questions and a guaranteed typed interface.** `ask()` reads documented question forms; `solve(capability_id, **inputs)` always solves a supported task with valid inputs.
- **Explains itself.** Steps, formulas in LaTeX, units, assumptions and the model used, ready for notebooks, tutoring apps and homework checkers.
- **Small and portable.** Pure Python (sympy, mpmath); identical answers on Windows and Linux (Python 3.10, 3.11, 3.12, 3.13, 3.14).
- **Coverage:** sampling, Fourier series and transforms, the DFT and FFT, filters, convolution, z-transforms, modulation, communication links, RF transmission lines and radar.

## Model overview

| Property | Value |
|---|---|
| Package | `signal-processing-ai999x` 1.0.0 (import `signal_processing_ai999x`) |
| Type | Deterministic neuro-symbolic signal processing engine (rule-based reading + certified symbolic capabilities) |
| Capabilities | 74 certified procedures |
| Knowledge pack | version 1.0.0 |
| Golden answers | 334 documented answers, identical on every tested platform |
| Platforms tested | Windows (Python 3.13), Linux x86-64 (Python 3.10, 3.11, 3.12, 3.13, 3.14) |
| Runtime | CPU only, offline; no GPU, no network, no API key |
| License | Apache-2.0 |
| Creator / engineer | Ibnul Jaif Farabi |
| Organization | AI999X |

## Quickstart

```bash
pip install signal-processing-ai999x
```

```python
import signal_processing_ai999x as signal_processing

signal_processing.ask("What is the Nyquist rate for a signal with a maximum frequency of 4 kHz?").show()
# -> SOLVED — fs_min = 8000, Ts_max = 0.000125  (nyquist_rate)
signal_processing.ask("Compute the DFT of x = [1, 2, 3, 4].").show()
# -> SOLVED — peak_magnitude = 10, peak_bin = 0, magnitude = [10.0, 2.8284271247461903, 2.0, 2.8284271247461903], phase = [0.0, 2.356194490192345, 3.141592653589793, -2.356194490192345]  (dft_spectrum)
signal_processing.ask("What is the bit error rate of BPSK at Eb/N0 = 8 dB?").show()
# -> SOLVED — ber = 0.000190908, ber_bfsk = 0.00600439  (bpsk_ber)
signal_processing.ask("A 50 ohm line is terminated in a load of 75 + j25 ohm. Find the reflection coefficient and the VSWR.").show()
# -> SOLVED — gamma_mag = 0.27735, gamma_angle_deg = 33.6901, gamma_re = 0.230769, gamma_im = 0.153846  (reflection_coefficient)
signal_processing.solve("butterworth_design", N=2, fc="1 kHz", fs="8 kHz", btype="lowpass").show()
# -> SOLVED — b = [0.0976310729378, 0.195262145876, 0.0976310729378], a = [1.0, -0.942809041582, 0.333333333333]  (butterworth_design)
signal_processing.ask("Who created you?").message
# -> signal-processing-ai999x was created and engineered by Ibnul Jaif Farabi at AI999X.
```

Command line: `signal-processing-ai999x "<question>"`, `signal-processing-ai999x --capabilities`. The outputs above were produced by running this exact code on signal-processing-ai999x 1.0.0.

## How every answer is verified

1. **Reading** — the question is turned into typed quantities (value, unit, kind, role) and one asked quantity; every stated number must be used or the question is refused.
2. **Planning** — only certified capabilities may answer; a capability applies only under the conditions the wording states (for example a pendulum, a liquid at rest, boiling).
3. **Computing** — exact symbolic or high-precision numeric evaluation.
4. **Checking** — dimensions, substitution back into the defining equation, and agreement between independent readings; any disagreement becomes a refusal, never a number.

Capabilities are certified before they ship: each must reproduce hand-worked reference cases, reject a deliberately wrong variant (a control), and leave every benchmark answer unchanged.

## Capabilities

74 certified procedures. A few of them:

- **Procedures:** AM modulation index from the envelope; Alias (apparent) frequency after sampling; Amplitude (voltage) ratio to decibels; Analytic signal, Hilbert transform and envelope; Bilinear transform of a first-order low-pass filter; Bit and symbol error rates of QPSK in AWGN; Bit error rate of BPSK (and coherent BFSK) in AWGN; Bit error rates of DPSK and noncoherent BFSK; Characteristic impedance of a coaxial line; Circular (cyclic) convolution; Coefficients of a window function; Cross-correlation and the lag of its peak; DC gain of a digital filter; DFT spectrum of a sequence (FFT); Decibels to amplitude (voltage) ratio; Decibels to power ratio; Digital Butterworth filter design (bilinear transform); Doppler shift of a radar echo; FIR filter output; FIR low-pass filter design by the window method; FM spectrum: Carson bandwidth and Bessel sideband amplitudes; Fourier transform of a decaying exponential; Fourier transform of a rectangular pulse; Free-space path loss …

`signal_processing_ai999x.templates()` lists every question form `ask()` reads; the capability guide (PDF, every example executed) is included in this repository.

## Out of scope

- Open-ended explanation, essay writing or tutoring dialogue: it is a solver, not a chatbot.
- Questions outside its certified signal processing capabilities, and any question whose stated numbers it cannot all use — it refuses with the reason instead.
- High-stakes engineering, medical or legal decisions without independent review.

## Limitations

- It answers what its certified capabilities cover (sampling, Fourier series and transforms, the DFT and FFT, filters, convolution, z-transforms, modulation, communication links, RF transmission lines and radar); anything else is refused with a reason.
- `ask()` reads documented question forms and common phrasings; unusual wording may be refused — use `solve()` for guaranteed results.
- Results are as correct as the certified relations and their stated conditions; a certified relation is a checked model, not a claim about every physical situation.

## Citation

```bibtex
@software{signal_processing_ai999x_1_0_0,
  title  = {signal-processing-ai999x: an offline, verified signal processing solver},
  author = {Farabi, Ibnul Jaif}, organization = {AI999X},
  year   = {2026}, version = {1.0.0},
  url    = {https://huggingface.co/AI999X/signal-processing-ai999x}
}
```

## Contact

Questions, bug reports and requests: open a discussion on https://huggingface.co/AI999X/signal-processing-ai999x/discussions or visit https://ai999x.com.

## Support the project

signal-processing-ai999x is free and open source. If it helps you, you can support further development on [Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi).

Created and engineered by **Ibnul Jaif Farabi** · **[AI999X](https://huggingface.co/AI999X)** · [ai999x.com](https://ai999x.com)

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/signal-processing-ai999x/) (`pip install .` builds it). Releases are tagged `v<version>`; the wheel is attached to each [GitHub release](https://github.com/AI999X/signal-processing-ai999x/releases). The optional ONNX models (formula graphs and, where measured, a decision model) are on [Hugging Face](https://huggingface.co/AI999X/signal-processing-ai999x/tree/main/onnx).
