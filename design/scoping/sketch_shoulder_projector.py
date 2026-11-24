# SPDX-FileCopyrightText: 2026 Quack Robotics contributors
# SPDX-License-Identifier: Apache-2.0
"""Scoping sketch only (NOT part of the v1 GLB): shoulder projector placement on the v1.8 neutral pose.
GLB frame, mm: +x forward, y = height above ground, z lateral (left = -z).
Bounds from inspect_glb.py on qk_pose_neutral_v1.8.glb: trunk shell x -154..46, y 161..304, z +-55;
head shell x -99..101, y 371..430, z +-99; neck sheets x -14..16, z +-23.
Projector proxy: cylinder D14 x 60 (Woz proposal), axis along x, lens flush with chest front (x=+46), z=-45, axis h=312.
"""
import math

P = dict(len=60.0, dia=14.0, lens_x=46.0, z=-45.0, axis_h=312.0, mount_tilt_deg=8.0, body_pitch_deg=10.0)

out = ['<svg xmlns="http://www.w3.org/2000/svg" width="900" height="430" font-family="sans-serif" font-size="11">',
       '<rect width="100%" height="100%" fill="white"/>',
       '<text x="10" y="16" font-weight="bold">Shoulder projector scoping (NOT v1 GLB): side view left, top view right. mm, v1.8 neutral</text>']


def side(x, y):
    return 40 + (x + 160) * 0.9, 400 - y * 0.81


def top(x, z):
    return 640 + z * 1.2, 230 - x


def box(view, x0, a0, x1, a1, stroke, fill):
    pts = [view(x0, a0), view(x1, a1)]
    xs = sorted(p[0] for p in pts)
    ys = sorted(p[1] for p in pts)
    out.append(f'<rect x="{xs[0]:.1f}" y="{ys[0]:.1f}" width="{xs[1]-xs[0]:.1f}" height="{ys[1]-ys[0]:.1f}" fill="{fill}" stroke="{stroke}"/>')


def text(view, x, a, s, col="black"):
    px, py = view(x, a)
    out.append(f'<text x="{px:.1f}" y="{py:.1f}" fill="{col}">{s}</text>')


r = P["dia"] / 2
# side view
box(side, -154, 161, 46, 304, "#555", "#eee")
box(side, -99, 371, 101, 430, "#555", "#f6f6f6")
box(side, -14, 275, 16, 371, "#555", "#ddd")
box(side, P["lens_x"] - P["len"], P["axis_h"] - r, P["lens_x"], P["axis_h"] + r, "#c00", "#f99")
for ang, col in [(P["mount_tilt_deg"], "#c00"), (P["mount_tilt_deg"] + P["body_pitch_deg"], "#f80"),
                 (P["mount_tilt_deg"] - P["body_pitch_deg"], "#f80")]:
    x0, y0 = P["lens_x"], P["axis_h"]
    d = (y0 / math.tan(math.radians(ang))) if ang > 0 else 600
    x1 = x0 + min(d, 600)
    y1 = y0 - (x1 - x0) * math.tan(math.radians(ang))
    a, b = side(x0, y0), side(x1, y1)
    out.append(f'<line x1="{a[0]:.1f}" y1="{a[1]:.1f}" x2="{b[0]:.1f}" y2="{b[1]:.1f}" stroke="{col}" stroke-dasharray="4 3"/>')
g0, g1 = side(-160, 0), side(660, 0)
out.append(f'<line x1="{g0[0]}" y1="{g0[1]}" x2="{g1[0]}" y2="{g1[1]}" stroke="black"/>')
text(side, -150, 440, "head shell 371-430 (overhangs trunk)")
text(side, 60, 330, "projector D14x60, lens x=+46, h=312, tilt -8 deg", "#c00")
text(side, 160, 220, "+-10 deg body pitch envelope (Atlas sets real limit)", "#f80")
# top view
box(top, -154, -55, 46, 55, "#555", "#eee")
cx, cy = top(1, 0)
out.append(f'<circle cx="{cx}" cy="{cy}" r="{99*1.2:.0f}" fill="none" stroke="#999" stroke-dasharray="3 3"/>')
box(top, -14, -23, 16, 23, "#555", "#ddd")
box(top, P["lens_x"] - P["len"], P["z"] - r, P["lens_x"], P["z"] + r, "#c00", "#f99")
text(top, 75, -110, "fwd +x (up), left = -z")
text(top, -180, -110, "dashed: head footprint above it")
text(top, 60, -60, "left shoulder z=-45", "#c00")
out.append("</svg>")
open("shoulder_projector_sketch.svg", "w").write("\n".join(out))
print("ok")
