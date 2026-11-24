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
- **15° up-tilt.** The top edge of the tilted board reaches the inner face of the 3 mm wall. (Stepped window / angled boss options superseded by the bore below.)
- **Lens opening: flush Ø6.0 bore along the 15° lens axis (Luxo acked, Woz 2026-11-23 10:08 Paris).** Drilled along the lens optical axis (15° up), not normal to the face. Lens tip at (0, 0, −58.31) in head_assembly, 0.74 mm behind the outer face (z −59.05); the barrel enters the bore ~2.2 mm past the inner face (z −56.1). Bottom edge of the opening flush at x −2.78. 45° matte-black chamfer on the outer edge. Opening on the face ~6 × 7.5 mm (Luxo). Woz check: a Ø6 bore at 15° cuts each face as 6.0 × 6.21 mm (6 / cos 15°); across the 3 mm wall the clear slot is 6.21 + 3 × tan 15° = 7.0 mm tall, so ~7.5 mm with the chamfer is consistent. Barrel Ø5.75 (product brief) in Ø6.0 gives 0.125 mm radial clearance: FDM holes print undersize (Woz rule of thumb ~0.1–0.2 mm), so ream to Ø6.0 after printing and check the lens does not touch the bore at full autofocus travel. Matte black inside the bore against flare. This replaces the 4 mm window conflict flagged in b9e0bca.
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
- **Page:** max_volume = **−6 dBFS** (−12 dBFS while an SG90 moves); supersedes −3 dBFS. See "Audio driver" below for the numbers. Budgets keep the full-scale rail peak, now 1.51 A (power basis, see "Audio driver").

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
- Price: EUR 3.10, BerryBase GY-INMP441 (listed not in stock 2026-11-23; listing gross weight 2 g). BOM line 22 updated 09:55.

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

## Camera lens vs Luxo's ≤ Ø4 window (Woz 2026-11-23 09:55)

- **Conflict.** The Camera Module 3 product brief (RP-008151-DS) gives a lens barrel of **Ø5.75 mm**. A Ø4 window is smaller than the barrel itself, so it cuts into the 66° × 41° (75° diagonal) field of view at any lens position. Ø4 does not work for the CM3.
- **Window needed** (Woz geometry, thin-aperture approximation using the barrel diameter as the clear aperture, which is an upper bound: the real entrance pupil is smaller and not published):
  - Lens barrel pushed into a bore so its front is flush with the outer face (z = −59.05): window = barrel bore **Ø6.0** (0.25 mm clearance), with no wall left in front of the glass.
  - Lens front at the inner face (z = −56.1, 3 mm wall): window ≥ 5.75 + 2 × 3 × tan 37.5° = **Ø10.4** (diagonal half-FOV 37.5°).
  - The 15° up-tilt makes the bottom edge worse. Use a stepped or angled window, not a straight hole.
- **Recommendation:** Ø6.0 bore, with the barrel flush to the outer face and a 0.5 mm chamfer. Luxo decides whether a Ø6 dark ring reads as a "nostril" or needs a bezel. Luxo must ack; until then the mount stays at x = 0.

## Right eye = mirrored left_eye.stl at 4.3 mm (Woz 2026-11-23 09:55, `hardware/tools/eyes.py` on upstream b23317a `print/`)

| STL (sha256, first 16) | Thickness (axis) | Outline | Bore | Volume | Solid PLA mass |
|---|---|---|---|---|---|
| left_eye.stl 04ec9038953f63f5 | 4.3 mm | 35 × 35 mm | Ø25.0 | 1.428 cm³ | 1.77 g |
| right_eye.stl 1d8ab5b5f5f153a2 | 8.4 mm | 40 × 40 mm | Ø25.0 | 4.629 cm³ | 5.74 g |
| bulb.stl 043cea34b4b37867 | 2.0 mm, print x 115–117 (3 mm behind the eye's back face at x 120) | 35 × 35 mm | Ø8 centre hole, inner ring r 8.0–8.6 mm | 1.695 cm³ | 2.10 g |

- **Fit.** Both eyes have the same Ø25 bore and start at the same back plane (print x = 120). A mirrored left eye (y → −y) therefore sits on the same Ø25 head eye hole (eye-hole edges at y ±27.5 on both sides). The 35 mm outline still overlaps the hole by 5 mm radially. **Fits, medium confidence:** this is mesh geometry only and is confirmed on the test print.
- **LED / mount.** The upstream bulb carrier (Ø8 hole, seat ring r 8.0–8.6) is behind the eye plane and does not touch the eye part. Changing the eye leaves the LED carrier and its 3 mm gap unchanged. Any LED or LED ring up to Ø25 OD passes through the bore. Luxo's LED part number is not in this repo, so its own fit is not checked.
- **Mass / CoM.** −3.97 g at the right eye. Head **438.6 → 434.6 g**, CoM (8.31, −1.55, 20.44) → **(8.29, −1.20, 21.20) mm**. That is 3.14 mm from Playground's CoM, still under 5 mm and inside 366–447 g. Hinton's regression list gets this delta.

## Audio driver: combined overlay, gain, volume cap, listening indicator (Woz 2026-11-23 09:55)

**Overlay** (`quack-audio`). Full duplex on the one PCM block: BCLK GPIO18, LRCLK GPIO19, mic DATA → PCM_DIN GPIO20, amp DIN ← PCM_DOUT GPIO21. The BCM I²S pins default to 18–21, so there is no pinctrl fragment. 48 kHz. The CPU DAI is forced to 2 × 32-bit slots = **64 BCLK/frame = 3.072 MHz**. That is the same clock as the MAX98357A datasheet test condition and inside the INMP441's 3.2 MHz max. The structure follows the stock `max98357a-overlay.dts` (raspberrypi/linux rpi-6.6.y). It was **not compiled here**: there is no dtc on the Team Computer.

```dts
/dts-v1/;
/plugin/;
/ {
    compatible = "brcm,bcm2835";
    fragment@0 { target = <&i2s_clk_producer>; __overlay__ { status = "okay"; }; };
    fragment@1 {
        target-path = "/";
        __overlay__ {
            quack_amp: max98357a {
                compatible = "maxim,max98357a";
                #sound-dai-cells = <0>;
                sdmode-gpios = <&gpio 4 0>;      /* SD_MODE GPIO4: PF/safe-stop mute */
                status = "okay";
            };
            quack_mic: inmp441 {
                compatible = "invensense,ics43432"; /* mainline I2S MEMS mic codec, capture only */
                #sound-dai-cells = <0>;
                status = "okay";
            };
        };
    };
    fragment@2 {
        target = <&sound>;
        __overlay__ {
            compatible = "simple-audio-card";
            simple-audio-card,name = "quack-audio";
            simple-audio-card,format = "i2s";
            simple-audio-card,bitclock-master = <&quack_cpu>;
            simple-audio-card,frame-master = <&quack_cpu>;
            status = "okay";
            quack_cpu: simple-audio-card,cpu {
                sound-dai = <&i2s_clk_producer>;
                dai-tdm-slot-num = <2>;
                dai-tdm-slot-width = <32>;          /* forces 64 fs */
            };
            simple-audio-card,codec {
                sound-dai = <&quack_amp>, <&quack_mic>; /* multi-codec link */
            };
        };
    };
};
```

Bench / software-owner checks, all before this is called done:
1. `dtc -@ -I dts -O dtb` compiles it. `modinfo snd-soc-ics43432 snd-soc-max98357a` shows both modules in the shipped kernel.
2. The kernel accepts a multi-codec link in simple-audio-card. If it doesn't, fall back to `googlevoicehat-soundcard`, which is Google's MAX98357A + ICS-43432 design. Its overlay drives SD_MODE on **GPIO16**, so our GPIO4 would need a param or a fork.
3. `aplay` + `arecord -f S32_LE -r 48000 -c 2` run at the same time. Scope BCLK: 3.072 MHz. The mic data is in the left slot.

**Amp gain: keep 9 dB** (GAIN pin unconnected, no part, the module default). The MAX98357A datasheet gives gain relative to 2.1 dBV, so 0 dBFS at 9 dB = 11.1 dBV = 3.59 Vrms = 3.2 W into 4 Ω. That is already at the 5 V clip point (Adafruit: 3 W at 10% THD, 2.5 W at 1%). 12/15 dB would only clip earlier, and a lower gain throws away headroom that the softvol cap already controls. Tolerance 8.4–9.6 dB.

**Page volume caps** (ALSA softvol, 4 Ω, from the 9 dB figure):

| Cap | Vrms | Power | Peak current |
|---|---|---|---|
| 0 dBFS (cap bypassed, bench only) | 3.59 V | 3.2 W | 1.51 A rail peak (was 1.27 A load-side) |
| **−6 dBFS = max_volume** | 1.80 V | 0.81 W | 0.38 A rail peak (was 0.64) |
| **−12 dBFS while any SG90 is moving** | 0.90 V | 0.20 W | 0.10 A rail peak (was 0.32) |

At −12 dBFS the AUX pair peak is 1.5 A (SG90 budget) + 0.10 A = 1.60 A, against a 2.76 A design.
**Amp supply-current basis (Woz 2026-11-24 11:20, replaces the amplitude-scaled 1.27/0.64/0.32 A column).** The old column was the speaker-side peak current (3.59 Vrms / 4 Ω × √2 = 1.27 A), halved per −6 dB. That is load current, not what 5V_AUX supplies. The rail supplies power: I_peak = 2 × P_out / (η × 5 V) for a sine (instantaneous power peaks at 2× average), so it scales with power, ¼ per −6 dB. η = 0.85 is MY ASSUMPTION (MAX98357A efficiency curve not read); quiescent current not included. Result: 0 dBFS 1.51 A (was 1.27, so the old figure was LOW), −6 dBFS 0.38 A, −12 dBFS 0.10 A. Bench measurement replaces this.


**Listening indicator: use Luxo's eyes, no separate LED in v1.** Why:
- A separate LED needs a new GPIO, a new window in the shell and a new part.
- A software-driven LED proves no more than an eye state does: the INMP441 is always powered from 3.3 V, so neither one is a hardware guarantee.
- Recommendation: Luxo owns a distinct "listening" eye pattern. If a hardware privacy guarantee is wanted later, put a GPIO-switched load switch on the mic 3V3 (about 1.4 mA) and tie the indicator to that same switch. That is v2.

## SD_MODE power-fail gate + GPIO16 (Woz 2026-11-23 18:40, PROPOSED)

- **Primary: SD_MODE on GPIO4 (pin 7)** through the 100 Ω series R (harness Atlas condition 2), with a **100 kΩ pull-down to GND at the HAT**. The amp is muted whenever the Pi is in reset, unbooted or dead. The quack-audio driver drives it high only after CAPGOOD reads high and 5V_AUX is up. The PFO handler drives it low first (bench pass ≤ 50 µs, harness_neck_v1).
- **GPIO16 fallback without a fork:** the HAT routes the SD_MODE net to GPIO4 through a fitted 0 Ω and to GPIO16 (pin 36) through a DNP 0 Ω. If the multi-codec simple-audio-card check fails and we move to `googlevoicehat-soundcard` (SD_MODE on GPIO16), swap the resistor. No overlay fork. GPIO16 is unused in the current pin map.
- **Channel select:** a GPIO-high level selects one channel on the MAX98357A instead of (L+R)/2. The SD_MODE thresholds were not re-read this run, so bench check 3 (aplay + arecord) must also confirm audio in the slot we write. Firmware writes mono into both slots.
