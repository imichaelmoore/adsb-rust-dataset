# dump1090 SBS-1 Message Forwarder to DataSet

This project provides a utility to capture SBS-1 format messages from dump1090's output and then forwards them to SentinelOne's DataSet for further analysis and storage.

## Table of Contents

- [About dump1090 and SBS-1](#about-dump1090-and-sbs-1)
- [About DataSet (formerly Scalyr)](#about-dataset-formerly-scalyr)
- [Setting Up dump1090 with rtl_sdr](#setting-up-dump1090-with-rtl_sdr)
- [Requirements](#requirements)
- [Getting Started](#getting-started)
- [Running Services with pmtr](#running-services-with-pmtr)
- [Setting up pmtr as a launchd service](#setting-up-pmtr-as-a-launchd-service)
- [Contributions](#contributions)
- [License](#license)

## About dump1090 and SBS-1

`dump1090` is a popular ADS-B (Automatic Dependent Surveillance-Broadcast) Mode S decoder built specifically for RTL-SDR devices. When run, it outputs aircraft data in SBS-1 format on port 30003. This format provides real-time information about airborne aircraft, including details like speed, position, altitude, and more.

## About DataSet (formerly known as Scalyr)

SentinelOne's DataSet, formerly known as Scalyr, offers high-speed logging and server metrics for engineers. It's known for its lightning-fast search capabilities. Engineers often rely on DataSet to troubleshoot server issues, understand application behaviors, and ensure smooth operations.

## Setting Up dump1090 with rtl_sdr

To get dump1090 producing SBS-1 messages from an RTL-SDR:

1. Ensure you have an RTL-SDR dongle.
2. Install `dump1090`:

   sudo apt-get install dump1090

3. Run `dump1090` in interactive mode:

   dump1090 --interactive

4. SBS-1 formatted data will be available on port `30003`. Ensure no firewall or other network restrictions block this port.

## Requirements

- Rust
- Cargo (Comes bundled with Rust)
- dump1090 (or a derivative, like [dump1090-fa](https://github.com/topics/dump1090-fa) or [dump1090-mutability](https://github.com/adsb-related-code/dump1090-mutability)) or [PiAware](https://www.flightaware.com/adsb/piaware/)

## Getting Started

1. Clone this repository.
2. If not installed, [install Rust and Cargo](https://www.rust-lang.org/learn/get-started).
3. Adjust constants in `main.rs` for your setup, especially tokens and DataSet endpoints.
4. Run:

   cargo run

Ensure `dump1090` is running and emitting SBS-1 messages on port `30003`.

## Creating a binary

Just run `cargo build --release` and a standalone binary `adsb` will be created in `/target/release`.

## Running Services with pmtr

[`pmtr`](https://troydhanson.github.io/pmtr/) is a versatile tool for running background services. It restarts services that fail and can manage both `dump1090` and this project as services.

Create a `pmtr.conf` configuration file in `/etc`:

    job {
      name dump1090
      cmd /path/to/dump1090 --net
    }

    job {
      name sbs1-forwarder
      cmd /path/to/adsb_binary
    }

Replace `/path/to/` with the appropriate paths.

Start the services with:

    pmtr -c /etc/pmtr.conf

Both `dump1090` and the SBS-1 forwarder will now run as managed background services. `pmtr` will restart them if they fail.

## Setting up pmtr as a launchd service

**Option 1: RPM**

A RHEL/CentOS 7 x86_64 RPM package for pmtr can be found [here](https://troydhanson.github.io/pmtr/).

**Option 2: Build from source**

1.  Install the prerequisite tools:

    _Ubuntu:_

        sudo apt install git build-essential autoconf automake

    _RedHat/CentOS:_

        sudo yum install git gcc autoconf automake make

2.  Clone pmtr:

        git clone https://github.com/troydhanson/pmtr.git

3.  Build and install:

        cd pmtr
        ./autogen.sh
        ./configure --bindir=/usr/bin --sysconfdir=/etc
        make
        sudo make install
        sudo touch /etc/pmtr.conf

4.  Set up initscript to start pmtr automatically at boot:

        cd initscripts
        sudo ./setup-initscript --auto

This is where pmtr reports on starting jobs, or on any errors in parsing the configuration file. Any output generated by the jobs also appears in the syslog by default.

## Contributions

Pull requests are welcome! Please ensure that contributions adhere to the current coding style.

## License

This code is licensed under the [MIT License](https://github.com/imichaelmoore/adsb-rust-dataset/blob/main/LICENSE).

