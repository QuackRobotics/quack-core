# Stronger servo candidates — knee / hip pitch / ankle (Woz, 2026-11-22 22:50 Paris)

Status: **PROPOSED — not in the BOM.** Needs Woz + Atlas sign-off and a Hinton gate rerun.
Source of every datasheet number: the FEETECH wiki model pages (manufacturer tables, checked by FEETECH 2026-10-02).
Anything marked *est.* is my derivation (linear-in-voltage DC-motor scaling), not a datasheet value.

## Bottom line

**No FEETECH STS TTL servo meets all of Atlas's criteria on a 2S pack.** I screened every STS/ST model
in FEETECH's selector data (20 models). The near-miss is **STS3046 (ST-3046-C001)**: 3.92 Nm at 7.4 V
(criterion is 4.0 Nm) and its no-load speed is likely ~12 % below STS3215 at 7.4 V (est.).
The 12 V parts (STS3250, STS3235, ST-3025) only reach ~3 Nm or less on 2S (est.).
Everything stronger at 7.4 V (STS3095) is 195 g and 65 mm long.

Decision needed (Atlas / Quack Chief): (S1) accept STS3046 with criterion 1 relaxed to 3.9 Nm, gated by Hinton;
or (S2) go 3S for STS3250, which reopens the power sign-off and can't share a bus with STS3215-C001 (6–7.4 V).

## Screen against Atlas's criteria

| | STS3215-C001 (baseline) | **STS3046 (ST-3046-C001)** | STS3250 (ST-3250-C001) on 2S | STS3095 (ST-3095-C001) |
|---|---|---|---|---|
| Datasheet stall | 19.5 kg·cm **@6 V** (1.91 Nm) | **40 kg·cm @7.4 V = 3.92 Nm** | 50 kg·cm @12 V (4.90 Nm) | 95 kg·cm @7.4 V (9.3 Nm); 105 @8.4 V |
| (1) stall @7.4 V ≥4.0 Nm | ~2.36 Nm est. | **3.92 Nm — misses by 0.08** | ~3.0 Nm est. — FAIL | 9.3 Nm pass |
| (1) stall @6.6 V ≥3.5 Nm | 2.10 Nm est. | 3.50 Nm est. — borderline | 2.7 Nm est. — FAIL | pass |
| Rated torque | 6.5 kg·cm @6 V (0.64 Nm)\* | 13.3 kg·cm @7.4 V (1.30 Nm) | 16 kg·cm @12 V (~0.97 Nm @7.4 V est.) | 35 kg·cm @8.4 V |
| (2) no-load speed | 0.238 s/60° @6 V (~0.193 @7.4 V est.) | 0.22 s/60° @7.4 V — likely FAIL (~12 % slower, est.) | 0.133 s/60° @12 V (~0.216 @7.4 V est.) — FAIL | 0.25 s/60° @8.4 V — FAIL |
| (3) encoder / backlash | 12-bit magnetic / ≤0.5° | 12-bit magnetic / ≤0.5° | 12-bit magnetic / ≤0.5° | 12-bit magnetic / ≤0.5° |
| (4) bus | STS TTL, 38.4 k–1 Mbps, ID 0–253 | same family/baud/IDs | same family/baud/IDs | same family/baud/IDs |
| (4) register map | reference | **not published** ("memory table not supplied") — bench-verify the sync_read block | not published | not published |
| Stall current | 2.0 A @6 V | 3.1 A @7.4 V | 4.2 A @12 V (~2.6 A @7.4 V est.) | 10.5 A @8.4 V |
| No-load current | 150 mA @6 V | 300 mA @7.4 V | 280 mA @12 V | 800 mA @8.4 V |
| Mass | 55 g | 89 g (+34 g) | 74.5 g (+19.5 g) | 194.5 g (+139.5 g) |
| Size A×B×C | 45.2×24.7×35 mm | 40×20×43.05 mm (different body) | 45.22×24.72×35 mm (same body) | 30×65×48 mm |
| Spline / shaft | 25T / dual | 25T OD5.9 / **single** (a "-DS" double-shaft variant is listed by an Alibaba seller, not on the FEETECH page) | 25T OD5.9 / dual (retail listings) | 15T OD7.6 / dual |
| Gear ratio | 1:345 | **not published** | 1:345 (C002, WowRobo retail page — not datasheet) | not published |
| Motor | brushed iron-core | brushed coreless | brushed coreless | brushed iron-core |
| Voltage range | 6–7.4 V (website) | 6–7.4 V website / 8.4 V PDF (FEETECH flags the conflict) | 6–12 V table, but "9–12.6 V" headline | 6–12 V website / 9 V PDF |
| Price (1 pc, ex-ship/tariff) | €14 (upstream sheet) | ~US$63–65 (Alibaba "180-day lowest") ≈ €59–61 | US$43 (WowRobo) ≈ €40 | not checked (rejected) |

\* The FEETECH table gives STS3215-C001 numbers at **6 V** (19.5 kg·cm, 2.0 A stall, 6.5 kg·cm rated), while the product title says "7.4V 19kg". Our pinned 1.91 Nm / 0.49 Nm may therefore be conservative at 7.4 V. Not changing the gate on that; a bench stall test at 7.4 V settles it.

Rejected: STS3095 (mass, size, 15T spline, 10.5 A stall); STS3235 / STS3215-C018 / ST-3025-C001/C002 / ST-3035 (12 V class, ≤~2.4 Nm on 2S est.); ST-3120 / ST-3200 (210 g, 65 mm body, 9–12.6 V); ST-2000 / ST-3020 / ST-5420 (≤25 kg·cm @7.4–8.4 V); small 6 V models (ST-3009/3032/3036/3045).

## Mechanical impact

- **STS3046:** different body (40×20 vs 45.2×24.7, 8 mm taller). New knee, hip-pitch (and ankle) brackets/STLs; new bolt pattern; single output shaft means the back side needs an idler bearing in the bracket (or the unverified -DS variant). Same 25T spline, so horns carry over. M3×6 horn screw, same as STS3215.
- **STS3250:** same envelope and spline as STS3215 → drop-in brackets. Only helps if we go 3S.

## Do the ankles need it?

Yes, if we change anything. Ankles hit the stall cap on 4,247 / 2,253 steps, so they are torque-saturated on STS3215 too. Swapping knees + hip pitch alone moves the bottleneck to the ankles. My recommendation: swap 6 (2 knees, 2 hip pitch, 2 ankles); Hinton's gate tells us whether ankles can stay.

## MuJoCo inputs (STS3046)

From the datasheet: forcerange ±3.92 Nm (7.4 V); ±3.50 Nm (6.6 V, est.); mass 89 g; 4096 counts/rev.
Derived (est., linear motor model): no-load speed 60°/0.22 s = 4.76 rad/s; torque–speed slope τs/ω0 = 0.82 N·m·s/rad
(STS3215: 1.91/4.40 = 0.43 at 6 V); output torque constant τs/Is = 1.26 Nm/A (STS3215: 0.96 Nm/A at 6 V).
**Not published: gear ratio, rotor inertia → armature, friction.** These need bench sysID (no-load spin-down + stall sweep). Until then I'd keep the STS3215 armature scaled by mass and flag the result.
kp-equivalent: comes from firmware P gain, not the datasheet; keep the same firmware gains and change only forcerange/velocity.

## Hip pitch RMS thermal bands

- **STS3215-C001** (same servo as the knee, same bands): ≤0.49 Nm OK; 0.49–0.6 only with ≤50 °C case monitoring + firmware cutback; >0.7 reject. Upper bands are my estimate. Measured hip-pitch RMS 0.99 / 0.91 Nm = ~2× rated → **reject on STS3215**. The hip pitch servo sits inside the pelvis with less airflow than the knee, so I would not loosen these.
- **STS3046** (same ratios to rated 1.30 Nm): ≤1.30 OK; 1.30–1.59 with ≤50 °C monitoring + cutback; >1.86 reject (upper bands est.). 0.99 / 0.91 Nm sits inside OK. Caveat: coreless motors have little thermal mass; case-temperature monitoring stays mandatory.
- **STS3250 on 2S:** rated ~0.97 Nm est. → 0.99 is just over OK. Another reason it doesn't fit 2S.

## Battery / BMS per candidate (stand-up: 6 servos at stall)

Assumptions: 8 remaining STS3215 at ~0.2 A each = 1.6 A; UBEC input ~4.0 A from pack (see below).

| Candidate (6 swapped) | 6× stall @7.4 V | @8.4 V full pack | Total @8.4 V incl. rest + UBEC |
|---|---|---|---|
| STS3215 baseline | 14.8 A est. (2.47 A each) | 16.8 A est. | **~22 A** |
| STS3046 | 18.6 A (3.1 A datasheet) | 21.1 A est. | **~27 A** |
| STS3250 (2S) | 15.5 A est. | 17.6 A est. | ~23 A |

- **BMS trip floor:** the current 15 A floor is already below the baseline stand-up worst case (~22 A) once the UBEC load is counted. For STS3046 the floor must be **≥25 A**, and the per-servo firmware clamp is what keeps us under the P30B's 30 A. Final trip still set from the first-power-up measurement.
- Suggested firmware clamp for STS3046: 2.5 A per servo (≈3.2 Nm est.) → 6×2.5 + 5.6 = ~21 A. That is ≈3.15 Nm, just under the 3.23 Nm C-FIXED knee peak, so Hinton should gate at the clamped value, not the stall value.

## UBEC sizing (Atlas's line-10 question)

Simultaneous worst case on the 5 V rail: Pi Zero 2 W 1.5 A (my design figure) + LTC3226 input 0.36 A (datasheet limit) + 2 stalled SG90 (TowerPro datasheet gives no stall current; I budget 0.75 A each = 1.5 A) + MAX98357A peak (~1.0 A, my estimate for 3.2 W/4 Ω) = **~4.4 A**.
→ A 5 A UBEC is at 87 %; **specify the 8 A HENGE version** (upstream lists 4/6/8/12 A). Pack-side draw ≈ 4.4 × 5.1 V / 0.85 / 6.6 V ≈ 4.0 A.
Layout: LTC3226 VOUT → Pi via a short, wide pour to **both** 5 V header pins (2 and 4), ground return on pins 6/9, no thin traces or long jumper.

## Price / BOM delta vs €368.74 (estimates, ex shipping/tariff; USD→EUR at 1.07)

| Option | Δ per servo | 4 swapped (knee + hip pitch) | 6 swapped (+ ankles) |
|---|---|---|---|
| STS3046 | +€46 | +€184 → ~€553 | +€276 → ~€645 |
| STS3250 (needs 3S) | +€26 | +€105 → ~€474 | +€157 → ~€526 (before 3S pack/BMS/other-servo changes) |

Plus idler bearings for STS3046 brackets (unpriced) and the 8 A UBEC (~same price band; unpriced).

## Links

- [ST-3046-C001 (STS3046)](https://ftservo.github.io/ftservo-wiki/en/products/models/st-3046-c001/main/) · [FEETECH product page](https://www.feetech.cn/en/74v-40-kgcm-metal-shell-steel-teeth-360-degree-magnetic-code-single-shaft-ttl-serial-port-steering-gear.html)
- [ST-3215-C001 (STS3215)](https://ftservo.github.io/ftservo-wiki/en/products/models/st-3215-c001/main/)
- [ST-3250-C001 (STS3250)](https://ftservo.github.io/ftservo-wiki/en/products/models/st-3250-c001/main/)
- [ST-3095-C001 (STS3095)](https://ftservo.github.io/ftservo-wiki/en/products/models/st-3095-c001/main/)
- Prices: [Alibaba STS3046](https://www.alibaba.com/product-detail/Feetech-40KG-Servo-7-4V-360_1601869336896.html) · [WowRobo STS3250 C002](https://shop.wowrobo.com/products/feetech-sts3250-c002-servo-12v-50kg-1-345-servo) · upstream BOM sheet (STS3215 €14)

## Leg mass breakdown (Woz 2026-11-23, from upstream robot.urdf, onshape-to-robot export of Onshape doc 64074dfcfa379b37d8a47762)

Every servo housing (WJ-WK00-0122/0123/0124 case parts = STS3215 case) appears as geometry in the link it is bolted to, one per link, 14 total, so onshape-to-robot's link masses include the servos. Per-part masses were not opened in Onshape (no access).

| Link (per leg) | Mass (g) | Servo housing in it | Printed/other parts |
|---|---|---|---|
| hip_roll_assembly | 66.5 | hip roll servo | roll_motor_top/bottom, horns |
| roll_to_pitch_assembly | 75.2 | hip pitch servo | roll_to_pitch, horns |
| knee_and_ankle_assembly (thigh) | 124.1 | knee servo | cache, sheets, spacer, horns |
| knee_and_ankle_assembly_2 (shin) | 72.6 | ankle servo | sheets, spacer, horns |
| foot_assembly | 75.2 | none | foot PLA/TPU parts |

Trunk (698.5 g) holds 2 hip-yaw + neck-pitch servos. Model total 2.0615 kg. Conclusion: an STS3046 swap is +34 g per servo (replace), not +89 g; COM/inertia of the thigh, shin and roll_to_pitch links also change (different body shape).

## Continuous (rated) torque at 7.4 V (Woz, 2026-11-23 01:45 Paris)

| Model | FEETECH rated torque | At 7.4 V |
|---|---|---|
| STS3215-C001 | 6.5 kg·cm @6 V = 0.64 Nm | ~0.79 Nm *est.* |
| STS3046 (ST-3046-C001) | 13.3 kg·cm @7.4 V = **1.30 Nm** | 1.30 Nm (datasheet) |
| STS3250 (ST-3250-C001) | 16 kg·cm @12 V = 1.57 Nm | ~0.97 Nm *est.* |
| STS3095 (ST-3095-C001) | 35 kg·cm @8.4 V = 3.43 Nm | ~3.0 Nm *est.* |

The 1.30 Nm figure is FEETECH's own rated torque for ST-3046-C001 at 7.4 V (wiki model table), 13.3 × 0.0981.
FEETECH does not state a duty cycle for "rated", so these count as continuous only until a case-temperature
bench log confirms them. Hinton's reference walk (1.60 / 1.76 Nm knee RMS, sim, 5.2 Nm actuator cap) is above
every candidate except STS3095.

## STS3215-C001 current–torque line (Woz 2026-11-23 02:45, for Hinton / stand-up current bound)

FEETECH publishes **no current–torque curve** for the C001, only two points (FEETECH table, values at 6 V): no-load 150 mA, stall 2.0 A at 19.5 kg·cm (1.91 Nm). A brushed DC motor behind a fixed gearbox is linear in torque, so:

- **6 V:** I = 0.15 A + 0.97 A/Nm × τ (output-shaft Kt ≈ 1.03 Nm/A; τ_stall 1.91 Nm).
- **7.4 V (derived, not measured):** same slope; stall scales with voltage, ≈ 2.47 A at ≈ 2.36 Nm (resistance-limited). No-load current rises slightly with speed; keep 0.15–0.30 A as the band.
- At our pinned rated 0.49 Nm: ≈ 0.63 A. At the 1.0 A EEPROM clamp: ≈ 0.88 Nm available.
- Caveats: gear friction makes the real line slightly non-linear near no load; the firmware protection-current register can cap current below stall; temperature raises winding R (≈ +0.39 %/°C copper), lowering hot stall current.
- **Bench to replace this:** one C001 on a torque arm at 7.4 V, 6 load points 0–stall, log bus present-current vs torque. Until then Hinton should treat this as a model.

## 1.0 A clamp: which register (Woz 2026-11-23 09:10)

Source: FEETECH STS/SMS control table as used by LeRobot (`STS_SMS_SERIES_CONTROL_TABLE`, lerobot v0.3.2 `motors/feetech/tables.py`, citing FEETECH's SMS-STS e-manual). Not yet checked on our servo firmware.

| Register | Addr | Area | What it does |
|---|---|---|---|
| Max_Torque_Limit | 16 (2 B) | EEPROM | Power-on default output limit, 0–1000 = 0–100 % of **PWM duty**. Copied to Torque_Limit at boot. |
| Protection_Current | 28 (2 B) | EEPROM | **Over-current protection trip**, not a regulator: above it for Over_Current_Protection_Time (addr 38) the servo unloads (torque off). |
| Torque_Limit | 48 (2 B) | SRAM | Live duty limit, writable every tick. |
| Present_Voltage / Present_Current | 62 / 69 | SRAM | Read-back. |

**Consequence for the clamp (and for Hinton's low-V verdict):**
- Neither register is a true regulating current limit. Protection_Current used alone would turn the hip servos **off** at 1.0 A (a fall), so it cannot be "the clamp".
- The clamp is the **duty limit**. At stall the STS3215 behaves as ≈ 3.0 Ω (7.4 V / 2.47 A, same as 6 V / 2.0 A), so I_max ≈ duty × V_pack / 3.0 Ω. A fixed duty therefore gives a current (and torque) cap that **falls with pack voltage**: set for 1.0 A at 8.4 V (duty 35.7 %, value 357) it allows only 0.75 A at 6.3 V.
- **A fixed EEPROM duty clamp = Hinton's "literal −20 %" case, which FAILS.** The "clamp-kept" case (which passes by 0.0007 m/s) needs a constant current cap.

**Implementation I propose (both layers, needs Atlas ack):**
1. EEPROM Max_Torque_Limit (16) = **357** on the 4 clamped servos: 1.0 A at the 8.4 V full pack, so the cap is safe at power-on and if the controller dies (Atlas's EEPROM requirement).
2. Control loop rewrites SRAM Torque_Limit (48) each 20 ms tick: duty = 1.0 A × 3.0 Ω / V_pack (Present_Voltage, filtered), clamped to ≤ 1000, so the cap stays ≈ 1.0 A (≈ 0.88 Nm) at any pack voltage = Hinton's "clamp-kept".
3. EEPROM Protection_Current (28) as a backstop trip **above** the clamp (≈ 1.4 A; unit is reported as 6.5 mA/LSB → ≈ 215, to be bench-confirmed), so a stuck SRAM write cannot reach the BMS trip.
- Margin is 0.0007 m/s, i.e. none. A 3.0 Ω estimate that is 5 % off moves the cap by 5 %. Bench: one STS3215 on the torque arm at 6.3 / 7.4 / 8.4 V, log Present_Current at each Torque_Limit value, before this is signed.
- **Speed dependence of the clamped cap (corrected 2026-11-23).** Under a duty cap the cap falls with speed: τ ≈ 0.876 × (1 − ω/ω_c) Nm. ω_c ≈ **2.0 rad/s**: 1.99 if the 0.15 A no-load current is kept as a torque loss at both ends (5.78 × 3.0/7.4 × 0.85), or 2.02 with Chief's τ_cap/τ_stall × ω0 (0.88 × 5.43/2.36). That's a 1.5 % model difference. With the per-tick rescale ω_c is independent of V_pack. The 2.17 rad/s in an earlier Woz message is withdrawn: it was 0.40 × 5.43 and ignored the no-load current.

## 0.864 vs 0.876 Nm, and bench additions 1–3 (Woz 2026-11-23 10:40 Paris)

**Where the two figures come from.** Both use τ = Kt × (I − I0) with I = 1.0 A and I0 = 0.15 A, so I − I0 = 0.85 A. They differ only in Kt:
- **0.876 Nm:** Kt from the FEETECH datasheet point at 6 V: 1.91 Nm / (2.0 − 0.15) A = 1.032 Nm/A (the "0.97 A/Nm" slope). 1.032 × 0.85 = 0.877, quoted as 0.876 from (1.0 − 0.15) / 0.97.
- **0.864 Nm:** Kt from the *derived* 7.4 V point: 2.36 Nm / (2.47 − 0.15) A = 1.017 Nm/A. 1.017 × 0.85 = 0.864. Written with Kt rounded to 1.02 it is 1.02 × 0.85 = **0.867**, not 0.864. The 0.864 needs the unrounded 1.017.
- **Why they differ.** The 7.4 V point scales stall current and stall torque by 7.4/6 but keeps I0 at 0.15 A. If I0 is scaled too, Kt comes back to 1.032 and τ to 0.878. Kt is a motor constant and doesn't change with voltage, so **0.876 is the model-consistent value**. The 1.5 % gap is an artifact of the derived point, and it's the same 1.5 % as the ω_c 1.99 vs 2.02 above.
- **Which one is conservative.** 0.876 is conservative against the retracted 1.31 Nm, but **0.864 is 1.4 % lower still**. With a stated margin of 0.0007 m/s, that 1.4 % may flip the verdict. Recommendation: Hinton reports go/no-go at both 0.876 and 0.864. If they disagree, the decision waits for bench item 1. Neither number is measured.

**Bench additions 1–3 (approved by Chief):**
1. **Stall current + torque at 6.3 / 7.4 / 8.4 V.** One STS3215-C001 on the torque arm, ≥ 6 load points from 0 to stall at each voltage. Log Present_Current (addr 69) and an external shunt, and the torque from the arm's load cell. Output: Kt, I0 and stall R per voltage. This settles 0.864 vs 0.876, the 3.0 Ω stall figure behind the duty rescale, and the 2.8 A per-servo figure behind the 39.2 A sizing.
2. **8.4 V cold-start peak.** Full pack (8.4 V rest), cold servos, switch soft-start, then torque-enable all 14 servos in one broadcast at the stand-up pose. Scope pack current at the INA226 shunt (≥ 100 kS/s) and the switched rail. Pass: peak < 25.2 A (the BMS low corner), no BMS trip, and switched rail ≥ 6.3 V after 50 ms. Repeat 10×.
3. **Over_Current_Protection_Time (addr 38) behaviour.** Protection_Current (addr 28) at 215 counts (≈ 1.4 A if 6.5 mA/LSB holds). Sweep addr 38 over at least 3 values and stall the servo above 1.4 A. Record the time from crossing to unload, what Present_Current reads, whether the servo re-arms by itself, and the status/error bits. Also confirm that with Max_Torque_Limit 357 and the per-tick rescale active, a normal stand-up never trips addr 28.

## Option B servo (6 joints)

Option B (Naoufel 2026-11-22 22:32) = **STS3046 (ST-3046-C001)** on 2 knees, 2 hip pitch, 2 ankles. It is the only option-B candidate in this file. The 4 clamped joints (2 hip_yaw, 2 hip_roll) stay **stock STS3215-C001**, so Hinton's use of the STS3215 line there is correct. Added to the BOM as line 23, PROPOSED, not in the TOTAL. Status of line 2 stays SUPERSEDED-PENDING until Atlas/Chief decide B.

## Status 2026-11-23 09:55 (Woz)

- **Clamp register plan** (EEPROM 16 = 357, SRAM 48 rescaled every tick, Protection_Current 28 backstop): **still waiting for Atlas's ack.** Torque-arm bench at 6.3 / 7.4 / 8.4 V is needed before signing.
- **ω_c ≈ 2.0 rad/s** is the only speed-cap figure. The earlier 2.17 rad/s is withdrawn in 26f2173, so the Chief/Woz mismatch is closed.
- **Option B (line 23)** is still PROPOSED. The STS3046 unit price was not re-read today.

## Chief 09:44 questions: STS3046 speed rating and price (Woz 2026-11-23 10:15)

Labels: the doc's earlier decision labels are now **(S1) STS3046 on 2S** and **(S2) STS3250 on 3S**, so they no longer collide with Naoufel's "option B" (stronger knee/hip-pitch servos, no retrain).

**At what voltage is 0.22 s/60° rated? 7.4 V.** FEETECH's own table, mirrored on the [FEETECH wiki ST-3046-C001 page](https://ftservo.github.io/ftservo-wiki/en/products/models/st-3046-c001/main/) ("FEETECH official product page, checked 2026-10-02"): *No load speed 0.22 sec/60° @7.4V*, 300 mA no-load @7.4 V, 40 kg·cm stall @7.4 V, 3.1 A stall @7.4 V, operating range 6–7.4 V (the attached PDF says 4–8.4 V; FEETECH flags the conflict). [ZennixTek](https://www.zennixtek.com/products/feetech-sts3046-servo-motor) repeats 0.22 s/60° @7.4 V.
- 60°/0.22 s = **4.76 rad/s at 7.4 V**, which is the top of its rated range. Hinton's 4.76 rad/s is the right datasheet figure.
- STS3215-C001 at 7.4 V, 5.43 rad/s, is itself an **estimate** (0.238 s/60° @6 V datasheet, scaled linearly to 7.4 V). On the same basis the STS3046 is ~12 % slower at 7.4 V and, scaled down to 6.6 V, ~4.2 rad/s (est.).
- So the STS3046 trades speed for torque. If every Hinton torque–speed run fails the speed bar, **the STS3046 does not meet option B as stated (no retrain)** and I would not lock it. No other 2S FEETECH STS part in the screen above is both stronger and as fast. The 12 V parts need 3S (S2), which reopens the power sign-off. Decision: Chief/Naoufel, with Hinton's runs as the gate.

**Price** (qty-1 retail, USD, ex shipping/VAT/tariff, read 2026-11-23 from the stores' Shopify product JSON; USD→EUR at 1.07 as elsewhere in this file):

| Source | STS3046 unit | EUR |
|---|---|---|
| [ZennixTek](https://www.zennixtek.com/products/feetech-sts3046-servo-motor) | US$90.00 | €84.11 |
| [AIFITLAB](https://aifitlab.com/products/feetech-sts3046-servo-motor) / [Robotopian](https://robotopian.com/products/feetech-sts3046-servo-motor) | US$95.00 | €88.79 |
| Alibaba "180-day lowest" (earlier read, not re-read today: page returned empty) | ~US$63–65 | ~€59–61 |

| Set (unit €84.11, ZennixTek) | Servos | Servos € | Idler bearings (F693ZZ 3×8×4, 1 per joint + 1 spare) | Brackets (printed, new STLs) | STS3215 removed from line 2 (€14 each) | Net delta |
|---|---|---|---|---|---|---|
| ×4 (knee + hip pitch) + 1 spare | 5 | €420.55 | 5 pcs, **not priced** (listings found, no price captured) | new design, filament only, **not priced** | −4 → −€56 | **+€364.55** + bearings + brackets |
| ×6 (+ ankles) + 1 spare | 7 | €588.77 | 7 pcs, **not priced** | new design, **not priced** | −6 → −€84 | **+€504.77** + bearings + brackets |

At the Alibaba price (~€60) the deltas would be ~+€244 (×4+1) and ~+€336 (×6+1). The earlier "+€46/servo" row above used the Alibaba price and no spare. Bearings are a small cost in any case; the real cost of brackets is CAD time (new body 40×20×43.05 mm, single shaft, back-side idler), owner Woz, after Chief/Naoufel decide.

**Joint count is not settled.** Naoufel said knee + hip pitch (4); Hinton ran 6 (incl. ankles); my recommendation was 6 because the ankles saturate too. Chief/Naoufel decide.
