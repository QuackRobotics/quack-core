# Neck harness v1 (trunk to head)

Woz, 2026-11-23 02:45 Paris. Status: PROPOSED. It needs Atlas sign-off.

The Pi Zero 2W, the Zero Robot HAT (with the LTC3226 and supercaps) and the camera sit in the head. The regulator, BMS, cells, BNO055 and Waveshare bus adapter sit in the trunk. The harness passes 4 joints: neck pitch, head pitch, head yaw (±160°) and head roll. The run is about 0.5 m one way, which I estimate at about 1 m round trip.

## Conductors

| Bundle | Conductors | Gauge | Load | Drop (Woz calc, ~1 m round trip) |
|---|---|---|---|---|
| PI 5 V pair | 5V_PI, GND_PI | 18 AWG silicone | Pi 1.5 A + camera 0.30 A + LTC3226 charge 0.36 A = 2.16 A | ~21 mΩ, 0.045 V |
| AUX 5 V pair | 5V_AUX, GND_AUX | 20 AWG silicone | 2× SG90 1.5 A + MAX98357A 1.26 A = 2.76 A peak (audio is IN for v1) | ~33 mΩ, 0.09 V |
| I²C (BNO055) | SDA+GND, SCL+GND twisted; 3V3 | 26–28 AWG | ~12 mA | n/a |
| Foot contacts | GPIO22, GPIO27, GND | 26–28 AWG | µA | n/a; 1 kΩ + 100 nF at the Pi |
| STS bus to head servos | V_pack, GND, DATA | per upstream | head servos | n/a |
| Servo-switch gate (added 2026-11-23) | SW_EN (Pi e-stop GPIO, head) to trunk-board switch gate; pull-down to OFF on the trunk side | 26–28 AWG | µA | n/a; broken wire = servos off |
| USB, Pi to Waveshare adapter | upstream 50 cm micro-USB to USB-C | — | data | — |

- **INA226 ALERT (BOM line 24; Atlas acked with 7 conditions, 2026-11-23 09:4x; Page ack pending).** ALERT uses the spare JST-GH pin: **17 conductors**, JST-GH 10 fully used (10/10). It is open-drain, pulled up on the Pi side (HAT), and goes to a Pi GPIO interrupt only.
- **Neck conductor count (after SW_EN).** 16 conductors + the USB cable, up from 15 under the two-pair split (PI + AUX). Power 4 (PI pair, AUX pair), signals 9 (I²C SDA, SCL, 2 twist GNDs, 3V3; foot GPIO22, GPIO27, foot GND; SW_EN), STS bus 3 (stock Feetech daisy-chain lead, not in the harness connector). If the BNO055 moves to UART mode, the I²C group becomes TX/RX/GND + 3V3: 1 fewer.
- **Neck connectors (proposed, ratings to confirm at schematic).** Power: Molex Micro-Fit 3.0, 2×2 (4 pos), one circuit per pin; per-pin load ≤ 2.76 A. Signals: JST-GH 10-pin (9 used + 1 spare). Both keyed, with strain relief on the trunk side of the neck pitch joint.
- **Pi pair.** Only the PI pair feeds the LTC3226, and only the Pi and camera are held up.
- **Aux pair.** The SG90s and the amp tolerate sag and are never on the backup rail.
- **Power-fail trip (PFI).** Sensed on the HAT. 4.70 V nominal with a 0.1% divider (trips 4.60–4.80 V); regulator setpoint 5.10 V (DECIDED, Quack Chief 02:26; the 5.15 V in a967d62 is withdrawn because its 5.23 V no-load corner breaks Atlas's ≤ 5.16 V cap). With ±1% reference and 0.1% feedback resistors the output is 5.04–5.16 V, so the HAT sits ≥ 4.99 V after the 0.045 V Pi-pair drop, above the worst PFI release (4.88 V). See BOM line 10 for the tolerance stack.
- **Ground returns (Atlas condition 1).** Each pair has its own GND conductor. PI GND and AUX GND join only at the regulator output in the trunk, never in the head. The STS bus GND joins at the battery/BMS negative.
- **Amp supply (Atlas condition 2).** The MAX98357A VIN and GND are wired ONLY to the AUX pair. It is never connected to the HAT 5 V or the LTC3226 output. Its I²S and SD_MODE inputs get 100 Ω series resistors so an unpowered amp is not fed from Pi GPIO.
- **Audio signals stay in the head.** BCLK GPIO18, LRCLK GPIO19, mic DATA GPIO20, amp DIN GPIO21, SD_MODE GPIO4 are short head-internal runs; none cross the neck.

## Routing rules

1. Keep I²C away from the STS bus and both 5 V pairs. Run it on the opposite side of each joint's cable channel, and never in the same sleeve as the STS bus.
2. Twist SDA with a GND and SCL with a GND, at about 1 turn per 25 mm. Run I²C at 100 kHz, the Pi default; don't raise it. 400 kHz only if the measured neck bus capacitance is below the 400 pF I²C limit (Atlas line-24 condition 6); not measured, so v1 stays at 100 kHz.
3. Give each joint a service loop sized for the full joint range. Head yaw (±160°) gets a flat loop around the yaw axis inside `head_yaw_to_roll`, not a straight pass.
4. Strain-relieve each bundle at the trunk exit, at every joint and at the HAT. Every cable channel edge in the path must be free of sharp edges.
5. Silicone insulation only. PVC stiffens and cracks at the yaw loop.

## Fallbacks if the bench shows I²C errors (in order)

1. **BNO055 UART mode.** It costs €0 and adds no mass. Upstream `imu.py` lines 26–27 already contain the UART path, commented out. Strap PS1 high, PS0 low (both pins are on the Adafruit breakout). 115200 baud on the PL011, GPIO14/15 (pins 8/10), as Atlas specified. Use `dtoverlay=miniuart-bt` rather than `disable-bt` if the Bluetooth gamepad stays, because disable-bt removes Bluetooth.
2. **PCA9615 differential extender** (BOM line 20, OPTIONAL). It needs 2 chips and adds one extra pair, about 1.5 g and €8.

## Bench check before mechanical green

- Read the IMU at 100 Hz for 10 min while the robot walks. Pass: 0 NACKs and 0 bus resets.
- Measure 5V_PI at the HAT under a 2.16 A load with the AUX pair loaded at 2.76 A. Pass: ≥ 4.75 V.
- Flex the head through its full yaw range for 1,000 cycles. Pass: no conductor break and no insulation wear. **Continuity is logged on all 17 neck conductors, including SW_EN and INA226 ALERT** (Atlas line-24 condition 5).
- **Audio + stall rail test (Atlas condition 4).** Both SG90s stalled and full-scale audio (0 dBFS, max_volume cap bypassed) for 60 s. Scope 5V_PI at the Pi pins referenced to the Pi GND pin. Pass: ≥ 4.63 V min, PFO never asserts, no audible dropout. Repeat with the camera streaming.
- **No back-feed into 5V_AUX during hold-up (Chief 09:11).** 5V_AUX splits from 5V_PI at the TPS56637 output in the trunk, upstream of the LTC3226. The only path from the supercaps to 5V_AUX is LTC3226 VOUT/VCAP → VIN → PI pair → trunk junction → AUX pair. Reverse blocking of that path is NOT yet confirmed from the ADI datasheet. Test: trunk regulator off, 10 Ω on the AUX pair at the trunk, supercaps charged. Trigger PFI and scope 5V_AUX plus the current in the PI pair. Pass: 5V_AUX < 0.5 V within 10 ms, PI-pair reverse current < 1 mA, hold-up ≥ 0.53 s. If it fails: add an ideal-diode (reverse-blocking) stage on 5V_PI at the HAT input, ahead of the LTC3226 VIN, and recheck the 4.75 V rail margin.
- **PF mute.** On PFO, SD_MODE (GPIO4) goes low first in the handler; scope SD_MODE vs PFO, pass ≤ 50 µs.
- **SG90 back-feed with 1 kΩ fitted (Chief 09:52, extends the 1bbfeee scope check).** 5V_AUX dead (regulator off, AUX pair disconnected at the trunk), both GPIO12 and GPIO13 held HIGH (3.3 V, static). Measure (a) the SG90 signal pin at the servo connector and (b) the dead 5V_AUX rail at the head end, with a DMM and on the scope. Pass: 5V_AUX < 0.5 V and the SG90 does not twitch or power up; record the signal-pin voltage and the current through each 1 kΩ (V across R / 1 kΩ, expected ≤ 2.7 mA, Woz estimate). If 5V_AUX rises toward ~2.5 V the leak is partly powering the rail: raise the series R to 4.7 kΩ after re-checking the pulse edges, and the firmware rule below becomes mandatory, not belt-and-braces.

## 5 V regulator: TPS56637 @ 5.10 V on the trunk board (DECIDED 2026-11-23 02:26)

Proposed by Woz, signed off by Atlas 02:24, decided by Quack Chief 02:26. The Pololu D36V50F5 is bring-up only; the policy never runs on it. Atlas's conditions:

1. **Feedback divider 0.1%.** Output 5.10 V nominal, 5.04–5.16 V over ±1% reference + 0.1% resistors. Max 5.16 V at no load, under the Pi's 5.25 V.
2. **FCCM, not Eco-mode.** Set by the MODE pin strap; the strap value comes from the TI datasheet table at schematic time and is a layout-review checkpoint.
3. **Low-voltage cutoff on LOADED pack voltage:** pack ≥ 6.3 V during stand-up and walk (regulator dropout ≈ 5.8 V cold, ≈ 5.9 V hot, to be measured). See "Pack resistance and the stand-up block" below.
4. **Bench (3 boards):**
   - 30 min at 4.9 A, Vin 6.0 / 7.4 / 8.4 V, 35 °C ambient: case ≤ 100 °C, no foldback or hiccup.
   - Efficiency at 1 / 3 / 4.9 A; expect ≥ 88% at 7.4 V.
   - Dropout sweep: regulation holds down to ≤ 5.9 V in, hot.
   - Load step 0.5 → 4.9 A while Vin steps 8.4 → 6.3 V: HAT rail > 4.78 V, no PFI trip.
   - Ripple < 50 mV p-p measured at the HAT (tip-and-barrel probe).
   - BNO055 UART errors = 0 at full load.

## Backup mode (LTC3226): setpoints and Vmin bench (Atlas ack 2026-11-23)

Setpoints (Atlas acked; detail and tolerance stack in BOM line 10):
- Backup LDO **4.40 V** nominal, 0.1% divider: 4.29–4.51 V over LDO_FB 0.76–0.82 V.
- PFI trip **4.70 V** nominal, 0.1% divider: trips 4.60–4.80 V, always above the LDO high corner (4.51 V); releases at ≤ 4.88 V, below the HAT minimum (≥ 4.99 V at 5.10 V regulator setpoint).
- Gate in backup = "the Pi stays up and the sync finishes", not the 4.63 V under-voltage flag.

Hold-up and the load current behind it (5 F stack, LDO path 0.26 Ω = 0.2 Ω pass FET + 0.06 Ω cap ESR, worst stack start 5.17 V, LDO high corner 4.51 V):

| Backup load | Window (worst) | Hold-up worst | Hold-up nominal |
|---|---|---|---|
| **1.8 A, camera streaming (design case)** | 5.17 − (4.51 + 1.8 × 0.26) = 0.192 V | **0.53 s** | 5.30 − 4.868 = 0.43 V → 1.2 s |
| 1.5 A, camera off | 5.17 − (4.51 + 1.5 × 0.26) = 0.27 V | 0.90 s | 5.30 − 4.79 = 0.51 V → 1.7 s |

- **The 0.53 s figure is at 1.8 A, camera streaming.** On PFO the camera stop runs in parallel with the flush, so the load falls toward 1.5 A partway through; 0.53 s is the conservative bound. SD-sync budget = half of 0.53 s = **0.26 s**.
- Both loads are Woz budgets (Pi Zero 2W 1.5 A design + camera 0.3 A), not measurements. The bench below replaces them.

Vmin bench (Atlas conditions, all must pass before the 4.40 V LDO is locked; 10 boards):
1. **Real load, not a resistor.** Each board runs the flight load while its 5 V input is ramped down 5.0 → 4.0 V: log writer active with fsync, sync triggered, camera streaming and then stopped by the PFO handler, CPU loaded (all 4 cores). Record the voltage at which the Pi browns out (reset/hang) and whether the sync completed.
2. **Pass: the sync finishes at ≤ 4.25 V on all 10 boards.** One board needing more than 4.25 V fails the 4.40 V LDO plan → fall back to 2S2P supercaps (10 F, ESR halves) plus load shed, or a buck-boost backup (BOM line 10 item d).
3. **Sync timed while the under-voltage flag is set.** At 4.40 V the Pi is below its 4.63 V flag and throttles, so PFO-to-fsync-complete time is measured in that throttled state (vcgencmd get_throttled bit 0 set), not at 5 V. Worst of 10 boards must be ≤ 0.26 s (2× margin on 0.53 s).
4. **1,000 power pulls, fsck clean.** Pull pack power at random points (camera streaming, logging) 1,000 times; after each, fsck the SD card: zero errors, zero lost log records beyond the ≤ 100 ms ring buffer.
5. **PFI latches into safe stop.** A PFO interrupt latches: servos to safe stop (behaviour per Atlas's safe-stop policy), camera stopped, amp muted (SD_MODE low, GPIO4), log flushed. The latch does NOT clear on PFI release; it clears only after the rail is back above release and CAPGOOD reads high again (boot rule: no servo torque until CAPGOOD). Verify on a scope with a 4.88 V ↔ 4.60 V input toggle: no servo re-enable while toggling.

## Pack resistance and the stand-up block

Shared path from cells to the regulator tap (the regulator feed is starred at the BMS output, so servo-wire drop downstream of the star is excluded):

| Element | Nominal | Worst (cold / low SoC / aged) | Source |
|---|---|---|---|
| 2× Molicel P30B cell DCIR | 2 × 17 = 34 mΩ | 2 × 22 = 44 mΩ | 17 mΩ DCIR (Molicel P30B sheet, Cell Saviors entry); 22 mΩ is a Woz derate, not a datasheet figure |
| Cell interconnect (nickel / busbar) | 2 mΩ | 2 mΩ | Woz estimate |
| BMS discharge FETs | 4 mΩ | 6 mΩ | Woz estimate; depends on the BMS chosen |
| Main switch + pack connector | 3 mΩ | 5 mΩ | Woz estimate |
| Shared 16 AWG, ~0.2 m round trip | 3 mΩ | 3 mΩ | 13.2 mΩ/m |
| **Total** | **≈ 46 mΩ** | **≈ 60 mΩ** | |

At the 23.9 A clamped stand-up worst case:
- Sag = 1.10 V nominal, 1.43 V worst.
- From a 7.2 V rest pack: 6.10 V nominal, 5.77 V worst. Both are below the 6.3 V loaded floor; the worst case is at the regulator's cold dropout.
- **So the 7.2 V rest block must rise.** Required rest voltage = 6.3 V + 23.9 A × R: 7.40 V nominal, 7.73 V worst.
- **Provisional stand-up block: 7.85 V at rest (3.92 V/cell)** (Quack Chief 02:40, after Atlas showed 60 mΩ needs 6.3 + 23.9 × 0.060 + 0.1 = 7.83 V; Woz's earlier 7.6 V and 7.73 V omitted the 0.1 V margin and are withdrawn). **The per-unit rule overrides 7.85 V:** final value per unit = 6.3 V + 23.9 A × R_measured + 0.1 V, with R_measured from a bench step (1 A → 20 A, 1 s, at the BMS output) on every pack.
- The hard guard stays the loaded check: abort stand-up / sit down if the pack is < 6.3 V for > 50 ms (filtered so single PWM edges don't trip it).
- **Max-R reject cap: R_measured > 60 mΩ → reject the pack** (its per-unit gate would exceed 7.83 V, i.e. above the provisional 7.85 V that the whole fleet is sized to). Screening target ≤ 50 mΩ → gate ≤ 7.60 V; if every pack in a batch screens ≤ 50 mΩ, the batch gate may drop to its per-unit values.
- Two separate checks (Atlas acks both): (1) Woz's 50 ms < 6.3 V loaded abort during stand-up — the hard guard; (2) Atlas's > 1 s < 6.3 V loaded low_battery event — a fuel-gauge signal.
- **Fleet default until benched (Chief 09:52, 2026-11-23).** Every unit ships and runs on the 7.85 V rest gate until it has been benched. A lower per-unit gate (toward ~3.7 V/cell) is allowed only after that unit's bench R and its MEASURED stand-up current I_su are in: gate = 6.3 V + I_su,measured × R_measured + 0.1 V (the 23.9 A design figure is replaced by the measured I_su). This is a factory bring-up step, logged per serial number; no field or software-only lowering. Atlas's 7.16 V worked example is ~7.09 V (Chief's correction); it is an illustration only and no unit uses it.
- Walking draws less than stand-up, so walking may continue below the rest gate while the loaded check holds. power.low_battery (Luxo / Page / Atlas) = **7.85 V rest, provisional**, then the per-unit value once bench R is in.

## Power-fail safe stop (Atlas, settled 2026-11-23; Quack Chief)

On PFO (GPIO6) the handler executes **torque off within one 20 ms control tick**, in this order (all three are issued in the same tick):
1. Broadcast torque-disable on the STS bus (ID 0xFE, Torque_Enable addr 40 = 0).
2. Open the servo-power switch (same GPIO as the e-stop). Stop the SG90 PWM on GPIO12/13 (drive low, no pulses: SG90 goes limp).
3. Pull SD_MODE (GPIO4) low: amp muted.

**Latched, no auto-resume.** Re-arm needs all three: CAPGOOD high, rest pack voltage ≥ the stand-up gate (7.85 V provisional / per-unit), and an explicit operator re-arm (gamepad or API).

### Which servo is on which rail (16 servos)

| Servos | Qty | Supply | Behind LTC3226 / supercaps? | Cut on PFO by |
|---|---|---|---|---|
| STS leg + neck servos (STS3215, STS3046 on the 6 option-B joints if adopted) | 14 | Pack direct (2S, 6.0–8.4 V) via the servo-power switch | **No** | bus torque-disable + switch open |
| SG90 antennas (GPIO12/13 PWM) | 2 | 5V_AUX neck pair from the trunk TPS56637 (SG90 is a 4.8–6 V part, cannot sit on the pack) | **No** — 5V_AUX never touches the HAT backup rail | PWM stopped on GPIO12/13; 5V_AUX also collapses with the regulator |

So the 0.53 s hold-up (1.8 A = Pi + camera, no servo) stands.

**SG90 PWM series resistors (Woz 2026-11-23 10:25).** Fit **1 kΩ in series on GPIO12 and GPIO13**, at the HAT. This follows the same logic as the amp's 100 Ω. If 5V_AUX is down while a GPIO is high, the SG90 signal input could back-feed its own unpowered rail through its input protection. The 1 kΩ caps that at ≤ (3.3 − ~0.6 V diode) / 1 kΩ ≈ 2.7 mA per pin (Woz estimate), which is within a Pi GPIO's drive. The SG90 input current at 3.3 V is **not published** (I found no datasheet figure). The input is a logic-level PWM receiver, so 1 kΩ should not change the edges at 50 Hz. Bench check: scope the pulse at the servo with 1 kΩ fitted and confirm the servo tracks. The SG90s are **not** cut by the servo switch: on PFI they are stopped by driving PWM low, and 5V_AUX dies with the regulator when the pack goes. Not all 16 are behind the servo switch: the 2 SG90 are on 5V_AUX, and that is deliberate.

**Firmware rule, 5V_AUX off (owner: Atlas; Chief 09:52).** Whenever 5V_AUX is off or unknown (boot before the regulator is up, safe stop, PFO latch, e-stop, brown-out, shutdown), GPIO12 and GPIO13 are driven LOW or set as inputs (no pull-up), never left high and never pulsing. Back-feed into the SG90 is then ~0. Boot default: inputs until the 5 V rail is confirmed. This covers the power-fail safe stop (step 2 above) and also every other path that drops 5V_AUX.

**Servo-power switch location (flag):** it must carry the ~25 A stand-up bus current, so it sits on the **trunk board** at the pack, not on the HAT. The AO3401A in the original HAT plan (≈4 A P-FET) cannot carry the bus and is withdrawn for this role. The Pi drives the switch gate from the e-stop GPIO through one neck conductor (added to the conductor list at schematic time). Part to be chosen at schematic: ≥ 40 A, ≤ 5 mΩ, with gate pulled to OFF by default so a broken neck wire = servos off.

### Servo-power switch on the trunk board: Atlas's 7 conditions (APPROVED by Quack Chief 2026-11-23 09:44)

The switch moves from the HAT to the trunk board. It is approved only with all seven of Atlas's conditions, which are design requirements for the trunk-board schematic:

1. **High-side** switch (in the pack + line, so servo ground stays common with the Pi).
2. **Gate off on an open wire.** A broken SW_EN conductor turns the servos off (pull-down to OFF on the trunk side). **SW_EN continuity is part of the neck flex test** (the 1,000-cycle yaw test above).
3. Driven from a **Pi GPIO with a boot pull-down, active-high**, so the servos stay off through boot and reset.
4. **SW_EN AND LTC3226 PFO** on the HAT: the gate only turns on when the Pi asks *and* the power-fail comparator is not tripped (hardware AND, not only software).
5. **Gate soft-start** (controlled turn-on into the servos' bulk capacitance, to keep inrush below the BMS 8 A overcurrent trip).
6. **TVS on the switched rail** (clamps the inductive kick when the switch opens under ~25 A).
7. **No back-power through an idle-high TX line.** The STS bus TX idles high; with the servos unpowered it must not feed them through their signal pins. Either buffer the bus on the switched rail, or use a series R and drive TX low when the switch is off.

Status: requirements only; the parts are chosen at schematic time (Woz). Atlas owns items 3 and 7 in software (pin defaults, TX low when off).

**Protection_Current (addr 28) backstop: 1.4 A** (Chief 09:44). No objection: 1.4 A is above the 1.0 A clamp with margin for the 8.4 V full-pack case before the per-tick rescale catches up (duty 0.405 at 8.4 V gives ~1.14 A stall), and well below the STS3215 7.4 V stall (~2.4 A est.). My earlier 1.2 A was too close to 1.14 A. ≈ 215 counts if the 6.5 mA/LSB unit is right (bench-confirm).

## Distributor check, list only, NOTHING ordered (Woz 2026-11-23 10:08 Paris)

Mouser (Access Denied page) and DigiKey (Cloudflare check) both blocked this computer's browser and fetch today, so their own unit price and stock were NOT read. Prices below are the Octopart headline (lowest listed, distributor not named) and are labelled as such.

| Part | Order code | Mouser | DigiKey | Price read | Stock |
|---|---|---|---|---|---|
| LTC3226 (backup PowerPath) | LTC3226IUD#PBF (I-grade, as asked); LTC3226EUD#PBF (E-grade) | [Mouser EUD](https://eu.mouser.com/ProductDetail/Analog-Devices/LTC3226EUDPBF?qs=hVkxg5c3xu9ymFYCt8%252BBGQ%3D%3D) | [DigiKey search](https://www.digikey.fr/en/products/result?keywords=LTC3226) | EUR 5.619 ([Octopart IUD](https://octopart.com/part/analog-devices/LTC3226IUD%23PBF), lowest listed) | not read |
| CSD25404Q3 (LTC3226 path P-FET) | CSD25404Q3 (reel), CSD25404Q3T (small reel) | [Mouser search](https://www.mouser.com/c/?q=CSD25404Q3) | [DigiKey 5764611](https://www.digikey.com/en/products/detail/texas-instruments/CSD25404Q3/5764611) | EUR 0.412 ([Octopart](https://octopart.com/part/texas-instruments/CSD25404Q3), lowest listed, likely a bulk break) | not read; TI lists it ACTIVE |
| Mic, INMP441 module alt | INMP441 itself: no Mouser/DigiKey listing found (TDK part is end-of-life; atomic14 calls ICS-43434 its replacement). Module alt at both: Adafruit 6049 (ICS-43434) | [Mouser 6049](https://www.mouser.com/en/ProductDetail/Adafruit/6049?qs=olJun0bQHM88XeFsw90dVw%3D%3D) | [DigiKey 25589349](https://www.digikey.com/en/products/detail/adafruit-industries-llc/6049/25589349) | US$8.95 at [Adafruit](https://www.adafruit.com/product/6049) | Adafruit page: ICS-43434 DISCONTINUED, Adafruit names SPH0645 as replacement (rejected 02:24) |

Mic sourcing consequence: there is no long-life Mouser/DigiKey module for line 22. Keep the INMP441 module (BerryBase EUR 3.10, out of stock 2026-11-23) and buy fleet stock now from a second EU seller (TinyTronics / LaskaKit / AZ-Delivery, prices not read), or qualify a current TDK I²S part at schematic time. Not ordered.

## Line 9 / line 10 parts as priced (Woz 2026-11-23 09:55)

- **TPS56637 stage** (TI SLVSEG1A Table 4, 5 V row; LCSC qty-1 prices): TPS56637RPAR C841386, L Vishay IHLP3232DZER3R3M11 3.3 µH C511015 (14 mΩ, Isat 9.7 A; TI's own reference inductor), 2 × 22 µF 25 V X7R 1210 GRM32ER71E226KE15L C21397 out + 1 in, 100 nF VIN + BOOT, R6 75 k / R7 10 k 0.1 % (C723587 / C95204) → 0.6 × (1 + 75/10) = **5.10 V** exact. Total **€4.70**.
- **Efficiency.** TI Fig. 13 (5 V out, the same inductor, FCCM), lowest plotted VIN 8 V: **~93 % at ~5 A** (Woz curve read, ±1 pt). There is no 7.4 V curve. Hinton's 90 % stands as conservative; the bench point at 7.4 V / 4.9 A replaces it.
- **LTC3226 path P-FET:** TI CSD25404Q3 (5.5 / 6.5 mΩ typ / max at −4.5 V). **11.7 mV max at 1.8 A**, inside the ≤ 18 mV spec. Line 10 total **€9.05**. **Stock flag:** LTC3226 and CSD25404Q3 both show 0 at LCSC; source them from Mouser/DigiKey before the HAT order.

## Pack sense for the Pi (BOM line 24, PROPOSED 2026-11-23 09:55)

- **v1 as drawn has no pack-current sense and no pack-V ADC.** The only readings are per-servo Present_Voltage (addr 62) and Present_Current (addr 69) over the STS bus.
- **Proposal:** TI INA226 + 1 mΩ 2512 shunt, high-side on the trunk board at the BMS output, upstream of the regulator star.
  - Bus: the same 100 kHz trunk I²C as the BNO055, address 0x40.
  - Rate: 588 µs conversions with 4× averaging gives ~213 Hz for the V/I pair.
  - Alert: bus-undervoltage **6.0 V, latched** (Atlas condition 3; replaces Woz's 6.3 V). The 6.3 V crouch stays in software (10 consecutive samples).
  - Cost: €0.77.
- **Atlas's 7 conditions (acked 2026-11-23 09:4x, written in by Woz 09:59 Paris). Schematic requirements for line 24:**
  1. **Placement.** Shunt on the pack side, after the BMS and before the servo-switch / 5 V regulator split; VBUS sensed at the same node. Kelvin sense: two separate sense traces from the inner edges of the shunt pads to IN+/IN−, no load current in them. Rating ≥ 1 W: the BOM part LR2512D-2W-1mR-1% (LCSC C500612) is 2 W, against 25 A² × 1 mΩ = 0.63 W (3.2× margin).
  2. **ALERT routing.** ALERT goes to a Pi GPIO interrupt only. It never connects to SW_EN, the servo-switch gate or the SW_EN AND PFO gate. The servo switch is cut by software (safe stop) and by PFO, not by the INA226.
  3. **ALERT limit.** Bus-undervoltage 6.0 V, latched (Mask/Enable LEN = 1); the Pi clears it by reading Mask/Enable. The 6.3 V crouch stays in software over 10 samples.
  4. **Pull-up.** ALERT is open-drain: 10 kΩ to 3V3 on the Pi side (HAT), so an open neck wire reads as "alert" (fail-safe direction). Added to line 24 (cents).
  5. **Flex test.** The 17th neck conductor (ALERT) is in the 1,000-cycle continuity check.
  6. **I²C speed.** Neck I²C stays at 100 kHz. 400 kHz needs the measured bus capacitance (harness + BNO055 + INA226 + pull-ups) under 400 pF; not measured yet, so 100 kHz is the v1 setting.
  7. **Address.** INA226 at 0x40 (A0 = A1 = GND). The only other trunk I²C device in this repo is the BNO055, at 0x28 (0x29 with COM3 high), so 0x40 is free. The PCA9615 option is a buffer and has no address. Re-check if any I²C part is added to the trunk.
- **Effect on the stand-up rule:** the shunt adds 1 mΩ to the shared path (+0.024 V at 23.9 A). The per-unit R step must be measured downstream of the shunt, or have 1 mΩ added.
- **It also replaces estimates with data:** walking current and the rest-voltage → SoC map below.

## Runtime cost of the 7.85 V stand-up gate (Woz 2026-11-23 09:55)

Source: Molicel INR-18650-P30B product data sheet, "Discharge Rate Characteristics" plot. These are Woz reads of the 0.2C curve at 23 °C, ±100 mAh; that curve is close to rest voltage at 0.6 A. 2S pack, so cell mAh = pack mAh.

| Rest gate | Per cell | Discharged when the gate is reached (0.2C curve) | Capacity above the gate (of ~2,930 mAh at 0.2C) |
|---|---|---|---|
| **7.85 V (provisional)** | 3.92 V | ~750 mAh | **~26 %** |
| 7.20 V (old) | 3.60 V | ~1,800 mAh | ~61 % |

- **Cost: about 1,050 mAh, or about 36 % of rated capacity, that can no longer start a stand-up.**
- **Walking itself is not cut.** Walking continues below the gate while the loaded 6.3 V check holds. Only stand-up and recovery after a fall need the gate.
- **Walking current is not published** in the upstream docs or our repo (searched 2026-11-23). The minutes below are for assumed pack currents (Woz estimates). The INA226 line above is how we measure the real figure.

| Assumed walking pack current (est.) | Minutes above the 7.85 V gate (~750 mAh) | Minutes above 7.2 V (~1,800 mAh) |
|---|---|---|
| 2 A | ~22 | ~54 |
| 3 A | ~15 | ~36 |
| 4 A | ~11 | ~27 |

- With a per-unit gate of ≤ 7.60 V (packs screened at ≤ 50 mΩ) the window grows. Reading the same curve, 3.80 V/cell sits at ~1,150 mAh, which is ~23 min at 3 A (est.).
