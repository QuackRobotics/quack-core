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
- Head-roll ±30° clearance (bbox estimate): head underside at z 45 drops to ~351 → ~33 mm over the projector top
  (318). Verify in 3D mirror with neck/head pitch combos.
- Occlusion: neck_pitch forward ≳ ~28° brings the head shell in front of the beam (bbox estimate). Rule:
  `proj_max_neck_pitch_deg` ≈ 25 while projecting.
- Rejected: back (beam useless, USB/switch there); chest centre (reads "heart light", cute-for-cute's-sake; adds +x
  CoM).
- Side (left/right) = Atlas's call on lateral CoM; character works either way.

## Aim by body pitch only

## Update 10:33 — Woz tube Ø14×60, trunk top, outboard of neck (still NOT v1 GLB)

- Placement: z −45 (−52…−38): 15 mm outboard of neck sheets, 3 mm inside shell edge ±55; lens x +46 (behind
  head front x 101), tail x −14; axis h 312, tube top 319 on shell top 304.
- Head yaw ±160: head underside 371 → ~52 mm free sweep at neutral, no contact at any yaw.
- Head roll (estimate, pivot assumed at head-underside centre): 20° → ~34 mm, 30° → ~26 mm clear. Verify in 3D
  mirror with neck-pitch combos.
- Camera: tube lens is behind and below the camera; CM3 lower FOV edge 5.5° below horizontal forward → tube never in
  frame.
- Beam vs head/beak: upper cone edge at head front (x 101) ≤ ~334 mm even at −2° (lean back) with a 20° half-cone
  → ≥37 mm under head shell. Beak below the shell is not in my bbox → mirror check. Occlusion rule
  proj_max_neck_pitch_deg ≈ 25 stands.
- Ø8 head hole: delete from the print (param proj_hole_enabled=false); don't plug, don't keep as a feature.
- v1 on/off only: light_the_path "dims" → switches off; all three behaviours work on/off.

Params: `proj_mount_tilt_deg` (default 8° down), body pitch envelope from Atlas (±10° assumed here).
Floor spot distance = 312 / tan(tilt + pitch): 8° → 2.2 m; 13° → 1.35 m; 18° → 0.96 m; 3° (lean back) → wall
shot.

## Behaviours

1. light_the_path — walking torch, lean = closer spot.
2. show_and_tell — wall gobo, head looks at own image, then at the human.
3. caught_out — startle: lean-back sweeps the spot up, cut LED, sheepish look at the lamp.

## Update 2026-11-24 11:01 — Atlas clearance sweep: interim HAL box (supersedes my bbox clearances above)

Superseded by the 11:14 section below, which is itself on hold (see its note).

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
motion respect `audio_cap_dbfs_antenna_moving` (default -12).

## Update 2026-11-24 11:14 — interim head envelope v2 (Atlas b23317a; supersedes the 11:01 box)

**On hold pending Atlas table (2026-11-24 11:46).** Chief withdrew this box: Atlas's all-vertex recheck clears only 1.95
mm in the worst pose, under the 4 mm condition. Until the new table lands, plan no head movement while the projector is
mounted (eyes, sound and antennas only). This section will be rewritten once against that table; do not choreograph
against the numbers below.

Atlas swept 36,855 poses (minimum clearance 4.5 mm). Interim limits while the projector is mounted:
neck_pitch >= 0 deg, head_pitch >= -22.5 deg, |head_roll| <= 22.5 deg, head_yaw free.
Change from the 11:01 box: proj_box_neck_pitch_min_deg goes from -9.4 to 0 (no neck pitch below zero at all).
Until the HAL clamp lands, head offsets are rejected while the projector is mounted. Eyes, sound and antennas still
animate (antennas are not head joints). There is no beak joint: "beak click" here means a sound clip only.

The final HAL clearance table will be tighter. Expected rejections: head_yaw near +35 deg or near -115 deg combined
with full head pitch and roll. Choreography rule while mounted: no move is keyed into those corners. Near those yaw
angles, pitch and roll accents stay small; the corner keep-out is a parameter set
(proj_corner_yaw_deg = [+35, -115], proj_corner_yaw_halfwidth_deg, proj_corner_pitch_roll_scale), replaced by the
HAL table when it lands.

Behaviour fit with neck_pitch >= 0:

- light_the_path: still body pitch only. idle_tired droop comes from head_pitch (down to the -22.5 deg limit) and
  eyelid-style blinks, never from neck pitch.
- show_and_tell: yaw turn to the image; if the image sits near +35 deg or -115 deg yaw, the turn is pure yaw, no tilt.
- caught_out: the /expression part is yaw plus a small roll only.

Body moves are not expressions: /expression has no body joints. The light_the_path aim and the caught_out
lean-back are separate POST /state behaviours that go through the balance controller and need Atlas's balance ack
before they are keyed.

