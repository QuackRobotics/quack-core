"""Fast printed-part mass bound: divergence volume + watertight check only (no voxel cross-check).
Usage: python3 stlmass.py file.stl ...   Units mm. Mass = solid PLA 1.24 g/cm3 (upper bound)
and a 4-wall/20%-infill estimate using shell area x 1.6 mm (Woz rule of thumb, not weighed)."""
import sys
import numpy as np
sys.path.insert(0, __file__.rsplit('/', 1)[0])
_argv, sys.argv = sys.argv, sys.argv[:1]  # stlvol runs its CLI loop on import
from stlvol import load, watertight
sys.argv = _argv

RHO = 1.24  # g/cm3 PLA
for f in sys.argv[1:]:
    t, h = load(f)
    a, b, c = t[:, 0], t[:, 1], t[:, 2]
    vol = abs(np.einsum('ij,ij->i', a, np.cross(b, c)).sum() / 6) / 1000  # cm3
    area = np.linalg.norm(np.cross(b - a, c - a), axis=1).sum() / 2 / 100  # cm2
    shell = min(vol, area * 0.16)  # 1.6 mm walls
    infill = shell + 0.2 * max(vol - shell, 0)
    ext = t.reshape(-1, 3).max(0) - t.reshape(-1, 3).min(0)
    print(f"{f.split('/')[-1]}: sha {h[:12]} open_edges {watertight(t)} bbox {ext.round(1)} mm "
          f"vol {vol:.3f} cm3 area {area:.1f} cm2 solid {vol*RHO:.2f} g est20% {infill*RHO:.2f} g")
