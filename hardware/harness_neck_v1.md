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
- **Power-fail trip (PFI).** Sensed on the HAT. 4.70 V nominal with a 0.1% divider (trips 4.60–4.80 V); regulator setpoint 5.15 V so the HAT always sits ≥ 5.03 V, above the worst release (4.88 V). See BOM line 10 for the tolerance stack.
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
