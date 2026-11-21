# Signal Processing (signal-processing-ai999x)

[![tests](https://github.com/AI999X/signal-processing-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/signal-processing-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/signal-processing-ai999x)](https://pypi.org/project/signal-processing-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/signal-processing-ai999x)

Signal Processing covers sampling, Fourier series and transforms, the DFT and FFT, filters, convolution, z-transforms, modulation, communication links, RF transmission lines and radar: 77 calculations in 18 topics, each with its formula, its steps and its units, and a plain reason instead of a number when an input is missing or out of range. One compiled engine, four ways to use it: the desktop app, the command line, the browser and Python. Everything runs on your own computer, offline.

[![The Signal Processing desktop app: Nyquist rate, with its worked example and live scene](https://www.ai999x.com/?ax_shot=signal-processing-ai999x/hero.webp)](https://www.ai999x.com/models/signal-processing-ai999x/)

## Four ways to use it

| | |
|---|---|
| **Desktop app** (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-signal-processing-1.6.5-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-signal-processing-1.6.5-linux-x86_64.tar.gz) |
| **Command line** `signal-processing` (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-cli-1.6.4-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-cli-1.6.4-linux-x86_64.tar.gz) |
| **Browser** (any device) | [Try it on the model page](https://www.ai999x.com/models/signal-processing-ai999x/#capabilities) |
| **Python** 3.10+ | `pip install signal-processing-ai999x`: on Windows and Linux (x86-64) `pip` installs the same compiled engine; on other systems (macOS, ARM) it installs a pure-Python build that gives the same answers |

## See it

From the Signal Processing desktop app: every scene and chart is drawn from the computed answer.

[![Signal Processing desktop app: a slider moves an input and the answer, the scene and the charts follow](https://www.ai999x.com/?ax_shot=signal-processing-ai999x/clip-loop.webp)](https://www.ai999x.com/?ax_shot=signal-processing-ai999x/clip.mp4)

*[Watch the full clip (MP4)](https://www.ai999x.com/?ax_shot=signal-processing-ai999x/clip.mp4)*

| | |
|---|---|
| [![Nyquist rate: the live scene in the Signal Processing desktop app](https://www.ai999x.com/?ax_shot=signal-processing-ai999x/scene-1.webp)](https://www.ai999x.com/models/signal-processing-ai999x/#cap-nyquist_rate)<br>Nyquist rate | [![DFT spectrum of a sequence (FFT): the live scene in the Signal Processing desktop app](https://www.ai999x.com/?ax_shot=signal-processing-ai999x/scene-2.webp)](https://www.ai999x.com/models/signal-processing-ai999x/#cap-dft_spectrum)<br>DFT spectrum of a sequence (FFT) |
| [![Bit error rate of BPSK (and coherent BFSK) in AWGN: the live scene in the Signal Processing desktop app](https://www.ai999x.com/?ax_shot=signal-processing-ai999x/scene-3.webp)](https://www.ai999x.com/models/signal-processing-ai999x/#cap-bpsk_ber)<br>Bit error rate of BPSK (and coherent BFSK) in AWGN | [![Reflection coefficient, VSWR and return loss of a load: the live scene in the Signal Processing desktop app](https://www.ai999x.com/?ax_shot=signal-processing-ai999x/scene-4.webp)](https://www.ai999x.com/models/signal-processing-ai999x/#cap-reflection_coefficient)<br>Reflection coefficient, VSWR and return loss of a load |
| [![Digital Butterworth filter design (bilinear transform): the live scene in the Signal Processing desktop app](https://www.ai999x.com/?ax_shot=signal-processing-ai999x/scene-5.webp)](https://www.ai999x.com/models/signal-processing-ai999x/#cap-butterworth_design)<br>Digital Butterworth filter design (bilinear transform) | [![Magnitude of an analog Butterworth low-pass response: the live scene in the Signal Processing desktop app](https://www.ai999x.com/?ax_shot=signal-processing-ai999x/scene-6.webp)](https://www.ai999x.com/models/signal-processing-ai999x/#cap-butterworth_magnitude)<br>Magnitude of an analog Butterworth low-pass response |
| [![Gain of a parabolic dish (aperture) antenna: the live scene in the Signal Processing desktop app](https://www.ai999x.com/?ax_shot=signal-processing-ai999x/scene-7.webp)](https://www.ai999x.com/models/signal-processing-ai999x/#cap-dish_antenna_gain)<br>Gain of a parabolic dish (aperture) antenna | [![Shannon capacity of an AWGN channel: the live scene in the Signal Processing desktop app](https://www.ai999x.com/?ax_shot=signal-processing-ai999x/scene-8.webp)](https://www.ai999x.com/models/signal-processing-ai999x/#cap-shannon_capacity)<br>Shannon capacity of an AWGN channel |

Charts:

|   |   |   |   |
|---|---|---|---|
| [![Nyquist rate: a chart in the Signal Processing desktop app](https://www.ai999x.com/?ax_shot=signal-processing-ai999x/chart-1.webp)](https://www.ai999x.com/models/signal-processing-ai999x/#cap-nyquist_rate)<br>Nyquist rate | [![Bit error rate of BPSK (and coherent BFSK) in AWGN: a chart in the Signal Processing desktop app](https://www.ai999x.com/?ax_shot=signal-processing-ai999x/chart-2.webp)](https://www.ai999x.com/models/signal-processing-ai999x/#cap-bpsk_ber)<br>Bit error rate of BPSK (and coherent BFSK) in AWGN | [![Reflection coefficient, VSWR and return loss of a load: a chart in the Signal Processing desktop app](https://www.ai999x.com/?ax_shot=signal-processing-ai999x/chart-3.webp)](https://www.ai999x.com/models/signal-processing-ai999x/#cap-reflection_coefficient)<br>Reflection coefficient, VSWR and return loss of a load | [![Magnitude of an analog Butterworth low-pass response: a chart in the Signal Processing desktop app](https://www.ai999x.com/?ax_shot=signal-processing-ai999x/chart-4.webp)](https://www.ai999x.com/models/signal-processing-ai999x/#cap-butterworth_magnitude)<br>Magnitude of an analog Butterworth low-pass response |

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

## Command line: 15 examples

Each command below was run with `signal-processing` of this release; the lines under it are the answer it printed (the full answer goes on with the method, every input in SI units and the conditions it assumes).

```text
$ signal-processing solve butterworth_magnitude N=4 "fc=1 kHz" "f=2 kHz"
magnitude = 0.06237828616
attenuation_db = 24.09933123

$ signal-processing solve dish_antenna_gain "D=3 m" "f=10 GHz" eta=0.55
G_dbi = 47.35263538
G = 54358.00866
Ae = 3.887720909 m^2

$ signal-processing solve shannon_min_snr "B=2 MHz" C=10000000
snr_db = 14.91361694
snr = 31
ebn0_min_db = 7.923916895

$ signal-processing solve circular_convolution "x=[1, 2, 3, 4]" "h=[1, 0, 0, 1]"
y = [3.0,5.0,7.0,5.0]

$ signal-processing solve dft_spectrum "x=[1, 2, 3, 4]"
peak_magnitude = 10
peak_bin = 0
magnitude = [10.0,2.8284271247461903,2.0,2.8284271247461903]
phase = [0.0,2.356194490192345,3.141592653589793,-2.356194490192345]
real = [10.0,-2.0,-2.0,-2.0]
imag = [0.0,2.0,0.0,-2.0]

$ signal-processing solve amplitude_ratio_to_db ratio=50
dB = 33.97940009

$ signal-processing solve qpsk_error_rates ebn0_db=6
ber = 0.002388290781
ser = 0.004770877629

$ signal-processing solve bilinear_rc_lowpass "fs=8 kHz" "fc=1 kHz"
b0 = 0.2819698001
b1 = 0.2819698001
a1 = -0.4360603998

$ signal-processing solve butterworth_design btype=lowpass N=2 "fs=8 kHz" "fc=1 kHz"
b = [0.0976310729378,0.195262145876,0.0976310729378]
a = [1.0,-0.942809041582,0.333333333333]

$ signal-processing solve analytic_envelope "x=[1, 0, -1, 0]"
envelope = [1.0,1.0,1.0,1.0]
hilbert = [0.0,1.0,0.0,-1.0]

$ signal-processing solve am_modulation_index "Vmax=10 V" "Vmin=2 V"
m = 0.6666666667
Ac = 6 V

$ signal-processing solve free_space_path_loss "d=100 m" "f=2.4 GHz"
loss_db = 80.05200806
loss = 101204728.8
wavelength = 0.1249135242 m

$ signal-processing solve radar_doppler "v=30 m/s" "f=10 GHz"
fd = 2001.384771 Hz
wavelength = 0.0299792458 m

$ signal-processing solve alias_frequency "fs=10 kHz" "f=7 kHz"
f_alias = 3000 Hz

$ signal-processing solve coax_impedance "D=3.5 mm" "d=1 mm" er=2.25
Z0 = 50.07585194 ohm
vf = 0.6666666667
```

## Rust library: 15 examples

The crate `signal-processing-ai999x` (in `rust/` of the GitHub repository) answers a JSON request with `signal_processing_ai999x::solve_json`. Each call below was compiled and run on this release; the comment under it is what it printed. The whole program is `rust/signal-processing/examples/readme.rs`: `cd rust && cargo run --release --example readme -p signal-processing-ai999x`.

```rust
use serde_json::{json, Value};

fn show(request: Value) {
    let answer: Value = serde_json::from_str(&signal_processing_ai999x::solve_json(&request.to_string())).unwrap();
    println!("{}", answer["outcome"].as_str().unwrap_or(""));
    for v in answer["values"].as_array().into_iter().flatten() {
        let unit = v[2].as_str().unwrap_or("");
        let unit = if unit.is_empty() || unit == "1" { String::new() } else { format!(" {unit}") };
        println!("{} = {}{}", v[0].as_str().unwrap_or(""), v[1], unit);
    }
}

show(json!({"key": "butterworth_magnitude", "inputs": {"N": "4", "fc": "1 kHz", "f": "2 kHz"}}));
// SOLVED
// magnitude = 0.06237828615518053
// attenuation_db = 24.099331233312945

show(json!({"key": "dish_antenna_gain", "inputs": {"D": "3 m", "f": "10 GHz", "eta": "0.55"}}));
// SOLVED
// G_dbi = 47.352635384659806
// G = 54358.00865599061
// Ae = 3.8877209088173696 m^2

show(json!({"key": "shannon_min_snr", "inputs": {"B": "2 MHz", "C": "10000000"}}));
// SOLVED
// snr_db = 14.913616938342727
// snr = 31.0
// ebn0_min_db = 7.923916894982539

show(json!({"key": "circular_convolution", "inputs": {"x": [1, 2, 3, 4], "h": [1, 0, 0, 1]}}));
// SOLVED
// y = [3.0,5.0,7.0,5.0]

show(json!({"key": "dft_spectrum", "inputs": {"x": [1, 2, 3, 4]}}));
// SOLVED
// peak_magnitude = 10.0
// peak_bin = 0
// magnitude = [10.0,2.8284271247461903,2.0,2.8284271247461903]
// phase = [0.0,2.356194490192345,3.141592653589793,-2.356194490192345]
// real = [10.0,-2.0,-2.0,-2.0]
// imag = [0.0,2.0,0.0,-2.0]

show(json!({"key": "amplitude_ratio_to_db", "inputs": {"ratio": "50"}}));
// SOLVED
// dB = 33.979400086720375

show(json!({"key": "qpsk_error_rates", "inputs": {"ebn0_db": "6"}}));
// SOLVED
// ber = 0.0023882907809328075
// ser = 0.004770877629011326

show(json!({"key": "bilinear_rc_lowpass", "inputs": {"fs": "8 kHz", "fc": "1 kHz"}}));
// SOLVED
// b0 = 0.2819698001234662
// b1 = 0.2819698001234662
// a1 = -0.4360603997530677

show(json!({"key": "butterworth_design", "inputs": {"btype": "lowpass", "N": "2", "fs": "8 kHz", "fc": "1 kHz"}}));
// SOLVED
// b = [0.0976310729378,0.195262145876,0.0976310729378]
// a = [1.0,-0.942809041582,0.333333333333]

show(json!({"key": "analytic_envelope", "inputs": {"x": [1, 0, -1, 0]}}));
// SOLVED
// envelope = [1.0,1.0,1.0,1.0]
// hilbert = [0.0,1.0,0.0,-1.0]

show(json!({"key": "am_modulation_index", "inputs": {"Vmax": "10 V", "Vmin": "2 V"}}));
// SOLVED
// m = 0.6666666666666666
// Ac = 6.0 V

show(json!({"key": "free_space_path_loss", "inputs": {"d": "100 m", "f": "2.4 GHz"}}));
// SOLVED
// loss_db = 80.0520080561155
// loss = 101204728.84315346
// wavelength = 0.12491352416666666 m

show(json!({"key": "radar_doppler", "inputs": {"v": "30 m/s", "f": "10 GHz"}}));
// SOLVED
// fd = 2001.3847714659423 Hz
// wavelength = 0.0299792458 m

show(json!({"key": "alias_frequency", "inputs": {"fs": "10 kHz", "f": "7 kHz"}}));
// SOLVED
// f_alias = 3000.0 Hz

show(json!({"key": "coax_impedance", "inputs": {"D": "3.5 mm", "d": "1 mm", "er": "2.25"}}));
// SOLVED
// Z0 = 50.07585194223886 ohm
// vf = 0.6666666666666666
```

## Guides

- Desktop app guide: the window, every calculation, scenes and charts: [PDF](https://huggingface.co/AI999X/signal-processing-ai999x/blob/main/docs/signal_processing_ai999x_desktop_guide.pdf) · [HTML](https://huggingface.co/AI999X/signal-processing-ai999x/blob/main/docs/signal_processing_ai999x_desktop_guide.html) · [Markdown](https://huggingface.co/AI999X/signal-processing-ai999x/blob/main/docs/signal_processing_ai999x_desktop_guide.md)
- Python guide: every calculation with a Python example: [PDF](https://huggingface.co/AI999X/signal-processing-ai999x/blob/main/docs/signal_processing_ai999x_capabilities.pdf) · [HTML](https://huggingface.co/AI999X/signal-processing-ai999x/blob/main/docs/signal_processing_ai999x_capabilities.html) · [Markdown](https://huggingface.co/AI999X/signal-processing-ai999x/blob/main/docs/signal_processing_ai999x_capabilities.md)
- Command-line guide: every calculation with the command and what it printed: [PDF](https://huggingface.co/AI999X/signal-processing-ai999x/blob/main/docs/signal_processing_ai999x_rust_guide.pdf) · [HTML](https://huggingface.co/AI999X/signal-processing-ai999x/blob/main/docs/signal_processing_ai999x_rust_guide.html) · [Markdown](https://huggingface.co/AI999X/signal-processing-ai999x/blob/main/docs/signal_processing_ai999x_rust_guide.md)

Every calculation, with its formula, a worked example and its figures: **[Signal Processing on ai999x.com](https://www.ai999x.com/models/signal-processing-ai999x/)**. Install guides for the app and the command line: [https://www.ai999x.com/desktop-cli/](https://www.ai999x.com/desktop-cli/).

Version 1.6.4 · License: Apache-2.0 · Author: Ibnul Jaif Farabi · Organization: AI999X · Contact: support@ai999x.com · [Support the project](https://buymeacoffee.com/ibnuljaiffarabi)

*The Future Is Self-Improving.*

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/signal-processing-ai999x/) (`pip install .` builds it). Releases are tagged `v1.6.6`; the [GitHub release](https://github.com/AI999X/signal-processing-ai999x/releases/tag/v1.6.6) carries the wheel, the desktop app and the command-line program for Windows and Linux, and their checksums.

## Build the program from source

`rust/` holds the Rust source of the `signal-processing` command-line program and browser module: the AI999X engine (`rust/ax-engine`), this model (`rust/signal-processing`) and its data (`rust/models/signal_processing/pack.json`). With Rust 1.80 or later:

```bash
cd rust
cargo build --release -p signal-processing-ai999x
./target/release/signal-processing --help
cargo test -p signal-processing-ai999x
```
