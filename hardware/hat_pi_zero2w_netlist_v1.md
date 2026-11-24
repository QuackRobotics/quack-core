# Zero Robot HAT v1 (Pi Zero 2 W): netlist + part list, TEXT DRAFT (Woz 2026-11-24 12:30 Paris)

Status: **DRAFT, not a schematic.** KiCad 9 is not installed on the Team Computer and razer is unreachable (Chief 12:17), so no `.kicad_sch` is written and **no ERC has been run**. This file is the input for schematic capture once KiCad is available. Source of every net: `hat_pi_zero2w_scope_v1.md` (scope, Atlas ack 1543092, SD_MODE rev 29f0387), `harness_neck_v1.md`, `projector_v1.md`, `bom/bom_v1.0_delta.csv` line 10. Nothing here is ordered.

Conventions: refdes are provisional. `J` = connector, `R` = resistor, `C` = capacitor, `U` = IC, `TP` = test pad. Header pins = Pi 40-pin physical numbers, BCM names.

## 1. Ground domains (Atlas harness cond. 1 + 2)

- **GND_PI**: Pi ground. Header pins 6, 9, 14, 20, 25, 30, 34, 39; neck I²C twist GNDs; foot GND; LTC3226 GND.
- **GND_AUX**: SG90s + MAX98357A return only. Enters on J2 pin 4, goes to J5/J6/J7 only.
- **GND_PI and GND_AUX never join on this board** (they join only at the TPS56637 output in the trunk). Schematic: two separate ground symbols, no net-tie; DRC: keep-out between pours. ERC will need to accept two power-ground nets.
- **Amp I²S/SD_MODE return: ruled, isolated at J7 (Woz proposal 12:40, Atlas ack via Chief 2026-11-24).** U3 = TI **ISO7740FDBQR** (4/0 channels, F = default-LOW) on BCLK, LRCLK, DIN, SD_MODE. Side 1 = 3V3_PI/GND_PI, side 2 = 5V_AUX/GND_AUX, so no I²S edge returns through the trunk and GND_PI/GND_AUX still never join on this board. Check in §3 "U3 isolator check" [SLLSEP4K]. A second GND_PI–GND_AUX tie on the HAT stays rejected (loop; ~60% of AUX return current onto the 18 AWG GND_PI wire). Amp on 5V_PI stays rejected (Atlas cond. 2).

## 2. Connectors and pinouts

### J1: Pi 40-pin header, low-profile female 2×20, 2.54 mm (plan of record, head_grams_v1 l.124)

| Pin | BCM | Net |
|---|---|---|
| 2, 4 | — | 5V_PI_HOLD (LTC3226 VOUT, wide pour to both, Atlas layout rule) |
| 1, 17 | — | 3V3_PI (Pi regulator out; used only for pull-ups + BNO055 3V3 to neck) |
| 3 / 5 | 2 / 3 | I2C_SDA / I2C_SCL → J3 |
| 7 | 4 | SD_MODE_GPIO → R9 (100 Ω) → SD_MODE_PI (R10 0 Ω fitted) → U3 IND; R35 10 kΩ SD_MODE_PI → GND_PI |
| 36 | 16 | SD_MODE_ALT → R11 (0 Ω **DNP**) → SD_MODE_PI |
| 8 / 10 | 14 / 15 | UART_TX / UART_RX → TP1 / TP2 only (BNO055 UART fallback; no neck pin in v1) |
| 11 | 17 | SW_EN → J3 (e-stop / servo-power switch gate; trunk pull-down = OFF) |
| 12 | 18 | I2S_BCLK_GPIO → R5 (100 Ω) → I2S_BCLK |
| 35 | 19 | I2S_LRCLK_GPIO → R6 (100 Ω) → I2S_LRCLK |
| 38 | 20 | I2S_MIC_DATA ← R7 (100 Ω) ← mic DATA |
| 40 | 21 | I2S_AMP_DIN_GPIO → R8 (100 Ω) → I2S_AMP_DIN |
| 13 / 15 | 27 / 22 | FOOT_R_GPIO / FOOT_L_GPIO ← RC filter ← J3 |
| 16 / 18 | 23 / 24 | EYE_L / EYE_R → R28/R29 (0 Ω fitted) → R22/R23 (100 Ω) → Q1/Q2 gate (low-side N-FET), see J9 |
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

**8-pin JST-GH (1.25 mm), AWG26, equal-length power pairs** (Woz proposal; Atlas ack on current + Chief GO 2026-11-24 14:10). All four signals are U3 side-2 outputs, 5 V logic referenced to GND_AUX. The amp's logic reference is GND_AUX and its I²S edges return on GND_AUX in the J7 cable, not through the trunk loop.

| Pin | Net |
|---|---|
| 1 | 5V_AUX |
| 2 | 5V_AUX |
| 3 | GND_AUX |
| 4 | AMP_BCLK |
| 5 | GND_AUX |
| 6 | AMP_LRCLK |
| 7 | AMP_DIN |
| 8 | AMP_SD_MODE |

Pin order is a Woz choice (BCLK, the fastest edge, sits between two GND_AUX returns); confirm at schematic.

- **Contact rating:** JST GH 1.0 A per contact with AWG26; contact resistance 30 mΩ max initial, 50 mΩ max after environmental tests [JST eGH, read 2026-11-24; local copy lost in a Team Computer reset, re-download to razer pending].
- **Amp draw** (head_layout_v1 audio-driver table, Woz calc, η = 0.85 assumed, sine): −6 dBFS max_volume 0.19 A average / 0.38 A rail peak; 0 dBFS bench-only 0.75 A average / 1.51 A rail peak. One pair would pass at max_volume; two pairs are kept for margin.
- **Why two pairs:** 1.51 A bench peak split evenly = 0.76 A per contact; with one contact at 30 mΩ and its partner at 50 mΩ the split is 0.94 A against 1.0 A, so each pair's two wires must be the same length. A shorted-speaker fault is set by the MAX98357A current limit (ILIM 2.8 A typ, p.5 [19-6779 Rev 7]), not the contacts. The same datasheet (p.4, Absolute Maximum Ratings) limits continuous current in/out of VDD/GND/OUT_ to ±1.6 A, so a single 1.0 A pair would be the weak link before the IC is; another reason for two pairs (re-read 2026-11-24, sha256 abac92d3…403e8cd9).
- **Routing gate (Atlas condition):** open v1 DoD bench gate, see harness_neck_v1 "J7 harness bench-flex".
- Bench check unchanged: no I²S errors and no audible clicks at 48 kHz.

### J8: mic (INMP441, BOM line 22, left wall)

3V3_PI, GND_PI, I2S_BCLK, I2S_LRCLK, I2S_MIC_DATA, L/R (tie to GND_PI on the module = left). 6 pins.

### J9: eye LEDs, 3-pin (Luxo eye_anim_constraints @ 9770686 + Chief 2026-11-24: one plain single-colour Ø5 LED per eye, Jewel not in v1)

Pin 1 EYE_A (common anode, 5V_PI_HOLD) / pin 2 EYE_L_K / pin 3 EYE_R_K. Cathode → R24/R25 (**470 Ω 1%**, sized below) → Q1/Q2 drain; Q1/Q2 source GND_PI. Gate pull-down R26/R27 100 kΩ to GND_PI (eyes off while GPIO23/24 float at boot).

> **HARNESS WARNING: wire the eyes to the HAT, NOT the upstream way.** J9 is **common anode**: pin 1 = both LED anodes on 5V_PI_HOLD, pins 2/3 = the left/right cathodes into the low-side FETs. Upstream Open Duck (per Chief) puts the anodes on GPIO23/24 with a common cathode to GND. An eye harness built the upstream way will not light, and its common wire would land on 5V_PI_HOLD. QA: check the J9 pin 1 wire goes to both LED anodes (long leg, round side of the flange) before the first power-up.

**Eye LED: Cree C513A-MSN-CW0Z0231** (Chief 2026-11-24, from Luxo: warm white, target 5 mA, 10 mA hard ceiling). Datasheet read: Cree C513A-WSS/WSN/MSS/MSN, CLD-CT1042 REV 12, `shared/datasheets/cree_C513A_rev12.pdf` (sha256 8959e9f7…94cfb) [C513A rev12].

- Warm white, CCT min 2500 K / typ 2800 K (Luxo asked ~3000 K), 55° viewing angle, Ø5 round, MSN = no standoff, colour bins M2/M3 (I did not map M2/M3 to CCT) [C513A rev12].
- **Not diffused:** the datasheet never calls it diffused. Luxo's diffused look needs a diffuser in the eye shell (his call), or a diffused warm-white part from a datasheet someone has read. I could not get one: Kingbright's warm-white list has no Ø5 part, and the Mouser, Würth and Lighthouse pages were blocked.
- Abs. max IF 30 mA, PD 120 mW, VR 5 V. Vf at 20 mA: typ 3.2 / max 4.0 V, voltage bins 2.8–4.0 V, ±0.05 V tolerance [C513A rev12].
- **Vf at 5 mA (my estimate):** Fig. 1 (typical) reads about 2.85 V at 5 mA against 3.2 V at 20 mA. Shifting the bin limits by that −0.35 V gives **2.40–3.70 V at 5 mA**. That is an extrapolation along the typical curve, not a datasheet limit.

**R24/R25 = 470 Ω.** I = (5V_PI_HOLD − Vf) / 470. The FET drop at ≤ 6 mA is taken as 0 V, which errs high on current.

| Rail case (source) | 5V_PI_HOLD | Typ Vf 2.85 V | Min current (Vf 3.70 V) | Max current (Vf 2.40 V) |
|---|---|---|---|---|
| Normal (harness_neck_v1: HAT ≥ 4.99 V, regulator max 5.16 V) | 4.99–5.16 V | 4.8 mA at 5.10 V | **2.7 mA** at 4.99 V | **5.9 mA** at 5.16 V |
| Backup LDO ([3226f] 4.18–4.51 V, ±28 mV leakage) | 4.15–4.54 V | 3.3 mA at 4.40 V | **1.0 mA** at 4.15 V | **4.6 mA** at 4.54 V |

- Every corner stays under the 10 mA ceiling (worst 5.9 mA, 20% of the 30 mA abs. max). LED dissipation ≤ 5.9 mA × 3.7 V ≈ 22 mW. R dissipation ≤ 16 mW, fine for 0603.
- The eyes dim on backup, as Chief accepted. At the high-Vf corner they are very dim (1 mA).
- **QA at kitting:** the Vf bins span 1.2 V, so two random LEDs in one head can differ by about 2× in current. Pair each head's two LEDs by Vf at 5 mA, within 0.1 V, on the bench supply before they go in the harness.
- **Eye safety:** this is an indicator LED, at ≤ 5.9 mA through a 55° lens behind the eye shell. The datasheet's vision advisory says not to look at an exposed lamp in operation, so the LED is never exposed. I have not done an IEC 62471 assessment.

- **Why a transistor, not direct GPIO:** GPIO load stays at gate leakage only, LED current comes from 5 V not the Pi 3V3 rail, and v1.1 PWM can drive the same FETs. I did not read a Pi GPIO current-limit datasheet, so I am not sizing a direct-drive R against one.
- **v1:** on/off/blink on the 20 ms tick, plain GPIO writes.
- **v1.1 PWM, footprint reserved:** GPIO23/24 are not hardware-PWM pins and PWM/PCM are taken (antennas, I²S), so ≥ 8-bit at ≥ 2 kHz needs an external driver. U2 = I²C PWM/LED driver, **DNP**, on the neck I²C bus (3V3_PI, GND_PI); outputs → R30/R31 (0 Ω **DNP**) → Q1/Q2 gate; to switch, fit R30/R31 and remove R28/R29. U2 PN **TBD**; requirements: ≥ 2 channels, ≥ 8-bit (10-bit preferred), PWM ≥ 2 kHz, I²C address clear of the BNO055, 3.3 V. Not chosen: I have not read a candidate datasheet. From memory the PCA9685 tops out below 2 kHz, so it must be checked before it is considered.

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
| AMP_SD_MODE | U3 OUTD → R34 → J7; R19 | **R34 2.0 kΩ series** (datasheet-suggested ~2 kΩ to limit SD_MODE pin current [19-6779 p.17]); **R19 DNP** (a 1.0 kΩ pull-down would divide the 5 V high to ≈ 1.67 V, too close to B2 max 1.5 V); boot/fault low comes from U3 default-LOW. Side 1: **R35 10 kΩ SD_MODE_PI → GND_PI** | Woz 2026-11-24, see U3 isolator check |
| AMP_BCLK / AMP_LRCLK / AMP_DIN | R36 / R37 / R38 | **100 Ω** series at U3 OUTA/B/C, edge control on the J7 cable (same role as R5/R6/R8 on side 1) | Woz choice, confirm at bench |
| FOOT_L / FOOT_R | R32/R33, R20/R21, C20/C21 | **Active-low.** R32/R33 10 kΩ to 3V3_PI and C20/C21 100 nF to GND_PI at the GPIO; R20/R21 1 kΩ series from J3 (kept from harness l.13). Release τ = 11 kΩ × 100 nF ≈ 1.1 ms; press τ ≈ 0.09 ms; closed level 3.3 V × 1/11 = 0.30 V. Switch return J3 pin 9 = GND_PI, never GND_AUX | Atlas spec via Chief 2026-11-24 (GPIO22 = left, GPIO27 = right, 10 kΩ + 100 nF to GND_PI) |
| I2C pull-ups | — | **none on HAT** | Adafruit BNO055 breakout carries its own; confirm at capture to avoid a double pull-up |

**Datasheet basis:** every value tagged [3226f] is from LTC3226 datasheet revision **3226f** (original, not Rev A 3226fa), file `shared/datasheets/3226f_distrelec.pdf` (sha256 c9e37b5d…2e82ca, media.distrelec.com), re-read by Woz 2026-11-24. Re-check against 3226fa if analog.com becomes reachable.

**PFI error budget:** threshold spread (±25 mV at the pin = ±98 mV at 5V_PI_IN) dominates; leakage adds ±16 mV; 0.1% resistors add < ±10 mV. Worst-case falling trip **4.59–4.82 V**. Open for Atlas: the upper bound sits close to a sagging 5 V rail; confirm the 5V_PI_IN minimum under load is > 4.82 V + margin, or the HAT drops to backup on load steps.

**LDO vs PFI:** backup output max 4.51 V + 28 mV is below the PFI minimum trip 4.59 V. LDO dropout from CPO not read this run.

**SD_MODE margin flag [19-6779 Rev 7]:** MAX98357A datasheet 19-6779 Rev 7 (2/16), `shared/datasheets/MAX98357A-B_maxim_19-6779_rev7.pdf` (sha256 abac92d3…403e8cd9, from cdn-shop.adafruit.com), now read directly: SD_MODE B0 trip **0.08 / 0.16 / 0.355 V** (min/typ/max), internal pull-down RPD 92/100/108 kΩ, I²S VIH 1.3 V min, VIL 0.6 V max. With 1.0 kΩ the boot level is ≈ 0.07 V, guaranteed below B0 min, but only **10 mV** margin, and that rests on the 50 kΩ Pi pull-up assumption. Fix follows the ground decision: with the isolator, SD_MODE is driven by a default-low output referenced to GND_AUX and GPIO4 gets a 10 kΩ pull-down to GND_PI on side 1; without it, R19 drops to 470 Ω (≈ 0.033 V boot). Value not changed in this commit.

**SD_MODE ground note:** superseded by U3. GPIO4 now ends at U3 IND on side 1 (GND_PI); AMP_SD_MODE is driven from side 2 (GND_AUX). No signal crosses the two grounds except through the isolation barrier.

**U3 isolator check (Woz 2026-11-24) [SLLSEP4K] [19-6779 Rev 7].** Sources read: TI ISO774x datasheet SLLSEP4K (rev. August 2026), `shared/datasheets/ti_ISO7740_SLLSEP4K.pdf`; MAX98357A 19-6779 Rev 7 (above).

- Part: **ISO7740FDBQR**, QSOP/SSOP DBQ-16, orderable and Active, marking 7740F [SLLSEP4K §11]. Pins: VCC1 1, GND1 2/8, INA 3 ← I2S_BCLK, INB 4 ← I2S_LRCLK, INC 5 ← I2S_AMP_DIN, IND 6 ← SD_MODE_PI, NC 7, GND2 9/15, EN2 10 → 5V_AUX, OUTD 11, OUTC 12, OUTB 13, OUTA 14, VCC2 16. C22 100 nF VCC1–GND1, C23 100 nF VCC2–GND2 (datasheet: 0.1 µF each side).
- Speed: recommended DR ≤ 50 Mbps (100 Mbps max spec); I²S BCLK 3.072 MHz. 5 V tPLH/tPHL 6/12.7/17 ns, PWD ≤ 5.9 ns, same-direction skew tsk(o) ≤ 4 ns: < 2% of the 325 ns BCLK period, and BCLK/LRCLK/DIN share the package so they shift together. Meets ≥ 25 MHz.
- Supplies: VCC1/VCC2 2.25–5.5 V operating, 6 V abs max. Side 1 3V3_PI; side 2 5V_AUX ≤ 5.16 V (top of the normal-rail corner used in f57293f) → 0.34 V under 5.5 V.
- Default-LOW: F suffix outputs LOW when INx is open or VCC1 is unpowered; output undetermined only while VCCO is 1.7–2.25 V (5V_AUX ramp, amp is also below its supply range then). So: Pi boot/crash/3V3_PI loss → SD_MODE low → amp shutdown.
- Input levels: U3 VIH 0.7 × VCCI = 2.31 V, VIL 0.3 × VCCI = 0.99 V. GPIO4 boot: Pi default pull-up (50 kΩ, assumption already noted above, not read) vs R35 10 kΩ → 3.3 × 10/60 = **0.55 V < 0.99 V** → reads LOW. Pi GPIO VOH not read (BCM2710 data not read); bench check: 3.3 V high reaches ≥ 2.31 V at U3.
- Output into the amp: U3 VOH ≥ VCCO − 0.4 V at 4 mA, VOL ≤ 0.4 V at 4 mA (5 V table) vs amp VIH 1.3 V min / VIL 0.6 V max → ≥ 3.3 V and 0.2 V of margin.
- **MAX98357A abs-max at 5V_AUX:** BCLK, LRCLK, DIN to GND −0.3 V to +6 V (independent of VDD) → U3 output ≤ 5.16 V: **0.84 V margin**. SD_MODE ("all other pins") −0.3 V to VDD + 0.3 V, ±20 mA continuous input current. U3 VCC2 and amp VDD are the same 5V_AUX, but amp VDD sits below HAT 5V_AUX by the J7 cable drop while the amp draws current, and is 0 V if the J7 5V contact opens. R34 2.0 kΩ bounds the pin current: J7 5V open, (5.16 − ~0.6 V diode, assumed) / 2.0 kΩ ≈ 2.3 mA; any partial drop is less. Well under ±20 mA. Left-channel level with R34: 5.16 × RPD/(RPD + R34) ≥ 5.16 × 92/94 = **5.05 V > B2 max 1.5 V** (worst case VOH 4.76 V at VCCO 5.16 V still > 1.5 V). Shutdown: U3 VOL ≤ 0.4 V at 4 mA, at ~0.05 mA into RPD it is far lower; datasheet does not give VOL at that load, so **bench check: AMP_SD_MODE < 0.08 V (B0 min) at boot with GPIO4 low**.
- Not read: U3 price at qty 100 (LCSC C544482 search snippet says "from $0.9446", page not opened, so not a quote) and package mass (no mass in SLLSEP4K; DBQ-16 body is smaller than the 10.3 × 7.5 mm DW-16). Neither goes in the BOM until read.
- Alt: TI ISO6740F is functionally equivalent (50 Mbps, default-LOW F, 2.25–5.5 V) [SLLSFJ6G, read] but only in DW-16 (10.30 × 7.50 mm), so it costs board area; listed as the alt. ISO644x (named by TI as smaller) not read.

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
| J7 | JST-GH 8-pin, SMD side entry + GHR-08V-S housing + crimps, AWG26 | 1 set | JST SM08B-GHS-TB (series naming; confirm PN) | none mating-compatible: single source, buy spare | Atlas ack + Chief GO 2026-11-24 |
| J8 | 6-pin wire-to-board | 1 | JST SH/GH (**TBD**) | **TBD** | — |
| R5–R9 | 100 Ω 1% 0603 | 5 | YAGEO RC0603FR series | UNI-ROYAL 0603WAF series | — |
| R12, R13, R20, R21 | 1.0 kΩ 1% 0603 | 4 | YAGEO RC0603FR series | UNI-ROYAL 0603WAF series | — |
| R19 | 1.0 kΩ 1% 0603 | 0 (DNP) | YAGEO RC0603FR series | UNI-ROYAL 0603WAF series | footprint kept, DNP with U3 |
| R16–R18, R32, R33 | 10 kΩ 1% 0603 | 5 | YAGEO RC0603FR series | UNI-ROYAL 0603WAF series | — |
| R2, R3, R4, R4b | 324k / 111k / 459k / 102k, 0.1% 25 ppm 0603 | 4 | **TBD** (0.1% thin film) | **TBD** | — |
| R1 | 33.2 kΩ 1% 0603 | 1 | YAGEO RC0603FR series | UNI-ROYAL 0603WAF series | — |
| R10, R11, R28–R31 | 0 Ω 0603 | 6 (R11, R30, R31 DNP) | generic | generic | — |
| R22, R23 | 100 Ω 1% 0603 (FET gate) | 2 | YAGEO RC0603FR series | UNI-ROYAL 0603WAF series | — |
| R24, R25 | LED current R, 470 Ω 1% 0603 | 2 | 470 Ω 1% (PN **TBD**) | **TBD** | value set (f57293f) |
| R26, R27 | 100 kΩ 1% 0603 | 2 | YAGEO RC0603FR series | UNI-ROYAL 0603WAF series | — |
| Q1, Q2 | logic-level N-MOSFET, SOT-23 | 2 | **TBD** (2N7002 class; confirm R_DS(on) at V_GS = 3.3 V from the datasheet) | **TBD** | — |
| U2 | I²C PWM/LED driver | 0 (DNP) | **TBD** (≥ 2 kHz, ≥ 8-bit) | **TBD** | footprint only, v1.1 |
| U3 | 4-ch digital isolator, default-LOW | 1 | TI ISO7740FDBQR (DBQ-16) [SLLSEP4K] | TI ISO6740FDWR (DW-16, needs footprint change) [SLLSFJ6G] | price + mass not read |
| R34 | 2.0 kΩ 1% 0603 | 1 | YAGEO RC0603FR series | UNI-ROYAL 0603WAF series | — |
| R35 | 10 kΩ 1% 0603 | 1 | YAGEO RC0603FR series | UNI-ROYAL 0603WAF series | — |
| R36–R38 | 100 Ω 1% 0603 | 3 | YAGEO RC0603FR series | UNI-ROYAL 0603WAF series | — |
| C22, C23 | 100 nF X7R 0603 (U3 VCC1/VCC2) | 2 | **TBD** | **TBD** | — |
| J9 | 3-pin wire-to-board | 1 | JST SH/GH (**TBD**) | **TBD** | — |
| D_EYE | Ø5 warm-white TH LED, clear 55° | 2 (+2 kit spares) | Cree C513A-MSN-CW0Z0231 [C513A rev12] | **TBD** | off-board, in head; pair within 0.1 V at 5 mA (f57293f) |
| R14, R15 | CPO_FB divider | 2 | 1.00 MΩ / 301 kΩ 0.1% 0603 (PN TBD) | — | values set [3226f] |
| C1 | 10 µF X5R 10 V | 1 | **TBD** (MLCC) | **TBD** | DC-bias derating check |
| C3, C3b | 47 µF X5R ≥10 V 1206, low ESR (VOUT ≥ 47 µF [3226f]) | 2 | **TBD** (MLCC) | **TBD** | effective C at 5 V bias must stay ≥ 47 µF total |
| C2, C4 | 100 nF X7R 0603 | 2 | **TBD** | **TBD** | — |
| C20, C21 | 100 nF X7R 0603 | 2 | **TBD** | **TBD** | — |
| TP1–TP3 | test pads | 3 | — | — | — |
| PCB | 65 × 30 mm, 2-layer, 1.0 mm FR4, 4× M2.5 on 58 × 23 mm | 1 | JLCPCB | **TBD** second fab | outline re-check vs official Zero drawing |

## 5. Open items before capture (owner → due, Paris)

1. KiCad 9 on the Team Computer **or** razer access: Naoufel (Chief asked 12:17) → blocks capture + ERC.
2. ~~Eye-LED driver~~ set 2026-11-24 (J9 above). Remaining: LED colour/Vf and brightness → Luxo; U2 part → Woz at v1.1.
3. ~~LTC3226 datasheet re-read~~ done from rev 3226f (2026-11-24), see §3. Remaining: 5V_PI_IN minimum under load vs the 4.82 V PFI upper bound → Atlas; 47 µF part DC-bias check → Woz at capture.
4. ~~Foot inputs~~ set from Atlas's spec 2026-11-24 (§3). Atlas to confirm the kept 1 kΩ series R (press τ 0.09 ms).
5. ~~Amp I²S/SD_MODE return~~ ruled 2026-11-24: U3 ISO7740FDBQR isolator (§1, §3). J7 set to 8-pin GH, 2 power pairs (J7 above). Still open: U3 stock (LCSC 0; Mouser/DigiKey not read), J7 harness bench-flex gate, bench checks listed in the U3 check.
6. ~~LTC3226 single source~~ ruled by Chief 2026-11-24: single-source for v1, 2 spares per build kit.
7. Fasteners: HAT to Zero standoffs M2.5; torque spec **not yet defined** for M2.5 (only M2 is in the spec) → Woz, before mechanical green.

## 6. Done-condition (unchanged from scope)

Pin map accepted by Chief + Atlas, **ERC clean**, DRC at JLCPCB 2-layer rules, BOM lines with an alt each, 3D clearance against head_layout_v1. No board ordered without Naoufel's yes.
