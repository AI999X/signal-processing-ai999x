//! Signal Processing's own live figures, part two: spectra, modulation, error rates, links, filter orders and value
//! fields over two inputs. Every value drawn is computed by the model for the visitor's inputs (further solves
//! through ax_engine::figkit::Solved).

use crate::figures::{clip, hz, idx, jl, lst, panels, Sp};
use crate::figures_kit::{self as g, Panel};
use ax_engine::figkit::{self as k, eq, num, q, Canvas, Solved};
use ax_engine::figures::Figure;
use ax_engine::Input;

pub fn live(s: &Solved) -> Vec<Option<Figure>> {
    match s.key {
        "alias_frequency" => vec![folding(s)],
        "am_modulation_index" => vec![am_wave(s)],
        "am_power" => vec![am_spectrum(s), am_efficiency(s)],
        "analytic_envelope" => vec![envelope(s)],
        "bin_frequency" => vec![bins(s)],
        "bpsk_ber" => vec![ber_pair(s, &[("ber", "coherent BPSK"), ("ber_bfsk", "coherent BFSK")], "ber",
                                    "Bit error rate: BPSK and coherent BFSK", "Bit error probability")],
        "qpsk_error_rates" => vec![ber_pair(s, &[("ber", "bit error rate"), ("ser", "symbol error rate")], "ber",
                                            "QPSK bit and symbol error rates", "Error probability")],
        "noncoherent_ber" => vec![noncoherent(s)],
        "mpsk_symbol_error" => vec![mpsk(s)],
        "mqam_symbol_error" => vec![ber_family(s, &["4", "16", "64", "256"], "M-QAM")],
        "butterworth_magnitude" => vec![butter_family(s)],
        "butterworth_order_analog" => vec![order_vs_stopband(s)],
        "butterworth_order_digital" => vec![order_digital(s)],
        "chebyshev1_order_analog" => vec![order_chebyshev(s)],
        "coax_impedance" => vec![coax(s)],
        "dbm_power_sum" => vec![dbm_bars(s)],
        "exponential_pulse_spectrum" => vec![exp_spectrum(s)],
        "fm_spectrum" => vec![fm(s)],
        "fourier_series_harmonic" => vec![harmonics(s)],
        "friis_received_power" => vec![budget(s)],
        "noise_figure_cascade" => vec![nf_bars(s)],
        "periodogram" => vec![periodogram(s)],
        "rect_pulse_spectrum" => vec![rect_spectrum(s)],
        "sine_rms" => vec![sine(s)],
        "thd" => vec![thd(s)],
        "free_space_path_loss" => vec![field2(s, "d", Rng::Decade, "f", Rng::Decade, "loss_db",
                                              "Free-space path loss over distance and frequency")],
        "frequency_resolution" => vec![field2(s, "N", Rng::Pow2, "fs", Rng::Near, "df",
                                              "Frequency resolution over the number of points and the sampling rate")],
        "radar_doppler" => vec![field2(s, "v", Rng::Near, "f", Rng::Near, "fd",
                                       "Doppler shift of a radar echo over speed and frequency")],
        "shannon_capacity" => vec![field2(s, "B", Rng::Near, "snr_db", Rng::Around(15.0), "C",
                                          "Shannon capacity over bandwidth and signal-to-noise ratio")],
        "snr_from_powers" => vec![field2(s, "Ps", Rng::Decade, "Pn", Rng::Decade, "snr_db",
                                         "Signal-to-noise ratio over signal and noise powers")],
        _ => vec![],
    }
}

// ------------------------------------------------------------------------------------------------ waves and spectra
fn folding(s: &Solved) -> Option<Figure> {
    let (f, fs, fa) = (s.inp("f")?, s.inp("fs")?, s.out("f_alias")?);
    if !(fs > 0.0) {
        return None;
    }
    let top = (2.5 * fs).max(f * 1.15);
    let (sc, u) = hz(top);
    let pts: Vec<(f64, f64)> = s.curve("f", &k::span(0.0, top, 201, false), "f_alias", &[]).into_iter()
        .map(|(x, y)| (x / sc, y / sc)).collect();
    if pts.len() < 2 {
        return None;
    }
    let ys: Vec<f64> = pts.iter().map(|p| p.1).chain([fs / 2.0 / sc]).collect();
    let (mut p, pre) = g::single((0.0, top / sc), g::range(&ys, true), (false, false),
                                 &format!("f = {} appears at {}", q(f, "Hz"), q(fa, "Hz")), &[]);
    let mut m = 1;
    while (m as f64) * fs / 2.0 <= top && m <= 12 {
        let name = match m {
            1 => "fs/2".to_string(),
            2 => "fs".to_string(),
            _ if m % 2 == 0 => format!("{}fs", m / 2),
            _ => format!("{m}fs/2"),
        };
        p.vline(m as f64 * fs / 2.0 / sc, &name, (m + 1) % 2);
        m += 1;
    }
    p.line(&pts, 0);
    p.mark(f / sc, fa / sc, &eq("f_alias", fa, "Hz"));
    p.labels(&format!("Input frequency f ({u})"), &format!("Apparent frequency ({u})"));
    let body = format!("{pre}{}", p.done());
    Some(s.chart("folding", "Aliasing: the folding of frequencies about fs/2",
                 &format!("The frequency a sinusoid appears at after sampling at fs = {}, for input frequencies 0 to {} \
                           (201 runs of the model): the apparent frequency rises to fs/2 = {}, then folds back down, a \
                           saw-tooth repeating every fs (dashed lines at multiples of fs/2). Marked: f = {} appears at {}.",
                          q(fs, "Hz"), q(top, "Hz"), q(fs / 2.0, "Hz"), q(f, "Hz"), q(fa, "Hz")), &body))
}

fn am_wave(s: &Solved) -> Option<Figure> {
    let (vmax, vmin) = (s.inp("Vmax")?, s.inp("Vmin")?);
    let (m, ac) = (s.out("m")?, s.out("Ac")?);
    let tau = 2.0 * std::f64::consts::PI;
    let ts = k::span(0.0, 2.0, 481, false);
    let wave: Vec<(f64, f64)> = ts.iter().map(|t| (*t, ac * (1.0 + m * (tau * t).cos()) * (tau * 8.0 * t).cos())).collect();
    let up: Vec<(f64, f64)> = ts.iter().map(|t| (*t, ac * (1.0 + m * (tau * t).cos()))).collect();
    let dn: Vec<(f64, f64)> = up.iter().map(|p| (p.0, -p.1)).collect();
    let lim = vmax.abs().max(ac * (1.0 + m.abs())) * 1.12;
    let items = vec![("AM wave".to_string(), 0), ("envelope".to_string(), 2)];
    let (mut p, pre) = g::single((0.0, 2.0), (-lim, lim), (false, false),
                                 &format!("m = {}, Ac = {}", num(m), q(ac, "V")), &items);
    p.line(&wave, 0);
    p.dash(&up, 2);
    p.dash(&dn, 2);
    p.hline(vmax, &format!("Vmax {}", q(vmax, "V")));
    p.hline_left(vmin, &format!("Vmin {}", q(vmin, "V")));
    p.labels("Time (modulation periods)", "Voltage (V)");
    let body = format!("{pre}{}", p.done());
    Some(s.chart("waveform", "An AM wave between its envelope limits",
                 &format!("An AM wave with envelope maximum Vmax = {} and minimum Vmin = {} (inputs): the model returns \
                           the modulation index m = {} and carrier amplitude Ac = {}. The wave Ac(1 + m cos 2πt)·cos 2π·8t \
                           is drawn from those returned values (a carrier of 8 cycles per modulation period, chosen for \
                           the drawing); its envelope (dashed) swings between Ac(1 - m) and Ac(1 + m).", q(vmax, "V"),
                          q(vmin, "V"), num(m), q(ac, "V")), &body))
}

fn am_efficiency(s: &Solved) -> Option<Figure> {
    let (m, e) = (s.inp("m")?, s.out("efficiency")?);
    let hi = m.max(1.0);
    let pts = s.curve("m", &k::span(0.0, hi, 51, false), "efficiency", &[]);
    if pts.len() < 2 {
        return None;
    }
    let body = k::lines_y(&[("efficiency".into(), pts)], "Modulation index m", "Sideband share of the power",
                          &format!("efficiency {} at m = {}", num(e), num(m)), Some((m, e)),
                          &format!("m = {}: {}", num(m), num(e)), false, true);
    Some(s.chart("efficiency", "AM efficiency against the modulation index",
                 &format!("The share of the transmitted AM power that is in the sidebands, for modulation index m from 0 \
                           to {} (51 runs of the model, Pc = {}). Marked: m = {}, efficiency {}.", num(hi),
                          q(s.inp("Pc")?, "W"), num(m), num(e)), &body))
}

fn am_spectrum(s: &Solved) -> Option<Figure> {
    let pc = s.inp("Pc")?;
    let (pt, psb, pu, e) = (s.out("Pt")?, s.out("Psb")?, s.out("Pusb")?, s.out("efficiency")?);
    let pts = vec![(-1.0, pu), (0.0, pc), (1.0, pu)];
    let (body, ht) = panels(&format!("total {}, sidebands {}", q(pt, "W"), q(psb, "W")), &[], &[
        Sp::new("", "Frequency offset from the carrier (multiples of fm)", "Power (W)").stem(pts, 0)
            .mark(0.0, pc, &format!("carrier {}", q(pc, "W"))).mark(1.0, pu, &format!("USB {}", q(pu, "W"))),
    ])?;
    Some(s.figure("spectrum", "AM line spectrum: carrier and sidebands",
                  &format!("The line spectrum of an AM signal with carrier power Pc = {} and modulation index m = {} \
                            (inputs): the carrier and the upper and lower sidebands at ±fm, each sideband carrying the \
                            returned Pusb = {}. Returned: total power Pt = {}, both sidebands Psb = {}, efficiency {}.",
                           q(pc, "W"), num(s.inp("m")?), q(pu, "W"), q(pt, "W"), q(psb, "W"), num(e)), &body, ht))
}

fn envelope(s: &Solved) -> Option<Figure> {
    let x = s.vec("x")?;
    let (env, hil) = (s.out_list("envelope")?, s.out_list("hilbert")?);
    if x.len() > 2048 {
        return None;
    }
    let neg = |v: &[f64]| -> Vec<(f64, f64)> { v.iter().enumerate().map(|(i, y)| (i as f64, -y)).collect() };
    let xt: Vec<f64> = (0..64).map(|n| {
        let n = n as f64;
        (1.0 + 0.6 * (2.0 * std::f64::consts::PI * n / 64.0).cos()) * (2.0 * std::f64::consts::PI * 8.0 * n / 64.0).cos()
    }).collect();
    let et = k::list_of(&s.with_input(&[("x", jl(&xt))])?, "envelope")?;
    let items = vec![("signal".to_string(), 3), ("Hilbert transform".to_string(), 2), ("envelope (±)".to_string(), 0)];
    let (body, ht) = panels(&clip(&format!("envelope = {}", lst(&env)), 52), &items, &[
        Sp::new("your signal", "Sample n", "Value").stem(idx(&x), 3).stem(idx(&hil), 2).line(idx(&env), 0)
            .line(neg(&env), 0),
        Sp::new("a chosen modulated tone and the envelope the model returns", "Sample n", "Value").line(idx(&xt), 3)
            .line(idx(&et), 0).line(neg(&et), 0).real(),
    ])?;
    Some(s.figure("envelope", "A signal and its analytic envelope",
                  &format!("Your signal x = {} with its Hilbert transform {} and the envelope |x + j·H{{x}}| = {} returned \
                            by the model (drawn at ± the envelope). Below, a tone chosen for the drawing, x[n] = (1 + 0.6 \
                            cos 2πn/64)·cos 2π·8n/64, n = 0 to 63: the envelope the model returns traces the slow \
                            modulation.", lst(&x), lst(&hil), lst(&env)), &body, ht))
}

fn bins(s: &Solved) -> Option<Figure> {
    let (fs, fk) = (s.inp("fs")?, s.out("f_k")?);
    let (kk, n) = (s.val("k")?.int(), s.val("N")?.int());
    if n < 2 {
        return None;
    }
    let half = n / 2;
    let step = ((half as f64) / 64.0).ceil().max(1.0) as i64;
    let mut pts = vec![];
    let mut i = 0;
    while i <= half {
        if let Some(a) = s.with_input(&[("k", Input::Int(i))]) {
            if let Some(v) = k::num_of(&a, "f_k") {
                pts.push((i as f64, v));
            }
        }
        i += step;
    }
    if pts.len() < 2 {
        return None;
    }
    let (sc, u) = hz(fs / 2.0);
    let pts: Vec<(f64, f64)> = pts.into_iter().map(|p| (p.0, p.1 / sc)).collect();
    let xmax = (half as f64).max(kk as f64);
    let ys: Vec<f64> = pts.iter().map(|p| p.1).chain([fk / sc, fs / 2.0 / sc]).collect();
    let (p0, pre) = g::single((0.0, xmax), g::range(&ys, true), (false, false),
                              &format!("bin {kk}: f = {}", q(fk, "Hz")), &[]);
    let (top, bot, h) = (p0.top, p0.bot, p0.h);
    let mut p = Panel::ints((0.0, xmax), g::range(&ys, true), false, top, bot, h);
    p.line(&pts, 0);
    p.dots(&pts, 0, 2.2);
    p.hline(fs / 2.0 / sc, &format!("fs/2 = {}", q(fs / 2.0, "Hz")));
    p.mark(kk as f64, fk / sc, &format!("k = {kk}"));
    p.labels("Bin index k", &format!("Bin frequency ({u})"));
    let body = format!("{pre}{}", p.done());
    Some(s.chart("bins", "DFT bin frequencies up to fs/2",
                 &format!("The centre frequency of DFT bin k for N = {n} points sampled at fs = {}, k = 0 to {half} (one \
                           run of the model per plotted bin): a straight line of slope fs/N reaching fs/2 at k = N/2 \
                           (dashed). Marked: k = {kk}, f_k = {}.", q(fs, "Hz"), q(fk, "Hz")), &body))
}

fn periodogram(s: &Solved) -> Option<Figure> {
    let (x, fs) = (s.vec("x")?, s.inp("fs")?);
    let (fr, psd, pf) = (s.out_list("freqs")?, s.out_list("psd")?, s.out("peak_frequency")?);
    if fr.len() != psd.len() || x.len() > 2048 || !(fs > 0.0) {
        return None;
    }
    let ppk = fr.iter().zip(&psd).find(|(f, _)| (**f - pf).abs() <= 1e-9 * pf.abs().max(1.0)).map(|x| *x.1)?;
    let t: Vec<(f64, f64)> = x.iter().enumerate().map(|(i, v)| (i as f64 / fs, *v)).collect();
    let sp: Vec<(f64, f64)> = fr.iter().copied().zip(psd.iter().copied()).collect();
    let (body, ht) = panels(&format!("peak at {}", q(pf, "Hz")), &[], &[
        Sp::new("samples x[n]", "Time (s)", "x").stem(t, 3).real(),
        Sp::new("periodogram returned", "Frequency (Hz)", "PSD").stem(sp, 0).mark(pf, ppk, &q(pf, "Hz")).real(),
    ])?;
    Some(s.figure("spectrum", "Periodogram of a sampled signal",
                  &format!("The {} samples x = {} at fs = {} (top) and the periodogram the model returns for them \
                            (bottom): {} frequencies from 0 to fs/2. Marked: the returned peak frequency {}.", x.len(),
                           lst(&x), q(fs, "Hz"), fr.len(), q(pf, "Hz")), &body, ht))
}

fn exp_spectrum(s: &Solved) -> Option<Figure> {
    let (a0, tau, f) = (s.inp("A")?, s.inp("tau")?, s.inp("f")?);
    let (m, ph) = (s.out("magnitude")?, s.out("phase")?);
    if !(f > 0.0) {
        return None;
    }
    let fr = k::span(f / 100.0, f * 100.0, 61, true);
    let (mut mp, mut pp) = (vec![], vec![]);
    for x in &fr {
        if let Some(r) = s.with(&[("f", *x)]) {
            if let (Some(mm), Some(p)) = (k::num_of(&r, "magnitude"), k::num_of(&r, "phase")) {
                mp.push((*x, mm));
                pp.push((*x, p));
            }
        }
    }
    if mp.len() < 2 {
        return None;
    }
    let top = k::lines(&[("|X(f)|".into(), mp)], "", "Magnitude |X(f)|",
                       &format!("|X| = {} at f = {}", num(m), q(f, "Hz")), Some((f, m)), &num(m), true);
    let bottom = k::lines(&[("phase".into(), pp)], "Frequency (Hz)", "Phase (rad)", "", Some((f, ph)),
                          &format!("{} rad", num(ph)), true);
    let (body, h) = k::stack(&[top, bottom]);
    Some(s.figure("spectrum", "Spectrum of a decaying exponential pulse",
                  &format!("Magnitude and phase of the spectrum of A·e^(-t/τ) for t ≥ 0 with A = {}, τ = {}, from f/100 to \
                            100 f (61 runs of the model, logarithmic axes): flat at low frequency, falling 20 dB per \
                            decade at high frequency, the phase going from 0 toward -π/2. Marked: f = {}, |X| = {}, phase \
                            {} rad.", num(a0), q(tau, "s"), q(f, "Hz"), num(m), num(ph)), &body, h))
}

fn rect_spectrum(s: &Solved) -> Option<Figure> {
    let (a0, tau, f) = (s.inp("A")?, s.inp("tau")?, s.inp("f")?);
    let (x, null) = (s.out("X")?, s.out("first_null")?);
    if !(null > 0.0) {
        return None;
    }
    let top = (5.0 * null).max(f.abs() * 1.1);
    let (sc, u) = hz(top);
    let pts: Vec<(f64, f64)> = s.curve("f", &k::span(0.0, top, 201, false), "X", &[]).into_iter()
        .map(|p| (p.0 / sc, p.1)).collect();
    if pts.len() < 2 {
        return None;
    }
    let ys: Vec<f64> = pts.iter().map(|p| p.1).chain([x]).collect();
    let (mut p, pre) = g::single((0.0, top / sc), g::range(&ys, true), (false, false),
                                 &format!("first null at {}", q(null, "Hz")), &[]);
    let mut j = 1;
    while j as f64 * null <= top && j <= 10 {
        p.vline(j as f64 * null / sc, if j == 1 { "first null" } else { "" }, 0);
        j += 1;
    }
    p.line(&pts, 0);
    p.mark(f / sc, x, &format!("X = {}", num(x)));
    p.labels(&format!("Frequency ({u})"), "X(f)");
    let body = format!("{pre}{}", p.done());
    Some(s.chart("spectrum", "Spectrum of a rectangular pulse",
                 &format!("The Fourier transform of a rectangular pulse of amplitude A = {} and width τ = {}, X(f) = \
                           Aτ·sinc(fτ) (even in f), from 0 to {} (201 runs of the model): a main lobe and side lobes of \
                           alternating sign, nulls at multiples of the returned first null {} (dashed). Marked: f = {}, X \
                           = {}.", num(a0), q(tau, "s"), q(top, "Hz"), q(null, "Hz"), q(f, "Hz"), num(x)), &body))
}

fn fm(s: &Solved) -> Option<Figure> {
    let (df, fmod) = (s.inp("df")?, s.inp("fm")?);
    let (beta, bw, j0) = (s.out("beta")?, s.out("carson_bandwidth")?, s.out("carrier")?);
    if !(fmod > 0.0) {
        return None;
    }
    let nmax = ((beta.ceil() as i64) + 4).clamp(6, 30);
    let mut pts = vec![(0.0, j0.abs())];
    let mut lines = vec![format!("J0 = {}", num(j0))];
    for n in 1..=nmax {
        let Some(jn) = s.with_input(&[("n", Input::Int(n))]).and_then(|r| k::num_of(&r, "Jn")) else { continue };
        pts.push((n as f64 * fmod, jn.abs()));
        pts.push((-(n as f64) * fmod, jn.abs()));
        if n <= 4 {
            lines.push(format!("J{n} = {}", num(jn)));
        }
    }
    let (sc, u) = hz(nmax as f64 * fmod);
    let pts: Vec<(f64, f64)> = pts.into_iter().map(|p| (p.0 / sc, p.1)).collect();
    let xr = (-(nmax as f64 + 0.8) * fmod / sc, (nmax as f64 + 0.8) * fmod / sc);
    let ys: Vec<f64> = pts.iter().map(|p| p.1).collect();
    let (mut p, pre) = g::single(xr, g::range(&ys, true), (false, false),
                                 &format!("β = {}, Carson bandwidth {}", num(beta), q(bw, "Hz")), &[]);
    p.stems(&pts, 0, 0.0);
    p.vline(-bw / 2.0 / sc, "", 0);
    p.vline(bw / 2.0 / sc, "Carson", 0);
    p.mark(0.0, j0.abs(), &format!("carrier |J0| = {}", num(j0.abs())));
    p.labels(&format!("Offset from the carrier ({u})"), "Line amplitude |Jn(β)|");
    let body = format!("{pre}{}", p.done());
    Some(s.chart("spectrum", "FM line spectrum and Carson bandwidth",
                 &format!("The line spectrum of FM with peak deviation Δf = {} and modulating frequency fm = {}: \
                           modulation index β = {} and line amplitudes |Jn(β)| at ±n·fm for n = 0 to {nmax} (n = 0 is the \
                           carrier), each a run of the model ({}). Dashed: the returned Carson bandwidth {} centred on the \
                           carrier.", q(df, "Hz"), q(fmod, "Hz"), num(beta), lines.join(", "), q(bw, "Hz")), &body))
}

fn harmonics(s: &Solved) -> Option<Figure> {
    let (a0, wf, n0) = (s.inp("A")?, s.txt("waveform")?, s.val("n")?.int());
    let amp = s.out("amplitude")?;
    let nn = 11.max(n0.min(25));
    let labels: Vec<String> = (1..=nn).map(|n| n.to_string()).collect();
    let mut groups = vec![];
    let mut fund = vec![];
    for w in ["square", "triangle", "sawtooth"] {
        let mut v = vec![];
        for n in 1..=nn {
            let a = if w == wf && n == n0 { Some(amp) } else {
                s.with_input(&[("waveform", Input::Text(w.into())), ("n", Input::Int(n))]).and_then(|r| k::num_of(&r, "amplitude"))
            };
            v.push(a.map(|x| x.abs()));
        }
        fund.push(format!("{w}: fundamental {}", v[0].map(num).unwrap_or_default()));
        groups.push((if w == wf { format!("{w} (yours)") } else { w.to_string() }, v));
    }
    let body = k::bar_groups(&labels, &groups, "Harmonic number n", "Amplitude",
                             &format!("{wf}, harmonic {n0}: {}", num(amp)));
    Some(s.chart("harmonics", "Harmonic content of square, triangle and sawtooth waves",
                 &format!("Fourier-series harmonic amplitudes of three waves of peak amplitude A = {} (harmonics 1 to {nn}, \
                           each a run of the model, side by side): the square wave has odd harmonics falling as 1/n, the \
                           triangle odd harmonics as 1/n², the sawtooth every harmonic as 1/n. {}. Your harmonic: {wf} n = \
                           {n0}, amplitude {}.", num(a0), fund.join("; "), num(amp)), &body))
}

fn sine(s: &Solved) -> Option<Figure> {
    let vp = s.inp("Vp")?;
    let (vrms, vpp, vavg) = (s.out("Vrms")?, s.out("Vpp")?, s.out("Vavg")?);
    let tau = 2.0 * std::f64::consts::PI;
    let ts = k::span(0.0, 2.0, 241, false);
    let wave: Vec<(f64, f64)> = ts.iter().map(|t| (*t, vp * (tau * t).sin())).collect();
    let rect: Vec<(f64, f64)> = wave.iter().map(|p| (p.0, p.1.abs())).collect();
    let lim = vp.abs() * 1.15;
    let items = vec![("sine".to_string(), 0), ("rectified".to_string(), 2)];
    let (mut p, pre) = g::single((0.0, 2.0), (-lim, lim), (false, false),
                                 &format!("Vrms = {}, Vavg = {}", q(vrms, "V"), q(vavg, "V")), &items);
    p.line(&wave, 0);
    p.dash(&rect, 2);
    p.hline(vrms, &format!("Vrms {}", q(vrms, "V")));
    p.hline_left(vavg, &format!("Vavg {}", q(vavg, "V")));
    p.hline(-vp.abs(), &format!("-Vp, Vpp = {}", q(vpp, "V")));
    p.labels("Time (periods)", "Voltage (V)");
    let body = format!("{pre}{}", p.done());
    Some(s.chart("waveform", "A sine wave: peak, RMS and average",
                 &format!("A sinusoid of peak Vp = {} (input) over two periods, with its rectified wave (dashed); returned \
                           by the model: Vrms = {}, average of the rectified wave Vavg = {}, peak-to-peak Vpp = {}.",
                          q(vp, "V"), q(vrms, "V"), q(vavg, "V"), q(vpp, "V")), &body))
}

fn thd(s: &Solved) -> Option<Figure> {
    let v = s.vec("V")?;
    let (t, pc, db) = (s.out("thd")?, s.out("percent")?, s.out("db")?);
    if v.len() > 40 {
        return None;
    }
    let labels: Vec<String> = (1..=v.len()).map(|i| format!("V{i}")).collect();
    let body = k::bars(&labels, &v, &[0], "Harmonic", "Amplitude", &format!("THD = {} % ({} dB)", num(pc), num(db)));
    Some(s.chart("harmonics", &format!("Harmonic distortion: THD = {} %", num(pc)),
                 &format!("Harmonic amplitudes V1 to V{} = {} (input; the fundamental first, highlighted). The model \
                           returns THD = √(V2² + ...)/V1 = {} ({} %, {} dB).", v.len(), lst(&v), num(t), num(pc), num(db)),
                 &body))
}

// ------------------------------------------------------------------------------------------------ error rates
/// Eb/N0 values for an error-rate sweep: 0 to 14 dB, widened to hold the visitor's value.
fn ebn0_span(e: f64, step: f64) -> Vec<f64> {
    let lo = (e - 2.0).min(0.0).floor();
    let hi = (e + 2.0).max(14.0).ceil();
    let n = ((hi - lo) / step).round() as usize + 1;
    k::span(lo, hi, n.min(121), false)
}

/// Error-probability points kept on the plot (values below 1e-8 left off the axis).
fn keep(v: f64) -> f64 {
    if v >= 1e-8 { v } else { f64::NAN }
}

fn ber_pair(s: &Solved, outs: &[(&str, &str)], mark: &str, title: &str, ylab: &str) -> Option<Figure> {
    let e = s.inp("ebn0_db")?;
    let mv = s.out(mark)?;
    let es = ebn0_span(e, 0.25);
    let mut series: Vec<(String, Vec<(f64, f64)>)> = outs.iter().map(|o| (o.1.to_string(), vec![])).collect();
    for x in &es {
        if let Some(a) = s.with(&[("ebn0_db", *x)]) {
            for (i, (sym, _)) in outs.iter().enumerate() {
                if let Some(v) = k::num_of(&a, sym) {
                    series[i].1.push((*x, keep(v)));
                }
            }
        }
    }
    series.retain(|x| x.1.iter().filter(|p| p.1.is_finite()).count() >= 2);
    if series.is_empty() {
        return None;
    }
    let body = k::lines(&series, "Eb/N0 (dB)", ylab, &format!("{mark} = {} at {} dB", num(mv), num(e)),
                        (mv >= 1e-8).then_some((e, mv)), &format!("{mark} {}", num(mv)), false);
    let vals: Vec<String> = outs.iter().filter_map(|o| s.out(o.0).map(|v| format!("{} = {}", o.0, num(v)))).collect();
    Some(s.chart("ber", title,
                 &format!("{} against Eb/N0 from {} to {} dB, one run of the model per 0.25 dB, logarithmic axis (values \
                           below 1e-8 are off the axis): {}. Marked: Eb/N0 = {} dB, {}.", ylab, num(es[0]),
                          num(es[es.len() - 1]), outs.iter().map(|o| o.1).collect::<Vec<_>>().join(", "), num(e),
                          vals.join(", ")), &body))
}

fn noncoherent(s: &Solved) -> Option<Figure> {
    let e = s.inp("ebn0_db")?;
    let (bd, bf) = (s.out("ber_dpsk")?, s.out("ber_ncfsk")?);
    let es = ebn0_span(e, 0.25);
    let (mut d, mut f, mut c) = (vec![], vec![], vec![]);
    for x in &es {
        if let Some(a) = s.with(&[("ebn0_db", *x)]) {
            if let (Some(v1), Some(v2)) = (k::num_of(&a, "ber_dpsk"), k::num_of(&a, "ber_ncfsk")) {
                d.push((*x, keep(v1)));
                f.push((*x, keep(v2)));
            }
        }
        if let Some(v) = s.solve_other("bpsk_ber", &[("ebn0_db", Input::Float(*x))]).and_then(|a| k::num_of(&a, "ber")) {
            c.push((*x, keep(v)));
        }
    }
    let mut series = vec![("DPSK".to_string(), d), ("non-coherent FSK".to_string(), f), ("coherent BPSK".to_string(), c)];
    series.retain(|x| x.1.iter().filter(|p| p.1.is_finite()).count() >= 2);
    if series.len() < 2 {
        return None;
    }
    let body = k::lines(&series, "Eb/N0 (dB)", "Bit error probability", &format!("DPSK {} at {} dB", num(bd), num(e)),
                        (bd >= 1e-8).then_some((e, bd)), &format!("DPSK {}", num(bd)), false);
    Some(s.chart("ber", "Non-coherent detection: DPSK and FSK against BPSK",
                 &format!("Bit error probability of DPSK and non-coherent binary FSK, with coherent BPSK (the model's BPSK \
                           capability) for comparison. Eb/N0 from {} to {} dB, one run of the model per 0.25 dB, \
                           logarithmic axis (values below 1e-8 are off the axis). Marked: Eb/N0 = {} dB, DPSK {}, \
                           non-coherent FSK {}.", num(es[0]), num(es[es.len() - 1]), num(e), num(bd), num(bf)), &body))
}

/// M-PSK: one solve of this capability takes about a third of a second in a browser, so a curve family of it would
/// keep the page waiting; the visitor's point is drawn against the BPSK (M = 2) and QPSK (M = 4) symbol error curves
/// from the model's own BPSK and QPSK capabilities, which are fast.
fn mpsk(s: &Solved) -> Option<Figure> {
    let e = s.inp("ebn0_db")?;
    let m0 = s.txt("M")?;
    let ser = s.out("ser")?;
    let es = ebn0_span(e, 0.25);
    let (mut b2, mut q4) = (vec![], vec![]);
    for x in &es {
        if let Some(v) = s.solve_other("bpsk_ber", &[("ebn0_db", Input::Float(*x))]).and_then(|a| k::num_of(&a, "ber")) {
            b2.push((*x, keep(v)));
        }
        if let Some(v) = s.solve_other("qpsk_error_rates", &[("ebn0_db", Input::Float(*x))])
            .and_then(|a| k::num_of(&a, "ser")) {
            q4.push((*x, keep(v)));
        }
    }
    let mut series = vec![("M = 2 (BPSK)".to_string(), b2), ("M = 4 (QPSK)".to_string(), q4)];
    series.retain(|x| x.1.iter().filter(|p| p.1.is_finite()).count() >= 2);
    if series.is_empty() {
        return None;
    }
    let body = k::lines(&series, "Eb/N0 (dB)", "Symbol error probability", "", (ser >= 1e-8).then_some((e, ser)),
                        &format!("M = {m0}: ser {}", num(ser)), false);
    Some(s.chart("ber", "M-PSK symbol error rate against the BPSK and QPSK curves",
                 &format!("Symbol error probability of M-PSK: your point, M = {m0} at Eb/N0 = {} dB, ser = {} (returned by \
                           the model), against the symbol error curves of M = 2 (BPSK) and M = 4 (QPSK) from the model's \
                           BPSK and QPSK capabilities, one run per 0.25 dB from {} to {} dB, logarithmic axis (values \
                           below 1e-8 are off the axis). A larger M needs more Eb/N0 for the same error rate.", num(e),
                          num(ser), num(es[0]), num(es[es.len() - 1])), &body))
}

fn ber_family(s: &Solved, ms: &[&str], name: &str) -> Option<Figure> {
    let e = s.inp("ebn0_db")?;
    let m0 = s.txt("M")?;
    let ser = s.out("ser")?;
    let es = ebn0_span(e, 0.5);
    let mut list: Vec<String> = ms.iter().map(|x| x.to_string()).collect();
    if !list.contains(&m0) {
        list.push(m0.clone());
    }
    let mut series = vec![];
    for m in &list {
        let mut pts = vec![];
        for x in &es {
            if let Some(v) = s.with_input(&[("M", Input::Text(m.clone())), ("ebn0_db", Input::Float(*x))])
                .and_then(|a| k::num_of(&a, "ser")) {
                pts.push((*x, keep(v)));
            }
        }
        if pts.iter().filter(|p| p.1.is_finite()).count() >= 2 {
            series.push((format!("M = {m}"), pts));
        }
    }
    if series.is_empty() {
        return None;
    }
    let body = k::lines(&series, "Eb/N0 (dB)", "Symbol error probability",
                        "", (ser >= 1e-8).then_some((e, ser)),
                        &format!("M = {m0}: ser {}", num(ser)), false);
    Some(s.chart("ber", &format!("{name} symbol error rate for {} constellation sizes", series.len()),
                 &format!("Symbol error probability of {name} for M = {}: each step up in M costs more Eb/N0. Eb/N0 from \
                           {} to {} dB, one run of the model per 0.5 dB, logarithmic axis (values below 1e-8 are off the \
                           axis). Marked: M = {m0}, Eb/N0 = {} dB, ser = {}.", list.join(", "), num(es[0]),
                          num(es[es.len() - 1]), num(e), num(ser)), &body))
}

// ------------------------------------------------------------------------------------------------ filter orders
fn butter_family(s: &Solved) -> Option<Figure> {
    let (fc, f) = (s.inp("fc")?, s.inp("f")?);
    let n0 = s.val("N")?.int();
    let att = s.out("attenuation_db")?;
    if !(fc > 0.0) {
        return None;
    }
    let mut ns: Vec<i64> = vec![1, 2, 4, 8];
    if !ns.contains(&n0) {
        ns.push(n0);
        ns.sort();
    }
    let xs = k::span(fc / 10.0, fc * 10.0, 41, true);
    let mut series = vec![];
    for n in &ns {
        let mut pts = vec![];
        for x in &xs {
            if let Some(v) = s.with_input(&[("N", Input::Int(*n)), ("f", Input::Float(*x))])
                .and_then(|a| k::num_of(&a, "attenuation_db")) {
                pts.push((*x, -v));
            }
        }
        if pts.len() >= 2 {
            series.push((format!("N = {n}{}", if *n == n0 { " (yours)" } else { "" }), pts));
        }
    }
    if series.is_empty() {
        return None;
    }
    let body = k::lines_y(&series, "Frequency (Hz)", "Gain (dB)", &format!("N = {n0}: {} dB at {}", num(-att), q(f, "Hz")),
                          Some((f, -att)), &format!("{} dB", num(-att)), true, true);
    Some(s.chart("response", "Butterworth low-pass responses of several orders",
                 &format!("Gain of Butterworth low-pass filters of order N = {} with cutoff fc = {} (each curve 41 \
                           frequencies computed by the model, 0.1 fc to 10 fc on a logarithmic axis; the gain is minus the \
                           returned attenuation); every order is 3 dB down at fc and falls by 20N dB per decade. Marked: f \
                           = {}, N = {n0}, gain = {} dB.", ns.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(", "),
                          q(fc, "Hz"), q(f, "Hz"), num(-att)), &body))
}

/// Order curves drawn as steps in one chart: (name, colour, points); `vl`: a dashed guide and its name.
#[allow(clippy::too_many_arguments)]
fn order_chart(s: &Solved, kind: &str, title: &str, caption: &str, series: &[(String, Vec<(f64, f64)>)], xlab: &str,
               mark: (f64, f64), head: &str, vl: Option<(f64, String)>, sc: f64) -> Option<Figure> {
    let series: Vec<(String, Vec<(f64, f64)>)> = series.iter().filter(|x| x.1.len() >= 2)
        .map(|(n, p)| (n.clone(), p.iter().map(|q| (q.0 / sc, q.1)).collect())).collect();
    if series.is_empty() {
        return None;
    }
    let xs: Vec<f64> = series.iter().flat_map(|x| x.1.iter().map(|p| p.0)).chain([mark.0 / sc]).collect();
    let ys: Vec<f64> = series.iter().flat_map(|x| x.1.iter().map(|p| p.1)).chain([mark.1, 0.0]).collect();
    let (x0, x1) = xs.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(a, b), x| (a.min(*x), b.max(*x)));
    let items: Vec<(String, usize)> = series.iter().enumerate().map(|(i, x)| (x.0.clone(), i)).collect();
    let (mut p, pre) = g::single((x0, x1), g::range(&ys, true), (false, false), head, &items);
    for (i, (_, pts)) in series.iter().enumerate() {
        p.steps(pts, i);
    }
    if let Some((x, t)) = vl {
        p.vline(x / sc, &t, 0);
    }
    p.mark(mark.0 / sc, mark.1, &format!("order {}", num(mark.1)));
    p.labels(xlab, "Minimum order");
    let body = format!("{pre}{}", p.done());
    Some(s.chart(kind, title, caption, &body))
}

fn order_vs_stopband(s: &Solved) -> Option<Figure> {
    let (fp, fst, rp, a_s) = (s.inp("fp")?, s.inp("fstop")?, s.inp("Rp")?, s.inp("As")?);
    let n0 = s.out("order")?;
    let ty = s.out_text("type")?;
    let xs = if fst > fp { k::span((fp * 1.1).min(fst), (fp * 5.0).max(fst * 1.1), 60, false) } else {
        k::span((fp / 5.0).min(fst / 1.1), (fp / 1.1).max(fst), 60, false)
    };
    let mut series = vec![];
    for a in [a_s - 20.0, a_s, a_s + 20.0] {
        if a <= rp + 0.5 {
            continue;
        }
        let pts = s.curve("fstop", &xs, "order", &[("As", a)]);
        series.push((format!("As = {} dB{}", num(a), if a == a_s { " (yours)" } else { "" }), pts));
    }
    let (sc, u) = hz(xs[xs.len() - 1]);
    order_chart(s, "order", "Butterworth order for a transition band",
                &format!("The smallest Butterworth order ({ty}) meeting a passband edge fp = {} with at most Rp = {} dB \
                          ripple and a stopband attenuation As at the stopband edge fstop, for fstop from {} to {} (60 runs \
                          of the model per curve, drawn as steps): a sharper transition or more attenuation needs a higher \
                          order. Marked: fstop = {}, As = {} dB, order {}.", q(fp, "Hz"), num(rp), q(xs[0], "Hz"),
                         q(xs[xs.len() - 1], "Hz"), q(fst, "Hz"), num(a_s), num(n0)),
                &series, &format!("Stopband edge fstop ({u})"), (fst, n0),
                &format!("{ty}, fp = {}: order {}", q(fp, "Hz"), num(n0)), None, sc)
}

fn order_digital(s: &Solved) -> Option<Figure> {
    let (fp, fst, fs, rp, a_s) = (s.inp("fp")?, s.inp("fstop")?, s.inp("fs")?, s.inp("Rp")?, s.inp("As")?);
    let n0 = s.out("order")?;
    let xs = if fst > fp { k::span((fp * 1.1).min(fst), fs / 2.0 * 0.985, 60, false) } else {
        k::span((fp / 5.0).min(fst), (fp / 1.1).max(fst), 60, false)
    };
    if xs[0] >= xs[xs.len() - 1] {
        return None;
    }
    let dig = s.curve("fstop", &xs, "order", &[]);
    let mut ana = vec![];
    for x in &xs {
        if let Some(v) = s.solve_other("butterworth_order_analog", &[("fp", Input::Float(fp)), ("fstop", Input::Float(*x)),
                                                                     ("Rp", Input::Float(rp)), ("As", Input::Float(a_s))])
            .and_then(|a| k::num_of(&a, "order")) {
            ana.push((*x, v));
        }
    }
    let (sc, u) = hz(fs / 2.0);
    order_chart(s, "order", "Digital against analog Butterworth order",
                &format!("The smallest Butterworth order for fp = {}, Rp = {} dB, As = {} dB as the stopband edge moves \
                          from {} to {}: the digital design (bilinear transform at fs = {}) against the analog one, 60 \
                          runs of the model each; frequency warping near fs/2 lets the digital filter use a lower order. \
                          Marked: fstop = {}, order {}.", q(fp, "Hz"), num(rp), num(a_s), q(xs[0], "Hz"),
                         q(xs[xs.len() - 1], "Hz"), q(fs, "Hz"), q(fst, "Hz"), num(n0)),
                &[("digital (yours)".to_string(), dig), ("analog".to_string(), ana)],
                &format!("Stopband edge fstop ({u})"), (fst, n0), &format!("fs = {}: order {}", q(fs, "Hz"), num(n0)),
                (fst > fp).then(|| (fs / 2.0, "fs/2".to_string())), sc)
}

fn order_chebyshev(s: &Solved) -> Option<Figure> {
    let (fp, fst, rp, a_s) = (s.inp("fp")?, s.inp("fstop")?, s.inp("Rp")?, s.inp("As")?);
    let n0 = s.out("order")?;
    let lo = (rp + 1.0).max(10.0f64.min(a_s));
    let hi = 90.0f64.max(a_s + 5.0);
    let xs = k::span(lo, hi, 41, false);
    let ch = s.curve("As", &xs, "order", &[]);
    let mut bw = vec![];
    for x in &xs {
        if let Some(v) = s.solve_other("butterworth_order_analog", &[("fp", Input::Float(fp)), ("fstop", Input::Float(fst)),
                                                                     ("Rp", Input::Float(rp)), ("As", Input::Float(*x))])
            .and_then(|a| k::num_of(&a, "order")) {
            bw.push((*x, v));
        }
    }
    order_chart(s, "order", "Chebyshev against Butterworth: order needed",
                &format!("The smallest analog filter order for passband edge fp = {} (ripple Rp = {} dB) and stopband edge \
                          fstop = {}, as the stopband attenuation As grows from {} to {} dB: Chebyshev type I against \
                          Butterworth (the model's Butterworth capability), 41 runs of the model each, drawn as steps; the \
                          equiripple passband needs a lower order. Marked: As = {} dB, Chebyshev order {}.", q(fp, "Hz"),
                         num(rp), q(fst, "Hz"), num(lo), num(hi), num(a_s), num(n0)),
                &[("Chebyshev type I (yours)".to_string(), ch), ("Butterworth".to_string(), bw)],
                "Stopband attenuation As (dB)", (a_s, n0), &format!("As = {} dB: order {}", num(a_s), num(n0)), None, 1.0)
}

// ------------------------------------------------------------------------------------------------ links, power, lines
fn coax(s: &Solved) -> Option<Figure> {
    let (dd, d) = (s.inp("D")?, s.inp("d")?);
    let er = s.inp("er").unwrap_or(1.0);
    let (z0, vf) = (s.out("Z0")?, s.out("vf")?);
    if !(dd > d && d > 0.0) {
        return None;
    }
    let mut c = Canvas::new(340.0);
    let (cx, cy, r_out) = (170.0, 180.0, 120.0);
    let r_in = (r_out * d / dd).max(2.0);
    c.raw(&format!("<circle class=\"axp-k\" style=\"stroke-width:10\" cx=\"{cx}\" cy=\"{cy}\" r=\"{:.1}\"/>\
                    <circle class=\"axp-ka\" cx=\"{cx}\" cy=\"{cy}\" r=\"{r_out:.1}\"/>\
                    <circle class=\"axp-kd\" cx=\"{cx}\" cy=\"{cy}\" r=\"{r_in:.1}\"/>", r_out + 5.0));
    // the two diameters, measured
    c.arrow((cx, cy + r_out * 0.55), (cx - r_out, cy + r_out * 0.55));
    c.arrow((cx, cy + r_out * 0.55), (cx + r_out, cy + r_out * 0.55));
    c.value(cx, cy + r_out * 0.55 - 6.0, "middle", &format!("D = {}", q(dd, "m")));
    c.arrow((cx, cy - r_in - 18.0), (cx - r_in, cy - r_in - 18.0));
    c.arrow((cx, cy - r_in - 18.0), (cx + r_in, cy - r_in - 18.0));
    c.value(cx, cy - r_in - 26.0, "middle", &format!("d = {}", q(d, "m")));
    c.name(cx, cy + r_out * 0.82, "middle", &format!("dielectric εr = {}", num(er)));
    c.bold(16.0, 20.0, "start", "Coaxial line, to scale");
    c.bold(330.0, 120.0, "start", "Returned");
    c.value(330.0, 145.0, "start", &format!("Z0 = {}", q(z0, "ohm")));
    c.value(330.0, 168.0, "start", &format!("velocity factor {}", num(vf)));
    c.name(330.0, 205.0, "start", &format!("D/d = {}", num(dd / d)));
    c.name(330.0, 228.0, "start", "outer conductor (ring)");
    c.name(330.0, 251.0, "start", "inner conductor (disc)");
    Some(s.figure("section", "Coaxial line cross-section",
                  &format!("Cross-section of a coaxial line to scale: inner conductor d = {}, outer conductor inner diameter \
                            D = {}, dielectric εr = {} (inputs). Returned by the model: characteristic impedance Z0 = {} and \
                            velocity factor {}.", q(d, "m"), q(dd, "m"), num(er), q(z0, "ohm"), num(vf)), c.body(), 340.0))
}

fn dbm_bars(s: &Solved) -> Option<Figure> {
    let lv = s.vec("levels")?;
    let (tw, td) = (s.out("total_W")?, s.out("total_dBm")?);
    if lv.is_empty() || lv.len() > 16 {
        return None;
    }
    let mut w = vec![];
    for l in &lv {
        w.push(s.solve_other("dbm_to_watts", &[("dBm", Input::Float(*l))]).and_then(|a| k::num_of(&a, "P"))?);
    }
    let mut labels: Vec<String> = lv.iter().map(|l| format!("{} dBm", num(*l))).collect();
    labels.push("total".into());
    let mut vals = w.clone();
    vals.push(tw);
    let body = k::bars(&labels, &vals, &[vals.len() - 1], "Source", "Power (W)",
                       &format!("total {} = {} dBm", q(tw, "W"), num(td)));
    Some(s.chart("bars", "Summing powers given in dBm",
                 &format!("Adding incoherent powers given in dBm: the {} sources of {} dBm, shown in watts (from the \
                           model's dBm-to-watts capability: {}). The model returns their total {} = {} dBm: the levels add \
                           in watts, not in dB.", lv.len(), lv.iter().map(|x| num(*x)).collect::<Vec<_>>().join(", "),
                          w.iter().map(|x| q(*x, "W")).collect::<Vec<_>>().join(", "), q(tw, "W"), num(td)), &body))
}

fn nf_bars(s: &Solved) -> Option<Figure> {
    let (nf, gn) = (s.vec("nf_db")?, s.vec("gain_db")?);
    let total = s.out("nf_db")?;
    if nf.is_empty() || nf.len() != gn.len() || nf.len() > 16 {
        return None;
    }
    let mut vals = vec![];
    for j in 1..=nf.len() {
        let v = if j == nf.len() { total } else {
            s.with_input(&[("nf_db", jl(&nf[..j])), ("gain_db", jl(&gn[..j]))]).and_then(|a| k::num_of(&a, "nf_db"))?
        };
        vals.push(v);
    }
    let labels: Vec<String> = (1..=nf.len()).map(|j| format!("after {j}")).collect();
    let body = k::bars(&labels, &vals, &[vals.len() - 1], "Stages in the cascade", "Cascade noise figure (dB)",
                       &format!("cascade noise figure {} dB", num(total)));
    Some(s.chart("bars", "Noise-figure cascade stage by stage",
                 &format!("Friis noise cascade of {} stages with noise figures {} dB and gains {} dB (inputs): the \
                           cascade figure after each stage, each a run of the model on the first stages: {} dB. The first \
                           stage dominates; later stages add little because the gain in front divides their noise.",
                          nf.len(), lst(&nf), lst(&gn), vals.iter().map(|v| num(*v)).collect::<Vec<_>>().join(", ")),
                 &body))
}

fn budget(s: &Solved) -> Option<Figure> {
    let (pt, gt, gr, f, d) = (s.inp("Pt")?, s.inp("Gt_db")?, s.inp("Gr_db")?, s.inp("f")?, s.inp("d")?);
    let pr = s.out("Pr_dbm")?;
    let ptd = s.solve_other("watts_to_dbm", &[("P", Input::Float(pt))]).and_then(|a| k::num_of(&a, "dBm"))?;
    let loss = s.solve_other("free_space_path_loss", &[("d", Input::Float(d)), ("f", Input::Float(f))])
        .and_then(|a| k::num_of(&a, "loss_db"))?;
    // the running level must end where the model's answer is, or the budget is not drawn
    let end = ptd + gt - loss + gr;
    if (end - pr).abs() > 1e-6 * pr.abs().max(1.0) {
        return None;
    }
    let levels = [ptd, ptd + gt, ptd + gt - loss, pr];
    let (lo, hi) = g::range(&levels, true);
    let yr = (lo - (hi - lo) * 0.12, hi + (hi - lo) * 0.12);
    let (p0, pre) = g::single((-0.5, 3.5), yr, (false, false),
                              &format!("received {} dBm = {}", num(pr), q(s.out("Pr")?, "W")), &[]);
    let (top, bot, h) = (p0.top, p0.bot, p0.h);
    let mut p = Panel::ints((-0.5, 3.5), yr, false, top, bot, h);
    let pts: Vec<(f64, f64)> = levels.iter().enumerate().map(|(i, v)| (i as f64, *v)).collect();
    p.steps(&pts, 0);
    p.dots(&pts, 0, 4.0);
    // each level named beside its point: the first two above, the last two on either side of the drop's foot
    for (i, v) in levels.iter().enumerate() {
        let (px, py) = (p.xa.px(i as f64), p.ya.px(*v));
        let dy = if i == 2 { 20.0 } else { -12.0 };
        let (dx, anchor) = (0.0, "middle");
        let t = format!("{} dBm", num(*v));
        g::text(&mut p.s, px + dx, py + dy, anchor, "axp-vt", &t, h);
    }
    p.hline(0.0, "0 dBm = 1 mW");
    p.labels("Stage (0 transmitter, 1 after Gt, 2 after the path, 3 received)", "Level (dBm)");
    let body = format!("{pre}{}", p.done());
    Some(s.chart("budget", "Link budget: from transmitter to receiver",
                 &format!("A free-space link budget: Pt = {} = {} dBm, transmit gain {} dBi, path loss {} dB at d = {} and \
                           f = {} (from the model's dBm and path-loss capabilities), receive gain {} dBi; the running \
                           level ends at the model's received power Pr = {} dBm = {}.", q(pt, "W"), num(ptd), num(gt),
                          num(loss), q(d, "m"), q(f, "Hz"), num(gr), num(pr), q(s.out("Pr")?, "W")), &body))
}

// ------------------------------------------------------------------------------------------------ value fields
enum Rng {
    /// half to one and a half times the value
    Near,
    /// a tenth to ten times (logarithmic)
    Decade,
    /// the value plus or minus this much
    Around(f64),
    /// whole powers of two around the value (a number of points)
    Pow2,
}

fn values_of(x0: f64, r: &Rng, n: usize) -> (Vec<f64>, bool) {
    match r {
        Rng::Near if x0 != 0.0 => {
            let (a, b) = (x0 * 0.5, x0 * 1.5);
            (k::span(a.min(b), a.max(b), n, false), false)
        }
        Rng::Near => (k::span(-1.0, 1.0, n, false), false),
        Rng::Decade if x0 > 0.0 => (k::span(x0 / 10.0, x0 * 10.0, n, true), true),
        Rng::Decade => (k::span(-1.0, 1.0, n, false), false),
        Rng::Around(d) => (k::span(x0 - d, x0 + d, n, false), false),
        Rng::Pow2 => {
            let e = x0.max(2.0).log2().round() as i32;
            let lo = (e - 6).max(1);
            ((lo..=lo + 12).map(|p| 2f64.powi(p)).collect(), true)
        }
    }
}

fn unit_of(s: &Solved, sym: &str) -> String {
    let kind = s.p.procedure(s.key).and_then(|p| p.inputs.iter().find(|i| i.symbol == sym)).map(|i| i.kind.clone());
    match kind.and_then(|kd| s.p.kinds.get(&kd).map(|x| x.unit.clone())) {
        Some(u) if u != "1" && !u.is_empty() => format!("{sym} ({u})"),
        _ => sym.to_string(),
    }
}

fn field2(s: &Solved, xs_: &str, xr: Rng, ys_: &str, yr: Rng, out: &str, title: &str) -> Option<Figure> {
    let int_x = matches!(xr, Rng::Pow2);
    // a whole-number input (a count of points) is read as given
    let x0 = if int_x { s.val(xs_).map(|v| v.int() as f64)? } else { s.inp(xs_)? };
    let y0 = s.inp(ys_)?;
    let z0 = s.out(out)?;
    let (xs, xlog) = values_of(x0, &xr, 13);
    let (ys, ylog) = values_of(y0, &yr, 11);
    let mut vals = vec![];
    let mut n = 0;
    for x in &xs {
        let mut col = vec![];
        for y in &ys {
            let a = if int_x {
                s.with_input(&[(xs_, Input::Int(*x as i64)), (ys_, Input::Float(*y))])
            } else {
                s.with(&[(xs_, *x), (ys_, *y)])
            };
            let v = a.and_then(|a| k::num_of(&a, out));
            n += v.is_some() as usize;
            col.push(v);
        }
        vals.push(col);
    }
    if n < 4 {
        return None;
    }
    let meaning = s.p.procedure(s.key).and_then(|p| p.outputs.iter().find(|o| o.symbol == out))
        .map(|o| o.meaning.clone()).unwrap_or_else(|| out.to_string());
    let body = k::field(&xs, &ys, &vals, (x0, y0), &unit_of(s, xs_), &unit_of(s, ys_), &format!("{out} = {}", num(z0)),
                        (xlog, ylog));
    Some(s.chart("field", title,
                 &format!("{meaning} ({out}) over {xs_} from {} to {} and {ys_} from {} to {}{}: each of the {n} cells is \
                           one computation of the model, shaded by its value (darker is larger; a blank cell: inputs the \
                           model refuses). Marked: the inputs given, {out} = {}.", num(xs[0]), num(xs[xs.len() - 1]),
                          num(ys[0]), num(ys[ys.len() - 1]), if xlog || ylog { " (logarithmic axes)" } else { "" },
                          num(z0)), &body))
}
