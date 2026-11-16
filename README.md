# Signal Processing (signal-processing-ai999x)

[![tests](https://github.com/AI999X/signal-processing-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/signal-processing-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/signal-processing-ai999x)](https://pypi.org/project/signal-processing-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/signal-processing-ai999x)

Signal Processing covers sampling, Fourier series and transforms, the DFT and FFT, filters, convolution, z-transforms, modulation, communication links, RF transmission lines and radar: 77 calculations in 18 topics, each with its formula, its steps and its units, and a plain reason instead of a number when an input is missing or out of range. One compiled engine, four ways to use it: the desktop app, the command line, the browser and Python. Everything runs on your own computer, offline.

[![The Signal Processing desktop app: Power in watts to dBm and dBW, with its worked example and live scene](https://www.ai999x.com/?ax_shot=signal-processing-ai999x/hero.webp)](https://www.ai999x.com/models/signal-processing-ai999x/)

## Four ways to use it

| | |
|---|---|
| **Desktop app** (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-1.6.4-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-1.6.4-linux-x86_64.tar.gz) |
| **Command line** `signal-processing` (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-cli-1.6.4-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-cli-1.6.4-linux-x86_64.tar.gz) |
| **Browser** (any device) | [Try it on the model page](https://www.ai999x.com/models/signal-processing-ai999x/#try) |
| **Python** 3.10+ | `pip install signal-processing-ai999x`: on Windows and Linux (x86-64) `pip` installs the same compiled engine; on other systems (macOS, ARM) it installs a pure-Python build that gives the same answers |

## Ask in words: 15 examples

Each question below was asked on this release; the comment under it is the line it printed.

```python
import signal_processing_ai999x as signal_processing

a = signal_processing.ask("Express a voltage ratio of 50 in decibels.")
print(a.outcome, a.values)
# SOLVED {'dB': 33.979400086720375}

a = signal_processing.ask(
    "What are the bit and symbol error rates of QPSK at Eb/N0 = 6 dB?"
)
print(a.outcome, a.values)
# SOLVED {'ber': 0.002388290780932807, 'ser': 0.004770877629011325}

a = signal_processing.ask(
    "An AM envelope has a maximum of 10 V and a minimum of 2 V. What "
    "is the modulation index?"
)
print(a.outcome, a.values)
# SOLVED {'m': 0.6666666666666666, 'Ac': 6.0}

a = signal_processing.ask(
    "A 10 GHz radar sees a target approaching at 30 m/s. What is the "
    "Doppler shift?"
)
print(a.outcome, a.values)
# SOLVED {'fd': 2001.3847714659423, 'wavelength': 0.0299792458}

a = signal_processing.ask(
    "A 7 kHz tone is sampled at 10 kHz. What is the alias frequency?"
)
print(a.outcome, a.values)
# SOLVED {'f_alias': 3000.0}

a = signal_processing.ask(
    "What is the characteristic impedance of a coaxial cable with an "
    "inner conductor diameter of 1 mm, an outer conductor diameter of "
    "3.5 mm and a dielectric constant of 2.25?"
)
print(a.outcome, a.values)
# SOLVED {'Z0': 50.07585194223886, 'vf': 0.6666666666666666}

a = signal_processing.ask(
    "What Butterworth filter order is needed for a passband edge of 1 "
    "kHz with at most 1 dB ripple and a stopband edge of 2 kHz with at "
    "least 40 dB attenuation?"
)
print(a.outcome, a.values)
# SOLVED {'order': 8, 'type': 'lowpass'}

a = signal_processing.ask(
    "What is the Shannon capacity of a 3 kHz channel with an SNR of 30 "
    "dB?"
)
print(a.outcome, a.values)
# SOLVED {'C': 29901.67877650798, 'efficiency': 9.967226258835993}

a = signal_processing.ask(
    "What frequency does bin 100 of a 1024-point FFT correspond to at "
    "a sampling rate of 48 kHz?"
)
print(a.outcome, a.values)
# SOLVED {'f_k': 4687.5}

a = signal_processing.ask("What voltage ratio corresponds to 6 dB?")
print(a.outcome, a.values)
# SOLVED {'ratio': 1.9952623149688795}

a = signal_processing.ask(
    "What is the bit error rate of BPSK at Eb/N0 = 8 dB?"
)
print(a.outcome, a.values)
# SOLVED {'ber': 0.00019090777407599314, 'ber_bfsk': 0.006004386400163562}

a = signal_processing.ask(
    "What is the DC gain of the filter with b = [1, 2, 1] and a = [1, "
    "-0.5, 0.25]?"
)
print(a.outcome, a.values)
# SOLVED {'gain': 5.333333333333333, 'gain_db': 14.539974558725246}

a = signal_processing.ask(
    "What is the Fourier transform of a decaying exponential with "
    "amplitude 2 and time constant 0.5 s at 0 Hz?"
)
print(a.outcome, a.values)
# SOLVED {'magnitude': 1.0, 'phase': 0.0, 're': 1.0, 'im': 0.0}

a = signal_processing.ask(
    "What is the maximum range of a radar with a peak power of 1 MW, "
    "an antenna gain of 40 dBi, a frequency of 3 GHz, a radar "
    "cross-section of 1 m^2 and a minimum detectable signal of 1e-13 "
    "W?"
)
print(a.outcome, a.values)
# SOLVED {'Rmax': 266343.6371351149}

a = signal_processing.ask(
    "What is the frequency of a signal with a period of 2 ms?"
)
print(a.outcome, a.values)
# SOLVED {'f': 500.0, 'omega': 3141.592653589793}
```

## Screenshots

[![The Signal Processing desktop app: Power in watts to dBm and dBW, with its worked example and live scene](https://www.ai999x.com/?ax_shot=signal-processing-ai999x/hero-dark.webp)](https://www.ai999x.com/models/signal-processing-ai999x/)

[![Frequency and angular frequency from period: the live scene in the Signal Processing desktop app](https://www.ai999x.com/?ax_shot=signal-processing-ai999x/see-1.webp)](https://www.ai999x.com/models/signal-processing-ai999x/)

*Frequency and angular frequency from period*

[![AM modulation index from the envelope: the live scene in the Signal Processing desktop app](https://www.ai999x.com/?ax_shot=signal-processing-ai999x/see-2.webp)](https://www.ai999x.com/models/signal-processing-ai999x/)

*AM modulation index from the envelope*

[![Minimum SNR for a data rate (Shannon limit): the live scene in the Signal Processing desktop app](https://www.ai999x.com/?ax_shot=signal-processing-ai999x/see-3.webp)](https://www.ai999x.com/models/signal-processing-ai999x/)

*Minimum SNR for a data rate (Shannon limit)*

Every calculation, with its formula, a worked example and its figures: **[Signal Processing on ai999x.com](https://www.ai999x.com/models/signal-processing-ai999x/)**. Install guides for the app and the command line: [https://www.ai999x.com/desktop-cli/](https://www.ai999x.com/desktop-cli/).

Version 1.6.4 · License: Apache-2.0 · Author: Ibnul Jaif Farabi · Organization: AI999X · Contact: support@ai999x.com · [Support the project](https://buymeacoffee.com/ibnuljaiffarabi)

*The Future Is Self-Improving.*

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/signal-processing-ai999x/) (`pip install .` builds it). Releases are tagged `v1.6.4`; the [GitHub release](https://github.com/AI999X/signal-processing-ai999x/releases/tag/v1.6.4) carries the wheel, the desktop app and the command-line program for Windows and Linux, and their checksums.

## Build the program from source

`rust/` holds the Rust source of the `signal-processing` command-line program and browser module: the AI999X engine (`rust/ax-engine`), this model (`rust/signal-processing`) and its data (`rust/models/signal_processing/pack.json`). With Rust 1.80 or later:

```bash
cd rust
cargo build --release -p signal-processing-ai999x
./target/release/signal-processing --help
cargo test -p signal-processing-ai999x
```
