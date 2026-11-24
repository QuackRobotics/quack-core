# Shoulder pen-projector: v1 final proposal (Woz 2026-11-23 18:40 Paris)

Status: PROPOSED, out of TOTAL. Nothing ordered. Frame = upstream robot.xml b23317a `trunk_assembly`, mm.

## Decision asked of Chief: centreline fails, so I propose Luxo's left shoulder
`hardware/tools/projclear.py` does forward kinematics of the whole neck_pitch_assembly chain (neck, head, antennas), with vertices subsampled 1/7, so ±~2 mm. It sweeps the XML joint ranges: neck_pitch −20…65°, head_pitch ±45°, head_yaw ±160°, head_roll ±30°. The tube is a Ø12×50 stand-in, lens face at chest front x = +46 (body_front max x 46.0 ✓ matches Luxo), axis 8° down, beam 312 mm → z = 115.9 (shell top 304 mm = body top z 107.9).

| Placement | Neutral | Any single joint, full range | All joints combined (5⁴ grid) |
|---|---|---|---|
| Centreline y 0 | **−1.6 mm (interference)** | −1.6 mm | −1.6 mm |
| Luxo left shoulder y −45 | **17.3 mm** | 17.3 mm | −5.6 mm at neck 44°, head pitch −45°, yaw −80°, roll +30° |

- Centreline can't move lower either: the neck-pitch servo fills x −11.4…13.4, z 54.9…100.1 at y −18.5…17.2, so a 50 mm tube from x 46 hits it. That leaves ≤ 32 mm of length.
- Left shoulder: 17.3 mm, not Luxo's ~33 mm estimate. The combined-extreme interference needs Atlas: either the motion envelope never reaches that pose, or a soft joint limit is added. Until then it's an open condition.

## Final variant
- 0.5 W class: Cree XP-G3 white on a 10 mm star (fits the Ø10 ID tube), 10 mm optic + gobo (part not chosen), Ø12×50 holster.
- Driver = no IC: GPIO25 → logic-level N-FET low-side switch, LED + **2× 9.1 Ω 1 W 2512 in series** (18.2 Ω) from 5V_AUX. 100 kΩ pull-down on the gate, so it's off at boot and when the Pi is dead.
- Sizing: XP-G3 datasheet gives white Vf @350 mA, 85 °C, typ 2.70 V, max 2.90 V. **No white Vf minimum is published**, so I assume 2.5 V at 150 mA as the low corner (Woz estimate, bench to measure).
  - Max If = (5.16 − 2.5) / 18.2 = 146 mA. Nominal 5.10 V, Vf ~2.6 V → ~137 mA, ≈ 0.36 W in the LED.
  - Resistors in normal running: 0.17 W each. LED-short fault: 5.16 / 18.2 = 0.28 A → 0.73 W each, under 1 W. That's why it's two parts, not one 18 Ω (1.5 W in fault).
- Thermal: the earlier estimate (10:37) gives ≈ 37 °C case at 0.5 W in Ø12×50. That passes my 50 °C limit (1 W ≈ 51 °C fails). XP-G3 Rth j-sp 1.1 °C/W (datasheet), so the junction is < 1 °C over the board.

## 5V_AUX budget (TPS56637: 6 A max continuous, TI product page; bench-validated to 4.9 A, harness_neck_v1)
| Load | Normal (SG90 moving → amp capped −12 dBFS) | Fault / bench |
|---|---|---|
| 2× SG90 | 1.50 A (budget) | 1.50 A |
| MAX98357A (power basis, head_layout_v1) | 0.10 A | 1.51 A (0 dBFS, bench only) |
| Eyes 2× 20 mA | 0.04 A | 0.04 A |
| Projector (trunk side, not in the neck pair) | 0.15 A | 0.28 A (LED short) |
| Jewel 7 (PROPOSED, 0.5 A hold polyfuse) | 0.42 A (7 × 60 mA full white, Adafruit figure, not re-read) | ≈ 1.0 A until PTC trips (generic 2× hold, part not chosen) |
| **AUX total** | **2.21 A** | 3.61 A bench / 2.92 A fault |
| Neck AUX pair (excludes projector), design 2.76 A | 2.06 A ✓ | 2.64 A in Jewel fault ✓ (0.12 A under) |
| **Regulator total incl. 5V_PI 2.16 A** | **4.37 A ✓ (≤ 4.9 bench, ≤ 6 A rating)** | 5.77 A bench case: over the 4.9 A validation. Firmware interlock: no 0 dBFS with an SG90 moving, or extend the bench to 5.8 A |

## Position, mass, distances
- Assembly CoM ≈ tube midpoint **(21.2, −45.0, 119.4)**. Rear end (−3.5, −45, 122.9).
- Mass ESTIMATE ≈ 8 g in the trunk: tube 2.5 g (PLA, 1.2 mm wall, computed), star LED 0.5 g, optic + gobo 1 g, holster 3 g, resistors + wire 1 g. **Head out: 0 g vs the trained head** (the upstream projector parts were never in robot.xml). Robot TOTAL MASS ESTIMATE 2.107 kg (Chief, f698836 bodies) + 0.008 = **2.115 kg**. Whole-robot CoM y shift ≈ −0.17 mm.
- Distance to BNO055 (box x −97.1…−77.1, y ±13.5, z 41.8…44.8): **≈ 106 mm** surface to tube. 0.15 A DC wire field at 0.1 m ≈ 0.3 µT vs ~50 µT Earth field: negligible.
- Distance to the battery cells (x −138.6…−120.6, y ±19.5, z 0…65): **≈ 127 mm**. It sits outside the battery fire-containment bay. Its wiring must not route through the bay.

## Eye safety (IEC 62471)
- Not classified. No exempt / RG1 claim until the beam is measured or assessed to IEC 62471. **Treat as RG2: "do not stare into the beam".**
- Hardware clamp: current fixed by the resistors (≤ 146 mA), so no firmware path can overdrive it. GPIO25 on/off only, no PWM.
- SW clamp (owner Atlas/Page): off at boot, auto-off after 30 s, off on fall / lift / safe stop / PFO, no use when a face is detected within 0.5 m (if vision supports it).
- Beam 8° down from 312 mm meets the floor at ≈ 2.2 m. Floor-level eyes (pets, infants) can still intercept it.

## Links
- [Cree XP-G3 datasheet](https://downloads.cree-led.com/files/ds/x/XLamp-XPG3.pdf)
- [TI TPS56637](https://www.ti.com/product/TPS56637)
- Optic, gobo and star-board part numbers: not chosen yet (no verified 10 mm part).

## Open conditions
1. Atlas: the combined-pose −5.6 mm (envelope or soft limit).
2. Bench: XP-G3 Vf at 150 mA, 25 °C (sets R).
3. Bench: case temperature at 0.5 W ≤ 50 °C.
4. IEC 62471 assessment.

## Collision meshes: upstream b23317a vs Playground (Woz 2026-11-24 11:25 Paris, Chief/Atlas ask)
**Not identical. Keep the sweep on b23317a.**
- Playground commit 968b18de is not a commit in github.com/apirrone/Open_Duck_Playground (git: not a valid object). Compared against f698836, the Playground rev the head-mass work uses.
- b23317a robot.xml: every geom collides (default contype/conaffinity 1). neck_pitch_assembly chain = 31 mesh geoms, incl. the STS3215 cases (wj-wk00-*), horns, Pi Zero, board, 2x SG90, bearings, head_roll_mount. Full-resolution STLs (head 21,634 tris).
- f698836 open_duck_mini_v2.xml (and _backlash.xml): trunk_assembly and head_assembly have **zero collision geoms**; the only collision class geoms in the file are the 2 foot_bottom_tpu. Neck chain = 10 visual meshes, printed parts only, no servos/electronics. Every STL is a 104-triangle proxy; bounding boxes match b23317a within 0.0-2.7 mm (head_pitch_to_yaw worst).
- projclear.py on f698836 meshes: centreline +10.0 mm (vs -1.6 mm on b23317a), left shoulder neutral 20.6 mm (vs 17.3), combined worst -3.8 mm (vs -5.6). Less conservative only because the neck-pitch servo and the electronics are missing. A Playground-only sweep would wrongly pass the centreline.
- Holster CAD: none exists. The Ø12x50 cylinder (lens face x +46, axis 8 deg down, at (46, -45, 115.9)) is the whole envelope; the holster wraps it inside that envelope (3 g budget). The 4 mm clearance table should be built on this cylinder + 1.2 mm print-tolerance margin until a holster is drawn.
