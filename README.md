# signal-processing-ai999x — offline, verified signal processing solver with step-by-step answers

[![tests](https://github.com/AI999X/signal-processing-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/signal-processing-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/signal-processing-ai999x)](https://pypi.org/project/signal-processing-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/signal-processing-ai999x)

**[Desktop app & command line](https://www.ai999x.com/desktop-cli/)** · **[Compute in your browser](https://www.ai999x.com/models/signal-processing-ai999x/#try)** · **[Install (Python)](https://pypi.org/project/signal-processing-ai999x/)** · **[Guides](https://huggingface.co/AI999X/signal-processing-ai999x/tree/main/docs)** · **[Evaluation](https://huggingface.co/AI999X/signal-processing-ai999x/tree/main/evaluation)** · **[AI999X on Hugging Face](https://huggingface.co/AI999X)** · **[Website](https://ai999x.com)** · **[Support: Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi)**

License: Apache-2.0 · Author: Ibnul Jaif Farabi (creator and engineer) · Organization: AI999X · Version 1.1.0

Field: **Engineering** · Tasks: Question Answering, Token Classification, Text Classification, Sentence Similarity, Feature Extraction

signal-processing-ai999x is an offline, deterministic Python solver that answers signal processing questions with verified results from 74 certified capabilities, gives 334 documented answers identically on Windows and Linux, and refuses rather than guesses when an answer cannot be verified.

**signal-processing-ai999x** is a compact, deterministic signal processing engine for Python. It reads a question in plain English (or a typed call), picks a certified signal processing capability, computes the answer exactly, checks it, and returns the result with its steps, units, LaTeX and assumptions — or an honest refusal that says what is missing. It runs fully offline on any CPU, needs no GPU, no API key and no internet, and is built to give **zero wrong answers**: when it is not sure, it refuses instead of guessing.

[![Buy me a coffee](https://img.shields.io/badge/Buy%20me%20a%20coffee-support%20development-FFDD00?logo=buymeacoffee&logoColor=black)](https://buymeacoffee.com/ibnuljaiffarabi) ![License](https://img.shields.io/badge/license-Apache--2.0-blue) ![Python](https://img.shields.io/badge/python-3.10%20%7C%203.11%20%7C%203.12%20%7C%203.13%20%7C%203.14-blue) ![Offline](https://img.shields.io/badge/runs-offline%20on%20CPU-green) ![Rust](https://img.shields.io/badge/Rust-Windows%20%7C%20Linux%20%7C%20WebAssembly-orange)

**Run it your way.** signal-processing-ai999x comes as the AI999X desktop app and as the command-line program `signal-processing` for Windows and Linux, runs in your browser as WebAssembly, and installs as a Python package. The desktop app, the command line and the browser run the same compiled Rust program on your own device: offline, no Python needed.

## Get it

| Way to run it | For | Get it |
|---|---|---|
| **AI999X Signal Processing desktop app** 1.4.0 | Windows, Linux — this model in its own app: search, ask in words, forms, live figures, explanations, offline | [Windows (x86-64)](https://downloads.ai999x.com/desktop/1.4.0/ai999x-signal-processing-1.4.0-windows-x86_64.zip) · [Linux (x86-64)](https://downloads.ai999x.com/desktop/1.4.0/ai999x-signal-processing-1.4.0-linux-x86_64.tar.gz) |
| **AI999X desktop app** 1.4.0 | Windows, Linux — every AI999X model in one app: search, ask in words, forms, live figures, explanations, offline | [Windows (x86-64)](https://downloads.ai999x.com/desktop/1.4.0/ai999x-1.4.0-windows-x86_64.zip) · [Linux (x86-64)](https://downloads.ai999x.com/desktop/1.4.0/ai999x-1.4.0-linux-x86_64.tar.gz) |
| **Command line** (`signal-processing`) | Windows, Linux — one program per model, offline, no Python | [Windows (x86-64)](https://downloads.ai999x.com/desktop/1.4.0/ai999x-cli-1.4.0-windows-x86_64.zip) · [Linux (x86-64)](https://downloads.ai999x.com/desktop/1.4.0/ai999x-cli-1.4.0-linux-x86_64.tar.gz) |
| **In your browser** | any device — nothing to install | [Compute on the model page](https://www.ai999x.com/models/signal-processing-ai999x/#try) |
| **Python** | Python 3.10+ | `pip install signal-processing-ai999x` |

Checksums: [SHA256SUMS](https://downloads.ai999x.com/desktop/1.4.0/SHA256SUMS) · every version: [https://downloads.ai999x.com/desktop/latest/](https://downloads.ai999x.com/desktop/latest/) · install guide: [desktop & command line](https://www.ai999x.com/desktop-cli/)

## Quick start

### Desktop app

Download **AI999X Signal Processing** for your system (above), unpack it and start it; type a question in words or pick a capability, fill in the form and press Compute. Figures and an explanation of the answer appear beside it. The AI999X desktop app has the same, with Signal Processing among all AI999X models.

Linux, from a terminal:

```bash
curl -LO https://downloads.ai999x.com/desktop/1.4.0/ai999x-signal-processing-1.4.0-linux-x86_64.tar.gz
curl -LO https://downloads.ai999x.com/desktop/1.4.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-signal-processing-1.4.0-linux-x86_64.tar.gz
./ai999x-signal-processing-1.4.0-linux-x86_64/ai999x-signal-processing
```

### Command line

Linux:

```bash
curl -LO https://downloads.ai999x.com/desktop/1.4.0/ai999x-cli-1.4.0-linux-x86_64.tar.gz
curl -LO https://downloads.ai999x.com/desktop/1.4.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
mkdir -p ~/.local/share && tar xzf ai999x-cli-1.4.0-linux-x86_64.tar.gz -C ~/.local/share
echo 'export PATH="$HOME/.local/share/ai999x-cli-1.4.0-linux-x86_64:$PATH"' >> ~/.bashrc && source ~/.bashrc
signal-processing --version
```

Windows (PowerShell):

```powershell
$F = "ai999x-cli-1.4.0-windows-x86_64.zip"
Invoke-WebRequest "https://downloads.ai999x.com/desktop/1.4.0/$F" -OutFile $F
Invoke-WebRequest "https://downloads.ai999x.com/desktop/1.4.0/SHA256SUMS" -OutFile SHA256SUMS
(Get-FileHash $F -Algorithm SHA256).Hash.ToLower() -eq ((Select-String -SimpleMatch $F SHA256SUMS).Line -split ' ')[0]
Expand-Archive $F -DestinationPath "$env:LOCALAPPDATA\AI999X" -Force
$D = "$env:LOCALAPPDATA\AI999X\ai999x-cli-1.4.0-windows-x86_64"; $env:Path += ";$D"
[Environment]::SetEnvironmentVariable("Path", [Environment]::GetEnvironmentVariable("Path", "User") + ";$D", "User")
signal-processing --version
```

Every command below was run as shown; the text under it is what the program printed.

**Ask in words** — Write the question as you would say it: the program reads the quantities and their units, picks the capability and answers.

```console
$ signal-processing ask "What is the Nyquist rate for a signal with a maximum frequency of 4 kHz?"
Read as: Nyquist rate
fs_min = 8000 Hz
Ts_max = 0.000125 s
  by: Nyquist rate  (sampling theorem: fs >= 2 fmax)
  fmax = 4000 Hz
  assumes: fmax > 0; a baseband (lowpass) signal
```

**What a capability needs** — The equation or method, every input and output with its SI unit. Read this before solve: it tells you the input names.

```console
$ signal-processing info reflection_coefficient
Reflection coefficient, VSWR and return loss of a load
  Gamma = (ZL - Z0)/(ZL + Z0); VSWR = (1 + |Gamma|)/(1 - |Gamma|)
  in   RL       load resistance [ohm]
  in   Z0       characteristic impedance of the line [ohm]
  in   XL       load reactance (0 if not given) [ohm]
  out  gamma_mag |Gamma| []
  out  gamma_angle_deg angle of Gamma in degrees []
  out  gamma_re real part of Gamma []
  out  gamma_im imaginary part of Gamma []
  out  vswr     voltage standing wave ratio []
  out  return_loss_db return loss in dB []
  out  mismatch_loss_db mismatch loss in dB []
```

**Solve** — Inputs as name=value, each with any unit of the right kind. The answer lists every output, the method used, each input converted to SI, and the conditions the answer assumes.

```console
$ signal-processing solve reflection_coefficient "RL=75 ohm" "XL=25 ohm" "Z0=50 ohm"
gamma_mag = 0.2773500981
gamma_angle_deg = 33.69006753
gamma_re = 0.2307692308
gamma_im = 0.1538461538
vswr = 1.767591879
return_loss_db = 11.13943352
mismatch_loss_db = 0.3476210626
  by: Reflection coefficient, VSWR and return loss of a load  (Gamma = (ZL - Z0)/(ZL + Z0); VSWR = (1 + |Gamma|)/(1 - |Gamma|))
  RL = 75 ohm
  XL = 25 ohm
  Z0 = 50 ohm
  assumes: RL > 0; Z0 real (lossless line)
```

**Many questions at once** — One JSON request per line in, one JSON answer per line out, in order: for scripts, notebooks and agents (the program starts once). In PowerShell: Get-Content cases.jsonl | signal-processing batch

`cases.jsonl`:

```json
{"key": "reflection_coefficient", "inputs": {"RL": "75 ohm", "XL": "25 ohm", "Z0": "50 ohm"}}
{"key": "nyquist_rate", "inputs": {"fmax": "4 kHz"}}
```

```console
$ signal-processing batch < cases.jsonl
{"outcome":"SOLVED","message":"","value":0.2773500981126146,"unit":"","solved_for":"gamma_mag","si":[["RL",75.0],["XL",25.0],["Z0",50.0]],"values":[["gamma_mag",0.2773500981126146,""],["gamma_angle_deg",33.690067525979785,""],["gamma_re",0.23076923076923078,""],["gamma_im",0.15384615384615385,""],["vswr",1.7675918792439984,""],["return_loss_db",11.139433523068368,""],["mismatch_loss_db",0.34762106259211917,""]],"target":"gamma_mag","phrase":"|Gamma|","assumptions":["RL > 0","Z0 real (lossless line)"],"name":"Reflection coefficient, VSWR and return loss of a load","equation":"Gamma = (ZL - Z0)/(ZL + Z0); VSWR = (1 + |Gamma|)/(1 - |Gamma|)"}
{"outcome":"SOLVED","message":"","value":8000.0,"unit":"Hz","solved_for":"fs_min","si":[["fmax",4000.0]],"values":[["fs_min",8000.0,"Hz"],["Ts_max",0.000125,"s"]],"target":"fs_min","phrase":"Nyquist rate (minimum sampling rate)","assumptions":["fmax > 0","a baseband (lowpass) signal"],"name":"Nyquist rate","equation":"sampling theorem: fs >= 2 fmax"}
```

### In your browser

Open [Signal Processing on ai999x.com](https://www.ai999x.com/models/signal-processing-ai999x/#try), choose a capability and press **Compute**: the model runs on your own device as WebAssembly; nothing is sent anywhere. Change any input (any unit of the right kind) and the answer, its explanation and the figures follow.

In your own page: `signal_processing_ai999x.wasm` and the loader `ax-wasm.js` (https://www.ai999x.com/compute/rust/ax-wasm.js).

### Python

```bash
pip install signal-processing-ai999x
```

## Worked examples in every form

The same worked example from the command line, from Python and in the browser. Each command-line output is what the program printed.

### Magnitude of an analog Butterworth low-pass response

|H(f)| = 1/sqrt(1 + (f/fc)^(2N)) of an N-th order Butterworth low-pass filter, and the attenuation in dB.

Command line:

```console
$ signal-processing solve butterworth_magnitude N=4 "fc=1 kHz" "f=2 kHz"
magnitude = 0.06237828616
attenuation_db = 24.09933123
  by: Magnitude of an analog Butterworth low-pass response  (the Butterworth magnitude-squared function)
  fc = 1000 Hz
  f = 2000 Hz
  assumes: N >= 1; fc > 0; f >= 0
```

Python:

```python
import signal_processing_ai999x as m

a = m.solve("butterworth_magnitude", N="4", fc="1 kHz", f="2 kHz")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/signal-processing-ai999x/#cap-butterworth_magnitude) and press **Compute in your browser**.

_What this means:_ This calculation determines the magnitude response of an analog Butterworth low-pass filter. Using the filter order 4, the cutoff frequency 1 kHz, and the specific input frequency 2 kHz, the model computes the normalized gain 0.0623783. This value represents the ratio of output amplitude to input amplitude at that frequency. The corresponding attenuation in decibels, 24.0993, indicates how much signal strength is lost relative to the passband. A lower magnitude signifies greater attenuation, which is typical for frequencies above the cutoff point.

### Minimum order of a analog Butterworth filter

The smallest order meeting a passband edge with at most Rp dB attenuation and a stopband edge with at least As dB.

Command line:

```console
$ signal-processing solve butterworth_order_analog "fp=1 kHz" "fstop=2 kHz" Rp=1 As=40
order = 8
type = lowpass
  by: Minimum order of a analog Butterworth filter  (N = ceil( log10((10^(As/10)-1)/(10^(Rp/10)-1)) / (2 log10(ws/wp)) ))
  fp = 1000 Hz
  fstop = 2000 Hz
  Rp = 1
  As = 40
  assumes: 0 < Rp < As; fp != fstop
```

Python:

```python
import signal_processing_ai999x as m

a = m.solve("butterworth_order_analog", fp="1 kHz", fstop="2 kHz", Rp="1", As="40")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/signal-processing-ai999x/#cap-butterworth_order_analog) and press **Compute in your browser**.

_What this means:_ This calculation determines the smallest integer order N required for an analog Butterworth filter to meet specific passband and stopband attenuation constraints. Given a maximum passband ripple of 1 dB at the passband edge frequency 1 kHz Hz and a minimum stopband attenuation of 40 dB at the stopband edge frequency 2 kHz Hz, the resulting minimum filter order is 8. This ensures the filter provides a maximally flat magnitude response in the passband while achieving the necessary signal rejection in the stopband.

### Minimum order of a analog Chebyshev type I filter

The smallest order meeting a passband edge with at most Rp dB attenuation and a stopband edge with at least As dB.

Command line:

```console
$ signal-processing solve chebyshev1_order_analog "fp=1 kHz" "fstop=2 kHz" Rp=1 As=40
order = 5
type = lowpass
  by: Minimum order of a analog Chebyshev type I filter  (N = ceil( acosh(sqrt((10^(As/10)-1)/(10^(Rp/10)-1))) / acosh(ws/wp) ))
  fp = 1000 Hz
  fstop = 2000 Hz
  Rp = 1
  As = 40
  assumes: 0 < Rp < As; fp != fstop
```

Python:

```python
import signal_processing_ai999x as m

a = m.solve("chebyshev1_order_analog", fp="1 kHz", fstop="2 kHz", Rp="1", As="40")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/signal-processing-ai999x/#cap-chebyshev1_order_analog) and press **Compute in your browser**.

_What this means:_ This calculation determines the smallest integer order required for an analog Chebyshev Type I filter to meet specific attenuation constraints. By analyzing the passband edge frequency of 1 kHz, the stopband edge frequency of 2 kHz, the maximum passband ripple of 1, and the minimum stopband attenuation of 40, the algorithm computes that a filter of order 5 is necessary. This result indicates the complexity needed to achieve the desired transition band sharpness while maintaining the specified ripple characteristics, identifying the filter type as lowpass.

### Gain of a parabolic dish (aperture) antenna

The gain eta (pi D / lambda)^2 of a circular aperture antenna of diameter D and aperture efficiency eta, in dBi, and its effective aperture.

Command line:

```console
$ signal-processing solve dish_antenna_gain "D=3 m" "f=10 GHz" eta=0.55
G_dbi = 47.35263538
G = 54358.00866
Ae = 3.887720909 m^2
  by: Gain of a parabolic dish (aperture) antenna  (G = eta (pi D / lambda)^2 = 4 pi Ae / lambda^2)
  D = 3 m
  f = 1e+10 Hz
  eta = 0.55
  assumes: 0 < eta <= 1; D >= one wavelength
```

Python:

```python
import signal_processing_ai999x as m

a = m.solve("dish_antenna_gain", D="3 m", f="10 GHz", eta="0.55")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/signal-processing-ai999x/#cap-dish_antenna_gain) and press **Compute in your browser**.

_What this means:_ This calculation determines the directional gain of a parabolic dish antenna using its diameter and operating frequency. Given a dish diameter of 3 m and a frequency of 10 GHz, combined with an aperture efficiency of 0.55, the resulting gain expressed as a ratio is 54358. This translates to 47.3526 in decibels relative to an isotropic source. The effective aperture, which represents the area that captures power from the incoming wave, is calculated as 3.88772 m^2. Higher gain values indicate a narrower beamwidth and greater ability to focus energy in a specific direction.

## Documentation

- **Rust user guide** — the command line, every capability with a command and what it printed: [PDF](https://huggingface.co/AI999X/signal-processing-ai999x/blob/main/docs/signal_processing_ai999x_rust_guide.pdf) · [Markdown](https://huggingface.co/AI999X/signal-processing-ai999x/blob/main/docs/signal_processing_ai999x_rust_guide.md)
- **Python guide** — every capability with a Python example: [PDF](https://huggingface.co/AI999X/signal-processing-ai999x/blob/main/docs/signal_processing_ai999x_capabilities.pdf) · [Markdown](https://huggingface.co/AI999X/signal-processing-ai999x/blob/main/docs/signal_processing_ai999x_capabilities.md)
- **Desktop app & command line** — downloads, install, ask and search, figures, offline use, checksums: https://www.ai999x.com/desktop-cli/
- **Model page** — compute in your browser, every capability explained: https://www.ai999x.com/models/signal-processing-ai999x/

## Highlights

- **Verified, not generated.** Every answer is computed by a certified signal processing capability and checked (substituted back, dimension-checked or compared with an independent computation) before it is returned.
- **Zero wrong answers by design.** Seven explicit outcomes — SOLVED, NEEDS_INPUT, AMBIGUOUS, INVALID_INPUT, OUT_OF_SCOPE, UNRECOGNIZED_FORMAT, INTERNAL_ERROR — never a silent guess. Every one of its 334 documented answers is reproduced exactly on every tested platform.
- **Plain-English questions and a guaranteed typed interface.** `ask()` reads documented question forms; `solve(capability_id, **inputs)` always solves a supported task with valid inputs.
- **Explains itself.** Steps, formulas in LaTeX, units, assumptions and the model used, ready for notebooks, tutoring apps and homework checkers.
- **Native and in the browser.** Every capability also computes natively in Rust: the desktop app and the `signal-processing` command line for Windows and Linux, and WebAssembly on the model page — no Python, no network.
- **Small and portable.** Pure Python (sympy, mpmath); identical answers on Windows and Linux (Python 3.10, 3.11, 3.12, 3.13, 3.14).
- **Coverage:** sampling, Fourier series and transforms, the DFT and FFT, filters, convolution, z-transforms, modulation, communication links, RF transmission lines and radar.

## Model overview

| Property | Value |
|---|---|
| Package | `signal-processing-ai999x` 1.1.0 (import `signal_processing_ai999x`) |
| Type | Deterministic neuro-symbolic signal processing engine (rule-based reading + certified symbolic capabilities) |
| Capabilities | 74 certified procedures |
| Knowledge pack | version 1.0.0 |
| Golden answers | 334 documented answers, identical on every tested platform |
| Platforms tested | Windows (Python 3.13), Linux x86-64 (Python 3.10, 3.11, 3.12, 3.13, 3.14) |
| Runtime | CPU only, offline; no GPU, no network, no API key |
| License | Apache-2.0 |
| Creator / engineer | Ibnul Jaif Farabi |
| Organization | AI999X |

## Python quick start

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

The Python package also installs a Python command, `signal-processing-ai999x "<question>"`; for the native program use `signal-processing` (above). The outputs above were produced by running this exact code on signal-processing-ai999x 1.1.0.

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

## Python worked examples, with their steps

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
@software{signal_processing_ai999x_1_1_0,
  title  = {signal-processing-ai999x: an offline, verified signal processing solver},
  author = {Farabi, Ibnul Jaif}, organization = {AI999X},
  year   = {2026}, version = {1.1.0},
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
