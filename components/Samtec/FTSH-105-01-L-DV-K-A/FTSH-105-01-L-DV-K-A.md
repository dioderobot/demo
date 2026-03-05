# FTSH-105-01-L-DV-K-A — Samtec 10-Pin SMD Micro Header

## 1 Features

- 10-position (2x5) high reliability micro header strip
- 1.27 mm (0.050") pitch, surface mount (SMD)
- Up to 8 Gbps performance
- Extended Life Product (E.L.P.) — 10-year manufacturing commitment with 30 µ" gold
- Severe Environment Testing (SET) qualified; aligns with MIL-DTL-55302
- Keying shroud (-K) for polarized mating with FFSD cable
- Alignment pins (-A) for board-to-board registration
- Lead-free solderable, RoHS compliant

## 2 Applications

- ARM SWD / JTAG debug headers
- Board-to-board interconnect
- In-circuit programming / test interfaces
- Compact instrumentation connectors

## 3 Description

The FTSH-105-01-L-DV-K-A is a 10-position, double-row vertical (DV) surface-mount micro header from Samtec's FTSH series. It uses a 1.27 mm pitch and is designed for high mating-cycle applications requiring compact, reliable connectivity. The "-K" suffix adds a keying shroud for polarized mating with FFSD flat flexible cable assemblies. The "-A" suffix adds alignment pins for precise board-to-board positioning.

Lead style -01 provides a 3.05 mm (0.120") post height, compatible with FFSD cable mates and CLP board-mount receptacles.

## 4 Part Number Breakdown

| Code | Meaning |
|------|---------|
| FTSH | Surface mount micro header series |
| 105 | 5 positions per row (10 pins total) |
| 01 | Lead style: 3.05 mm (0.120") post height |
| L | 10 µ" (0.25 µm) gold on post, matte tin on tail |
| DV | Double vertical row configuration |
| K | Keying shroud (mates with FFSD, style -01 only) |
| A | Alignment pin for board registration |

## 5 Pin Configuration

Standard 2x5 pin header. Odd pins on row 1, even pins on row 2.

| Pin | Row | Position |
|-----|-----|----------|
| 1 | 1 | 1 |
| 2 | 2 | 1 |
| 3 | 1 | 2 |
| 4 | 2 | 2 |
| 5 | 1 | 3 |
| 6 | 2 | 3 |
| 7 | 1 | 4 |
| 8 | 2 | 4 |
| 9 | 1 | 5 |
| 10 | 2 | 5 |

## 6 Specifications

| Parameter | Value |
|-----------|-------|
| Pitch | 1.27 mm (0.050") |
| Number of positions | 10 (2 rows × 5) |
| Row spacing | 1.27 mm (0.050") |
| Post height (lead style -01) | 3.05 mm (0.120") |
| Mounting | Surface mount (SMD) |
| Insulator material | Black Liquid Crystal Polymer (LCP) |
| Terminal material | Phosphor Bronze |
| Contact plating (mating) | 10 µ" (0.25 µm) Au over 50 µ" (1.27 µm) Ni |
| Tail plating | Matte Sn |
| Current rating (with CLP mate) | 3.4 A per pin (2 pins powered) |
| Current rating (with FLE mate) | 2.9 A per pin (2 pins powered) |
| Voltage rating | 280 VAC / 396 VDC |
| Operating temperature | -55 °C to +125 °C |
| SMT lead coplanarity | 0.10 mm (0.004") max |
| Lead-free solderable | Yes |

## 7 Mating Connectors

| Type | Compatible Series |
|------|-------------------|
| Board receptacle | CLP, FLE |
| Cable assembly | FFSD, FFTP |

The keying shroud (-K option) ensures polarized mating with FFSD cable assemblies to prevent reversed connections.

## 8 Mechanical Dimensions

| Dimension | Value |
|-----------|-------|
| Overall length | 6.35 mm (0.250") |
| Overall depth | 3.43 mm (0.135") |
| Body height above board | ~1.75 mm |

## 9 Design Considerations

### Footprint
- 10 SMD pads at 1.27 mm pitch, 2 rows spaced 1.27 mm apart
- Recommended pad size: 0.74 mm × 2.22 mm
- Alignment pin holes required when using -A option

### Reflow Soldering
- Compatible with standard lead-free reflow profiles
- SMT lead coplanarity is tightly controlled at 0.004" max

### Mating Force
- Designed for high mating cycle counts
- Meets or exceeds MIL-DTL-55302 testing requirements

## 10 Reference Documents

- Catalog page: [FTSH Surface Mount Series](https://suddendocs.samtec.com/catalog_english/ftsh_mt.pdf)
- Product page: [FTSH-105-01-L-DV-K](https://www.samtec.com/products/ftsh-105-01-l-dv-k)
- Severe Environment Testing: [samtec.com/SET](https://www.samtec.com/testing/severe-environment)
