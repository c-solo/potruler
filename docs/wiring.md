# Chassis Control System Wiring Schema

**Board:** STM32F401CCU6 "Black Pill"
**Development Language:** Rust (`embassy-stm32`)
**Architecture:** Differential Drive (Skid-Steer), 2 encoder channels.

---

### Global Rust Configuration

- tim2: timer 2 is used for as embassy time driver (add time-driver-tim2 for embassy-stm32)
- tim5: timer 5 for wheel encoders

---

## Pinout Map

### 1. Main Computer Communication (Raspberry Pi 5)
*Protocol: UART (Serial). Baud-rate: 115200+.*

| STM32 Pin | Function    | Connection (RPi 5)         | Notes                        |
|:----------|:------------|:---------------------------|:-----------------------------|
| **PA2**   | USART2_TX   | **GPIO 15 (RXD)** (Pin 10) | Data FROM Robot TO Pi        |
| **PA3**   | USART2_RX   | **GPIO 14 (TXD)** (Pin 8)  | Data FROM Pi TO Robot        |
| **GND**   | Ground      | **GND** (Pin 6/9/14)       | **MUST** share common ground |

### 2. Motors (Power Stage)
*Drivers: 2x BTS7960. Timers used in PWM Generation mode.*

| STM32 Pin | Timer/Channel | Rust Function | Connection (Drivers)                     |
|:----------|:--------------|:--------------|:-----------------------------------------|
| **PA6**   | TIM3_CH1      | `SimplePwm`   | **Left** BTS7960 -> **RPWM** (Forward)   |
| **PA7**   | TIM3_CH2      | `SimplePwm`   | **Left** BTS7960 -> **LPWM** (Backward)  |
| **PB6**   | TIM4_CH1      | `SimplePwm`   | **Right** BTS7960 -> **RPWM** (Forward)  |
| **PB7**   | TIM4_CH2      | `SimplePwm`   | **Right** BTS7960 -> **LPWM** (Backward) |
| **3.3V**  | Power         | -             | **R_EN** & **L_EN** (Both drivers)       |
| **3.3V**  | Power         | -             | **VCC** (Driver logic)                   |
| **GND**   | Ground        | -             | **GND** (Driver logic)                   |

*> Note: Connect driver power terminals (B+/B-) to the 11.1V Battery. Connect M+/M- terminals to Motors.*

### 3. Encoders (Feedback)
*Wheel speed reading. Hardware timers used in QEI mode. Reading FRONT wheels only.*

| STM32 Pin | Timer    | Rust Function | Connection (Encoders)              |
|:----------|:---------|:--------------|:-----------------------------------|
| **PA8**   | TIM1_CH1 | `Qei`         | **Left** Front -> Phase **A**      |
| **PA9**   | TIM1_CH2 | `Qei`         | **Left** Front -> Phase **B**      |
| **PA0**   | TIM2_CH1 | `Qei`         | **Right** Front -> Phase **A**     |
| **PA1**   | TIM2_CH2 | `Qei`         | **Right** Front -> Phase **B**     |
| **3.3V**  | Power    | -             | Encoder **VCC** (Blue wire)        |
| **GND**   | Ground   | -             | Encoder **GND** (Black/Green wire) |

*> Warning: PA0 is the on-board KEY button. It will cease functioning as a button. Ensure encoders are powered by 3.3V 
to avoid damaging the PA0 pin (it is not 5V-tolerant).*

### 4. Sensors (Navigation & Obstacle Avoidance)

- **BNO085** - 9-DOF IMU (Inertial Measurement Unit)
- **VL53L0X** - Distance Sensor 
- **VL6180X** - Cliff Sensor

*Bus: I2C1. Address Conflict Resolution: GPIO Control (XSHUT).*

All sensors share the I2C bus (PB8/PB9). Since VL53L0X and VL6180X share the default address `0x29`, 
their **XSHUT** pins must be connected to individual GPIOs. The MCU must enable them sequentially 
at startup to assign unique addresses (e.g., `0x30`, `0x31`...).

| STM32 Pin | Function | Device                   | Device Pin        |
|:----------|:---------|:-------------------------|:------------------|
| **PB8**   | I2C1_SCL | All Sensors              | **SCL**           |
| **PB9**   | I2C1_SDA | All Sensors              | **SDA**           |
| **3.3V**  | Power    | All Sensors              | **VIN / VCC**     |
| **GND**   | Ground   | All Sensors              | **GND**           |
| **PB0**   | GPIO_OUT | VL53L0X (Front Distance) | **XSHUT**         |
| **PB1**   | GPIO_OUT | VL53L0X (Back Distance)  | **XSHUT**         |
| **PB12**  | GPIO_OUT | VL6180X (Front Cliff)    | **XSHUT / GPIO0** |
| **PB13**  | GPIO_OUT | VL6180X (Back Cliff)     | **XSHUT / GPIO0** |

### 5. Miscellaneous (Debug & Status)

| STM32 Pin      | Purpose                                                              |
|:---------------|:---------------------------------------------------------------------|
| **PC13**       | On-board Blue LED (Active Low). Use for "Heartbeat" indication.      |
| **G, CLK, IO** | ST-Link V2 header (GND, SWCLK, SWDIO). For flashing and RTT logging. |

---
