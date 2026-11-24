import numpy as np
import struct
import re
import hashlib
D = '/tmp/odm/print/'

def load(name):
    b = open(D + name + '.stl', 'rb').read()
    h = hashlib.sha256(b).hexdigest()[:16]
    if b[:5] == b'solid' and b'facet' in b[:300]:
        v = re.findall(rb'vertex\s+(\S+)\s+(\S+)\s+(\S+)', b)
        return np.array(v, float).reshape(-1, 3, 3), h
    n = struct.unpack('<I', b[80:84])[0]
    a = np.frombuffer(b[84:84 + n * 50], dtype=np.dtype([('n', '<3f4'), ('v', '<9f4'), ('a', '<u2')]))
    return a['v'].reshape(-1, 3, 3).astype(float), h

def vol(t):
    return np.einsum('ij,ij->i', t[:, 0], np.cross(t[:, 1], t[:, 2])).sum() / 6.0

for name in ['left_eye', 'right_eye', 'bulb']:
    t, h = load(name)
    p = t.reshape(-1, 3)
    lo, hi = p.min(0), p.max(0)
    print(name, h, 'tris', len(t), 'bbox lo', lo.round(2), 'hi', hi.round(2), 'size', (hi - lo).round(2), 'vol cm3', round(abs(vol(t)) / 1000, 3))
    # axis = smallest extent
    ax = int(np.argmin(hi - lo))
    others = [i for i in range(3) if i != ax]
    c = (lo + hi) / 2
    r = np.hypot(p[:, others[0]] - c[others[0]], p[:, others[1]] - c[others[1]])
    z = p[:, ax]
    print('   axis', 'xyz'[ax], 'centre', c.round(2))
    # radial profile: for slices along axis, min and max radius of vertices
    for zz in np.linspace(lo[ax], hi[ax], 9):
        m = np.abs(z - zz) < (hi[ax] - lo[ax]) / 16
        if m.any():
            rr = np.unique(np.round(r[m], 1))
            print('   %s=%.2f  radii %s' % ('xyz'[ax], zz, rr[:3].tolist() + ['..'] + rr[-3:].tolist()))
