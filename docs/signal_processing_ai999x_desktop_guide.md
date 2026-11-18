# Signal Processing: the desktop app, user guide

**Signal Processing**, desktop app 1.6.5 for Windows and Linux. **Creator / engineer:** Ibnul Jaif Farabi · **Company:** AI999X · support@ai999x.com

Signal Processing covers sampling, Fourier series and transforms, the DFT and FFT, filters, convolution, z-transforms, modulation, communication links, RF transmission lines and radar. Its desktop app holds all 77 calculations in 18 topics: pick one, change its inputs and the answer, a live scene and charts follow at once. It runs on your own computer with no network connection and nothing else to install. The pictures in this guide were taken in this app.

## Contents

- 1. Get it and start it
- 2. The window
- 3. Compute a calculation
- 4. Ask in words
- 5. The live scene and the charts
- 6. Light and dark
- 7. Every calculation
- 8. Other ways to use it

## 1. Get it and start it

| system | download |
|---|---|
| Windows x86-64 | https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-signal-processing-1.6.5-windows-x86_64.zip |
| Linux x86-64 | https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-signal-processing-1.6.5-linux-x86_64.tar.gz |

Windows: Download ai999x-signal-processing-1.6.5-windows-x86_64.zip, right-click it → Extract All, then open the folder and start ai999x-signal-processing.exe. If Windows SmartScreen asks, choose More info → Run anyway.

Linux:

```shell
curl -LO https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-signal-processing-1.6.5-linux-x86_64.tar.gz
curl -LO https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-signal-processing-1.6.5-linux-x86_64.tar.gz
./ai999x-signal-processing-1.6.5-linux-x86_64/ai999x-signal-processing
```

Checksums of every download: https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/SHA256SUMS. Install steps for every app: https://www.ai999x.com/desktop-cli/

## 2. The window

![The Signal Processing app, opened on Nyquist rate with its worked example.](https://www.ai999x.com/?ax_shot=signal-processing-ai999x/hero.webp)

*The Signal Processing app, opened on Nyquist rate with its worked example.*

On the left, a search box and the topics with their calculations. In the middle, the chosen calculation: its answer at the top, then the live scene (or its charts), what the answer means and the formula. On the right, its inputs, each with a slider where that makes sense, and the Compute and Reset buttons. At the bottom, a box for a question in words.

## 3. Compute a calculation

Open Sampling and signal basics › Nyquist rate (or type part of its name in the search box). It opens with a worked example filled in: “What is the Nyquist rate for a signal with a maximum frequency of 4 kHz?” The answer: fs_min = 8000 Hz, Ts_max = 0.000125 s.

Change any input (type a number with its unit, or drag its slider) and the answer, the scene and the charts follow; Compute recomputes after typing, Reset brings back the worked example. An input outside the range the formula holds for is refused with the reason, never answered with a guessed number.

![Nyquist rate: the live scene of the worked example.](https://www.ai999x.com/?ax_shot=signal-processing-ai999x/scene-1.webp)

*Nyquist rate: the live scene of the worked example.*

## 4. Ask in words

Type a question as you would say it, for example “What is the Nyquist rate for a signal with a maximum frequency of 4 kHz?” The app reads the quantities and their units, picks the calculation and answers; the command-line program reads it the same way and printed:

```text
Read as: Nyquist rate
fs_min = 8000 Hz
Ts_max = 0.000125 s
```

When a question names a quantity the app cannot place, it says what is missing instead of guessing.

## 5. The live scene and the charts

Every calculation draws a scene from its answer: it moves as an input changes. The Charts button shows charts that vary one input around your values; Enlarge opens one in a large view. Scenes from other topics:

![DFT spectrum of a sequence (FFT)](https://www.ai999x.com/?ax_shot=signal-processing-ai999x/scene-2.webp)

*DFT spectrum of a sequence (FFT)*

![Bit error rate of BPSK (and coherent BFSK) in AWGN](https://www.ai999x.com/?ax_shot=signal-processing-ai999x/scene-3.webp)

*Bit error rate of BPSK (and coherent BFSK) in AWGN*

![Reflection coefficient, VSWR and return loss of a load](https://www.ai999x.com/?ax_shot=signal-processing-ai999x/scene-4.webp)

*Reflection coefficient, VSWR and return loss of a load*

![Digital Butterworth filter design (bilinear transform)](https://www.ai999x.com/?ax_shot=signal-processing-ai999x/scene-5.webp)

*Digital Butterworth filter design (bilinear transform)*

![Magnitude of an analog Butterworth low-pass response](https://www.ai999x.com/?ax_shot=signal-processing-ai999x/scene-6.webp)

*Magnitude of an analog Butterworth low-pass response*

![Gain of a parabolic dish (aperture) antenna](https://www.ai999x.com/?ax_shot=signal-processing-ai999x/scene-7.webp)

*Gain of a parabolic dish (aperture) antenna*

![Shannon capacity of an AWGN channel](https://www.ai999x.com/?ax_shot=signal-processing-ai999x/scene-8.webp)

*Shannon capacity of an AWGN channel*

![Nyquist rate: a chart, enlarged.](https://www.ai999x.com/?ax_shot=signal-processing-ai999x/chart-1.webp)

*Nyquist rate: a chart, enlarged.*

![Bit error rate of BPSK (and coherent BFSK) in AWGN: a chart, enlarged.](https://www.ai999x.com/?ax_shot=signal-processing-ai999x/chart-2.webp)

*Bit error rate of BPSK (and coherent BFSK) in AWGN: a chart, enlarged.*

## 6. Light and dark

The app follows a light or a dark theme (the switch is in its top bar).

![The Signal Processing app in the dark theme.](https://www.ai999x.com/?ax_shot=signal-processing-ai999x/hero-dark.webp)

*The Signal Processing app in the dark theme.*

## 7. Every calculation

77 calculations in 18 topics; each has its own entry on the model's page (https://www.ai999x.com/models/signal-processing-ai999x/) with its formula, worked example and charts.

| topic | calculations |
|---|---|
| Analog filters | Magnitude of an analog Butterworth low-pass response, Minimum order of a analog Butterworth filter, Minimum order of a analog Chebyshev type I filter |
| Antennas | Gain of a parabolic dish (aperture) antenna |
| Communication links | Minimum SNR for a data rate (Shannon limit), Shannon capacity of an AWGN channel |
| Convolution and correlation | Circular (cyclic) convolution, Cross-correlation and the lag of its peak, Linear convolution of two sequences |
| DFT and FFT | DFT spectrum of a sequence (FFT), Frequency of a DFT bin, Frequency resolution of an N-point DFT, Inverse discrete Fourier transform, One bin of the discrete Fourier transform |
| Decibels and levels | Amplitude (voltage) ratio to decibels, Decibels to amplitude (voltage) ratio, Decibels to power ratio, Power in watts to dBm and dBW, Power level in dBm to watts, Power ratio to decibels, Signal-to-noise ratio from signal and noise powers, Total power of several signals given in dBm |
| Digital communication | Bit and symbol error rates of QPSK in AWGN, Bit error rate of BPSK (and coherent BFSK) in AWGN, Bit error rates of DPSK and noncoherent BFSK, Symbol error rate of M-PSK in AWGN, Symbol error rate of square M-QAM in AWGN |
| Digital filters | Bilinear transform of a first-order low-pass filter, DC gain of a digital filter, FIR filter output, Frequency response of a digital filter at one frequency, Group delay of a digital filter, IIR filter output (difference equation), Impulse response of a digital filter, Minimum order of a digital (bilinear) Butterworth filter, Moving-average filter output, Step response of a digital filter |
| Filter design | Digital Butterworth filter design (bilinear transform), FIR low-pass filter design by the window method, Second-order IIR notch filter design |
| Fourier series and transforms | Analytic signal, Hilbert transform and envelope, Fourier transform of a decaying exponential, Fourier transform of a rectangular pulse, Harmonic amplitude of a square, triangle or sawtooth wave, Total harmonic distortion |
| Modulation | AM modulation index from the envelope, FM spectrum: Carson bandwidth and Bessel sideband amplitudes, Power in an AM (DSB full-carrier) signal |
| Noise and link budgets | Free-space path loss, Noise figure of cascaded stages (Friis formula), Received power of a radio link (Friis transmission equation) |
| Radar | Doppler shift of a radar echo, Maximum detection range of a radar, Received echo power (radar range equation) |
| Sampling and signal basics | Alias (apparent) frequency after sampling, Frequency and angular frequency from period, Mean, RMS, energy and average power of a sampled signal, Nyquist rate, Period and angular frequency from frequency, RMS, peak-to-peak and rectified average of a sine wave, Signal-to-quantization-noise ratio of an ideal ADC |
| Transmission lines | Characteristic impedance of a coaxial line, Input impedance of a terminated lossless line, Quarter-wave impedance transformer, Reflection coefficient and return loss from the VSWR, Reflection coefficient, VSWR and return loss of a load, VSWR and return loss from the reflection coefficient |
| Wavelets | Haar discrete wavelet transform, Inverse Haar wavelet transform (one level) |
| Windows and spectral analysis | Coefficients of a window function, Main-lobe width of a spectral window, Periodogram (power spectral density estimate) |
| Z-transform and stability | Inverse z-transform by partial fractions, Linear difference equation solved by the z-transform, Poles and stability of a second-order all-pole system, Poles, zeros and stability of a digital filter, z-transform of a sequence (table terms) |

## 8. Other ways to use it

The same engine runs on the command line (`signal-processing`, its own guide), in Python (`pip install signal-processing-ai999x`, `import signal_processing_ai999x`, its own guide) and in the browser on https://www.ai999x.com/models/signal-processing-ai999x/.

Licence: Apache-2.0 · AI999X · https://www.ai999x.com/models/signal-processing-ai999x/
