"""Bounding boxes of trunk_assembly geoms (upstream robot.xml b23317a) in the trunk_assembly frame, mm.
Usage: python3 trunkbox.py <robot_dir>. Reads geom pos/quat, transforms STL vertices (binary/ascii)."""
import sys, re, numpy as np
import xml.etree.ElementTree as ET
sys.path.insert(0, __file__.rsplit('/', 1)[0])
_argv, sys.argv = sys.argv, sys.argv[:1]  # stlvol runs its CLI loop on import
from stlvol import load
sys.argv = _argv

def R(q):
    w, x, y, z = q
    return np.array([[1-2*(y*y+z*z), 2*(x*y-w*z), 2*(x*z+w*y)],
                     [2*(x*y+w*z), 1-2*(x*x+z*z), 2*(y*z-w*x)],
                     [2*(x*z-w*y), 2*(y*z+w*x), 1-2*(x*x+y*y)]])

d = sys.argv[1]
root = ET.parse(d + '/robot.xml').getroot()
body = [b for b in root.iter('body') if b.get('name') == 'trunk_assembly'][0]
out = {}
for i, g in enumerate(body.findall('geom')):
    m = g.get('mesh')
    if not m:
        continue
    p = np.array([float(v) for v in g.get('pos', '0 0 0').split()])
    q = np.array([float(v) for v in g.get('quat', '1 0 0 0').split()])
    t, _ = load(f'{d}/{m}.stl')
    v = t.reshape(-1, 3) @ R(q).T + p
    lo, hi = v.min(0) * 1000, v.max(0) * 1000
    key = f'{m}#{i}'
    out[key] = (lo, hi)
    print(f'{key:40s} lo {lo.round(1)} hi {hi.round(1)}')
