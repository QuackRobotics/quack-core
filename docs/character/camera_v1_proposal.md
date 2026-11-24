<!--
SPDX-FileCopyrightText: 2026 Quack Robotics contributors
SPDX-License-Identifier: CC-BY-SA-4.0
-->
# Camera in v1 — character placement + 3 behaviours (Luxo, draft v0.1, 2026-11-23)

Status: proposal to Quack Chief. GLB unchanged until Woz sends mount dimensions.

## 1. Lens placement
- Where: face centreline, on the beak bridge just above the beak root, between and slightly below the two LED eyes.
  Reads as a nostril / beak ridge, not a third eye.
- Bezel: recessed, matte, same colour as `servo_dark`; no ring, no shine. Lens set back so it does not catch a highlight
  in renders.
- Tilt: pitched up in the head frame (start value 15°, parameter `cam_pitch_offset_deg`, final from Woz). At ~400 mm
  lens height an adult face at 1.5 m is ~39° above horizontal; the up-tilt keeps faces in frame without big neck pitch.
- Not chosen: forehead (reads as third eye), inside an eye (breaks LED eyes, Chief ruled them the visible eyes), inside
  the beak (beak motion would block it).
- Camera moves with head_yaw/head_pitch/head_roll, so every pixel target must be converted to body frame using the
  current head joint angles.

## 2. Behaviours (Pi Zero 2W budget)
Pipeline: 320x240 grey, ~10 fps target (to benchmark), frame differencing for motion, small face detector (OpenCV Haar
or YuNet) at 160x120. Output per frame: `{motion_centroid, face_bbox, face_conf}`. No frames stored or streamed by
default.
All timings/gains are personality parameters; head offsets clamped to <=15% of joint range; head moves only when Atlas
reports standing/stance (gait hook), during walking only eyes + antennas react.

### B1 notice (someone appears)
Trigger: motion area > `notice_threshold` after `quiet_time_s` of stillness.
Beat: blink -> antennas perk up together -> beak click (<=500 ms sound) -> small counter-yaw (anticipation) -> head_yaw
toward centroid. Cooldown `notice_cooldown_s`.

### B2 look-at-person
Trigger: face found with conf > `face_conf_min`.
Head leads and the eyes blink once on acquire (v1 eyes are single on/off LEDs, no gaze); head_yaw/head_pitch follow with
a damped spring (`blink_on_acquire`, `head_follow_freq_hz`), deadband so it does not jitter. Never chases faster than
`head_max_rate_deg_s`.

### B3 antenna follow + lost face
While tracking: antennas tilt asymmetrically toward the face side (`antenna_follow_gain`), like ears.
Face lost > `lost_face_s`: one search glance at last-known side, then a small shrug (antennas droop, head roll tick) and
back to idle_curious.

## 3. Open dependencies
- Woz: lens/board mount dimensions, exact tilt, cable route (Pi Zero 2W uses the 22-pin CSI connector).
- Atlas: standing/stance flag from gait so head offsets never run against balance.
- Pin check: antennas use hardware PWM on GPIO12/13; the WS2812 ring should then go on SPI (GPIO10), not PWM, and audio
  must not need PWM either.
