# Zero Robot HAT v1: re-scope for Raspberry Pi Zero 2 W (Woz 2026-11-24 10:55 Paris, PROPOSED)

Replaces the stale `radxa_hat_v1` spec (Radxa pin names, GPIO3_A0 pin 37). Board is the Pi Zero 2 W (Chief 2026-11-23 01:07). This is the scope only; KiCad 9 schematic starts after Chief accepts it. Pin numbers are 40-pin header physical pins, BCM GPIO names.

## Form factor

- pHAT outline on the Zero: 65 × 30 mm, 4× M2.5 holes on 58 × 23 mm (Raspberry Pi Zero mechanical drawing; re-check against the official PDF at schematic time).
- 2-layer, 1.0 mm FR4 (head grams cut −2.2 g, head_grams_v1). JLCPCB, panelized.
- Sits in the head (harness_neck_v1). Mass target ≤ the current HAT minus 2.2 g; weigh the first article.

## Pin map (every line from an existing doc; "NEW" = proposed here)

| BCM | Pin | Net | Source |
|---|---|---|---|
| 2 / 3 | 3 / 5 | I²C1 SDA/SCL → neck → BNO055 + INA226 | harness_neck_v1 |
| 4 | 7 | SD_MODE (100 Ω series, **1.0 kΩ pull-down** on the amp side, fitted 0 Ω; was 100 kΩ, see "SD_MODE boot state") | head_layout_v1 18:40, rev 2026-11-24 |
| 16 | 36 | SD_MODE alt (DNP 0 Ω, googlevoicehat fallback) | head_layout_v1 |
| 6 | 31 | PFO from LTC3226 (power-fail interrupt) | head_layout_v1 / harness |
| 12 / 13 | 32 / 33 | SG90 antenna PWM, 1 kΩ series at HAT | harness_neck_v1 |
| 14 / 15 | 8 / 10 | PL011 UART, BNO055 UART fallback | harness_neck_v1 |
| 18 / 19 / 20 / 21 | 12 / 35 / 38 / 40 | I²S BCLK / LRCLK / mic DATA / amp DIN, 100 Ω series | head_layout_v1 |
| 22 / 27 | 15 / 13 | Foot contacts → neck | harness_neck_v1 |
| 23 / 24 | 16 / 18 | Eye LEDs | head_layout_v1 (23–25 LEDs, projector) |
| 25 | 22 | Projector FET gate | projector_v1 |
| 17 | 11 | **NEW: SW_EN** servo-power switch = e-stop output (harness: "same GPIO as the e-stop") | Atlas to confirm |
| 26 | 37 | **NEW: INA226 ALERT** in, 10 kΩ pull-up to 3V3 on HAT | harness line-24 cond. 4 |
| 5 | 29 | **NEW: CAPGOOD** (LTC3226 PGOOD) in | head_layout_v1 SD_MODE text uses it |
| 0 / 1 | 27 / 28 | ID EEPROM: **not fitted in v1** (pHAT, no auto-overlay); pads only | Woz |
| 7–11 | — | Kept free (SPI0) for a future display/eye driver | — |
| 5 V | 2 / 4 | LTC3226 VOUT, wide pour to both pins | servo_candidates_v1 |
| GND | 6 / 9 + others | Return, PI GND only (AUX GND never joins in the head) | harness cond. 1 |

## On the board

- LTC3226 + supercap stack, PFO/CAPGOOD; **or** a 2-pin cap connector to the trunk if Chief takes the caps-to-trunk move (e863193: worst case 0.43 s hold-up, needs SD sync ≤ 0.21 s on the bench). Footprint both; pick at schematic freeze.
- JST-GH 12 neck connector (Atlas + Chief ack 2026-11-24; 11/12 used, 1 spare) + PI 5 V pair and AUX 5 V pair entries with strain relief.
- Passives listed above; I²S/SD_MODE 100 Ω, SG90 1 kΩ, ALERT 10 kΩ, SD_MODE 1.0 kΩ pull-down.
- Not on the HAT: MAX98357A (own breakout, AUX-pair only, harness cond. 2), regulator, BMS, INA226 (trunk).

## Open conflict found while scoping (needs Chief)

**Projector GPIO25 crosses the neck.** The projector now sits on the left shoulder (trunk), the Pi in the head, so GPIO25 needs a neck conductor: 18 conductors, and the JST-GH 10 is already full. Options: (a) 12-pin connector, (b) drive the projector FET from a spare INA226-side GPIO-expander in the trunk (new part), (c) route the projector gate via the Waveshare/USB side (no spare GPIO there that I have verified). My lean: (a), a 12-pin JST-GH, one spare. The flex test then covers 18 conductors.

## SD_MODE boot state (Atlas/Chief finding; Woz fix 2026-11-24)

Problem: GPIO4 boots with the SoC pull-up on (Chief/Atlas figure 50–65 kΩ to 3.3 V; I have not read a Broadcom value). Against 100 kΩ the pin sits near 1.7 V, so the amp is live (left channel) from power-on until firmware runs `gpio=4=op,dl`.

SD_MODE thresholds (MAX98357A), as quoted in the [Adafruit MAX98357 guide](https://learn.adafruit.com/adafruit-max98357-i2s-class-d-mono-amp/pinouts). The ADI PDF would not download here, so these are second-hand until someone reads the datasheet: **< 0.16 V shutdown**, 0.16–0.77 V (L+R)/2, 0.77–1.4 V right, > 1.4 V left. The chip has an internal 100 kΩ pull-down on SD_MODE, and the Adafruit breakout adds 1 MΩ from SD to VIN (5 V).

Worst case at boot, pin not yet driven: the sources are the SoC pull-up (3.3 V / 50 kΩ = 66 µA) and the breakout's 1 MΩ (5 µA), about 71 µA into the total pull-down.

| External pull-down (‖ internal 100 kΩ) | Boot voltage | State |
|---|---|---|
| 100 kΩ (old) | ≈ 1.7 V | **left: amp live** |
| 10 kΩ (Atlas) | ≈ 0.54 V | (L+R)/2: **amp live** |
| 2.2 kΩ | ≈ 0.15 V | shutdown, 0.01 V margin: reject |
| **1.0 kΩ (chosen)** | **≈ 0.07 V** | **shutdown, 0.09 V margin** |

- Driven high through the 100 Ω series resistor: 3.3 × 1.0/1.1 ≈ 3.0 V → left channel (> 1.4 V). Firmware sends mono on both channels, so which channel doesn't matter. The GPIO sources 3.0 mA, inside the default 8 mA drive.
- Moving SD_MODE to GPIO 9–27 (boot pull-down ≈ 50 kΩ) was rejected. 100 kΩ ‖ 100 kΩ ‖ 50 kΩ ≈ 25 kΩ against the breakout's 1 MΩ to 5 V reads ≈ 0.12 V, only 0.04 V margin, and the only free pin in 9–27 is GPIO16, the DNP fallback.
- Optional: cut the breakout's 1 MΩ. The 1.0 kΩ choice doesn't depend on it.
- Firmware still sets `gpio=4=op,dl` in config.txt, as a second layer.
- Hardware change: needs Woz + Atlas sign-off. Woz: signed on paper. Bench check: scope SD_MODE from pack-on until firmware runs. Pass: below 0.16 V throughout, and no speaker pop.

## Done-condition for the HAT schematic

Pin map accepted by Chief + Atlas (SW_EN/e-stop GPIO17), ERC clean, DRC at JLCPCB 2-layer rules, BOM lines with an alt part each, and 3D clearance check against head_layout_v1. No board is ordered without Naoufel's yes.
