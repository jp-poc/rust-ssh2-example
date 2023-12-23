# BASIC SSH using RUST Programming Language

Establish an ssh connection using rust and connect to a Cisco IOS XR router to execute a command.

## Pre-requisites:
- RUST (https://www.rust-lang.org/tools/install)
- Linux

# Install the following libraries
```shell
apt install install libssl-dev
apt install pkg-config
```

# Create a .env file for the ssh configuration.
```env
DEVICE_HOSTNAME="LOCALHOST"
DEVICE_USERNAME="USERNAME123"
DEVICE_PASSWORD="PASSWORD123"
```

# To Execute

```shell
cargo run
```

# Developer
- JP Mateo