# Base Protocol

## Terminology
### `bmRequestType`
- 1 byte 
- specifying the type of request and recipient 
- the following table describes `bmRequestType` with respect to its binary structure
    - `D0-D4` defines the recipient (4bits)
        - `0` device
        - `1` interface
        - `2` endpoint
        - `3` other
        - `4-31` reserved
    - `D5-D6` defines type of request (2bits)
        - `0` standard
        - `1` class
        - `2` vendor
        - `3` reserved
    - `D7` defines the direction (get/set) (1bit)
        - `0` host to device
        - `1` device to host
- example: `0xa1` translates to `0b10100001`, we read from left to right (`D7->D0`):
    - `1` -> device to host
    - `01`-> class 
    - `00001` -> interface 

- for further information read the usb specification pdf in the repo from page 248
## Get Headers
- bmRequestType: `0xa1` (device to host, class, interface)
- bRequest: GET_REPORT `0x01`
- wValue: `0x03a1`
    - 
    - ReportID: 161
    - ReportType: Feature (3)
- wIndex: `0x0001`
- wLength: 64

## Set Headers
- bmRequestType: `0x21`
- bRequest: SET_REPORT `0x09`
- wValue: `0x03a1`
    - 
    - ReportID: 161
    - ReportType: Feature (3)
- wIndex: `0x0001`
- wLength: 64

## Functions <-> Addresses
Interface_Number: `0x01`
wLength: 1041 bytes (ProfileLength) 
ATTR <-> PROFILE_POS : RANGE
- Polling Rate                  <-> 21:     [8, 4, 2, 1]  <=> [1000, 2000, 4000, 8000]
- Slamclick Filter              <-> 22:     [0x00 - 0x01] <=> {OFF - ON}
- Disable LED on Liftoff        <-> 24:     [0x00 - 0x01] <=> {OFF - ON}
- LOD                           <-> 25:     [0x00 - 0x0a] <=> [0.7mm - 1.7mm]
- Angle Snapping                <-> 26:     [0x00 - 0x01] <=> {OFF - ON}
- Ripple Control                <-> 27:     [0x00 - 0x01] <=> {OFF - ON}
- Motion Sync                   <-> 28:     [0x00 - 0x01] <=> {OFF - ON}
- CPI Levels                    <-> 30:     [0x01 - 0x04] <=> [1-4]
- CPI Prof 1 Left               <-> 52,53:  [0x0a 0x00 - 0x30 0x75] <=> [10 - 30000] (stepsize: 10)
- CPI Prof 1 Right              <-> 54,55:  [0x0a 0x00 - 0x30 0x75] <=> [10 - 30000] (stepsize: 10)
- CPI Prof 2 Left               <-> 57,58:  [0x0a 0x00 - 0x30 0x75] <=> [10 - 30000] (stepsize: 10)
- CPI Prof 2 Right              <-> 59,60:  [0x0a 0x00 - 0x30 0x75] <=> [10 - 30000] (stepsize: 10)
- CPI Prof 3 Left               <-> 62,63:  [0x0a 0x00 - 0x30 0x75] <=> [10 - 30000] (stepsize: 10)
- CPI Prof 3 Right              <-> 64,65:  [0x0a 0x00 - 0x30 0x75] <=> [10 - 30000] (stepsize: 10)
- CPI Prof 4 Left               <-> 67,68:  [0x0a 0x00 - 0x30 0x75] <=> [10 - 30000] (stepsize: 10)
- CPI Prof 4 Right              <-> 69,70:  [0x0a 0x00 - 0x30 0x75] <=> [10 - 30000] (stepsize: 10)
- LB MF SPDT                    <-> 77:     {0x00-0x19, 0xf1, 0xf0} <=> {[0-25], GX Speed Mode, GX Safe Mode}
- RB MF SPDT                    <-> 84:     {0x00-0x19, 0xf1, 0xf0} <=> {[0-25], GX Speed Mode, GX Safe Mode}
- Middle Btn Multiclick Filter  <-> 91:     [0x00 - 0x19] <=> [0 - 25]
- Forward Btn Multiclick Filter <-> 98:     [0x00 - 0x19] <=> [0 - 25]
- Back Btn Multiclick Filter    <-> 105:    [0x00 - 0x19] <=> [0 - 25]
- Glass Mode                    <-> 127:    [0x00 - 0x01] <=> {OFF - ON} 
    - Important: LOD            <-> 25:     [0x00 - 0x01] <=> [1.0mm, 2.0mm]
    - Internal Fix of Polling Rate to 1000hz if Glass Mode = ON
- Sensor Angle Tuning           <-> 128:    [0x81 - 0x7b] <=> [-127, 127]
- Right Button                  <-> 78, 79:     [MOUSE BUTTON MODES]
    - Data Fields               <-> 80, 81, 82, 83 
- Middle Button                 <-> 85, 86
    - Data Fields               <-> 87, 88, 89, 90
- Forward Button                <-> 92, 93
    - Data Fields               <-> 94, 95, 96, 97
- Back Button                   <-> 99, 100
    - Data Fields               <-> 101, 102, 103, 104
- Wheel Up                      <-> 113, 114
    - Data Fields               <-> 115, 116, 117, 118
    - Wheel up is not supposed to have `Fixed CPI` function
- Wheel Down                    <-> 120, 121
    - Data Fields               <-> 122, 123, 124, 125
    - Wheel down is not supposed to have `Fixed CPI` function
- Left Handed Mode              <-> 72, 79: Left Handed {2, 1}, Right Handed {1, 2} (Switch Adresses for LEFT<-> RIGHT)

- I understand that double click filter is not... <-> accept is fixed to 130 = 0x01

### Mouse Button Modes

- Left Click      0x00 0x01
- Right Click     0x00 0x02
- Middle Click    0x00 0x04
- Forward         0x00 0x10 
- Back            0x00 0x08
- Scroll Up       0x01 0x01
- Scroll Down     0x01 0xff


- CPI Loop        0x09 0xf1
- Fixed CPI       0xc0 0x00 -> Datafield CPI Prof Values     

- Play/Pause      0x20 0xcd
- Next            0x20 0xb5
- Previous        0x20 0xb6
- Mute            0x20 0xe2
- Volume Up       0x20 0xe9
- Volume Down     0x20 0xea
- Browser         0x18 0x96 -> Datafield 0x01 0x00 0x00 0x00
- Explorer        0x18 0x94 -> Datafield 0x01 0x00 0x00 0x00 

- Disable         0xff 0x00

- Keyboard Key +  0x02 DECORATOR -> Datafield describes Key

Field 1:
-> Keys are HID usage IDs! (Human Interface Device)
A 0x04
B 0x05
...
Z 0x1d
1 0x1e
...
9 0x26
0 0x27
ENTER 0x28

DECORATOR:
CTRL    - 0x01  - b0001
SHIFT   - 0x02  - b0010
ALT     - 0x04  - b0100
WIN     - 0x08  - b1000

Combinations are simply the bitstring or added decorators

SHIFT+ALT = 0x06 = b0110
