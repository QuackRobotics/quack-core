# Zero Robot HAT v1 (Pi Zero 2 W): netlist + part list, TEXT DRAFT (Woz 2026-11-24 12:30 Paris)

Status: **DRAFT, not a schematic.** KiCad 9 is not installed on the Team Computer and razer is unreachable (Chief 12:17), so no `.kicad_sch` is written and **no ERC has been run**. This file is the input for schematic capture once KiCad is available. Source of every net: `hat_pi_zero2w_scope_v1.md` (scope, Atlas ack 1543092, SD_MODE rev 29f0387), `harness_neck_v1.md`, `projector_v1.md`, `bom/bom_v1.0_delta.csv` line 10. Nothing here is ordered.

Conventions: refdes are provisional. `J` = connector, `R` = resistor, `C` = capacitor, `U` = IC, `TP` = test pad. Header pins = Pi 40-pin physical numbers, BCM names.

## 1. Ground domains (Atlas harness cond. 1 + 2)
- **GND_PI**: Pi ground. Header pins 6, 9, 14, 20, 25, 30, 34, 39; neck I²C twist GNDs; foot GND; LTC3226 GND.
- **GND_AUX**: SG90s + MAX98357A return only. Enters on J2 pin 4, goes to J5/J6/J7 only.
- **GND_PI and GND_AUX never join on this board** (they join only at the TPS56637 output in the trunk). Schematic: two separate ground symbols, no net-tie; DRC: keep-out between pours. ERC will need to accept two power-ground nets.

## 2. Connectors and pinouts

### J1: Pi 40-pin header, low-profile female 2×20, 2.54 mm (plan of record, head_grams_v1 l.124)
| Pin | BCM | Net |
|---|---|---|
| 2, 4 | — | 5V_PI_HOLD (LTC3226 VOUT, wide pour to both, Atlas layout rule) |
| 1, 17 | — | 3V3_PI (Pi regulator out; used only for pull-ups + BNO055 3V3 to neck) |
| 3 / 5 | 2 / 3 | I2C_SDA / I2C_SCL → J3 |
| 7 | 4 | SD_MODE_GPIO → R9 (100 Ω) → SD_MODE (R10 0 Ω fitted) |
| 36 | 16 | SD_MODE_ALT → R11 (0 Ω **DNP**) → SD_MODE |
| 8 / 10 | 14 / 15 | UART_TX / UART_RX → TP1 / TP2 only (BNO055 UART fallback; no neck pin in v1) |
| 11 | 17 | SW_EN → J3 (e-stop / servo-power switch gate; trunk pull-down = OFF) |
| 12 | 18 | I2S_BCLK_GPIO → R5 (100 Ω) → I2S_BCLK |
| 35 | 19 | I2S_LRCLK_GPIO → R6 (100 Ω) → I2S_LRCLK |
| 38 | 20 | I2S_MIC_DATA ← R7 (100 Ω) ← mic DATA |
| 40 | 21 | I2S_AMP_DIN_GPIO → R8 (100 Ω) → I2S_AMP_DIN |
| 13 / 15 | 27 / 22 | FOOT_R_GPIO / FOOT_L_GPIO ← RC filter ← J3 |
| 16 / 18 | 23 / 24 | EYE_L / EYE_R → J8 (**driver open**, see §5) |
| 22 | 25 | PROJ_EN → J3 (FET + 100 kΩ gate pull-down are on the trunk, projector_v1) |
| 29 | 5 | CAPGOOD ← U1 CAPGOOD |
| 31 | 6 | PFO_N ← U1 PFO (open-drain, BOM line 10) |
| 32 / 33 | 12 / 13 | SG90_L_PWM / SG90_R_PWM → R12 / R13 (1 kΩ) → J5 / J6 |
| 37 | 26 | INA226_ALERT_N ← J3 (open-drain, pull-up here) |
| 27 / 28 | 0 / 1 | ID_SD / ID_SC → EEPROM pads only, **not fitted in v1** |
| 19, 21, 23, 24, 26 | 10, 9, 11, 8, 7 | NC, kept free (SPI0, future display/eye driver) |

### J2: neck power entry, Molex Micro-Fit 3.0, 2×2 (harness_neck_v1 l.21)
| Pin | Net | Wire | Load (harness_neck_v1) |
|---|---|---|---|
| 1 | 5V_PI_IN | 18 AWG | 2.16 A |
| 2 | GND_PI | 18 AWG | 2.16 A |
| 3 | 5V_AUX | 20 AWG | 2.76 A peak (3.01 A bench-only peak) |
| 4 | GND_AUX | 20 AWG | 2.76 A |
Per-pin ≤ 2.76 A design. Terminal rating and an 18 AWG crimp: **confirm at schematic** (not read from the Molex datasheet). Strain relief on the trunk side of the neck pitch joint.

### J3: neck signal connector, JST-GH 12-pin (Atlas + Chief ack 2026-11-24): 11 used, 1 spare
Order chosen by Woz: each I²C line next to its own return; slow GPIOs grouped; SW_EN away from the I²C clock.
| Pin | Net | Notes |
|---|---|---|
| 1 | I2C_SDA | twisted with 2 |
| 2 | GND_PI (SDA return) | |
| 3 | I2C_SCL | twisted with 4 |
| 4 | GND_PI (SCL return) | |
| 5 | 3V3_PI | BNO055 supply, ~12 mA |
| 6 | INA226_ALERT_N | open-drain from trunk |
| 7 | FOOT_L | to SS-10 left pair |
| 8 | FOOT_R | to SS-10 right pair |
| 9 | GND_PI (foot GND) | |
| 10 | PROJ_EN | GPIO25 |
| 11 | SW_EN | GPIO17; broken wire = servos off |
| 12 | SPARE | NC, routed to TP3 |
Neck total = 4 (J2) + 11 (J3) + 3 (STS bus, own Feetech lead) = **18**, matching the 1,000-cycle flex test (harness_neck_v1 l.46).

### J4: supercap stack, 2-pin (only if caps-to-trunk is chosen; footprint both options, pick at freeze)
SCAP_TOP, GND_PI. Default (caps on HAT): C10/C11 fitted, J4 DNP.

### J5 / J6: SG90 left / right, 3-pin 2.54 mm (SG90 order: PWM, +, −)
PWM via R12/R13; 5V_AUX; GND_AUX.

### J7: amp breakout (MAX98357A, off-board, right wall; harness cond. 2)
5V_AUX, GND_AUX, I2S_BCLK, I2S_LRCLK, I2S_AMP_DIN, SD_MODE. 6 pins, JST-SH or GH (pick at schematic). The amp's logic reference is GND_AUX; its ground joins GND_PI only in the trunk. **Flag for Atlas:** I²S edges then return through the trunk loop, ~1 m. The 100 Ω series resistors slow the edges; bench check: no I²S errors and no audible clicks at 48 kHz.

### J8: mic (INMP441, BOM line 22, left wall)
3V3_PI, GND_PI, I2S_BCLK, I2S_LRCLK, I2S_MIC_DATA, L/R (tie to GND_PI on the module = left). 6 pins.

### J9: eye LEDs. Pinout open until the eye-LED driver is chosen (§5).

## 3. Power-path netlist (U1 = ADI LTC3226EUD, 16-QFN 3×3; values from BOM line 10 + harness_neck_v1 l.67–72)
| Net / node | Parts | Value | Basis |
|---|---|---|---|
| 5V_PI_IN → U1 VIN | C1, C2 | 10 µF + 100 nF, X5R 10 V 0603/0805 | decoupling, Woz default |
| U1 PROG | R1 | 33.2 kΩ 1% (E96 nearest 33.3 kΩ) | 315 mA charge (BOM line 10) |
| PFI divider from 5V_PI_IN | R2 (top) / R3 (bottom) | **324 kΩ / 111 kΩ 0.1%** → 1.2 V × (1 + 324/111) = **4.703 V** falling trip. Spread from VPFI(TH) 1.175–1.225 V: **4.605–4.801 V**; PFI leakage ±50 nA × 324 kΩ = **±16 mV**; rising trip ≈ +78 mV (20 mV hyst × 3.919) | 4.70 V nominal (harness l.71); VPFI(TH) 1.175/1.2/1.225 V, hyst 20 mV, IPFI(LEAK) ±50 nA [3226f] |
| LDO_FB divider from VOUT | R4 (top) / R4b (bottom) | **459 kΩ / 102 kΩ 0.1%** → 0.8 V × (1 + 459/102) = **4.400 V**. Spread from VLDO_FB 0.76–0.82 V: **4.18–4.51 V**; leakage ±60 nA × 459 kΩ = **±28 mV** | 4.40 V backup LDO (harness l.70); VLDO_FB 0.76/0.8/0.82 V, ILDO_FB(LEAK) ±60 nA [3226f] |
| CPO_FB divider | R14 (top) / R15 (bottom) | **1.00 MΩ / 301 kΩ 0.1%** → 1.21 V × (1 + 1000/301) = **5.23 V** stack (spread 5.10–5.36 V from 1.18–1.24 V; leakage ±50 nA × 1 MΩ = ±50 mV). CAPGOOD rises at 1.11 V × 4.322 = **4.80 V** (4.71–4.88 V) = 92% of stack. Divider drain ≈ 4 µA | VCPO_FB sleep 1.18/1.21/1.24 V, CAPGOOD rising 1.09/1.11/1.13 V, ICPO_FB ±50 nA, VSTACK typ 5.3 / max 5.5 V [3226f]. Woz choice: 5.23 V, not 5.3 V, so the worst-case stack (5.36 V + 50 mV) stays under VSTACK max 5.5 V; 1 MΩ top (not the datasheet example 3.83M/1.21M) to cut leakage error |
| SCAP stack | C10, C11 | 2× Maxwell BCAP0010P270S01 in series (5 F, ~60 mΩ) | BOM line 10 |
| U1 VOUT → 5V_PI_HOLD | C3, C3b, C4 | **2× 47 µF X5R ≥10 V 1206 + 100 nF** | VOUT needs ≥ 47 µF low-ESR ceramic [3226f pin 1]. Two parts because X5R loses capacitance under 5 V DC bias; effective C at 5 V to be checked on the chosen part's DC-bias curve (PN TBD) |
| PFO_N | R16 | 10 kΩ to 3V3_PI | open-drain, 1 µA leak @ 5 V [3226f] |
| CAPGOOD | R17 | 10 kΩ to 3V3_PI | open-drain, VOL 65 mV @ 5 mA, 1 µA leak @ 5 V [3226f]; 0.33 mA sink, OK |
| INA226_ALERT_N | R18 | 10 kΩ to 3V3_PI | harness line-24 cond. 4 |
| SD_MODE | R10 → R19 | 0 Ω fitted; **1.0 kΩ to GND_AUX on amp side** | scope rev 29f0387 (boot ≈ 0.07 V < 0.16 V) |
| FOOT_L / FOOT_R | R20/C20, R21/C21 | 1 kΩ + 100 nF to GND_PI, at the Pi | harness l.13; pull-up = Pi internal (assumed, Atlas to confirm) |
| I2C pull-ups | — | **none on HAT** | Adafruit BNO055 breakout carries its own; confirm at capture to avoid a double pull-up |

**Datasheet basis:** every value tagged [3226f] is from LTC3226 datasheet revision **3226f** (original, not Rev A 3226fa), file `shared/datasheets/3226f_distrelec.pdf` (sha256 c9e37b5d…2e82ca, media.distrelec.com), re-read by Woz 2026-11-24. Re-check against 3226fa if analog.com becomes reachable.

**PFI error budget:** threshold spread (±25 mV at the pin = ±98 mV at 5V_PI_IN) dominates; leakage adds ±16 mV; 0.1% resistors add < ±10 mV. Worst-case falling trip **4.59–4.82 V**. Open for Atlas: the upper bound sits close to a sagging 5 V rail; confirm the 5V_PI_IN minimum under load is > 4.82 V + margin, or the HAT drops to backup on load steps.

**LDO vs PFI:** backup output max 4.51 V + 28 mV is below the PFI minimum trip 4.59 V. LDO dropout from CPO not read this run.

**SD_MODE ground note:** the 1.0 kΩ pull-down returns to GND_AUX (amp side, scope l.17 "on the amp side"), so the GPIO4 path crosses domains through R9. That is a signal, not a ground join, and does not break cond. 1.

## 4. Part list (draft; merges into `bom/bom_v1.0_delta.csv` at schematic freeze, no shadow spreadsheet)
Every line needs an alt before freeze. PNs marked **TBD** have not been looked up; none is invented.

| Ref | Part | Qty | Primary | Alt | Status |
|---|---|---|---|---|---|
| U1 | Supercap charger + backup LDO + PFI | 1 | ADI LTC3226EUD#PBF (Mouser USD 8.80 @1 / 5.78 @100, BOM line 10) | **No drop-in alt.** Chief ruling 2026-11-24: accepted single-source for v1, **2 spares per build kit**; caps-to-trunk stays off the table (needs Woz + Atlas sign-off) | ruled |
| C10, C11 | 10 F 2.7 V EDLC | 2 | Maxwell BCAP0010P270S01 | Maxwell BCAP0005P270S01 (5 F, rejected for hold-up, BOM line 10); other-vendor 10 F 2.7 V cell **TBD** | price not found |
| J1 | Low-profile 2×20 female, 2.54 mm | 1 | **TBD** | **TBD** | part not chosen |
| J2 | Micro-Fit 3.0 2×2 header + housing + crimps | 1 set | Molex Micro-Fit 3.0 (PN **TBD**) | **TBD** | rating check |
| J3 | JST-GH 12-pin, SMD side entry + GHR-12V-S housing + crimps | 1 set | JST SM12B-GHS-TB (series naming; confirm PN) | none mating-compatible: single source, buy spare | Atlas + Chief acked 12-pin |
| J4 | 2-pin cap connector | 0/1 | **TBD** | **TBD** | DNP by default |
| J5, J6 | 1×3 2.54 mm male header | 2 | generic | generic | — |
| J7, J8 | 6-pin wire-to-board | 2 | JST SH/GH (**TBD**) | **TBD** | — |
| R5–R9 | 100 Ω 1% 0603 | 5 | YAGEO RC0603FR series | UNI-ROYAL 0603WAF series | — |
| R12, R13, R19, R20, R21 | 1.0 kΩ 1% 0603 | 5 | YAGEO RC0603FR series | UNI-ROYAL 0603WAF series | — |
| R16–R18 | 10 kΩ 1% 0603 | 3 | YAGEO RC0603FR series | UNI-ROYAL 0603WAF series | — |
| R2, R3, R4, R4b | 324k / 111k / 459k / 102k, 0.1% 25 ppm 0603 | 4 | **TBD** (0.1% thin film) | **TBD** | — |
| R1 | 33.2 kΩ 1% 0603 | 1 | YAGEO RC0603FR series | UNI-ROYAL 0603WAF series | — |
| R10, R11 | 0 Ω 0603 | 2 (R11 DNP) | generic | generic | — |
| R14, R15 | CPO_FB divider | 2 | 1.00 MΩ / 301 kΩ 0.1% 0603 (PN TBD) | — | values set [3226f] |
| C1 | 10 µF X5R 10 V | 1 | **TBD** (MLCC) | **TBD** | DC-bias derating check |
| C3, C3b | 47 µF X5R ≥10 V 1206, low ESR (VOUT ≥ 47 µF [3226f]) | 2 | **TBD** (MLCC) | **TBD** | effective C at 5 V bias must stay ≥ 47 µF total |
| C2, C4 | 100 nF X7R 0603 | 2 | **TBD** | **TBD** | — |
| C20, C21 | 100 nF X7R 0603 | 2 | **TBD** | **TBD** | — |
| TP1–TP3 | test pads | 3 | — | — | — |
| PCB | 65 × 30 mm, 2-layer, 1.0 mm FR4, 4× M2.5 on 58 × 23 mm | 1 | JLCPCB | **TBD** second fab | outline re-check vs official Zero drawing |

## 5. Open items before capture (owner → due, Paris)
1. KiCad 9 on the Team Computer **or** razer access: Naoufel (Chief asked 12:17) → blocks capture + ERC.
2. Eye-LED driver for GPIO23/24 (topology, current, J9 pinout): Luxo/Woz → not on any doc yet.
3. ~~LTC3226 datasheet re-read~~ done from rev 3226f (2026-11-24), see §3. Remaining: 5V_PI_IN minimum under load vs the 4.82 V PFI upper bound → Atlas; 47 µF part DC-bias check → Woz at capture.
4. Foot inputs: confirm Pi internal pull-up and active level → Atlas.
5. I²S returning via GND_AUX through the trunk (J7): Atlas to ack or ask for a change → before capture.
6. ~~LTC3226 single source~~ ruled by Chief 2026-11-24: single-source for v1, 2 spares per build kit.
7. Fasteners: HAT to Zero standoffs M2.5; torque spec **not yet defined** for M2.5 (only M2 is in the spec) → Woz, before mechanical green.

## 6. Done-condition (unchanged from scope)
Pin map accepted by Chief + Atlas, **ERC clean**, DRC at JLCPCB 2-layer rules, BOM lines with an alt each, 3D clearance against head_layout_v1. No board ordered without Naoufel's yes.
