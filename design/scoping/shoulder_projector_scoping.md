<!--
SPDX-FileCopyrightText: 2026 Quack Robotics contributors
SPDX-License-Identifier: CC-BY-SA-4.0
-->
# Shoulder projector — scoping (NOT in v1 GLB; needs Woz + Atlas sign-off)
Luxo, 2026-11-23. Frame: v1.8 neutral GLB, mm, +x fwd, y = height, left = −z. Bounds from `inspect_glb.py`.
Sketch: `design/scoping/shoulder_projector_sketch.svg` (script `sketch_shoulder_projector.py`, all params at top).

## Placement (recommended: left shoulder holster)
- Proxy Ø12 × 50 cylinder, axis fore-aft, lens flush with chest front (x +46), z −45, axis h 312 (shell top 304).
- Clear of neck sheets (z ±23) and shell edge (z ±55). Under the head disc (head 371–430, ±99).
- Head-roll ±30° clearance (bbox estimate): head underside at z 45 drops to ~351 → ~33 mm over the projector top (318).
  Verify in 3D mirror with neck/head pitch combos.
- Occlusion: neck_pitch forward ≳ ~28° brings the head shell in front of the beam (bbox estimate). Rule:
  `proj_max_neck_pitch_deg` ≈ 25 while projecting.
- Rejected: back (beam useless, USB/switch there); chest centre (reads "heart light", cute-for-cute's-sake; adds +x
  CoM).
- Side (left/right) = Atlas's call on lateral CoM; character works either way.

## Aim by body pitch only
## Update 10:33 — Woz tube Ø14×60, trunk top, outboard of neck (still NOT v1 GLB)
- Placement: z −45 (−52…−38): 15 mm outboard of neck sheets, 3 mm inside shell edge ±55; lens x +46 (behind head front x
  101), tail x −14; axis h 312, tube top 319 on shell top 304.
- Head yaw ±160: head underside 371 → ~52 mm free sweep at neutral, no contact at any yaw.
- Head roll (estimate, pivot assumed at head-underside centre): 20° → ~34 mm, 30° → ~26 mm clear. Verify in 3D mirror
  with neck-pitch combos.
- Camera: tube lens is behind and below the camera; CM3 lower FOV edge 5.5° below horizontal forward → tube never in
  frame.
- Beam vs head/beak: upper cone edge at head front (x 101) ≤ ~334 mm even at −2° (lean back) with a 20° half-cone → ≥37
  mm under head shell. Beak below the shell is not in my bbox → mirror check. Occlusion rule proj_max_neck_pitch_deg ≈
  25 stands.
- Ø8 head hole: delete from the print (param proj_hole_enabled=false); don't plug, don't keep as a feature.
- v1 on/off only: light_the_path "dims" → switches off; all three behaviours work on/off.

Params: `proj_mount_tilt_deg` (default 8° down), body pitch envelope from Atlas (±10° assumed here).
Floor spot distance = 312 / tan(tilt + pitch): 8° → 2.2 m; 13° → 1.35 m; 18° → 0.96 m; 3° (lean back) → wall shot.

## Behaviours
1. light_the_path — walking torch, lean = closer spot.
2. show_and_tell — wall gobo, head looks at own image, then at the human.
3. caught_out — startle: lean-back sweeps the spot up, cut LED, sheepish look at the lamp.

## Update 2026-11-24 11:01 — Atlas clearance sweep: interim HAL box (supersedes my bbox clearances above)

While the projector is mounted, the HAL limits are (Atlas, via Chief):
neck_pitch >= -9.4 deg, head_pitch >= -22.5 deg, |head_roll| <= 22.5 deg, head_yaw free.
Until that clamp is in code: no head commands at all with the projector mounted.
A 4 mm clearance table from Woz's holster CAD will replace this box later.
Sign convention of the pitch limits to be confirmed against Atlas's HAL before any choreography is keyed.

Params (no hardcoded limits): proj_box_neck_pitch_min_deg, proj_box_head_pitch_min_deg, proj_box_head_roll_abs_deg.
The occlusion rule proj_max_neck_pitch_deg stays as a separate, beam-side limit.

Behaviour fit:
- light_the_path: aim is body pitch only, so it fits as-is. idle_tired head droop is clamped to the box while mounted.
- show_and_tell: the turn to the image is head yaw (free) plus a beak click, so it fits. Interim (no head commands):
  eyes and beak only.
- caught_out: the glance at the lamp is mostly yaw; pitch and roll accents stay inside the box. Interim: eye glance
  only.

Audio: capped at -12 dBFS while the antennas move (provisional). Chirps and beak clicks that coincide with antenna
motion respect udio_cap_dbfs_antenna_moving (default -12).
