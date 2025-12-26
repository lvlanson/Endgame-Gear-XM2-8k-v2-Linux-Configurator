### Installation

Copy udev rule
```bash
sudo cp 99-egg_xm28kv2.rules /etc/udev/rules.d/
```

If `"failed to open device"`error occurs try to reload udev 
```bash
sudo udevadm control --reload-rules
sudo udevadm trigger
```

or simply unplug and replug mouse.
