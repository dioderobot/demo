# (infineon 

## Please note that Cypress is an Infineon Technologies Company.

The document following this cover page is marked as "Cypress" document as this is the company that originally developed the product. Please note that Infineon will continue to offer the product to new and existing customers as part of the Infineon product portfolio.

## Continuity of document content

The fact that Infineon offers the following product as part of the Infineon product portfolio does not lead to any changes to this document. Future revisions will occur when appropriate, and any changes will be set out on the document history page.

## Continuity of ordering part numbers

Infineon continues to support existing part numbers. Please continue to use the ordering part numbers listed in the datasheet for ordering.![img-0.jpeg](img-0.jpeg)

# 256-Kbit (32K × 8) Serial (SPI) F-RAM 

## Features

■ 256-Kbit ferroelectric random access memory (F-RAM) logically organized as $32 \mathrm{~K} \times 8$
$\square$ High-endurance 100 trillion $\left(10^{14}\right)$ read/writes
$\square$ 151-year data retention (See Data Retention and Endurance on page 14)
$\square$ NoDelay ${ }^{\text {TM }}$ writes
$\square$ Advanced high-reliability ferroelectric process
■ Very fast serial peripheral interface (SPI)
$\square$ Up to $40-\mathrm{MHz}$ frequency
$\square$ Direct hardware replacement for serial flash and EEPROM
$\square$ Supports SPI mode $0(0,0)$ and mode $3(1,1)$
■ Sophisticated write-protection scheme
$\square$ Hardware protection using the Write Protect ( $\overline{\mathrm{WP}}$ ) pin
$\square$ Software protection using Write Disable instruction
$\square$ Software block protection for $1 / 4,1 / 2$, or entire array
■ Device ID
$\square$ Manufacturer ID and Product ID
■ Low power consumption
$\square 2.5-\mathrm{mA}$ active current at 40 MHz
$\square 150-\mu \mathrm{A}$ standby current
$\square 8-\mu \mathrm{A}$ sleep mode current
■ Low-voltage operation: $\mathrm{V}_{\mathrm{DD}}=2.0 \mathrm{~V}$ to 3.6 V
■ Industrial temperature: $-40^{\circ} \mathrm{C}$ to $+85^{\circ} \mathrm{C}$
■ Packages
$\square 8$-pin small outline integrated circuit (SOIC) package
$\square 8$-pin dual flat no-leads (DFN) package
■ Restriction of hazardous substances (RoHS) compliant

## Functional Description

The FM25V02A is a 256-Kbit nonvolatile memory employing an advanced ferroelectric process. An F-RAM is nonvolatile and performs reads and writes similar to a RAM. It provides reliable data retention for 151 years while eliminating the complexities, overhead, and system-level reliability problems caused by serial flash, EEPROM, and other nonvolatile memories.
Unlike serial flash and EEPROM, the FM25V02A performs write operations at bus speed. No write delays are incurred. Data is written to the memory array immediately after each byte is successfully transferred to the device. The next bus cycle can commence without the need for data polling. In addition, the product offers substantial write endurance compared with other nonvolatile memories. The FM25V02A is capable of supporting $10^{14}$ read/write cycles, or 100 million times more write cycles than EEPROM.
These capabilities make the FM25V02A ideal for nonvolatile memory applications requiring frequent or rapid writes. Examples range from data logging, where the number of write cycles may be critical, to demanding industrial controls where the long write time of serial flash or EEPROM can cause data loss.
The FM25V02A provides substantial benefits to users of serial EEPROM or flash as a hardware drop-in replacement. The FM25V02A uses the high-speed SPI bus, which enhances the high-speed write capability of F-RAM technology. The device incorporates a read-only Device ID that allows the host to determine the manufacturer, product density, and product revision. The device specifications are guaranteed over an industrial range of $-40^{\circ} \mathrm{C}$ to $+85^{\circ} \mathrm{C}$.
For a complete list of related resources, click here.

## Logic Block Diagram

![img-1.jpeg](img-1.jpeg)## Contents

Pinouts ..... 3
Pin Definitions ..... 3
Functional Overview ..... 4
Memory Architecture ..... 4
Serial Peripheral Interface - SPI Bus ..... 4
SPI Overview ..... 4
SPI Modes ..... 5
Power-Up to First Access ..... 6
Command Structure ..... 6
WREN - Set Write Enable Latch ..... 6
WRDI - Reset Write Enable Latch ..... 6
Status Register and Write Protection ..... 7
RDSR - Read Status Register ..... 7
WRSR - Write Status Register ..... 7
Memory Operation ..... 8
Write Operation ..... 8
Read Operation ..... 8
Fast Read Operation ..... 8
HOLD Pin Operation ..... 10
Sleep Mode ..... 10
Device ID ..... 11
Endurance ..... 12
Maximum Ratings ..... 13
Operating Range ..... 13
DC Electrical Characteristics ..... 13
Data Retention and Endurance ..... 14
Capacitance ..... 14
Thermal Resistance ..... 14
AC Test Conditions ..... 14
AC Switching Characteristics ..... 15
Power Cycle Timing ..... 17
Ordering Information ..... 18
Ordering Code Definitions ..... 18
Package Diagrams ..... 19
Acronyms ..... 21
Document Conventions ..... 21
Units of Measure ..... 21
Document History Page ..... 22
Sales, Solutions, and Legal Information ..... 24
Worldwide Sales and Design Support ..... 24
Products ..... 24
PSoC® Solutions ..... 24
Cypress Developer Community ..... 24
Technical Support ..... 24# Pinouts 

Figure 1. 8-pin SOIC Pinout

| $\overline{C S} \square$ | 1 | 8 | $\square \mathrm{V}_{\mathrm{DD}}$ |
| :--: | :--: | :--: | :--: |
| SO $\square$ | 2 | Top View not to scale | 7 HOLD |
| $\overline{\mathrm{WP}} \square$ | 3 |  | 6 SCK |
| $V_{S S} \square$ | 4 |  | 5 SI |

Figure 2. 8-pin DFN Pinout

| $\overline{C S}$ | 1 | 8 | $\mathrm{V}_{\mathrm{DD}}$ |
| :--: | :--: | :--: | :--: |
| SO | 2 | EXPOSED PAD | 7 HOLD |
| $\overline{\mathrm{WP}}$ | 3 |  | 6 SCK |
| $V_{S S}$ | 4 |  | 5 SI |

Top View not to scale

## Pin Definitions

| Pin Name | I/O Type | Description |
| :--: | :--: | :--: |
| SCK | Input | Serial Clock. All I/O activity is synchronized to the serial clock. Inputs are latched on the rising edge and outputs occur on the falling edge. Because the device is synchronous, the clock frequency may be any value between 0 and 40 MHz and may be interrupted at any time. |
| $\overline{\mathrm{CS}}$ | Input | Chip Select. This active LOW input activates the device. When HIGH, the device enters the low-power standby mode, ignores other inputs, and the output is tristated. When LOW, the device internally activates the SCK signal. A falling edge on $\overline{\mathrm{CS}}$ must occur before every opcode. |
| $\mathrm{SI}^{[1]}$ | Input | Serial Input. All data is input to the device on this pin. The pin is sampled on the rising edge of SCK and is ignored at other times. It should always be driven to a valid logic level to meet $\mathrm{I}_{\mathrm{DD}}$ specifications. |
| $\mathrm{SO}^{[1]}$ | Output | Serial Output. This is the data output pin. It is driven during a read and remains tristated at all other times including when $\overline{\text { HOLD }}$ is LOW. Data transitions are driven on the falling edge of the serial clock. |
| $\overline{\mathrm{WP}}$ | Input | Write Protect. This active LOW pin prevents write operation to the Status Register when WPEN is set to ' 1 '. This is critical because other write protection features are controlled through the Status Register. A complete explanation of write protection is provided on Status Register and Write Protection on page 7. This pin must be tied to $\mathrm{V}_{\mathrm{DD}}$ if not used. |
| $\overline{\text { HOLD }}$ | Input | HOLD Pin. The HOLD pin is used when the host CPU must interrupt a memory operation for another task. When $\overline{\text { HOLD }}$ is LOW, the current operation is suspended. The device ignores any transition on SCK or $\overline{\mathrm{CS}}$. All transitions on $\overline{\text { HOLD }}$ must occur while SCK is LOW. This pin has a weak internal pull-up (refer to the $R_{I N}$ spec in DC Electrical Characteristics). |
| $\mathrm{V}_{\mathrm{SS}}$ | Power supply | Ground for the device. Must be connected to the ground of the system. |
| $\mathrm{V}_{\mathrm{DD}}$ | Power supply | Power supply input to the device. |
| EXPOSED PAD | No connect | The EXPOSED PAD on the bottom of 8-pin DFN package is not connected to the die. The EXPOSED PAD should not be soldered on the PCB. |

## Note

1. SI may be connected to SO for a single pin data interface.## Functional Overview

The FM25V02A is a serial F-RAM memory. The memory array is logically organized as $32,768 \times 8$ bits and is accessed using an industry-standard serial peripheral interface (SPI) bus. The functional operation of the F-RAM is similar to serial flash and serial EEPROMs. The major difference between the FM25V02A and a serial flash or EEPROM with the same pinout is the F-RAM's superior write performance, high endurance, and low power consumption.

## Memory Architecture

When accessing the FM25V02A, the user addresses 32K locations of eight data bits each. These eight data bits are shifted in or out serially. The addresses are accessed using the SPI protocol, which includes a chip select (to permit multiple devices on the bus), an opcode, and a two-byte address. The upper bit of the address range is 'don't care' value. The complete address of 15 bits specifies each byte address uniquely.
Most functions of the FM25V02A are either controlled by the SPI interface or are handled by on-board circuitry. The access time for the memory operation is essentially zero, beyond the time needed for the serial protocol. That is, the memory is read or written at the speed of the SPI bus. Unlike a serial flash or EEPROM, it is not necessary to poll the device for a ready condition because writes occur at bus speed. By the time a new bus transaction can be shifted into the device, a write operation is complete. This is explained in more detail in Memory Operation on page 8.

## Serial Peripheral Interface - SPI Bus

The FM25V02A is a SPI slave device and operates at speeds up to 40 MHz . This high-speed serial bus provides high-performance serial communication to a SPI master. Many common microcontrollers have hardware SPI ports allowing a direct interface. It is quite simple to emulate the port using ordinary port pins for microcontrollers that do not. The FM25V02A operates in SPI Mode 0 and 3.

## SPI Overview

The SPI is a four-pin interface with Chip Select ( $\overline{\mathrm{CS}}$ ), Serial Input (SI), Serial Output (SO), and Serial Clock (SCK) pins.
The SPI is a synchronous serial interface, which uses clock and data pins for memory access and supports multiple devices on the data bus. A device on the SPI bus is activated using the $\overline{\mathrm{CS}}$ pin.
The relationship between chip select, clock, and data is dictated by the SPI mode. This device supports SPI modes 0 and 3. In both of these modes, data is clocked into the F-RAM on the rising edge of SCK starting from the first rising edge after $\overline{\mathrm{CS}}$ goes active.
The SPI protocol is controlled by opcodes. These opcodes specify the commands from the bus master to the slave device. After $\overline{\mathrm{CS}}$ is activated, the first byte transferred from the bus
master is the opcode. Following the opcode, any addresses and data are then transferred. The $\overline{\mathrm{CS}}$ must go inactive after an operation is complete and before a new opcode can be issued. The commonly used terms in the SPI protocol are as follows:

## SPI Master

The SPI master device controls the operations on a SPI bus. An SPI bus may have only one master with one or more slave devices. All the slaves share the same SPI bus lines and the master may select any of the slave devices using the $\overline{\mathrm{CS}}$ pin. All of the operations must be initiated by the master activating a slave device by pulling the $\overline{\mathrm{CS}}$ pin of the slave LOW. The master also generates the SCK and all the data transmission on SI and SO lines are synchronized with this clock.

## SPI Slave

The SPI slave device is activated by the master through the Chip Select line. A slave device gets the SCK as an input from the SPI master and all the communication is synchronized with this clock. An SPI slave never initiates a communication on the SPI bus and acts only on the instruction from the master.
The FM25V02A operates as an SPI slave and may share the SPI bus with other SPI slave devices.

## Chip Select ( $\overline{\mathrm{CS}}$ )

To select any slave device, the master needs to pull down the corresponding $\overline{\mathrm{CS}}$ pin. Any instruction can be issued to a slave device only while the $\overline{\mathrm{CS}}$ pin is LOW. When the device is not selected, data through the SI pin is ignored and the serial output pin (SO) remains in a high-impedance state.
Note A new instruction must begin with the falling edge of $\overline{\mathrm{CS}}$. Therefore, only one opcode can be issued for each active Chip Select cycle.

## Serial Clock (SCK)

The serial clock is generated by the SPI master and the communication is synchronized with this clock after $\overline{\mathrm{CS}}$ goes LOW.
The FM25V02A enables SPI modes 0 and 3 for data communication. In both of these modes, the inputs are latched by the slave device on the rising edge of SCK and outputs are issued on the falling edge. Therefore, the first rising edge of SCK signifies the arrival of the first bit (MSB) of a SPI instruction on the SI pin. Further, all data inputs and outputs are synchronized with SCK.

## Data Transmission (SI/SO)

The SPI data bus consists of two lines, SI and SO, for serial data communication. SI is also referred to as Master Out Slave In (MOSI) and SO is referred to as Master In Slave Out (MISO). The master issues instructions to the slave through the SI pin, while the slave responds through the SO pin. Multiple slave devices may share the SI and SO lines as described earlier.
The FM25V02A has two separate pins for SI and SO, which can be connected with the master as shown in Figure 3 on page 5.For a microcontroller that has no dedicated SPI bus, a general-purpose port may be used. To reduce hardware resources on the controller, it is possible to connect the two data pins (SI, SO) together and tie off (HIGH) the $\overline{\mathrm{HOLD}}$ and $\overline{\mathrm{WP}}$ pins. Figure 4 shows such a configuration, which uses only three pins.

## Most Significant Bit (MSB)

The SPI protocol requires that the first bit to be transmitted is the Most Significant Bit (MSB). This is valid for both address and data transmission.
The 256-Kbit serial F-RAM requires a 2-byte address for any read or write operation. Because the address is only 15 bits, the upper bit which is fed in is ignored by the device. Although the upper bit is 'don't care', Cypress recommends that these bits be set to 0 s to enable seamless transition to higher memory densities.

## Serial Opcode

After the slave device is selected with $\overline{\mathrm{CS}}$ going LOW, the first byte received is treated as the opcode for the intended operation. FM25V02A uses the standard opcodes for memory accesses.

## Invalid Opcode

If an invalid opcode is received, the opcode is ignored and the device ignores any additional serial data on the SI pin until the next falling edge of $\overline{\mathrm{CS}}$, and the SO pin remains tristated.

## Status Register

FM25V02A has an 8-bit Status Register. The bits in the Status Register are used to configure the device. These bits are described in Table 3 on page 7.

Figure 3. System Configuration with SPI Port
![img-2.jpeg](img-2.jpeg)

Figure 4. System Configuration Without SPI Port
![img-3.jpeg](img-3.jpeg)

## SPI Modes

FM25V02A may be driven by a microcontroller with its SPI peripheral running in either of the following two modes:

- SPI Mode 0 (CPOL $=0, \mathrm{CPHA}=0$ )
- SPI Mode 3 (CPOL $=1, \mathrm{CPHA}=1$ )

For both these modes, the input data is latched in on the rising edge of SCK starting from the first rising edge after $\overline{\mathrm{CS}}$ goes
active. If the clock starts from a HIGH state (in mode 3), the first rising edge after the clock toggles is considered. The output data is available on the falling edge of SCK. The two SPI modes are shown in Figure 5 on page 6 and Figure 6 on page 6. The status of the clock when the bus master is not transferring data is:

- SCK remains at 0 for Mode 0
- SCK remains at 1 for Mode 3The device detects the SPI mode from the status of the SCK pin when the device is selected by bringing the $\overline{\mathrm{CS}}$ pin LOW. If the SCK pin is LOW when the device is selected, SPI Mode 0 is assumed and if the SCK pin is HIGH, it works in SPI Mode 3.

Figure 5. SPI Mode 0
![img-4.jpeg](img-4.jpeg)

Figure 6. SPI Mode 3
![img-5.jpeg](img-5.jpeg)

## Power-Up to First Access

The FM25V02A is not accessible for a $t_{\text {PU }}$ time after power-up. Users must comply with the timing parameter $t_{P U}$, which is the minimum time from $\mathrm{V}_{\mathrm{DD}}(\mathrm{min})$ to the first $\overline{\mathrm{CS}}$ LOW.

## Command Structure

There are nine commands, called opcodes, that can be issued by the bus master to the FM25V02A. They are listed in Table 1. These opcodes control the functions performed by the memory.

Table 1. Opcode Commands

| Name | Description | Opcode |
| :-- | :-- | :-- |
| WREN | Set write enable latch | 00000110 b |
| WRDI | Reset write enable latch | 00000100 b |
| RDSR | Read Status Register | 00000101 b |
| WRSR | Write Status Register | 00000001 b |
| READ | Read memory data | 00000011 b |
| FSTRD | Fast read memory data | 00001011 b |
| WRITE | Write memory data | 00000010 b |
| SLEEP | Enter sleep mode | 10111001 b |
| RDID | Read device ID | 10011111 b |
| Reserved | Reserved | 11000011 b |
|  |  | 11000010 b |
|  |  | 01011010 b |
|  |  | 01011011 b |

## WREN - Set Write Enable Latch

The FM25V02A will power up with writes disabled. The WREN command must be issued before any write operation. Sending the WREN opcode allows the user to issue subsequent opcodes for write operations. These include writing the Status Register (WRSR) and writing the memory (WRITE).
Sending the WREN opcode causes the internal Write Enable Latch to be set. A flag bit in the Status Register, called WEL, indicates the state of the latch. WEL = '1' indicates that writes are permitted. Attempting to write the WEL bit in the Status Register has no effect on the state of this bit - only the WREN opcode can set this bit. The WEL bit will be automatically cleared on the rising edge of $\overline{\mathrm{CS}}$ following a WRDI, a WRSR, or a WRITE operation. This prevents further writes to the Status Register or the F-RAM array without another WREN command. Figure 7 illustrates the WREN command bus configuration.

Figure 7. WREN Bus Configuration
![img-6.jpeg](img-6.jpeg)

## WRDI - Reset Write Enable Latch

The WRDI command disables all write activity by clearing the Write Enable Latch. The user can verify that writes are disabled by reading the WEL bit in the Status Register and verifying that WEL is equal to ' 0 '. Figure 8 illustrates the WRDI command bus configuration.

Figure 8. WRDI Bus Configuration
![img-7.jpeg](img-7.jpeg)# Status Register and Write Protection 

The write protection features of the FM25V02A are multi-tiered and are enabled through the status register. The status register is organized as follows (the default value shipped from the factory for bits in the status register is ' 0 '):

Table 2. Status Register

| Bit 7 | Bit 6 | Bit 5 | Bit 4 | Bit 3 | Bit 2 | Bit 1 | Bit 0 |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| WPEN (0) | $X(0)$ | $X(0)$ | $X(0)$ | BP1 (0) | BP0 (0) | WEL (0) | $X(0)$ |

Table 3. Status Register Bit Definition

| Bit | Definition | Description |
| :--: | :--: | :--: |
| Bit 0 | Don't care | This bit is non-writable and always returns ' 0 ' upon read. |
| Bit 1 (WEL) | Write Enable | WEL indicates if the device is write enabled. This bit defaults to ' 0 ' (disabled) on power-up. WEL = ' 1 ' $\rightarrow$ Write enabled <br> WEL = ' 0 ' $\rightarrow$ Write disabled |
| Bit 2 (BP0) | Block Protect bit ' 0 ' | Used for block protection. For details, see Table 4 on page 7. |
| Bit 3 (BP1) | Block Protect bit ' 1 ' | Used for block protection. For details, see Table 4 on page 7. |
| Bit 4-6 | Don't care | These bits are non-writable and always return ' 0 ' upon read. |
| Bit 7 (WPEN) | Write Protect Enable bit | Used to enable the function of Write Protect Pin $(\overline{\mathrm{WP}})$. For details, see Table 5 on page 7. |

Bits 0 and 4-6 are fixed at ' 0 '; none of these bits can be modified. Note that bit 0 ("Ready or Write in progress" bit in serial flash and EEPROM) is unnecessary, as the F-RAM writes in real-time and is never busy, so it reads out as a ' 0 '. An exception to this is when the device is waking up from sleep mode, which is described in Sleep Mode on page 10. The BP1 and BP0 control the software write-protection features and are nonvolatile bits. The WEL flag indicates the state of the Write Enable Latch. Attempting to directly write the WEL bit in the Status Register has no effect on its state. This bit is internally set and cleared via the WREN and WRDI commands, respectively.
BP1 and BP0 are memory block write protection bits. They specify portions of memory that are write-protected as shown in Table 4.

Table 4. Block Memory Write Protection

| BP1 | BP0 | Protected Address Range |
| :--: | :--: | :--: |
| 0 | 0 | None |
| 0 | 1 | 6000 h to 7FFFh (upper 1/4) |
| 1 | 0 | 4000 h to 7FFFh (upper 1/2) |
| 1 | 1 | 0000 h to 7FFFh (all) |

The BP1 and BP0 bits and the Write Enable Latch are the only mechanisms that protect the memory from writes. The remaining write protection features protect inadvertent changes to the block protect bits.
The write protect enable bit (WPEN) in the Status Register controls the effect of the hardware write protect $(\overline{\mathrm{WP}})$ pin. When the WPEN bit is set to ' 0 ', the status of the $\overline{\mathrm{WP}}$ pin is ignored. When the WPEN bit is set to ' 1 ', a LOW on the $\overline{\mathrm{WP}}$ pin inhibits a
write to the Status Register. Thus the Status Register is write-protected only when WPEN = ' 1 ' and $\overline{\mathrm{WP}}=$ ' 0 '.

Table 5 summarizes the write protection conditions.
Table 5. Write Protection

| WEL | WPEN | $\overline{\mathrm{WP}}$ | Protected <br> Blocks | Unprotected <br> Blocks | Status <br> Register |
| :--: | :--: | :--: | :--: | :--: | :--: |
| 0 | X | X | Protected | Protected | Protected |
| 1 | 0 | X | Protected | Unprotected | Unprotected |
| 1 | 1 | 0 | Protected | Unprotected | Protected |
| 1 | 1 | 1 | Protected | Unprotected | Unprotected |

## RDSR - Read Status Register

The RDSR command allows the bus master to verify the contents of the Status Register. Reading the status register provides information about the current state of the write-protection features. Following the RDSR opcode, the FM25V02A will return one byte with the contents of the Status Register.

## WRSR - Write Status Register

The WRSR command allows the SPI bus master to write into the Status Register and change the write protect configuration by setting the WPEN, BP0, and BP1 bits as required. Prior to issuing a WRSR command, the $\overline{\mathrm{WP}}$ pin must be HIGH or inactive. Note that on the FM25V02A, $\overline{\mathrm{WP}}$ only prevents writing to the Status Register, not the memory array. Before sending the WRSR command, the user must send a WREN command to enable writes. Executing a WRSR command is a write operation and therefore, clears the Write Enable Latch.Figure 9. RDSR Bus Configuration
![img-8.jpeg](img-8.jpeg)

Figure 10. WRSR Bus Configuration (WREN not shown)
![img-9.jpeg](img-9.jpeg)

## Memory Operation

The SPI interface, which is capable of a high clock frequency, highlights the fast write capability of the F-RAM technology. Unlike serial flash and EEPROMs, the FM25V02A can perform sequential writes at bus speed. No page register is needed and any number of sequential writes may be performed.

## Write Operation

All writes to the memory begin with a WREN opcode with $\overline{C S}$ being asserted and deasserted. The next opcode is WRITE. The WRITE opcode is followed by a two-byte address containing the 15-bit address (A14-A0) of the first data byte to be written into the memory. The upper bit of the two-byte address is ignored. Subsequent bytes are data bytes, which are written sequentially. Addresses are incremented internally as long as the bus master continues to issue clocks and keeps $\overline{\mathrm{CS}}$ LOW. If the last address of 7FFFh is reached, the counter will roll over to 0000h. Data is written MSB first. The rising edge of $\overline{\mathrm{CS}}$ terminates a write operation. A write operation is shown in Figure 11 on page 9.
Note When a burst write reaches a protected block address, the automatic address increment stops and all the subsequent data bytes received for write will be ignored by the device.
EEPROMs use page buffers to increase their write throughput. This compensates for the technology's inherently slow write operations. F-RAM memories do not have page buffers because each byte is written to the F-RAM array immediately after it is
clocked in (after the eighth clock). This allows any number of bytes to be written without page buffer delays.
Note If the power is lost in the middle of the write operation, only the last completed byte will be written.

## Read Operation

After the falling edge of $\overline{\mathrm{CS}}$, the bus master can issue a READ opcode. Following the READ command is a two-byte address containing the 15-bit address (A14-A0) of the first byte of the read operation. The upper bit of the address is ignored. After the opcode and address are issued, the device drives out the read data on the next eight clocks. The SI input is ignored during read data bytes. Subsequent bytes are data bytes, which are read out sequentially. Addresses are incremented internally as long as the bus master continues to issue clocks and $\overline{\mathrm{CS}}$ is LOW. If the last address of 7FFFh is reached, the counter will roll over to 0000h. Data is read MSB first. The rising edge of $\overline{\mathrm{CS}}$ terminates a read operation and tristates the SO pin. A read operation is shown in Figure 12 on page 9.

## Fast Read Operation

The FM25V02A supports a FAST READ opcode (0Bh) that is provided for code compatibility with serial flash devices. The FAST READ opcode is followed by a two-byte address containing the 15-bit address (A14-A0) of the first byte of the read operation and then a dummy byte. The dummy byte inserts a read latency of an 8 -clock cycle. The fast read operation isotherwise the same as an ordinary read operation except that it requires an additional dummy byte. After receiving the opcode, address, and a dummy byte, the FM25V02A starts driving its SO line with data bytes, with the MSB first, and continues transmitting as long as the device is selected and the clock is available. In case of bulk read, the internal address counter is
incremented automatically, and after the last address 7FFFh is reached, the counter rolls over to 0000 h . When the device is driving data on its SO line, any transition on its SI line is ignored. The rising edge of $\overline{C S}$ terminates a fast read operation and tristates the SO pin. A Fast Read operation is shown in Figure 13.

Figure 11. Memory Write (WREN not shown) Operation
![img-10.jpeg](img-10.jpeg)

Figure 12. Memory Read Operation
![img-11.jpeg](img-11.jpeg)

Figure 13. Fast Read Operation
![img-12.jpeg](img-12.jpeg)## HOLD Pin Operation

The HOLD pin can be used to interrupt a serial operation without aborting it. If the bus master pulls the HOLD pin LOW while SCK is LOW, the current operation will pause. Taking the HOLD pin

HIGH while SCK is LOW will resume an operation. The transitions of $\overline{\text { HOLD }}$ must occur while SCK is LOW, but the SCK and $\overline{\mathrm{CS}}$ pin can toggle during a hold state.

Figure 14. HOLD Operation ${ }^{[2]}$
![img-13.jpeg](img-13.jpeg)

## Sleep Mode

A low-power sleep mode is implemented on the FM25V02A device. The device will enter the low-power state when the SLEEP opcode B9h is clocked-in and a rising edge of $\overline{\mathrm{CS}}$ is applied. When in sleep mode, the SCK and SI pins are ignored and SO will be HI-Z, but the device continues to monitor the $\overline{\mathrm{CS}}$
pin. On the next falling edge of $\overline{\mathrm{CS}}$, the device will return to normal operation within $t_{\text {REC }}$ time. The SO pin remains in a HI-Z state during the wakeup period. The device does not necessarily respond to an opcode within the wakeup period. To start the wakeup procedure, the controller may send a "dummy" read, for example, and wait the remaining $t_{\text {REC }}$ time.

Figure 15. Sleep Mode Operation
![img-14.jpeg](img-14.jpeg)

Note
2. Figure 14 shows the HOLD operation for input mode and output mode.# Device ID 

The FM25V02A device can be interrogated for its manufacturer, product identification, and die revision. The RDID opcode 9Fh allows the user to read the manufacturer ID and product ID, both of which are read-only bytes. The JEDEC-assigned
manufacturer ID places the Cypress (Ramtron) identifier in bank 7; therefore, there are six bytes of the continuation code 7 Fh followed by the single byte C 2 h . There are two bytes of product ID, which includes a family code, a density code, a sub code, and the product revision code.

Table 6. Device ID

| Device ID (9 bytes) | Device ID Description |  |  |  |  |  |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
|  | 71-16 <br> (56 bits) | 15-13 <br> (3 bits) | 12-8 <br> (5 bits) | 7-6 <br> (2 bits) | 5-3 <br> (3 bits) | 2-0 <br> (3 bits) |
|  | Manufacturer ID | Product ID |  |  |  |  |
|  |  | Family | Density | Sub | Rev | Rsvd |
| 7F7F7F7F7F7FC22208h | 0111111101111111011111110111 1111011111110111111111000010 | 001 | 00010 | 00 | 001 | 000 |

Figure 16. Read Device ID
![img-15.jpeg](img-15.jpeg)## Endurance

The FM25V02A devices are capable of being accessed at least $10^{14}$ times, reads or writes. An F-RAM memory operates with a read and restore mechanism. Therefore, an endurance cycle is applied on a row basis for each access (read or write) to the memory array. The F-RAM architecture is based on an array of rows and columns of 4 K rows of 64 -bits each. The entire row is internally accessed once whether a single byte or all eight bytes are read or written. Each byte in the row is counted only once in an endurance calculation. Table 7 shows endurance calculations for a 64-byte repeating loop, which includes an opcode, a starting address, and a sequential 64-byte data stream. This causes each byte to experience one endurance cycle through the loop.

Table 7. Time to Reach Endurance Limit for Repeating 64-byte Loop

| SCK Freq <br> (MHz) | Endurance <br> Cycles/sec | Endurance <br> Cycles/year | Years to Reach <br> Limit |
| :--: | :--: | :--: | :--: |
| 40 | 74,620 | $2.35 \times 10^{12}$ | 42.6 |
| 20 | 37,310 | $1.18 \times 10^{12}$ | 85.1 |
| 10 | 18,660 | $5.88 \times 10^{11}$ | 170.2 |
| 5 | 9,330 | $2.94 \times 10^{11}$ | 340.3 |# **FACTS**

## **Maximum Ratings**

Exceeding maximum ratings may shorten the useful life of the device. These user guidelines are not tested.

Storage temperature: –65 °C to +125 °C

Maximum accumulated storage time: -1000 h

At 125 °C ambient temperature: -10 Years

At 85 °C ambient temperature: -10 Years

Ambient temperature with power applied: –55 °C to +125 °C

Supply voltage on VDD relative to VSS: –1.0 V to +4.5 V

Input voltage: –1.0 V to +4.5 V and VIN < VDD + 1.0 V

DC voltage applied to outputs in HI-Z state: –0.5 V to VDD + 0.5 V

Transient voltage (< 20 ns) on any pin to ground potential: –2.0 V to VDD + 2.0 V

Package power dissipation capability (TA = 25 °C): 1.0 W

Surface mount lead soldering temperature (3 seconds): +260 °C

DC output current (1 output at a time, 1s duration): 15 mA

Electrostatic discharge voltage: -1000 V

Human Body Model (JEDEC Std JESD22-A114-B): 2 kV -Charged Device Model (JEDEC Std JESD22-C101-A): 500 V -Latch-up current: > 140 mA

## **Operating Range**

|  Range | Ambient Temperature (TA) | VDD  |
| --- | --- | --- |
|  Industrial | –40 °C to +85 °C | 2.0 V to 3.6 V  |

## **DC Electrical Characteristics**

Over the Operating Range

|  Parameter | Description | Test Conditions | Min | Typ[3] | Max | Unit  |
| --- | --- | --- | --- | --- | --- | --- |
|  VDD | Power supply |  | 2.0 | 3.3 | 3.6 | V  |
|  IDD | VDD supply current | SCK toggling between | fSCK = 40 MHz | – | – | mA  |
|   |  | VDD – 0.2 V and VSS, | fSCK = 1 MHz | – | – | mA  |
|   |  | other inputs |  |  |  | mA  |
|   |  | VSS or VDD – 0.2 V, |  |  |  |   |
|   |  | SO = Open. |  |  |  |   |
|  SS | VDD standby current | CS = VDD. All other inputs VSS or VDD. | – | 90 | 150 | µA  |
|  IZZ | Sleep mode current | CS = VDD. All other inputs VSS or VDD. | – | 5 | 8 | µA  |
|  ILI | Input leakage current (Except HOLD) | VSS ≤ VIN ≤ VDD | –1 | – | +1 | µA  |
|   | Input leakage current (for HOLD) |  | –100 | – | +1 | µA  |
|  ILO | Output leakage current | VSS ≤ VOUT ≤ VDD | –1 | – | +1 | µA  |
|  VIH | Input HIGH voltage |  | 0.7 × VDD | – | VDD + 0.3 | V  |
|  VL | Input LOW voltage |  | –0.3 | – | 0.3 × VDD | V  |
|  VOH1 | Output HIGH voltage | IOH = –1 mA, VDD = 2.7 V. | 2.4 | – | – | V  |
|  VOH2 | Output HIGH voltage | IOH = –100 µA | VDD – 0.2 | – | – | V  |
|  VOL1 | Output LOW voltage | IOL = 2 mA, VDD = 2.7 V | – | – | 0.4 | V  |
|  VOL2 | Output LOW voltage | IOL = 150 µA | – | – | 0.2 | V  |
|  RII[4] | Input resistance (HOLD) | For VIN = VIL (max) | 800 | – | – | kΩ  |
|   |  | For VIN = VIH (min) | 30 | – | – | kΩ  |

#### **Notes**

1. Typical values are at 25 °C, VDD = VDD (typ). Not 100% tested.
2. The input pull-up circuit is strong (30 kΩ) when the input voltage is above VIN and weak (800 kΩ) when the input voltage is below VL.

Document Number: 001-90865 Rev. *I Page 13 of 24# Data Retention and Endurance 

| Parameter | Description | Test condition | Min | Max | Unit |
| :-- | :-- | :-- | :--: | :--: | :--: |
| $T_{D R}$ | Data retention | $\mathrm{T}_{\mathrm{A}}=85^{\circ} \mathrm{C}$ | 10 | - | Years |
|  |  | $\mathrm{T}_{\mathrm{A}}=75^{\circ} \mathrm{C}$ | 38 | - |  |
|  |  | $\mathrm{T}_{\mathrm{A}}=65^{\circ} \mathrm{C}$ | 151 | - |  |
| $\mathrm{NV}_{\mathrm{C}}$ | Endurance | Over operating temperature | $10^{14}$ | - | Cycles |

## Capacitance

| Parameter ${ }^{[5]}$ | Description | Test Conditions | Max | Unit |
| :-- | :-- | :-- | :--: | :--: |
| $\mathrm{C}_{\mathrm{O}}$ | Output pin capacitance (SO) | $\mathrm{T}_{\mathrm{A}}=25^{\circ} \mathrm{C}, \mathrm{f}=1 \mathrm{MHz}, \mathrm{V}_{\mathrm{DD}}=\mathrm{V}_{\mathrm{DD}}(\mathrm{typ})$ | 8 | pF |
| $\mathrm{C}_{\mathrm{t}}$ | Input pin capacitance |  | 6 | pF |

## Thermal Resistance

| Parameter ${ }^{[5]}$ | Description | Test Conditions | 8-pin SOIC | 8-pin DFN | Unit |
| :-- | :-- | :-- | :--: | :--: | :--: |
| $\Theta_{\text {JA }}$ | Thermal resistance <br> (junction to ambient) | Test conditions follow standard test methods <br> and procedures for measuring thermal <br> impedance, per EIA/JESD51. | 146 | 31 | ${ }^{\circ} \mathrm{C} / \mathrm{W}$ |
| $\Theta_{\text {JC }}$ | Thermal resistance <br> (junction to case) | 48 | 35 | ${ }^{\circ} \mathrm{C} / \mathrm{W}$ |  |

## AC Test Conditions

Input pulse levels ..............................10\% and $90 \%$ of $\mathrm{V}_{\mathrm{DD}}$
Input rise and fall times ................................................. 3 ns
Input and output timing reference levels ................ $0.5 \times \mathrm{V}_{\mathrm{DD}}$
Output load capacitance ........................................... 30 pF

## Note

5. This parameter is periodically sampled and not $100 \%$ tested.# AC Switching Characteristics 

Over the Operating Range

| Parameters ${ }^{[6]}$ |  | Description | $\mathrm{V}_{\mathrm{DD}}=2.0 \mathrm{~V}$ to 3.6 V |  | $\mathrm{V}_{\mathrm{DD}}=2.7 \mathrm{~V}$ to 3.6 V |  | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| Cypress <br> Parameter | Alt. <br> Parameter |  | Min | Max | Min | Max |  |
| $t_{\text {SCK }}$ | - | SCK clock frequency | 0 | 25 | 0 | 40 | MHz |
| $t_{\mathrm{CH}}$ | - | Clock HIGH time | 18 | - | 11 | - | ns |
| $t_{\mathrm{CL}}$ | - | Clock LOW time | 18 | - | 11 | - | ns |
| $t_{\text {CSU }}$ | $t_{\text {CSS }}$ | Chip select setup | 12 | - | 10 | - | ns |
| $t_{\mathrm{CSH}}$ | $t_{\mathrm{CSH}}$ | Chip select hold | 12 | - | 10 | - | ns |
| $t_{\mathrm{DD}}{ }^{[7,8]}$ | $t_{\text {HZCS }}$ | Output disable time | - | 20 | - | 12 | ns |
| $t_{\text {ODV }}$ | $t_{\mathrm{CO}}$ | Output data valid time | - | 16 | - | 9 | ns |
| $t_{\mathrm{OH}}$ | - | Output hold time | 0 | - | 0 | - | ns |
| $t_{D}$ | - | Deselect time | 60 | - | 40 | - | ns |
| $t_{R}{ }^{[9,10]}$ | - | Data in rise time | - | 50 | - | 50 | ns |
| $t_{R}{ }^{[9,10]}$ | - | Data in fall time | - | 50 | - | 50 | ns |
| $t_{\text {SU }}$ | $t_{\text {SD }}$ | Data setup time | 8 | - | 5 | - | ns |
| $t_{H}$ | $t_{\text {HD }}$ | Data hold time | 8 | - | 5 | - | ns |
| $t_{H S}$ | $t_{S H}$ | HÖLD setup time | 12 | - | 10 | - | ns |
| $t_{H H}$ | $t_{H H}$ | HÖLD hold time | 12 | - | 10 | - | ns |
| $t_{\text {HZ }}{ }^{[7,8]}$ | $t_{\text {HHZ }}$ | HÖLD LOW to HI-Z | - | 25 | - | 20 | ns |
| $t_{L Z}{ }^{[8]}$ | $t_{\text {HLZ }}$ | HÖLD HIGH to data active | - | 25 | - | 20 | ns |

## Notes

[^0]
[^0]:    6. Test conditions assume a signal transition time of 3 ns or less, timing reference levels of $0.5 \times V_{D D}$, input pulse levels of $10 \%$ to $90 \%$ of $V_{D D}$, output loading of the specified $t_{\mathrm{CL}} / t_{\mathrm{OH}}$ and 30 pF load capacitance shown in AC Test Conditions.
    7. $t_{\mathrm{DD}}$ and $t_{\mathrm{HZ}}$ are specified with a load capacitance of 5 pF . Transition is measured when the outputs enter a high impedance state.
    8. Characterized but not $100 \%$ tested in production.
    9. Rise and fall times measured between $10 \%$ and $90 \%$ of waveform.
    10. These parameters are guaranteed by design and are not tested.Figure 17. Synchronous Data Timing (Mode 0)
![img-16.jpeg](img-16.jpeg)

Figure 18. HOLD Timing
![img-17.jpeg](img-17.jpeg)# Power Cycle Timing 

Over the Operating Range

| Parameter | Description | Min | Max | Unit |
| :--: | :--: | :--: | :--: | :--: |
| $t_{P U}$ | Power-up $\mathrm{V}_{\mathrm{DD}}$ (min) to first access ( $\overline{\mathrm{CS}}$ LOW) | 250 | - | $\mu \mathrm{s}$ |
| $t_{P D}$ | Last access ( $\overline{\mathrm{CS}}$ HIGH) to power-down ( $\mathrm{V}_{\mathrm{DD}}(\mathrm{min})$ ) | 0 | - | $\mu \mathrm{s}$ |
| $t_{V R}{ }^{[11,12]}$ | $V_{D D}$ power-up ramp rate | 50 | - | $\mu \mathrm{s} / \mathrm{V}$ |
| $t_{V F}{ }^{[11,12]}$ | $V_{D D}$ power-down ramp rate | 100 | - | $\mu \mathrm{s} / \mathrm{V}$ |
| $t_{\text {REC }}{ }^{[13]}$ | Recovery time from sleep mode | - | 400 | $\mu \mathrm{s}$ |

Figure 19. Power Cycle Timing
![img-18.jpeg](img-18.jpeg)

## Notes

11. Slope measured at any point on $\mathrm{V}_{\mathrm{DD}}$ waveform.
12. These parameters are guaranteed by design and are not tested.
13. Refer to Figure 15 on page 10 for sleep mode recovery timing.# Ordering Information

|  Ordering Code | Package
Diagram | Package Type | Operating
Range  |
| --- | --- | --- | --- |
|  FM25V02A-G | $51-85066$ | 8-pin SOIC | Industrial  |
|  FM25V02A-GTR | $51-85066$ | 8-pin SOIC |   |
|  FM25V02A-DG | $001-85260$ | 8-pin DFN |   |
|  FM25V02A-DGTR | $001-85260$ | 8-pin DFN |   |

All these parts are Pb -free. Contact your local Cypress sales representative for availability of these parts.

## Ordering Code Definitions

![img-19.jpeg](img-19.jpeg)# Package Diagrams 

Figure 20. 8-pin SOIC (150 Mils) Package Outline, 51-85066

1. DIMENSIONS IN INCHES[MM] MIN. MAX.
2. PIN 1 ID IS OPTIONAL.

ROUND ON SINGLE LEADFRAME RECTANGULAR ON MATRIX LEADFRAME
3. REFERENCE JEDEC MS-012
4. PACKAGE WEIGHT 0.07 gms

|  | PART \# |
| :-- | :-- |
| S08.15 | STANDARD PKG |
| SZ08.15 | LEAD FREE PKG |
| SW8.15 | LEAD FREE PKG |

![img-20.jpeg](img-20.jpeg)# Package Diagrams (continued) 

Figure 21. 8-pin DFN ( $4.0 \mathrm{~mm} \times 4.5 \mathrm{~mm} \times 0.8 \mathrm{~mm}$ ) Package Outline, 001-85260

## TOP VIEW

![img-21.jpeg](img-21.jpeg)

NOTES:

1. REFERENCE JEDEC \# MO-229F
2. ALL DIMENSIONS ARE IN MILLIMETERS

SIDE VIEW
![img-22.jpeg](img-22.jpeg)

## BOTTOM VIEW

![img-23.jpeg](img-23.jpeg)
$001-85260{ }^{\circ} \mathrm{B}$## Acronyms

| Acronym | Description |
| :-- | :-- |
| CPHA | Clock Phase |
| CPOL | Clock Polarity |
| DFN | Dual Flat No-lead |
| EEPROM | Electrically Erasable Programmable Read-Only <br> Memory |
| EIA | Electronic Industries Alliance |
| F-RAM | Ferroelectric Random Access Memory |
| I/O | Input/Output |
| JEDEC | Joint Electron Devices Engineering Council |
| JESD | JEDEC Standards |
| LSB | Least Significant Bit |
| MSB | Most Significant Bit |
| RoHS | Restriction of Hazardous Substances |
| SPI | Serial Peripheral Interface |
| SOIC | Small Outline Integrated Circuit |

## Document Conventions

Units of Measure

| Symbol | Unit of Measure |
| :-- | :-- |
| ${ }^{\circ} \mathrm{C}$ | degree Celsius |
| Hz | hertz |
| kHz | kilohertz |
| $\mathrm{k} \Omega$ | kilohm |
| Kbit | Kilobit |
| MHz | megahertz |
| $\mu \mathrm{A}$ | microampere |
| $\mu \mathrm{F}$ | microfarad |
| $\mu \mathrm{s}$ | microsecond |
| mA | milliampere |
| ms | millisecond |
| ns | nanosecond |
| $\Omega$ | ohm |
| $\%$ | percent |
| pF | picofarad |
| V | volt |
| W | watt |Document History Page Document Title: FM25V02A, 256-Kbit (32K × 8) Serial (SPI) F-RAM Document Number: 001-90865

|  Rev. | ECN No. | Orig. of Change | Submission Date | Description of Change  |
| --- | --- | --- | --- | --- |
|  ** | 4265427 | GVCH | 01/29/2014 | New data sheet.  |
|  *A | 4390913 | GVCH | 06/20/2014 | Changed status from Advance to Preliminary. Updated Pin Definitions: Updated details in "Description" column of "HOLD" pin (Added the sentence, "This pin has a weak internal pull-up (refer to the RIN spec in DC Electrical Characteristics on page 13)"). Updated Maximum Ratings: Removed "Machine Model" under "Electrostatic Discharge Voltage". Updated DC Electrical Characteristics: Added typical value for ISB and IZZ parameters. Changed minimum value of Rin parameter from 40 kΩ to 30 kΩ corresponding to Test Condition "VIN = VIN(min)". Changed minimum value of Rin parameter from 1 MΩ to 800 kΩ corresponding to Test Condition "VIN = VIL(max)". Updated Note 4. Updated Thermal Resistance: Replaced TBD with values in "8-pin SOIC" and "8-pin TDFN" columns.  |
|  *B | 4571858 | GVCH | 11/18/2014 | Updated Serial Peripheral Interface - SPI Bus: Updated Command Structure: Updated Table 1: Added reserved opcodes - 0xC3, 0xC2, 0x5A, 0x5B.  |
|  *C | 4197512 | ZSK | 02/10/2015 | Changed status from Preliminary to Final. Replaced "TDFN" with "DFN" in all instances across the document. Updated Functional Description: Added "For a complete list of related resources, click here." at the end. Updated Pin Definitions: Updated details in "Description" column of "EXPOSED PAD" pin. Updated Package Diagrams: spec 51-85066 – Changed revision from *F to *G.  |
|  *D | 4784430 | GVCH | 06/02/2015 | Updated Package Diagrams: spec 001-85260 – Changed revision from *A to *B. Updated to new template.  |
|  *E | 4879715 | ZSK / PSR | 08/11/2015 | Updated Maximum Ratings: Removed "Maximum junction temperature" and its corresponding details. Added "Maximum accumulated storage time" and its corresponding details. Added "Ambient temperature with power applied" and its corresponding details.  |
|  *F | 5085935 | GVCH | 01/14/2016 | Updated Ordering Information: Updated part numbers. Updated Package Diagrams: spec 51-85066 – Changed revision from *G to *H.  |
|  *G | 5450688 | ZSK | 09/27/2016 | Updated Power Cycle Timing: Changed minimum value of tPU parameter from 1 ms to 250 µs. Updated to new template.  |
|  *H | 5768943 | AESATMP9 | 06/09/2017 | Updated logo and copyright.  |

Document Number: 001-90865 Rev. *I Page 22 of 24Document History Page (continued)

Document Title: FM25V02A, 256-Kbit (32K × 8) Serial (SPI) F-RAM Document Number: 001-90865

|  Rev. | ECN No. | Orig. of Change | Submission Date | Description of Change  |
| --- | --- | --- | --- | --- |
|  *I | 6404910 | GVCH | 12/07/2018 | Updated Maximum Ratings:  |
|   |  |  |  | Replaced "–55 °C to +125 °C" with "–65 °C to +125 °C" in ratings corresponding to "Storage temperature".  |
|   |  |  |  | Updated Package Diagrams:  |
|   |  |  |  | spec 51-85066 – Changed revision from *H to *I.  |
|   |  |  |  | Updated to new template.  |# Sales, Solutions, and Legal Information 

## Worldwide Sales and Design Support

Cypress maintains a worldwide network of offices, solution centers, manufacturer's representatives, and distributors. To find the office closest to you, visit us at Cypress Locations.

## Products

Arm ${ }^{\circledR}$ Cortex ${ }^{\circledR}$ Microcontrollers
Automotive
Clocks \& Buffers
Interface
Internet of Things
Memory
Microcontrollers
PSoC
Power Management ICs
Touch Sensing
USB Controllers
Wireless Connectivity
cypress.com/arm
cypress.com/automotive
cypress.com/clocks
cypress.com/interface
cypress.com/iot
cypress.com/memory
cypress.com/mcu
cypress.com/psoc
cypress.com/pmic
cypress.com/touch
cypress.com/usb
cypress.com/wireless

## PSoC ${ }^{\circledR}$ Solutions

PSoC 1 | PSoC 3 | PSoC 4 | PSoC 5LP | PSoC 6 MCU

## Cypress Developer Community

Community | Projects | Video | Blogs | Training | Components

## Technical Support

cypress.com/support

[^0]
[^0]:    (c) Cypress Semiconductor Corporation, 2014-2018. This document is the property of Cypress Semiconductor Corporation and its subsidiaries, including Spansion LLC ("Cypress"). This document, including any software or firmware included or referenced in this document ("Software"), is owned by Cypress under the intellectual property laws and treaties of the United States and other countries worldwide. Cypress reserves all rights under such laws and treaties and does not, except as specifically stated in this paragraph, grant any license under its patents, copyrights, trademarks, or other intellectual property rights. If the Software is not accompanied by a license agreement and you do not otherwise have a written agreement with Cypress governing the use of the Software, then Cypress hereby grants you a personal, non-exclusive, nontransferable license (without the right to sublicense) (1) under its copyright rights in the Software (a) for Software provided in source code form, to modify and reproduce the Software solely for use with Cypress hardware products, only internally within your organization, and (b) to distribute the Software in binary code form externally to end users (either directly or indirectly through resellers and distributors), solely for use on Cypress hardware product units, and (2) under those claims of Cypress's patents that are infringed by the Software (as provided by Cypress, unmodified) to make, use, distribute, and import the Software solely for use with Cypress hardware products. Any other use, reproduction, modification, translation, or compilation of the Software is prohibited.
    TO THE EXTENT PERMITTED BY APPLICABLE LAW, CYPRESS MAKES NO WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, WITH REGARD TO THIS DOCUMENT OR ANY SOFTWARE OR ACCOMPANYING HARDWARE, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE. No computing device can be absolutely secure. Therefore, despite security measures implemented in Cypress hardware or software products, Cypress does not assume any liability arising out of any security breach, such as unauthorized access to or use of a Cypress product. In addition, the products described in these materials may contain design defects or errors known as errata which may cause the product to deviate from published specifications. To the extent permitted by applicable law, Cypress reserves the right to make changes to this document without further notice. Cypress does not assume any liability arising out of the application or use of any product or circuit described in this document. Any information provided in this document, including any sample design information or programming code, is provided only for reference purposes. It is the responsibility of the user of this document to properly design, program, and test the functionality and safety of any application made of this information and any resulting product. Cypress products are not designed, intended, or authorized for use as critical components in systems designed or intended for the operation of weapons, weapons systems, nuclear installations, life-support devices or systems, other medical devices or systems (including resuscitation equipment and surgical implants), pollution control or hazardous substances management, or other uses where the failure of the device or system could cause personal injury, death, or property damage ("Unintended Uses"). A critical component is any component of a device or system whose failure to perform can be reasonably expected to cause the failure of the device or system, or to affect its safety or effectiveness. Cypress is not liable, in whole or in part, and you shall and hereby do release Cypress from any claim, damage, or other liability arising from or related to all Unintended Uses of Cypress products. You shall indemnify and hold Cypress harmless from and against all claims, costs, damages, and other liabilities, including claims for personal injury or death, arising from or related to any Unintended Uses of Cypress products.
    Cypress, the Cypress logo, Spansion, the Spansion logo, and combinations thereof, WICED, PSoC, CapSense, EZ-USB, F-RAM, and Traveo are trademarks or registered trademarks of Cypress in the United States and other countries. For a more complete list of Cypress trademarks, visit cypress.com. Other names and brands may be claimed as property of their respective owners.