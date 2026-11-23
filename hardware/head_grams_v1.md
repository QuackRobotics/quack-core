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
