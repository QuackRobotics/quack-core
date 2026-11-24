"""Head CoM-y options for Hinton's lateral band (Woz scoping, 2026-11-24). Point-mass estimates on headrow.py.
Usage: python3 headopts.py"""
from headrow import AMP_ON_HAT, BASE, HEADER, OPTIONS, PATH, Y_RIGHT, ballast_for, row

P414 = BASE + PATH
PLAN = P414 + AMP_ON_HAT + HEADER
ALT = P414 + HEADER
MIC_R = OPTIONS["A mic to right wall (mirror)"]
AMP_WALL = [("amp breakout off y -45", -2, (10, -45, 25)), ("amp breakout at right wall", 2, (10, Y_RIGHT, 25))]
HAT_NET = 18 - 6.2 - 2.2 - 1.5  # g left on the Pi at y +47 after cuts 2, 3 and the header

if __name__ == "__main__":
    cases = (("PLAN 411.2 (amp on HAT)", PLAN), ("ALT amp breakout stays right", ALT),
             ("ALT + mic right wall", ALT + MIC_R), ("ALT + mic right + amp to wall", ALT + MIC_R + AMP_WALL))
    for yt in (-4.2, -2.91):
        print(f"== target CoM y {yt}")
        for lab, b in cases:
            m, c, _ = row(b)
            g = ballast_for(b, yt, (10, Y_RIGHT, 25))
            print(f"{lab:32s} {m:6.1f} g  y {c[1]:+.2f}  moment {m * (c[1] - yt):5.0f} g.mm  "
                  f"ballast@y{Y_RIGHT:.0f} {g:5.2f} g -> {m + g:6.1f} g")
        m, c, _ = row(ALT + MIC_R + AMP_WALL)
        print(f"   or shift HAT net {HAT_NET:.1f} g right by {m * (c[1] - yt) / HAT_NET:.0f} mm")
