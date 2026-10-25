"""Signal processing procedures: deterministic algorithms and formulas with typed inputs and several outputs (domain
code, loaded by the pack; enabled only once tools/certify_procedures.py has certified them).

Covers sampling, decibels, Fourier series and transforms, the DFT and FFT, convolution and correlation, digital
filters, z-domain poles, windows, modulation, digital communication error rates, link budgets and noise, RF
transmission lines, antennas and radar.

Each procedure states its inputs, outputs, conditions, method, LaTeX, the documented question templates it parses,
verified worked examples, an INDEPENDENT reference computation (used only by certification, run in the checking
environment: numpy, scipy, PyWavelets, scikit-rf, scikit-dsp-comm — imported inside the reference, never shipped) and
a deliberately wrong variant (certification shows the checks reject it). `run`, `verify` and `parse` are plain Python.
"""
from __future__ import annotations

import cmath
import math
import re

C0 = 299792458.0                     # speed of light in vacuum, m/s (exact)
KB = 1.380649e-23                    # Boltzmann constant, J/K (exact)
T0 = 290.0                           # standard noise reference temperature, K
MU0 = 1.25663706127e-6               # vacuum permeability, N/A^2 (CODATA 2022)
EPS0 = 8.8541878188e-12              # vacuum permittivity, F/m (CODATA 2022)
INF = float("inf")

# =================================================================================================== checks
def _pos(v, name):
    if isinstance(v, bool) or not isinstance(v, (int, float)) or not math.isfinite(v) or v <= 0:
        raise ValueError(f"{name} must be a positive finite number")
    return float(v)


def _nonneg(v, name):
    if isinstance(v, bool) or not isinstance(v, (int, float)) or not math.isfinite(v) or v < 0:
        raise ValueError(f"{name} must be a finite number >= 0")
    return float(v)


def _fin(v, name):
    if isinstance(v, bool) or not isinstance(v, (int, float)) or not math.isfinite(v):
        raise ValueError(f"{name} must be a finite real number")
    return float(v)


def _int(v, name, lo=None, hi=None):
    if isinstance(v, bool) or not isinstance(v, (int, float)) or not math.isfinite(v) or float(v) != int(v):
        raise ValueError(f"{name} must be a whole number")
    v = int(v)
    if lo is not None and v < lo:
        raise ValueError(f"{name} must be at least {lo}")
    if hi is not None and v > hi:
        raise ValueError(f"{name} must be at most {hi}")
    return v


def _vec(v, name, lo=1, hi=None):
    if not isinstance(v, (list, tuple)) or not all(isinstance(x, (int, float)) and not isinstance(x, bool)
                                                   and math.isfinite(x) for x in v):
        raise ValueError(f"{name} must be a list of finite numbers")
    if len(v) < lo:
        raise ValueError(f"{name} needs at least {lo} values")
    if hi is not None and len(v) > hi:
        raise ValueError(f"{name} allows at most {hi} values")
    return [float(x) for x in v]


# =================================================================================================== residuals
def _diff(a, b) -> float:
    """Relative difference of two outputs (numbers, complex numbers, texts, nested lists); 0 when identical."""
    if isinstance(a, str) or isinstance(b, str):
        return 0.0 if a == b else INF
    if isinstance(a, (list, tuple)) or isinstance(b, (list, tuple)):
        if not (isinstance(a, (list, tuple)) and isinstance(b, (list, tuple))) or len(a) != len(b):
            return INF
        return max([_diff(x, y) for x, y in zip(a, b)], default=0.0)
    if a is None or b is None:
        return 0.0 if a is b else INF
    try:
        a, b = complex(a), complex(b)
    except (TypeError, ValueError):
        return INF
    if a == b:
        return 0.0
    d = abs(a - b)
    if d != d:
        return INF
    return d / max(abs(a), abs(b))


def _resid(got: dict, want: dict) -> float:
    worst = 0.0
    for k, w in want.items():
        if k not in got:
            return INF
        worst = max(worst, _diff(got[k], w))
    return worst


def _recheck(run):
    """Residual of a closed-form or deterministic algorithm: the outputs recomputed from the inputs."""
    def verify(inputs, outputs):
        try:
            want = run(**inputs)
        except Exception:
            return INF
        return _resid(outputs, want)
    return verify


# =================================================================================================== reading questions
_N = r"[-+]?(?:\d+(?:\.\d*)?|\.\d+)(?:[eE][-+]?\d+)?"
_LEFT = re.compile(r"(?<![\w.^])(?:\d+(?:\.\d*)?|\.\d+)")
_PREMASK = [r"-?3\s*-?dB(?=\s+(?:cutoff|corner|frequency|point|bandwidth|down))"]

_UNIT_TABLE = {
    "frequency": {"THz": "THz", "GHz": "GHz", "MHz": "MHz", "kHz": "kHz", "Hz": "Hz", "KHz": "kHz", "khz": "kHz",
                  "ghz": "GHz", "GHZ": "GHz", "MHZ": "MHz", "KHZ": "kHz", "hz": "Hz", "gigahertz": "GHz",
                  "megahertz": "MHz", "kilohertz": "kHz", "hertz": "Hz"},
    "time": {"ms": "ms", "us": "us", "µs": "us", "μs": "us", "ns": "ns", "ps": "ps", "s": "s", "seconds": "s",
             "second": "s", "milliseconds": "ms", "millisecond": "ms", "microseconds": "us", "microsecond": "us",
             "nanoseconds": "ns", "nanosecond": "ns", "sec": "s"},
    "power": {"kW": "kW", "MW": "MW", "mW": "mW", "uW": "uW", "µW": "uW", "μW": "uW", "nW": "nW", "pW": "pW",
              "W": "W", "watts": "W", "watt": "W", "milliwatts": "mW", "milliwatt": "mW",
              "kilowatts": "kW", "kilowatt": "kW", "microwatts": "uW", "microwatt": "uW"},
    "length": {"km": "km", "cm": "cm", "mm": "mm", "m": "m", "meters": "m", "meter": "m", "metres": "m", "metre": "m",
               "kilometers": "km", "kilometres": "km", "kilometer": "km", "kilometre": "km"},
    "speed": {"m/s": "m/s", "km/h": "km/h", "km/s": "km/s"},
    "impedance": {"kohms": "kohm", "kohm": "kohm", "kΩ": "kohm", "ohms": "ohm", "ohm": "ohm", "Ω": "ohm"},
    "resistance": {"Mohms": "Mohm", "Mohm": "Mohm", "MΩ": "Mohm", "kohms": "kohm", "kohm": "kohm", "kΩ": "kohm",
                   "ohms": "ohm", "ohm": "ohm", "Ω": "ohm"},
    "capacitance": {"pF": "pF", "nF": "nF", "uF": "uF", "µF": "uF", "μF": "uF", "mF": "mF", "F": "F"},
    "temperature": {"K": "K", "kelvin": "K"},
    "area": {"m^2": "m^2", "m²": "m^2", "m2": "m^2", "square meters": "m^2", "square metres": "m^2", "cm^2": "cm^2"},
    "voltage": {"kV": "kV", "mV": "mV", "uV": "uV", "µV": "uV", "μV": "uV", "V": "V", "volts": "V", "volt": "V"},
}
_RATES = {"bit/s": 1.0, "bits/s": 1.0, "bps": 1.0, "b/s": 1.0, "kbit/s": 1e3, "kbits/s": 1e3, "kbps": 1e3,
          "kb/s": 1e3, "Mbit/s": 1e6, "Mbits/s": 1e6, "Mbps": 1e6, "Mb/s": 1e6, "Gbit/s": 1e9, "Gbits/s": 1e9,
          "Gbps": 1e9, "Gb/s": 1e9, "bits per second": 1.0}


def _alt(words):
    return "(?:" + "|".join(re.escape(w) for w in sorted(words, key=len, reverse=True)) + ")"


_UNIT_RE = {k: _alt(v) for k, v in _UNIT_TABLE.items()}
_RATE_RE = _alt(_RATES)
_CONN = r"\s*(?:=|:|\bof\b|\bis\b|\bequals\b|\bequal to\b|\bat\b|\bwith\b|\bbe\b)?\s*(?:about\s+|approximately\s+)?"


def _value_pattern(mode):
    if mode.startswith("kind:"):
        return rf"({_N})\s*({_UNIT_RE[mode[5:]]})(?![A-Za-z0-9/^²])"
    return {"num": rf"({_N})(?![\w%]|\.\d)(?!\s*(?i:d[bB]))",
            "db": rf"({_N})\s*(?i:dB)(?![A-Za-z])",
            "dbm": rf"({_N})\s*(?i:dBm)(?![A-Za-z])",
            "dbi": rf"({_N})\s*(?i:dBi|dB)(?![A-Za-z])",
            "int": r"(\d+)(?!\w|\.\d)",
            "vec": r"\[([^\[\]]*)\]",
            "frac": rf"({_N})\s*(%)?(?!\w|\.\d)",
            "rate": rf"({_N})\s*({_RATE_RE})(?![A-Za-z])",
            "ord": r"(\d+)(?:st|nd|rd|th)?(?!\w|\.\d)"}[mode]


def _convert(mode, m):
    if mode.startswith("kind:"):
        return f"{m.group(1)} {_UNIT_TABLE[mode[5:]][m.group(2)]}"
    if mode in ("int", "ord"):
        return int(m.group(1))
    if mode == "vec":
        return m.group(1)
    if mode == "frac":
        return float(m.group(1)) / (100.0 if m.group(2) else 1.0)
    if mode == "rate":
        return float(m.group(1)) * _RATES[m.group(2)]
    return float(m.group(1))


def _find(text, mode, label):
    vp = _value_pattern(mode)
    v = "(?P<v>" + vp + ")"
    if label == "":
        pat = v
    elif label.startswith("^"):
        pat = v + r"\s*(?i:" + label[1:] + r")"
    else:
        pat = r"(?i:" + label + r")" + _CONN + v
    m = re.search(pat, text)
    if not m:
        return None
    sub = re.match(vp, m.group("v"))
    return _convert(mode, sub), m.span()


def _parse(q, trigger, slots, exclude=None, special=None, require=None):
    """Read a documented question: `trigger` must match, `exclude` must not; each slot (symbol, mode, labels) takes
    the first value found after one of its labels (a label starting with '^' follows the value). Every number in the
    question must be read by a slot, otherwise the question is not this capability's (None): nothing is guessed."""
    if not isinstance(q, str) or not re.search(trigger, q, re.I):
        return None
    if exclude and re.search(exclude, q, re.I):
        return None
    if require and not re.search(require, q, re.I):
        return None
    text, out = q, {}
    for pm in _PREMASK:
        text = re.sub(pm, lambda m: "~" * len(m.group(0)), text)
    if special:
        got = special(text)
        if got is None:
            return None
        vals, spans = got
        out.update(vals)
        for a, b in spans:
            text = text[:a] + "~" * (b - a) + text[b:]
    for sym, mode, labels in slots:
        if sym in out:
            continue
        for lab in labels:
            got = _find(text, mode, lab)
            if got:
                out[sym] = got[0]
                a, b = got[1]
                text = text[:a] + "~" * (b - a) + text[b:]
                break
    if _LEFT.search(text):
        return None
    return out


def _choice(q, table):
    """The first choice whose words appear in the question: table = [(regex, canonical), ...]."""
    for pat, canon in table:
        if re.search(pat, q, re.I):
            return canon
    return None


_ASK = r"(?:what\s+(?:is|are|will be|would be)|find|compute|calculate|determine|give|estimate|evaluate|work out|how\s+(?:large|big|long|far|fast|many|much))"
_FS = ["sampling rate", "sampling frequency", "sample rate", "sampled at", "sampling at", r"\bfs\b"]

PROCEDURES: dict = {}


def _p(key, **spec):
    spec.setdefault("verify", _recheck(spec["run"]))
    spec.setdefault("conditions", [])
    PROCEDURES[key] = spec


def _i(symbol, meaning, kind="number", required=True, **extra):
    d = {"symbol": symbol, "kind": kind, "meaning": meaning, "required": required}
    d.update(extra)
    return d


def _o(symbol, meaning, kind="number"):
    return {"symbol": symbol, "kind": kind, "meaning": meaning}


def _db10(x):
    return 10.0 * math.log10(x)


def _db20(x):
    return 20.0 * math.log10(x)


def _logu(x, lo, hi, rng):
    return 10 ** rng.uniform(math.log10(lo), math.log10(hi))


# =================================================================================================== 1. sampling and basics
def period_from_frequency(f):
    f = _pos(f, "frequency")
    return {"T": 1.0 / f, "omega": 2.0 * math.pi * f}


def ref_period_from_frequency(f):
    import numpy as np
    from scipy.signal import find_peaks
    fs = 64.0 * f                                     # 64 samples per period: the peaks fall on samples
    n = np.arange(64 * 12)
    pk, _ = find_peaks(np.cos(2 * np.pi * f * n / fs))
    T = float(np.mean(np.diff(pk))) / fs
    return {"T": T, "omega": 2 * np.pi / T}


_p("period_from_frequency", name="Period and angular frequency from frequency", topic="sampling and signal basics",
   purpose="The period T = 1/f and the angular frequency omega = 2*pi*f of a periodic signal of frequency f.",
   inputs=[_i("f", "frequency of the signal", "frequency")],
   outputs=[_o("T", "period", "time"), _o("omega", "angular frequency", "angular_frequency")],
   primary="T", conditions=["f > 0"], method="T = 1/f, omega = 2 pi f", latex=r"T = \frac{1}{f},\quad \omega = 2\pi f",
   run=period_from_frequency, reference=ref_period_from_frequency,
   wrong=lambda f: {"T": 2 * math.pi / f, "omega": f},
   random_inputs=lambda rng: {"f": _logu(None, 0.1, 1e9, rng)},
   parse=lambda q: _parse(q, _ASK + r"\s+(?:the\s+)?period\b", [("f", "kind:frequency", ["frequency", "^(?:signal|tone|sinusoid|sine wave|square wave|wave|carrier|oscillation)"])],
                          exclude=r"\bperiod of\s+[-+.\d]|\bpulse repetition|\bPRF\b|\bsampling (?:period|interval)"),
   templates=["What is the period of a signal with a frequency of 50 Hz?", "Find the period and angular frequency of a 2 kHz signal."],
   examples=[{"inputs": {"f": 50.0}, "outputs": {"T": 0.02, "omega": 100 * math.pi}},
             {"inputs": {"f": 2000.0}, "outputs": {"T": 5e-4, "omega": 4000 * math.pi}}],
   questions=[{"q": "What is the period of a signal with a frequency of 50 Hz?", "outputs": {"T": 0.02}},
              {"q": "Find the period and angular frequency of a 2 kHz signal.", "outputs": {"T": 5e-4, "omega": 4000 * math.pi}},
              {"q": "What is the period of a signal with a frequency of -5 Hz?", "outcome": "INVALID_INPUT"},
              {"q": "What is the period of the signal?", "outcome": "NEEDS_INPUT"}])


def frequency_from_period(T):
    T = _pos(T, "period")
    return {"f": 1.0 / T, "omega": 2.0 * math.pi / T}


def ref_frequency_from_period(T):
    import numpy as np
    fs, N = 64.0 / T, 512                             # 8 whole periods in 512 samples: the peak falls on bin 8
    X = np.abs(np.fft.rfft(np.cos(2 * np.pi * np.arange(N) / 64.0)))
    f = float(np.fft.rfftfreq(N, 1.0 / fs)[int(np.argmax(X))])
    return {"f": f, "omega": 2 * np.pi * f}


_p("frequency_from_period", name="Frequency and angular frequency from period", topic="sampling and signal basics",
   purpose="The frequency f = 1/T and the angular frequency omega = 2*pi/T of a periodic signal of period T.",
   inputs=[_i("T", "period of the signal", "time")],
   outputs=[_o("f", "frequency", "frequency"), _o("omega", "angular frequency", "angular_frequency")],
   primary="f", conditions=["T > 0"], method="f = 1/T, omega = 2 pi / T", latex=r"f = \frac{1}{T},\quad \omega = \frac{2\pi}{T}",
   run=frequency_from_period, reference=ref_frequency_from_period,
   wrong=lambda T: {"f": 2 * math.pi / T, "omega": 1 / T},
   random_inputs=lambda rng: {"T": _logu(None, 1e-9, 10, rng)},
   parse=lambda q: _parse(q, _ASK + r"\s+(?:the\s+)?(?:angular\s+)?frequency\b", [("T", "kind:time", ["period"])],
                          exclude=r"sampl|nyquist|resolution|cutoff|corner|alias|doppler|\bbin\b|resonan|carrier"),
   templates=["What is the frequency of a signal with a period of 2 ms?", "Find the frequency of a waveform whose period is 20 us."],
   examples=[{"inputs": {"T": 0.002}, "outputs": {"f": 500.0, "omega": 1000 * math.pi}},
             {"inputs": {"T": 2e-5}, "outputs": {"f": 50000.0}}],
   questions=[{"q": "What is the frequency of a signal with a period of 2 ms?", "outputs": {"f": 500.0}},
              {"q": "Find the frequency of a waveform whose period is 20 us.", "outputs": {"f": 50000.0}},
              {"q": "What is the frequency of a signal with a period of 0 s?", "outcome": "INVALID_INPUT"}])


def nyquist_rate(fmax):
    fmax = _pos(fmax, "maximum frequency")
    return {"fs_min": 2.0 * fmax, "Ts_max": 1.0 / (2.0 * fmax)}


def ref_nyquist_rate(fmax):
    from sk_dsp_comm.sigsys import prin_alias
    lo, hi = fmax, 4.0 * fmax                          # the lowest rate at which fmax is its own principal alias
    for _ in range(200):
        mid = 0.5 * (lo + hi)
        if float(prin_alias(fmax, mid)) >= fmax:
            hi = mid
        else:
            lo = mid
    return {"fs_min": hi, "Ts_max": 1.0 / hi}


_p("nyquist_rate", name="Nyquist rate", topic="sampling and signal basics",
   purpose="The minimum sampling rate 2*f_max that avoids aliasing of a baseband signal whose highest frequency is f_max.",
   inputs=[_i("fmax", "highest frequency in the signal", "frequency")],
   outputs=[_o("fs_min", "Nyquist rate (minimum sampling rate)", "frequency"), _o("Ts_max", "longest sampling interval", "time")],
   primary="fs_min", conditions=["fmax > 0", "a baseband (lowpass) signal"], method="sampling theorem: fs >= 2 fmax",
   latex=r"f_{s,\min} = 2 f_{\max}", run=nyquist_rate, reference=ref_nyquist_rate,
   wrong=lambda fmax: {"fs_min": fmax, "Ts_max": 1 / fmax},
   random_inputs=lambda rng: {"fmax": _logu(None, 1, 1e9, rng)},
   parse=lambda q: _parse(q, r"nyquist rate|minimum sampling (?:rate|frequency)|lowest sampling (?:rate|frequency)",
                          [("fmax", "kind:frequency", ["maximum frequency", "highest frequency", "max frequency", "maximum frequency component",
                                                       "frequencies up to", "band-?limited to", "bandlimited to", "contains frequencies up to"])],
                          exclude=r"\bbandpass\b|\bband-pass\b"),
   templates=["What is the Nyquist rate for a signal with a maximum frequency of 4 kHz?",
              "Find the minimum sampling rate for an audio signal band-limited to 20 kHz."],
   examples=[{"inputs": {"fmax": 4000.0}, "outputs": {"fs_min": 8000.0, "Ts_max": 1.25e-4}},
             {"inputs": {"fmax": 20000.0}, "outputs": {"fs_min": 40000.0}}],
   questions=[{"q": "What is the Nyquist rate for a signal with a maximum frequency of 4 kHz?", "outputs": {"fs_min": 8000.0}},
              {"q": "Find the minimum sampling rate for an audio signal band-limited to 20 kHz.", "outputs": {"fs_min": 40000.0}},
              {"q": "What is the Nyquist rate of the signal?", "outcome": "NEEDS_INPUT"}])


def alias_frequency(f, fs):
    f, fs = _nonneg(f, "signal frequency"), _pos(fs, "sampling rate")
    r = math.fmod(f, fs)
    return {"f_alias": min(r, fs - r)}


def ref_alias_frequency(f, fs):
    from sk_dsp_comm.sigsys import prin_alias
    return {"f_alias": float(prin_alias(f, fs))}


_p("alias_frequency", name="Alias (apparent) frequency after sampling", topic="sampling and signal basics",
   purpose="The frequency in [0, fs/2] at which a sinusoid of frequency f appears after sampling at rate fs.",
   inputs=[_i("f", "frequency of the sampled sinusoid", "frequency"), _i("fs", "sampling rate", "frequency")],
   outputs=[_o("f_alias", "apparent (principal alias) frequency", "frequency")],
   primary="f_alias", conditions=["f >= 0", "fs > 0"], method="fold f about multiples of fs: |f - fs*round(f/fs)|",
   latex=r"f_a = \left| f - f_s\,\mathrm{round}(f/f_s) \right|", run=alias_frequency, reference=ref_alias_frequency,
   wrong=lambda f, fs: {"f_alias": math.fmod(f, fs)},
   random_inputs=lambda rng: (lambda fs: {"fs": fs, "f": (rng.randint(0, 19) + (rng.uniform(0.55, 0.95) if rng.random() < 0.85 else rng.uniform(0.05, 0.45))) * fs})(_logu(None, 100, 1e5, rng)),
   parse=lambda q: _parse(q, r"\balias|apparent frequency|appear(?:s)? at|aliased",
                          [("fs", "kind:frequency", _FS), ("f", "kind:frequency", ["signal frequency", "frequency of", "frequency", "^(?:tone|sinusoid|sine wave|signal|cosine|sine)"])]),
   templates=["A 7 kHz tone is sampled at 10 kHz. What is the alias frequency?",
              "What is the apparent frequency of a 1.2 kHz sinusoid sampled at a sampling rate of 1 kHz?"],
   examples=[{"inputs": {"f": 7000.0, "fs": 10000.0}, "outputs": {"f_alias": 3000.0}},
             {"inputs": {"f": 1200.0, "fs": 1000.0}, "outputs": {"f_alias": 200.0}},
             {"inputs": {"f": 300.0, "fs": 1000.0}, "outputs": {"f_alias": 300.0}}],
   questions=[{"q": "A 7 kHz tone is sampled at 10 kHz. What is the alias frequency?", "outputs": {"f_alias": 3000.0}},
              {"q": "What is the apparent frequency of a 1.2 kHz sinusoid sampled at a sampling rate of 1 kHz?", "outputs": {"f_alias": 200.0}},
              {"q": "A 7 kHz tone is sampled. What is the alias frequency?", "outcome": "NEEDS_INPUT"}])


# --- decibels ---------------------------------------------------------------------------------------------------
def power_ratio_to_db(ratio):
    return {"dB": _db10(_pos(ratio, "power ratio"))}


def db_to_power_ratio(dB):
    return {"ratio": 10.0 ** (_fin(dB, "level in dB") / 10.0)}


def amplitude_ratio_to_db(ratio):
    return {"dB": _db20(_pos(ratio, "amplitude ratio"))}


def db_to_amplitude_ratio(dB):
    return {"ratio": 10.0 ** (_fin(dB, "level in dB") / 20.0)}


_PR = r"(?:power ratio|power gain|ratio of (?:the )?(?:output|input|received|transmitted|two)? ?powers?)"
_AR = r"(?:voltage ratio|amplitude ratio|current ratio|voltage gain|amplitude gain|current gain)"
_HASDB = _N + r"\s*(?i:dB)(?![A-Za-z])"

_p("power_ratio_to_db", name="Power ratio to decibels", topic="decibels and levels",
   purpose="A power ratio expressed in decibels, 10*log10(P2/P1).",
   inputs=[_i("ratio", "power ratio P2/P1 (a positive number)")], outputs=[_o("dB", "level in decibels (dB)")],
   primary="dB", conditions=["ratio > 0"], method="10 log10(ratio)", latex=r"L = 10\log_{10}\frac{P_2}{P_1}\ \mathrm{dB}",
   run=power_ratio_to_db,
   reference=lambda ratio: {"dB": float(__import__("skrf").mathFunctions.mag_2_db10(ratio))},
   wrong=lambda ratio: {"dB": _db20(ratio)},
   random_inputs=lambda rng: {"ratio": _logu(None, 1e-6, 1e9, rng)},
   parse=lambda q: _parse(q, _PR, [("ratio", "num", [_PR])], exclude=_HASDB + r"|dbm|(?:what|which) power ratio|(?:to|as|into) a power ratio|power ratio (?:does|for|corresponding)"),
   templates=["Express a power ratio of 200 in decibels.", "Convert a power gain of 1000 to dB."],
   examples=[{"inputs": {"ratio": 200}, "outputs": {"dB": 10 * math.log10(200)}}, {"inputs": {"ratio": 1000}, "outputs": {"dB": 30.0}}],
   questions=[{"q": "Express a power ratio of 200 in decibels.", "outputs": {"dB": 10 * math.log10(200)}},
              {"q": "Convert a power gain of 1000 to dB.", "outputs": {"dB": 30.0}},
              {"q": "Convert a power gain of -4 to dB.", "outcome": "INVALID_INPUT"},
              {"q": "Convert a power ratio to dB.", "outcome": "NEEDS_INPUT"}])

_p("db_to_power_ratio", name="Decibels to power ratio", topic="decibels and levels",
   purpose="The power ratio P2/P1 = 10^(L/10) that a level of L dB represents.",
   inputs=[_i("dB", "level in decibels")], outputs=[_o("ratio", "power ratio P2/P1")],
   primary="ratio", conditions=["finite level"], method="10^(L/10)", latex=r"\frac{P_2}{P_1} = 10^{L/10}",
   run=db_to_power_ratio,
   reference=lambda dB: {"ratio": float(__import__("skrf").mathFunctions.db10_2_mag(dB))},
   wrong=lambda dB: {"ratio": 10 ** (dB / 20)},
   random_inputs=lambda rng: {"dB": rng.uniform(-60, 90)},
   parse=lambda q: _parse(q, r"(?:what|which) power ratio|to a power ratio|as a power ratio|into a power ratio|power ratio (?:does|for|of|corresponding)",
                          [("dB", "db", [""])], exclude=r"dbm|dbi|" + _AR),
   templates=["What power ratio corresponds to 23 dB?", "Convert 30 dB to a power ratio."],
   examples=[{"inputs": {"dB": 23}, "outputs": {"ratio": 10 ** 2.3}}, {"inputs": {"dB": 30}, "outputs": {"ratio": 1000.0}},
             {"inputs": {"dB": -3}, "outputs": {"ratio": 10 ** -0.3}}],
   questions=[{"q": "What power ratio corresponds to 23 dB?", "outputs": {"ratio": 10 ** 2.3}},
              {"q": "Convert 30 dB to a power ratio.", "outputs": {"ratio": 1000.0}},
              {"q": "Convert -10 dB to a power ratio.", "outputs": {"ratio": 0.1}},
              {"q": "What power ratio corresponds to the level?", "outcome": "NEEDS_INPUT"}])

_p("amplitude_ratio_to_db", name="Amplitude (voltage) ratio to decibels", topic="decibels and levels",
   purpose="A voltage, current or field amplitude ratio in decibels, 20*log10(V2/V1).",
   inputs=[_i("ratio", "amplitude ratio V2/V1 (a positive number)")], outputs=[_o("dB", "level in decibels (dB)")],
   primary="dB", conditions=["ratio > 0", "both amplitudes across equal impedances"], method="20 log10(ratio)",
   latex=r"L = 20\log_{10}\frac{V_2}{V_1}\ \mathrm{dB}", run=amplitude_ratio_to_db,
   reference=lambda ratio: {"dB": float(__import__("skrf").mathFunctions.mag_2_db(ratio))},
   wrong=lambda ratio: {"dB": _db10(ratio)},
   random_inputs=lambda rng: {"ratio": _logu(None, 1e-5, 1e6, rng)},
   parse=lambda q: _parse(q, _AR, [("ratio", "num", [_AR])], exclude=_HASDB + r"|dbm|(?:what|which) (?:voltage|amplitude|current) ratio|(?:to|as|into) an? (?:voltage|amplitude|current) ratio|(?:voltage|amplitude|current) ratio (?:does|for|corresponding)"),
   templates=["Express a voltage ratio of 50 in decibels.", "Convert a voltage gain of 100 to dB."],
   examples=[{"inputs": {"ratio": 50}, "outputs": {"dB": 20 * math.log10(50)}}, {"inputs": {"ratio": 100}, "outputs": {"dB": 40.0}}],
   questions=[{"q": "Express a voltage ratio of 50 in decibels.", "outputs": {"dB": 20 * math.log10(50)}},
              {"q": "Convert a voltage gain of 100 to dB.", "outputs": {"dB": 40.0}},
              {"q": "Convert a voltage gain of 0 to dB.", "outcome": "INVALID_INPUT"}])

_p("db_to_amplitude_ratio", name="Decibels to amplitude (voltage) ratio", topic="decibels and levels",
   purpose="The amplitude ratio V2/V1 = 10^(L/20) that a level of L dB represents.",
   inputs=[_i("dB", "level in decibels")], outputs=[_o("ratio", "amplitude ratio V2/V1")],
   primary="ratio", conditions=["finite level"], method="10^(L/20)", latex=r"\frac{V_2}{V_1} = 10^{L/20}",
   run=db_to_amplitude_ratio,
   reference=lambda dB: {"ratio": float(__import__("skrf").mathFunctions.db_2_mag(dB))},
   wrong=lambda dB: {"ratio": 10 ** (dB / 10)},
   random_inputs=lambda rng: {"dB": rng.uniform(-80, 80)},
   parse=lambda q: _parse(q, r"(?:what|which) (?:voltage|amplitude|current) ratio|to an? (?:voltage|amplitude|current) ratio|as an? (?:voltage|amplitude|current) ratio|(?:voltage|amplitude|current) ratio (?:does|for|of|corresponding)",
                          [("dB", "db", [""])], exclude=r"dbm|dbi|" + _PR),
   templates=["What voltage ratio corresponds to 6 dB?", "Convert 40 dB to an amplitude ratio."],
   examples=[{"inputs": {"dB": 6}, "outputs": {"ratio": 10 ** 0.3}}, {"inputs": {"dB": 40}, "outputs": {"ratio": 100.0}}],
   questions=[{"q": "What voltage ratio corresponds to 6 dB?", "outputs": {"ratio": 10 ** 0.3}},
              {"q": "Convert 40 dB to an amplitude ratio.", "outputs": {"ratio": 100.0}},
              {"q": "Convert -20 dB to a voltage ratio.", "outputs": {"ratio": 0.1}}])


def watts_to_dbm(P):
    P = _pos(P, "power")
    return {"dBm": _db10(P / 1e-3), "dBW": _db10(P)}


def dbm_to_watts(dBm):
    dBm = _fin(dBm, "power level in dBm")
    return {"P": 1e-3 * 10.0 ** (dBm / 10.0), "dBW": dBm - 30.0}


_p("watts_to_dbm", name="Power in watts to dBm and dBW", topic="decibels and levels",
   purpose="A power expressed as a level relative to 1 mW (dBm) and to 1 W (dBW).",
   inputs=[_i("P", "power", "power")], outputs=[_o("dBm", "power level in dBm"), _o("dBW", "power level in dBW")],
   primary="dBm", conditions=["P > 0"], method="10 log10(P / 1 mW)", latex=r"P_{\mathrm{dBm}} = 10\log_{10}\frac{P}{1\,\mathrm{mW}}",
   run=watts_to_dbm,
   reference=lambda P: (lambda mf: {"dBm": float(mf.mag_2_db10(P / 1e-3)), "dBW": float(mf.mag_2_db10(P))})(__import__("skrf").mathFunctions),
   wrong=lambda P: {"dBm": _db10(P), "dBW": _db10(P)},
   random_inputs=lambda rng: {"P": _logu(None, 1e-15, 1e6, rng)},
   parse=lambda q: _parse(q, r"\b(?:to|in|into|as)\s+dBm\b|how many dBm", [("P", "kind:power", [""])], exclude=_N + r"\s*(?i:dBm)\b"),
   templates=["Convert 2 W to dBm.", "Express a power of 50 mW in dBm."],
   examples=[{"inputs": {"P": 2.0}, "outputs": {"dBm": 10 * math.log10(2000), "dBW": 10 * math.log10(2)}},
             {"inputs": {"P": 0.05}, "outputs": {"dBm": 10 * math.log10(50)}}, {"inputs": {"P": 1e-3}, "outputs": {"dBm": 0.0}}],
   questions=[{"q": "Convert 2 W to dBm.", "outputs": {"dBm": 10 * math.log10(2000)}},
              {"q": "Express a power of 50 mW in dBm.", "outputs": {"dBm": 10 * math.log10(50)}},
              {"q": "Convert 0 W to dBm.", "outcome": "INVALID_INPUT"}])

_p("dbm_to_watts", name="Power level in dBm to watts", topic="decibels and levels",
   purpose="The power in watts of a level given in dBm (and the same level in dBW).",
   inputs=[_i("dBm", "power level in dBm")], outputs=[_o("P", "power", "power"), _o("dBW", "power level in dBW")],
   primary="P", conditions=["finite level"], method="P = 1 mW * 10^(L/10)", latex=r"P = 1\,\mathrm{mW}\cdot 10^{P_{\mathrm{dBm}}/10}",
   run=dbm_to_watts,
   reference=lambda dBm: {"P": float(__import__("skrf").mathFunctions.db10_2_mag(dBm)) * 1e-3, "dBW": float(__import__("skrf").mathFunctions.mag_2_db10(float(__import__("skrf").mathFunctions.db10_2_mag(dBm)) * 1e-3))},
   wrong=lambda dBm: {"P": 10 ** (dBm / 10), "dBW": dBm},
   random_inputs=lambda rng: {"dBm": rng.uniform(-120, 90)},
   parse=lambda q: _parse(q, r"dBm\s+(?:to|in|into|as)\s+(?:watts?|milliwatts?|W|mW)\b|(?:how many|what power in) (?:watts|milliwatts)",
                          [("dBm", "dbm", [""])]),
   templates=["Convert 30 dBm to watts.", "Convert -20 dBm to milliwatts."],
   examples=[{"inputs": {"dBm": 30}, "outputs": {"P": 1.0, "dBW": 0.0}}, {"inputs": {"dBm": -20}, "outputs": {"P": 1e-5}}],
   questions=[{"q": "Convert 30 dBm to watts.", "outputs": {"P": 1.0}},
              {"q": "Convert -20 dBm to milliwatts.", "outputs": {"P": 1e-5}},
              {"q": "Convert 43 dBm to W.", "outputs": {"P": 10 ** 4.3 * 1e-3}}])


def snr_from_powers(Ps, Pn):
    Ps, Pn = _pos(Ps, "signal power"), _pos(Pn, "noise power")
    return {"snr": Ps / Pn, "snr_db": _db10(Ps / Pn)}


_p("snr_from_powers", name="Signal-to-noise ratio from signal and noise powers", topic="decibels and levels",
   purpose="The signal-to-noise ratio Ps/Pn, as a ratio and in dB.",
   inputs=[_i("Ps", "signal power", "power"), _i("Pn", "noise power", "power")],
   outputs=[_o("snr_db", "signal-to-noise ratio in dB"), _o("snr", "signal-to-noise ratio (linear)")],
   primary="snr_db", conditions=["Ps > 0", "Pn > 0"], method="SNR = Ps/Pn; 10 log10(SNR)",
   latex=r"\mathrm{SNR}_{\mathrm{dB}} = 10\log_{10}\frac{P_s}{P_n}", run=snr_from_powers,
   reference=lambda Ps, Pn: {"snr": Ps / Pn, "snr_db": float(__import__("skrf").mathFunctions.mag_2_db10(Ps)) - float(__import__("skrf").mathFunctions.mag_2_db10(Pn))},
   wrong=lambda Ps, Pn: {"snr": Ps / Pn, "snr_db": _db20(Ps / Pn)},
   random_inputs=lambda rng: {"Ps": _logu(None, 1e-12, 10, rng), "Pn": _logu(None, 1e-15, 1e-3, rng)},
   parse=lambda q: _parse(q, r"signal[- ]to[- ]noise|\bSNR\b", [("Ps", "kind:power", ["signal power", "power of the signal", "received signal power"]),
                                                               ("Pn", "kind:power", ["noise power", "power of the noise"])],
                          exclude=r"bandwidth|capacity|eb/n0|eb/no|quantiz|bits?\b"),
   templates=["What is the SNR in dB for a signal power of 2 mW and a noise power of 10 uW?"],
   examples=[{"inputs": {"Ps": 2e-3, "Pn": 1e-5}, "outputs": {"snr": 200.0, "snr_db": 10 * math.log10(200)}}],
   questions=[{"q": "What is the SNR in dB for a signal power of 2 mW and a noise power of 10 uW?", "outputs": {"snr_db": 10 * math.log10(200)}},
              {"q": "What is the signal-to-noise ratio for a signal power of 1 W and a noise power of 1 mW?", "outputs": {"snr_db": 30.0}},
              {"q": "What is the SNR in dB for a signal power of 2 mW?", "outcome": "NEEDS_INPUT"}])


# --- statistics of a sampled signal ------------------------------------------------------------------------------
def signal_statistics(x):
    x = _vec(x, "signal x")
    n = len(x)
    e = math.fsum(v * v for v in x)
    return {"mean": math.fsum(x) / n, "rms": math.sqrt(e / n), "energy": e, "power": e / n,
            "peak": max(abs(v) for v in x)}


def ref_signal_statistics(x):
    import numpy as np
    a = np.asarray(x, float)
    return {"mean": float(np.mean(a)), "rms": float(np.sqrt(np.mean(a ** 2))), "energy": float(np.sum(a ** 2)),
            "power": float(np.mean(np.abs(a) ** 2)), "peak": float(np.max(np.abs(a)))}


_p("signal_statistics", name="Mean, RMS, energy and average power of a sampled signal", topic="sampling and signal basics",
   purpose="For samples x[0..N-1]: the mean, the RMS value, the energy sum |x|^2 and the average power (energy / N).",
   inputs=[_i("x", "samples of the signal", type="vector", min_length=1, max_length=100000)],
   outputs=[_o("rms", "root-mean-square value"), _o("mean", "mean (DC) value"), _o("energy", "energy: sum of x[n]^2"),
            _o("power", "average power: energy / N"), _o("peak", "peak absolute value")],
   primary="rms", conditions=["at least one sample"], method="direct sums (compensated)",
   latex=r"E = \sum_n |x[n]|^2,\quad P = \frac{E}{N},\quad x_{\mathrm{rms}} = \sqrt{P}", run=signal_statistics,
   reference=ref_signal_statistics,
   wrong=lambda x: dict(signal_statistics(x), rms=math.sqrt(math.fsum(v * v for v in x) / max(1, len(x) - 1)) * (1 if len(x) > 1 else 2)),
   random_inputs=lambda rng: {"x": [round(rng.uniform(-10, 10), rng.choice([0, 2, 6])) for _ in range(rng.randint(2, 40))]},
   parse=lambda q: _parse(q, r"\b(?:rms|root[- ]mean[- ]square|energy|average power|mean value|mean)\b",
                          [("x", "vec", [r"\bx(?:\[n\])?", "samples", "signal", "sequence", ""])],
                          exclude=r"dft|fft|fourier|convol|filter|correlat|window|spectrum|harmonic|dwt|wavelet|thd|dbm|noise figure", require=r"\["),
   templates=["Find the energy, average power and RMS value of the signal x = [1, -2, 3, 0].",
              "What is the RMS value of the samples [2, -2, 2, -2]?"],
   examples=[{"inputs": {"x": "1, -2, 3, 0"}, "outputs": {"energy": 14.0, "power": 3.5, "rms": math.sqrt(3.5), "mean": 0.5, "peak": 3.0}},
             {"inputs": {"x": [2, -2, 2, -2]}, "outputs": {"rms": 2.0, "mean": 0.0}}],
   questions=[{"q": "Find the energy, average power and RMS value of the signal x = [1, -2, 3, 0].", "outputs": {"energy": 14.0, "power": 3.5}},
              {"q": "What is the RMS value of the samples [2, -2, 2, -2]?", "outputs": {"rms": 2.0}},
              {"q": "Find the energy of the signal x = [1, a, 3].", "outcome": "INVALID_INPUT"}])


def sine_rms(Vp):
    Vp = _pos(Vp, "peak value")
    return {"Vrms": Vp / math.sqrt(2.0), "Vpp": 2.0 * Vp, "Vavg": 2.0 * Vp / math.pi}


def ref_sine_rms(Vp):
    from math import pi, sin
    from scipy.integrate import quad
    ms, _ = quad(lambda t: (Vp * sin(t)) ** 2, 0, 2 * pi, epsabs=0, epsrel=1e-13)
    av, _ = quad(lambda t: abs(Vp * sin(t)), 0, 2 * pi, points=[pi], epsabs=0, epsrel=1e-13)
    return {"Vrms": (ms / (2 * pi)) ** 0.5, "Vpp": 2 * Vp, "Vavg": av / (2 * pi)}


_p("sine_rms", name="RMS, peak-to-peak and rectified average of a sine wave", topic="sampling and signal basics",
   purpose="For a sinusoid of peak amplitude Vp: the RMS value Vp/sqrt(2), the peak-to-peak value 2*Vp and the average of its absolute value 2*Vp/pi.",
   inputs=[_i("Vp", "peak amplitude of the sinusoid", "voltage")],
   outputs=[_o("Vrms", "RMS value", "voltage"), _o("Vpp", "peak-to-peak value", "voltage"), _o("Vavg", "average of the rectified wave", "voltage")],
   primary="Vrms", conditions=["Vp > 0", "a pure sinusoid with no DC offset"], method="V_rms = V_p / sqrt(2)",
   latex=r"V_{\mathrm{rms}} = \frac{V_p}{\sqrt{2}}", run=sine_rms, reference=ref_sine_rms,
   wrong=lambda Vp: {"Vrms": Vp / 2, "Vpp": 2 * Vp, "Vavg": 2 * Vp / math.pi},
   random_inputs=lambda rng: {"Vp": _logu(None, 1e-6, 1e5, rng)},
   parse=lambda q: _parse(q, r"(?:sine|sinusoid|sinusoidal|\bac\b|cosine)", [("Vp", "kind:voltage", ["peak voltage", "peak value", "peak amplitude", "amplitude", "peak of", "^peak"])],
                          exclude=r"\[|harmonic|square|triangle|sawtooth|quantiz|\bdbm\b") if re.search(r"\brms\b|root[- ]mean[- ]square|effective value", q or "", re.I) else None,
   templates=["What is the RMS value of a sine wave with a peak voltage of 10 V?", "Find the RMS value of a sinusoidal voltage with an amplitude of 170 V."],
   examples=[{"inputs": {"Vp": 10.0}, "outputs": {"Vrms": 10 / math.sqrt(2), "Vpp": 20.0, "Vavg": 20 / math.pi}}],
   questions=[{"q": "What is the RMS value of a sine wave with a peak voltage of 10 V?", "outputs": {"Vrms": 10 / math.sqrt(2)}},
              {"q": "Find the RMS value of a sinusoidal voltage with an amplitude of 170 V.", "outputs": {"Vrms": 170 / math.sqrt(2)}},
              {"q": "Find the RMS value of a sinusoidal voltage.", "outcome": "NEEDS_INPUT"}])


def quantization_snr(bits):
    b = _int(bits, "number of bits", 1, 64)
    return {"sqnr_db": _db10(1.5) + 2 * b * _db10(2.0), "dynamic_range_db": b * _db20(2.0)}


def ref_quantization_snr(bits):
    from math import log10, pi, sin
    from scipy.integrate import quad
    A = 1.0
    d = 2 * A / 2 ** bits                                            # step of a full-scale uniform quantizer
    ps, _ = quad(lambda t: (A * sin(t)) ** 2, 0, 2 * pi, epsabs=0, epsrel=1e-13)
    pn, _ = quad(lambda e: e * e / d, -d / 2, d / 2, epsabs=0, epsrel=1e-13)
    return {"sqnr_db": 10 * log10(ps / (2 * pi) / pn), "dynamic_range_db": 20 * log10(2 ** bits)}


_p("quantization_snr", name="Signal-to-quantization-noise ratio of an ideal ADC", topic="sampling and signal basics",
   purpose="The SQNR of an ideal N-bit uniform quantizer driven by a full-scale sine wave (about 6.02 N + 1.76 dB).",
   inputs=[_i("bits", "resolution in bits", type="integer", min=1, max=64)],
   outputs=[_o("sqnr_db", "signal-to-quantization-noise ratio in dB"), _o("dynamic_range_db", "ideal dynamic range 20 log10(2^N) in dB")],
   primary="sqnr_db", conditions=["ideal uniform quantizer", "full-scale sinusoidal input", "noise uniformly distributed"],
   method="ratio of the sine's power A^2/2 to the quantization noise power Delta^2/12 with Delta = 2A/2^N",
   latex=r"\mathrm{SQNR} = 10\log_{10}\!\left(\tfrac{3}{2}\,2^{2N}\right) \approx 6.02N + 1.76\ \mathrm{dB}",
   run=quantization_snr, reference=ref_quantization_snr,
   wrong=lambda bits: {"sqnr_db": 6.02 * bits, "dynamic_range_db": bits * _db20(2.0)},
   random_inputs=lambda rng: {"bits": rng.randint(1, 32)},
   parse=lambda q: _parse(q, r"quantiz|quantis|sqnr|\badc\b|analog[- ]to[- ]digital",
                          [("bits", "int", ["^(?:-|\\s)?bits?\\b", "resolution of", "bits"])],
                          exclude=r"bit rate|bits? per second|bps\b|step size") if re.search(r"snr|sqnr|noise ratio|signal[- ]to", q or "", re.I) else None,
   templates=["What is the signal-to-quantization-noise ratio of an ideal 12-bit ADC?", "Find the SQNR of a 16-bit quantizer."],
   examples=[{"inputs": {"bits": 12}, "outputs": {"sqnr_db": 10 * math.log10(1.5 * 4 ** 12)}},
             {"inputs": {"bits": 16}, "outputs": {"sqnr_db": 10 * math.log10(1.5 * 4 ** 16)}}],
   questions=[{"q": "What is the signal-to-quantization-noise ratio of an ideal 12-bit ADC?", "outputs": {"sqnr_db": 10 * math.log10(1.5 * 4 ** 12)}},
              {"q": "Find the SQNR of a 16-bit quantizer.", "outputs": {"sqnr_db": 10 * math.log10(1.5 * 4 ** 16)}},
              {"q": "Find the SQNR of a 0-bit quantizer.", "outcome": "INVALID_INPUT"}])


def dbm_power_sum(levels):
    L = _vec(levels, "power levels in dBm")
    p = math.fsum(10.0 ** (v / 10.0) for v in L) * 1e-3
    return {"total_dBm": _db10(p / 1e-3), "total_W": p}


def ref_dbm_power_sum(levels):
    import numpy as np
    from skrf import mathFunctions as mf
    p = float(np.sum(mf.db10_2_mag(np.asarray(levels, float)))) * 1e-3
    return {"total_dBm": float(mf.mag_2_db10(p / 1e-3)), "total_W": p}


_p("dbm_power_sum", name="Total power of several signals given in dBm", topic="decibels and levels",
   purpose="The combined power of uncorrelated signals whose levels are given in dBm (powers add, levels do not).",
   inputs=[_i("levels", "power levels in dBm", type="vector", min_length=1, max_length=10000)],
   outputs=[_o("total_dBm", "total power level in dBm"), _o("total_W", "total power", "power")],
   primary="total_dBm", conditions=["uncorrelated signals (their powers add)"], method="convert to milliwatts, add, convert back",
   latex=r"P_{\mathrm{tot}} = 10\log_{10}\sum_i 10^{P_i/10}\ \mathrm{dBm}", run=dbm_power_sum, reference=ref_dbm_power_sum,
   wrong=lambda levels: {"total_dBm": math.fsum(levels), "total_W": 1e-3 * 10 ** (math.fsum(levels) / 10)},
   random_inputs=lambda rng: {"levels": [round(rng.uniform(-90, 30), 1) for _ in range(rng.randint(2, 8))]},
   parse=lambda q: _parse(q, r"\b(?:total|combined|sum|add|adding|together)\b",
                          [("levels", "vec", ["levels", "powers", "signals", "of", ""])], exclude=r"noise figure|gain") if re.search(r"\]\s*dBm\b", q or "", re.I) else None,
   templates=["What is the total power of signals at [10, 10, 7] dBm combined?", "Add the powers [-3, 0, 2] dBm."],
   examples=[{"inputs": {"levels": "10, 10"}, "outputs": {"total_dBm": 10 + 10 * math.log10(2)}},
             {"inputs": {"levels": [0, 0, 0, 0]}, "outputs": {"total_dBm": 10 * math.log10(4), "total_W": 4e-3}}],
   questions=[{"q": "What is the total power of signals at [10, 10] dBm combined?", "outputs": {"total_dBm": 10 + 10 * math.log10(2)}},
              {"q": "Add the powers [0, 0, 0, 0] dBm.", "outputs": {"total_W": 4e-3}}])


def frequency_resolution(fs, N):
    fs, N = _pos(fs, "sampling rate"), _int(N, "number of points", 2)
    return {"df": fs / N, "duration": N / fs}


_p("frequency_resolution", name="Frequency resolution of an N-point DFT", topic="DFT and FFT",
   purpose="The bin spacing fs/N of an N-point DFT or FFT and the record length N/fs it covers.",
   inputs=[_i("fs", "sampling rate", "frequency"), _i("N", "number of DFT points", type="integer", min=2)],
   outputs=[_o("df", "frequency resolution (bin spacing)", "frequency"), _o("duration", "record length N/fs", "time")],
   primary="df", conditions=["fs > 0", "N >= 2"], method="delta f = fs / N", latex=r"\Delta f = \frac{f_s}{N}",
   run=frequency_resolution,
   reference=lambda fs, N: (lambda f: {"df": float(f[1]), "duration": 1 / float(f[1])})(__import__("numpy").fft.rfftfreq(N, 1.0 / fs)),
   wrong=lambda fs, N: {"df": fs / (2 * N), "duration": 2 * N / fs},
   random_inputs=lambda rng: {"fs": _logu(None, 10, 1e9, rng), "N": rng.choice([2 ** rng.randint(1, 16), rng.randint(2, 5000)])},
   parse=lambda q: _parse(q, r"frequency resolution|bin spacing|bin width|resolution bandwidth|spacing between (?:the )?bins",
                          [("fs", "kind:frequency", _FS), ("N", "int", ["^-?\\s?point", "^samples", r"\bN\s*=", "length", "size"])]),
   templates=["What is the frequency resolution of a 1024-point FFT at a sampling rate of 48 kHz?"],
   examples=[{"inputs": {"fs": 48000.0, "N": 1024}, "outputs": {"df": 46.875, "duration": 1024 / 48000}}],
   questions=[{"q": "What is the frequency resolution of a 1024-point FFT at a sampling rate of 48 kHz?", "outputs": {"df": 46.875}},
              {"q": "Find the bin spacing of a DFT of 500 samples at a sampling frequency of 1 kHz.", "outputs": {"df": 2.0}},
              {"q": "What is the frequency resolution of a 1024-point FFT?", "outcome": "NEEDS_INPUT"}])


def bin_frequency(k, fs, N):
    N = _int(N, "number of points", 2)
    k = _int(k, "bin index", 0, N // 2)
    fs = _pos(fs, "sampling rate")
    return {"f_k": k * fs / N}


_p("bin_frequency", name="Frequency of a DFT bin", topic="DFT and FFT",
   purpose="The frequency k*fs/N that bin k of an N-point DFT represents (0 <= k <= N/2).",
   inputs=[_i("k", "bin index", type="integer", min=0), _i("fs", "sampling rate", "frequency"), _i("N", "number of DFT points", type="integer", min=2)],
   outputs=[_o("f_k", "frequency of bin k", "frequency")], primary="f_k", conditions=["0 <= k <= N/2", "fs > 0"],
   method="f_k = k fs / N", latex=r"f_k = \frac{k\,f_s}{N}", run=bin_frequency,
   reference=lambda k, fs, N: {"f_k": float(__import__("numpy").fft.rfftfreq(N, 1.0 / fs)[k])},
   wrong=lambda k, fs, N: {"f_k": k * fs / (N - 1)},
   random_inputs=lambda rng: (lambda N: {"N": N, "k": rng.randint(1, N // 2), "fs": _logu(None, 10, 1e9, rng)})(rng.choice([2 ** rng.randint(2, 14), rng.randint(4, 3000)])),
   parse=lambda q: _parse(q, r"\bbin\b", [("fs", "kind:frequency", _FS), ("N", "int", ["^-?\\s?point", r"\bN\s*="]),
                                         ("k", "int", [r"\bbin(?: number| index)?(?:\s*k)?", r"\bk\s*="])],
                          exclude=r"\[|resolution|spacing|width") if re.search(r"frequency|correspond", q or "", re.I) else None,
   templates=["What frequency does bin 100 of a 1024-point FFT correspond to at a sampling rate of 48 kHz?"],
   examples=[{"inputs": {"k": 100, "fs": 48000.0, "N": 1024}, "outputs": {"f_k": 4687.5}}],
   questions=[{"q": "What frequency does bin 100 of a 1024-point FFT correspond to at a sampling rate of 48 kHz?", "outputs": {"f_k": 4687.5}},
              {"q": "What frequency does bin 900 of a 1024-point FFT correspond to at a sampling rate of 48 kHz?", "outcome": "INVALID_INPUT"}])


# =================================================================================================== 2. DFT and FFT
def _tw(m, N, sign=-1):
    """exp(sign * j 2 pi m / N), exact at the quarter points."""
    m %= N
    if (4 * m) % N == 0:
        return (1 + 0j, complex(0, sign), -1 + 0j, complex(0, -sign))[(4 * m) // N]
    a = sign * 2.0 * math.pi * m / N
    return complex(math.cos(a), math.sin(a))


def _dft_direct(x, sign=-1):
    N, out = len(x), []
    for k in range(N):
        t = [complex(x[n]) * _tw(k * n, N, sign) for n in range(N)]
        out.append(complex(math.fsum(z.real for z in t), math.fsum(z.imag for z in t)))
    return out


def _fft2(x, sign):
    N = len(x)
    if N == 1:
        return [complex(x[0])]
    ev, od = _fft2(x[0::2], sign), _fft2(x[1::2], sign)
    out = [0j] * N
    for k in range(N // 2):
        t = _tw(k, N, sign) * od[k]
        out[k], out[k + N // 2] = ev[k] + t, ev[k] - t
    return out


def _fft(x, sign=-1):
    """The DFT: a radix-2 decimation-in-time FFT when N is a power of two, the direct sum otherwise."""
    N = len(x)
    return _fft2(list(x), sign) if N & (N - 1) == 0 else _dft_direct(x, sign)


def _snap(z, scale):
    """Components below 1e-11 of the sum of |inputs| are rounding residue: shown as exact zeros (no -0, no phase of
    noise)."""
    tol = 1e-11 * scale
    z = complex(z)
    return complex((z.real if abs(z.real) > tol else 0.0) + 0.0, (z.imag if abs(z.imag) > tol else 0.0) + 0.0)


def _peak(mags, scale):
    top = max(mags)
    return next(i for i, m in enumerate(mags) if m >= top - 1e-9 * max(scale, 1e-300))


def dft_bin(x, k):
    x = _vec(x, "signal x", 1, 65536)
    N = len(x)
    k = _int(k, "bin index k", 0, N - 1)
    t = [x[n] * _tw(k * n, N) for n in range(N)]
    X = _snap(complex(math.fsum(z.real for z in t), math.fsum(z.imag for z in t)), math.fsum(abs(v) for v in x))
    return {"magnitude": abs(X), "phase": math.atan2(X.imag, X.real), "re": X.real, "im": X.imag}


def ref_dft_bin(x, k):
    import numpy as np
    X = _snap(complex(np.fft.fft(np.asarray(x, float))[k]), float(np.sum(np.abs(x))))
    return {"magnitude": float(np.abs(X)), "phase": float(np.angle(X)), "re": X.real, "im": X.imag}


def _rand_vec(rng, lo=2, hi=32):
    return [round(rng.uniform(-10, 10), rng.choice([0, 1, 3, 8])) for _ in range(rng.randint(lo, hi))]


_p("dft_bin", name="One bin of the discrete Fourier transform", topic="DFT and FFT",
   purpose="X[k] = sum_n x[n] exp(-j 2 pi k n / N) for one bin k: real and imaginary parts, magnitude and phase.",
   inputs=[_i("x", "samples x[0..N-1]", type="vector", min_length=1, max_length=65536), _i("k", "bin index (0 <= k < N)", type="integer", min=0)],
   outputs=[_o("magnitude", "|X[k]|"), _o("phase", "phase of X[k] in radians (-pi, pi]", "angle"), _o("re", "real part of X[k]"),
            _o("im", "imaginary part of X[k]")],
   primary="magnitude", conditions=["0 <= k < N"], method="direct evaluation of the DFT sum (compensated sums, exact twiddles at quarter turns)",
   latex=r"X[k] = \sum_{n=0}^{N-1} x[n]\,e^{-j2\pi kn/N}", run=dft_bin, reference=ref_dft_bin,
   wrong=lambda x, k: (lambda r: dict(r, im=-r["im"], phase=-r["phase"]))(dft_bin(x, k)),
   random_inputs=lambda rng: (lambda x: {"x": x, "k": rng.choice([i for i in range(1, len(x)) if 2 * i != len(x)] or [0])})(_rand_vec(rng, 3)),
   parse=lambda q: _parse(q, r"\b(?:dft|fft|discrete fourier transform)\b", [("x", "vec", [r"\bx(?:\[n\])?(?=\s*[=:])", "sequence", "signal", ""]),
                                                                        ("k", "int", [r"\bk(?=\s*=)", r"\bbin(?: number| index)?", r"\bindex"])],
                          exclude=r"inverse|\bidft\b|\bifft\b|window|convol|resolution|spacing") if re.search(r"\bbin\b|\bk\s*=", q or "", re.I) else None,
   templates=["Compute the DFT of x = [1, 2, 3, 4] at bin k = 1.", "Find X[k] for k = 2 of the DFT of x = [1, 0, -1, 0, 1, 0, -1, 0]."],
   examples=[{"inputs": {"x": "1, 2, 3, 4", "k": 1}, "outputs": {"re": -2.0, "im": 2.0, "magnitude": 2 * math.sqrt(2), "phase": 3 * math.pi / 4}},
             {"inputs": {"x": [1, 2, 3, 4], "k": 2}, "outputs": {"re": -2.0, "im": 0.0, "phase": math.pi}},
             {"inputs": {"x": [1, 0, -1, 0, 1, 0, -1, 0], "k": 2}, "outputs": {"magnitude": 4.0, "phase": 0.0}}],
   questions=[{"q": "Compute the DFT of x = [1, 2, 3, 4] at bin k = 1.", "outputs": {"re": -2.0, "im": 2.0}},
              {"q": "Find X[k] for k = 2 of the DFT of x = [1, 0, -1, 0, 1, 0, -1, 0].", "outputs": {"magnitude": 4.0}},
              {"q": "Compute the DFT of x = [1, 2, 3, 4] at bin k = 7.", "outcome": "INVALID_INPUT"},
              {"q": "Compute the DFT of x = [1, 2, 3, 4] at bin k.", "outcome": "NEEDS_INPUT"}])


def dft_spectrum(x):
    x = _vec(x, "signal x", 1, 65536)
    s = math.fsum(abs(v) for v in x)
    X = [_snap(z, s) for z in _fft(x)]
    mags = [abs(z) for z in X]
    kp = _peak(mags[: len(x) // 2 + 1], s)
    return {"peak_magnitude": mags[kp], "peak_bin": kp, "magnitude": mags, "phase": [math.atan2(z.imag, z.real) for z in X],
            "real": [z.real for z in X], "imag": [z.imag for z in X]}


def ref_dft_spectrum(x):
    import numpy as np
    s = float(np.sum(np.abs(x)))
    X = [_snap(z, s) for z in np.fft.fft(np.asarray(x, float))]
    mags = [float(abs(z)) for z in X]
    kp = _peak(mags[: len(x) // 2 + 1], s)
    return {"peak_magnitude": mags[kp], "peak_bin": kp, "magnitude": mags, "phase": [float(np.angle(z)) for z in X],
            "real": [z.real for z in X], "imag": [z.imag for z in X]}


_p("dft_spectrum", name="DFT spectrum of a sequence (FFT)", topic="DFT and FFT",
   purpose="The whole N-point DFT of a real sequence: magnitude, phase, real and imaginary parts of every bin, and the strongest bin up to N/2.",
   inputs=[_i("x", "samples x[0..N-1]", type="vector", min_length=1, max_length=65536)],
   outputs=[_o("peak_magnitude", "largest |X[k]| for 0 <= k <= N/2"), _o("peak_bin", "bin of the largest magnitude"),
            _o("magnitude", "|X[k]| for every bin"), _o("phase", "phase of X[k] in radians for every bin"),
            _o("real", "real parts"), _o("imag", "imaginary parts")],
   primary="peak_magnitude", conditions=["real samples"], method="radix-2 decimation-in-time FFT (N a power of two) or the direct DFT sum",
   latex=r"X[k] = \sum_{n=0}^{N-1} x[n]\,e^{-j2\pi kn/N},\ k = 0,\dots,N-1", run=dft_spectrum, reference=ref_dft_spectrum,
   wrong=lambda x: (lambda r: dict(r, imag=[-v + 0.0 for v in r["imag"]], phase=[-v + 0.0 for v in r["phase"]]))(dft_spectrum(x)),
   random_inputs=lambda rng: {"x": _rand_vec(rng, 3, rng.choice([8, 16, 33, 64]))},
   parse=lambda q: _parse(q, r"\b(?:dft|fft|discrete fourier transform|spectrum)\b", [("x", "vec", [r"\bx(?:\[n\])?(?=\s*[=:])", "sequence", "signal", ""])],
                          exclude=r"inverse|\bidft\b|\bifft\b|\bbin\b|\bk\s*=|window|convol|filter|resolution|spacing|power of signals|\]\s*dbm", require=r"\["),
   templates=["Compute the DFT of x = [1, 2, 3, 4].", "Find the FFT spectrum of the sequence [1, 0, -1, 0, 1, 0, -1, 0]."],
   examples=[{"inputs": {"x": "1, 2, 3, 4"}, "outputs": {"real": [10.0, -2.0, -2.0, -2.0], "imag": [0.0, 2.0, 0.0, -2.0], "peak_bin": 0, "peak_magnitude": 10.0}},
             {"inputs": {"x": [1, 0, -1, 0, 1, 0, -1, 0]}, "outputs": {"magnitude": [0.0, 0.0, 4.0, 0.0, 0.0, 0.0, 4.0, 0.0], "peak_bin": 2}},
             {"inputs": {"x": [1, 1, 1]}, "outputs": {"magnitude": [3.0, 0.0, 0.0]}}],
   questions=[{"q": "Compute the DFT of x = [1, 2, 3, 4].", "outputs": {"real": [10.0, -2.0, -2.0, -2.0], "imag": [0.0, 2.0, 0.0, -2.0]}},
              {"q": "Find the FFT spectrum of the sequence [1, 0, -1, 0, 1, 0, -1, 0].", "outputs": {"peak_bin": 2, "peak_magnitude": 4.0}},
              {"q": "Compute the DFT of x = [].", "outcome": "INVALID_INPUT"}])


def inverse_dft(re_, im=None):
    re_ = _vec(re_, "real parts", 1, 65536)
    im = [0.0] * len(re_) if im is None else _vec(im, "imaginary parts", 1, 65536)
    if len(im) != len(re_):
        raise ValueError("the real and imaginary parts must have the same length")
    X = [complex(a, b) for a, b in zip(re_, im)]
    N = len(X)
    s = math.fsum(abs(z) for z in X) / N
    x = [_snap(z / N, s) for z in _fft(X, +1)]
    return {"real": [z.real for z in x], "imag": [z.imag for z in x]}


def ref_inverse_dft(re_, im=None):
    import numpy as np
    X = np.asarray(re_, float) + 1j * (np.zeros(len(re_)) if im is None else np.asarray(im, float))
    s = float(np.sum(np.abs(X))) / len(X)
    x = [_snap(z, s) for z in np.fft.ifft(X)]
    return {"real": [z.real for z in x], "imag": [z.imag for z in x]}


_p("inverse_dft", name="Inverse discrete Fourier transform", topic="DFT and FFT",
   purpose="x[n] = (1/N) sum_k X[k] exp(+j 2 pi k n / N), from the real and imaginary parts of X[k].",
   inputs=[_i("re_", "real parts of X[0..N-1]", type="vector", min_length=1, max_length=65536),
           _i("im", "imaginary parts of X[0..N-1] (zeros if not given)", type="vector", required=False, min_length=1, max_length=65536)],
   outputs=[_o("real", "real parts of x[n]"), _o("imag", "imaginary parts of x[n]")],
   primary="real", conditions=["both lists have the same length"], method="inverse FFT (conjugate twiddles) divided by N",
   latex=r"x[n] = \frac{1}{N}\sum_{k=0}^{N-1} X[k]\,e^{j2\pi kn/N}", run=inverse_dft, reference=ref_inverse_dft,
   wrong=lambda re_, im=None: (lambda r, N: {"real": [v * N for v in r["real"]], "imag": [v * N for v in r["imag"]]})(inverse_dft(re_, im), len(re_)),
   random_inputs=lambda rng: (lambda n: {"re_": [round(rng.uniform(-10, 10), 3) for _ in range(n)], "im": [round(rng.uniform(-10, 10), 3) for _ in range(n)]})(rng.choice([4, 8, 7, 16, 12])),
   parse=lambda q: _parse(q, r"inverse (?:dft|fft|discrete fourier transform)|\bidft\b|\bifft\b",
                          [("re_", "vec", [r"real parts?", r"\bre(?=\s*=)"]), ("im", "vec", [r"imaginary parts?", r"\bim(?=\s*=)"])]),
   templates=["Compute the inverse DFT with real parts [10, -2, -2, -2] and imaginary parts [0, 2, 0, -2]."],
   examples=[{"inputs": {"re_": "10, -2, -2, -2", "im": "0, 2, 0, -2"}, "outputs": {"real": [1.0, 2.0, 3.0, 4.0], "imag": [0.0, 0.0, 0.0, 0.0]}},
             {"inputs": {"re_": [4, 0, 0, 0]}, "outputs": {"real": [1.0, 1.0, 1.0, 1.0]}}],
   questions=[{"q": "Compute the inverse DFT with real parts [10, -2, -2, -2] and imaginary parts [0, 2, 0, -2].", "outputs": {"real": [1.0, 2.0, 3.0, 4.0]}},
              {"q": "Compute the inverse DFT with real parts [4, 0, 0, 0].", "outputs": {"real": [1.0, 1.0, 1.0, 1.0]}},
              {"q": "Compute the inverse DFT with real parts [1, 2] and imaginary parts [1, 2, 3].", "outcome": "INVALID_INPUT"}])


# =================================================================================================== 3. convolution and correlation
def linear_convolution(x, h):
    x, h = _vec(x, "x", 1, 20000), _vec(h, "h", 1, 20000)
    n = len(x) + len(h) - 1
    y = [math.fsum(x[k] * h[i - k] for k in range(max(0, i - len(h) + 1), min(i, len(x) - 1) + 1)) for i in range(n)]
    return {"y": y, "length": n}


def _rv(rng, lo, hi):
    return [rng.choice([-1, 1]) * round(rng.uniform(0.5, 5), rng.choice([0, 1, 4])) for _ in range(rng.randint(lo, hi))]


_V2 = lambda a, b: [(a, "vec", [r"\b" + a + r"(?:\[n\])?(?=\s*[=:])", ""]), (b, "vec", [r"\b" + b + r"(?:\[n\])?(?=\s*[=:])", ""])]   # noqa: E731

_p("linear_convolution", name="Linear convolution of two sequences", topic="convolution and correlation",
   purpose="y[n] = sum_k x[k] h[n-k], the full linear convolution (length Nx + Nh - 1).",
   inputs=[_i("x", "first sequence x[n]", type="vector", min_length=1, max_length=20000), _i("h", "second sequence h[n]", type="vector", min_length=1, max_length=20000)],
   outputs=[_o("y", "convolution y[n], n = 0 .. Nx+Nh-2"), _o("length", "number of output samples")],
   primary="y", conditions=["finite sequences starting at n = 0"], method="direct convolution sum (compensated)",
   latex=r"y[n] = \sum_k x[k]\,h[n-k]", run=linear_convolution,
   reference=lambda x, h: {"y": [float(v) for v in __import__("numpy").convolve(x, h)], "length": len(x) + len(h) - 1},
   wrong=lambda x, h: linear_convolution(x, list(reversed(h))),
   random_inputs=lambda rng: {"x": _rv(rng, 2, 20), "h": (lambda v: v if v != v[::-1] else v + [1.0])(_rv(rng, 2, 8))},
   parse=lambda q: _parse(q, r"convol", _V2("x", "h"), exclude=r"circular|cyclic|periodic", require=r"\["),
   templates=["Compute the linear convolution of x = [1, 2, 3] and h = [1, -1].", "Convolve [1, 1, 1] with [1, 2]."],
   examples=[{"inputs": {"x": "1, 2, 3", "h": "1, -1"}, "outputs": {"y": [1.0, 1.0, 1.0, -3.0], "length": 4}},
             {"inputs": {"x": [1, 1, 1], "h": [1, 2]}, "outputs": {"y": [1.0, 3.0, 3.0, 2.0]}}],
   questions=[{"q": "Compute the linear convolution of x = [1, 2, 3] and h = [1, -1].", "outputs": {"y": [1.0, 1.0, 1.0, -3.0]}},
              {"q": "Convolve [1, 1, 1] with [1, 2].", "outputs": {"y": [1.0, 3.0, 3.0, 2.0]}},
              {"q": "Compute the linear convolution of x = [1, 2, 3].", "outcome": "NEEDS_INPUT"}])


def circular_convolution(x, h):
    x, h = _vec(x, "x", 1, 20000), _vec(h, "h", 1, 20000)
    if len(x) != len(h):
        raise ValueError("circular convolution needs two sequences of the same length N")
    N = len(x)
    return {"y": [math.fsum(x[k] * h[(n - k) % N] for k in range(N)) for n in range(N)]}


def ref_circular_convolution(x, h):
    import numpy as np
    return {"y": [float(v) for v in np.real(np.fft.ifft(np.fft.fft(x) * np.fft.fft(h)))]}


_p("circular_convolution", name="Circular (cyclic) convolution", topic="convolution and correlation",
   purpose="The N-point circular convolution y[n] = sum_k x[k] h[(n-k) mod N] of two sequences of length N.",
   inputs=[_i("x", "first sequence (length N)", type="vector", min_length=1, max_length=20000), _i("h", "second sequence (length N)", type="vector", min_length=1, max_length=20000)],
   outputs=[_o("y", "circular convolution y[0..N-1]")], primary="y", conditions=["both sequences have length N"],
   method="direct cyclic sum (indices modulo N)", latex=r"y[n] = \sum_{k=0}^{N-1} x[k]\,h[(n-k)\bmod N]",
   run=circular_convolution, reference=ref_circular_convolution,
   wrong=lambda x, h: {"y": linear_convolution(x, h)["y"][:len(x)]},
   random_inputs=lambda rng: (lambda n: {"x": [round(rng.uniform(-5, 5), 2) for _ in range(n)], "h": [round(rng.uniform(-5, 5), 2) for _ in range(n)]})(rng.randint(3, 24)),
   parse=lambda q: _parse(q, r"circular convolution|cyclic convolution|periodic convolution|circularly convolve", _V2("x", "h")),
   templates=["Compute the circular convolution of x = [1, 2, 3, 4] and h = [1, 0, 0, 1]."],
   examples=[{"inputs": {"x": "1, 2, 3, 4", "h": "1, 0, 0, 1"}, "outputs": {"y": [3.0, 5.0, 7.0, 5.0]}},
             {"inputs": {"x": [1, 2, 3], "h": [0, 1, 0]}, "outputs": {"y": [3.0, 1.0, 2.0]}}],
   questions=[{"q": "Compute the circular convolution of x = [1, 2, 3, 4] and h = [1, 0, 0, 1].", "outputs": {"y": [3.0, 5.0, 7.0, 5.0]}},
              {"q": "Compute the circular convolution of x = [1, 2, 3] and h = [1, 2].", "outcome": "INVALID_INPUT"}])


def cross_correlation(x, y):
    x, y = _vec(x, "x", 1, 20000), _vec(y, "y", 1, 20000)
    lags = list(range(-(len(y) - 1), len(x)))
    r = [math.fsum(x[n + L] * y[n] for n in range(max(0, -L), min(len(y), len(x) - L))) for L in lags]
    i = _peak(r, max(abs(v) for v in r))
    return {"peak_lag": lags[i], "peak_value": r[i], "r": r, "lags": lags}


def ref_cross_correlation(x, y):
    import numpy as np
    from scipy import signal
    r = [float(v) for v in signal.correlate(np.asarray(x, float), np.asarray(y, float), mode="full", method="direct")]
    lags = [int(v) for v in signal.correlation_lags(len(x), len(y), mode="full")]
    i = _peak(r, max(abs(v) for v in r))
    return {"peak_lag": lags[i], "peak_value": r[i], "r": r, "lags": lags}


_p("cross_correlation", name="Cross-correlation and the lag of its peak", topic="convolution and correlation",
   purpose="r_xy[l] = sum_n x[n+l] y[n] for every lag l, and the lag at which it is largest (the delay of x relative to y).",
   inputs=[_i("x", "first sequence x[n]", type="vector", min_length=1, max_length=20000), _i("y", "second (reference) sequence y[n]", type="vector", min_length=1, max_length=20000)],
   outputs=[_o("peak_lag", "lag of the largest correlation (positive: x lags y)"), _o("peak_value", "largest correlation value"),
            _o("r", "correlation at every lag"), _o("lags", "the lags, -(Ny-1) .. Nx-1")],
   primary="peak_lag", conditions=["finite sequences starting at n = 0"], method="direct correlation sum over every lag (first lag on ties)",
   latex=r"r_{xy}[\ell] = \sum_n x[n+\ell]\,y[n]", run=cross_correlation, reference=ref_cross_correlation,
   wrong=lambda x, y: (lambda r: {"peak_lag": -r["peak_lag"], "peak_value": r["peak_value"], "r": r["r"], "lags": [-v for v in r["lags"]]})(cross_correlation(y, x)),
   random_inputs=lambda rng: (lambda y, d: {"y": y, "x": [0.0] * d + [v + round(rng.uniform(-0.3, 0.3), 2) for v in y]})(_rv(rng, 4, 12), rng.randint(1, 6)),
   parse=lambda q: _parse(q, r"cross[- ]?correlat|correlate", _V2("x", "y"), exclude=r"auto[- ]?correlat|coefficient", require=r"\["),
   templates=["Find the cross-correlation of x = [0, 0, 1, 2, 1] and y = [1, 2, 1] and the lag of its peak."],
   examples=[{"inputs": {"x": "0, 0, 1, 2, 1", "y": "1, 2, 1"}, "outputs": {"peak_lag": 2, "peak_value": 6.0}},
             {"inputs": {"x": [1, 2, 3], "y": [1, 1]}, "outputs": {"r": [1.0, 3.0, 5.0, 3.0], "lags": [-1, 0, 1, 2], "peak_lag": 1}}],
   questions=[{"q": "Find the cross-correlation of x = [0, 0, 1, 2, 1] and y = [1, 2, 1] and the lag of its peak.", "outputs": {"peak_lag": 2}},
              {"q": "Cross-correlate x = [1, 2, 3] with y = [1, 1].", "outputs": {"r": [1.0, 3.0, 5.0, 3.0], "peak_lag": 1}}])


# =================================================================================================== 4. digital filters
def moving_average(x, M):
    x = _vec(x, "signal x", 1, 100000)
    M = _int(M, "window length M", 1, 100000)
    return {"y": [math.fsum(x[n - i] for i in range(M) if n - i >= 0) / M for n in range(len(x))]}


_p("moving_average", name="Moving-average filter output", topic="digital filters",
   purpose="The causal M-point moving average y[n] = (1/M) sum_{i=0}^{M-1} x[n-i], with x[n] = 0 before the first sample.",
   inputs=[_i("x", "input samples", type="vector", min_length=1, max_length=100000), _i("M", "window length (points averaged)", type="integer", min=1)],
   outputs=[_o("y", "output samples (same length as x)")], primary="y", conditions=["M >= 1", "zero initial conditions"],
   method="direct sum of the last M samples divided by M", latex=r"y[n] = \frac{1}{M}\sum_{i=0}^{M-1} x[n-i]",
   run=moving_average,
   reference=lambda x, M: {"y": [float(v) for v in __import__("scipy.signal", fromlist=["lfilter"]).lfilter([1.0 / M] * M, [1.0], x)]},
   wrong=lambda x, M: {"y": [math.fsum(x[max(0, n - M + 1):n + 1]) / min(M, n + 1) for n in range(len(x))]},
   random_inputs=lambda rng: (lambda x: {"x": x, "M": rng.randint(2, len(x))})(_rv(rng, 3, 30)),
   parse=lambda q: _parse(q, r"moving[- ]average|running average|boxcar filter", [("M", "int", ["^-?\\s?point", "window (?:length|size)(?: of)?", r"\bM(?=\s*=)", "over"]),
                                                                                  ("x", "vec", [r"\bx(?:\[n\])?(?=\s*[=:])", "input", "signal", "sequence", ""])]),
   templates=["Apply a 3-point moving average to x = [3, 6, 9, 12]."],
   examples=[{"inputs": {"x": "3, 6, 9, 12", "M": 3}, "outputs": {"y": [1.0, 3.0, 6.0, 9.0]}},
             {"inputs": {"x": [2, 4], "M": 2}, "outputs": {"y": [1.0, 3.0]}}],
   questions=[{"q": "Apply a 3-point moving average to x = [3, 6, 9, 12].", "outputs": {"y": [1.0, 3.0, 6.0, 9.0]}},
              {"q": "Apply a moving average with a window length of 2 to the signal [2, 4].", "outputs": {"y": [1.0, 3.0]}},
              {"q": "Apply a moving average to x = [3, 6, 9, 12].", "outcome": "NEEDS_INPUT"}])


def _lfilter(b, a, x):
    if a[0] == 0:
        raise ValueError("a[0] must not be 0")
    y = []
    for n in range(len(x)):
        acc = math.fsum([b[k] * x[n - k] for k in range(len(b)) if n - k >= 0] + [-a[k] * y[n - k] for k in range(1, len(a)) if n - k >= 0])
        y.append(acc / a[0])
    return y


def fir_filter_output(b, x):
    b, x = _vec(b, "coefficients b", 1, 10000), _vec(x, "input x", 1, 100000)
    return {"y": _lfilter(b, [1.0], x)}


_SIG = r"scipy.signal"
_p("fir_filter_output", name="FIR filter output", topic="digital filters",
   purpose="The output y[n] = sum_k b[k] x[n-k] of an FIR filter with coefficients b (zero initial conditions, same length as x).",
   inputs=[_i("b", "filter coefficients (taps) b[0..M]", type="vector", min_length=1, max_length=10000), _i("x", "input samples", type="vector", min_length=1, max_length=100000)],
   outputs=[_o("y", "output samples")], primary="y", conditions=["zero initial conditions"], method="direct-form FIR difference equation",
   latex=r"y[n] = \sum_{k=0}^{M} b_k\,x[n-k]", run=fir_filter_output,
   reference=lambda b, x: {"y": [float(v) for v in __import__(_SIG, fromlist=["lfilter"]).lfilter(b, [1.0], x)]},
   wrong=lambda b, x: fir_filter_output(list(reversed(b)), x),
   random_inputs=lambda rng: {"b": (lambda v: v if v != v[::-1] else v + [0.5])(_rv(rng, 2, 6)), "x": _rv(rng, 3, 30)},
   parse=lambda q: _parse(q, r"\bfir\b", [("b", "vec", [r"\bb(?=\s*[=:])", "coefficients", "taps", r"\bh(?:\[n\])?(?=\s*[=:])"]),
                                         ("x", "vec", [r"\bx(?:\[n\])?(?=\s*[=:])", "input", "signal", "sequence"])],
                          exclude=r"design|window|cutoff|frequency response|magnitude|group delay|\ba\s*=|impulse response|step response|dc gain|lowpass|low-pass|highpass|high-pass"),
   templates=["Filter x = [1, 2, 3, 4] with the FIR filter b = [0.5, 0.5]."],
   examples=[{"inputs": {"b": "0.5, 0.5", "x": "1, 2, 3, 4"}, "outputs": {"y": [0.5, 1.5, 2.5, 3.5]}},
             {"inputs": {"b": [1, -1], "x": [1, 4, 9, 16]}, "outputs": {"y": [1.0, 3.0, 5.0, 7.0]}}],
   questions=[{"q": "Filter x = [1, 2, 3, 4] with the FIR filter b = [0.5, 0.5].", "outputs": {"y": [0.5, 1.5, 2.5, 3.5]}},
              {"q": "What is the output of the FIR filter b = [1, -1] for the input x = [1, 4, 9, 16]?", "outputs": {"y": [1.0, 3.0, 5.0, 7.0]}},
              {"q": "Filter x = [1, 2, 3, 4] with an FIR filter.", "outcome": "NEEDS_INPUT"}])


def iir_filter_output(b, a, x):
    b, a, x = _vec(b, "b", 1, 1000), _vec(a, "a", 1, 1000), _vec(x, "input x", 1, 100000)
    return {"y": _lfilter(b, a, x)}


def _stable_a(rng, order):
    """Denominator coefficients of a stable filter (poles drawn inside the unit circle)."""
    poles = []
    while len(poles) < order:
        if order - len(poles) >= 2 and rng.random() < 0.5:
            r, t = rng.uniform(0.2, 0.9), rng.uniform(0.3, 2.8)
            poles += [cmath.rect(r, t), cmath.rect(r, -t)]
        else:
            poles.append(complex(rng.uniform(-0.9, 0.9), 0))
    c = [1 + 0j]
    for p in poles:
        c = [u - p * v for u, v in zip(c + [0j], [0j] + c)]
    return [round(v.real, 12) for v in c]


_p("iir_filter_output", name="IIR filter output (difference equation)", topic="digital filters",
   purpose="The output of the recursive filter a[0] y[n] = sum_k b[k] x[n-k] - sum_{k>=1} a[k] y[n-k] (zero initial conditions).",
   inputs=[_i("b", "feed-forward coefficients b", type="vector", min_length=1, max_length=1000), _i("a", "feedback coefficients a (a[0] != 0)", type="vector", min_length=1, max_length=1000),
           _i("x", "input samples", type="vector", min_length=1, max_length=100000)],
   outputs=[_o("y", "output samples")], primary="y", conditions=["a[0] != 0", "zero initial conditions"],
   method="direct-form I difference equation", latex=r"a_0 y[n] = \sum_k b_k x[n-k] - \sum_{k\ge 1} a_k y[n-k]",
   run=iir_filter_output,
   reference=lambda b, a, x: {"y": [float(v) for v in __import__(_SIG, fromlist=["lfilter"]).lfilter(b, a, x)]},
   wrong=lambda b, a, x: {"y": _lfilter(b, [a[0]] + [-v for v in a[1:]], x)},
   random_inputs=lambda rng: {"b": _rv(rng, 1, 4), "a": _stable_a(rng, rng.randint(1, 3)), "x": [1.0] + _rv(rng, 3, 20)},
   parse=lambda q: _parse(q, r"\biir\b|difference equation|recursive filter|filter with b\s*=",
                          [("b", "vec", [r"\bb(?=\s*[=:])"]), ("a", "vec", [r"\ba(?=\s*[=:])"]), ("x", "vec", [r"\bx(?:\[n\])?(?=\s*[=:])", "input", "signal", "sequence"])],
                          exclude=r"impulse response|step response|frequency response|magnitude|poles?\b|zeros?\b|group delay|dc gain|gain at"),
   templates=["Find the output of the IIR filter with b = [1] and a = [1, -0.5] for the input x = [1, 0, 0, 0]."],
   examples=[{"inputs": {"b": "1", "a": "1, -0.5", "x": "1, 0, 0, 0"}, "outputs": {"y": [1.0, 0.5, 0.25, 0.125]}},
             {"inputs": {"b": [1, 1], "a": [2, -1], "x": [2, 2, 2]}, "outputs": {"y": [1.0, 2.5, 3.25]}}],
   questions=[{"q": "Find the output of the IIR filter with b = [1] and a = [1, -0.5] for the input x = [1, 0, 0, 0].", "outputs": {"y": [1.0, 0.5, 0.25, 0.125]}},
              {"q": "Find the output of the IIR filter with b = [1] and a = [0, 1] for the input x = [1, 0].", "outcome": "INVALID_INPUT"},
              {"q": "Find the output of the IIR filter with b = [1] for the input x = [1, 0, 0, 0].", "outcome": "NEEDS_INPUT"}])


def _pad2(b, a):
    L = max(len(b), len(a))
    return list(b) + [0.0] * (L - len(b)), list(a) + [0.0] * (L - len(a))


def impulse_response(b, a, n):
    b, a = _vec(b, "b", 1, 1000), _vec(a, "a", 1, 1000)
    n = _int(n, "number of samples", 1, 100000)
    return {"h": _lfilter(b, a, [1.0] + [0.0] * (n - 1))}


def ref_impulse_response(b, a, n):
    from scipy import signal
    bp, ap = _pad2(b, a)                               # equal lengths: the z and z^-1 forms coincide
    _, (y,) = signal.dimpulse((bp, ap, 1), n=n)
    return {"h": [float(v) for v in y.ravel()]}


def step_response(b, a, n):
    b, a = _vec(b, "b", 1, 1000), _vec(a, "a", 1, 1000)
    n = _int(n, "number of samples", 1, 100000)
    return {"s": _lfilter(b, a, [1.0] * n)}


def ref_step_response(b, a, n):
    from scipy import signal
    bp, ap = _pad2(b, a)
    _, (y,) = signal.dstep((bp, ap, 1), n=n)
    return {"s": [float(v) for v in y.ravel()]}


_BA = [("b", "vec", [r"\bb(?=\s*[=:])", "numerator"]), ("a", "vec", [r"\ba(?=\s*[=:])", "denominator"])]
_p("impulse_response", name="Impulse response of a digital filter", topic="digital filters",
   purpose="The first n samples of the impulse response h[n] of the filter with coefficients b and a.",
   inputs=[_i("b", "numerator coefficients b", type="vector", min_length=1, max_length=1000), _i("a", "denominator coefficients a (a[0] != 0)", type="vector", min_length=1, max_length=1000),
           _i("n", "number of samples", type="integer", min=1)],
   outputs=[_o("h", "impulse response h[0..n-1]")], primary="h", conditions=["a[0] != 0"],
   method="the difference equation driven by a unit impulse", latex=r"h[n] = \mathcal{Z}^{-1}\{B(z)/A(z)\}",
   run=impulse_response, reference=ref_impulse_response,
   wrong=lambda b, a, n: {"h": [0.0] + impulse_response(b, a, n)["h"][:-1]},
   random_inputs=lambda rng: {"b": _rv(rng, 1, 4), "a": _stable_a(rng, rng.randint(1, 3)), "n": rng.randint(2, 30)},
   parse=lambda q: _parse(q, r"impulse response", _BA + [("n", "int", ["first", r"\bn(?=\s*=)"])],
                          exclude=r"frequency response|magnitude|\bfir\b"),
   templates=["Find the first 5 samples of the impulse response of the filter with b = [1] and a = [1, -0.5]."],
   examples=[{"inputs": {"b": "1", "a": "1, -0.5", "n": 5}, "outputs": {"h": [1.0, 0.5, 0.25, 0.125, 0.0625]}},
             {"inputs": {"b": [1, 1], "a": [1, -1, 0.5], "n": 4}, "outputs": {"h": [1.0, 2.0, 1.5, 0.5]}}],
   questions=[{"q": "Find the first 5 samples of the impulse response of the filter with b = [1] and a = [1, -0.5].", "outputs": {"h": [1.0, 0.5, 0.25, 0.125, 0.0625]}},
              {"q": "Find the impulse response of the filter with b = [1] and a = [1, -0.5].", "outcome": "NEEDS_INPUT"}])

_p("step_response", name="Step response of a digital filter", topic="digital filters",
   purpose="The first n samples of the response s[n] of the filter (b, a) to a unit step.",
   inputs=[_i("b", "numerator coefficients b", type="vector", min_length=1, max_length=1000), _i("a", "denominator coefficients a (a[0] != 0)", type="vector", min_length=1, max_length=1000),
           _i("n", "number of samples", type="integer", min=1)],
   outputs=[_o("s", "step response s[0..n-1]")], primary="s", conditions=["a[0] != 0"],
   method="the difference equation driven by a unit step", latex=r"s[n] = \sum_{m=0}^{n} h[m]",
   run=step_response, reference=ref_step_response,
   wrong=lambda b, a, n: {"s": impulse_response(b, a, n)["h"]},
   random_inputs=lambda rng: {"b": _rv(rng, 1, 4), "a": _stable_a(rng, rng.randint(1, 3)), "n": rng.randint(2, 30)},
   parse=lambda q: _parse(q, r"step response", _BA + [("n", "int", ["first", r"\bn(?=\s*=)"])], exclude=r"frequency response|magnitude"),
   templates=["Find the first 4 samples of the step response of the filter with b = [1] and a = [1, -0.5]."],
   examples=[{"inputs": {"b": "1", "a": "1, -0.5", "n": 4}, "outputs": {"s": [1.0, 1.5, 1.75, 1.875]}}],
   questions=[{"q": "Find the first 4 samples of the step response of the filter with b = [1] and a = [1, -0.5].", "outputs": {"s": [1.0, 1.5, 1.75, 1.875]}}])


def _poly_at(c, w):
    """sum_k c[k] e^{-j w k} and sum_k k c[k] e^{-j w k}, compensated."""
    t = [ck * cmath.exp(-1j * w * k) for k, ck in enumerate(c)]
    td = [k * z for k, z in enumerate(t)]
    s = complex(math.fsum(z.real for z in t), math.fsum(z.imag for z in t))
    sd = complex(math.fsum(z.real for z in td), math.fsum(z.imag for z in td))
    return s, sd


def frequency_response(b, f, fs, a=None):
    b = _vec(b, "b", 1, 10000)
    a = [1.0] if a is None else _vec(a, "a", 1, 10000)
    fs, f = _pos(fs, "sampling rate"), _nonneg(f, "frequency")
    if f > fs / 2 * (1 + 1e-12):
        raise ValueError("the frequency must lie between 0 and fs/2")
    w = 2 * math.pi * f / fs
    B = _snap(_poly_at(b, w)[0], math.fsum(abs(v) for v in b))
    A = _snap(_poly_at(a, w)[0], math.fsum(abs(v) for v in a))
    if A == 0:
        raise ValueError("the denominator vanishes at this frequency (a pole on the unit circle)")
    H = B / A
    if H == 0:
        return {"magnitude": 0.0, "magnitude_db": -INF, "phase": 0.0}
    return {"magnitude": abs(H), "magnitude_db": _db20(abs(H)), "phase": math.atan2(H.imag, H.real)}


def ref_frequency_response(b, f, fs, a=None):
    import numpy as np
    from scipy import signal
    _, h = signal.freqz(b, [1.0] if a is None else a, worN=[f], fs=fs)
    return {"magnitude": float(abs(h[0])), "magnitude_db": float(20 * np.log10(abs(h[0]))), "phase": float(np.angle(h[0]))}


def _rand_ba(rng):
    return {"b": _rv(rng, 1, 5), "a": _stable_a(rng, rng.randint(1, 3)), "fs": _logu(None, 100, 1e6, rng)}


_BAF = _BA + [("fs", "kind:frequency", _FS), ("f", "kind:frequency", ["at", "frequency of", "frequency", ""])]
_p("frequency_response", name="Frequency response of a digital filter at one frequency", topic="digital filters",
   purpose="H(e^{jw}) = B(e^{jw}) / A(e^{jw}) at w = 2 pi f / fs: magnitude, magnitude in dB and phase.",
   inputs=[_i("b", "numerator coefficients b", type="vector", min_length=1, max_length=10000), _i("f", "frequency", "frequency"),
           _i("fs", "sampling rate", "frequency"), _i("a", "denominator coefficients a (1 if not given)", type="vector", required=False, min_length=1, max_length=10000)],
   outputs=[_o("magnitude", "|H|"), _o("magnitude_db", "20 log10 |H| in dB"), _o("phase", "phase of H in radians", "angle")],
   primary="magnitude", conditions=["0 <= f <= fs/2", "no pole on the unit circle at f"],
   method="evaluation of the numerator and denominator polynomials in z^-1 on the unit circle",
   latex=r"H(e^{j\omega}) = \frac{\sum_k b_k e^{-j\omega k}}{\sum_k a_k e^{-j\omega k}},\ \omega = 2\pi f/f_s",
   run=frequency_response, reference=ref_frequency_response,
   wrong=lambda b, f, fs, a=None: frequency_response(b, f, fs, None),          # the denominator forgotten
   random_inputs=lambda rng: (lambda d: dict(d, f=rng.uniform(0.05, 0.45) * d["fs"]))(_rand_ba(rng)),
   parse=lambda q: _parse(q, r"(?:magnitude|frequency|amplitude|phase) response|\bgain\b[^.?]*\bat\b", _BAF,
                          exclude=r"group delay|impulse|step response|dc gain|butterworth|\brc\b|resistor|capacitor|antenna"),
   templates=["What is the magnitude response of the filter b = [1, 1] at 1 kHz with a sampling rate of 8 kHz?",
              "Find the frequency response of the filter with b = [1] and a = [1, -0.9] at 0 Hz with a sampling rate of 1 kHz."],
   examples=[{"inputs": {"b": "1, 1", "f": 1000.0, "fs": 8000.0}, "outputs": {"magnitude": 2 * math.cos(math.pi / 8), "phase": -math.pi / 8}},
             {"inputs": {"b": [1], "a": [1, -0.9], "f": 0.0, "fs": 1000.0}, "outputs": {"magnitude": 10.0, "magnitude_db": 20.0, "phase": 0.0}},
             {"inputs": {"b": [1, 1], "f": 4000.0, "fs": 8000.0}, "outputs": {"magnitude": 0.0}}],
   questions=[{"q": "What is the magnitude response of the filter b = [1, 1] at 1 kHz with a sampling rate of 8 kHz?", "outputs": {"magnitude": 2 * math.cos(math.pi / 8)}},
              {"q": "Find the frequency response of the filter with b = [1] and a = [1, -0.9] at 0 Hz with a sampling rate of 1 kHz.", "outputs": {"magnitude_db": 20.0}},
              {"q": "What is the magnitude response of the filter b = [1, 1] at 5 kHz with a sampling rate of 8 kHz?", "outcome": "INVALID_INPUT"},
              {"q": "What is the magnitude response of the filter b = [1, 1] at 1 kHz?", "outcome": "NEEDS_INPUT"}])


def group_delay(b, f, fs, a=None):
    b = _vec(b, "b", 1, 10000)
    a = [1.0] if a is None else _vec(a, "a", 1, 10000)
    fs, f = _pos(fs, "sampling rate"), _nonneg(f, "frequency")
    if f > fs / 2 * (1 + 1e-12):
        raise ValueError("the frequency must lie between 0 and fs/2")
    w = 2 * math.pi * f / fs
    (B, Bd), (A, Ad) = _poly_at(b, w), _poly_at(a, w)
    if abs(B) <= 1e-12 * math.fsum(abs(v) for v in b) or abs(A) <= 1e-12 * math.fsum(abs(v) for v in a):
        raise ValueError("the group delay is undefined at a zero or pole on the unit circle")
    tau = (Bd / B).real - (Ad / A).real
    return {"samples": tau, "seconds": tau / fs}


def ref_group_delay(b, f, fs, a=None):
    from scipy import signal
    _, gd = signal.group_delay((b, [1.0] if a is None else a), w=[f], fs=fs)
    return {"samples": float(gd[0]), "seconds": float(gd[0]) / fs}


_p("group_delay", name="Group delay of a digital filter", topic="digital filters",
   purpose="The group delay -d(phase)/dw of H(z) = B(z)/A(z) at frequency f, in samples and in seconds.",
   inputs=[_i("b", "numerator coefficients b", type="vector", min_length=1, max_length=10000), _i("f", "frequency", "frequency"),
           _i("fs", "sampling rate", "frequency"), _i("a", "denominator coefficients a (1 if not given)", type="vector", required=False, min_length=1, max_length=10000)],
   outputs=[_o("samples", "group delay in samples"), _o("seconds", "group delay in seconds", "time")],
   primary="samples", conditions=["0 <= f <= fs/2", "no zero or pole of H on the unit circle at f"],
   method="tau = Re[sum k b_k z^-k / sum b_k z^-k] - Re[the same for a] on the unit circle",
   latex=r"\tau_g(\omega) = -\frac{d\angle H(e^{j\omega})}{d\omega} = \mathrm{Re}\frac{\sum k b_k e^{-j\omega k}}{\sum b_k e^{-j\omega k}} - \mathrm{Re}\frac{\sum k a_k e^{-j\omega k}}{\sum a_k e^{-j\omega k}}",
   run=group_delay, reference=ref_group_delay,
   wrong=lambda b, f, fs, a=None: (lambda w: (lambda P, Q: {"samples": (P[1] / P[0]).real + (Q[1] / Q[0]).real, "seconds": ((P[1] / P[0]).real + (Q[1] / Q[0]).real) / fs})(_poly_at(b, w), _poly_at([1.0] if a is None else a, w)))(2 * math.pi * f / fs),
   random_inputs=lambda rng: (lambda d: dict(d, f=rng.uniform(0.05, 0.45) * d["fs"], a=_stable_a(rng, rng.randint(1, 3))))(_rand_ba(rng)),
   parse=lambda q: _parse(q, r"group delay", _BAF, require=r"\["),
   templates=["What is the group delay of the filter b = [1, 2, 1] and a = [1, -0.5] at 1 kHz with a sampling rate of 8 kHz?",
              "Find the group delay of the FIR filter b = [1, 2, 3, 2, 1] at 500 Hz with a sampling rate of 8 kHz."],
   examples=[{"inputs": {"b": "1, 2, 3, 2, 1", "f": 500.0, "fs": 8000.0}, "outputs": {"samples": 2.0, "seconds": 2.0 / 8000}},
             {"inputs": {"b": [1], "a": [1, -0.5], "f": 0.0, "fs": 1000.0}, "outputs": {"samples": 1.0}}],
   questions=[{"q": "Find the group delay of the FIR filter b = [1, 2, 3, 2, 1] at 500 Hz with a sampling rate of 8 kHz.", "outputs": {"samples": 2.0}},
              {"q": "What is the group delay of the filter b = [1] and a = [1, -0.5] at 0 Hz with a sampling rate of 1 kHz?", "outputs": {"samples": 1.0}}])


def dc_gain(b, a=None):
    b = _vec(b, "b", 1, 10000)
    a = [1.0] if a is None else _vec(a, "a", 1, 10000)
    sa = math.fsum(a)
    if sa == 0:
        raise ValueError("the sum of the denominator coefficients is 0 (a pole at z = 1): the DC gain is infinite")
    g = math.fsum(b) / sa
    return {"gain": g, "gain_db": _db20(abs(g)) if g != 0 else -INF}


def ref_dc_gain(b, a=None):
    import numpy as np
    from scipy import signal
    _, h = signal.freqz(b, [1.0] if a is None else a, worN=[0.0])
    return {"gain": float(h[0].real), "gain_db": float(20 * np.log10(abs(h[0])))}


_p("dc_gain", name="DC gain of a digital filter", topic="digital filters",
   purpose="The gain H(1) = sum b / sum a of the filter at zero frequency, and its level in dB.",
   inputs=[_i("b", "numerator coefficients b", type="vector", min_length=1, max_length=10000),
           _i("a", "denominator coefficients a (1 if not given)", type="vector", required=False, min_length=1, max_length=10000)],
   outputs=[_o("gain", "DC gain H(z = 1) (signed)"), _o("gain_db", "20 log10 |H(1)| in dB")],
   primary="gain", conditions=["sum of a != 0 (no pole at z = 1)"], method="H(1) = sum b_k / sum a_k",
   latex=r"H(1) = \frac{\sum_k b_k}{\sum_k a_k}", run=dc_gain, reference=ref_dc_gain,
   wrong=lambda b, a=None: (lambda g: {"gain": g, "gain_db": _db20(abs(g)) if g else -INF})(b[0] / (1.0 if a is None else a[0])),
   random_inputs=lambda rng: (lambda b: {"b": b if abs(sum(b)) > 0.1 and b[0] * 1.0 != sum(b) else b + [1.0], "a": _stable_a(rng, rng.randint(1, 3))})(_rv(rng, 2, 5)),
   parse=lambda q: _parse(q, r"\bdc gain|gain at dc|zero[- ]frequency gain|gain at (?:zero|0) (?:frequency|hz)", _BA, exclude=r"\brc\b|butterworth", require=r"\["),
   templates=["What is the DC gain of the filter with b = [1, 2, 1] and a = [1, -0.5, 0.25]?"],
   examples=[{"inputs": {"b": "1, 2, 1", "a": "1, -0.5, 0.25"}, "outputs": {"gain": 4 / 0.75}},
             {"inputs": {"b": [0.25, 0.25, 0.25, 0.25]}, "outputs": {"gain": 1.0, "gain_db": 0.0}}],
   questions=[{"q": "What is the DC gain of the filter with b = [1, 2, 1] and a = [1, -0.5, 0.25]?", "outputs": {"gain": 4 / 0.75}},
              {"q": "What is the DC gain of the filter with b = [1] and a = [1, -1]?", "outcome": "INVALID_INPUT"}])


# =================================================================================================== 5. analog filters
def butterworth_magnitude(N, fc, f):
    N = _int(N, "order", 1, 30)
    fc, f = _pos(fc, "cutoff frequency"), _nonneg(f, "frequency")
    r = (f / fc) ** (2 * N)
    return {"magnitude": 1 / math.sqrt(1 + r), "attenuation_db": 10 * math.log1p(r) / math.log(10)}


def ref_butterworth_magnitude(N, fc, f):
    import numpy as np
    from scipy import signal
    b, a = signal.butter(N, 2 * np.pi * fc, analog=True)
    h = signal.freqs(b, a, worN=[2 * np.pi * f])[1][0]
    return {"magnitude": float(abs(h)), "attenuation_db": float(-20 * np.log10(abs(h)))}


_p("butterworth_magnitude", name="Magnitude of an analog Butterworth low-pass response", topic="analog filters",
   purpose="|H(f)| = 1/sqrt(1 + (f/fc)^(2N)) of an N-th order Butterworth low-pass filter, and the attenuation in dB.",
   inputs=[_i("N", "filter order", type="integer", min=1, max=30), _i("fc", "cutoff (-3 dB) frequency", "frequency"), _i("f", "frequency", "frequency")],
   outputs=[_o("magnitude", "|H(f)|"), _o("attenuation_db", "attenuation in dB (positive)")],
   primary="magnitude", conditions=["N >= 1", "fc > 0", "f >= 0"], method="the Butterworth magnitude-squared function",
   latex=r"|H(f)| = \frac{1}{\sqrt{1 + (f/f_c)^{2N}}}", run=butterworth_magnitude, reference=ref_butterworth_magnitude,
   wrong=lambda N, fc, f: (lambda r: {"magnitude": 1 / (1 + r), "attenuation_db": 20 * math.log10(1 + r)})((f / fc) ** (2 * N)),
   random_inputs=lambda rng: (lambda fc: {"N": rng.randint(1, 10), "fc": fc, "f": fc * _logu(None, 0.7, 5, rng)})(_logu(None, 1, 1e9, rng)),
   parse=lambda q: _parse(q, r"butterworth", [("N", "ord", ["order(?: of)?", r"\bN(?=\s*=)", "^-?\\s?order", "^-?\\s?pole"]),
                                             ("fc", "kind:frequency", ["cutoff frequency", "cut-off frequency", "cutoff", r"~+\s*frequency", "corner frequency", r"\bfc(?=\s*=)"]),
                                             ("f", "kind:frequency", ["at", "frequency of", "frequency", ""])],
                          exclude=r"(?:needed|required|minimum|what|which) (?:filter )?order|order (?:is )?(?:needed|required)|design|coefficients|stopband|passband|sampl|bilinear",
                          require=r"magnitude|\bgain\b|attenuation|response"),
   templates=["What is the magnitude response of a 4th-order Butterworth low-pass filter with a cutoff frequency of 1 kHz at 2 kHz?",
              "Find the attenuation of a Butterworth filter of order 3 with a cutoff of 10 kHz at 30 kHz."],
   examples=[{"inputs": {"N": 4, "fc": 1000.0, "f": 2000.0}, "outputs": {"magnitude": 1 / math.sqrt(257)}},
             {"inputs": {"N": 3, "fc": 1e4, "f": 3e4}, "outputs": {"attenuation_db": 10 * math.log10(730)}},
             {"inputs": {"N": 5, "fc": 1e3, "f": 1e3}, "outputs": {"magnitude": 1 / math.sqrt(2)}}],
   questions=[{"q": "What is the magnitude response of a 4th-order Butterworth low-pass filter with a cutoff frequency of 1 kHz at 2 kHz?", "outputs": {"magnitude": 1 / math.sqrt(257)}},
              {"q": "Find the attenuation of a Butterworth filter of order 3 with a cutoff of 10 kHz at 30 kHz.", "outputs": {"attenuation_db": 10 * math.log10(730)}},
              {"q": "Find the attenuation of a Butterworth filter of order 3 at 30 kHz.", "outcome": "NEEDS_INPUT"}])


def _order_args(fp, fstop, Rp, As):
    fp, fstop = _pos(fp, "passband edge frequency"), _pos(fstop, "stopband edge frequency")
    Rp, As = _pos(Rp, "maximum passband attenuation (dB)"), _pos(As, "minimum stopband attenuation (dB)")
    if fp == fstop:
        raise ValueError("the passband and stopband edges must differ")
    if As <= Rp:
        raise ValueError("the stopband attenuation must exceed the passband attenuation")
    return fp, fstop, Rp, As


def _butter_ord(nat, Rp, As):
    GSTOP, GPASS = 10 ** (0.1 * As), 10 ** (0.1 * Rp)
    return int(math.ceil(math.log10((GSTOP - 1.0) / (GPASS - 1.0)) / (2 * math.log10(nat))))


def _cheb_ord(nat, Rp, As):
    GSTOP, GPASS = 10 ** (0.1 * As), 10 ** (0.1 * Rp)
    return int(math.ceil(math.acosh(math.sqrt((GSTOP - 1.0) / (GPASS - 1.0))) / math.acosh(nat)))


def butterworth_order_analog(fp, fstop, Rp, As):
    fp, fstop, Rp, As = _order_args(fp, fstop, Rp, As)
    low = fp < fstop
    return {"order": _butter_ord(fstop / fp if low else fp / fstop, Rp, As), "type": "lowpass" if low else "highpass"}


def butterworth_order_digital(fp, fstop, fs, Rp, As):
    fp, fstop, Rp, As = _order_args(fp, fstop, Rp, As)
    fs = _pos(fs, "sampling rate")
    if max(fp, fstop) >= fs / 2:
        raise ValueError("both edge frequencies must lie below fs/2")
    wp, ws = math.tan(math.pi * fp / fs), math.tan(math.pi * fstop / fs)      # bilinear pre-warping
    low = fp < fstop
    return {"order": _butter_ord(ws / wp if low else wp / ws, Rp, As), "type": "lowpass" if low else "highpass"}


def chebyshev1_order_analog(fp, fstop, Rp, As):
    fp, fstop, Rp, As = _order_args(fp, fstop, Rp, As)
    low = fp < fstop
    return {"order": _cheb_ord(fstop / fp if low else fp / fstop, Rp, As), "type": "lowpass" if low else "highpass"}


def _rand_spec(rng, digital=False):
    fp = _logu(None, 10, 1e6, rng)
    fstop = fp * rng.uniform(1.3, 4)
    d = {"Rp": round(rng.uniform(0.1, 3), 2), "As": round(rng.uniform(20, 80), 1)}
    if rng.random() < 0.3:
        fp, fstop = fstop, fp
    if digital:
        d["fs"] = max(fp, fstop) * rng.uniform(2.2, 10)
    return dict(d, fp=fp, fstop=fstop)


_SPEC = [("fp", "kind:frequency", ["passband edge(?: frequency)?", "passband (?:frequency|corner)", "pass ?band (?:ends|extends to|up to)", "passband"]),
         ("fstop", "kind:frequency", ["stopband edge(?: frequency)?", "stopband (?:frequency|begins|starts)", "stop ?band (?:begins|starts)", "stopband"]),
         ("Rp", "db", ["(?:passband )?ripple(?: of)?", "at most", "no more than", "maximum (?:passband )?(?:attenuation|loss)(?: of)?", "passband attenuation(?: of)?"]),
         ("As", "db", ["at least", "minimum (?:stopband )?attenuation(?: of)?", "stopband attenuation(?: of)?", "attenuation(?: of)?"])]
_ORDQ = r"order"
for _key, _fn, _kind, _dig in (("butterworth_order_analog", butterworth_order_analog, "butterworth", False),
                               ("butterworth_order_digital", butterworth_order_digital, "butterworth", True),
                               ("chebyshev1_order_analog", chebyshev1_order_analog, "chebyshev", False)):
    _scipy_fn = "cheb1ord" if _kind == "chebyshev" else "buttord"
    _p(_key, name=("Minimum order of a " + ("digital (bilinear) " if _dig else "analog ") + ("Butterworth" if _kind == "butterworth" else "Chebyshev type I") + " filter"),
       topic="analog filters" if not _dig else "digital filters",
       purpose="The smallest order meeting a passband edge with at most Rp dB attenuation and a stopband edge with at least As dB"
               + (" (digital design by the bilinear transform: edges pre-warped with tan(pi f / fs))." if _dig else "."),
       inputs=[_i("fp", "passband edge frequency", "frequency"), _i("fstop", "stopband edge frequency", "frequency")]
              + ([_i("fs", "sampling rate", "frequency")] if _dig else [])
              + [_i("Rp", "maximum passband attenuation (ripple) in dB"), _i("As", "minimum stopband attenuation in dB")],
       outputs=[_o("order", "minimum filter order"), _o("type", "lowpass (fp < fstop) or highpass (fp > fstop)")],
       primary="order", conditions=["0 < Rp < As", "fp != fstop"] + (["both edges below fs/2"] if _dig else []),
       method=("N = ceil( log10((10^(As/10)-1)/(10^(Rp/10)-1)) / (2 log10(ws/wp)) )" if _kind == "butterworth" else
               "N = ceil( acosh(sqrt((10^(As/10)-1)/(10^(Rp/10)-1))) / acosh(ws/wp) )") + (", with wp = tan(pi fp/fs), ws = tan(pi fstop/fs)" if _dig else ""),
       latex=(r"N \ge \frac{\log_{10}\frac{10^{A_s/10}-1}{10^{R_p/10}-1}}{2\log_{10}(\omega_s/\omega_p)}" if _kind == "butterworth" else
              r"N \ge \frac{\cosh^{-1}\sqrt{\frac{10^{A_s/10}-1}{10^{R_p/10}-1}}}{\cosh^{-1}(\omega_s/\omega_p)}"),
       run=_fn,
       reference=((lambda fp, fstop, fs, Rp, As, _s=_scipy_fn: {"order": int(getattr(__import__(_SIG, fromlist=[_s]), _s)(fp, fstop, Rp, As, fs=fs)[0])}) if _dig else
                  (lambda fp, fstop, Rp, As, _s=_scipy_fn: {"order": int(getattr(__import__(_SIG, fromlist=[_s]), _s)(2 * math.pi * fp, 2 * math.pi * fstop, Rp, As, analog=True)[0])})),
       wrong=((lambda fp, fstop, fs, Rp, As, _g=_fn: (lambda r: dict(r, order=2 * r["order"] + 1))(_g(fp, fstop, fs, Rp, As))) if _dig else
              (lambda fp, fstop, Rp, As, _g=_fn: (lambda r: dict(r, order=2 * r["order"] + 1))(_g(fp, fstop, Rp, As)))),
       random_inputs=(lambda rng, _d=_dig: _rand_spec(rng, _d)),
       parse=(lambda q, _k=_kind, _d=_dig: _parse(q, _k, (_SPEC[:2] + [("fs", "kind:frequency", _FS)] + _SPEC[2:]) if _d else _SPEC,
                                                  require=_ORDQ, exclude=(r"chebyshev|elliptic|bessel" if _k == "butterworth" else r"butterworth|type ii|type 2|inverse|elliptic")
                                                  + ("" if _d else r"|sampl|digital|bilinear"))
              if (bool(re.search(r"sampl|digital|bilinear", q or "", re.I)) == _d) else None),
       templates=[f"What {'Butterworth' if _kind == 'butterworth' else 'Chebyshev'} filter order is needed for a passband edge of 1 kHz with at most 1 dB ripple and a stopband edge of 2 kHz with at least 40 dB attenuation"
                  + (" at a sampling rate of 10 kHz?" if _dig else "?")],
       examples=[{"inputs": dict({"fp": 1000.0, "fstop": 2000.0, "Rp": 1.0, "As": 40.0}, **({"fs": 10000.0} if _dig else {})),
                  "outputs": {"order": (_fn(1000.0, 2000.0, 10000.0, 1.0, 40.0) if _dig else _fn(1000.0, 2000.0, 1.0, 40.0))["order"], "type": "lowpass"}}],
       questions=[{"q": f"What {'Butterworth' if _kind == 'butterworth' else 'Chebyshev'} filter order is needed for a passband edge of 1 kHz with at most 1 dB ripple and a stopband edge of 2 kHz with at least 40 dB attenuation"
                        + (" at a sampling rate of 10 kHz?" if _dig else "?"),
                   "outputs": {"order": (_fn(1000.0, 2000.0, 10000.0, 1.0, 40.0) if _dig else _fn(1000.0, 2000.0, 1.0, 40.0))["order"]}},
                  {"q": f"What {'Butterworth' if _kind == 'butterworth' else 'Chebyshev'} filter order is needed for a passband edge of 1 kHz with at most 1 dB ripple and a stopband edge of 2 kHz"
                        + (" at a sampling rate of 10 kHz?" if _dig else "?"), "outcome": "NEEDS_INPUT"}])

# hand-checked values of the examples above (the classic textbook answers): analog Butterworth 1 kHz/2 kHz, 1 dB/40 dB -> 8;
# Chebyshev I -> 5
assert butterworth_order_analog(1000.0, 2000.0, 1.0, 40.0)["order"] == 8 and chebyshev1_order_analog(1000.0, 2000.0, 1.0, 40.0)["order"] == 5


# =================================================================================================== 6. z-domain
def _canon_roots(roots):
    """Real roots ascending (as floats), then complex roots by real part, the positive imaginary part first."""
    real = sorted(float(r.real) + 0.0 for r in roots if abs(r.imag) <= 1e-12 * max(1.0, abs(r)))
    cplx = sorted((complex(r.real + 0.0, r.imag + 0.0) for r in roots if abs(r.imag) > 1e-12 * max(1.0, abs(r))), key=lambda z: (round(z.real, 9), -z.imag))
    return real + cplx


def _roots(c):
    """All roots of c[0] x^n + ... + c[n] (leading zeros dropped, trailing zeros give roots at 0): Durand-Kerner
    iteration, then Newton polishing on the original polynomial."""
    c = [complex(v) for v in c]
    while c and c[0] == 0:
        c.pop(0)
    if not c:
        raise ValueError("the polynomial is identically zero")
    zeros = 0
    while len(c) > 1 and c[-1] == 0:
        c.pop()
        zeros += 1
    n = len(c) - 1
    out = [0j] * zeros
    if n == 0:
        return out
    a = [v / c[0] for v in c]
    if n == 1:
        return out + [-a[1]]
    if n == 2:
        d = cmath.sqrt(a[1] * a[1] - 4 * a[2])
        q = -(a[1] + (d if (a[1].conjugate() * d).real >= 0 else -d)) / 2
        return out + ([q, a[2] / q] if q != 0 else [0j, 0j])
    p = lambda z: sum(ak * z ** (n - k) for k, ak in enumerate(a))          # noqa: E731
    dp = lambda z: sum(ak * (n - k) * z ** (n - k - 1) for k, ak in enumerate(a[:-1]))   # noqa: E731
    rad = 1 + max(abs(v) for v in a[1:])
    z = [rad * cmath.exp(1j * (2 * math.pi * k / n + 0.4)) for k in range(n)]
    for _ in range(500):
        new = []
        for i in range(n):
            den = 1 + 0j
            for j in range(n):
                if j != i:
                    den *= (z[i] - z[j])
            new.append(z[i] - p(z[i]) / den if den != 0 else z[i] + 1e-9)
        done = max(abs(u - v) for u, v in zip(new, z)) <= 1e-15 * max(1.0, max(abs(v) for v in new))
        z = new
        if done:
            break
    pol = []
    for r in z:
        for _ in range(3):
            d = dp(r)
            if d == 0:
                break
            step = p(r) / d
            if abs(step) > 1e-6 * max(1.0, abs(r)):
                break
            r -= step
        pol.append(r)
    return out + pol


def second_order_poles(a1, a2):
    a1, a2 = _fin(a1, "a1"), _fin(a2, "a2")
    if a2 == 0:
        raise ValueError("a2 must not be 0 (otherwise the system is first order)")
    disc = a1 * a1 - 4 * a2
    if disc >= 0:
        q = -(a1 + math.copysign(math.sqrt(disc), a1)) / 2
        r = sorted([q, a2 / q])
        p = [r[0] + 0.0, r[1] + 0.0]
    else:
        re_, im = -a1 / 2 + 0.0, math.sqrt(-disc) / 2
        p = [complex(re_, im), complex(re_, -im)]
    rad = max(abs(v) for v in p)
    return {"radius": rad, "p1": p[0], "p2": p[1], "stability": "stable" if rad < 1 else "unstable"}


def ref_second_order_poles(a1, a2):
    from scipy import signal
    _, p, _ = signal.tf2zpk([1.0], [1.0, a1, a2])
    p = _canon_roots(list(p))
    rad = max(abs(v) for v in p)
    return {"radius": float(rad), "p1": p[0], "p2": p[1], "stability": "stable" if rad < 1 else "unstable"}


_UN = r"(?:\d+(?:\.\d*)?|\.\d+)(?:[eE][-+]?\d+)?"
_Z1 = r"z\s*\^\s*\(?\s*-\s*1\s*\)?"
_Z2 = r"z\s*\^\s*\(?\s*-\s*2\s*\)?"


def _hz2(text):
    m = re.search(r"1\s*/\s*\(\s*1\s*([+-])\s*(" + _UN + r")?\s*\*?\s*" + _Z1 + r"\s*([+-])\s*(" + _UN + r")?\s*\*?\s*" + _Z2 + r"\s*\)", text)
    if m:
        a1 = (1 if m.group(1) == "+" else -1) * float(m.group(2) or 1)
        a2 = (1 if m.group(3) == "+" else -1) * float(m.group(4) or 1)
        return {"a1": a1, "a2": a2}, [m.span()]
    m = re.search(r"1\s*/\s*\(\s*1\s*([+-])\s*(" + _UN + r")?\s*\*?\s*" + _Z2 + r"\s*\)", text)
    if m:
        return {"a1": 0.0, "a2": (1 if m.group(1) == "+" else -1) * float(m.group(2) or 1)}, [m.span()]
    return {}, []


_p("second_order_poles", name="Poles and stability of a second-order all-pole system", topic="z-transform and stability",
   purpose="The poles of H(z) = 1/(1 + a1 z^-1 + a2 z^-2) (roots of z^2 + a1 z + a2), their largest radius and whether the causal system is stable.",
   inputs=[_i("a1", "coefficient of z^-1 in the denominator"), _i("a2", "coefficient of z^-2 in the denominator", condition="a2 != 0")],
   outputs=[_o("radius", "largest pole radius |p|"), _o("p1", "first pole (smaller real, or positive imaginary part)"), _o("p2", "second pole"),
            _o("stability", "stable if every pole lies inside the unit circle")],
   primary="radius", conditions=["a2 != 0", "a causal system (stable iff |p| < 1 for every pole)"],
   method="roots of z^2 + a1 z + a2 by the numerically stable quadratic formula", latex=r"z^2 + a_1 z + a_2 = 0,\quad \text{stable} \iff |p_i| < 1",
   run=second_order_poles, reference=ref_second_order_poles,
   wrong=lambda a1, a2: second_order_poles(-a1, a2),
   random_inputs=lambda rng: {"a1": rng.choice([-1, 1]) * round(rng.uniform(0.1, 2.5), rng.choice([1, 3, 6])), "a2": rng.choice([-1, 1]) * round(rng.uniform(0.05, 1.5), rng.choice([2, 4, 6]))},
   parse=lambda q: _parse(q, r"\bpoles?\b", [("a1", "num", [r"\ba1(?=\s*=)"]), ("a2", "num", [r"\ba2(?=\s*=)"])], special=_hz2, exclude=r"zeros?\b|\["),
   templates=["Find the poles of H(z) = 1/(1 - 1.2z^-1 + 0.5z^-2) and say whether the system is stable.",
              "Find the poles of the second-order system with a1 = -1.2 and a2 = 0.5."],
   examples=[{"inputs": {"a1": -1.2, "a2": 0.5}, "outputs": {"p1": complex(0.6, math.sqrt(0.14)), "p2": complex(0.6, -math.sqrt(0.14)), "radius": math.sqrt(0.5), "stability": "stable"}},
             {"inputs": {"a1": -2.5, "a2": 1.0}, "outputs": {"p1": 0.5, "p2": 2.0, "stability": "unstable"}}],
   questions=[{"q": "Find the poles of H(z) = 1/(1 - 1.2z^-1 + 0.5z^-2) and say whether the system is stable.", "outputs": {"radius": math.sqrt(0.5), "stability": "stable"}},
              {"q": "Find the poles of the second-order system with a1 = -2.5 and a2 = 1.", "outputs": {"p1": 0.5, "p2": 2.0, "stability": "unstable"}},
              {"q": "Find the poles of H(z) = 1/(1 + 0.81z^-2).", "outputs": {"p1": complex(0, 0.9), "radius": 0.9}},
              {"q": "Find the poles of the second-order system with a1 = -1.2.", "outcome": "NEEDS_INPUT"}])


def poles_zeros(b, a):
    b, a = _vec(b, "numerator b", 1, 5), _vec(a, "denominator a", 1, 5)
    if a[0] == 0:
        raise ValueError("a[0] must not be 0")
    if all(v == 0 for v in b):
        raise ValueError("the numerator is identically zero")
    bp, ap = _pad2(b, a)
    z, p = _canon_roots(_roots(bp)), _canon_roots(_roots(ap))
    rad = max([abs(v) for v in p], default=0.0)
    return {"max_pole_radius": rad, "zeros": z, "poles": p, "stability": "stable" if rad < 1 else "unstable"}


def ref_poles_zeros(b, a):
    from scipy import signal
    bp, ap = _pad2(b, a)
    z, p, _ = signal.tf2zpk(bp, ap)
    z, p = _canon_roots(list(z)), _canon_roots(list(p))
    rad = max([abs(v) for v in p], default=0.0)
    return {"max_pole_radius": float(rad), "zeros": z, "poles": p, "stability": "stable" if rad < 1 else "unstable"}


_p("poles_zeros", name="Poles, zeros and stability of a digital filter", topic="z-transform and stability",
   purpose="The zeros and poles of H(z) = sum b_k z^-k / sum a_k z^-k (up to 4th order), the largest pole radius and whether the causal filter is stable.",
   inputs=[_i("b", "numerator coefficients b (up to 5)", type="vector", min_length=1, max_length=5), _i("a", "denominator coefficients a (up to 5, a[0] != 0)", type="vector", min_length=1, max_length=5)],
   outputs=[_o("max_pole_radius", "largest pole radius"), _o("zeros", "zeros (real ascending, then complex)"), _o("poles", "poles (real ascending, then complex)"),
            _o("stability", "stable if every pole lies inside the unit circle")],
   primary="max_pole_radius", conditions=["a[0] != 0", "orders up to 4", "both polynomials in powers of z^-1 (padded to equal length)"],
   method="polynomial roots (closed form up to degree 2; Durand-Kerner iteration with Newton polishing above)",
   latex=r"H(z) = \frac{\sum_k b_k z^{-k}}{\sum_k a_k z^{-k}} = \frac{b_0\prod(1 - z_i z^{-1})}{a_0\prod(1 - p_i z^{-1})}",
   run=poles_zeros, reference=ref_poles_zeros,
   wrong=lambda b, a: (lambda r: dict(r, zeros=r["poles"], poles=r["zeros"]))(poles_zeros(b, a)),
   random_inputs=lambda rng: {"b": [rng.choice([-1, 1]) * round(rng.uniform(0.2, 3), 3) for _ in range(rng.randint(2, 5))],
                              "a": (lambda c: [v * rng.choice([1.0, 2.0]) for v in c])(_stable_a(rng, rng.randint(1, 4)) if rng.random() < 0.7 else [1.0] + [round(rng.uniform(-2, 2), 3) for _ in range(rng.randint(1, 4))])},
   parse=lambda q: _parse(q, r"poles?\b[^.?]*zeros?|zeros?\b[^.?]*poles?", _BA, require=r"\["),
   templates=["Find the poles and zeros of the filter with b = [1, -1] and a = [1, -0.9]."],
   examples=[{"inputs": {"b": "1, -1", "a": "1, -0.9"}, "outputs": {"zeros": [1.0], "poles": [0.9], "stability": "stable"}},
             {"inputs": {"b": [1, 0, -1], "a": [1, 0, 0.81]}, "outputs": {"zeros": [-1.0, 1.0], "poles": [complex(0, 0.9), complex(0, -0.9)], "max_pole_radius": 0.9}},
             {"inputs": {"b": [1], "a": [1, -1.5, 0.5]}, "outputs": {"zeros": [0.0, 0.0], "poles": [0.5, 1.0], "stability": "unstable"}}],
   questions=[{"q": "Find the poles and zeros of the filter with b = [1, -1] and a = [1, -0.9].", "outputs": {"zeros": [1.0], "poles": [0.9]}},
              {"q": "Find the zeros and poles of the filter with b = [1, 0, -1] and a = [1, 0, 0.81].", "outputs": {"max_pole_radius": 0.9}},
              {"q": "Find the poles and zeros of the filter with b = [1, -1] and a = [0, 1].", "outcome": "INVALID_INPUT"}])


def bilinear_rc_lowpass(fc, fs):
    fc, fs = _pos(fc, "cutoff frequency"), _pos(fs, "sampling rate")
    wc, K = 2 * math.pi * fc, 2 * fs
    return {"b0": wc / (wc + K), "b1": wc / (wc + K), "a1": (wc - K) / (wc + K)}


def ref_bilinear_rc_lowpass(fc, fs):
    import numpy as np
    from scipy import signal
    b, a = signal.bilinear([2 * np.pi * fc], [1.0, 2 * np.pi * fc], fs=fs)
    return {"b0": float(b[0] / a[0]), "b1": float(b[1] / a[0]), "a1": float(a[1] / a[0])}


_p("bilinear_rc_lowpass", name="Bilinear transform of a first-order low-pass filter", topic="digital filters",
   purpose="The digital filter H(z) = (b0 + b1 z^-1)/(1 + a1 z^-1) obtained from H(s) = wc/(s + wc) by the bilinear transform s = 2 fs (1 - z^-1)/(1 + z^-1), without pre-warping.",
   inputs=[_i("fc", "analog cutoff frequency", "frequency"), _i("fs", "sampling rate", "frequency")],
   outputs=[_o("b0", "numerator coefficient b0"), _o("b1", "numerator coefficient b1"), _o("a1", "denominator coefficient a1 (a0 = 1)")],
   primary="a1", conditions=["fc > 0", "fs > 0", "no frequency pre-warping"], method="substitute s = 2 fs (1 - z^-1)/(1 + z^-1) and normalize a0 = 1",
   latex=r"H(z) = \frac{\omega_c (1 + z^{-1})}{(\omega_c + 2f_s) + (\omega_c - 2f_s) z^{-1}}", run=bilinear_rc_lowpass, reference=ref_bilinear_rc_lowpass,
   wrong=lambda fc, fs: (lambda wc, K: {"b0": wc / (wc + K), "b1": wc / (wc + K), "a1": (wc - K) / (wc + K)})(2 * math.pi * fc, fs),
   random_inputs=lambda rng: (lambda fs: {"fs": fs, "fc": fs * rng.uniform(0.005, 0.45)})(_logu(None, 100, 1e7, rng)),
   parse=lambda q: _parse(q, r"bilinear", [("fs", "kind:frequency", _FS), ("fc", "kind:frequency", ["cutoff frequency", "cut-off frequency", "cutoff", "corner frequency", r"~+\s*frequency"])],
                          require=r"low[- ]?pass|\brc\b|first[- ]order", exclude=r"butterworth|chebyshev|pre-?warp|order of|high[- ]?pass"),
   templates=["Apply the bilinear transform to a first-order low-pass filter with a cutoff frequency of 1 kHz at a sampling rate of 8 kHz."],
   examples=[{"inputs": {"fc": 1000.0, "fs": 8000.0}, "outputs": bilinear_rc_lowpass(1000.0, 8000.0)}],
   questions=[{"q": "Apply the bilinear transform to a first-order low-pass filter with a cutoff frequency of 1 kHz at a sampling rate of 8 kHz.", "outputs": bilinear_rc_lowpass(1000.0, 8000.0)},
              {"q": "Apply the bilinear transform to a first-order low-pass filter with a cutoff frequency of 1 kHz.", "outcome": "NEEDS_INPUT"}])


# =================================================================================================== 7. windows
_WINDOWS = ["rectangular", "hann", "hamming", "blackman"]
_WIN_NAME = {"rectangular": "boxcar", "hann": "hann", "hamming": "hamming", "blackman": "blackman"}
_WIN_TABLE = [(r"\brectangular\b|\bboxcar\b|\brect\b", "rectangular"), (r"\bhann(?:ing)?\b|\bvon hann\b|raised[- ]cosine window", "hann"),
              (r"\bhamming\b", "hamming"), (r"\bblackman\b", "blackman")]
_MAINLOBE_BINS = {"rectangular": 2, "hann": 4, "hamming": 4, "blackman": 6}     # null-to-null, in DFT bins


def _win_special(text):
    found = [c for p, c in _WIN_TABLE if re.search(p, text, re.I)]
    if len(found) > 1:
        return None
    return ({"window": found[0]} if found else {}), []


def window_mainlobe_width(window, N, fs=None):
    if window not in _MAINLOBE_BINS:
        raise ValueError("window must be one of " + ", ".join(_WINDOWS))
    N = _int(N, "window length N", 8)
    bins = float(_MAINLOBE_BINS[window])
    out = {"width_bins": bins, "first_null_bins": bins / 2}
    if fs is not None:
        out["width_hz"] = bins * _pos(fs, "sampling rate") / N
    return out


def ref_window_mainlobe_width(window, N, fs=None):
    import numpy as np
    from scipy.signal import get_window
    w = get_window(_WIN_NAME[window], N, fftbins=True)
    W = np.abs(np.fft.fft(w, 64 * N))                  # the window's spectrum on a grid 64 times finer than the bins
    i = next(i for i in range(1, 64 * N - 1) if W[i] <= W[i - 1] and W[i] <= W[i + 1])
    out = {"width_bins": 2 * i / 64, "first_null_bins": i / 64}
    if fs is not None:
        out["width_hz"] = 2 * i / 64 * fs / N
    return out


_p("window_mainlobe_width", name="Main-lobe width of a spectral window", topic="windows and spectral analysis",
   purpose="The null-to-null main-lobe width of the rectangular, Hann, Hamming or Blackman window (DFT-even form), in bins and, with fs, in hertz.",
   inputs=[_i("window", "window type", type="choice", choices=_WINDOWS), _i("N", "window length", type="integer", min=8),
           _i("fs", "sampling rate (for the width in Hz)", "frequency", required=False)],
   outputs=[_o("width_bins", "null-to-null main-lobe width in DFT bins"), _o("first_null_bins", "position of the first null in bins"),
            _o("width_hz", "main-lobe width in Hz (fs given)", "frequency")],
   primary="width_bins", conditions=["N >= 8", "DFT-even (periodic) windows for spectral analysis"],
   method="tabulated widths 2, 4, 4, 6 bins (the first zero of each window's transform), times fs/N",
   latex=r"\Delta f_{\mathrm{ML}} = k\,\frac{f_s}{N},\ k = 2\ (\text{rect}),\ 4\ (\text{Hann, Hamming}),\ 6\ (\text{Blackman})",
   run=window_mainlobe_width, reference=ref_window_mainlobe_width,
   wrong=lambda window, N, fs=None: (lambda r: {k: v / 2 for k, v in r.items()})(window_mainlobe_width(window, N, fs)),
   random_inputs=lambda rng: {"window": rng.choice(_WINDOWS), "N": rng.choice([16, 32, 64, 100, 128, 256, 50]), "fs": _logu(None, 100, 1e6, rng)},
   parse=lambda q: _parse(q, r"main[- ]?lobe", [("N", "int", ["^-?\\s?point", r"\bN\s*=", "length(?: of)?", "^samples?"]), ("fs", "kind:frequency", _FS)],
                          special=_win_special),
   templates=["What is the main-lobe width of a 256-point Hann window at a sampling rate of 8 kHz?"],
   examples=[{"inputs": {"window": "hann", "N": 256, "fs": 8000.0}, "outputs": {"width_bins": 4.0, "width_hz": 125.0}},
             {"inputs": {"window": "rectangular", "N": 64}, "outputs": {"width_bins": 2.0, "first_null_bins": 1.0}},
             {"inputs": {"window": "blackman", "N": 100, "fs": 1000.0}, "outputs": {"width_hz": 60.0}}],
   questions=[{"q": "What is the main-lobe width of a 256-point Hann window at a sampling rate of 8 kHz?", "outputs": {"width_hz": 125.0}},
              {"q": "What is the main-lobe width of a 64-point rectangular window?", "outputs": {"width_bins": 2.0}},
              {"q": "What is the main-lobe width of a 4-point Hamming window?", "outcome": "INVALID_INPUT"},
              {"q": "What is the main-lobe width of a 64-point window?", "outcome": "NEEDS_INPUT"}])


def window_coefficients(window, N, form="symmetric"):
    if window not in _MAINLOBE_BINS:
        raise ValueError("window must be one of " + ", ".join(_WINDOWS))
    if form not in ("symmetric", "periodic"):
        raise ValueError("form must be symmetric or periodic")
    N = _int(N, "window length N", 2, 100000)
    M = N - 1 if form == "symmetric" else N
    coef = {"rectangular": [1.0], "hann": [0.5, 0.5], "hamming": [0.54, 0.46], "blackman": [0.42, 0.5, 0.08]}[window]
    w = [math.fsum(((-1) ** k) * c * math.cos(2 * math.pi * k * n / M) for k, c in enumerate(coef)) for n in range(N)]
    return {"w": w, "coherent_gain": math.fsum(w) / N}


def ref_window_coefficients(window, N, form="symmetric"):
    import numpy as np
    from scipy.signal import get_window
    w = get_window(_WIN_NAME[window], N, fftbins=(form == "periodic"))
    return {"w": [float(v) for v in w], "coherent_gain": float(np.mean(w))}


def _winform_special(text):
    got = _win_special(text)
    if got is None:
        return None
    vals = dict(got[0])
    if re.search(r"\bperiodic\b|dft[- ]even", text, re.I):
        vals["form"] = "periodic"
    elif re.search(r"\bsymmetric\b", text, re.I):
        vals["form"] = "symmetric"
    return vals, []


_p("window_coefficients", name="Coefficients of a window function", topic="windows and spectral analysis",
   purpose="The samples w[0..N-1] of a rectangular, Hann, Hamming or Blackman window (symmetric for filter design, periodic for spectral analysis) and its coherent gain.",
   inputs=[_i("window", "window type", type="choice", choices=_WINDOWS), _i("N", "window length", type="integer", min=2),
           _i("form", "symmetric (default) or periodic", type="choice", choices=["symmetric", "periodic"], required=False)],
   outputs=[_o("w", "window samples w[0..N-1]"), _o("coherent_gain", "coherent gain: mean of w")],
   primary="w", conditions=["N >= 2"], method="generalized cosine sums: w[n] = sum_k (-1)^k a_k cos(2 pi k n / M), M = N-1 (symmetric) or N (periodic)",
   latex=r"w[n] = \sum_k (-1)^k a_k \cos\frac{2\pi k n}{M}", run=window_coefficients, reference=ref_window_coefficients,
   wrong=lambda window, N, form="symmetric": window_coefficients(window, N, "periodic" if form == "symmetric" else "symmetric"),
   random_inputs=lambda rng: {"window": rng.choice(["hann", "hamming", "blackman"]), "N": rng.randint(4, 64), "form": rng.choice(["symmetric", "periodic"])},
   parse=lambda q: _parse(q, r"\bwindow\b", [("N", "int", ["^-?\\s?point", r"\bN\s*=", "length(?: of)?", "^samples?"])], special=_winform_special,
                          require=r"coefficients?|weights|values|samples of|list", exclude=r"main[- ]?lobe|fir|filter|design|\["),
   templates=["List the coefficients of an 8-point Hamming window.", "Give the coefficients of a 4-point periodic Hann window."],
   examples=[{"inputs": {"window": "hann", "N": 4, "form": "periodic"}, "outputs": {"w": [0.0, 0.5, 1.0, 0.5]}},
             {"inputs": {"window": "hann", "N": 5}, "outputs": {"w": [0.0, 0.5, 1.0, 0.5, 0.0], "coherent_gain": 0.4}},
             {"inputs": {"window": "hamming", "N": 3}, "outputs": {"w": [0.08, 1.0, 0.08]}}],
   questions=[{"q": "Give the coefficients of a 4-point periodic Hann window.", "outputs": {"w": [0.0, 0.5, 1.0, 0.5]}},
              {"q": "List the coefficients of a 3-point Hamming window.", "outputs": {"w": [0.08, 1.0, 0.08]}},
              {"q": "List the coefficients of a Hamming window.", "outcome": "NEEDS_INPUT"}])


# =================================================================================================== 8. Fourier series and transforms
_WAVES = ["square", "triangle", "sawtooth"]


def fourier_series_harmonic(waveform, A, n):
    if waveform not in _WAVES:
        raise ValueError("waveform must be square, triangle or sawtooth")
    A, n = _pos(A, "peak amplitude"), _int(n, "harmonic number", 1, 1000)
    if waveform == "square":
        c, rel = (4 * A / (n * math.pi), 1.0 / n) if n % 2 else (0.0, 0.0)
    elif waveform == "triangle":
        c, rel = (8 * A / (math.pi ** 2 * n * n), 1.0 / (n * n)) if n % 2 else (0.0, 0.0)
    else:
        c, rel = 2 * A / (n * math.pi), 1.0 / n
    return {"amplitude": c, "relative": rel, "rms": c / math.sqrt(2)}


def ref_fourier_series_harmonic(waveform, A, n):
    import numpy as np
    from scipy import integrate, signal
    x = {"square": lambda t: signal.square(2 * np.pi * t), "triangle": lambda t: signal.sawtooth(2 * np.pi * t, 0.5),
         "sawtooth": lambda t: signal.sawtooth(2 * np.pi * t, 1.0)}[waveform]

    def amp(m):
        pts = sorted(set([0.5] + [k / (2 * m) for k in range(1, 2 * m)]))[:200]
        an, _ = integrate.quad(lambda t: 2 * A * float(x(t)) * np.cos(2 * np.pi * m * t), 0, 1, points=pts, limit=2000, epsabs=1e-14, epsrel=1e-13)
        bn, _ = integrate.quad(lambda t: 2 * A * float(x(t)) * np.sin(2 * np.pi * m * t), 0, 1, points=pts, limit=2000, epsabs=1e-14, epsrel=1e-13)
        return float(np.hypot(an, bn))
    c = amp(n)
    c = 0.0 if abs(c) < 1e-10 * A else c               # even harmonics of the square and triangle are exactly 0
    c1 = amp(1)
    return {"amplitude": c, "relative": c / c1, "rms": c / np.sqrt(2)}


def _wave_special(text):
    found = [c for p, c in ((r"\bsquare\b", "square"), (r"\btriang(?:le|ular)\b", "triangle"), (r"\bsaw[- ]?tooth\b|\bramp wave\b", "sawtooth")) if re.search(p, text, re.I)]
    if len(found) > 1:
        return None
    return ({"waveform": found[0]} if found else {}), []


_p("fourier_series_harmonic", name="Harmonic amplitude of a square, triangle or sawtooth wave", topic="Fourier series and transforms",
   purpose="The amplitude of the n-th harmonic in the Fourier series of a zero-mean square, triangle or sawtooth wave of peak amplitude A, relative to the fundamental, and its RMS value.",
   inputs=[_i("waveform", "waveform", type="choice", choices=_WAVES), _i("A", "peak amplitude of the wave"), _i("n", "harmonic number (1 = fundamental)", type="integer", min=1, max=1000)],
   outputs=[_o("amplitude", "amplitude of the n-th harmonic"), _o("relative", "amplitude relative to the fundamental"), _o("rms", "RMS value of the n-th harmonic")],
   primary="amplitude", conditions=["zero-mean waveform with peak A", "50 % duty square wave", "symmetric triangle", "linear sawtooth"],
   method="Fourier coefficients: square 4A/(n pi) (odd n), triangle 8A/(pi^2 n^2) (odd n), sawtooth 2A/(n pi)",
   latex=r"c_n^{\mathrm{sq}} = \frac{4A}{n\pi},\ c_n^{\mathrm{tri}} = \frac{8A}{\pi^2 n^2}\ (n\ \text{odd}),\quad c_n^{\mathrm{saw}} = \frac{2A}{n\pi}",
   run=fourier_series_harmonic, reference=ref_fourier_series_harmonic,
   wrong=lambda waveform, A, n: (lambda r: dict(r, amplitude=r["amplitude"] / 2 if r["amplitude"] else A, rms=r["rms"] / 2 if r["rms"] else A))(fourier_series_harmonic(waveform, A, n)),
   random_inputs=lambda rng: {"waveform": rng.choice(_WAVES), "A": round(_logu(None, 0.1, 100, rng), 3), "n": rng.randint(1, 15)},
   parse=lambda q: _parse(q, r"\bharmonic\b", [("n", "ord", ["^harmonic", r"harmonic (?:number )?(?:n\s*=)?"]), ("A", "num", ["peak amplitude", "amplitude", "peak value", "peak"])],
                          special=_wave_special, exclude=r"\bthd\b|total harmonic|\[|distortion"),
   templates=["What is the amplitude of the 3rd harmonic of a square wave with amplitude 2?",
              "Find the 5th harmonic amplitude of a triangle wave with peak amplitude 1."],
   examples=[{"inputs": {"waveform": "square", "A": 2, "n": 3}, "outputs": {"amplitude": 8 / (3 * math.pi), "relative": 1 / 3}},
             {"inputs": {"waveform": "square", "A": 1, "n": 2}, "outputs": {"amplitude": 0.0}},
             {"inputs": {"waveform": "triangle", "A": 1, "n": 5}, "outputs": {"amplitude": 8 / (25 * math.pi ** 2)}},
             {"inputs": {"waveform": "sawtooth", "A": 1, "n": 4}, "outputs": {"amplitude": 1 / (2 * math.pi)}}],
   questions=[{"q": "What is the amplitude of the 3rd harmonic of a square wave with amplitude 2?", "outputs": {"amplitude": 8 / (3 * math.pi)}},
              {"q": "Find the 5th harmonic amplitude of a triangle wave with peak amplitude 1.", "outputs": {"amplitude": 8 / (25 * math.pi ** 2)}},
              {"q": "What is the amplitude of the 2nd harmonic of a sawtooth wave with amplitude 1?", "outputs": {"amplitude": 1 / math.pi}},
              {"q": "What is the amplitude of the 3rd harmonic of a square wave?", "outcome": "NEEDS_INPUT"}])


def rect_pulse_spectrum(A, tau, f):
    A, tau, f = _fin(A, "amplitude"), _pos(tau, "pulse width"), _nonneg(f, "frequency")
    if A == 0:
        raise ValueError("the amplitude must not be 0")
    x = f * tau
    s = 1.0 if x == 0 else math.sin(math.pi * x) / (math.pi * x)
    return {"X": A * tau * s, "magnitude": abs(A * tau * s), "first_null": 1.0 / tau}


def ref_rect_pulse_spectrum(A, tau, f):
    import numpy as np
    from scipy import integrate, optimize
    X = lambda fr: tau * integrate.quad(lambda u: A * np.cos(2 * np.pi * fr * tau * u), -0.5, 0.5, limit=500, epsabs=1e-15, epsrel=1e-13)[0]   # noqa: E731
    v = X(f)
    null = optimize.brentq(lambda fr: X(fr) / (A * tau), 0.5 / tau, 1.5 / tau, xtol=1e-300, rtol=8.9e-16)
    return {"X": v, "magnitude": abs(v), "first_null": null}


_p("rect_pulse_spectrum", name="Fourier transform of a rectangular pulse", topic="Fourier series and transforms",
   purpose="X(f) = A tau sinc(f tau) of a rectangular pulse of amplitude A and width tau centred at t = 0, its magnitude, and the first spectral null 1/tau.",
   inputs=[_i("A", "pulse amplitude"), _i("tau", "pulse width", "time"), _i("f", "frequency", "frequency")],
   outputs=[_o("X", "X(f) (real, in amplitude-seconds)"), _o("magnitude", "|X(f)|"), _o("first_null", "first zero of the spectrum, 1/tau", "frequency")],
   primary="X", conditions=["A != 0", "tau > 0", "the pulse centred at t = 0"], method="X(f) = A tau sin(pi f tau)/(pi f tau)",
   latex=r"X(f) = A\tau\,\mathrm{sinc}(f\tau) = A\tau\frac{\sin(\pi f\tau)}{\pi f\tau}", run=rect_pulse_spectrum, reference=ref_rect_pulse_spectrum,
   wrong=lambda A, tau, f: (lambda x: {"X": A * tau * (math.sin(x) / x if x else 1.0), "magnitude": abs(A * tau * (math.sin(x) / x if x else 1.0)), "first_null": 1 / (2 * tau)})(f * tau),
   random_inputs=lambda rng: (lambda tau: {"A": rng.choice([-1, 1]) * _logu(None, 0.1, 10, rng), "tau": tau, "f": rng.uniform(0.01, 6) / tau})(_logu(None, 1e-7, 1, rng)),
   parse=lambda q: _parse(q, r"rectangular pulse|rect(?:angular)? function|gate pulse|square pulse",
                          [("tau", "kind:time", ["pulse width", "width", "duration", "lasting", "^(?:-|\\s)?(?:wide|long)"]), ("f", "kind:frequency", ["at", "frequency of", "frequency"]),
                           ("A", "num", ["amplitude", "height"])], require=r"fourier|spectrum|transform"),
   templates=["What is the Fourier transform of a rectangular pulse of amplitude 2 and width 1 ms at 250 Hz?"],
   examples=[{"inputs": {"A": 2, "tau": 1e-3, "f": 250.0}, "outputs": {"X": 2e-3 * math.sin(math.pi / 4) / (math.pi / 4), "first_null": 1000.0}},
             {"inputs": {"A": 1, "tau": 2e-6, "f": 0.0}, "outputs": {"X": 2e-6}}, {"inputs": {"A": 1, "tau": 1e-3, "f": 1000.0}, "outputs": {"magnitude": 0.0}}],
   questions=[{"q": "What is the Fourier transform of a rectangular pulse of amplitude 2 and width 1 ms at 250 Hz?", "outputs": {"X": 2e-3 * math.sin(math.pi / 4) / (math.pi / 4)}},
              {"q": "Find the spectrum of a rectangular pulse of amplitude 1 and width 1 ms at 1500 Hz.", "outputs": {"X": 1e-3 * math.sin(1.5 * math.pi) / (1.5 * math.pi)}},
              {"q": "Find the spectrum of a rectangular pulse of amplitude 1 at 1500 Hz.", "outcome": "NEEDS_INPUT"}])


def exponential_pulse_spectrum(A, tau, f):
    A, tau, f = _pos(A, "amplitude"), _pos(tau, "time constant"), _nonneg(f, "frequency")
    x = 2 * math.pi * f * tau
    return {"magnitude": A * tau / math.sqrt(1 + x * x), "phase": -math.atan(x) + 0.0, "re": A * tau / (1 + x * x), "im": -A * tau * x / (1 + x * x) + 0.0}


def ref_exponential_pulse_spectrum(A, tau, f):
    import numpy as np
    from scipy import integrate
    g = lambda t: A * np.exp(-t / tau)                 # noqa: E731
    w = 2 * np.pi * f
    if w == 0:
        re_, im = integrate.quad(g, 0, 80 * tau, epsabs=0, epsrel=1e-13)[0], 0.0
    else:
        re_ = integrate.quad(g, 0, 80 * tau, weight="cos", wvar=w, epsabs=0, epsrel=1e-13, limit=1000)[0]
        im = -integrate.quad(g, 0, 80 * tau, weight="sin", wvar=w, epsabs=0, epsrel=1e-13, limit=1000)[0]
    return {"magnitude": float(np.hypot(re_, im)), "phase": float(np.arctan2(im, re_)), "re": re_, "im": im}


_p("exponential_pulse_spectrum", name="Fourier transform of a decaying exponential", topic="Fourier series and transforms",
   purpose="X(f) = A tau / (1 + j 2 pi f tau) of x(t) = A exp(-t/tau) for t >= 0: magnitude, phase, real and imaginary parts.",
   inputs=[_i("A", "initial amplitude"), _i("tau", "time constant", "time"), _i("f", "frequency", "frequency")],
   outputs=[_o("magnitude", "|X(f)|"), _o("phase", "phase of X(f) in radians", "angle"), _o("re", "real part"), _o("im", "imaginary part")],
   primary="magnitude", conditions=["A > 0", "tau > 0", "x(t) = 0 for t < 0"], method="X(f) = integral_0^inf A e^{-t/tau} e^{-j 2 pi f t} dt in closed form",
   latex=r"X(f) = \frac{A\tau}{1 + j2\pi f\tau}", run=exponential_pulse_spectrum, reference=ref_exponential_pulse_spectrum,
   wrong=lambda A, tau, f: (lambda x: {"magnitude": A * tau / (1 + x), "phase": -math.atan(x), "re": A * tau / (1 + x * x), "im": -A * tau * x / (1 + x * x)})(2 * math.pi * f * tau),
   random_inputs=lambda rng: (lambda tau: {"A": _logu(None, 0.1, 10, rng), "tau": tau, "f": _logu(None, 0.01, 10, rng) / tau})(_logu(None, 1e-6, 1, rng)),
   parse=lambda q: _parse(q, r"exponential|e\^\s*\(?\s*-\s*t", [("tau", "kind:time", ["time constant", "decay time", r"\btau(?=\s*=)"]), ("f", "kind:frequency", ["at", "frequency of", "frequency"]),
                                                             ("A", "num", ["initial amplitude", "amplitude", "initial value", "peak"])],
                          require=r"fourier|spectrum|transform", exclude=r"rectangular|growth|bandwidth"),
   templates=["What is the Fourier transform of a decaying exponential with amplitude 1 and time constant 1 ms at 159.15 Hz?"],
   examples=[{"inputs": {"A": 1, "tau": 1e-3, "f": 1000 / (2 * math.pi)}, "outputs": {"magnitude": 1e-3 / math.sqrt(2), "phase": -math.pi / 4}},
             {"inputs": {"A": 2, "tau": 0.5, "f": 0.0}, "outputs": {"magnitude": 1.0, "phase": 0.0}}],
   questions=[{"q": "What is the Fourier transform of a decaying exponential with amplitude 2 and time constant 0.5 s at 0 Hz?", "outputs": {"magnitude": 1.0}},
              {"q": "What is the Fourier transform of a decaying exponential with amplitude 1 and time constant 1 ms at 159.15 Hz?", "outputs": exponential_pulse_spectrum(1, 1e-3, 159.15)},
              {"q": "What is the Fourier transform of a decaying exponential with amplitude 1 at 100 Hz?", "outcome": "NEEDS_INPUT"}])


def thd(V):
    V = _vec(V, "harmonic amplitudes", 2, 10000)
    if V[0] <= 0 or any(v < 0 for v in V):
        raise ValueError("the amplitudes must be >= 0 and the fundamental > 0")
    t = math.sqrt(math.fsum(v * v for v in V[1:])) / V[0]
    return {"thd": t, "percent": 100 * t, "db": _db20(t) if t > 0 else -INF}


def ref_thd(V):
    import numpy as np
    K = len(V)
    N = 1 << max(6, (4 * K).bit_length())
    n = np.arange(N)
    x = sum(v * np.cos(2 * np.pi * (k + 1) * n / N + 0.3 * k) for k, v in enumerate(V))   # synthesized, then measured
    X = np.abs(np.fft.rfft(x)) * 2 / N
    t = float(np.sqrt(np.sum(X[2:K + 1] ** 2)) / X[1])
    return {"thd": t, "percent": 100 * t, "db": float(20 * np.log10(t))}


_p("thd", name="Total harmonic distortion", topic="Fourier series and transforms",
   purpose="THD = sqrt(V2^2 + V3^2 + ...)/V1 from the amplitudes of the fundamental and its harmonics, as a ratio, in percent and in dB.",
   inputs=[_i("V", "amplitudes [V1, V2, V3, ...] of the fundamental and harmonics", type="vector", min_length=2, max_length=10000)],
   outputs=[_o("thd", "THD as a ratio"), _o("percent", "THD in percent"), _o("db", "THD in dB")],
   primary="thd", conditions=["V1 > 0", "amplitudes >= 0 (RMS or peak, all alike)"], method="root-sum-square of the harmonics over the fundamental",
   latex=r"\mathrm{THD} = \frac{\sqrt{\sum_{n\ge 2} V_n^2}}{V_1}", run=thd, reference=ref_thd,
   wrong=lambda V: (lambda t: {"thd": t, "percent": 100 * t, "db": _db20(t) if t > 0 else -INF})(math.fsum(V[1:]) / V[0]),
   random_inputs=lambda rng: {"V": [_logu(None, 0.5, 10, rng)] + [round(rng.uniform(0.01, 1), 3) for _ in range(rng.randint(2, 9))]},
   parse=lambda q: _parse(q, r"\bthd\b|total harmonic distortion", [("V", "vec", ["amplitudes", "harmonics", "amplitudes of", ""])], require=r"\["),
   templates=["What is the total harmonic distortion of a signal with harmonic amplitudes [10, 0.5, 0.3, 0.1]?"],
   examples=[{"inputs": {"V": "10, 0.5, 0.3, 0.1"}, "outputs": {"thd": math.sqrt(0.35) / 10}},
             {"inputs": {"V": [1, 0.3, 0.4]}, "outputs": {"thd": 0.5, "percent": 50.0}}],
   questions=[{"q": "What is the total harmonic distortion of a signal with harmonic amplitudes [10, 0.5, 0.3, 0.1]?", "outputs": {"thd": math.sqrt(0.35) / 10}},
              {"q": "Find the THD for the amplitudes [1, 0.3, 0.4].", "outputs": {"percent": 50.0}},
              {"q": "Find the THD for the amplitudes [0, 0.3, 0.4].", "outcome": "INVALID_INPUT"}])


def analytic_envelope(x):
    x = _vec(x, "signal x", 1, 65536)
    N, s = len(x), math.fsum(abs(v) for v in x)
    X = _fft(x)
    h = [0.0] * N
    h[0] = 1.0
    if N % 2 == 0:
        h[N // 2] = 1.0
        for k in range(1, N // 2):
            h[k] = 2.0
    else:
        for k in range(1, (N + 1) // 2):
            h[k] = 2.0
    z = [_snap(v / N, s) for v in _fft([a * b for a, b in zip(X, h)], +1)]
    return {"envelope": [abs(v) for v in z], "hilbert": [v.imag for v in z]}


def ref_analytic_envelope(x):
    import numpy as np
    from scipy.signal import hilbert
    s = float(np.sum(np.abs(x)))
    z = [_snap(v, s) for v in hilbert(np.asarray(x, float))]
    return {"envelope": [abs(v) for v in z], "hilbert": [v.imag for v in z]}


_p("analytic_envelope", name="Analytic signal, Hilbert transform and envelope", topic="Fourier series and transforms",
   purpose="The discrete Hilbert transform of a real sequence and its envelope |x + j H{x}|, computed through the DFT (negative frequencies removed, positive ones doubled).",
   inputs=[_i("x", "real samples", type="vector", min_length=1, max_length=65536)],
   outputs=[_o("envelope", "envelope |x_a[n]|"), _o("hilbert", "Hilbert transform (imaginary part of the analytic signal)")],
   primary="envelope", conditions=["the sequence treated as one period (DFT-based transform)"],
   method="FFT, multiply by h[k] = 1, 2, ..., 2, (1), 0, ..., 0, inverse FFT",
   latex=r"x_a[n] = \mathrm{IDFT}\{X[k]\,h[k]\},\quad \text{envelope} = |x_a[n]|", run=analytic_envelope, reference=ref_analytic_envelope,
   wrong=lambda x: (lambda r: {"envelope": [v / 2 for v in r["envelope"]], "hilbert": [v / 2 for v in r["hilbert"]]})(analytic_envelope(x)),
   random_inputs=lambda rng: {"x": _rand_vec(rng, 4, rng.choice([8, 16, 15, 32]))},
   parse=lambda q: _parse(q, r"hilbert transform|analytic signal|envelope", [("x", "vec", [r"\bx(?:\[n\])?(?=\s*[=:])", "sequence", "signal", "samples", ""])], require=r"\["),
   templates=["Find the Hilbert transform and envelope of x = [1, 0, -1, 0]."],
   examples=[{"inputs": {"x": "1, 0, -1, 0"}, "outputs": {"hilbert": [0.0, 1.0, 0.0, -1.0], "envelope": [1.0, 1.0, 1.0, 1.0]}}],
   questions=[{"q": "Find the Hilbert transform and envelope of x = [1, 0, -1, 0].", "outputs": {"envelope": [1.0, 1.0, 1.0, 1.0]}},
              {"q": "Find the envelope of the analytic signal of x = [2, 0, -2, 0].", "outputs": {"envelope": [2.0, 2.0, 2.0, 2.0]}}])


def periodogram(x, fs):
    x = _vec(x, "signal x", 2, 65536)
    fs = _pos(fs, "sampling rate")
    N = len(x)
    mu = math.fsum(x) / N
    X = _fft([v - mu for v in x])
    K = N // 2 + 1
    psd = []
    for k in range(K):
        p = abs(X[k]) ** 2 / (fs * N)
        if k != 0 and not (N % 2 == 0 and k == N // 2):
            p *= 2
        psd.append(p)
    i = _peak(psd, max(psd))
    return {"peak_frequency": i * fs / N, "freqs": [k * fs / N for k in range(K)], "psd": psd}


def ref_periodogram(x, fs):
    from scipy import signal
    f, p = signal.periodogram(x, fs)
    p = [float(v) for v in p]
    i = _peak(p, max(p))
    return {"peak_frequency": float(f[i]), "freqs": [float(v) for v in f], "psd": p}


_p("periodogram", name="Periodogram (power spectral density estimate)", topic="windows and spectral analysis",
   purpose="The one-sided periodogram PSD estimate (rectangular window, mean removed) of a sequence sampled at fs, its frequency grid and the frequency of its peak.",
   inputs=[_i("x", "samples", type="vector", min_length=2, max_length=65536), _i("fs", "sampling rate", "frequency")],
   outputs=[_o("peak_frequency", "frequency of the largest PSD value", "frequency"), _o("freqs", "frequencies k fs / N, k = 0 .. N/2"),
            _o("psd", "one-sided power spectral density (power per hertz)")],
   primary="peak_frequency", conditions=["mean removed first", "rectangular window", "one-sided density scaling"],
   method="FFT of the mean-removed samples; |X[k]|^2 / (fs N), doubled except at DC and Nyquist",
   latex=r"P[k] = \frac{2\,|X[k]|^2}{f_s N}\ (0 < k < N/2)", run=periodogram, reference=ref_periodogram,
   wrong=lambda x, fs: (lambda r: dict(r, psd=[v / 2 for v in r["psd"]]))(periodogram(x, fs)),
   random_inputs=lambda rng: {"x": _rand_vec(rng, 8, rng.choice([16, 32, 25])), "fs": _logu(None, 10, 1e5, rng)},
   parse=lambda q: _parse(q, r"periodogram|power spectral density|\bpsd\b", [("x", "vec", [r"\bx(?:\[n\])?(?=\s*[=:])", "sequence", "signal", "samples", ""]), ("fs", "kind:frequency", _FS)], require=r"\["),
   templates=["Compute the periodogram of x = [1, 0, -1, 0, 1, 0, -1, 0] at a sampling rate of 8 Hz."],
   examples=[{"inputs": {"x": "1, 0, -1, 0, 1, 0, -1, 0", "fs": 8.0}, "outputs": {"peak_frequency": 2.0, "psd": [0.0, 0.0, 0.5, 0.0, 0.0]}}],
   questions=[{"q": "Compute the periodogram of x = [1, 0, -1, 0, 1, 0, -1, 0] at a sampling rate of 8 Hz.", "outputs": {"peak_frequency": 2.0}},
              {"q": "Compute the periodogram of x = [1, 0, -1, 0, 1, 0, -1, 0].", "outcome": "NEEDS_INPUT"}])


# =================================================================================================== 9. modulation
def am_power(Pc, m):
    Pc, m = _pos(Pc, "carrier power"), _pos(m, "modulation index")
    if m > 1:
        raise ValueError("the modulation index must be at most 1 (no overmodulation)")
    return {"Pt": Pc * (1 + m * m / 2), "Psb": Pc * m * m / 2, "Pusb": Pc * m * m / 4, "efficiency": m * m / (2 + m * m)}


def ref_am_power(Pc, m):
    import numpy as np
    N = 64
    n = np.arange(N)
    s = np.sqrt(2 * Pc) * (1 + m * np.cos(2 * np.pi * 2 * n / N)) * np.cos(2 * np.pi * 16 * n / N)   # simulated, then measured
    X = np.abs(np.fft.rfft(s)) * 2 / N
    pu, pl = X[18] ** 2 / 2, X[14] ** 2 / 2
    pt = float(np.mean(s ** 2))
    return {"Pt": pt, "Psb": pu + pl, "Pusb": pu, "efficiency": (pu + pl) / pt}


_p("am_power", name="Power in an AM (DSB full-carrier) signal", topic="modulation",
   purpose="Total power Pc (1 + m^2/2), sideband power, power per sideband and transmission efficiency of an AM signal with carrier power Pc and modulation index m (sinusoidal message).",
   inputs=[_i("Pc", "carrier power", "power"), _i("m", "modulation index (0 < m <= 1)")],
   outputs=[_o("Pt", "total transmitted power", "power"), _o("Psb", "power in both sidebands", "power"), _o("Pusb", "power in one sideband", "power"),
            _o("efficiency", "fraction of the power in the sidebands")],
   primary="Pt", conditions=["0 < m <= 1", "a single sinusoidal message"], method="Pt = Pc (1 + m^2/2); each sideband Pc m^2/4",
   latex=r"P_t = P_c\left(1 + \frac{m^2}{2}\right)", run=am_power, reference=ref_am_power,
   wrong=lambda Pc, m: {"Pt": Pc * (1 + m * m / 4), "Psb": Pc * m * m / 4, "Pusb": Pc * m * m / 8, "efficiency": m * m / (4 + m * m)},
   random_inputs=lambda rng: {"Pc": _logu(None, 1e-3, 1e6, rng), "m": round(rng.uniform(0.1, 1.0), 3)},
   parse=lambda q: _parse(q, r"\bam\b|amplitude[- ]modulat", [("Pc", "kind:power", ["carrier power(?: of)?", "unmodulated carrier power(?: of)?", "power of the carrier"]),
                                                             ("m", "frac", ["modulation index(?: of)?", "modulation depth(?: of)?", "modulation factor(?: of)?", "^modulation"])],
                          require=r"carrier power|power of the carrier", exclude=r"v_?max|vmin|envelope"),
   templates=["An AM transmitter has a carrier power of 1000 W and a modulation index of 0.8. What is the total transmitted power?"],
   examples=[{"inputs": {"Pc": 1000.0, "m": 0.8}, "outputs": {"Pt": 1320.0, "Psb": 320.0, "Pusb": 160.0}},
             {"inputs": {"Pc": 50.0, "m": 1.0}, "outputs": {"Pt": 75.0, "efficiency": 1 / 3}}],
   questions=[{"q": "An AM transmitter has a carrier power of 1000 W and a modulation index of 0.8. What is the total transmitted power?", "outputs": {"Pt": 1320.0}},
              {"q": "An AM signal has a carrier power of 50 W and 100% modulation. Find the total power and the efficiency.", "outputs": {"Pt": 75.0}},
              {"q": "An AM transmitter has a carrier power of 1000 W and a modulation index of 1.2. What is the total transmitted power?", "outcome": "INVALID_INPUT"},
              {"q": "An AM transmitter has a carrier power of 1000 W. What is the total transmitted power?", "outcome": "NEEDS_INPUT"}])


def am_modulation_index(Vmax, Vmin):
    Vmax, Vmin = _pos(Vmax, "maximum envelope"), _nonneg(Vmin, "minimum envelope")
    if Vmin >= Vmax:
        raise ValueError("the maximum of the envelope must exceed its minimum")
    return {"m": (Vmax - Vmin) / (Vmax + Vmin), "Ac": (Vmax + Vmin) / 2}


def ref_am_modulation_index(Vmax, Vmin):
    import numpy as np
    N = 64
    n = np.arange(N)
    env = (Vmax + Vmin) / 2 + (Vmax - Vmin) / 2 * np.cos(2 * np.pi * 2 * n / N)    # an envelope swinging Vmin..Vmax
    X = np.abs(np.fft.rfft(env * np.cos(2 * np.pi * 16 * n / N))) * 2 / N         # measured from the spectrum
    return {"m": float((X[14] + X[18]) / X[16]), "Ac": float(X[16])}


_p("am_modulation_index", name="AM modulation index from the envelope", topic="modulation",
   purpose="The modulation index m = (Vmax - Vmin)/(Vmax + Vmin) of an AM wave from the maximum and minimum of its envelope, and the carrier amplitude.",
   inputs=[_i("Vmax", "maximum of the envelope", "voltage"), _i("Vmin", "minimum of the envelope", "voltage")],
   outputs=[_o("m", "modulation index"), _o("Ac", "carrier amplitude (Vmax + Vmin)/2", "voltage")],
   primary="m", conditions=["0 <= Vmin < Vmax", "sinusoidal message"], method="m = (Vmax - Vmin)/(Vmax + Vmin)",
   latex=r"m = \frac{V_{\max} - V_{\min}}{V_{\max} + V_{\min}}", run=am_modulation_index, reference=ref_am_modulation_index,
   wrong=lambda Vmax, Vmin: {"m": (Vmax - Vmin) / Vmax, "Ac": (Vmax + Vmin) / 2},
   random_inputs=lambda rng: (lambda a: {"Vmax": a, "Vmin": a * rng.uniform(0.05, 0.9)})(_logu(None, 1e-3, 1e3, rng)),
   parse=lambda q: _parse(q, r"modulation index|modulation depth|percent(?:age)? modulation", [("Vmax", "kind:voltage", ["maximum(?: envelope)?(?: amplitude| voltage| value)?(?: of)?", r"\bV_?max(?=\s*=)"]),
                                                                                             ("Vmin", "kind:voltage", ["minimum(?: envelope)?(?: amplitude| voltage| value)?(?: of)?", r"\bV_?min(?=\s*=)"])],
                          require=r"envelope|v_?max|maximum"),
   templates=["An AM envelope has a maximum of 10 V and a minimum of 2 V. What is the modulation index?"],
   examples=[{"inputs": {"Vmax": 10.0, "Vmin": 2.0}, "outputs": {"m": 2 / 3, "Ac": 6.0}}],
   questions=[{"q": "An AM envelope has a maximum of 10 V and a minimum of 2 V. What is the modulation index?", "outputs": {"m": 2 / 3}},
              {"q": "An AM envelope has a maximum of 2 V and a minimum of 10 V. What is the modulation index?", "outcome": "INVALID_INPUT"}])


def _besselj(n, x):
    import mpmath
    with mpmath.workdps(30):
        return float(mpmath.besselj(n, x))


def fm_spectrum(df, fm, n=1):
    df, fm = _pos(df, "peak frequency deviation"), _pos(fm, "modulating frequency")
    n = _int(n, "sideband number", 0, 100)
    beta = df / fm
    if beta > 100:
        raise ValueError("the modulation index df/fm must be at most 100")
    jn = _besselj(n, beta)
    nmax = int(math.floor(beta + 1 + 1e-9))
    pin = _besselj(0, beta) ** 2 + 2 * math.fsum(_besselj(k, beta) ** 2 for k in range(1, nmax + 1))
    return {"carson_bandwidth": 2 * (df + fm), "beta": beta, "carrier": _besselj(0, beta), "Jn": jn,
            "pair_power": jn * jn * (1 if n == 0 else 2), "power_in_carson": pin}


def ref_fm_spectrum(df, fm, n=1):
    from math import floor
    from scipy.special import jv
    beta = df / fm
    nmax = int(floor(beta + 1 + 1e-9))
    pin = float(jv(0, beta) ** 2 + 2 * sum(jv(k, beta) ** 2 for k in range(1, nmax + 1)))
    return {"carson_bandwidth": 2 * (df + fm), "beta": beta, "carrier": float(jv(0, beta)), "Jn": float(jv(n, beta)),
            "pair_power": float(jv(n, beta) ** 2) * (1 if n == 0 else 2), "power_in_carson": pin}


_p("fm_spectrum", name="FM spectrum: Carson bandwidth and Bessel sideband amplitudes", topic="modulation",
   purpose="For sinusoidal FM with peak deviation df and modulating frequency fm: the index beta = df/fm, Carson's bandwidth 2(df + fm), the carrier and n-th sideband amplitudes J0(beta), Jn(beta) (relative to the unmodulated carrier) and the power fraction inside Carson's bandwidth.",
   inputs=[_i("df", "peak frequency deviation", "frequency"), _i("fm", "modulating (message) frequency", "frequency"),
           _i("n", "sideband number (default 1)", type="integer", min=0, max=100, required=False)],
   outputs=[_o("carson_bandwidth", "Carson's rule bandwidth 2(df + fm)", "frequency"), _o("beta", "modulation index df/fm"),
            _o("carrier", "carrier amplitude J0(beta)"), _o("Jn", "amplitude of each n-th sideband Jn(beta)"),
            _o("pair_power", "power fraction in the pair of n-th sidebands"), _o("power_in_carson", "power fraction within Carson's bandwidth")],
   primary="carson_bandwidth", conditions=["sinusoidal message", "beta <= 100"],
   method="Bessel functions of the first kind (30-digit evaluation); Carson's rule", latex=r"B_C = 2(\Delta f + f_m),\quad s(t) = A\sum_n J_n(\beta)\cos[2\pi(f_c + n f_m)t]",
   run=fm_spectrum, reference=ref_fm_spectrum,
   wrong=lambda df, fm, n=1: fm_spectrum(fm, df, n) if df != fm else fm_spectrum(df, 2 * fm, n),
   random_inputs=lambda rng: (lambda fm: {"fm": fm, "df": fm * rng.uniform(0.2, 10), "n": rng.randint(0, 6)})(_logu(None, 10, 1e5, rng)),
   parse=lambda q: _parse(q, r"\bfm\b|frequency[- ]modulat|carson", [("df", "kind:frequency", ["peak (?:frequency )?deviation(?: of)?", "frequency deviation(?: of)?", "deviation(?: of)?"]),
                                                                    ("fm", "kind:frequency", ["modulating frequency(?: of)?", "modulation frequency(?: of)?", "message frequency(?: of)?", "audio frequency(?: of)?", "tone frequency(?: of)?", "baseband frequency(?: of)?"]),
                                                                    ("n", "ord", ["^(?:-|\\s)?sideband", r"sideband (?:number )?(?:n\s*=)?", r"\bn(?=\s*=)"])]),
   templates=["An FM signal has a peak frequency deviation of 75 kHz and a modulating frequency of 15 kHz. Find the Carson bandwidth."],
   examples=[{"inputs": {"df": 75000.0, "fm": 15000.0}, "outputs": {"carson_bandwidth": 180000.0, "beta": 5.0}},
             {"inputs": {"df": 2405.0, "fm": 1000.0, "n": 1}, "outputs": {"beta": 2.405}}],
   questions=[{"q": "An FM signal has a peak frequency deviation of 75 kHz and a modulating frequency of 15 kHz. Find the Carson bandwidth.", "outputs": {"carson_bandwidth": 180000.0}},
              {"q": "An FM signal has a frequency deviation of 5 kHz and a modulating frequency of 1 kHz. What is the amplitude of the 2nd sideband?", "outputs": {"beta": 5.0, "Jn": _besselj(2, 5.0)}},
              {"q": "An FM signal has a peak frequency deviation of 75 kHz. Find the Carson bandwidth.", "outcome": "NEEDS_INPUT"}])


# =================================================================================================== 10. digital communication
def _qf(x):
    return 0.5 * math.erfc(x / math.sqrt(2.0))


_EBN0 = ("ebn0_db", "db", [r"e_?b\s*/\s*n_?[0o](?: ratio)?(?: of)?", r"energy[- ]per[- ]bit[- ]to[- ]noise(?: power spectral density| density)?(?: ratio)?(?: of)?"])


def shannon_capacity(B, snr_db):
    B, snr_db = _pos(B, "bandwidth"), _fin(snr_db, "SNR in dB")
    C = B * math.log2(1 + 10 ** (snr_db / 10))
    return {"C": C, "efficiency": C / B}


def _cap_entropy(B, snr_db):
    import numpy as np
    from scipy.stats import norm
    s = 10 ** (snr_db / 10)
    h = norm(scale=np.sqrt(1 + s)).entropy() - norm(scale=1.0).entropy()        # mutual information per real sample, nats
    return float(2 * B * h / np.log(2))


_p("shannon_capacity", name="Shannon capacity of an AWGN channel", topic="communication links",
   purpose="The channel capacity C = B log2(1 + SNR) of a band-limited additive white Gaussian noise channel and the spectral efficiency C/B.",
   inputs=[_i("B", "bandwidth", "frequency"), _i("snr_db", "signal-to-noise ratio in dB")],
   outputs=[_o("C", "capacity in bit/s"), _o("efficiency", "spectral efficiency in bit/s per Hz")],
   primary="C", conditions=["additive white Gaussian noise", "SNR stated in dB"], method="Shannon-Hartley theorem",
   latex=r"C = B\log_2(1 + \mathrm{SNR})", run=shannon_capacity,
   reference=lambda B, snr_db: (lambda C: {"C": C, "efficiency": C / B})(_cap_entropy(B, snr_db)),
   wrong=lambda B, snr_db: {"C": B * math.log2(1 + abs(snr_db)), "efficiency": math.log2(1 + abs(snr_db))},
   random_inputs=lambda rng: {"B": _logu(None, 100, 1e9, rng), "snr_db": rng.uniform(-10, 50)},
   parse=lambda q: _parse(q, r"shannon|channel capacity|\bcapacity\b|maximum (?:data|bit|error-free) rate", [("B", "kind:frequency", ["bandwidth(?: of)?", "^(?:-|\\s)?(?:wide|channel|bandwidth)"]),
                                                                                                       ("snr_db", "db", [r"snr(?: of)?", "signal[- ]to[- ]noise ratio(?: of)?", r"s/n(?: of)?"])],
                          exclude=r"minimum snr|required snr|snr (?:is )?(?:needed|required)|eb/n0"),
   templates=["What is the Shannon capacity of a 3 kHz channel with an SNR of 30 dB?"],
   examples=[{"inputs": {"B": 3000.0, "snr_db": 30.0}, "outputs": {"C": 3000 * math.log2(1001)}},
             {"inputs": {"B": 1e6, "snr_db": 0.0}, "outputs": {"C": 1e6, "efficiency": 1.0}}],
   questions=[{"q": "What is the Shannon capacity of a 3 kHz channel with an SNR of 30 dB?", "outputs": {"C": 3000 * math.log2(1001)}},
              {"q": "Find the channel capacity for a bandwidth of 1 MHz and a signal-to-noise ratio of 0 dB.", "outputs": {"C": 1e6}},
              {"q": "What is the Shannon capacity of a 3 kHz channel with an SNR of 1000?", "outcome": "UNRECOGNIZED_FORMAT"},
              {"q": "What is the Shannon capacity of a 3 kHz channel?", "outcome": "NEEDS_INPUT"}])


def shannon_min_snr(C, B):
    C, B = _pos(C, "data rate"), _pos(B, "bandwidth")
    eta = C / B
    if eta > 200:
        raise ValueError("the spectral efficiency C/B must be at most 200 bit/s/Hz")
    s = math.expm1(eta * math.log(2))
    return {"snr_db": _db10(s), "snr": s, "ebn0_min_db": _db10(s / eta)}


def ref_shannon_min_snr(C, B):
    from scipy.optimize import brentq
    x = brentq(lambda d: _cap_entropy(B, d) - C, -150, 700, xtol=1e-14, rtol=8.9e-16, maxiter=500)
    s = 10 ** (x / 10)
    return {"snr_db": x, "snr": s, "ebn0_min_db": x - 10 * __import__("math").log10(C / B)}


_p("shannon_min_snr", name="Minimum SNR for a data rate (Shannon limit)", topic="communication links",
   purpose="The smallest SNR = 2^(C/B) - 1 that lets an AWGN channel of bandwidth B carry C bit/s, and the corresponding minimum Eb/N0.",
   inputs=[_i("C", "data rate in bit/s"), _i("B", "bandwidth", "frequency")],
   outputs=[_o("snr_db", "minimum SNR in dB"), _o("snr", "minimum SNR (linear)"), _o("ebn0_min_db", "minimum Eb/N0 in dB")],
   primary="snr_db", conditions=["additive white Gaussian noise", "C/B <= 200"], method="invert C = B log2(1 + SNR)",
   latex=r"\mathrm{SNR}_{\min} = 2^{C/B} - 1,\quad \left(\tfrac{E_b}{N_0}\right)_{\min} = \frac{2^{C/B}-1}{C/B}",
   run=shannon_min_snr, reference=ref_shannon_min_snr,
   wrong=lambda C, B: (lambda s: {"snr_db": _db10(s), "snr": s, "ebn0_min_db": _db10(s * B / C)})(2 ** (C / B)),
   random_inputs=lambda rng: (lambda B: {"B": B, "C": B * rng.uniform(0.05, 12)})(_logu(None, 100, 1e9, rng)),
   parse=lambda q: _parse(q, r"minimum snr|required snr|snr (?:is )?(?:needed|required)|minimum signal[- ]to[- ]noise|shannon limit",
                          [("B", "kind:frequency", ["bandwidth(?: of)?", "^(?:-|\\s)?(?:wide|channel|bandwidth)"]), ("C", "rate", ["data rate(?: of)?", "bit rate(?: of)?", "rate(?: of)?", ""])]),
   templates=["What minimum SNR is needed to send 10 Mbit/s over a bandwidth of 2 MHz?"],
   examples=[{"inputs": {"C": 1e7, "B": 2e6}, "outputs": {"snr": 31.0, "snr_db": 10 * math.log10(31)}},
             {"inputs": {"C": 1e6, "B": 1e6}, "outputs": {"snr": 1.0, "ebn0_min_db": 0.0}}],
   questions=[{"q": "What minimum SNR is needed to send 10 Mbit/s over a bandwidth of 2 MHz?", "outputs": {"snr": 31.0}},
              {"q": "What minimum SNR is needed to send 10 Mbit/s?", "outcome": "NEEDS_INPUT"}])


def bpsk_ber(ebn0_db):
    g = 10 ** (_fin(ebn0_db, "Eb/N0 in dB") / 10)
    return {"ber": 0.5 * math.erfc(math.sqrt(g)), "ber_bfsk": 0.5 * math.erfc(math.sqrt(g / 2))}


_p("bpsk_ber", name="Bit error rate of BPSK (and coherent BFSK) in AWGN", topic="digital communication",
   purpose="The bit error probability of coherent BPSK, Q(sqrt(2 Eb/N0)), and of coherent orthogonal BFSK, Q(sqrt(Eb/N0)), in additive white Gaussian noise.",
   inputs=[_i("ebn0_db", "Eb/N0 in dB")], outputs=[_o("ber", "BPSK bit error rate"), _o("ber_bfsk", "coherent BFSK bit error rate")],
   primary="ber", conditions=["coherent detection", "AWGN channel", "Eb/N0 stated in dB"], method="P_b = Q(sqrt(2 Eb/N0)) = erfc(sqrt(Eb/N0))/2",
   latex=r"P_b = Q\!\left(\sqrt{2E_b/N_0}\right) = \tfrac12\,\mathrm{erfc}\sqrt{E_b/N_0}", run=bpsk_ber,
   reference=lambda ebn0_db: (lambda g, sf: {"ber": float(sf((2 * g) ** 0.5)), "ber_bfsk": float(sf(g ** 0.5))})(10 ** (ebn0_db / 10), __import__("scipy.stats", fromlist=["norm"]).norm.sf),
   wrong=lambda ebn0_db: (lambda r: {"ber": r["ber_bfsk"], "ber_bfsk": r["ber_bfsk"] / 2})(bpsk_ber(ebn0_db)),
   random_inputs=lambda rng: {"ebn0_db": rng.uniform(-5, 12)},
   parse=lambda q: _parse(q, r"\bbpsk\b|binary phase[- ]shift|\bbfsk\b|binary (?:frequency[- ]shift|fsk)", [_EBN0], exclude=r"qpsk|qam|dpsk|non-?coherent|8-?psk|m-?psk"),
   templates=["What is the bit error rate of BPSK at Eb/N0 = 8 dB?"],
   examples=[{"inputs": {"ebn0_db": 8.0}, "outputs": {"ber": 0.5 * math.erfc(math.sqrt(10 ** 0.8))}},
             {"inputs": {"ebn0_db": 0.0}, "outputs": {"ber": 0.5 * math.erfc(1.0)}}],
   questions=[{"q": "What is the bit error rate of BPSK at Eb/N0 = 8 dB?", "outputs": {"ber": 0.5 * math.erfc(math.sqrt(10 ** 0.8))}},
              {"q": "What is the bit error rate of BPSK?", "outcome": "NEEDS_INPUT"}])


def qpsk_error_rates(ebn0_db):
    g = 10 ** (_fin(ebn0_db, "Eb/N0 in dB") / 10)
    q = _qf(math.sqrt(2 * g))
    return {"ber": q, "ser": q * (2 - q)}


_p("qpsk_error_rates", name="Bit and symbol error rates of QPSK in AWGN", topic="digital communication",
   purpose="Gray-coded coherent QPSK: bit error rate Q(sqrt(2 Eb/N0)) and symbol error rate 2Q - Q^2.",
   inputs=[_i("ebn0_db", "Eb/N0 in dB")], outputs=[_o("ber", "bit error rate (Gray coding)"), _o("ser", "symbol error rate")],
   primary="ser", conditions=["coherent detection", "Gray coding", "AWGN channel"], method="independent BPSK on I and Q: P_s = 1 - (1 - Q)^2",
   latex=r"P_b = Q\!\left(\sqrt{2E_b/N_0}\right),\quad P_s = 2Q - Q^2", run=qpsk_error_rates,
   reference=lambda ebn0_db: (lambda q: {"ber": q, "ser": float(1 - (1 - q) ** 2) if q > 1e-6 else 2 * q - q * q})(float(__import__("scipy.stats", fromlist=["norm"]).norm.sf((2 * 10 ** (ebn0_db / 10)) ** 0.5))),
   wrong=lambda ebn0_db: (lambda q: {"ber": q, "ser": q * (2 - q)})(_qf(math.sqrt(10 ** (ebn0_db / 10)))),
   random_inputs=lambda rng: {"ebn0_db": rng.uniform(-5, 12)},
   parse=lambda q: _parse(q, r"\bqpsk\b|quadrature phase[- ]shift|4-?psk", [_EBN0], exclude=r"bpsk|qam|dpsk|non-?coherent"),
   templates=["What are the bit and symbol error rates of QPSK at Eb/N0 = 6 dB?"],
   examples=[{"inputs": {"ebn0_db": 6.0}, "outputs": qpsk_error_rates(6.0)}],
   questions=[{"q": "What are the bit and symbol error rates of QPSK at Eb/N0 = 6 dB?", "outputs": qpsk_error_rates(6.0)}])


def noncoherent_ber(ebn0_db):
    g = 10 ** (_fin(ebn0_db, "Eb/N0 in dB") / 10)
    return {"ber_dpsk": 0.5 * math.exp(-g), "ber_ncfsk": 0.5 * math.exp(-g / 2)}


def ref_noncoherent_ber(ebn0_db):
    import numpy as np
    from scipy import integrate
    from scipy.stats import rice
    g = 10 ** (ebn0_db / 10)

    def orth(a):          # P(Rayleigh envelope > Rice envelope): noncoherent orthogonal detection, unit noise
        return integrate.quad(lambda r: rice.pdf(r, a) * np.exp(-r * r / 2), 0, np.inf, epsabs=0, epsrel=1e-13, limit=500)[0]
    return {"ber_ncfsk": orth(np.sqrt(2 * g)), "ber_dpsk": orth(np.sqrt(4 * g))}     # DPSK = orthogonal signalling with 2 Eb


_p("noncoherent_ber", name="Bit error rates of DPSK and noncoherent BFSK", topic="digital communication",
   purpose="The bit error rates of differentially coherent BPSK (DPSK), exp(-Eb/N0)/2, and of noncoherent orthogonal BFSK, exp(-Eb/2N0)/2, in AWGN.",
   inputs=[_i("ebn0_db", "Eb/N0 in dB")], outputs=[_o("ber_dpsk", "DPSK bit error rate"), _o("ber_ncfsk", "noncoherent BFSK bit error rate")],
   primary="ber_dpsk", conditions=["AWGN channel", "slow phase variation (DPSK)", "orthogonal tones (FSK)"], method="closed forms of noncoherent detection",
   latex=r"P_b^{\mathrm{DPSK}} = \tfrac12 e^{-E_b/N_0},\quad P_b^{\mathrm{NCFSK}} = \tfrac12 e^{-E_b/2N_0}", run=noncoherent_ber, reference=ref_noncoherent_ber,
   wrong=lambda ebn0_db: (lambda r: {"ber_dpsk": r["ber_ncfsk"], "ber_ncfsk": r["ber_dpsk"]})(noncoherent_ber(ebn0_db)),
   random_inputs=lambda rng: {"ebn0_db": rng.uniform(-3, 10)},
   parse=lambda q: _parse(q, r"\bdpsk\b|differential(?:ly)?[- ](?:coherent )?(?:binary )?(?:psk|phase)|non-?coherent", [_EBN0], exclude=r"\bqam\b"),
   templates=["What is the bit error rate of DPSK at Eb/N0 = 8 dB?"],
   examples=[{"inputs": {"ebn0_db": 10 * math.log10(2)}, "outputs": {"ber_dpsk": 0.5 * math.exp(-2), "ber_ncfsk": 0.5 * math.exp(-1)}}],
   questions=[{"q": "What is the bit error rate of DPSK at Eb/N0 = 8 dB?", "outputs": noncoherent_ber(8.0)},
              {"q": "Find the bit error rate of noncoherent FSK at Eb/N0 = 10 dB.", "outputs": noncoherent_ber(10.0)}])


_QAM_M = ["4", "16", "64", "256", "1024"]


def mqam_symbol_error(M, ebn0_db):
    M = int(M) if str(M) in _QAM_M else None
    if M is None:
        raise ValueError("M must be 4, 16, 64, 256 or 1024 (square QAM)")
    k = int(round(math.log2(M)))
    gs = 10 ** (_fin(ebn0_db, "Eb/N0 in dB") / 10) * k
    p = 2 * (1 - 1 / math.sqrt(M)) * _qf(math.sqrt(3 * gs / (M - 1)))
    ps = p * (2 - p)
    return {"ser": ps, "ber_approx": ps / k}


def ref_mqam_symbol_error(M, ebn0_db):
    import numpy as np
    from scipy.stats import norm
    M = int(M)
    L = int(round(np.sqrt(M)))
    lev = np.arange(-(L - 1), L, 2.0)                       # the constellation, minimum distance 2
    pts = np.array([(i, q) for i in lev for q in lev])
    Es = float(np.mean(np.sum(pts ** 2, axis=1)))
    sigma = np.sqrt(Es / (np.log2(M) * 10 ** (ebn0_db / 10)) / 2)
    e1 = lambda v: (norm.sf(1 / sigma) * (2 if abs(v) < L - 1 else 1))      # noqa: E731  per-dimension error of one level
    ps = float(np.mean([e1(i) + e1(q) - e1(i) * e1(q) for i, q in pts]))
    return {"ser": ps, "ber_approx": ps / np.log2(M)}


_p("mqam_symbol_error", name="Symbol error rate of square M-QAM in AWGN", topic="digital communication",
   purpose="The exact symbol error rate of coherent square M-QAM (4 to 1024 points) at a given Eb/N0, and the Gray-coded bit error rate approximation SER/log2 M.",
   inputs=[_i("M", "constellation size", type="choice", choices=_QAM_M), _i("ebn0_db", "Eb/N0 in dB")],
   outputs=[_o("ser", "symbol error rate"), _o("ber_approx", "bit error rate approximation (Gray coding)")],
   primary="ser", conditions=["square constellation", "coherent detection", "AWGN channel"],
   method="P_s = 1 - (1 - P_L)^2 with P_L = 2(1 - 1/sqrt(M)) Q(sqrt(3 Es / ((M-1) N0)))",
   latex=r"P_s = 1 - \left[1 - 2\left(1 - \tfrac{1}{\sqrt M}\right)Q\!\left(\sqrt{\tfrac{3E_s}{(M-1)N_0}}\right)\right]^2",
   run=mqam_symbol_error, reference=ref_mqam_symbol_error,
   wrong=lambda M, ebn0_db: (lambda Mi: (lambda p: {"ser": p * (2 - p), "ber_approx": p * (2 - p) / math.log2(Mi)})(2 * _qf(math.sqrt(3 * 10 ** (ebn0_db / 10) * math.log2(Mi) / (Mi - 1)))))(int(M)),
   random_inputs=lambda rng: (lambda M: {"M": M, "ebn0_db": rng.uniform(0, {"4": 8, "16": 12, "64": 16, "256": 20, "1024": 24}[M])})(rng.choice(_QAM_M)),
   parse=lambda q: _parse(q, r"\bqam\b", [_EBN0], special=lambda t: (lambda m: ({"M": m.group(1)}, [m.span(1)]) if m else ({}, []))(re.search(r"(\d+)\s*-?\s*qam\b", t, re.I))),
   templates=["What is the symbol error rate of 16-QAM at Eb/N0 = 10 dB?"],
   examples=[{"inputs": {"M": "16", "ebn0_db": 10.0}, "outputs": mqam_symbol_error("16", 10.0)},
             {"inputs": {"M": "4", "ebn0_db": 6.0}, "outputs": {"ser": qpsk_error_rates(6.0)["ser"]}}],
   questions=[{"q": "What is the symbol error rate of 16-QAM at Eb/N0 = 10 dB?", "outputs": mqam_symbol_error("16", 10.0)},
              {"q": "What is the symbol error rate of 32-QAM at Eb/N0 = 10 dB?", "outcome": "INVALID_INPUT"},
              {"q": "What is the symbol error rate of 64-QAM?", "outcome": "NEEDS_INPUT"}])


_PSK_M = ["2", "4", "8", "16", "32", "64"]


def mpsk_symbol_error(M, ebn0_db):
    import mpmath
    if str(M) not in _PSK_M:
        raise ValueError("M must be 2, 4, 8, 16, 32 or 64")
    M = int(M)
    gs = 10 ** (_fin(ebn0_db, "Eb/N0 in dB") / 10) * math.log2(M)
    with mpmath.workdps(30):
        s2 = mpmath.sin(mpmath.pi / M) ** 2
        v = mpmath.quad(lambda t: mpmath.exp(-gs * s2 / mpmath.sin(t) ** 2), [0, mpmath.pi / 2, (M - 1) * mpmath.pi / M]) / mpmath.pi
    ps = float(v)
    return {"ser": ps, "ber_approx": ps / math.log2(M)}


def ref_mpsk_symbol_error(M, ebn0_db):
    import numpy as np
    from scipy import integrate
    from scipy.stats import norm
    M = int(M)
    g = 10 ** (ebn0_db / 10) * np.log2(M)
    # the pdf of the received phase of a PSK symbol in AWGN; P_s = 1 - P(|phase| < pi/M)
    p = lambda f: (np.exp(-g) + np.sqrt(4 * np.pi * g) * np.cos(f) * np.exp(-g * np.sin(f) ** 2) * norm.cdf(np.sqrt(2 * g) * np.cos(f))) / (2 * np.pi)   # noqa: E731
    pc = 2 * integrate.quad(p, 0, np.pi / M, epsabs=0, epsrel=1e-13, limit=500)[0]
    ps = 1 - pc
    return {"ser": float(ps), "ber_approx": float(ps / np.log2(M))}


_p("mpsk_symbol_error", name="Symbol error rate of M-PSK in AWGN", topic="digital communication",
   purpose="The exact symbol error rate of coherent M-PSK (M = 2 to 64) at a given Eb/N0 (Craig's integral), and the Gray-coded bit error rate approximation.",
   inputs=[_i("M", "number of phases", type="choice", choices=_PSK_M), _i("ebn0_db", "Eb/N0 in dB")],
   outputs=[_o("ser", "symbol error rate"), _o("ber_approx", "bit error rate approximation SER/log2 M")],
   primary="ser", conditions=["coherent detection", "AWGN channel"], method="Craig's integral evaluated to 30 digits",
   latex=r"P_s = \frac{1}{\pi}\int_0^{(M-1)\pi/M} \exp\!\left(-\frac{E_s}{N_0}\frac{\sin^2(\pi/M)}{\sin^2\theta}\right)d\theta",
   run=mpsk_symbol_error, reference=ref_mpsk_symbol_error,
   wrong=lambda M, ebn0_db: mpsk_symbol_error(M, ebn0_db - 10 * math.log10(2)),             # Es/N0 taken as half its value
   random_inputs=lambda rng: (lambda M: {"M": M, "ebn0_db": rng.uniform(-2, {"2": 6, "4": 6, "8": 9, "16": 13, "32": 17, "64": 21}[M])})(rng.choice(_PSK_M)),
   parse=lambda q: _parse(q, r"\bpsk\b", [_EBN0], exclude=r"\bbpsk\b|\bqpsk\b|\bdpsk\b",
                          special=lambda t: (lambda m: ({"M": m.group(1)}, [m.span(1)]) if m else ({}, []))(re.search(r"(\d+)\s*-?\s*psk\b", t, re.I))),
   templates=["What is the symbol error rate of 8-PSK at Eb/N0 = 10 dB?"],
   examples=[{"inputs": {"M": "2", "ebn0_db": 4.0}, "outputs": {"ser": bpsk_ber(4.0)["ber"]}},
             {"inputs": {"M": "4", "ebn0_db": 5.0}, "outputs": {"ser": qpsk_error_rates(5.0)["ser"]}},
             {"inputs": {"M": "8", "ebn0_db": 10.0}, "outputs": mpsk_symbol_error("8", 10.0)}],
   questions=[{"q": "What is the symbol error rate of 8-PSK at Eb/N0 = 10 dB?", "outputs": mpsk_symbol_error("8", 10.0)},
              {"q": "What is the symbol error rate of 8-PSK?", "outcome": "NEEDS_INPUT"}])


# =================================================================================================== 11. noise and links
def noise_figure_cascade(nf_db, gain_db):
    nf_db, gain_db = _vec(nf_db, "noise figures in dB", 1, 100), _vec(gain_db, "gains in dB", 1, 100)
    if len(nf_db) != len(gain_db):
        raise ValueError("give one noise figure and one gain per stage")
    if any(v < 0 for v in nf_db):
        raise ValueError("a noise figure in dB is never negative")
    F, g = 10 ** (nf_db[0] / 10), 10 ** (gain_db[0] / 10)
    terms = [F]
    for nf, gd in zip(nf_db[1:], gain_db[1:]):
        terms.append((10 ** (nf / 10) - 1) / g)
        g *= 10 ** (gd / 10)
    Ft = math.fsum(terms)
    return {"nf_db": _db10(Ft), "F": Ft, "Te": T0 * (Ft - 1), "gain_db": math.fsum(gain_db)}


def ref_noise_figure_cascade(nf_db, gain_db):
    import numpy as np
    import skrf
    f = skrf.Frequency(1, 1, 1, "GHz")
    net = None
    for nf, g in zip(nf_db, gain_db):            # matched noisy two-ports, cascaded with noise correlation matrices
        s = np.zeros((1, 2, 2), complex)
        s[0, 1, 0] = 10 ** (g / 20)
        n = skrf.Network(frequency=f, s=s, z0=50)
        n.set_noise_a(f, nfmin_db=nf, gamma_opt=0, rn=1)
        net = n if net is None else net ** n
    F = float(np.real(net.nf(50)[0]))
    return {"nf_db": float(10 * np.log10(F)), "F": F, "Te": 290.0 * (F - 1), "gain_db": float(np.sum(gain_db))}


_p("noise_figure_cascade", name="Noise figure of cascaded stages (Friis formula)", topic="noise and link budgets",
   purpose="The overall noise figure, noise factor, equivalent noise temperature and gain of a chain of matched stages with given noise figures and gains.",
   inputs=[_i("nf_db", "noise figure of each stage in dB, input first", type="vector", min_length=1, max_length=100),
           _i("gain_db", "available gain of each stage in dB", type="vector", min_length=1, max_length=100)],
   outputs=[_o("nf_db", "overall noise figure in dB"), _o("F", "overall noise factor"), _o("Te", "equivalent input noise temperature", "temperature"),
            _o("gain_db", "overall gain in dB")],
   primary="nf_db", conditions=["impedance-matched stages", "noise figures referred to T0 = 290 K", "one gain per stage"],
   method="Friis: F = F1 + (F2 - 1)/G1 + (F3 - 1)/(G1 G2) + ...", latex=r"F = F_1 + \frac{F_2 - 1}{G_1} + \frac{F_3 - 1}{G_1 G_2} + \cdots",
   run=noise_figure_cascade, reference=ref_noise_figure_cascade,
   wrong=lambda nf_db, gain_db: (lambda d: {"nf_db": d, "F": 10 ** (d / 10), "Te": T0 * (10 ** (d / 10) - 1), "gain_db": math.fsum(gain_db)})(math.fsum(nf_db)),
   random_inputs=lambda rng: (lambda k: {"nf_db": [round(rng.uniform(0.5, 10), 2) for _ in range(k)], "gain_db": [round(rng.uniform(-6, 30), 1) for _ in range(k)]})(rng.randint(2, 5)),
   parse=lambda q: _parse(q, r"noise figure|noise factor", [("nf_db", "vec", [r"noise figures?(?: of)?", r"\bnf(?: values)?(?: of)?", r"\bnfs?(?=\s*[=:])"]), ("gain_db", "vec", [r"gains?(?: of)?"])],
                          require=r"\[", exclude=r"\bdbm\b"),
   templates=["Find the overall noise figure of a cascade with noise figures [2, 6, 10] dB and gains [15, 20, 10] dB."],
   examples=[{"inputs": {"nf_db": "2, 6, 10", "gain_db": "15, 20, 10"}, "outputs": noise_figure_cascade([2, 6, 10], [15, 20, 10])},
             {"inputs": {"nf_db": [3, 3], "gain_db": [10, 10]}, "outputs": {"F": 10 ** 0.3 + (10 ** 0.3 - 1) / 10, "gain_db": 20.0}}],
   questions=[{"q": "Find the overall noise figure of a cascade with noise figures [2, 6, 10] dB and gains [15, 20, 10] dB.", "outputs": noise_figure_cascade([2, 6, 10], [15, 20, 10])},
              {"q": "Find the overall noise figure of a cascade with noise figures [2, 6] dB and gains [15] dB.", "outcome": "INVALID_INPUT"}])


def _fspl_ratio(d, f):
    return (4 * math.pi * d * f / C0) ** 2


def _ref_iso_link(f, d):
    """Pr/Pt between two isotropic antennas, from the fields of two short dipoles: radiated power and radiation
    resistance by integrating the far field over the sphere, received power from the open-circuit voltage."""
    import numpy as np
    from scipy import constants, integrate
    eta = np.sqrt(constants.mu_0 / constants.epsilon_0)
    k = 2 * np.pi * f / constants.c
    l = 1e-3 * constants.c / f
    E = lambda r, th: eta * k * l * np.sin(th) / (4 * np.pi * r)          # noqa: E731  far field of a 1 A element
    P = integrate.dblquad(lambda th, ph: E(1.0, th) ** 2 / (2 * eta) * np.sin(th), 0, 2 * np.pi, 0, np.pi, epsabs=0, epsrel=1e-13)[0]
    D = 4 * np.pi * (E(1.0, np.pi / 2) ** 2 / (2 * eta)) / P                # directivity, found numerically
    Rrad = 2 * P
    Pr = (E(d, np.pi / 2) * l) ** 2 / (8 * Rrad)
    return Pr / P / (D * D)


def free_space_path_loss(d, f):
    d, f = _pos(d, "distance"), _pos(f, "frequency")
    if d < C0 / f:
        raise ValueError("the distance must be at least one wavelength (far field)")
    L = _fspl_ratio(d, f)
    return {"loss_db": _db10(L), "loss": L, "wavelength": C0 / f}


_p("free_space_path_loss", name="Free-space path loss", topic="noise and link budgets",
   purpose="The loss (4 pi d f / c)^2 between isotropic antennas a distance d apart at frequency f, in dB and as a ratio.",
   inputs=[_i("d", "distance", "length"), _i("f", "frequency", "frequency")],
   outputs=[_o("loss_db", "free-space path loss in dB"), _o("loss", "path loss as a power ratio"), _o("wavelength", "wavelength", "length")],
   primary="loss_db", conditions=["free space, line of sight", "far field (d >= one wavelength)", "isotropic antennas"],
   method="FSPL = (4 pi d / lambda)^2", latex=r"L_{\mathrm{fs}} = \left(\frac{4\pi d f}{c}\right)^2",
   run=free_space_path_loss,
   reference=lambda d, f: (lambda r: {"loss_db": float(-10 * __import__("math").log10(r)), "loss": 1 / r, "wavelength": __import__("scipy.constants", fromlist=["c"]).c / f})(_ref_iso_link(f, d)),
   wrong=lambda d, f: (lambda L: {"loss_db": _db10(math.sqrt(L)), "loss": math.sqrt(L), "wavelength": C0 / f})(_fspl_ratio(d, f)),
   random_inputs=lambda rng: (lambda f: {"f": f, "d": C0 / f * _logu(None, 2, 1e8, rng)})(_logu(None, 1e6, 1e11, rng)),
   parse=lambda q: _parse(q, r"free[- ]space (?:path )?loss|\bfspl\b|path loss", [("d", "kind:length", ["distance(?: of)?", "over", "range(?: of)?", "^(?:away|apart|link)"]),
                                                                                ("f", "kind:frequency", ["frequency(?: of)?", "at", "^(?:link|signal|carrier)", ""])],
                          exclude=r"gain|dbi|transmit|received"),
   templates=["What is the free-space path loss at 2.4 GHz over a distance of 100 m?"],
   examples=[{"inputs": {"d": 100.0, "f": 2.4e9}, "outputs": {"loss_db": 20 * math.log10(4 * math.pi * 100 * 2.4e9 / C0)}}],
   questions=[{"q": "What is the free-space path loss at 2.4 GHz over a distance of 100 m?", "outputs": {"loss_db": 20 * math.log10(4 * math.pi * 100 * 2.4e9 / C0)}},
              {"q": "What is the free-space path loss at 2.4 GHz over a distance of 1 cm?", "outcome": "INVALID_INPUT"},
              {"q": "What is the free-space path loss at 2.4 GHz?", "outcome": "NEEDS_INPUT"}])


def friis_received_power(Pt, Gt_db, Gr_db, f, d):
    Pt, f, d = _pos(Pt, "transmit power"), _pos(f, "frequency"), _pos(d, "distance")
    Gt_db, Gr_db = _fin(Gt_db, "transmit antenna gain"), _fin(Gr_db, "receive antenna gain")
    if d < C0 / f:
        raise ValueError("the distance must be at least one wavelength (far field)")
    pr_dbm = _db10(Pt / 1e-3) + Gt_db + Gr_db - _db10(_fspl_ratio(d, f))
    return {"Pr_dbm": pr_dbm, "Pr": 1e-3 * 10 ** (pr_dbm / 10)}


def ref_friis(Pt, Gt_db, Gr_db, f, d):
    import numpy as np
    pr = Pt * 10 ** (Gt_db / 10) * 10 ** (Gr_db / 10) * _ref_iso_link(f, d)
    return {"Pr_dbm": float(10 * np.log10(pr / 1e-3)), "Pr": float(pr)}


def _dbm_power(text):
    m = re.search(r"(?i:transmit(?:ted|ter)? power|power transmitted|transmits|radiates)\s*(?:of\s+|is\s+|=\s*)?(" + _N + r")\s*(?i:dBm)(?![A-Za-z])", text)
    if not m:
        return {}, []
    return {"Pt": f"{10 ** (float(m.group(1)) / 10)} mW"}, [m.span()]


_p("friis_received_power", name="Received power of a radio link (Friis transmission equation)", topic="noise and link budgets",
   purpose="The power received over a free-space link: Pr = Pt Gt Gr (lambda / 4 pi d)^2, in dBm and watts.",
   inputs=[_i("Pt", "transmit power", "power"), _i("Gt_db", "transmit antenna gain in dBi"), _i("Gr_db", "receive antenna gain in dBi"),
           _i("f", "frequency", "frequency"), _i("d", "distance", "length")],
   outputs=[_o("Pr_dbm", "received power in dBm"), _o("Pr", "received power", "power")],
   primary="Pr_dbm", conditions=["free space, line of sight", "far field", "matched, co-polarized antennas"],
   method="link budget in dB: Pt + Gt + Gr - FSPL", latex=r"P_r = P_t G_t G_r \left(\frac{\lambda}{4\pi d}\right)^2",
   run=friis_received_power, reference=ref_friis,
   wrong=lambda Pt, Gt_db, Gr_db, f, d: (lambda p: {"Pr_dbm": p, "Pr": 1e-3 * 10 ** (p / 10)})(_db10(Pt / 1e-3) + Gt_db + Gr_db - 10 * math.log10(4 * math.pi * d * f / C0)),
   random_inputs=lambda rng: (lambda f: {"Pt": _logu(None, 1e-3, 1e3, rng), "Gt_db": rng.uniform(0, 40), "Gr_db": rng.uniform(0, 40), "f": f, "d": C0 / f * _logu(None, 5, 1e8, rng)})(_logu(None, 1e7, 1e11, rng)),
   parse=lambda q: _parse(q, r"received power|power received|receive[sd]? power|link budget|friis",
                          [("Pt", "kind:power", ["transmit(?:ted|ter)? power(?: of)?", "power transmitted", "transmits", "radiates"]),
                           ("Gt_db", "dbi", ["transmit(?:ting)? (?:antenna )?gain(?: of)?", "gain of the transmit(?:ting)? antenna(?: of)?"]),
                           ("Gr_db", "dbi", ["receiv(?:e|ing) (?:antenna )?gain(?: of)?", "gain of the receiv(?:e|ing) antenna(?: of)?"]),
                           ("f", "kind:frequency", ["frequency(?: of)?", "at"]), ("d", "kind:length", ["distance(?: of)?", "over", "range(?: of)?"])],
                          special=_dbm_power, exclude=r"radar|cross[- ]section"),
   templates=["Find the received power for a transmit power of 1 W, a transmit antenna gain of 10 dBi, a receive antenna gain of 5 dBi, a frequency of 2.4 GHz and a distance of 1 km."],
   examples=[{"inputs": {"Pt": 1.0, "Gt_db": 10.0, "Gr_db": 5.0, "f": 2.4e9, "d": 1000.0}, "outputs": friis_received_power(1.0, 10.0, 5.0, 2.4e9, 1000.0)}],
   questions=[{"q": "Find the received power for a transmit power of 1 W, a transmit antenna gain of 10 dBi, a receive antenna gain of 5 dBi, a frequency of 2.4 GHz and a distance of 1 km.",
               "outputs": friis_received_power(1.0, 10.0, 5.0, 2.4e9, 1000.0)},
              {"q": "Find the received power for a transmit power of 30 dBm, a transmit antenna gain of 10 dBi, a receive antenna gain of 5 dBi, a frequency of 2.4 GHz and a distance of 1 km.",
               "outputs": friis_received_power(1.0, 10.0, 5.0, 2.4e9, 1000.0)},
              {"q": "Find the received power for a transmit power of 1 W, a transmit antenna gain of 10 dBi, a frequency of 2.4 GHz and a distance of 1 km.", "outcome": "NEEDS_INPUT"}])


def dish_antenna_gain(D, f, eta):
    D, f, eta = _pos(D, "dish diameter"), _pos(f, "frequency"), _pos(eta, "aperture efficiency")
    if eta > 1:
        raise ValueError("the aperture efficiency must be at most 1 (100 %)")
    lam = C0 / f
    if D < lam:
        raise ValueError("the aperture must be at least one wavelength across")
    G = eta * (math.pi * D / lam) ** 2
    return {"G_dbi": _db10(G), "G": G, "Ae": eta * math.pi * D * D / 4}


def ref_dish_antenna_gain(D, f, eta):
    import numpy as np
    from scipy import constants, integrate
    lam = constants.c / f
    a = D / 2
    # directivity of a uniformly illuminated circular aperture: 4 pi |integral E dA|^2 / (lambda^2 integral |E|^2 dA)
    s1 = integrate.dblquad(lambda r, p: r, 0, 2 * np.pi, 0, a, epsabs=0, epsrel=1e-13)[0]
    s2 = integrate.dblquad(lambda r, p: 1.0 * r, 0, 2 * np.pi, 0, a, epsabs=0, epsrel=1e-13)[0]
    G = eta * 4 * np.pi * s1 ** 2 / (lam ** 2 * s2)
    return {"G_dbi": float(10 * np.log10(G)), "G": float(G), "Ae": float(eta * s1)}


_p("dish_antenna_gain", name="Gain of a parabolic dish (aperture) antenna", topic="antennas",
   purpose="The gain eta (pi D / lambda)^2 of a circular aperture antenna of diameter D and aperture efficiency eta, in dBi, and its effective aperture.",
   inputs=[_i("D", "dish diameter", "length"), _i("f", "frequency", "frequency"), _i("eta", "aperture efficiency (0 < eta <= 1)")],
   outputs=[_o("G_dbi", "gain in dBi"), _o("G", "gain as a ratio"), _o("Ae", "effective aperture", "area")],
   primary="G_dbi", conditions=["0 < eta <= 1", "D >= one wavelength"], method="G = eta (pi D / lambda)^2 = 4 pi Ae / lambda^2",
   latex=r"G = \eta\left(\frac{\pi D}{\lambda}\right)^2", run=dish_antenna_gain, reference=ref_dish_antenna_gain,
   wrong=lambda D, f, eta: (lambda G: {"G_dbi": _db10(G), "G": G, "Ae": eta * math.pi * D * D / 4})(eta * math.pi * D * f / C0),
   random_inputs=lambda rng: (lambda f: {"f": f, "D": C0 / f * _logu(None, 2, 500, rng), "eta": round(rng.uniform(0.4, 0.8), 2)})(_logu(None, 1e9, 1e11, rng)),
   parse=lambda q: _parse(q, r"\bdish\b|parabolic|reflector antenna|aperture antenna", [("D", "kind:length", ["diameter(?: of)?", "^(?:-|\\s)?(?:diameter|dish)"]),
                                                                                        ("f", "kind:frequency", ["frequency(?: of)?", "at", "operating at"]),
                                                                                        ("eta", "frac", ["(?:aperture )?efficiency(?: of)?", "^(?:-|\\s)?(?:aperture )?efficiency", "^efficient"])]),
   templates=["What is the gain of a parabolic dish with a diameter of 3 m at 10 GHz and an aperture efficiency of 55%?"],
   examples=[{"inputs": {"D": 3.0, "f": 1e10, "eta": 0.55}, "outputs": dish_antenna_gain(3.0, 1e10, 0.55)}],
   questions=[{"q": "What is the gain of a parabolic dish with a diameter of 3 m at 10 GHz and an aperture efficiency of 55%?", "outputs": dish_antenna_gain(3.0, 1e10, 0.55)},
              {"q": "What is the gain of a parabolic dish with a diameter of 3 m at 10 GHz?", "outcome": "NEEDS_INPUT"}])


# =================================================================================================== 12. radar
def _radar_k(Pt, G_db, f, sigma):
    Pt, f, sigma = _pos(Pt, "peak transmit power"), _pos(f, "frequency"), _pos(sigma, "radar cross-section")
    G = 10 ** (_fin(G_db, "antenna gain in dBi") / 10)
    lam = C0 / f
    return Pt * G * G * lam * lam * sigma / (4 * math.pi) ** 3


def radar_received_power(Pt, G_db, f, sigma, R):
    R = _pos(R, "range")
    pr = _radar_k(Pt, G_db, f, sigma) / R ** 4
    return {"Pr": pr, "Pr_dbm": _db10(pr / 1e-3)}


def _ref_radar_pr(Pt, G_db, f, sigma, R):
    import numpy as np
    lam = 299792458.0 / f
    Gs = np.sqrt(4 * np.pi * sigma) / lam          # the target as a matched re-radiating antenna of gain Gs (sigma = Gs^2 lambda^2 / 4 pi)
    G = 10 ** (G_db / 10)
    leg = _ref_iso_link(f, R) * G * Gs              # one way, from the dipole-field link
    return Pt * leg * leg


def radar_max_range(Pt, G_db, f, sigma, Pmin):
    Pmin = _pos(Pmin, "minimum detectable signal")
    return {"Rmax": (_radar_k(Pt, G_db, f, sigma) / Pmin) ** 0.25}


def ref_radar_max_range(Pt, G_db, f, sigma, Pmin):
    import numpy as np
    from scipy.optimize import brentq
    x = brentq(lambda lr: np.log(_ref_radar_pr(Pt, G_db, f, sigma, np.exp(lr))) - np.log(Pmin), np.log(1e-2), np.log(1e10), xtol=1e-15, rtol=8.9e-16)
    return {"Rmax": float(np.exp(x))}


_RADAR = [("Pmin", "kind:power", ["minimum detectable (?:signal|power)(?: of)?", "receiver sensitivity(?: of)?", "minimum (?:received |detectable )?power(?: of)?"]),
          ("Pt", "kind:power", ["peak (?:transmit(?:ted)? )?power(?: of)?", "transmit(?:ted|ter)? power(?: of)?", "transmits", "power(?: of)?"]),
          ("sigma", "kind:area", ["radar cross[- ]section(?: of)?", r"\brcs(?: of)?", "cross[- ]section(?: of)?"]),
          ("G_db", "dbi", ["antenna gain(?: of)?", "gain(?: of)?"]), ("f", "kind:frequency", ["frequency(?: of)?", "operat(?:es|ing) at", "at", "^radar"])]
_p("radar_received_power", name="Received echo power (radar range equation)", topic="radar",
   purpose="The echo power Pr = Pt G^2 lambda^2 sigma / ((4 pi)^3 R^4) of a monostatic radar from a target of cross-section sigma at range R.",
   inputs=[_i("Pt", "peak transmit power", "power"), _i("G_db", "antenna gain in dBi (transmit and receive)"), _i("f", "frequency", "frequency"),
           _i("sigma", "radar cross-section", "area"), _i("R", "range to the target", "length")],
   outputs=[_o("Pr", "received echo power", "power"), _o("Pr_dbm", "received echo power in dBm")],
   primary="Pr", conditions=["monostatic radar, one antenna", "free space, no losses", "far field"],
   method="radar range equation", latex=r"P_r = \frac{P_t G^2 \lambda^2 \sigma}{(4\pi)^3 R^4}", run=radar_received_power,
   reference=lambda Pt, G_db, f, sigma, R: (lambda pr: {"Pr": float(pr), "Pr_dbm": float(10 * __import__("math").log10(pr / 1e-3))})(_ref_radar_pr(Pt, G_db, f, sigma, R)),
   wrong=lambda Pt, G_db, f, sigma, R: (lambda pr: {"Pr": pr, "Pr_dbm": _db10(pr / 1e-3)})(_radar_k(Pt, G_db, f, sigma) / R ** 2),
   random_inputs=lambda rng: {"Pt": _logu(None, 10, 1e6, rng), "G_db": rng.uniform(10, 45), "f": _logu(None, 1e8, 1e11, rng), "sigma": _logu(None, 1e-3, 1e3, rng), "R": _logu(None, 100, 1e6, rng)},
   parse=lambda q: _parse(q, r"radar", _RADAR[1:] + [("R", "kind:length", ["range(?: of)?", "distance(?: of)?", "target at", "^(?:away)"])],
                          require=r"received (?:echo )?power|echo power|power received|returned power", exclude=r"maximum range|max(?:imum)? detection range|doppler"),
   templates=["A radar with a peak power of 1 MW and an antenna gain of 40 dBi operates at 3 GHz. What is the received echo power from a target with a radar cross-section of 1 m^2 at a range of 100 km?"],
   examples=[{"inputs": {"Pt": 1e6, "G_db": 40.0, "f": 3e9, "sigma": 1.0, "R": 1e5}, "outputs": radar_received_power(1e6, 40.0, 3e9, 1.0, 1e5)}],
   questions=[{"q": "A radar with a peak power of 1 MW and an antenna gain of 40 dBi operates at 3 GHz. What is the received echo power from a target with a radar cross-section of 1 m^2 at a range of 100 km?",
               "outputs": radar_received_power(1e6, 40.0, 3e9, 1.0, 1e5)},
              {"q": "A radar with a peak power of 1 MW operates at 3 GHz. What is the received echo power from a target with a radar cross-section of 1 m^2 at a range of 100 km?", "outcome": "NEEDS_INPUT"}])

_p("radar_max_range", name="Maximum detection range of a radar", topic="radar",
   purpose="The largest range R = (Pt G^2 lambda^2 sigma / ((4 pi)^3 Smin))^(1/4) at which the echo still reaches the minimum detectable signal.",
   inputs=[_i("Pt", "peak transmit power", "power"), _i("G_db", "antenna gain in dBi"), _i("f", "frequency", "frequency"),
           _i("sigma", "radar cross-section", "area"), _i("Pmin", "minimum detectable signal power", "power")],
   outputs=[_o("Rmax", "maximum detection range", "length")], primary="Rmax",
   conditions=["monostatic radar", "free space, no losses"], method="radar range equation solved for R",
   latex=r"R_{\max} = \left(\frac{P_t G^2 \lambda^2 \sigma}{(4\pi)^3 S_{\min}}\right)^{1/4}", run=radar_max_range, reference=ref_radar_max_range,
   wrong=lambda Pt, G_db, f, sigma, Pmin: {"Rmax": (_radar_k(Pt, G_db, f, sigma) / Pmin) ** 0.5},
   random_inputs=lambda rng: {"Pt": _logu(None, 10, 1e6, rng), "G_db": rng.uniform(10, 45), "f": _logu(None, 1e8, 1e11, rng), "sigma": _logu(None, 1e-3, 1e3, rng), "Pmin": _logu(None, 1e-16, 1e-10, rng)},
   parse=lambda q: _parse(q, r"radar", _RADAR, require=r"maximum (?:detection )?range|max(?:imum)? range|detection range|how far"),
   templates=["What is the maximum range of a radar with a peak power of 1 MW, an antenna gain of 40 dBi, a frequency of 3 GHz, a radar cross-section of 1 m^2 and a minimum detectable signal of 1e-13 W?"],
   examples=[{"inputs": {"Pt": 1e6, "G_db": 40.0, "f": 3e9, "sigma": 1.0, "Pmin": 1e-13}, "outputs": radar_max_range(1e6, 40.0, 3e9, 1.0, 1e-13)}],
   questions=[{"q": "What is the maximum range of a radar with a peak power of 1 MW, an antenna gain of 40 dBi, a frequency of 3 GHz, a radar cross-section of 1 m^2 and a minimum detectable signal of 1e-13 W?",
               "outputs": radar_max_range(1e6, 40.0, 3e9, 1.0, 1e-13)}])


def radar_doppler(v, f):
    v, f = _fin(v, "radial velocity"), _pos(f, "frequency")
    if abs(v) >= C0 / 10:
        raise ValueError("the radial velocity must be far below the speed of light")
    return {"fd": 2 * v * f / (C0 - v), "wavelength": C0 / f}


def ref_radar_doppler(v, f):
    from fractions import Fraction as Fr
    c, v, f, R0 = Fr(299792458), Fr(v), Fr(f), Fr(1000)
    # exact arrival times of two successive wave crests reflected by a target approaching at v (rational arithmetic)
    def back(te):
        tm = (c * te + R0) / (c + v)                    # the crest meets the target
        return tm + (R0 - v * tm) / c                   # and returns to the radar
    fr = 1 / (back(1 / f) - back(Fr(0)))
    return {"fd": float(fr - f), "wavelength": float(c / f)}


_p("radar_doppler", name="Doppler shift of a radar echo", topic="radar",
   purpose="The two-way Doppler shift of the echo from a target closing at radial velocity v (negative when receding): fd = 2 v f / (c - v), essentially 2 v / lambda.",
   inputs=[_i("v", "radial velocity, positive when approaching", "speed"), _i("f", "transmitted frequency", "frequency")],
   outputs=[_o("fd", "Doppler shift of the echo", "frequency"), _o("wavelength", "wavelength", "length")],
   primary="fd", conditions=["monostatic radar", "|v| much smaller than c"], method="two-way Doppler of a reflector: f_r = f (c + v)/(c - v)",
   latex=r"f_d = \frac{2vf}{c - v} \approx \frac{2v}{\lambda}", run=radar_doppler, reference=ref_radar_doppler,
   wrong=lambda v, f: {"fd": v * f / C0, "wavelength": C0 / f},
   random_inputs=lambda rng: {"v": rng.choice([-1, 1]) * _logu(None, 0.1, 3000, rng), "f": _logu(None, 1e8, 1e11, rng)},
   parse=lambda q: _parse(q, r"doppler", [("v", "kind:speed", ["radial (?:speed|velocity)(?: of)?", "speed(?: of)?", "velocity(?: of)?", "moving at", "approaching at", "closing at", ""]),
                                         ("f", "kind:frequency", ["frequency(?: of)?", "operating at", "^radar", "at", ""])],
                          require=r"radar|echo|target", exclude=r"reced|moving away|away from"),
   templates=["A 10 GHz radar sees a target approaching at 30 m/s. What is the Doppler shift?"],
   examples=[{"inputs": {"v": 30.0, "f": 1e10}, "outputs": {"fd": 2 * 30 * 1e10 / (C0 - 30)}}],
   questions=[{"q": "A 10 GHz radar sees a target approaching at 30 m/s. What is the Doppler shift?", "outputs": {"fd": 2 * 30 * 1e10 / (C0 - 30)}},
              {"q": "What is the Doppler shift of the radar echo from a target approaching at 30 m/s?", "outcome": "NEEDS_INPUT"}])


# =================================================================================================== 13. transmission lines
_ZUNIT = r"(?:ohms?|Ω)"


def _load_special(text):
    m = re.search(r"(" + _N + r")\s*([+-])\s*j\s*(" + _UN + r")\s*" + _ZUNIT, text)
    if not m:
        m2 = re.search(r"(" + _N + r")\s*([+-])\s*(" + _UN + r")\s*j\s*" + _ZUNIT, text)
        if not m2:
            return {}, []
        m = m2
    return {"RL": f"{m.group(1)} ohm", "XL": f"{('-' if m.group(2) == '-' else '')}{m.group(3)} ohm"}, [m.span()]


_Z0 = ("Z0", "kind:impedance", ["characteristic impedance(?: of)?", r"\bZ_?0(?=\s*=)", "^(?:-|\\s)?(?:line|cable|transmission line|system|feed ?line|coax)"])
_RL = ("RL", "kind:impedance", ["load (?:resistance|impedance)(?: of)?", r"\bR_?L(?=\s*=)", r"\bZ_?L(?=\s*=)", "load of", "terminated (?:in|with|by)(?: a)?", "^(?:-|\\s)?(?:load|resistor|termination)"])


def _gamma(RL, XL, Z0):
    RL, XL, Z0 = _pos(RL, "load resistance"), _fin(XL, "load reactance"), _pos(Z0, "characteristic impedance")
    d = (RL + Z0) ** 2 + XL * XL
    return complex(((RL - Z0) * (RL + Z0) + XL * XL) / d + 0.0, 2 * Z0 * XL / d + 0.0), Z0


def reflection_coefficient(RL, Z0, XL=0.0):
    g, Z0 = _gamma(RL, XL, Z0)
    a = abs(g)
    return {"gamma_mag": a, "gamma_angle_deg": math.degrees(math.atan2(g.imag, g.real)) + 0.0 if a else 0.0,
            "gamma_re": g.real, "gamma_im": g.imag, "vswr": (1 + a) / (1 - a),
            "return_loss_db": -_db20(a) if a else INF, "mismatch_loss_db": -_db10(1 - a * a)}


def ref_reflection_coefficient(RL, Z0, XL=0.0):
    import numpy as np
    from skrf import mathFunctions as mf
    from skrf import tlineFunctions as tl
    g = complex(np.ravel(tl.zl_2_Gamma0(Z0, complex(RL, XL)))[0])
    return {"gamma_mag": abs(g), "gamma_angle_deg": float(np.degrees(np.angle(g))), "gamma_re": g.real, "gamma_im": g.imag,
            "vswr": float(np.real(np.ravel(tl.zl_2_swr(Z0, complex(RL, XL)))[0])), "return_loss_db": float(np.ravel(-mf.complex_2_db(g))[0]),
            "mismatch_loss_db": float(-mf.mag_2_db10(1 - abs(g) ** 2))}


_p("reflection_coefficient", name="Reflection coefficient, VSWR and return loss of a load", topic="transmission lines",
   purpose="For a load ZL = RL + jXL on a line of characteristic impedance Z0: the reflection coefficient (magnitude, angle, parts), VSWR, return loss and mismatch loss.",
   inputs=[_i("RL", "load resistance", "impedance"), _i("Z0", "characteristic impedance of the line", "impedance"),
           _i("XL", "load reactance (0 if not given)", "impedance", required=False)],
   outputs=[_o("gamma_mag", "|Gamma|"), _o("gamma_angle_deg", "angle of Gamma in degrees"), _o("gamma_re", "real part of Gamma"), _o("gamma_im", "imaginary part of Gamma"),
            _o("vswr", "voltage standing wave ratio"), _o("return_loss_db", "return loss in dB"), _o("mismatch_loss_db", "mismatch loss in dB")],
   primary="gamma_mag", conditions=["RL > 0", "Z0 real (lossless line)"], method="Gamma = (ZL - Z0)/(ZL + Z0); VSWR = (1 + |Gamma|)/(1 - |Gamma|)",
   latex=r"\Gamma = \frac{Z_L - Z_0}{Z_L + Z_0},\quad \mathrm{VSWR} = \frac{1 + |\Gamma|}{1 - |\Gamma|}", run=reflection_coefficient, reference=ref_reflection_coefficient,
   wrong=lambda RL, Z0, XL=0.0: (lambda r: dict(r, gamma_re=-r["gamma_re"], gamma_im=-r["gamma_im"], gamma_angle_deg=r["gamma_angle_deg"] - math.copysign(180, r["gamma_angle_deg"] or 1)))(reflection_coefficient(RL, Z0, XL)),
   random_inputs=lambda rng: {"RL": _logu(None, 1, 1000, rng), "Z0": rng.choice([50.0, 75.0, 300.0, _logu(None, 20, 200, rng)]), "XL": rng.uniform(-300, 300)},
   parse=lambda q: _parse(q, r"reflection coefficient|\bvswr\b|standing wave ratio|return loss|mismatch loss", [_Z0, _RL], special=_load_special,
                          require=r"\bload\b|terminat|Z_?L\s*="),
   templates=["A 50 ohm line is terminated in a load of 75 + j25 ohm. Find the reflection coefficient and the VSWR."],
   examples=[{"inputs": {"RL": 75.0, "XL": 25.0, "Z0": 50.0}, "outputs": reflection_coefficient(75.0, 50.0, 25.0)},
             {"inputs": {"RL": 100.0, "Z0": 50.0}, "outputs": {"gamma_mag": 1 / 3, "vswr": 2.0, "gamma_angle_deg": 0.0}}],
   questions=[{"q": "A 50 ohm line is terminated in a load of 75 + j25 ohm. Find the reflection coefficient and the VSWR.", "outputs": reflection_coefficient(75.0, 50.0, 25.0)},
              {"q": "What is the VSWR of a 100 ohm load on a 50 ohm line?", "outputs": {"vswr": 2.0}},
              {"q": "A 50 ohm line is terminated in a load of -10 ohm. Find the reflection coefficient.", "outcome": "INVALID_INPUT"}])


def vswr_from_reflection(gamma):
    a = _nonneg(gamma, "reflection coefficient magnitude")
    if a >= 1:
        raise ValueError("|Gamma| must be below 1")
    return {"vswr": (1 + a) / (1 - a), "return_loss_db": -_db20(a) if a else INF, "mismatch_loss_db": -_db10(1 - a * a)}


_p("vswr_from_reflection", name="VSWR and return loss from the reflection coefficient", topic="transmission lines",
   purpose="The VSWR (1 + |Gamma|)/(1 - |Gamma|), the return loss -20 log10 |Gamma| and the mismatch loss from the magnitude of the reflection coefficient.",
   inputs=[_i("gamma", "magnitude of the reflection coefficient (0 <= |Gamma| < 1)")],
   outputs=[_o("vswr", "voltage standing wave ratio"), _o("return_loss_db", "return loss in dB"), _o("mismatch_loss_db", "mismatch loss in dB")],
   primary="vswr", conditions=["0 <= |Gamma| < 1"], method="VSWR = (1 + |Gamma|)/(1 - |Gamma|)", latex=r"\mathrm{VSWR} = \frac{1 + |\Gamma|}{1 - |\Gamma|}",
   run=vswr_from_reflection,
   reference=lambda gamma: (lambda tl, mf: {"vswr": float(tl.Gamma0_2_swr(gamma)), "return_loss_db": float(-mf.mag_2_db(gamma)), "mismatch_loss_db": float(-mf.mag_2_db10(1 - gamma ** 2))})(
       __import__("skrf").tlineFunctions, __import__("skrf").mathFunctions),
   wrong=lambda gamma: {"vswr": (1 - gamma) / (1 + gamma), "return_loss_db": -_db20(gamma) if gamma else INF, "mismatch_loss_db": -_db10(1 - gamma * gamma)},
   random_inputs=lambda rng: {"gamma": rng.uniform(0.01, 0.95)},
   parse=lambda q: _parse(q, r"\bvswr\b|standing wave ratio|\bswr\b|return loss", [("gamma", "num", [r"reflection coefficient(?: magnitude)?(?: of)?", r"\|?gamma\|?(?=\s*=)", r"\|Γ\|(?=\s*=)"])],
                          exclude=r"\bload\b|terminat|Z_?L\s*=|ohm|Ω"),
   templates=["What is the VSWR for a reflection coefficient of 0.2?"],
   examples=[{"inputs": {"gamma": 0.2}, "outputs": {"vswr": 1.5, "return_loss_db": -20 * math.log10(0.2)}}],
   questions=[{"q": "What is the VSWR for a reflection coefficient of 0.2?", "outputs": {"vswr": 1.5}},
              {"q": "What is the VSWR for a reflection coefficient of 1.2?", "outcome": "INVALID_INPUT"}])


def reflection_from_vswr(vswr):
    s = _pos(vswr, "VSWR")
    if s < 1:
        raise ValueError("the VSWR is at least 1")
    a = (s - 1) / (s + 1)
    return {"gamma": a, "return_loss_db": -_db20(a) if a else INF, "mismatch_loss_db": -_db10(1 - a * a)}


def ref_reflection_from_vswr(vswr):
    from scipy.optimize import brentq
    from skrf import mathFunctions as mf
    from skrf import tlineFunctions as tl
    a = brentq(lambda g: float(tl.Gamma0_2_swr(g)) - vswr, 0.0, 1 - 1e-12, xtol=1e-300, rtol=8.9e-16)
    return {"gamma": a, "return_loss_db": float(-mf.mag_2_db(a)), "mismatch_loss_db": float(-mf.mag_2_db10(1 - a * a))}


_p("reflection_from_vswr", name="Reflection coefficient and return loss from the VSWR", topic="transmission lines",
   purpose="The reflection coefficient magnitude (VSWR - 1)/(VSWR + 1), the return loss and the mismatch loss for a given VSWR.",
   inputs=[_i("vswr", "voltage standing wave ratio (>= 1)")],
   outputs=[_o("gamma", "|Gamma|"), _o("return_loss_db", "return loss in dB"), _o("mismatch_loss_db", "mismatch loss in dB")],
   primary="gamma", conditions=["VSWR >= 1"], method="|Gamma| = (VSWR - 1)/(VSWR + 1)", latex=r"|\Gamma| = \frac{\mathrm{VSWR} - 1}{\mathrm{VSWR} + 1}",
   run=reflection_from_vswr, reference=ref_reflection_from_vswr,
   wrong=lambda vswr: (lambda a: {"gamma": a, "return_loss_db": -_db20(a), "mismatch_loss_db": -_db10(1 - a * a) if a < 1 else INF})((vswr - 1) / vswr),
   random_inputs=lambda rng: {"vswr": rng.uniform(1.05, 20)},
   parse=lambda q: _parse(q, r"reflection coefficient|return loss|mismatch loss", [("vswr", "num", [r"vswr(?: of)?", r"standing wave ratio(?: of)?", r"\bswr(?: of)?"])],
                          require=r"vswr|standing wave|\bswr\b", exclude=r"\bload\b|terminat|Z_?L\s*=|ohm|Ω"),
   templates=["What reflection coefficient and return loss correspond to a VSWR of 2?"],
   examples=[{"inputs": {"vswr": 2.0}, "outputs": {"gamma": 1 / 3, "return_loss_db": 20 * math.log10(3)}}],
   questions=[{"q": "What reflection coefficient and return loss correspond to a VSWR of 2?", "outputs": {"gamma": 1 / 3}},
              {"q": "What reflection coefficient corresponds to a VSWR of 0.5?", "outcome": "INVALID_INPUT"}])


def input_impedance(RL, Z0, length, XL=0.0):
    g, Z0 = _gamma(RL, XL, Z0)
    length = _nonneg(length, "line length in wavelengths")
    gi = g * cmath.exp(complex(0, -4 * math.pi * length))
    z = _snap(Z0 * (1 + gi) / (1 - gi), Z0)          # an exact quarter-wave line gives Xin = 0, not rounding residue
    return {"Rin": z.real + 0.0, "Xin": z.imag + 0.0, "abs_zin": abs(z)}


def _abcd_zin(Z0, ZL, bl):
    """Input impedance from the transmission (ABCD) matrix of a lossless line of electrical length bl (numpy)."""
    import numpy as np
    T = np.array([[np.cos(bl), 1j * Z0 * np.sin(bl)], [1j * np.sin(bl) / Z0, np.cos(bl)]])
    v = T @ np.array([ZL, 1.0])                       # (V, I) at the input for V = ZL, I = 1 at the load
    return complex(v[0] / v[1])


def ref_input_impedance(RL, Z0, length, XL=0.0):
    import numpy as np
    z = _abcd_zin(Z0, complex(RL, XL), 2 * np.pi * length)
    return {"Rin": z.real, "Xin": z.imag, "abs_zin": abs(z)}


def _wl_special(text):
    m = re.search(r"(" + _UN + r")\s*(?:wavelengths?|λ|lambda)\b", text, re.I)
    return ({"length": float(m.group(1))}, [m.span()]) if m else ({}, [])


_p("input_impedance", name="Input impedance of a terminated lossless line", topic="transmission lines",
   purpose="The impedance seen at the input of a lossless line of characteristic impedance Z0 and electrical length l (in wavelengths) terminated in ZL = RL + jXL.",
   inputs=[_i("RL", "load resistance", "impedance"), _i("Z0", "characteristic impedance", "impedance"), _i("length", "line length in wavelengths"),
           _i("XL", "load reactance (0 if not given)", "impedance", required=False)],
   outputs=[_o("Rin", "input resistance", "impedance"), _o("Xin", "input reactance", "impedance"), _o("abs_zin", "|Zin|", "impedance")],
   primary="Rin", conditions=["lossless line", "RL > 0", "length >= 0 in wavelengths"],
   method="Gamma moved along the line: Gamma_in = Gamma_L e^{-j 4 pi l / lambda}, Zin = Z0 (1 + Gamma_in)/(1 - Gamma_in)",
   latex=r"Z_{in} = Z_0\frac{Z_L + jZ_0\tan\beta l}{Z_0 + jZ_L\tan\beta l}", run=input_impedance, reference=ref_input_impedance,
   wrong=lambda RL, Z0, length, XL=0.0: (lambda g: (lambda gi: (lambda z: {"Rin": z.real, "Xin": z.imag, "abs_zin": abs(z)})(g[1] * (1 + gi) / (1 - gi)))(g[0] * cmath.exp(complex(0, 4 * math.pi * length))))(_gamma(RL, XL, Z0)),
   random_inputs=lambda rng: {"RL": _logu(None, 5, 500, rng), "Z0": rng.choice([50.0, 75.0, _logu(None, 20, 200, rng)]), "XL": rng.uniform(-200, 200), "length": rng.uniform(0.02, 0.48) + rng.randint(0, 3) * 0.5},
   parse=lambda q: _parse(q, r"input impedance", [_Z0, _RL], special=lambda t: (lambda a, b: (dict(a[0], **b[0]), a[1] + b[1]))(_load_special(t), _wl_special(t)),
                          exclude=r"quarter[- ]wave"),
   templates=["What is the input impedance of a 0.125 wavelength 50 ohm line terminated in a load of 100 + j0 ohm?"],
   examples=[{"inputs": {"RL": 100.0, "Z0": 50.0, "length": 0.25}, "outputs": {"Rin": 25.0, "Xin": 0.0}},
             {"inputs": {"RL": 100.0, "Z0": 50.0, "length": 0.125}, "outputs": input_impedance(100.0, 50.0, 0.125)}],
   questions=[{"q": "What is the input impedance of a 0.125 wavelength 50 ohm line terminated in a load of 100 + j0 ohm?", "outputs": input_impedance(100.0, 50.0, 0.125)},
              {"q": "What is the input impedance of a 0.25 wavelength 50 ohm line terminated in a 100 ohm load?", "outputs": {"Rin": 25.0}},
              {"q": "What is the input impedance of a 50 ohm line terminated in a 100 ohm load?", "outcome": "NEEDS_INPUT"}])


def quarter_wave_transformer(RL, Z0, f=None, vf=1.0):
    RL, Z0 = _pos(RL, "load resistance"), _pos(Z0, "line impedance to match")
    vf = _pos(vf, "velocity factor")
    if vf > 1:
        raise ValueError("the velocity factor must be at most 1")
    out = {"Zt": math.sqrt(RL * Z0)}
    if f is not None:
        out["length"] = vf * C0 / (4 * _pos(f, "frequency"))
    return out


def ref_quarter_wave_transformer(RL, Z0, f=None, vf=1.0):
    import numpy as np
    import skrf
    from scipy.optimize import brentq
    lo, hi = min(RL, Z0), max(RL, Z0)
    zt = lo if lo == hi else brentq(lambda z: _abcd_zin(z, RL, np.pi / 2).real - Z0, lo, hi, xtol=1e-300, rtol=8.9e-16)
    out = {"Zt": zt}
    if f is not None:
        fs = skrf.media.Freespace(frequency=skrf.Frequency(f, f, 1, "Hz"), ep_r=1 / vf ** 2)
        out["length"] = float(2 * np.pi / abs(np.imag(fs.gamma[0])) / 4)
    return out


_p("quarter_wave_transformer", name="Quarter-wave impedance transformer", topic="transmission lines",
   purpose="The characteristic impedance sqrt(RL Z0) of the quarter-wave section that matches a resistive load RL to a line Z0, and its physical length at frequency f.",
   inputs=[_i("RL", "load resistance", "impedance"), _i("Z0", "impedance to match to", "impedance"), _i("f", "design frequency", "frequency", required=False),
           _i("vf", "velocity factor of the section (default 1)", required=False)],
   outputs=[_o("Zt", "characteristic impedance of the transformer", "impedance"), _o("length", "physical length of the section (f given)", "length")],
   primary="Zt", conditions=["resistive load", "lossless lines", "exact match at the design frequency only"],
   method="Zin of a quarter-wave line is Zt^2 / RL; set it equal to Z0", latex=r"Z_t = \sqrt{R_L Z_0},\quad \ell = \frac{v_f\,c}{4f}",
   run=quarter_wave_transformer, reference=ref_quarter_wave_transformer,
   wrong=lambda RL, Z0, f=None, vf=1.0: dict(quarter_wave_transformer(RL, Z0, f, vf), Zt=(RL + Z0) / 2) if RL != Z0 else {"Zt": 2 * RL},
   random_inputs=lambda rng: {"RL": _logu(None, 5, 500, rng), "Z0": rng.choice([50.0, 75.0]), "f": _logu(None, 1e6, 1e10, rng), "vf": round(rng.uniform(0.6, 1.0), 2)},
   parse=lambda q: _parse(q, r"quarter[- ]wave", [("RL", "kind:impedance", ["load (?:resistance|impedance)(?: of)?", r"\bR_?L(?=\s*=)", "^(?:-|\\s)?(?:load|resistive load|antenna)"]),
                                                 ("Z0", "kind:impedance", ["characteristic impedance(?: of)?", r"\bZ_?0(?=\s*=)", "^(?:-|\\s)?(?:line|cable|feed ?line|system|source|coax)"]),
                                                 ("f", "kind:frequency", ["design frequency(?: of)?", "frequency(?: of)?", "at"]), ("vf", "frac", ["velocity factor(?: of)?"])]),
   templates=["Design a quarter-wave transformer to match a 100 ohm load to a 50 ohm line at 1 GHz."],
   examples=[{"inputs": {"RL": 100.0, "Z0": 50.0, "f": 1e9}, "outputs": {"Zt": math.sqrt(5000), "length": C0 / 4e9}},
             {"inputs": {"RL": 300.0, "Z0": 75.0, "f": 1e8, "vf": 0.66}, "outputs": {"Zt": 150.0, "length": 0.66 * C0 / 4e8}}],
   questions=[{"q": "Design a quarter-wave transformer to match a 100 ohm load to a 50 ohm line at 1 GHz.", "outputs": {"Zt": math.sqrt(5000), "length": C0 / 4e9}},
              {"q": "What impedance must a quarter-wave transformer have to match a 300 ohm load to a 75 ohm line?", "outputs": {"Zt": 150.0}},
              {"q": "Design a quarter-wave transformer to match a 100 ohm load.", "outcome": "NEEDS_INPUT"}])


def coax_impedance(D, d, er=1.0):
    D, d, er = _pos(D, "outer conductor diameter"), _pos(d, "inner conductor diameter"), _pos(er, "relative permittivity")
    if d >= D:
        raise ValueError("the inner diameter must be smaller than the outer diameter")
    if er < 1:
        raise ValueError("the relative permittivity is at least 1")
    eta0 = math.sqrt(MU0 / EPS0)
    return {"Z0": eta0 / (2 * math.pi * math.sqrt(er)) * math.log(D / d), "vf": 1 / math.sqrt(er)}


def ref_coax_impedance(D, d, er=1.0):
    import numpy as np
    import skrf
    c = skrf.media.Coaxial(frequency=skrf.Frequency(1, 1, 1, "GHz"), Dint=d, Dout=D, epsilon_r=er, sigma=1e300, tan_delta=0)
    return {"Z0": float(np.real(c.z0[0])), "vf": float(1 / np.sqrt(er))}


_p("coax_impedance", name="Characteristic impedance of a coaxial line", topic="transmission lines",
   purpose="The characteristic impedance (eta0 / (2 pi sqrt(er))) ln(D/d) of a lossless coaxial line and its velocity factor 1/sqrt(er).",
   inputs=[_i("D", "inner diameter of the outer conductor", "length"), _i("d", "diameter of the inner conductor", "length"),
           _i("er", "relative permittivity of the dielectric (1 if not given)", required=False)],
   outputs=[_o("Z0", "characteristic impedance", "impedance"), _o("vf", "velocity factor")],
   primary="Z0", conditions=["d < D", "er >= 1", "lossless conductors and dielectric"], method="Z0 = sqrt(L'/C') with L' = mu ln(D/d)/(2 pi), C' = 2 pi eps / ln(D/d)",
   latex=r"Z_0 = \frac{\eta_0}{2\pi\sqrt{\varepsilon_r}}\ln\frac{D}{d}", run=coax_impedance, reference=ref_coax_impedance,
   wrong=lambda D, d, er=1.0: {"Z0": math.sqrt(MU0 / EPS0) / (2 * math.pi * math.sqrt(er)) * math.log10(D / d), "vf": 1 / math.sqrt(er)},
   random_inputs=lambda rng: (lambda d: {"d": d, "D": d * rng.uniform(1.5, 10), "er": round(rng.uniform(1, 10), 2)})(_logu(None, 1e-4, 1e-2, rng)),
   parse=lambda q: _parse(q, r"\bcoax", [("D", "kind:length", ["outer (?:conductor )?diameter(?: of)?", "(?:inner )?diameter of the outer conductor(?: of)?", "shield diameter(?: of)?"]),
                                        ("d", "kind:length", ["inner (?:conductor )?diameter(?: of)?", "diameter of the inner conductor(?: of)?", "cent(?:er|re) conductor diameter(?: of)?"]),
                                        ("er", "num", ["dielectric constant(?: of)?", "relative permittivity(?: of)?", r"ε_?r\s*=", r"\beps(?:ilon)?_?r(?=\s*=)"])],
                          require=r"characteristic impedance|impedance"),
   templates=["What is the characteristic impedance of a coaxial cable with an inner conductor diameter of 1 mm, an outer conductor diameter of 3.5 mm and a dielectric constant of 2.25?"],
   examples=[{"inputs": {"D": 3.5e-3, "d": 1e-3, "er": 2.25}, "outputs": coax_impedance(3.5e-3, 1e-3, 2.25)}],
   questions=[{"q": "What is the characteristic impedance of a coaxial cable with an inner conductor diameter of 1 mm, an outer conductor diameter of 3.5 mm and a dielectric constant of 2.25?",
               "outputs": coax_impedance(3.5e-3, 1e-3, 2.25)},
              {"q": "What is the characteristic impedance of a coaxial cable with an inner conductor diameter of 4 mm and an outer conductor diameter of 3.5 mm?", "outcome": "INVALID_INPUT"}])


# =================================================================================================== 14. filter design
def _cpoly(roots):
    c = [1 + 0j]
    for r in roots:
        c = [u - r * v for u, v in zip(c + [0j], [0j] + c)]
    return c



def _r12(x: float) -> float:
    """12 significant digits (and no negative zero): filter coefficients built from cos/sin differ in the last
    bit between Windows and Linux libm, and an answer must be identical on every platform."""
    return 0.0 if abs(x) < 1e-15 else float(f"{x:.12g}") + 0.0

def butterworth_design(N, fc, fs, btype):
    N = _int(N, "order", 1, 8)
    fc, fs = _pos(fc, "cutoff frequency"), _pos(fs, "sampling rate")
    if btype not in ("lowpass", "highpass"):
        raise ValueError("btype must be lowpass or highpass")
    if fc >= fs / 2:
        raise ValueError("the cutoff must lie below fs/2")
    fs2 = 2 * fs
    wo = fs2 * math.tan(math.pi * fc / fs)                         # pre-warped cutoff
    p = [-cmath.exp(1j * math.pi * m / (2 * N)) for m in range(-N + 1, N, 2)]   # analog prototype poles
    if btype == "lowpass":
        p = [wo * v for v in p]
        k = wo ** N
        z = []
    else:
        k = 1.0
        for v in p:
            k /= -v
        k = k.real
        p = [wo / v for v in p]
        z = [0j] * N
    pz = [(fs2 + v) / (fs2 - v) for v in p]
    zz = [(fs2 + v) / (fs2 - v) for v in z] + [-1 + 0j] * (len(p) - len(z))
    num = 1 + 0j
    for v in z:
        num *= (fs2 - v)
    den = 1 + 0j
    for v in p:
        den *= (fs2 - v)
    kz = k * (num / den).real
    return {"b": [_r12((kz * v).real) for v in _cpoly(zz)], "a": [_r12(v.real) for v in _cpoly(pz)]}


def _btype_special(text):
    lo, hi = re.search(r"low[- ]?pass", text, re.I), re.search(r"high[- ]?pass", text, re.I)
    if lo and hi:
        return None
    return ({"btype": "lowpass"} if lo else {"btype": "highpass"} if hi else {}), []


_p("butterworth_design", name="Digital Butterworth filter design (bilinear transform)", topic="filter design",
   purpose="The coefficients b and a of an N-th order digital Butterworth low-pass or high-pass filter with cutoff fc at sampling rate fs (bilinear transform with pre-warping).",
   inputs=[_i("N", "filter order (1..8)", type="integer", min=1, max=8), _i("fc", "cutoff (-3 dB) frequency", "frequency"), _i("fs", "sampling rate", "frequency"),
           _i("btype", "lowpass or highpass", type="choice", choices=["lowpass", "highpass"])],
   outputs=[_o("b", "numerator coefficients b[0..N]"), _o("a", "denominator coefficients a[0..N] (a[0] = 1)")],
   primary="b", conditions=["0 < fc < fs/2", "1 <= N <= 8"],
   method="analog prototype poles, frequency scaling with the pre-warped cutoff 2 fs tan(pi fc / fs), bilinear transform, polynomial expansion",
   latex=r"s = 2f_s\frac{1 - z^{-1}}{1 + z^{-1}},\quad \omega_c = 2f_s\tan\frac{\pi f_c}{f_s}", run=butterworth_design,
   reference=lambda N, fc, fs, btype: (lambda ba: {"b": [float(v) for v in ba[0]], "a": [float(v) for v in ba[1]]})(__import__(_SIG, fromlist=["butter"]).butter(N, fc, btype=btype, fs=fs)),
   wrong=lambda N, fc, fs, btype: butterworth_design(N, fs / math.pi * math.atan(math.pi * fc / fs) if False else min(fs / 2.0001, 2 * fc), fs, btype) if 2 * fc < fs / 2 else butterworth_design(N, fc / 2, fs, btype),
   random_inputs=lambda rng: {"N": rng.randint(1, 6), "fs": (fs := _logu(None, 100, 1e6, rng)), "fc": fs * rng.uniform(0.02, 0.4), "btype": rng.choice(["lowpass", "highpass"])},
   parse=lambda q: _parse(q, r"butterworth", [("N", "ord", ["order(?: of)?", r"\bN(?=\s*=)", "^-?\\s?order", "^-?\\s?pole"]), ("fs", "kind:frequency", _FS),
                                             ("fc", "kind:frequency", ["cutoff frequency(?: of)?", "cut-off frequency(?: of)?", "cutoff(?: of)?", r"~+\s*frequency(?: of)?", "corner frequency(?: of)?"])],
                          special=_btype_special, require=r"design|coefficients"),
   templates=["Design a 2nd-order Butterworth low-pass filter with a cutoff frequency of 1 kHz at a sampling rate of 8 kHz. Give the coefficients."],
   examples=[{"inputs": {"N": 2, "fc": 1000.0, "fs": 8000.0, "btype": "lowpass"}, "outputs": butterworth_design(2, 1000.0, 8000.0, "lowpass")},
             {"inputs": {"N": 1, "fc": 2000.0, "fs": 8000.0, "btype": "lowpass"}, "outputs": {"b": [0.5, 0.5], "a": [1.0, 0.0]}}],
   questions=[{"q": "Design a 2nd-order Butterworth low-pass filter with a cutoff frequency of 1 kHz at a sampling rate of 8 kHz. Give the coefficients.", "outputs": butterworth_design(2, 1000.0, 8000.0, "lowpass")},
              {"q": "Design a 3rd-order Butterworth high-pass filter with a cutoff frequency of 500 Hz at a sampling rate of 8 kHz.", "outputs": butterworth_design(3, 500.0, 8000.0, "highpass")},
              {"q": "Design a 2nd-order Butterworth low-pass filter with a cutoff frequency of 5 kHz at a sampling rate of 8 kHz.", "outcome": "INVALID_INPUT"}])


def fir_window_design(N, fc, fs, window):
    N = _int(N, "number of taps", 1, 10001)
    fc, fs = _pos(fc, "cutoff frequency"), _pos(fs, "sampling rate")
    if fc >= fs / 2:
        raise ValueError("the cutoff must lie below fs/2")
    w = window_coefficients(window, N)["w"] if N > 1 else [1.0]
    c = 2 * fc / fs
    alpha = (N - 1) / 2
    h = []
    for n in range(N):
        x = c * (n - alpha)
        h.append(c * (1.0 if x == 0 else math.sin(math.pi * x) / (math.pi * x)) * w[n])
    s = math.fsum(h)
    return {"h": [_r12(v / s) for v in h]}


_p("fir_window_design", name="FIR low-pass filter design by the window method", topic="filter design",
   purpose="The N taps of a linear-phase FIR low-pass filter with cutoff fc: an ideal (sinc) impulse response shifted by (N-1)/2, multiplied by a rectangular, Hann, Hamming or Blackman window and scaled to unit DC gain.",
   inputs=[_i("N", "number of taps", type="integer", min=1), _i("fc", "cutoff frequency", "frequency"), _i("fs", "sampling rate", "frequency"),
           _i("window", "window type", type="choice", choices=_WINDOWS)],
   outputs=[_o("h", "filter taps h[0..N-1]")], primary="h", conditions=["0 < fc < fs/2", "symmetric window", "taps scaled so that the DC gain is 1"],
   method="windowed ideal low-pass: h[n] = (2fc/fs) sinc(2fc/fs (n - (N-1)/2)) w[n], normalized to sum 1",
   latex=r"h[n] \propto \frac{2f_c}{f_s}\,\mathrm{sinc}\!\left(\frac{2f_c}{f_s}\left(n - \tfrac{N-1}{2}\right)\right) w[n]",
   run=fir_window_design,
   reference=lambda N, fc, fs, window: {"h": [float(v) for v in __import__(_SIG, fromlist=["firwin"]).firwin(N, fc, window=_WIN_NAME[window], fs=fs)]},
   wrong=lambda N, fc, fs, window: (lambda c: {"h": [v * c for v in fir_window_design(N, fc, fs, window)["h"]]})(1.0 / (2 * fc / fs) if fc / fs != 0.5 else 3.0),
   random_inputs=lambda rng: {"N": rng.randint(3, 41), "fs": (fs := _logu(None, 100, 1e6, rng)), "fc": fs * rng.uniform(0.03, 0.45), "window": rng.choice(_WINDOWS)},
   parse=lambda q: _parse(q, r"\bfir\b", [("N", "int", ["^-?\\s?taps?", "^-?\\s?(?:tap|coefficient|point)", r"number of taps(?: of)?", "length(?: of)?", r"\bN(?=\s*=)"]),
                                         ("fs", "kind:frequency", _FS), ("fc", "kind:frequency", ["cutoff frequency(?: of)?", "cut-off frequency(?: of)?", "cutoff(?: of)?", "corner frequency(?: of)?"])],
                          special=_win_special, require=r"design|window method|windowed"),
   templates=["Design a 5-tap FIR low-pass filter with a Hamming window, a cutoff frequency of 1 kHz and a sampling rate of 8 kHz."],
   examples=[{"inputs": {"N": 5, "fc": 1000.0, "fs": 8000.0, "window": "hamming"}, "outputs": fir_window_design(5, 1000.0, 8000.0, "hamming")},
             {"inputs": {"N": 3, "fc": 2000.0, "fs": 8000.0, "window": "rectangular"}, "outputs": {"h": [2 / (4 + math.pi), math.pi / (4 + math.pi), 2 / (4 + math.pi)]}}],
   questions=[{"q": "Design a 5-tap FIR low-pass filter with a Hamming window, a cutoff frequency of 1 kHz and a sampling rate of 8 kHz.", "outputs": fir_window_design(5, 1000.0, 8000.0, "hamming")},
              {"q": "Design a 5-tap FIR low-pass filter with a cutoff frequency of 1 kHz and a sampling rate of 8 kHz.", "outcome": "NEEDS_INPUT"}])


def notch_filter_design(f0, Q, fs):
    f0, Q, fs = _pos(f0, "notch frequency"), _pos(Q, "quality factor"), _pos(fs, "sampling rate")
    if f0 >= fs / 2:
        raise ValueError("the notch frequency must lie below fs/2")
    w0 = 2 * math.pi * f0 / fs
    beta = math.tan(w0 / Q / 2)
    g = 1 / (1 + beta)
    return {"b": [g, -2 * g * math.cos(w0), g], "a": [1.0, -2 * g * math.cos(w0), 2 * g - 1]}


_p("notch_filter_design", name="Second-order IIR notch filter design", topic="filter design",
   purpose="The coefficients of a second-order IIR notch filter removing frequency f0 with quality factor Q (bandwidth f0/Q at -3 dB) at sampling rate fs.",
   inputs=[_i("f0", "frequency to remove", "frequency"), _i("Q", "quality factor f0 / bandwidth"), _i("fs", "sampling rate", "frequency")],
   outputs=[_o("b", "numerator coefficients b0, b1, b2"), _o("a", "denominator coefficients 1, a1, a2")], primary="b",
   conditions=["0 < f0 < fs/2", "Q > 0"], method="zeros on the unit circle at f0; poles at the same angle, radius set by tan(w0 / 2Q)",
   latex=r"H(z) = g\,\frac{1 - 2\cos\omega_0 z^{-1} + z^{-2}}{1 - 2g\cos\omega_0 z^{-1} + (2g - 1)z^{-2}},\ g = \frac{1}{1 + \tan(\omega_0/2Q)}",
   run=notch_filter_design,
   reference=lambda f0, Q, fs: (lambda ba: {"b": [float(v) for v in ba[0]], "a": [float(v) for v in ba[1]]})(__import__(_SIG, fromlist=["iirnotch"]).iirnotch(f0, Q, fs=fs)),
   wrong=lambda f0, Q, fs: notch_filter_design(f0, 1 / Q if Q != 1 else 2.0, fs),
   random_inputs=lambda rng: {"fs": (fs := _logu(None, 100, 1e6, rng)), "f0": fs * rng.uniform(0.02, 0.45), "Q": rng.uniform(2, 50)},
   parse=lambda q: _parse(q, r"\bnotch\b", [("fs", "kind:frequency", _FS), ("f0", "kind:frequency", ["notch frequency(?: of)?", "remove", "reject", "at", "centred at", "centered at"]),
                                          ("Q", "num", ["quality factor(?: of)?", r"\bQ(?=\s*=)", r"\bQ of"])]),
   templates=["Design a notch filter at 60 Hz with a quality factor of 30 for a sampling rate of 1 kHz."],
   examples=[{"inputs": {"f0": 60.0, "Q": 30.0, "fs": 1000.0}, "outputs": notch_filter_design(60.0, 30.0, 1000.0)}],
   questions=[{"q": "Design a notch filter at 60 Hz with a quality factor of 30 for a sampling rate of 1 kHz.", "outputs": notch_filter_design(60.0, 30.0, 1000.0)},
              {"q": "Design a notch filter at 600 Hz with a quality factor of 30 for a sampling rate of 1 kHz.", "outcome": "INVALID_INPUT"}])


# =================================================================================================== 15. wavelets
_S2 = math.sqrt(0.5)


def haar_dwt(x, levels=1):
    x = _vec(x, "signal x", 2, 65536)
    N = len(x)
    if N & (N - 1):
        raise ValueError("the length must be a power of two")
    levels = _int(levels, "number of levels", 1, N.bit_length() - 1)
    a, details = x, []
    for _ in range(levels):
        details.insert(0, [_S2 * (a[2 * i] - a[2 * i + 1]) for i in range(len(a) // 2)])
        a = [_S2 * (a[2 * i] + a[2 * i + 1]) for i in range(len(a) // 2)]
    return {"approximation": a, "details": details}


def ref_haar_dwt(x, levels=1):
    import pywt
    c = pywt.wavedec(x, "haar", mode="periodization", level=levels)
    return {"approximation": [float(v) for v in c[0]], "details": [[float(v) for v in d] for d in c[1:]]}


_p("haar_dwt", name="Haar discrete wavelet transform", topic="wavelets",
   purpose="The multi-level orthonormal Haar wavelet transform of a sequence whose length is a power of two: the final approximation and the detail coefficients of every level (coarsest first).",
   inputs=[_i("x", "samples (length a power of two)", type="vector", min_length=2, max_length=65536), _i("levels", "number of levels (default 1)", type="integer", min=1, required=False)],
   outputs=[_o("approximation", "approximation coefficients after the last level"), _o("details", "detail coefficients per level, coarsest first")],
   primary="approximation", conditions=["length a power of two", "levels <= log2 N"], method="repeated pairwise (a + b)/sqrt 2 and (a - b)/sqrt 2",
   latex=r"c_A[k] = \frac{x[2k] + x[2k+1]}{\sqrt 2},\quad c_D[k] = \frac{x[2k] - x[2k+1]}{\sqrt 2}", run=haar_dwt, reference=ref_haar_dwt,
   wrong=lambda x, levels=1: (lambda r, s: {"approximation": [v * s for v in r["approximation"]], "details": r["details"]})(haar_dwt(x, levels), _S2 ** levels),
   random_inputs=lambda rng: (lambda n: {"x": [round(rng.uniform(-10, 10), 2) for _ in range(n)], "levels": rng.randint(1, n.bit_length() - 1)})(rng.choice([2, 4, 8, 16, 32])),
   parse=lambda q: _parse(q, r"\bhaar\b", [("levels", "int", ["^-?\\s?levels?", r"levels?(?: of)?", r"\bL(?=\s*=)"]), ("x", "vec", [r"\bx(?:\[n\])?(?=\s*[=:])", "signal", "sequence", ""])],
                          exclude=r"inverse|reconstruct|\bidwt\b"),
   templates=["Compute the 2-level Haar wavelet transform of x = [4, 6, 10, 12, 8, 6, 5, 5]."],
   examples=[{"inputs": {"x": "4, 6, 10, 12, 8, 6, 5, 5", "levels": 2}, "outputs": {"approximation": [16.0, 12.0], "details": [[-6.0, 2.0], [-2 * _S2, -2 * _S2, 2 * _S2, 0.0]]}},
             {"inputs": {"x": [1, 3]}, "outputs": {"approximation": [4 * _S2], "details": [[-2 * _S2]]}}],
   questions=[{"q": "Compute the 2-level Haar wavelet transform of x = [4, 6, 10, 12, 8, 6, 5, 5].", "outputs": {"approximation": [16.0, 12.0]}},
              {"q": "Compute the Haar wavelet transform of x = [1, 2, 3].", "outcome": "INVALID_INPUT"}])


def haar_idwt(cA, cD):
    cA, cD = _vec(cA, "approximation coefficients", 1, 32768), _vec(cD, "detail coefficients", 1, 32768)
    if len(cA) != len(cD):
        raise ValueError("the approximation and detail lists must have the same length")
    x = []
    for a, d in zip(cA, cD):
        x += [_S2 * (a + d), _S2 * (a - d)]
    return {"x": x}


_p("haar_idwt", name="Inverse Haar wavelet transform (one level)", topic="wavelets",
   purpose="The sequence reconstructed from one level of Haar approximation and detail coefficients.",
   inputs=[_i("cA", "approximation coefficients", type="vector", min_length=1, max_length=32768), _i("cD", "detail coefficients", type="vector", min_length=1, max_length=32768)],
   outputs=[_o("x", "reconstructed samples")], primary="x", conditions=["equal lengths"], method="x[2k] = (a + d)/sqrt 2, x[2k+1] = (a - d)/sqrt 2",
   latex=r"x[2k] = \frac{c_A[k] + c_D[k]}{\sqrt 2},\quad x[2k+1] = \frac{c_A[k] - c_D[k]}{\sqrt 2}", run=haar_idwt,
   reference=lambda cA, cD: {"x": [float(v) for v in __import__("pywt").idwt(cA, cD, "haar", mode="periodization")]},
   wrong=lambda cA, cD: {"x": [v for a, d in zip(cA, cD) for v in (_S2 * (a - d), _S2 * (a + d))]},
   random_inputs=lambda rng: (lambda n: {"cA": [round(rng.uniform(-10, 10), 3) for _ in range(n)], "cD": [rng.choice([-1, 1]) * round(rng.uniform(0.5, 5), 3) for _ in range(n)]})(rng.randint(1, 16)),
   parse=lambda q: _parse(q, r"\bhaar\b", [("cA", "vec", [r"approximation(?: coefficients)?", r"\bc_?A(?=\s*[=:])"]), ("cD", "vec", [r"detail(?: coefficients)?", r"\bc_?D(?=\s*[=:])"])],
                          require=r"inverse|reconstruct|\bidwt\b"),
   templates=["Compute the inverse Haar transform with approximation coefficients [16, 12] and detail coefficients [-6, 2]."],
   examples=[{"inputs": {"cA": "16, 12", "cD": "-6, 2"}, "outputs": {"x": [10 * _S2, 22 * _S2, 14 * _S2, 10 * _S2]}}],
   questions=[{"q": "Compute the inverse Haar transform with approximation coefficients [16, 12] and detail coefficients [-6, 2].", "outputs": {"x": [10 * _S2, 22 * _S2, 14 * _S2, 10 * _S2]}},
              {"q": "Compute the inverse Haar transform with approximation coefficients [16, 12] and detail coefficients [-6].", "outcome": "INVALID_INPUT"}])
