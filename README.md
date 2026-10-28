# signal-processing-ai999x — offline, verified signal processing solver with step-by-step answers

[![tests](https://github.com/AI999X/signal-processing-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/signal-processing-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/signal-processing-ai999x)](https://pypi.org/project/signal-processing-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/signal-processing-ai999x)

**[Install](https://pypi.org/project/signal-processing-ai999x/)** · **[Capability guide](https://huggingface.co/AI999X/signal-processing-ai999x/tree/main/docs)** · **[Evaluation](https://huggingface.co/AI999X/signal-processing-ai999x/tree/main/evaluation)** · **[AI999X on Hugging Face](https://huggingface.co/AI999X)** · **[Website](https://ai999x.com)** · **[Support: Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi)**

License: Apache-2.0 · Author: Ibnul Jaif Farabi (creator and engineer) · Organization: AI999X · Version 1.0.3

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
| Package | `signal-processing-ai999x` 1.0.3 (import `signal_processing_ai999x`) |
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

Command line: `signal-processing-ai999x "<question>"`, `signal-processing-ai999x --capabilities`. The outputs above were produced by running this exact code on signal-processing-ai999x 1.0.3.

## Find the right capability, and draw it

`signal_processing.search()` finds which certified capability a question is about — across **every** AI999X model, so a question for another model points you to it. It runs **[ai999x-encoder](https://huggingface.co/AI999X/ai999x-encoder)**, AI999X's own capability encoder (4.7M parameters, a 4.9 MB ONNX model, offline; the root `model.safetensors` of this repository): `pip install "signal-processing-ai999x[search]"`. `ask(question, suggest=True)` adds the same pointers to a question it does not answer; an answer itself never changes.

```python
import signal_processing_ai999x as signal_processing

signal_processing.search("Solve x^2 - 5x + 6 = 0", k=2)
# -> Roots of a quadratic equation (math-foundations-ai999x, 0.63); Roots of a cubic equation (math-foundations-ai999x, 0.59)
```

## How every answer is verified

1. **Reading** — the question is turned into typed quantities (value, unit, kind, role) and one asked quantity; every stated number must be used or the question is refused.
2. **Planning** — only certified capabilities may answer; a capability applies only under the conditions the wording states (for example a pendulum, a liquid at rest, boiling).
3. **Computing** — exact symbolic or high-precision numeric evaluation.
4. **Checking** — dimensions, substitution back into the defining equation, and agreement between independent readings; any disagreement becomes a refusal, never a number.

Capabilities are certified before they ship: each must reproduce hand-worked reference cases, reject a deliberately wrong variant (a control), and leave every benchmark answer unchanged.

## Worked examples

Each question was put to `signal-processing-ai999x`; the answer is the one it returned, with the steps it took.

**Nyquist rate** · _Sampling and signal basics_

> What is the Nyquist rate for a signal with a maximum frequency of 4 kHz?

→ **fs_min = 8000 Hz, Ts_max = 0.000125 s**

<details><summary>Steps</summary>

- Nyquist rate: fmax = 4 kHz → fs_min = 8000 Hz, Ts_max = 0.000125 s

</details>

**DFT spectrum of a sequence (FFT)** · _DFT and FFT_

> Compute the DFT of x = [1, 2, 3, 4].

→ **peak_magnitude = 10, peak_bin = 0, magnitude = [10, 2.828427125, 2, 2.828427125], phase = [0, 2.35619449, 3.141592654, -2.35619449], real = [10, -2, -2, -2], imag = [0, 2, 0, -2]**

<details><summary>Steps</summary>

- DFT spectrum of a sequence (FFT): x = [1, 2, 3, 4] → peak_magnitude = 10, peak_bin = 0, magnitude = [10, 2.828427125, 2, 2.828427125], phase = [0, 2.35619449, 3.141592654, -2.35619449], real = [10, -2, -2, -2], imag = [0, 2, 0, -2]

</details>

**Bit error rate of BPSK (and coherent BFSK) in AWGN** · _Digital communication_

> What is the bit error rate of BPSK at Eb/N0 = 8 dB?

→ **ber = 0.0001909077741, ber_bfsk = 0.0060043864**

<details><summary>Steps</summary>

- Bit error rate of BPSK (and coherent BFSK) in AWGN: ebn0_db = 8 → ber = 0.0001909077741, ber_bfsk = 0.0060043864

</details>

**Reflection coefficient, VSWR and return loss of a load** · _Transmission lines_

> A 50 ohm line is terminated in a load of 75 + j25 ohm. Find the reflection coefficient and the VSWR.

→ **gamma_mag = 0.2773500981, gamma_angle_deg = 33.69006753, gamma_re = 0.2307692308, gamma_im = 0.1538461538, vswr = 1.767591879, return_loss_db = 11.13943352, mismatch_loss_db = 0.3476210626**

<details><summary>Steps</summary>

- Reflection coefficient, VSWR and return loss of a load: RL = 75 ohm, XL = 25 ohm, Z0 = 50 ohm → gamma_mag = 0.2773500981, gamma_angle_deg = 33.69006753, gamma_re = 0.2307692308, gamma_im = 0.1538461538, vswr = 1.767591879, return_loss_db = 11.13943352, mismatch_loss_db = 0.3476210626

</details>

**Digital Butterworth filter design (bilinear transform)** · _Filter design_

`m.solve("butterworth_design", N=2, fc="1 kHz", fs="8 kHz", btype="lowpass")`

→ **b = [0.09763107294, 0.1952621459, 0.09763107294], a = [1, -0.9428090416, 0.3333333333]**

<details><summary>Steps</summary>

- Digital Butterworth filter design (bilinear transform): N = 2, fc = 1 kHz, fs = 8 kHz, btype = lowpass → b = [0.09763107294, 0.1952621459, 0.09763107294], a = [1, -0.9428090416, 0.3333333333]

</details>

**Magnitude of an analog Butterworth low-pass response** · _Analog filters_

> What is the magnitude response of a 4th-order Butterworth low-pass filter with a cutoff frequency of 1 kHz at 2 kHz?

→ **magnitude = 0.06237828616, attenuation_db = 24.09933123**

<details><summary>Steps</summary>

- Magnitude of an analog Butterworth low-pass response: N = 4, fc = 1 kHz, f = 2 kHz → magnitude = 0.06237828616, attenuation_db = 24.09933123

</details>

## Figures

Every point was computed by `signal-processing-ai999x`.

**DFT magnitude spectrum of a two-tone signal**

![DFT magnitude spectrum of a two-tone signal — computed by signal-processing-ai999x](https://huggingface.co/AI999X/signal-processing-ai999x/resolve/main/figures/dft-spectrum-spectrum.svg)

_Magnitude of the 32-point DFT returned by the package for x[n] = sin(2π·3n/32) + 0.5 sin(2π·7n/32), n = 0…31 (bins 0–16 shown; the rest mirror them). The two tones appear at bins 3 and 7; marked: |X[3]| = 16, |X[7]| = 8._

**Butterworth low-pass responses of four orders**

![Butterworth low-pass responses of four orders — computed by signal-processing-ai999x](https://huggingface.co/AI999X/signal-processing-ai999x/resolve/main/figures/butterworth-magnitude-response.svg)

_Gain of Butterworth low-pass filters of order N = 1, 2, 4 and 8 with cutoff fc = 1000 Hz (each curve 81 frequencies computed by the package, 0.1 fc to 10 fc on a log axis); every order is 3 dB down at fc and falls by 20N dB per decade. Marked: f = 2000 Hz, N = 4, gain = −24.0993 dB._

**A designed digital filter: impulse and step response**

![A designed digital filter: impulse and step response — computed by signal-processing-ai999x](https://huggingface.co/AI999X/signal-processing-ai999x/resolve/main/figures/butterworth-design-stems.svg)

_Impulse response (stems) and step response (line) of the order-4 Butterworth low-pass filter designed by the package for fc = 1000 Hz at fs = 8000 Hz (b = [0.01021, 0.04084, 0.06126, 0.04084, 0.01021], a = [1, -1.968, 1.736, -0.7245, 0.1204]), 30 samples each computed by the package; the step response settles at the DC gain 1 after overshooting to 1.11966. Marked: the largest impulse sample h[4] = 0.289522._

**Poles and zeros of a digital low-pass filter**

![Poles and zeros of a digital low-pass filter — computed by signal-processing-ai999x](https://huggingface.co/AI999X/signal-processing-ai999x/resolve/main/figures/poles-zeros-poles.svg)

_Poles and zeros returned by the package for the order-4 Butterworth low-pass filter (fc = 1000 Hz, fs = 8000 Hz) designed by it: 4 poles, largest radius 0.757669 (inside the unit circle: stable), and 4 zeros clustered at z = −1 (each within 0.000351186 of it)._

**Filtering a signal: input and FIR low-pass output**

![Filtering a signal: input and FIR low-pass output — computed by signal-processing-ai999x](https://huggingface.co/AI999X/signal-processing-ai999x/resolve/main/figures/fir-filter-output-signal.svg)

_A sampled signal (fs = 8000 Hz; a 166.7 Hz tone plus a 3000 Hz tone of amplitude 0.4) and the output of the 21-tap Hamming-window FIR low-pass filter (fc = 1000 Hz) designed by the package, the output computed by it sample by sample: the 3000 Hz component is removed and the slow tone passes, delayed by 10 samples (1.25 ms)._

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
@software{signal_processing_ai999x_1_0_3,
  title  = {signal-processing-ai999x: an offline, verified signal processing solver},
  author = {Farabi, Ibnul Jaif}, organization = {AI999X},
  year   = {2026}, version = {1.0.3},
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
