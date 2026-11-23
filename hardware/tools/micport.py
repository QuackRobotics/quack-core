"""Mic port clearance check (head_assembly frame, mm). Reuses hatfit.py part placement."""
import numpy as np
src = open(__file__.replace('micport.py', 'hatfit.py')).read()
exec(src.split("for k, t in parts.items()")[0])
exec("def ray_hits" + src.split("def ray_hits")[1].split("others =")[0])

P = np.array([9.9, 79.2, 25.0])  # Luxo's left-wall port
for k, t in parts.items():
    v = t.reshape(-1, 3)
    d = np.linalg.norm(v - P, axis=1)
    i = d.argmin()
    print(f'{k:16s} nearest vertex {d.min():6.1f} mm at {np.round(v[i], 1)}')

# rays from outside the head along -y, through the port and a 3 mm ring around it
for dx, dz in [(0, 0), (3, 0), (-3, 0), (0, 3), (0, -3)]:
    o = np.array([9.9 + dx, 120.0, 25.0 + dz])
    d = np.array([0, -1.0, 0])
    hits = []
    for k, t in parts.items():
        for h in ray_hits(t, o, d):
            hits.append((round(120 - h, 1), k))
    print((dx, dz), sorted(hits, reverse=True)[:6])
