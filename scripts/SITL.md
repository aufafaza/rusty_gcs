# Lightweight ArduPilot SITL

Uses the official ArduCopter 4.6.3 Linux x86_64 binary.  
 

To install :

```bash
bash scripts/setup-sitl.sh
```

From the project root, start the simulator:

```bash
bash scripts/run-sitl.sh
```

In another terminal: 

```bash
cargo run --offline -j 1 -- udpin:127.0.0.1:14550
```
Source: https://firmware.ardupilot.org/Copter/stable-4.6.3/SITL_x86_64_linux_gnu/
