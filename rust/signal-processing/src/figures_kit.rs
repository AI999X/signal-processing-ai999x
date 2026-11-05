//! Signal Processing's own drawing helpers for its live figures (stems, stacked panels, a z-plane with the unit
//! circle, step curves, dashed guides): plain SVG in the website's figure classes (axp-*), so the page's light and
//! dark themes apply. Only drawing lives here; every value drawn comes from the model (figures.rs).

use ax_engine::figkit::num;

pub const X0: f64 = 96.0;
pub const X1: f64 = 504.0;
pub const LINE: [&str; 4] = ["axp-c", "axp-c2", "axp-c3", "axp-c4"];
pub const DOT: [&str; 4] = ["axp-d", "axp-d2", "axp-d3", "axp-d4"];
pub const BAR: [&str; 4] = ["axp-b", "axp-b2", "axp-b3", "axp-b4"];

pub fn esc(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;")
}

/// Width of a text as the figure check estimates it (headings 9 px a character, other text 7.6 px).
fn width(t: &str, class: &str) -> f64 {
    t.chars().count() as f64 * if class == "axp-mt" { 9.0 } else { 7.6 }
}

/// A text kept inside the 520-wide canvas and between y = 14 and `h` - 4.
pub fn text(out: &mut String, x: f64, y: f64, anchor: &str, class: &str, t: &str, h: f64) {
    if t.is_empty() {
        return;
    }
    let w = width(t, class);
    let left = match anchor {
        "middle" => x - w / 2.0,
        "end" => x - w,
        _ => x,
    };
    let shift = if left < 2.0 { 2.0 - left } else if left + w > 518.0 { 518.0 - (left + w) } else { 0.0 };
    let y = y.clamp(14.0, h - 4.0);
    out.push_str(&format!("<text class=\"{class}\" x=\"{:.1}\" y=\"{y:.1}\" text-anchor=\"{anchor}\">{}</text>",
                          x + shift, esc(t)));
}

/// A rotated axis label (left of a panel).
pub fn ylabel(out: &mut String, top: f64, bot: f64, t: &str) {
    if t.is_empty() {
        return;
    }
    let n = (((bot - top) / 8.5) as usize).max(6);
    let c: Vec<char> = t.chars().collect();
    let t = if c.len() <= n { t.to_string() } else { format!("{}...", c[..n - 3].iter().collect::<String>().trim_end()) };
    out.push_str(&format!("<text class=\"axp-l\" transform=\"translate(16,{:.1}) rotate(-90)\" text-anchor=\"middle\">{}</text>",
                          (top + bot) / 2.0, esc(&t)));
}

#[derive(Clone, Copy)]
pub struct Ax {
    pub lo: f64,
    pub hi: f64,
    pub log: bool,
    a: f64,
    b: f64,
    pub int: bool,
}

impl Ax {
    pub fn new(lo: f64, hi: f64, log: bool, a: f64, b: f64) -> Ax {
        let (mut lo, mut hi) = (lo, hi);
        if log {
            if !(hi > 0.0) {
                hi = 1.0;
            }
            if !(lo > 0.0) || lo >= hi {
                lo = hi / 10.0;
            }
        } else if !(hi > lo) || !(hi - lo).is_finite() {
            let d = if lo == 0.0 || !lo.is_finite() { 1.0 } else { lo.abs() * 0.1 };
            if !lo.is_finite() {
                lo = 0.0;
            }
            hi = lo + d;
            lo -= d;
        }
        Ax { lo, hi, log, a, b, int: false }
    }
    pub fn px(&self, v: f64) -> f64 {
        let f = if self.log { (v.log10() - self.lo.log10()) / (self.hi.log10() - self.lo.log10()) } else {
            (v - self.lo) / (self.hi - self.lo)
        };
        self.a + f * (self.b - self.a)
    }
    pub fn inside(&self, v: f64) -> bool {
        v.is_finite() && (!self.log || v > 0.0) && v >= self.lo.min(self.hi) - 1e-12 * self.hi.abs().max(1.0)
            && v <= self.hi + 1e-12 * self.hi.abs().max(1.0)
    }
    pub fn ticks(&self) -> Vec<f64> {
        if self.log {
            let (e0, e1) = (self.lo.log10().ceil() as i32, self.hi.log10().floor() as i32);
            if e1 >= e0 {
                let n = e1 - e0 + 1;
                let step = ((n as f64) / 6.0).ceil().max(1.0) as usize;
                return (e0..=e1).step_by(step).map(|e| 10f64.powi(e)).collect();
            }
            return nice(self.lo, self.hi, 3);
        }
        let mut t = nice(self.lo, self.hi, 5);
        if self.int {
            t.retain(|x| (x - x.round()).abs() < 1e-9);
            if t.is_empty() {
                t.push(self.lo.ceil());
            }
        }
        t
    }
}

fn nice(lo: f64, hi: f64, n: usize) -> Vec<f64> {
    let span = hi - lo;
    if !(span > 0.0) || !span.is_finite() {
        return vec![lo];
    }
    let raw = span / n as f64;
    let mag = 10f64.powf(raw.log10().floor());
    let norm = raw / mag;
    let step = mag * if norm < 1.5 { 1.0 } else if norm < 3.0 { 2.0 } else if norm < 7.0 { 5.0 } else { 10.0 };
    let mut v = vec![];
    let mut t = (lo / step).ceil() * step;
    while t <= hi + step * 1e-9 && v.len() < 12 {
        v.push(if t.abs() < step * 1e-9 { 0.0 } else { t });
        t += step;
    }
    v
}

/// The range of some values, padded by 6 % each side; `zero`: 0 is included (stems, bars).
pub fn range(v: &[f64], zero: bool) -> (f64, f64) {
    let f: Vec<f64> = v.iter().copied().filter(|x| x.is_finite()).collect();
    if f.is_empty() {
        return (0.0, 1.0);
    }
    let (mut lo, mut hi) = f.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(a, b), x| (a.min(*x), b.max(*x)));
    if zero {
        lo = lo.min(0.0);
        hi = hi.max(0.0);
    }
    if hi <= lo {
        let d = if lo == 0.0 { 1.0 } else { lo.abs() * 0.2 };
        return (lo - d, hi + d);
    }
    let d = (hi - lo) * 0.06;
    (lo - d, hi + d)
}

/// One plotting panel of a figure: axes, a grid with tick labels, and what is drawn in it (clipped to it).
pub struct Panel {
    pub xa: Ax,
    pub ya: Ax,
    pub x0: f64,
    pub x1: f64,
    pub top: f64,
    pub bot: f64,
    pub h: f64,
    pub s: String,
    clip: String,
}

impl Panel {
    /// A panel spanning the usual width (x 96 to 504) from `top` to `bot`, in a canvas `h` tall.
    pub fn new(xr: (f64, f64), yr: (f64, f64), log: (bool, bool), top: f64, bot: f64, h: f64) -> Panel {
        Panel::at(xr, yr, log, (X0, X1), top, bot, h, false)
    }
    /// A panel whose horizontal axis counts samples (integer ticks).
    pub fn ints(xr: (f64, f64), yr: (f64, f64), logy: bool, top: f64, bot: f64, h: f64) -> Panel {
        Panel::at(xr, yr, (false, logy), (X0, X1), top, bot, h, true)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn at(xr: (f64, f64), yr: (f64, f64), log: (bool, bool), xpx: (f64, f64), top: f64, bot: f64, h: f64,
              int: bool) -> Panel {
        let mut xa = Ax::new(xr.0, xr.1, log.0, xpx.0, xpx.1);
        xa.int = int;
        let ya = Ax::new(yr.0, yr.1, log.1, bot, top);
        let clip = format!("CLIPIDp{}", top as i64);
        let mut s = String::new();
        for t in xa.ticks() {
            let x = xa.px(t);
            s.push_str(&format!("<line class=\"axp-g\" x1=\"{x:.1}\" y1=\"{top:.1}\" x2=\"{x:.1}\" y2=\"{bot:.1}\"/>"));
            text(&mut s, x, bot + 18.0, "middle", "axp-t", &num(t), h);
        }
        let mut yt = ya.ticks();
        // a short panel gets fewer horizontal grid lines (one every 34 px or more)
        let room = ((bot - top) / 34.0).floor().max(2.0) as usize;
        if !ya.log && yt.len() > room + 1 {
            // the most ticks that fit, never fewer than two
            for n in (1..=room + 1).rev() {
                let t = nice(ya.lo, ya.hi, n);
                if t.len() <= room + 1 && t.len() >= 2 {
                    yt = t;
                    break;
                }
            }
        }
        for t in yt {
            let y = ya.px(t);
            s.push_str(&format!("<line class=\"axp-g\" x1=\"{:.1}\" y1=\"{y:.1}\" x2=\"{:.1}\" y2=\"{y:.1}\"/>", xpx.0, xpx.1));
            text(&mut s, xpx.0 - 6.0, y + 5.0, "end", "axp-t", &num(t), h);
        }
        s.push_str(&format!("<rect class=\"axp-f\" x=\"{:.1}\" y=\"{top:.1}\" width=\"{:.1}\" height=\"{:.1}\"/>\
                             <clipPath id=\"{clip}\"><rect x=\"{:.1}\" y=\"{top:.1}\" width=\"{:.1}\" height=\"{:.1}\"/></clipPath>",
                            xpx.0, xpx.1 - xpx.0, bot - top, xpx.0, xpx.1 - xpx.0, bot - top));
        Panel { xa, ya, x0: xpx.0, x1: xpx.1, top, bot, h, s, clip }
    }
    fn g(&self) -> String {
        format!("<g clip-path=\"url(#{})\">", self.clip)
    }
    /// The axis names: below the panel and rotated on its left.
    pub fn labels(&mut self, xlab: &str, ylab: &str) {
        let (x0, x1, bot, h) = (self.x0, self.x1, self.bot, self.h);
        text(&mut self.s, (x0 + x1) / 2.0, bot + 40.0, "middle", "axp-l", xlab, h);
        ylabel(&mut self.s, self.top, self.bot, ylab);
    }
    /// A bold title over the panel.
    pub fn title(&mut self, t: &str) {
        let (x0, top, h) = (self.x0, self.top, self.h);
        text(&mut self.s, x0, top - 8.0, "start", "axp-bt", t, h);
    }
    fn pt(&self, x: f64, y: f64) -> Option<(f64, f64)> {
        if !x.is_finite() || !y.is_finite() || (self.xa.log && x <= 0.0) || (self.ya.log && y <= 0.0) {
            return None;
        }
        let (px, py) = (self.xa.px(x), self.ya.px(y));
        (px.is_finite() && py.is_finite()).then_some((px, py))
    }
    /// A polyline in colour `c` (a non-finite point breaks it).
    pub fn line(&mut self, pts: &[(f64, f64)], c: usize) {
        self.path(pts, LINE[c % 4], "");
    }
    /// A dashed polyline in colour `c`.
    pub fn dash(&mut self, pts: &[(f64, f64)], c: usize) {
        self.path(pts, LINE[c % 4], " stroke-dasharray=\"6 4\"");
    }
    fn path(&mut self, pts: &[(f64, f64)], class: &str, extra: &str) {
        let mut d = String::new();
        let mut pen = false;
        for (x, y) in pts {
            match self.pt(*x, *y) {
                Some((px, py)) => {
                    d.push_str(&format!("{}{px:.1},{py:.1} ", if pen { "L" } else { "M" }));
                    pen = true;
                }
                None => pen = false,
            }
        }
        if !d.is_empty() {
            let g = self.g();
            self.s.push_str(&format!("{g}<path class=\"{class}\"{extra} d=\"{}\"/></g>", d.trim_end()));
        }
    }
    /// A step curve (constant until the next point), as an order against a frequency.
    pub fn steps(&mut self, pts: &[(f64, f64)], c: usize) {
        let mut v = vec![];
        for (i, p) in pts.iter().enumerate() {
            if i > 0 {
                v.push((p.0, pts[i - 1].1));
            }
            v.push(*p);
        }
        self.line(&v, c);
    }
    /// Stems from zero (or the bottom of the axis) to each value, with a dot; `dx`: a sideways shift in pixels (two
    /// series side by side).
    pub fn stems(&mut self, pts: &[(f64, f64)], c: usize, dx: f64) {
        let base_v = if self.ya.log { self.ya.lo } else { 0.0f64.clamp(self.ya.lo, self.ya.hi) };
        let base = self.ya.px(base_v);
        let mut out = self.g();
        for (x, y) in pts {
            let y = if self.ya.log { y.max(self.ya.lo) } else { *y };
            if let Some((px, py)) = self.pt(*x, y) {
                let px = px + dx;
                out.push_str(&format!("<path class=\"{}\" d=\"M{px:.1},{base:.1} L{px:.1},{py:.1}\"/>\
                                       <circle class=\"{}\" cx=\"{px:.1}\" cy=\"{py:.1}\" r=\"3.6\"/>", LINE[c % 4], DOT[c % 4]));
            }
        }
        out.push_str("</g>");
        self.s.push_str(&out);
    }
    /// Dots of colour `c`.
    pub fn dots(&mut self, pts: &[(f64, f64)], c: usize, r: f64) {
        let mut out = self.g();
        for (x, y) in pts {
            if let Some((px, py)) = self.pt(*x, *y) {
                out.push_str(&format!("<circle class=\"{}\" cx=\"{px:.1}\" cy=\"{py:.1}\" r=\"{r:.1}\"/>", DOT[c % 4]));
            }
        }
        out.push_str("</g>");
        self.s.push_str(&out);
    }
    /// A dashed horizontal guide at y, named at its left end (beside another guide named at the right).
    pub fn hline_left(&mut self, y: f64, label: &str) {
        if !self.ya.inside(y) {
            return;
        }
        let py = self.ya.py(y);
        self.s.push_str(&format!("<path class=\"axp-c4\" stroke-dasharray=\"5 4\" d=\"M{:.1},{py:.1} L{:.1},{py:.1}\"/>",
                                 self.x0, self.x1));
        let (x0, h) = (self.x0, self.h);
        let ty = if py + 15.0 > self.bot - 2.0 { py - 5.0 } else { py + 15.0 };
        text(&mut self.s, x0 + 4.0, ty, "start", "axp-lt", label, h);
    }
    /// A dashed horizontal guide at y, named at its right end.
    pub fn hline(&mut self, y: f64, label: &str) {
        if !self.ya.inside(y) {
            return;
        }
        let py = self.ya.py(y);
        self.s.push_str(&format!("<path class=\"axp-c4\" stroke-dasharray=\"5 4\" d=\"M{:.1},{py:.1} L{:.1},{py:.1}\"/>",
                                 self.x0, self.x1));
        let (x1, h) = (self.x1, self.h);
        let ty = if py - 5.0 < self.top + 12.0 { py + 15.0 } else { py - 5.0 };
        text(&mut self.s, x1 - 4.0, ty, "end", "axp-lt", label, h);
    }
    /// A dashed vertical guide at x, named near the top of the panel.
    pub fn vline(&mut self, x: f64, label: &str, row: usize) {
        if !self.xa.inside(x) {
            return;
        }
        let px = self.xa.px(x);
        self.s.push_str(&format!("<path class=\"axp-c4\" stroke-dasharray=\"5 4\" d=\"M{px:.1},{:.1} L{px:.1},{:.1}\"/>",
                                 self.top, self.bot));
        let w = width(label, "axp-lt");
        let (tx, anchor) = if px + 4.0 + w <= self.x1 { (px + 4.0, "start") } else { (px - 4.0, "end") };
        let (top, h) = (self.top, self.h);
        text(&mut self.s, tx, top + 14.0 + 15.0 * row as f64, anchor, "axp-lt", label, h);
    }
    /// The visitor's point, ringed and labelled (the label kept inside the panel).
    pub fn mark(&mut self, x: f64, y: f64, label: &str) {
        let Some((px, py)) = self.pt(x, y) else { return };
        let py = py.clamp(self.top, self.bot);
        self.s.push_str(&format!("<circle class=\"axp-m\" cx=\"{px:.1}\" cy=\"{py:.1}\" r=\"6\"/>"));
        let w = width(label, "axp-mt");
        let (tx, anchor) = if px + 10.0 + w <= self.x1 { (px + 10.0, "start") } else if px - 10.0 - w >= self.x0 {
            (px - 10.0, "end")
        } else {
            (((self.x0 + self.x1) / 2.0).clamp(self.x0 + w / 2.0, self.x1 - w / 2.0), "middle")
        };
        let ty = if py < self.top + 30.0 { py + 22.0 } else { py - 10.0 };
        let h = self.h;
        text(&mut self.s, tx, ty, anchor, "axp-mt", label, h);
    }
    pub fn done(self) -> String {
        self.s
    }
}

impl Ax {
    fn py(&self, v: f64) -> f64 {
        self.px(v)
    }
}

/// The rows a legend needs between x = 16 and 504.
pub fn legend_rows(items: &[(String, usize)]) -> usize {
    let mut rows = 1;
    let mut x = 16.0;
    for (n, _) in items {
        let w = 20.0 + width(n, "axp-lt") + 14.0;
        if x + w > 504.0 && x > 16.0 {
            rows += 1;
            x = 16.0;
        }
        x += w;
    }
    rows
}

/// A legend: a colour swatch and a name per item, in rows from y (a row is 16 px).
pub fn legend(out: &mut String, items: &[(String, usize)], y: f64, h: f64) {
    let mut x = 16.0;
    let mut y = y;
    for (n, c) in items {
        let w = 20.0 + width(n, "axp-lt") + 14.0;
        if x + w > 504.0 && x > 16.0 {
            y += 16.0;
            x = 16.0;
        }
        out.push_str(&format!("<rect class=\"{}\" x=\"{x:.1}\" y=\"{:.1}\" width=\"14\" height=\"10\"/>", BAR[c % 4], y - 9.0));
        text(out, x + 20.0, y, "start", "axp-lt", n, h);
        x += w;
    }
}

/// A heading at the top of a figure.
pub fn header(out: &mut String, t: &str, h: f64) {
    text(out, 16.0, 20.0, "start", "axp-mt", t, h);
}

/// One chart in the usual 520 x 340 frame, with a heading and an optional legend above the plot: the panel (its
/// top lowered by the legend's rows) and the prefix (heading + legend) to put before it.
pub fn single(xr: (f64, f64), yr: (f64, f64), log: (bool, bool), head: &str, items: &[(String, usize)])
              -> (Panel, String) {
    let h = 340.0;
    let mut pre = String::new();
    header(&mut pre, head, h);
    let mut top = 44.0;
    if !items.is_empty() {
        legend(&mut pre, items, 40.0, h);
        top = 50.0 + 16.0 * legend_rows(items) as f64;
    }
    (Panel::new(xr, yr, log, top, 278.0, h), pre)
}

/// Vertical positions of `n` stacked panels `ph` tall under a heading (and `lrows` legend rows): their (top,
/// bottom) and the canvas height.
pub fn stack_rows(n: usize, ph: f64, lrows: usize) -> (Vec<(f64, f64)>, f64) {
    let mut top = 58.0 + 16.0 * lrows as f64;
    let mut v = vec![];
    for _ in 0..n {
        v.push((top, top + ph));
        top += ph + 72.0;
    }
    let h = top - 72.0 + ph * 0.0 + 52.0;
    (v, h)
}

/// The z-plane (or the reflection-coefficient plane): equal scales, the unit circle, the axes through 0; `r` is the
/// half-width shown. Returns the panel, square, centred in a 520 x 340 canvas.
pub fn zplane(r: f64) -> Panel {
    plane(r, true)
}

/// An equal-scale plane centred on 0 (half-width `r`), the axes through 0 dashed, the unit circle when `unit`; the
/// panel is square, on the left of a 520 x 340 canvas (notes go right of it, from x = 320).
pub fn plane(r: f64, unit: bool) -> Panel {
    let h = 340.0;
    let (top, bot) = (44.0, 300.0);
    let side = bot - top;
    let x0 = 58.0;
    let mut p = Panel::at((-r, r), (-r, r), (false, false), (x0, x0 + side), top, bot, h, false);
    let (cx, cy) = (p.xa.px(0.0), p.ya.px(0.0));
    let rr = p.xa.px(1.0) - cx;
    let g = p.g();
    p.s.push_str(&format!("{g}<path class=\"axp-c4\" stroke-dasharray=\"5 4\" d=\"M{:.1},{cy:.1} L{:.1},{cy:.1} M{cx:.1},{top:.1} L{cx:.1},{bot:.1}\"/></g>",
                          p.x0, p.x1));
    if unit {
        p.s.push_str(&format!("{g}<circle class=\"axp-k\" cx=\"{cx:.1}\" cy=\"{cy:.1}\" r=\"{rr:.1}\"/></g>"));
    }
    p
}

impl Panel {
    /// An arrow from the origin to (x, y) in colour c (a phasor), with a dot at its tip.
    pub fn phasor(&mut self, x: f64, y: f64, c: usize) {
        let (Some((ox, oy)), Some((px, py))) = (self.pt(0.0, 0.0), self.pt(x, y)) else { return };
        let (dx, dy) = (px - ox, py - oy);
        let d = (dx * dx + dy * dy).sqrt();
        let g = self.g();
        if d < 1.0 {
            self.s.push_str(&format!("{g}<circle class=\"{}\" cx=\"{px:.1}\" cy=\"{py:.1}\" r=\"4\"/></g>", DOT[c % 4]));
            return;
        }
        let (ux, uy) = (dx / d, dy / d);
        let (bx, by) = (px - ux * 9.0, py - uy * 9.0);
        self.s.push_str(&format!("{g}<path class=\"{}\" d=\"M{ox:.1},{oy:.1} L{bx:.1},{by:.1}\"/>\
                                  <path class=\"{}\" d=\"M{px:.1},{py:.1} L{:.1},{:.1} L{:.1},{:.1} Z\"/></g>",
                                 LINE[c % 4], DOT[c % 4], bx - uy * 5.0, by + ux * 5.0, bx + uy * 5.0, by - ux * 5.0));
    }
    /// A label beside a point of the panel (kept in the canvas).
    pub fn label_at(&mut self, x: f64, y: f64, t: &str, class: &str) {
        let Some((px, py)) = self.pt(x, y) else { return };
        let h = self.h;
        let anchor = if px > (self.x0 + self.x1) / 2.0 { "start" } else { "end" };
        let tx = if anchor == "start" { px + 8.0 } else { px - 8.0 };
        let ty = if py < (self.top + self.bot) / 2.0 { py - 6.0 } else { py + 16.0 };
        text(&mut self.s, tx, ty, anchor, class, t, h);
    }
}

impl Panel {
    /// Crosses (poles) in colour c.
    pub fn crosses(&mut self, pts: &[(f64, f64)], c: usize) {
        let mut out = self.g();
        for (x, y) in pts {
            if let Some((px, py)) = self.pt(*x, *y) {
                out.push_str(&format!("<path class=\"{}\" d=\"M{:.1},{:.1} L{:.1},{:.1} M{:.1},{:.1} L{:.1},{:.1}\"/>",
                                      LINE[c % 4], px - 6.0, py - 6.0, px + 6.0, py + 6.0, px - 6.0, py + 6.0, px + 6.0,
                                      py - 6.0));
            }
        }
        out.push_str("</g>");
        self.s.push_str(&out);
    }
    /// Open rings (zeros) in colour c.
    pub fn rings(&mut self, pts: &[(f64, f64)], c: usize) {
        let mut out = self.g();
        for (x, y) in pts {
            if let Some((px, py)) = self.pt(*x, *y) {
                out.push_str(&format!("<circle class=\"{}\" cx=\"{px:.1}\" cy=\"{py:.1}\" r=\"6\"/>", LINE[c % 4]));
            }
        }
        out.push_str("</g>");
        self.s.push_str(&out);
    }
    /// A circle round (cx, cy) of radius r in data units, dashed.
    pub fn circle(&mut self, cx: f64, cy: f64, r: f64) {
        let Some((px, py)) = self.pt(cx, cy) else { return };
        let rr = (self.xa.px(cx + r) - px).abs();
        if !rr.is_finite() {
            return;
        }
        let g = self.g();
        self.s.push_str(&format!("{g}<circle class=\"axp-c4\" stroke-dasharray=\"5 4\" cx=\"{px:.1}\" cy=\"{py:.1}\" r=\"{rr:.1}\"/></g>"));
    }
}

/// Text lines in a column to the right of a plane panel (x from 330); a line with a colour gets a swatch.
pub fn side_notes(out: &mut String, lines: &[(String, &str, Option<usize>)], y0: f64, h: f64) {
    for (i, (t, class, c)) in lines.iter().enumerate() {
        let y = y0 + 18.0 * i as f64;
        let mut x = 330.0;
        if let Some(c) = c {
            out.push_str(&format!("<rect class=\"{}\" x=\"{x:.1}\" y=\"{:.1}\" width=\"12\" height=\"10\"/>", BAR[c % 4], y - 9.0));
            x += 18.0;
        }
        text(out, x, y, "start", class, t, h);
    }
}
