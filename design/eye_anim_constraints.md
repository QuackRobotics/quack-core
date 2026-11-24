<!--
SPDX-FileCopyrightText: 2026 Quack Robotics contributors
SPDX-License-Identifier: CC-BY-SA-4.0
-->

# Eye animation constraints (Jewel option, Chief/Atlas 2026-11-23 ~10:20)

Status: Jewel (Adafruit #2226) under scoping; plain Ø5 TH eyes are default until Woz + Atlas sign off.
Write all eye animations so they work unchanged on either.

- eye_update_hz_max = 50 (hard cap; frames rendered on cores 0-2, never core 3)
- eye_current_max_a = 0.25 (both eyes combined; enforced as a HARD clamp in the driver with a unit test, not in
  animations). Uncapped worst case 14 LEDs x 60 mA = 0.84 A.
- Animations must also look right at the clamp: design for brightness budget, not full white.
- Eyes dim during stand-up and low-battery mode (same as projector); parameter eye_dim_states = [standup, low_batt].
- Driver path: SPI on GPIO10 only (PWM/PCM ruled out: antenna servos + I2S audio).
- Level shift: 74AHCT1G125 on 5V_AUX, 330 ohm series, local decoupling.
- Gate: 30 min camera bench must pass with eyes animating at the SPI-required core clock (0 dropped frames, no throttle,
  loop timing unchanged), else plain eyes stay.
- Fallback mapping for plain LEDs: pupil/gaze layers ignored; blink + breathing map to PWM brightness.
