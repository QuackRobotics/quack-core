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
  - Revised HAT + caps CoM ≈ (17, 47, 6) mm (was 22, 47, 6). With the camera: ≈ 434.6 g, CoM ≈ (8.3, −1.7, 20.4) mm, a ≈ 2.2 mm shift, mostly +y because the Pi sits off-centre.
  - Not checked: the speaker (`full_speaker`, `speaker_stand`) is not in robot.xml, so its position against the HAT needs Onshape or Luxo's CAD.
- **If audio is cut.** The head loses `full_speaker` and its two mounts. Their mass is unknown and is inside the 406.6 g.
