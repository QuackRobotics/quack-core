# Head layout v1: camera mount, projector, beak keep-out, head mass

Woz, 2026-11-23 02:45 Paris. Status: PROPOSED. It needs Atlas and Luxo sign-off, and Hinton's regression check on mass.

## Frame

All positions are in the robot.xml `head_assembly` frame, in mm: +x is up, −z is forward. The print-frame to head transform comes from the `head` geom in robot.xml at b23317a: pos (−0.12906, 0, 0.06095), quat (0.707, 0, 0.707, 0). It gives

    x_h = z_print − 129.06,  y_h = y_print,  z_h = 60.95 − x_print

This is the same transform Luxo derived. The outer front face is at z = −59.05 and the inner face is at z = −56.1, so the wall is 3 mm thick.

## Camera Module 3 mount (line 17)

- **Lens offset.** From the product brief drawing, the board is 25 × 23.862 mm. The lens is centred left-right, but it sits 14.4 mm from the connector edge and 9.46 mm from the far edge.
- **Lens position.** The lens centre is at (0, 0, face). That is inside Luxo's range of x = 0 to −2. I picked 0 over −2 because the 15° tilt pushes the lower rear corner downward.
- **Orientation.** The connector edge points up, toward the Pi. The board then spans x −9.5 … +14.4.
- **Clearances:**
  - Projector hole (Ø8 at x 24, edges at +20.4): 6 mm.
  - Eye-hole edges (y ±27.5): 15 mm.
  - Bottom sheet (top surface at x −16.3) with the 15° tilt: 3.3 mm.
- **Eye height (x = +10) is rejected.** With the connector up, the board reaches +24.4 and overlaps the projector hole. With the connector down, it sits 0.9 mm from the hole edge, which leaves no room for the reflector.
- **Behind the camera.** Inside the camera box, upstream geometry has nothing except the front wall. The head-roll servo case starts at z −37.7, which leaves about 4.6 mm behind the module and bracket for the ribbon bend.
- **Ribbon.** It runs from the top edge up to the Pi CSI connector, using the 200 mm cable that comes in the box. It goes around the head-roll servo case, not over it.
- **15° up-tilt.** The top edge of the tilted board reaches the inner face of the 3 mm wall. Either recess the lens about 2 mm with a stepped window, or print an angled boss. I'll check this on the test print.
- **CoM.** The CoM of camera plus bracket (not the lens) is about (1, 0, −51), with a 10 g budget. I'll weigh it when it's on the bench.

## Projector (Ø8 hole at x 24, y 0)

This hole is the upstream projector LED. The assembly guide pin table lists "Projector Anode GPIO 25". The upstream BOM lists a projector reflector at €2, and the Playground part list includes `bulb`. The reflector's outer diameter must be 20 mm or less to clear the camera bracket. I haven't read that reflector's size yet.

## Beak keep-out (for Luxo)

The upstream head has no geometry in front of the face plane. With the lens at x = +10 or x = 0, no point of the head enters the field of view.

For a new beak: the camera tilts 15° up and the vertical half-FOV is 20.5°, so the lower edge of the view runs 5.5° below horizontal. The lens aperture radius is 2.9 mm. The beak's top surface must stay below

    x <= -2.9 - 0.096 * d   (d = mm forward of the face)

That means x ≤ −2.9 at the face, ≤ −4.8 at 20 mm forward and ≤ −6.7 at 40 mm forward. A beak whose top root is at x ≈ 0 would show at the bottom of the image.

The eye flanges (Ø35/Ø40 at y ±40, protruding 4.3/8.4 mm) sit more than 60° off-axis. That is outside the 33° horizontal half-FOV, so they cause no bezel conflict. The left/right asymmetry is a looks question for Luxo only.

## Head mass

| Source | Mass | Notes |
|---|---|---|
| robot.xml b23317a `head_assembly` | 352.6 g | the antenna holders are separate child bodies here |
| + 2 antenna holder bodies | 8.4 g | 4.2 g each, so robot.xml total = **361.0 g** |
| Playground `head_assembly` (open_duck_mini_v2.xml @ f698836) | **406.6 g** | CoM (8.15, −3.91, 22.77); antennas merged in |
| Gap after antennas | 45.6 g | expression parts in Playground only: `cam`, `bulb` ×2, `glass` ×4, `left_eye`, `right_eye`, `full_speaker`, `speaker_interface`, `speaker_stand` |

- **Correction to my 01:45 note.** I said robot.xml lacks the antennas. It doesn't: it has them as separate bodies. The real cause of the gap is the expression package.
- **What the policy already saw.** The 406.6 g head the policy trained on already contains a camera and a speaker. So the CM3 replaces a camera, and its net change is 0 to +10 g, not +10 g on top.
- **Unknowns.** Per-part masses and the upstream camera's position are not in the XML. They need Onshape.

**Deltas against 406.6 g (Woz estimates, nothing weighed):**

| Item | Mass | CoM (mm) |
|---|---|---|
| CM3 + bracket, minus the upstream cam | 0 … +10 g | (1, 0, −51) |
| Zero Robot HAT (FR4 65×30×1.6 ≈ 5.8 g, header ≈ 3 g, parts ≈ 3 g) + 2× Maxwell BCAP0010 (6.2 g) | ≈ +18 g | ≈ (22, 47, 6): on the Pi, on the −x side |
| Option B only: D36V50F5 + connectors | +9 g | TBD |

**Resulting head:**

| Case | Mass | CoM (mm) | CoM shift |
|---|---|---|---|
| Option C, camera net 0 | 424.6 g | (8.74, −1.75, 22.06) | 2.4 mm |
| Option C, camera net +10 | 434.6 g | (8.56, −1.71, 20.38) | 3.3 mm |
| Option B, camera net +10 | 443.6 g | (8.79, −2.63, 20.09) | 3.0 mm, 3 g under the 447 g edge |

All cases are inside the 366–447 g training range and under the 5 mm CoM-shift limit. They are over the "< 20 g" rule, however, because the HAT and supercaps were never in any head figure. Hinton's regression is needed whatever we decide.

- **HAT fit: checked, fits on the −x side** (Woz 2026-11-23 02:40, `hatfit.py` ray cast of upstream robot.xml head meshes in the `head_assembly` frame, 45 rays over the Pi footprint).
  - Pi Zero 2W spans x 27.9…32.1, y 31.8…63.0, z −26.5…39.3. Its components face −x, so the header and HAT go on −x.
  - +x side: the shell top is 0–5 mm away (median 5.0 mm). No room, as expected.
  - −x side: the only part under the Pi footprint is `head_bot_sheet`, 44.3 mm away (inner face x ≈ −16.3). No servo case, roll mount, camera or SG90 is in that column.
  - Stack: HAT PCB at x ≈ 17…18.5 (11.5 mm header spacing), 2× BCAP0010 (Ø10) under it to x ≈ 7. That leaves ≈ 23 mm to the bottom sheet. The camera board (y ±12.5) is ≥ 19 mm away sideways.
  - Revised HAT + caps CoM ≈ (17, 47, 6) mm (was 22, 47, 6). With the camera: ≈ 434.6 g, CoM ≈ (8.3, −1.7, 20.4) mm. CORRECTION 2026-11-23 02:40: the shift from Playground's head CoM (8.15, −3.91, 22.77) is (+0.15, +2.21, −2.37) mm = **3.2 mm**, not 2.2 mm; 2.2 mm was only the +y component.
  - Speaker vs HAT: checked below (Audio). Audio is IN for v1 (Naoufel 02:18).

## Audio: speaker + MAX98357A (BOM line 21, IN)

Woz 2026-11-23 03:30. Source for position: upstream `print/speaker_stand.stl` and `print/speaker_interface.stl`. Like `head.stl`, they are exported in the assembly position, so Luxo's transform `head_assembly = (z_print − 129.06, y, 60.95 − x_print)` applies. Medium confidence: confirm in Onshape.

| Part | Mass | Source | Centroid (mm, head frame) | Extent |
|---|---|---|---|---|
| Speaker ANGEEK 4 Ω 3 W, Ø40 × 20 mm, magnet Ø22 | ≤ 15 g budget | not published; weigh | ≈ on the interface, (6.5, −64, 20) | — |
| `speaker_interface` (printed) | ≤ 2.24 g | STL volume 1.804 cm³ × 1.24 g/cm³ (solid upper bound) | (6.5, −64.1, 20.1) | x −12…25, y −75…−54, z 1…40 |
| `speaker_stand` (printed) | ≤ 8.66 g | STL volume 6.986 cm³ × 1.24 | (18.2, −54.8, 17.9) | x −15…32, y −75…−29, z −8…43 |
| MAX98357A breakout | ≈ 2 g | Woz est. | ≈ (10, −45, 25), next to the speaker | — |

- The speaker and both printed parts are already in Playground's 406.6 g head (`full_speaker`, `speaker_interface`, `speaker_stand`). Only the amp is new mass.
- **Clearances:** Pi + HAT at y +31.8…+63 (opposite side, > 60 mm). Camera board/bracket z ≤ −47 (> 38 mm). Projector LED at (24, 0, −59), reflector ≤ Ø20 (> 19 mm in y). No clash.
- **GPIO:** BCLK GPIO18 (pin 12), LRCLK GPIO19 (pin 35), DIN GPIO21 (pin 40), SD_MODE GPIO4 (pin 7). No clash with GPIO12/13 (antenna PWM), 14/15 (BNO055 UART fallback), 22/27 (feet), 23–25 (LEDs, projector). PFO proposed on GPIO6 (pin 31).
- **Page:** max_volume = −3 dBFS (≈1.6 W, 0.89 A peak). Budgets keep the 1.26 A full-scale peak.

### Speaker printed-part volume audit (Woz 2026-11-23 02:45, `hardware/tools/stlvol.py`)

Hinton quoted 30.6 g + 4.3 g for the same two parts. Recomputed two independent ways:

| File (upstream b23317a `print/`) | sha256 (first 16) | Units | Triangles | Open edges | Divergence volume | Ray-parity volume (0.5 mm grid) | Bbox | Solid PLA 1.24 g/cm³ |
|---|---|---|---|---|---|---|---|---|
| speaker_stand.stl | eb306cdc8693a44d | mm | 21 390 | 0 | 6.986 cm³ | 6.991 cm³ | 51.4 × 46.0 × 47.4 mm (112.1 cm³) | 8.66 g |
| speaker_interface.stl | c363018e1ed27e34 | mm | 2 316 | 0 | 1.804 cm³ | 1.804 cm³ | 38.6 × 20.8 × 36.9 mm (29.7 cm³) | 2.24 g |

- Units are mm: head.stl from the same export spans 199.8 × 197.3 × 59.4 mm.
- Both meshes are closed (every edge shared by exactly 2 triangles), so the divergence volume is exact; the ray-parity check agrees within 0.1 %.
- Hinton's figures imply 24.7 cm³ and 3.5 cm³. Neither matches the bbox. A convex-hull or mesh-inertia volume is a plausible cause; not proven. What would change this: a different file sha, or a closed-solid method that gets more.

## Microphone (BOM line 22, PROPOSED) - rev 02:35 2026-11-23

- Part: INMP441-class I2S MEMS module (SPH0645 rejected per Chief 02:24, Pi bit-alignment issue). Bottom port, 24-bit I2S, 64 SCK per WS frame. Datasheet limits as quoted by secondary sources (TDK PDF not readable this run): SCK 0.5-3.2 MHz, WS 7.8-50 kHz, about 1.4 mA, 1.8-3.3 V. Powered from Pi 3.3 V.
- 48 kHz: BCLK = 64 x 48 kHz = 3.072 MHz, inside the 3.2 MHz max with 4 % margin. The shared clock MUST run 64 fs (2 x 32-bit slots); a 16-bit/32 fs stream (1.536 MHz) gives the mic too few clocks. MAX98357A accepts 64 fs. Combined overlay must force bclk_ratio 64. Bench: scope BCLK = 3.072 MHz.
- Wiring: BCLK GPIO18, LRCLK GPIO19 shared with amp; mic SD -> GPIO20; L/R -> GND (left slot). Amp DIN GPIO21.
- Count: 1 mic in v1. Reason: the only wide baseline is the right wall, which carries the speaker grille (centre (10.1, -79, 24.4)); a second mic there sits beside the speaker. A 2nd INMP441 needs no new GPIO (L/R high, same SD line) so it stays a no-rework option once Luxo finds a right-side spot >= 40 mm from the grille.
- Port: Luxo left wall, head_assembly (9.9, +79.2, 25.0), Ø1.0 mm. Ray-cast (hardware/tools/micport.py, 5 rays, port +-3 mm): wall 3.2 mm thick (y 76.0...79.2), then clear air across the head to the opposite wall at y -76.0. Nearest parts: Pi 26.3 mm, bottom sheet 26.7 mm, head-roll mount 56.2 mm, head servo case 72.7 mm, antenna SG90 L 72.0 mm, SG90 R 132.1 mm, 42x33 board 119.6 mm. Module (14 x 14 mm, about 3 mm stack) occupies y 76.0...73; Pi top face at y 63 leaves about 10 mm.
- Recommendation: counterbore Ø5 from inside to leave 1.0 mm wall at the Ø1.0 hole (short port tube, resonance well above voice band). Gasket-seal module to wall, mount on foam, not hard-screwed, to decouple SG90 gear noise through the shell.
- Mass: not published, budget <= 2 g. CoM about (9.9, 74.5, 25.0) mm.
- Price: not read yet; line 22 still carries the Adafruit 3421 figure (EUR 15.44) until repriced.

## The unidentified 42 × 33 mm board = Waveshare Bus Servo Adapter (A) (Woz 2026-11-23 02:45, `hardware/tools/boardid.py`)

- Upstream `board.part` (onshape-to-robot metadata): Onshape part "Board <1>", document 64074dfc…, its own Part Studio fbc3ecd1…, modelled in the duck's main document, not in the a18ff8cc electronics library that holds the Pi, BNO055, BMS, SG90, charger and switch.
- Mesh: a featureless 42.0 × 33.0 × 1.6 mm plate, 2.14 cm³ (a solid 42 × 33 × 1.6 slab is 2.22 cm³). No components modelled.
- Upstream BOM line "Servo control board" = [Waveshare Bus Servo Adapter (A)](https://www.waveshare.com/bus-servo-adapter-a.htm). Waveshare: **42.00 × 33.00 mm**, holes 37 × 28 mm, **16 g**. Exact size match: high confidence it is this board. It is not the amp and not the regulator.
- robot.xml places the same mesh **twice**: in the trunk next to the BNO055, and in `head_assembly` at x 30.5…32.1, y −76…−34, z −38.6…−5.5 (flat under the shell top, mirror of the Pi, below the speaker stand). The upstream BOM buys **1**. One instance is a CAD duplicate; the files don't say which.
- **Mass impact.** quack-core carries no Waveshare board (our trunk IMU-to-servo board replaces it). If Playground's 406.6 g head counts the head instance, the trained head includes a phantom board: ≈ 4 g if Onshape used an FR4-like density on the 2.14 cm³ plate, up to 16 g if a real mass was set. robot.xml can't tell which; Hinton has the Playground XML. Our 438.6 g estimate is unchanged until that is resolved (this board was never added or removed in it).

## Head mass with audio + mic (Woz est., nothing weighed)

- Option C, camera net +10, HAT + caps at (17, 47, 6), + amp 2 g + mic 2 g on the left wall: **≈ 438.6 g, CoM ≈ (8.31, −1.55, 20.44) mm, 3.3 mm from Playground's CoM.** Inside 366–447 g and under 5 mm. Over the < 20 g rule, so Hinton's regression stands.

## Frame and origin for every head CoM in this file

- **Frame:** MuJoCo body `head_assembly` in upstream `open_duck_mini_v2/robot.xml` (same body as Playground's `head_assembly`; Luxo's transform x_h = z_print − 129.06, y_h = y_print, z_h = 60.95 − x_print). Not the trunk frame.
- **Origin:** the `head_roll` joint point (joint pos 0 0 0 in that body). The roll axis is the body's +z.
- **Shift reference:** every "CoM shift" is the 3-D distance from Playground's head_assembly CoM (8.15, −3.91, 22.77) mm at 406.6 g.
  - Camera + HAT: 434.6 g, (8.3, −1.7, 20.4), shift 3.2 mm.
  - Camera + HAT + amp + mic (left wall): 438.6 g, (8.31, −1.55, 20.44), shift 3.3 mm.
