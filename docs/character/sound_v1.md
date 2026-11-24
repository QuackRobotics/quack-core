<!--
SPDX-FileCopyrightText: 2026 Quack Robotics contributors
SPDX-License-Identifier: CC-BY-SA-4.0
-->
# Sound v1 (Luxo, draft v0.2, 2026-11-23)

Companion to `camera_v1_params.md`. Naoufel decisions (via Chief 02:18 and 02:22): v1 has audio, the upstream speaker +
MAX98357A I2S amp in the head (upstream `docs/assembly_guide.md`: LRC GPIO 19, BCLK GPIO 18, DIN GPIO 21, VIN 5 V, GAIN
and SD not connected), and a microphone on the same I2S bus. All values are starting defaults for tuning, not measured.

Hard rules: every clip <= 500 ms, pre-loaded on SD. No hardcoded timings: every offset below is a parameter.

v1 has no beak part (upstream `print/` has none, nor `robot.urdf`). "Beak click" is a sound plus a small head-pitch nod
inside Atlas's overlay; it is not a moving beak.

## File format and bus rate

Mic and amp share one I2S bus (BCLK 18, LRCLK 19), so playback and capture run at one rate.

| Parameter | Default | Meaning |
|---|---|---|
| `i2s_rate_hz` | 48000 | single bus rate for playback and capture, 16-bit |
| `clip_container` | Ogg Opus, mono | files are `<sound_id>.opus` in `/sounds/` on the SD (path is a proposal for Page) |
| `clip_encode_bandwidth` | wideband (16 kHz audio bandwidth), about 24 kbit/s | this is the "16 kHz Opus" hard rule |
| `clip_decode_rate_hz` | 48000 | Opus decoders output 48 kHz natively, so no resampler is needed to feed the bus |

## Power and loudness

The amp shares the 5 V UBEC pair with the two SG90 antennas (Woz BOM line 10, peak 1.26 A per Chief). Peak amp current
scales with signal amplitude, so the ceiling is set in dBFS in the software mixer. dB SPL is not known until measured on
the robot.

| Parameter | Default | Unit | Meaning |
|---|---|---|---|
| `clip_master_peak_dbfs` | -1 | dBFS | every clip is mastered to this true peak |
| `sound_ceiling_dbfs` | -6 | dBFS | mixer ceiling; about 0.5 x full-scale amplitude, so about 0.63 A of the 1.26 A peak (safe-side figure; supply current tracks power, amplitude squared, so nearer 0.32 A, Chief 02:25) |
| `sound_ceiling_antenna_move_dbfs` | -12 | dBFS | ceiling while any SG90 is moving (about 0.32 A) |
| `antenna_sound_stagger_ms` | 60 | ms | a clip never starts in the first N ms of an antenna move (SG90 start current) |
| `sound_duty_max` | 0.2 | fraction | audio on-time over the window below |
| `sound_duty_window_s` | 10 | s | so at most 2 s of audio per 10 s |
| `sound_min_gap_ms` | 300 | ms | silence between two clips |
| `sound_priority` | fall_oops > low_batt > boot > sad_lost > happy > notice_quack > beak_click | order | a higher clip cuts a lower one; a lower one is dropped, never queued |
| `sound_enabled_before_capgood` | false | bool | no audio until the LTC3226 CAPGOOD GPIO is high |

GAIN not connected means the MAX98357A fixed 9 dB gain (datasheet default). If the measured level is too loud at
`sound_ceiling_dbfs`, lower the ceiling first; a hardware gain change is Woz's call.

## Sound set

Times are measured from the trigger event. "Nod" is a head-pitch offset from the policy-held pose
(`head_offset_reference`), gated on `gait.head_overlay_allowed`; if the gate is false the nod is skipped and the sound
still plays.

| id | Length | Trigger | Choreography (all parameters) |
|---|---|---|---|
| `beak_click` | 40 ms | B1 notice | antenna perk starts at 0; click at `click_offset_ms` 80 (perk peak); nod down `click_nod_deg` 2 over `click_nod_ms` 80, then back |
| `notice_quack` | 250 ms | B1 notice, only if changed area >= `quack_area_frac` 0.10, chance `notice_quack_prob` 0.3 | plays when the head turn arrives (`quack_on_turn_arrival` true), with one blink at the same moment; the click still plays first; obeys `notice_cooldown_s` |
| `happy` | 300 ms, rising chirp | B2 face acquired (`face_confirm_frames` met) | blink at 0 (`blink_on_acquire`), chirp at `happy_offset_ms` 120 (eyes reopening); antennas lift `happy_antenna_frac` 0.5 at 120; cooldown `happy_cooldown_s` 20; not replayed when the head re-acquires after a gate drop |
| `sad_lost` | 400 ms, falling two-note | B3 face lost | plays at `lost_glance_hold_ms` end, with the shrug roll start; antennas droop `sad_antenna_droop_frac` 0.6; no blink; cooldown `sad_cooldown_s` 20 |
| `boot` | 500 ms | `boot.ready` (policy loaded, torque on, CAPGOOD high) | left eye on at 0, right eye at `boot_eye_stagger_ms` 100, projector LED flash `boot_proj_flash_ms` 80 at 200; antennas one wiggle at `boot_antenna_offset_ms` 300 |
| `low_batt` | 400 ms, slow descending | `power.low_battery` (event name confirmed; the clip reacts to the event only and never reads a voltage. The threshold is the event field `threshold_v` (per unit; provisional default set by power config, not by this spec); `power.fail` is a separate event) | slow blink `low_batt_blink_ms` 300; antennas droop 0.5; at `sound_ceiling_antenna_move_dbfs`; repeat at most every `low_batt_repeat_s` 120 |
| `fall_oops` | 350 ms | `fall.settled` from Atlas (name confirmed; never during the fall) | plays at `fall_oops_delay_ms` 300 after settle; two quick blinks (`blink_ms` each, gap `fall_blink_gap_ms` 120); then the stumble_recovery shrug; no head offsets until Atlas re-opens the gate |

Worst case burst (B1 click + quack + B2 happy back to back): 40 + 250 + 300 ms = 590 ms of audio, well under the 2 s per
10 s duty cap.

## Listening behaviours (L1-L3)

Inputs: Page's mic API, `level` (RMS dBFS) and `sound_heard` events with `self_generated`. Events with `self_generated`
true never trigger anything. One microphone gives loudness but no direction, so no behaviour here turns the head
"toward" a sound from audio alone; the camera supplies the direction. All head offsets are inside Atlas's cap and the 22
deg `notice_turn_max_deg` (Atlas cap +/-24 deg), and gated on `gait.head_overlay_allowed`.

| id | Trigger | Choreography (all parameters) |
|---|---|---|
| L1 `startle` | `sound_heard` with peak >= noise floor + `startle_margin_db` 25 | at 0: antennas full perk, eye ring to `eye_wide` (brightness +30%, pupils dilated); `head_pitch` +3 (`startle_lift_deg` 3; positive head_pitch raises the nose, axis trunk -y, Atlas 02:29; cap 6.75 deg) over `startle_lift_ms` 60 (freeze-and-lift); hold `startle_hold_ms` 400; no sound; cooldown `startle_cooldown_s` 5; then L3 |
| L2 `listening` | sound present for >= `listen_min_ms` 300 with level >= noise floor + `listen_margin_db` 10 | eye state `listening` (slow brightness breathing, period `listen_breath_ms` 1200, gaze up `listen_gaze_up_frac` 0.3); head roll `head_roll_cock_deg` -4 to put the mic side up (an ear cock; head_roll cap 4.5 deg = 15% of +/-30 deg; -4 raises +y, the duck's left/mic side: axis trunk -x at zero pose, turns with the head so valid at any head_yaw, Atlas 02:29 from upstream MJCF; real servo direction verified by Atlas's 2 deg first-power-on nudge, HAL flips if needed), over `listen_cock_ms` 250; eye `listening` gaze tilt and the mic-side antenna `listen_antenna_mic_side_extra` +0.2 above the other carry the rest of the ear lift; antennas lift 0.3; release `listen_release_ms` 500 after the level drops |
| L3 `look_for_sound` | end of L1 hold, or end of L2 | if B1 sees motion within `look_window_ms` 1000: normal B1 turn (<= 22 deg, eye gaze lead up to 15 deg). Otherwise a scan: yaw to -`look_scan_deg` 20, then +20, `look_scan_leg_ms` 400 each, then back to the reference pose. A face found hands over to B2 (with `happy`). Nothing found ends with a small shrug (0.3 of the roll cap), no sad sound |

If Woz adds a second mic (two I2S MEMS mics can share one data line as left/right channels), Page can add a `direction`
field and L3 becomes a direct turn toward the sound, still <= 22 deg.

### Does the duck mute itself while listening?

| Parameter | Default | Meaning |
|---|---|---|
| `listen_ignore_self` | true | `sound_heard` with `self_generated` true is ignored |
| `self_generated_tail_ms` | 150 | `self_generated` stays true this long after playback ends (speaker and room decay); Page owns the flag |
| `listening_suppress_below` | sad_lost | during L1/L2, clips below sad_lost (happy, notice_quack, beak_click) are suppressed: the duck goes quiet while it listens |
| always played | fall_oops, low_batt, boot | safety and status clips are never suppressed by listening |

### Is level + sound_heard enough?

Yes for L1-L3, with these fields: `level` at >= 10 Hz; `sound_heard` with onset timestamp, peak dBFS, and an end
(duration or `sound_ended`) so L2 can release; a noise floor estimate (or I compute it from `level`). Ask: antenna SG90
and head servo noise will reach the mic, so `self_generated` (or a separate `self_motion` flag) should also be true
while an antenna moves, plus the tail.

## Audio API answers (for Page)

- Play while muted: **202 + `audio.playback` event with state `suppressed`** `audio.playback {playback_id, cue_seq?,
  source: api|loop, sound, state, reason?, mono_ms, boot_id}`; `playback_id` is issued by the audio server only,
  `cue_seq` is the loop counter, a `dropped` cue has `playback_id: null` and only its `cue_seq`. Suppressed reasons:
  `muted`, `listening`, `duty`, `priority`, `capgood`. Interrupted reasons: `stop`, `preempted`, `safe_stop`,
  `power_fail`. Not 409: muted is a normal state, and every suppression reason should reach clients the same way.
- Timestamp sync: **not needed, REST is fine for v1.** Each choreography (clip + eyes + antennas + nod) runs on the
  robot from one trigger, scheduled on 50 Hz loop ticks with the offsets above. REST only fires a named expression or a
  sound id. Please emit `audio.playback` state `started` with `mono_ms` (robot monotonic ms) and `boot_id` (Page's one
  family: states `started`, `finished`, `interrupted`, `suppressed`, `dropped`) for logs and 3D-mirror replay. Revisit
  gRPC only if a remote client must drive frame-by-frame sync; v1 has no beak or lip sync.
- Sound ids: `beak_click`, `notice_quack`, `happy`, `sad_lost`, `boot`, `low_batt`, `fall_oops`. Format: see "File
  format and bus rate".

## Speaker grille position

Keep the upstream grille; do not put one on the face. Upstream `head.stl` already has a perforated side grille in front
of `speaker_interface.stl` / `speaker_stand.stl` (ray-cast at 0.5 mm, Luxo 2026-11-23 02:22):

| | Print frame (mm) | head_assembly (mm, +x up, -z forward) |
|---|---|---|
| Grille open area | x 19…54, z 121…157, side wall y about -79 (-88…-71) | x -8…+28, y about -79, z +7…+42 |
| Centre | (36.6, -79, 139.2) | (10.1, -79, 24.4) |
| Holes | about 134 holes of about 1 mm2 at this sampling, about 115 mm2 open | same |

That is the duck's right side (+y is the duck's left in this frame), 66-101 mm behind the front face, at eye height. It
reads as an ear and leaves the front column (projector LED, eyes, camera window) alone. The exact hole diameter is not
confirmed at 0.5 mm sampling.

## Mic port position (proposal for Woz)

No mic placement from Woz exists yet in `~/quack-core` (grep, 02:23), and upstream has no mic part. Proposal: the
opposite side wall, mirroring the grille, as the duck's other "ear".

| | Print frame (mm) | head_assembly (mm) |
|---|---|---|
| Mic port centre | (36, 79.2, 139) | (9.9, +79.2, 25.0) |
| Port | through-hole `mic_port_d_mm` 1.0, MEMS mic sealed to the wall with a gasket so sound inside the head does not leak in | same |

The left side wall there is solid (ray-cast at 0.5 mm, no openings in x 14…62, z 118…162; wall y 71…88, sloped). The
port is the full head width (about 158 mm) from the speaker grille, 66 mm or more behind the face, so it stays clear of
the projector/eyes/camera column and reads as nothing on the face. Woz to confirm the inside is clear of the head servo
and the antenna SG90s, which are the main noise sources for the mic.
