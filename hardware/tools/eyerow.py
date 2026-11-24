"""Eye-position correction + eye-light net mass rows (Woz 2026-11-24). Point masses on headrow.py.
Fixes head_grams_v1 l.73: eye cut and bulbs were placed at (9.9, -40, 25) and (10, 0, 30). Basis is the
upstream printed shell b23317a print/head.stl: both eye holes Ø25 centred (10.35, ±40), front wall
z -59.05..-56.05 (woz/eyefit/printfit2.log). Z_EYE = cut #1 point (right bezel out at -62.06, mirrored left
in at -61.05, mass-weighted); Z_BULB = bulb.stl centroid; both from the print STLs in the head_assembly frame.
Disc Ø25 in both eyes (Chief 2026-11-24 15:06).
All masses are Woz estimates; nothing weighed. Usage: python3 eyerow.py"""
import numpy as np
from headrow import BASE, HEADER, OPTIONS, PATH, Y_RIGHT, ballast_for, row, fullinertia, C0

Z_EYE, Z_BULB, Z_LED = -62.51, -55.08, -47.0
X = 10.35
MIC_R = OPTIONS["A mic to right wall (mirror)"]
AMP_WALL = [("amp breakout off y -45", -2, (10, -45, 25)), ("amp breakout at right wall", 2, (10, Y_RIGHT, 25))]
YT = -4.2


def fixed(items):
    out = []
    for lab, dm, p in items:
        if lab.startswith("1 right eye"):
            out.append((lab, dm, (X, -40, Z_EYE)))
        elif lab.startswith("6 bulb"):
            out += [(lab + " L", dm / 2, (X, 40, Z_BULB)), (lab + " R", dm / 2, (X, -40, Z_BULB))]
        else:
            out.append((lab, dm, p))
    return out


def eyelight(final):
    """Replace cut 6 by: both upstream bulbs out (solid-PLA STL upper bound 2.10 g each),
    LED + wire 0.3 g + tapered PETG tube (half the straight tube) + disc in, per eye."""
    items = []
    for y, bore in ((40, 25.0), (-40, 25.0)):
        ri, ro, L = bore / 2, bore / 2 + 1.2, 22.35  # disc rear -57.55 .. epoxy -35.2
        tube = np.pi * (ro**2 - ri**2) * L / 1000 * 1.27 / 2
        disc = np.pi * ri**2 * (2.0 if final else 1.0) / 1000 * (1.19 if final else 1.27)
        items += [(f"bulb out y{y:+d}", -2.10, (X, y, Z_BULB)), (f"eye light in y{y:+d}", tube + disc + 0.3, (X, y, Z_LED))]
        print(f"   y{y:+d}: tube {tube:.2f} disc {disc:.2f} LED 0.30 -> {tube + disc + 0.3:.2f} g")
    return items


def build(fix):
    P = fixed(PATH) if fix else PATH
    A = BASE + P
    alt = A + HEADER
    r410 = alt + MIC_R + AMP_WALL + [("HAT parts+FR4 to trunk", -6.6, (17, 47, 6))]
    r407 = alt + MIC_R + AMP_WALL + [("all HAT to trunk", -8.1, (17, 47, 6))]
    rows = {"B 421.2": BASE + [it for it in P if not it[0].startswith("4 ")], "A 414.2": A, "412.7": alt}
    for k, r in (("410.0", r410), ("407.4", r407)):
        g = ballast_for(r, YT, (10, Y_RIGHT, 25))
        rows[k] = r + [("ballast", g, (10, Y_RIGHT, 25))]
    return rows


def show(lab, items):
    m, c, inertia = row(items)
    d = c - C0
    print(f"{lab:34s} {m:7.2f} g  CoM ({c[0]:.3f}, {c[1]:.3f}, {c[2]:.3f})  d ({d[0]:+.2f}, {d[1]:+.2f}, {d[2]:+.2f})"
          f"  |d| {np.linalg.norm(d):.2f}")
    return m, c, inertia


if __name__ == "__main__":
    print("== as published (old positions), should match head_grams_v1 rows")
    for k, r in build(False).items():
        show(k, r)
    print("== corrected eye/bulb positions (l.73 fix), ballast re-solved for y -4.2 where used")
    fixedrows = build(True)
    for k, r in fixedrows.items():
        m, c, inertia = show(k, r)
        print("   fullinertia:", fullinertia(inertia))
    for final in (False, True):
        print(f"== eye light net, {'final PMMA 2.0' if final else 'proto PETG 1.0'} disc, tapered tube; replaces cut 6")
        el = eyelight(final)
        print(f"   net vs upstream bulbs: {sum(dm for _, dm, _ in el):+.2f} g; vs cut-6 rows: {sum(dm for _, dm, _ in el) + 2.0:+.2f} g")
        for k, r in fixedrows.items():
            base = [it for it in r if not it[0].startswith("6 bulb") and it[0] != "ballast"]
            if k in ("410.0", "407.4"):
                g = ballast_for(base + el, YT, (10, Y_RIGHT, 25))
                base = base + [("ballast", g, (10, Y_RIGHT, 25))]
                lab = f"{k} + eye light (ballast {g:.2f})"
            else:
                lab = f"{k} + eye light"
            m, c, inertia = show(lab, base + el)
            print("   fullinertia:", fullinertia(inertia))
