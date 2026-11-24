# Number reconciliation for Chief (Woz 2026-11-24 10:55 Paris, PROPOSED)

Paper only. No parts in hand (Naoufel 2026-11-23). Sources are named per line; "est." = Woz estimate, not a datasheet value.

## 1. 5 V peak 4.4 A vs 4.7 A, and clamp 1.0 A vs 1.3 A

**5 V rail.** Both figures are superseded UBEC-era sums (servo_candidates_v1 "UBEC sizing", 2026-11-22 22:50):
- 4.4 A = Pi 1.5 + LTC3226 0.36 + 2× SG90 1.5 + MAX98357A **1.0 A (my early guess)**.
- 4.7 A = the same sum with the amp at its derived 0 dBFS peak, **1.27 A** (9 dB gain, 3.59 Vrms into 4 Ω; head_layout_v1 "Audio driver"): 4.4 − 1.0 + 1.27 = 4.67 A.
- Neither has the camera (0.30 A budget), eyes, projector or Jewel 7.

**Canonical now (projector_v1, TPS56637 output):** normal **4.59 A** (amp capped −12 dBFS while SG90 moves = 0.32 A); design/validation **4.9 A** (TPS56637 bench condition); bench case 0 dBFS + SG90 moving **5.53 A = forbidden**, firmware interlock (Chief 18:34). Pack side for the BMS sum: 4.9 A × 5 V / 0.913 / 6.6 V = **4.1 A** (BOM line 8, 01:45).

**Amp current.** The MAX98357A has no current-limit setting. Its current is set by the softvol cap: 0.64 A at −6 dBFS, 0.32 A at −12 dBFS, 1.27 A only with the cap bypassed. I have not read the MAX98357A datasheet's internal over-current threshold, so I don't quote one.

**Servo clamp 1.0 vs 1.3 A.** 1.3 A was my 01:45 proposal on the 4 non-swapped leg servos (hip yaw + hip roll ×2), worst case 25.1 A. **Chief decided 1.0 A at 02:22** on every unit (worst case 23.9 A), implemented as EEPROM Max_Torque_Limit 357 + per-tick SRAM duty rewrite + Protection_Current backstop ≈ 1.4 A (servo_candidates_v1 "1.0 A clamp"). 1.3 A is dead. The 1.4 A figure is the trip backstop above the clamp, not a clamp.

## 2. BMS worst case 26 A vs 30.9 A

Both superseded by the one consistent set in BOM line 8 (2026-11-23 01:45). They differed only in assumptions:
- 26 A = only hip roll near stall + 5 V rail 4.4 A, regulator input taken at 7.4 V.
- 30.9 A = all 4 non-swapped leg servos at stall + 4.7 A rail, input at 6.6 V sag.

Consistent set (servo currents at 8.4 V full pack, regulator input at 6.6 V): 6 swapped leg servos clamped 2.5 A = 15.0 A; 4 other leg servos; 4 head servos idle 0.8 A; regulator input 4.1 A.

| Case | Total | Verdict |
|---|---|---|
| Typical | 20.7 A | OK |
| Other 4 at stall, unclamped (2.8 A est. each) | 31.1 A | FAIL, over P30B 30 A |
| Other 4 clamped 1.3 A (dead) | 25.1 A | superseded |
| **Other 4 clamped 1.0 A (DECIDED)** | **23.9 A** | BMS trip 28 A nominal, ±10 % low corner 25.2 A, 1.3 A margin |

Separate bound, not a BMS case: **39.2 A all-14-at-stall** (14 × 2.8 A est. + 3.26 A regulator input at full pack) sizes copper and the 40 A fuse only (BOM line 25, db191ee), for the clamp-failed case before the BMS delay expires.

## 3. Pololu D36V50F5 (bring-up only, harness_neck_v1: the policy never runs on it)

Read from [pololu.com/product/4091](https://www.pololu.com/product/4091) on 2026-11-24 09:54 UTC, graphs read by eye (±0.2 A / ±1 %):
- **Price: US$44.95 at qty 1. 0 in stock, status "Rationed", backorder only.**
- Max continuous output, Vout 5 V, room temp, still air: ≈ 7.4 A at 6.5 V in (lowest plotted point), ≈ 8.2 A at 12 V. Our 6.0–8.4 V pack is at the left end: **≈ 7.4–7.7 A**, above our 4.9 A design load.
- Efficiency: the Vin = 6 V curve sits at ≈ 96 % from 1 to ≈ 2.9 A, where the plot ends (no 6 V data above 2.9 A). The 12 V curve is ≈ 95 % at 5 A.
- Dropout, Vout 5 V: ≈ 0.85 V at 1 A, ≈ 1.0 V at 5 A → needs ≥ 6.0 V in at 4.9 A, same floor as our 6.0 V latched undervoltage alert.
- Datasheet: Vin 5.5–50 V, 5 V ±4 %, so up to 5.20 V. Under the Pi's 5.25 V, but it does not meet Atlas's 5.04–5.16 V TPS56637 window. Bring-up use only, as already decided.
- Header pins rated 3 A each: solder wires direct for > 3 A.

## 4. Onshape mass

I cannot open Onshape mass properties (no Onshape account/API key on this computer). What I have is the onshape-to-robot export of doc 64074dfcfa379b37d8a47762 (upstream robot.urdf, servo_candidates_v1 "Leg mass breakdown"): **model total 2.0615 kg**, trunk 698.5 g; per leg hip_roll 66.5, roll_to_pitch 75.2, thigh 124.1, shin 72.6, foot 75.2 g. Playground MJCF @ f698836 (Chief): **2.107 kg**; with the shoulder projector 2.115 kg est. The 45.5 g gap between URDF and MJCF is consistent with the 45.6 g expression package that exists only in Playground (head_layout_v1). If Chief needs Onshape's own figure, someone with Onshape access has to run Mass properties on the assembly.

## 5. Knee thermal bands (STS3215-C001, RMS at the output shaft)

The old text left 0.6–0.7 Nm undefined. Closed here, my call (est.; FEETECH publishes no derating curve):

| RMS torque | Band |
|---|---|
| ≤ 0.49 Nm | OK (pinned rated 5 kg·cm) |
| 0.49–0.60 Nm | Only with case ≤ 50 °C monitoring + firmware cutback |
| > 0.60 Nm | **Reject** for design (was "> 0.7"; the gap is closed downward) |

STS3046, same ratios to rated 1.30 Nm: ≤ 1.30 OK, 1.30–1.59 monitored, > 1.59 reject.
Hinton's reference walk knee RMS 1.60 / 1.76 Nm (sim) is ≈ 3× the STS3215 reject line and just over the STS3046 reject line. On paper neither servo holds that walk's knee RMS continuously; the knee case-temperature bench log decides. Thermal limit: servo case 50 °C under load.

Battery safety (unchanged): 2S P30B, 30 A cell ceiling, BMS trip 28 A nominal, battery bay is a fire-containment zone.
