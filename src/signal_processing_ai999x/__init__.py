"""signal-processing-ai999x — offline signal processing answers with their formula, steps and verification.

    import signal_processing_ai999x as m
    m.ask("What is the Nyquist rate for a signal with a maximum frequency of 4 kHz?").show()     # 8000 Hz
    m.solve("dft_spectrum", x="1, 2, 3, 4").show()                                              # 10, -2+2j, -2, -2-2j
    m.capabilities()                   # every certified capability (74)

Sampling and aliasing, decibels and dBm, signal statistics, the DFT and FFT (bins, spectra, inverse, resolution,
periodogram, Hilbert envelope), linear and circular convolution and correlation, FIR/IIR filter outputs, impulse and
step responses, frequency response, group delay and DC gain, Butterworth and Chebyshev orders and designs, FIR window
design, notch filters, z-domain poles and zeros, windows, Fourier series and transforms, THD, AM and FM, Shannon
capacity, BPSK/QPSK/DPSK/M-QAM/M-PSK error rates, noise-figure cascades, path loss and link budgets, dish gain, radar
range equation and Doppler, reflection coefficient, VSWR, input impedance, quarter-wave transformers, coaxial lines and
Haar wavelets. Built by the AI999X model machine from the same engine as every domain; a question that is not
determined is refused rather than guessed.

Creator / engineer: Ibnul Jaif Farabi. Company: AI999X.
"""
from __future__ import annotations

from pathlib import Path

try:                                       # the installed package carries its own copy of the engine
    from ._engine import Answer
    from ._engine.product import Product
except ImportError:                        # the factory's source tree, during development
    from ai999x_factory.runtime import Answer
    from ai999x_factory.runtime.product import Product

__version__ = "1.0.0"
_P = Product("signal_processing", [Path(__file__).resolve().parent / "pack", Path(__file__).resolve().parents[3]])
pack, ask, solve, compile_problem = _P.pack, _P.ask, _P.solve, _P.compile_problem
formula, formulas, quantities, conditions, capabilities = _P.formula, _P.formulas, _P.quantities, _P.conditions, _P.capabilities
examples, family_examples, structured_example, to_si = _P.examples, _P.family_examples, _P.structured_example, _P.to_si
templates, procedure_examples = _P.templates, _P.procedure_examples
prove, derivations, about = _P.prove, _P.derivations, (lambda: {**_P.about(), "version": __version__})
knowledge, lookup, related = _P.knowledge, _P.lookup, _P.related
plot, plot_relation, search = _P.plot, _P.plot_relation, _P.search   # figures (SVG) and capability search ([search])
__all__ = ["ask", "solve", "prove", "derivations", "about", "formula", "formulas", "quantities", "conditions", "capabilities",
           "compile_problem", "pack", "Answer", "plot", "plot_relation", "search", "__version__"]
