"""Identify upstream 'board' mesh: Onshape part metadata + mesh extents vs other electronics."""
import json
import glob
import re
import struct
import os
import numpy as np
D = '/tmp/odm/mini_bdx/robots/open_duck_mini_v2/'

def load(f):
    b = open(f, 'rb').read()
    n = struct.unpack('<I', b[80:84])[0]
    if len(b) != 84 + 50 * n:
        return np.array(re.findall(rb'vertex\s+(\S+)\s+(\S+)\s+(\S+)', b), float).reshape(-1, 3, 3)
    a = np.frombuffer(b[84:84 + n * 50], dtype=np.dtype([('n', '<3f4'), ('v', '<9f4'), ('a', '<u2')]))
    return a['v'].reshape(-1, 3, 3).astype(float)

for f in sorted(glob.glob(D + '*.part')):
    d = json.load(open(f))
    print(f"{os.path.basename(f)[:-5]:32s} doc {d['documentId'][:8]} el {d['elementId'][:8]} '{d['name']}'")
print()
for f in ['board', 'bms', 'bno055', 'raspberrypizerow', 'usb_c_charger', 'power_switch']:
    t = load(D + f + '.stl')
    v = t.reshape(-1, 3)
    vol = abs(np.einsum('ij,ij->i', t[:, 0], np.cross(t[:, 1], t[:, 2])).sum() / 6)
    print(f"{f:18s} extents {np.round(np.sort(np.ptp(v, 0)), 4)} vol {vol:.3e}")
