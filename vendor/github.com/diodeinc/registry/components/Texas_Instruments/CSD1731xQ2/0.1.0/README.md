# CSD1731xQ2 — TI 30 V N-channel NexFET, WSON-6 (DQK) 2x2 mm

Pin-compatible 30 V N-channel NexFET power MOSFETs sharing the TI DQK
2 × 2 mm WSON-6 body. Both dice are logic-level (optimized for 5 V gate
drive).

## Parts

| `current_rating` | MPN | Carrier | R_DS(on) max @ 8 V | V_GS(th) max |
|---|---|---|---|---|
| 5 A  | CSD17313Q2  | 3000 reel | 30 mΩ   | 1.8 V |
| 5 A  | CSD17313Q2T | 250 reel  | 30 mΩ   | 1.8 V |
| 25 A | CSD17318Q2  | 3000 reel | 15.1 mΩ | 1.2 V |
| 25 A | CSD17318Q2T | 250 reel  | 15.1 mΩ | 1.2 V |

The first row of the filtered block becomes the primary `part`; the
remaining row becomes a BOM alternative.

## Pinout (DQK)

| Pin | Function |
|---|---|
| 1, 2, 5, 6 | Drain |
| 3 | Gate |
| 4 | Source |
| 7 (small pad) | Source (extension of pin 4) |
| 8 (thermal pad) | Drain |

The thermal pad is the drain — solder it for thermal and electrical
performance.
