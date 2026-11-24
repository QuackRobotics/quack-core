"""Head mass / CoM / inertia rows for the head grams gate (Woz). Point-mass deltas on the trained head.
Trained head: Playground open_duck_mini_v2.xml head_assembly (sha256 968b18de..., commit b9be205 = f698836 for this file):
mass 0.406607 kg, pos (0.00815416, -0.00390754, 0.0227726) m, fullinertia about that CoM in the body frame.
All positions head_assembly frame, mm. Every delta mass and position is a Woz ESTIMATE (nothing weighed).
Removed parts are modelled as negative point masses, so inertia is approximate (parts' own inertia ignored).
Usage: python3 headrow.py"""
import numpy as np

M0 = 406.607
C0 = np.array([8.15416, -3.90754, 22.7726])
# fullinertia order in MJCF: Ixx Iyy Izz Ixy Ixz Iyz (kg m2)
I0 = np.array([[2.44627e-3, -2.02404e-6, 9.2382e-05],
               [-2.02404e-6, 1.6864e-3, -1.84944e-05],
               [9.2382e-05, -1.84944e-05, 1.08002e-3]])

BASE = [("cam+bracket net +10 allowance", 10, (1, 0, -51)),
        ("HAT + 2 caps", 18, (17, 47, 6)),
        ("amp breakout (head_layout_v1 l.93, right, by speaker)", 2, (10, -45, 25)),
        ("mic (head_layout_v1 l.121, left wall)", 2, (9.9, 74.5, 25))]
PATH = [("1 right eye = mirrored left", -3.97, (9.9, -40, 25)),
        ("2 caps to trunk", -6.2, (12, 47, 6)),
        ("3 HAT FR4 1.0 mm", -2.2, (17, 47, 6)),
        ("5 speaker stand -> ring clamp", -3.0, (10.1, -70, 24)),
        ("6 bulb carriers -> LED pad", -2.0, (10, 0, 30)),
        ("4 cam+bracket net <= +3", -7.0, (1, 0, -51))]
AMP_ON_HAT = [("amp: remove breakout", -2.0, (10, -45, 25)),
              ("amp: chip + passives on HAT", 0.5, (17, 47, 6))]
HEADER = [("low-profile 40-pin header", -1.5, (17, 47, 6))]
Y_RIGHT = -72.0  # Woz est.: inside the right wall, mirror of the mic wall (inner face y ~ +76)
OPTIONS = {
    "A mic to right wall (mirror)": [("mic off left", -2, (9.9, 74.5, 25)), ("mic on right", 2, (9.9, -74.5, 25))],
    "B keep amp breakout on right (drop amp-on-HAT)": [],
    "C keep upstream right eye (drop cut 1)": [("right eye back", 3.97, (9.9, -40, 25))],
    "D keep speaker stand (drop cut 5)": [("speaker stand back", 3.0, (10.1, -70, 24))],
}


def pa(mass_g, r_mm):
    r = np.asarray(r_mm, float) / 1000
    return mass_g / 1000 * (np.dot(r, r) * np.eye(3) - np.outer(r, r))


def row(items):
    m = M0 + sum(dm for _, dm, _ in items)
    c = (M0 * C0 + sum(dm * np.array(p) for _, dm, p in items)) / m
    inertia = I0 + pa(M0, C0 - c)
    for _, dm, p in items:
        inertia = inertia + pa(dm, np.array(p) - c)
    return m, c, inertia


def show(name, items):
    m, c, inertia = row(items)
    d = c - C0
    print(f"{name:52s} {m:6.1f} g {m - M0:+5.1f}  CoM ({c[0]:.2f}, {c[1]:.2f}, {c[2]:.2f})  "
          f"d ({d[0]:+.2f}, {d[1]:+.2f}, {d[2]:+.2f})")
    return m, c, inertia


def ballast_for(items, y_target, pos):
    """grams at pos that bring CoM y to y_target."""
    m, c, _ = row(items)
    return m * (c[1] - y_target) / (y_target - pos[1])


def fullinertia(inertia):
    return " ".join("%.5e" % v for v in (inertia[0, 0], inertia[1, 1], inertia[2, 2],
                                         inertia[0, 1], inertia[0, 2], inertia[1, 2]))


if __name__ == "__main__":
    items = list(BASE)
    show("start (Option C)", items)
    for p in PATH:
        items.append(p)
        show("after " + p[0], items)
    p414 = list(items)
    plan = p414 + AMP_ON_HAT + HEADER
    show("414.2 + amp on HAT", p414 + AMP_ON_HAT)
    m, c, inertia = show("PLAN 411.2: + amp on HAT + low-profile header", plan)
    print("PLAN fullinertia about CoM (Ixx Iyy Izz Ixy Ixz Iyz, kg m2):", fullinertia(inertia))
    print("trained fullinertia:", fullinertia(I0))
    yt = C0[1] + 1.0
    print(f"\ntarget CoM y <= {yt:.2f} mm (trained {C0[1]:.2f} + 1.0)")
    alt = p414 + HEADER
    show("ALT: header only, amp breakout stays on right", alt)
    for name, extra in OPTIONS.items():
        base = alt if name.startswith("B") else plan
        show(name, base + extra)
    for lab, base in (("PLAN", plan), ("ALT", alt), ("ALT + A mic right", alt + OPTIONS["A mic to right wall (mirror)"])):
        g = ballast_for(base, yt, (10, Y_RIGHT, 25))
        m, c, inertia = show(f"{lab} + {g:.2f} g ballast at y {Y_RIGHT:.0f}", base + [("ballast", g, (10, Y_RIGHT, 25))])
        if lab == "ALT + A + E":
            print("   fullinertia:", fullinertia(inertia))
