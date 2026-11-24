<!--
SPDX-FileCopyrightText: 2026 Quack Robotics contributors
SPDX-License-Identifier: CC-BY-SA-4.0
-->

# Eye drive v1 (Luxo, draft v0.1, 2026-11-24)

Companion to `design/eye_anim_constraints.md` (Jewel option) and `sound_v1.md`. Written for Woz to pick the eye LED
driver on the HAT (netlist `c3e0b60`, eye LEDs on GPIO23/24). All values are starting requirements, not measurements.

## Plain-LED drive (v1)

v1 eyes are the default from `design/eye_anim_constraints.md`: plain Ø5 through-hole LEDs, unless Woz and Atlas sign off
on the Jewel.

| Parameter | Value | Meaning |
|---|---|---|
| `eye_led_count` | 2 (one per eye) | matches upstream Open Duck Mini wiring: anodes on header pins 16 and 18 (GPIO23/24), upstream `docs/assembly_guide.md` |
| `eye_led_colour` | single colour | no RGB in v1 |
| `eye_update_hz_max` | 50 | hard cap, same as `eye_anim_constraints.md`; frames rendered on cores 0-2, never core 3 |
| `eye_v1_modes` | `on`, `off`, `blink` | the v1 `EyeState` in quack-api `expression-0.2.0.yaml`; brightness is reserved for v1.1 and rejected in v1 |
| `blink_closed_ms` | from the request | a blink is a timed off-then-on on the 20 ms tick, so v1 needs no PWM |

### Required from the hardware now (used from v1.1)

Breathing, fades and dimming (`eye_dim_states = [standup, low_batt]`) need PWM brightness. The driver Woz picks must
support it even though v1 software only switches on/off.

| Parameter | Minimum | Why |
|---|---|---|
| `eye_pwm_levels_min` | 64 visible, gamma-corrected levels | fewer and slow fades step visibly at the dark end |
| `eye_pwm_bits_min` | 8 bits duty (10 if free) | needed to get 64 gamma-corrected levels |
| `eye_pwm_freq_hz_min` | 2000 | head camera must not see banding or flicker |
| `eye_brightness_update_hz_max` | 50 | same cap as the frames |

### Drive circuit: pending Woz

The driver and circuit are not chosen yet. Woz picks them within these constraints; this section is updated when he does.

- GPIO23/24 are not hardware-PWM pins on the Pi.
- The PWM and PCM peripherals are ruled out (antenna servos and I2S audio use them), and so is DMA-timed software PWM
  that borrows them.
- Acceptable v1.1 sources: a small I2C PWM driver, or GPIO23/24 driving a transistor if Woz accepts a CPU-timed PWM
  thread on cores 0-2.
- If the LEDs hang directly off the GPIOs, keep each well under the pin's drive limit with a sized series resistor; use
  a transistor for more brightness.

### Chosen eye LED and diffuser (Woz HAT PR #8, `f57293f`)

Woz picked the LED; Luxo accepted it on 2026-11-24. The diffuser and light tube below are design only.

| Parameter | Value | Meaning |
|---|---|---|
| `eye_led_part` | Cree C513A-MSN-CW0Z0231 | datasheet CLD-CT1042 rev12 (`/home/atlas/shared/datasheets/`) |
| `eye_led_cct_k` | 2800 typ, 2500 min | warm white; accepted in place of the ~3000 K first asked for |
| `eye_led_lens` | 55° (2θ½), clear | not diffused, so the diffuser lives in the eye shell |
| `eye_led_series_ohm` | 470 | 2.7-5.9 mA (4.8 typ) on normal power, 1.0-4.6 mA on backup; low backup brightness is accepted |
| `eye_led_pairing_v` | 0.1 | both eyes in a head are paired within 0.1 V Vf at kitting |
| `eye_diffuser_position` | one disc per Ø25 eye hole, front face set back 0.5 mm | head link z −59.05, centred on (10.34, ±40) |
| `eye_diffuser_proto` | natural or white PETG, 1.0 mm, 100% infill, concentric | prototype |
| `eye_diffuser_final` | opal (white translucent) cast PMMA, 2 mm, laser-cut | final |
| `eye_diffuser_thickness_mm` | to be picked by eye from a 0.8 / 1.0 / 1.2 mm test print | no transmission figures are known for these materials |
| `eye_led_to_diffuser_mm` | 10-12, LED dome tip to disc back, on axis | **pending Woz fit/mass check**; gives a ~Ø10 bright core fading to the rim |
| `eye_light_tube` | opaque tube LED to disc, white inside, black outside | **pending Woz fit/mass check**; keeps light out of the other eye, the shell and the camera/projector paths |

- Brightness: at 4.8 mA the LED gives roughly a quarter of its 3000-12000 mcd (20 mA) rating. This is a linear-scaling
  estimate, not checked against the datasheet's current curve. The diffuser is there to stop glare; if the eye is still
  too bright, use a thicker disc, not a different resistor.
- Both eyes use the same disc and the same depth, so the 0.1 V pairing still gives matching eyes.

### If the Jewel is approved later

The Jewel does not use GPIO23/24: SPI on GPIO10 through a 74AHCT1G125 on 5V_AUX, 330 Ω series resistor, local
decoupling, both eyes capped at 0.25 A by a hard driver clamp (see `design/eye_anim_constraints.md`). The netlist would
have to change.
