<!--
SPDX-FileCopyrightText: 2026 Quack Robotics contributors
SPDX-License-Identifier: CC-BY-SA-4.0
-->
# Camera v1 behaviour parameters (Luxo, draft v0.1, 2026-11-23)

Companion to `camera_v1_proposal.md` (accepted as draft by Chief 01:08). All values are starting defaults for tuning,
not measured. Every head/antenna offset is clamped to <= 15% of that joint's MJCF range (`expr_offset_max_frac`), and
head offsets apply only while Atlas reports standing.

Eye hardware correction (Chief 01:08, checked in upstream `docs/assembly_guide.md` lines 194-196): v1 eyes are single
LEDs on GPIO 23 (left), 24 (right), 25 (projector). They cannot show gaze direction, so B2 no longer has "LED gaze
leads". The head leads instead, and the eyes only blink (on/off). No colour or addressable eye is assumed.

## Shared

| Parameter | Default | Unit | Meaning |
|---|---|---|---|
| `cam_width` x `cam_height` | 320 x 240 | px | greyscale capture |
| `cam_fps_target` | 10 | fps | to confirm on Pi Zero 2W |
| `face_det_width` x `face_det_height` | 160 x 120 | px | detector input |
| `cam_pitch_offset_deg` | 15 | deg | lens up-tilt in head frame; Woz sets final |
| `expr_offset_max_frac` | 0.15 | of joint range | hard cap, all expressive offsets |
| `head_gate_signal` | `gait.head_overlay_allowed` | bool (Atlas, default false) | head offsets only while true; Atlas recentres head within 200 ms when it drops. Atlas overlay caps (15% range, rate) win over any value here |
| `head_offset_reference` | policy-held head pose | frame | offsets are added on top of the head pose the walk policy holds (~0.2 rad off home), measured from that pose, not from home (Chief 01:13) |
| `blink_ms` | 120 | ms | eye LEDs off time for one blink |

## B1 notice someone appearing

| Parameter | Default | Unit | Meaning |
|---|---|---|---|
| `motion_pixel_delta` | 25 | grey levels (0-255) | per-pixel change counted as motion |
| `motion_area_min_frac` | 0.02 | of frame | changed area needed to trigger |
| `stillness_before_notice_s` | 3.0 | s | scene must be still this long first |
| `notice_cooldown_s` | 8.0 | s | no re-trigger within this window |
| `anticipation_counter_yaw_frac` | 0.2 | of turn size | small opposite yaw before the turn |
| `anticipation_ms` | 150 | ms | counter-yaw hold |
| `anticipation_counter_yaw_max_deg` | 6 | deg | cap on the counter-yaw; 0.2 x a 48 deg turn would be ~9.6 deg and read as a flinch (Chief approved 02:16) |
| `notice_turn_max_deg` | 22 | deg | cap on the B1 head turn toward motion, 2 deg inside Atlas's +/-24 deg yaw overlay cap (15% of +/-160, around the held pose); beyond it, eye gaze lead + antenna perk + beak click (Chief approved 02:28) |
| `notice_gaze_lead_max_deg` | 15 | deg | eye gaze may lead up to this far beyond the head yaw on a notice turn (Chief approved 02:28) |
| `antenna_perk_frac` | 1.0 | of antenna cap | both antennas up |
| `notice_sound_id` | `beak_click` | id | <= 500 ms, 16 kHz Opus, on SD |

## B2 look at a person

| Parameter | Default | Unit | Meaning |
|---|---|---|---|
| `face_conf_min` | 0.6 | 0-1 | detector score to accept a face |
| `face_confirm_frames` | 2 | frames | consecutive hits before acting |
| `head_deadband_deg` | 4 | deg | ignore target error below this |
| `head_max_rate_deg_s` | 90 | deg/s | rate cap on head yaw/pitch offset |
| `head_follow_freq_hz` | 2.0 | Hz | damped-spring natural frequency |
| `head_follow_damping` | 0.8 | ratio | spring damping (no overshoot wobble) |
| `blink_on_acquire` | true | bool | one blink when a face is first locked |

## B3 antennas follow, losing the face

| Parameter | Default | Unit | Meaning |
|---|---|---|---|
| `antenna_follow_gain` | 0.5 | antenna deg per deg yaw error | ear-like lean toward the face |
| `antenna_asym` | 0.6 | ratio | far antenna moves this fraction of near one |
| `lost_face_timeout_s` | 1.5 | s | no face this long = lost |
| `lost_glance_hold_ms` | 600 | ms | one look toward last seen position |
| `shrug_antenna_droop_frac` | 0.5 | of antenna cap | droop on giving up |
| `shrug_head_roll_frac` | 0.3 | of head-roll cap | small roll tick |
| `shrug_ms` | 400 | ms | shrug duration, then back to `idle_curious` |

## Interruption (gate drops mid-move)

B1 and B2 must be cuttable at any frame without looking broken. When `gait.head_overlay_allowed` goes false: the head
layer stops issuing offsets at once and Atlas owns the return to centre (<= 200 ms); no Luxo head keyframe fights it.
Antennas and eye blink keep playing (they are not gated), so the beat still reads: in B1 the antenna perk and beak click
finish, in B2 the antennas keep their lean toward the face. Any pending anticipation counter-yaw is dropped, not queued.
When the gate returns true, B2 re-acquires from the current head pose (no snap to the old target) and the B1 cooldown
keeps running.

| Parameter | Default | Unit | Meaning |
|---|---|---|---|
| `reacquire_after_gate_ms` | 250 | ms | wait after the gate returns before B2 drives the head again |
| `antennas_continue_on_gate_drop` | true | bool | antennas/blink finish the beat while the head recentres |

## Camera window v2 (accepted: Chief 2026-11-23 ~10:12, Woz building to it 10:22)

Replaces the v1 window (Ø10 bezel, lens opening ≤ Ø4, lens 2 mm recessed). Reason: BOM line 17 / Woz b9e0bca, CM3 lens
barrel Ø5.75.
Frame: head_assembly / URDF head link, mm, +x up, −z forward. Front face z = −59.05.
Assumes CM3 standard lens (66° H, 41° V, 75° diag). The Wide variant (120° diag) needs a different window.

| param | value | note |
|---|---|---|
| cam_bore_d_mm | 6.0 | Woz flush option; 0.125 mm radial clearance on the Ø5.75 barrel |
| cam_axis_tilt_up_deg | 15 | unchanged; bore is coaxial with the lens axis |
| cam_lens_tip_centre | (0, 0, −58.31) | centre 0.74 mm behind the face, so the tilted barrel rim is flush at the bottom (x −2.78, z −59.05) and 1.49 mm deep at the top (x +2.78) |
| cam_spotface | ⟂ lens axis, Ø6.0 | barrel tip flush with this local face |
| cam_relief_half_angle_deg | 45 | matte black countersink from the Ø6.0 rim out to the face; clears the 37.5° half-diagonal FOV by 7.5° |
| cam_visible_opening_mm | ≈ 6.0 wide × 7.5 tall | the top relief reaches x ≈ +4.5; smaller than the v1 Ø10 bezel |
| cam_finish | matte black, servo_dark | no gloss, no bright rim |

Why not a plain Ø6.0 bore through the vertical face: the lens is tilted 15°, so the barrel top would sit 1.49 mm deep.
With 0.125 mm of radial clearance, any recess deeper than about 0.16 mm vignettes the diagonal corners (tan 37.5° =
0.77).
Check of Woz's 10.4 option: 5.75 + 2·3·tan 37.5° = 10.35 mm, so it agrees with the 75° diagonal.

Beak keep-out x ≤ −2.9 − 0.096·d: still holds. The bottom barrel edge is at x −2.78 at the face, so the line has 0.12 mm
of margin. v1 has no beak, so nothing crosses it.
Head CoM: the window centre stays at x = 0 (inside x 0…−2) around the camera mount point (1, 0, −51); the ~0.06 g change
leaves the head CoM at about (8.29, −1.20, 21.20) (Chief, 09:53).

## Camera window v2.1 (Chief 2026-11-24 10:58)

After Woz signed the camera: Pi Camera Module 3 SC1223, centreline, connector edge up, 4x M2.

| param | value | note |
|---|---|---|
| proj_hole_enabled | false | delete the upstream Ø8 face hole at link (24.0, 0); projector moved to the left shoulder. Deleted, not plugged (no extra part or glue seam). |
| beak_top_max_x_at_face_mm | −2.9 | beak top stays at or below x −2.9 at the face so it stays out of the camera view (keep-out line above: x ≤ −2.9 − 0.096·d). |
