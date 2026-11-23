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
| USB, Pi to Waveshare adapter | upstream 50 cm micro-USB to USB-C | — | data | — |

- **Pi pair.** Only the PI pair feeds the LTC3226, and only the Pi and camera are held up.
- **Aux pair.** The SG90s and the amp tolerate sag and are never on the backup rail.
- **Power-fail trip (PFI).** Sensed on the HAT. 4.70 V nominal with a 0.1% divider (trips 4.60–4.80 V); regulator setpoint 5.10 V (DECIDED, Quack Chief 02:26; the 5.15 V in a967d62 is withdrawn because its 5.23 V no-load corner breaks Atlas's ≤ 5.16 V cap). With ±1% reference and 0.1% feedback resistors the output is 5.04–5.16 V, so the HAT sits ≥ 4.99 V after the 0.045 V Pi-pair drop, above the worst PFI release (4.88 V). See BOM line 10 for the tolerance stack.
- **Ground returns (Atlas condition 1).** Each pair has its own GND conductor. PI GND and AUX GND join only at the regulator output in the trunk, never in the head. The STS bus GND joins at the battery/BMS negative.
- **Amp supply (Atlas condition 2).** The MAX98357A VIN and GND are wired ONLY to the AUX pair. It is never connected to the HAT 5 V or the LTC3226 output. Its I²S and SD_MODE inputs get 100 Ω series resistors so an unpowered amp is not fed from Pi GPIO.
- **Audio signals stay in the head.** BCLK GPIO18, LRCLK GPIO19, mic DATA GPIO20, amp DIN GPIO21, SD_MODE GPIO4 are short head-internal runs; none cross the neck.

## Routing rules

1. Keep I²C away from the STS bus and both 5 V pairs. Run it on the opposite side of each joint's cable channel, and never in the same sleeve as the STS bus.
2. Twist SDA with a GND and SCL with a GND, at about 1 turn per 25 mm. Run I²C at 100 kHz, the Pi default; don't raise it.
3. Give each joint a service loop sized for the full joint range. Head yaw (±160°) gets a flat loop around the yaw axis inside `head_yaw_to_roll`, not a straight pass.
4. Strain-relieve each bundle at the trunk exit, at every joint and at the HAT. Every cable channel edge in the path must be free of sharp edges.
5. Silicone insulation only. PVC stiffens and cracks at the yaw loop.

## Fallbacks if the bench shows I²C errors (in order)

1. **BNO055 UART mode.** It costs €0 and adds no mass. Upstream `imu.py` lines 26–27 already contain the UART path, commented out. Strap PS1 high, PS0 low (both pins are on the Adafruit breakout). 115200 baud on the PL011, GPIO14/15 (pins 8/10), as Atlas specified. Use `dtoverlay=miniuart-bt` rather than `disable-bt` if the Bluetooth gamepad stays, because disable-bt removes Bluetooth.
2. **PCA9615 differential extender** (BOM line 20, OPTIONAL). It needs 2 chips and adds one extra pair, about 1.5 g and €8.

## Bench check before mechanical green

- Read the IMU at 100 Hz for 10 min while the robot walks. Pass: 0 NACKs and 0 bus resets.
- Measure 5V_PI at the HAT under a 2.16 A load with the AUX pair loaded at 2.76 A. Pass: ≥ 4.75 V.
- Flex the head through its full yaw range for 1,000 cycles. Pass: no conductor break and no insulation wear.
- **Audio + stall rail test (Atlas condition 4).** Both SG90s stalled and full-scale audio (0 dBFS, max_volume cap bypassed) for 60 s. Scope 5V_PI at the Pi pins referenced to the Pi GND pin. Pass: ≥ 4.63 V min, PFO never asserts, no audible dropout. Repeat with the camera streaming.
- **PF mute.** On PFO, SD_MODE (GPIO4) goes low first in the handler; scope SD_MODE vs PFO, pass ≤ 50 µs.

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

So the 0.53 s hold-up (1.8 A = Pi + camera, no servo) stands. Not all 16 are behind the servo switch: the 2 SG90 are on 5V_AUX, and that is deliberate.

**Servo-power switch location (flag):** it must carry the ~25 A stand-up bus current, so it sits on the **trunk board** at the pack, not on the HAT. The AO3401A in the original HAT plan (≈4 A P-FET) cannot carry the bus and is withdrawn for this role. The Pi drives the switch gate from the e-stop GPIO through one neck conductor (added to the conductor list at schematic time). Part to be chosen at schematic: ≥ 40 A, ≤ 5 mΩ, with gate pulled to OFF by default so a broken neck wire = servos off.
