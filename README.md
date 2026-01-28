# VNP4 Runtime Server

## Overview

VNP4 Runtime Server is a P4Runtime-compliant gRPC server implementation for AMD Vitis Networking P4 IP. While the Vitis Networking P4 IP provides a shared library (`libvitisnetp4drv.so`) with APIs for IP table manipulation, it does not natively support the P4Runtime specification. This server bridges that gap by providing a standard P4Runtime interface, enabling:

- **Standard P4Runtime Protocol Support**: Full gRPC-based P4Runtime API implementation for device control
- **Vitis Networking P4 Integration**: Direct integration with AMD Vitis Networking P4 driver APIs
- **Testing and Development Environment**: Ideal for P4 users who need a testing environment or want to provide SDN capabilities
- **Hardware Compatibility**: Works with any hardware implementation using Vitis Networking P4 IP


## Requirements

- Install Rust: https://rust-lang.org/tools/install/
- Install Vivado 2024.2

### Building

#### Step 0: Clone Repository

```shell
$ git clone https://github.com/iHalt10/vnp4_runtime_server
```

#### Step 1: Build Vitis Networking P4 Driver

run vivado tcl:
```tcl
set p4_file "./main.p4"
set ip_path "./ip"
set ip_instance_name vitis_net_p4_core
create_ip -name vitis_net_p4 -vendor xilinx.com -library ip -module_name $ip_instance_name -dir $ip_path
set vitis_net_p4_config [dict create]
dict set vitis_net_p4_config CONFIG.P4_FILE "$p4_file"
dict set vitis_net_p4_config CONFIG.TDATA_NUM_BYTES 64
set_property -dict $vitis_net_p4_config [get_ips $vitis_net_p4]
generate_target all [get_ips $ip_instance_name]
```

run shell:
```shell
$ export DRIVER_ROOT= "./driver"
$ cd ./ip/vitis_net_p4_core/src/sw/drivers
$ make INSTALL_ROOT="$DRIVER_ROOT"
```

#### Step 2: Build VNP4 Runtime Server

```shell
$ export DRIVER_ROOT="./driver"
$ cargo build --release
```
output ./target/release/vnp4rs


### How To Use

#### step 1: Generate target-config

```shell
$ vnp4rs generate-target-config ./driver/lib/libvitisnetp4drv.so ./ip/vitis_net_p4_core/main.json vitis_net_p4_core
```

#### step 2: Run server

```shell
$ cat config.yaml
server:
  address: 0.0.0.0
  port: 50051

devices:
- id: 1
  mmio:
    path: /sys/bus/pci/devices/xxxx:xx:xx.x/resource2
    size: 8192
    offset: 0x100000
  cpuPort: eth0
  targetConfig: ./target-config.json

$ vnp4rs run-server config.yaml
```

## Features

This project is under active development. Currently implemented features:
- ✅ Table Entries: EXACT match support
- ✅ StreamChannel: Primary controller arbitration
- ✅ Read/Write operations for table entries

Features planned for future releases:
- ⏳ Table Entries: LPM, Ternary, and Range match types
- ⏳ Counters and Meters: DirectCounter, Counter, Meter entries
- ⏳ CPU Port: Packet I/O support

We are actively working on expanding the feature set. Contributions and feedback are welcome!
