# Head grams gate: best-case cuts (Woz 2026-11-23 18:40 Paris)

Status: PROPOSED. Every mass here is a Woz ESTIMATE unless it says "STL volume" (solid PLA 1.24 g/cm³, an upper bound). Nothing is weighed: no parts in hand (Naoufel 18:02).

## Gate input (Hinton 12:50, sim only, via Chief)

- Trained head (Playground `head_assembly` @ f698836): 406.6 g, CoM (8.15, −3.91, 22.77) mm.
- 434.6 g and 438.6 g heads (+28 / +32 g) fail the walk gate: vx ≈ 0.08 vs bar ≥ 0.0851, heading +52–58° (16–36° more left drift than control). Same with a point mass. Stand-up passes.
- Pass threshold between +0 and +28 g is unknown. Hinton needs rows at about +8, +13 and +20 g to find it.

## Measured inputs (upstream b23317a `print/`, `hardware/tools/stlmass.py`)

| Part (expression package) | STL volume | Solid PLA |
|---|---|---|
| bulb.stl (×2) | 1.695 cm³ each | 2.10 g each |
| left_eye.stl | 1.428 cm³ | 1.77 g |
| right_eye.stl | 4.629 cm³ | 5.74 g |
| speaker_stand.stl | 6.986 cm³ | 8.66 g |
| speaker_interface.stl | 1.804 cm³ | 2.24 g |
| Printed subtotal | | 22.61 g |
| Rest of the 45.6 g (cam, glass ×4, full_speaker) | | ≈ 23.0 g by difference, not itemised in the XML |

Upstream projector parts (`flash_light_module` 15.52 g, `flash_reflector_interface` 2.74 g, STL volume solid) are **not** in robot.xml (0 matches), so they were never in any trained head. Moving the projector to the shoulder keeps ≈ 18 g (upper bound) out of the head. That's why I don't count it as a cut.

## Cut list, against the current 438.6 g estimate

| # | Cut | Δ head | Confidence | Cost / owner |
|---|---|---|---|---|
| 1 | Right eye = mirrored left_eye.stl (already in the 434.6 g row if Hinton used it) | −3.97 g | STL volume | Luxo look call |
| 2 | 2× BCAP0010 supercaps from the head HAT to the trunk board | −6.2 g | catalogue mass from the 02:45 table | Hold-up path crosses the neck: the PI pair R adds to the 0.26 Ω LDO path, so the 0.53 s worst case shrinks. Must be re-derived, then Vmin bench |
| 3 | HAT FR4 1.6 → 1.0 mm (65×30, 4-layer allowed at JLC) | −2.2 g | FR4 density est. | Stiffness of the 40-pin header joint. Torque spec unchanged |
| 4 | CM3 bracket designed to camera net 0 g, not +10 | 0 … −10 g | unknown upstream cam mass | Weigh the upstream cam first |
| 5 | Speaker stand → ring clamp (8.66 g STL now) | −3 … −5 g | est. | Woz/Luxo CAD |
| 6 | Eye bulb carriers → LED on a glued pad (2× 2.10 g STL) | −2 … −3 g | est. | Luxo |
| 7 | Phantom Waveshare 42×33 board in the trained head (robot.xml places it twice, BOM buys 1) | −4 … −16 g *on paper* | open: Hinton has the Playground XML | If Playground counts it, our real head is lighter than the 438.6 g estimate by that much |

- **Realistic (only 1, 2, 3, 5 low, 7 low):** −19.4 g → **≈ 419 g (+12.6 g over trained)**.
- **Best case (all, high end):** −46.4 g → **≈ 392 g (−14 g under trained)**.
- Without #7 the best physical case is −30.4 g → **≈ 408 g (+1.6 g)**.

## Hypothesis for Hinton: the drift may come from lateral CoM, not grams

The added mass sits on the head's +y (left) side: HAT + caps at y ≈ +47 mm. The heavy rows drift **left**. Cut #2 moves the CoM y from −1.55 to ≈ −2.25 mm (438.6 → 432.4 g), back toward the trained −3.91. Requested sim rows:

1. +28 g with CoM y at the trained −3.91 mm (grams only).
2. +0 g with CoM y at −1.55 mm (offset only).

If (2) drifts and (1) doesn't, cut #2 or a right-side balance placement matters more than total grams. Not proven.

## Next

Hinton: the threshold rows and the two CoM rows. Luxo: #1, #5, #6. Woz: re-derive hold-up for #2 before it's proposed for the BOM. Nothing changes the BOM until Chief rules.

## v2 after Hinton's threshold rows (Woz 2026-11-24 11:05 Paris, via Chief)

Chief's provisional budget: head ≤ +13 g over trained (≤ ≈419.6 g). Playground 968b18de has the Waveshare board in the trunk only, so cut #7 = 0 g.

**Correction: the realistic case no longer fits.** Without #7 it is 438.6 − 15.4 = **423.2 g (+16.6 g), over budget by 3.6 g.**

| Step | Cut | Δ | Head | Over trained | CoM (mm) | Basis |
|---|---|---|---|---|---|---|
| start | Option C, cam net +10, HAT+caps, amp+mic | | 438.6 | +32.0 | (8.31, −1.55, 20.44) | head_layout_v1 |
| 1 | right eye = mirrored left | −3.97 | 434.6 | +28.0 | (8.30, −1.20, 20.40) | STL volume |
| 2 | 2× BCAP0010 to trunk (hold-up e863193, needs 0.21 s sync bench) | −6.2 | 428.4 | +21.8 | (8.24, −1.90, 20.61) | catalogue |
| 3 | HAT FR4 1.6 → 1.0 mm | −2.2 | 426.2 | +19.6 | (8.19, −2.15, 20.68) | FR4 est. |
| 5 | speaker stand → ring clamp (low end) | −3.0 | 423.2 | +16.6 | (8.18, −1.67, 20.66) | est. |
| 6 | bulb carriers → glued LED pad (low end) | −2.0 | 421.2 | +14.6 | (8.17, −1.68, 20.61) | est. |
| 4 | CM3 bracket held to camera net ≤ +3 g (was +10 allowance) | −7.0 | **414.2** | **+7.6** | (8.29, −1.70, 21.82) | needs upstream cam mass |

- Path to ≤ 419 g with margin: steps 1–6 plus #4 → ≈414 g, 5.4 g under the 419.6 g budget. Without #4 the best is 421.2 g (fails by 1.6 g).
- #4 is the deciding line. It depends on the upstream `cam` mass inside the 45.6 g package, which is not in the XML; someone with Onshape access must read it. If the CM3 + bracket can't come in ≤ cam + 5.5 g, the head fails the budget.
- Small extras, not counted: amp breakout merged onto the HAT (≈ −1.5 g est.), low-profile 40-pin header instead of a stacking header (≈ −1.5 g est.).
- CoM positions of cuts are Woz estimates: eye (10.35, −40, −58), caps (12, 47, 6), speaker stand (10.1, −70, 24), bulbs (10.35, ±40, −55), camera (1, 0, −51). Eye and bulbs **corrected 2026-11-24**: they were (9.9, −40, 25) and (10, 0, 30). The CoM columns in this table and in the Hinton rows below still use the old positions; use the corrected rows in "Correction: eye and bulb positions".

### Where the extra grams sit (+32 g, 438.6 case)

| Item | Δ | Position (head_assembly, mm) |
|---|---|---|
| CM3 + bracket net | +10 (0…+10) | (1, 0, −51) |
| HAT + 2 caps | +18 | (17, 47, 6) |
| amp breakout, right by speaker (head_layout_v1 l.93) | +2 | (10, −45, 25) |
| mic, left wall (head_layout_v1 l.121) | +2 | (9.9, 74.5, 25) |
| **Itemised centroid** | **+32** | **(11.1, 28.3, −9.4)** |
| Implied by 406.6 → 438.6 CoM | +32 | (10.3, 28.4, −9.2) |

Hinton's (10.3, +38.1, −1.6) matches in x, is 2–10 mm further +y and ≈8 mm higher in z. The 1.8–8 mm gap between my two rows is rounding in the published CoM values. Recommend the final candidate (414.2 g, CoM above) as a full-body row, not a point mass.

### Board masses (nothing weighed)

- Waveshare Bus Servo Adapter (A): 16 g, Waveshare product page. Not bought; our trunk IMU-to-servo board replaces it. Its mass in Playground's 0.698526 kg trunk: Hinton can read it; I have not.
- Our trunk IMU-to-servo board: not laid out, no mass.
- Zero Robot HAT (head): ≈11.8 g at 1.6 mm (FR4 5.8 + header 3 + parts 3), ≈9.6 g at 1.0 mm. Woz estimates.
- Waveshare board mass/CoM inside the trunk: not read (no Onshape access). Weigh it on the 0.1 g scale when one is on the bench (Chief 2026-11-24).

## Rows for Hinton, full inertia (Woz 2026-11-24, `hardware/tools/headrow.py`, `headopts.py`)

Point-mass deltas on the trained head (Playground `head_assembly` @ f698836 = b9be205: 406.607 g, CoM (8.154, −3.908, 22.773) mm). Inertia about the CoM in the head_assembly frame, kg·m², MJCF fullinertia order Ixx Iyy Izz Ixy Ixz Iyz. Removed parts are negative point masses (their own inertia ignored). All masses and positions are Woz estimates; nothing weighed.

| Row | Mass (g) | CoM (mm) | Δ from trained (mm) | fullinertia |
|---|---|---|---|---|
| B 421.2 (cuts 1, 2, 3, 5, 6) | 421.24 | (8.237, −1.685, 20.596) | (+0.08, +2.22, −2.18) | 2.52161e-3 1.74306e-3 1.10197e-3 −8.31130e-6 8.90054e-5 −1.01245e-5 |
| A 414.2 (+ cut 4) | 414.24 | (8.360, −1.714, 21.806) | (+0.21, +2.19, −0.97) | 2.48510e-3 1.70620e-3 1.10158e-3 −8.39812e-6 9.26939e-5 −1.09834e-5 |
| 411.2 (A + amp on HAT + header), DROPPED, fails y | 411.2 | (8.33, −1.62, 21.83) | (+0.18, +2.29, −0.94) | 2.47870e-3 1.70584e-3 1.09538e-3 −8.12032e-6 9.25675e-5 −1.20290e-5 |
| **410.0**: A + header, mic → right wall, amp breakout → right wall, HAT parts + FR4 (6.6 g) → trunk, 3.88 g ballast | 410.02 | (8.205, −4.200, 22.148) | (+0.05, −0.29, −0.62) | 2.48560e-3 1.70356e-3 1.10351e-3 −3.83553e-6 9.15456e-5 −1.55733e-5 |
| **407.4**: as 410.0, but all HAT mass (8.1 g) → trunk, 2.75 g ballast | 407.39 | (8.167, −4.200, 22.200) | (+0.01, −0.29, −0.57) | 2.47606e-3 1.70304e-3 1.09425e-3 −3.29793e-6 9.13375e-5 −1.70325e-5 |
| 412.7: A + low-profile header, amp breakout stays right (10, −45, 25) | 412.74 | (8.328, −1.891, 21.863) | (+0.17, +2.02, −0.91) | 2.48115e-3 1.70571e-3 1.09790e-3 −7.76448e-6 9.24883e-5 −1.21425e-5 |

- **Cut #4 basis (camera net ≤ +3 g):** the upstream `cam` can't be estimated by STL volume. It's a comment-only part in both Playground b9be205 (`<!-- Part cam -->`, no geom, no mesh) and upstream b23317a robot.xml, and no cam STL exists in `print/` or the assets. Its mass is merged into the 406.607 g head inertial at an Onshape-assigned value I can't read. The upstream BOM buys an unbranded "Raspberry pi camera" (AliExpress, ≈ €4.8), not the SC1223. The SC1223 product brief (RP-008151-DS-1) gives 25 × 24 × 11.5 mm and **no mass**. So cut #4 stays unverified on paper. A bench weigh settles it: SC1223 plus the printed bracket (the bracket by STL volume once Luxo draws it), against the upstream camera weighed or read from Onshape.
- **Y_RIGHT = −72 mm assumption (410.0 and 407.4 rows):** the amp breakout and the ballast sit at (10, −72, 25), my estimate of the inner right wall, mirrored from the mic wall. Not ray-cast against the head shell. Mic mirrored to (9.9, −74.5, 25). Ballast for y −4.2 is grams = m·(y − (−4.2)) / (−4.2 − Y_RIGHT); a wall 1 mm further in adds ≈0.06 g.
- Not counted in either row: neck pigtail/connector mass that replaces the board in the head.
- Hinton's lateral band (via Chief 2026-11-24): Δy ∈ [−1.55, +1.0] mm, absolute y −5.46 … −2.91 mm; aim y −4.2 so the ±0.6 mm centroid uncertainty fits.

## Correction: eye and bulb positions (Woz 2026-11-24 15:20 Paris)

- **What was wrong.** Cut #1 (right eye) and cut #6 (bulbs) were placed at (9.9, −40, 25) and (10, 0, 30). The eye holes are in the front wall. A circle fit to the head.stl rim (`woz/eyefit/rimfit.py` on razer, 6 slices z −58.8 … −56.3, head_assembly frame) gives Ø20.00 centred (10.35, +40.00) and Ø25.00 centred (10.35, −40.00), rms ≤ 0.006 mm, max − min radius ≤ 0.01 mm.
- **What the upstream parts are** (STL, `print/`): left_eye and right_eye are bezels with a Ø25 bore (radial r 12.50), 35 × 35 × 4.3 and 40 × 40 × 8.4 mm. bulb.stl is a 35 × 35 × 2 mm plate. Four Ø2 holes sit on a 20 mm square round the left hole and a 25 mm square round the right one.
- **New positions:** eye (10.35, −40, −58), bulbs (10.35, ±40, −55). The z values are Woz estimates: the parts are not in robot.xml and the Onshape placement is not read. ±6 mm in z moves the head CoM by ≤ 0.06 mm.
- **Effect:** masses unchanged; CoM z up ≈ 1.2 mm; x and y unchanged within 0.01 mm. `tools/eyerow.py` reproduces the published rows exactly with the old positions, then gives the corrected rows.

| Row (corrected) | Mass (g) | CoM (mm) | Δ from trained (mm) | fullinertia |
|---|---|---|---|---|
| B 421.2 | 421.24 | (8.232, −1.685, 21.782) | (+0.08, +2.22, −0.99) | 2.48212e-3 1.70675e-3 1.09876e-3 −8.37857e-6 8.79616e-5 2.21412e-6 |
| A 414.2 | 414.24 | (8.354, −1.714, 23.012) | (+0.20, +2.19, +0.24) | 2.44439e-3 1.66868e-3 1.09837e-3 −8.46532e-6 9.17082e-5 1.34109e-6 |
| 412.7 | 412.74 | (8.322, −1.891, 23.074) | (+0.17, +2.02, +0.30) | 2.44038e-3 1.66813e-3 1.09469e-3 −7.83124e-6 9.14868e-5 9.35099e-8 |
| 410.0 (3.88 g ballast) | 410.02 | (8.199, −4.200, 23.367) | (+0.04, −0.29, +0.59) | 2.44454e-3 1.66569e-3 1.10030e-3 −3.89654e-6 9.04815e-5 −4.49087e-6 |
| 407.4 (2.75 g ballast) | 407.39 | (8.161, −4.200, 23.426) | (+0.01, −0.29, +0.65) | 2.43495e-3 1.66511e-3 1.09104e-3 −3.35894e-6 9.02546e-5 −5.95007e-6 |

### Eye light net mass (replaces cut #6)

- **In, per eye:** Cree C513A-MSN + wire 0.3 g (est., not read), tapered PETG tube (half of a straight 1.2 mm wall over 22.35 mm, disc rear −57.55 to epoxy −35.2), diffuser disc sized to its own hole. Centroid (10.35, ±40, −47). Densities PETG 1.27, PMMA 1.19 g/cm³ are typical values, not datasheet.
- **Out:** both upstream bulb plates, 2.10 g each (solid-PLA STL volume, an upper bound). The bezels stay (Luxo look parts). The upstream glass is not itemised, so its removal is not counted: if the disc replaces it, the real net is lower.

| Build | Left Ø20 (g) | Right Ø25 (g) | Pair (g) | Net vs upstream bulbs | Net vs the cut-#6 rows |
|---|---|---|---|---|---|
| Proto, PETG 1.0 mm disc | 1.83 | 2.33 | 4.16 | −0.04 g | +1.96 g |
| Final, PMMA 2.0 mm disc | 2.18 | 2.87 | 5.05 | +0.85 g | +2.85 g |

| Row + eye light (proto) | Mass (g) | CoM (mm) | Δ from trained (mm) | fullinertia |
|---|---|---|---|---|
| A 414.2 | 416.20 | (8.363, −1.753, 22.725) | (+0.21, +2.15, −0.05) | 2.45442e-3 1.67564e-3 1.10145e-3 −8.43289e-6 9.19457e-5 1.72898e-7 |
| 410.0, ballast 3.71 g | 411.81 | (8.208, −4.200, 23.074) | (+0.05, −0.29, +0.30) | 2.45381e-3 1.67274e-3 1.10254e-3 −3.89254e-6 9.07395e-5 −5.38935e-6 |
| 407.4, ballast 2.58 g | 409.18 | (8.171, −4.200, 23.131) | (+0.02, −0.29, +0.36) | 2.44423e-3 1.67218e-3 1.09328e-3 −3.35494e-6 9.05173e-5 −6.84855e-6 |
| 410.0 final disc, ballast 3.65 g | 412.64 | (8.212, −4.200, 22.922) | (+0.06, −0.29, +0.15) | 2.45929e-3 1.67712e-3 1.10364e-3 −3.89110e-6 9.08735e-5 −5.68405e-6 |
| 407.4 final disc, ballast 2.52 g | 410.01 | (8.175, −4.200, 22.978) | (+0.02, −0.29, +0.21) | 2.44972e-3 1.67657e-3 1.09438e-3 −3.35350e-6 9.06538e-5 −7.14325e-6 |

- The 441.3 / 442.2 g figures from the 2026-11-24 eye-fit report are withdrawn: they were gross additions (no bulbs out) on the stale 434.6 g baseline.
- Nothing is weighed. A bench weigh of the bulb plates and the printed tube settles the net.

## HAT-to-trunk option: neck conductors (scoping, not a change)

Today the 12-pin JST-GH signal connector carries 11 of 12: SDA, SCL, 2 twist GNDs, 3V3 (I²C); GPIO22, GPIO27, foot GND; SW_EN (GPIO17); INA226 ALERT; projector gate GPIO25. Power stays on the Micro-Fit 2×2 (PI pair, AUX pair); the STS bus and USB are separate leads.

| HAT function | If moved to the trunk | Extra neck conductors |
|---|---|---|
| LTC3226 hold-up controller, PFO → GPIO6 | Controller next to the caps (already trunk, cut 2) | **+1** |
| LTC3226 PGOOD → CAPGOOD GPIO5 | as above | **+1** |
| Held-up 5 V to the Pi | rides the existing PI pair | 0 |
| INA226 ALERT 10 kΩ pull-up | to trunk, line already crosses | 0 |
| SG90 PWM GPIO12/13 1 kΩ series | in-line at the SG90 lead, head | 0 |
| I²S GPIO18–21 + SD_MODE 100 Ω, SD_MODE pull-down | onto the amp/mic breakouts, head | 0 |
| Eye LEDs GPIO23/24 | head loads | 0 |

- **Result: 13 conductors against 12. Over by 1.**
- Ways back inside: (a) JST-GH 14-pin (GH is made in 2–15 circuits, 1 A at AWG 26, JST GH series page): 13/14, 1 spare. (b) BNO055 UART mode (harness fallback 1) frees 1: 12/12, no spare. (c) Keep the LTC3226 in the head: 0 extra, but that mass stays at y +47 and the 410.0/407.4 rows no longer hold.
- Side effects to check before any decision: the PFI trip (4.70 V, sensed on the HAT today) would sense on the trunk side of the 21 mΩ PI pair, so the 0.045 V drop has to be added back into the setpoint; the head loses its board, so the neck harness needs an in-line connector or a pigtail soldered to the Pi header, with its own strain relief; the 1,000-cycle flex test would log 20 conductors (13 signal + 4 power + 3 STS bus), not 18.

## Mechanical-green additions (Atlas condition, Chief accepted 2026-11-24)

- [ ] **Head mass, weighed:** the assembled head (all parts, glue, cables up to the neck connector) on a 0.1 g scale is ≤ trained + 8 g = **≤ 414.6 g**. CAD numbers don't count.
- [ ] **Head CoM y, measured:** two-point hang or knife-edge balance on the assembled head. The method must resolve y to better than ±0.5 mm. Record the method, repeat count and spread. **WARNING, not a gate** (Chief 2026-11-24): if y lands outside Hinton's band (absolute y −5.46 … −2.91 mm, head_assembly frame), flag it to Chief and Atlas. The gate is Atlas's paired DR drift (median Δheading ≤ +8°/100 s, heading hold on) plus vx. The ±0.5 mm accuracy requirement stays a check.
- Plan of record: low-profile 40-pin header. The amp-to-HAT cut is dropped (fails the y band, rows above).
