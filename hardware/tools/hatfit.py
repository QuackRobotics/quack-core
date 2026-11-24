import numpy as np
import struct
import re
D = '/tmp/odm/mini_bdx/robots/open_duck_mini_v2/'

def load(name):
    b = open(D + name + '.stl', 'rb').read()
    if b[:5] == b'solid' and b'facet' in b[:300]:
        v = re.findall(rb'vertex\s+(\S+)\s+(\S+)\s+(\S+)', b)
        return np.array(v, float).reshape(-1, 3, 3)
    n = struct.unpack('<I', b[80:84])[0]
    a = np.frombuffer(b[84:84 + n * 50], dtype=np.dtype([('n', '<3f4'), ('v', '<9f4'), ('a', '<u2')]))
    return a['v'].reshape(-1, 3, 3).astype(float)

def qmat(w, x, y, z):
    return np.array([[1-2*(y*y+z*z), 2*(x*y-z*w), 2*(x*z+y*w)],
                     [2*(x*y+z*w), 1-2*(x*x+z*z), 2*(y*z-x*w)],
                     [2*(x*z-y*w), 2*(y*z+x*w), 1-2*(x*x+y*y)]])

def place(name, pos, quat):
    t = load(name)
    if np.abs(t).max() > 5:  # mm -> m guard
        t = t / 1000.0
    R = qmat(*quat)
    return (t.reshape(-1, 3) @ R.T + np.array(pos)).reshape(-1, 3, 3) * 1000.0  # mm, head_assembly frame

parts = {
    'head': place('head', (-0.12906, 0, 0.06095), (0.707107, 0, 0.707107, 0)),
    'head_bot_sheet': place('head_bot_sheet', (-0.12906, 0, 0.06095), (0.707107, 0, 0.707107, 0)),
    'head_roll_mount': place('head_roll_mount', (-0.12906, 0, 0.06095), (0.707107, 0, 0.707107, 0)),
    'pi': place('raspberrypizerow', (0.03205, 0.048, 0.00595), (0.5, 0.5, -0.5, 0.5)),
    'board': place('board', (0.03205, -0.034, -0.00555001), (0.5, -0.5, 0.5, -0.5)),
    'servo_case': np.concatenate([place(m, (0.0255, 0, -0.00945), (0.707107, 0.707107, 0, 0)) for m in
                                  ['wj-wk00-0123middlecase_56', 'wj-wk00-0124bottomcase_45', 'wj-wk00-0122topcabinetcase_95']]),
    'sg90_L': place('sg90', (-0.00624594, 0.0331408, 0.102812), (0.488321, -0.47889, -0.372991, -0.626964)),
    'sg90_R': place('sg90', (-0.00624594, -0.0331408, 0.102812), (0.488321, 0.47889, -0.372991, 0.626964)),
}
for k, t in parts.items():
    p = t.reshape(-1, 3)
    print(f'{k:16s} min {np.round(p.min(0),1)} max {np.round(p.max(0),1)}')

pi = parts['pi'].reshape(-1, 3)
lo, hi = pi.min(0), pi.max(0)
ext = hi - lo
ax = int(np.argmin(ext))
print('Pi thickness axis', 'xyz'[ax], 'extent', np.round(ext, 1))
# header side: the Pi PCB is ~1 mm; the 40-pin header footprint is along one long edge.
# histogram of vertices along thickness axis to see which side carries tall parts
h, e = np.histogram(pi[:, ax], bins=12)
print('vertex histogram along', 'xyz'[ax], list(zip(np.round(e[:-1], 1), h)))

def ray_hits(tris, o, d):
    v0, v1, v2 = tris[:, 0], tris[:, 1], tris[:, 2]
    e1, e2 = v1 - v0, v2 - v0
    p = np.cross(d, e2)
    det = (e1 * p).sum(1)
    ok = np.abs(det) > 1e-9
    inv = np.where(ok, 1 / np.where(ok, det, 1), 0)
    s = o - v0
    u = (s * p).sum(1) * inv
    q = np.cross(s, e1)
    v = (q * d).sum(1) * inv
    t = (q * e2).sum(1) * inv
    m = ok & (u >= 0) & (v >= 0) & (u + v <= 1) & (t > 1e-6)
    return t[m]

others = {k: v for k, v in parts.items() if k != 'pi'}
axes = [a for a in range(3) if a != ax]
for sgn, label in [(-1, '-'), (1, '+')]:
    d = np.zeros(3)
    d[ax] = sgn
    face = lo[ax] if sgn < 0 else hi[ax]
    res = {}
    for a in np.linspace(lo[axes[0]] + 2, hi[axes[0]] - 2, 9):
        for b in np.linspace(lo[axes[1]] + 2, hi[axes[1]] - 2, 5):
            o = np.zeros(3)
            o[ax] = face
            o[axes[0]] = a
            o[axes[1]] = b
            for k, t in others.items():
                hts = ray_hits(t, o, d)
                if hts.size:
                    res.setdefault(k, []).append(hts.min())
    print(f'Stack direction {label}{"xyz"[ax]} from Pi face at {face:.1f} mm:')
    for k, v in sorted(res.items(), key=lambda kv: min(kv[1])):
        print(f'   {k:16s} min clearance {min(v):6.1f} mm  (rays hit {len(v)}/45, median {np.median(v):.1f})')
