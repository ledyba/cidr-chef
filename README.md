# CIDR chef

 - [![Build on Linux](https://github.com/ledyba/cidr-chef/workflows/Build%20on%20Linux/badge.svg)](https://github.com/ledyba/cidr-chef/actions?query=workflow%3A%22Build+on+Linux%22)
   - [![Build single binary on Linux](https://github.com/ledyba/cidr-chef/workflows/Build%20single%20binary%20on%20Linux/badge.svg)](https://github.com/ledyba/cidr-chef/actions?query=workflow%3A%22Build+single+binary+on+Linux%22)
 - [![Build on macOS](https://github.com/ledyba/cidr-chef/workflows/Build%20on%20macOS/badge.svg)](https://github.com/ledyba/cidr-chef/actions?query=workflow%3A%22Build+on+macOS%22)
 - [![Build on Windows](https://github.com/ledyba/cidr-chef/workflows/Build%20on%20Windows/badge.svg)](https://github.com/ledyba/cidr-chef/actions?query=workflow%3A%22Build+on+Windows%22)


CIDR chef is a swiss-army knife to cook [CIDRs](https://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing) and IP addresses (Or at least it's supposed to be!).

cidr-chef both supports IPv4/v6.

## How to build.

```bash
git clone git@github.com:ledyba/cidr-chef.git
cd cidr-chef
cargo build
target/debug/cidr-chef -h
```

## Current supported operation

### CIDR set computation

```bash
% echo 'Get all CIDRs except private IPv4 addresses!'
% cidr-chef calc +0.0.0.0/0 -10.0.0.0/8 -172.16.0.0/12 -192.168.0.0/16
0.0.0.0/5
8.0.0.0/7
11.0.0.0/8
12.0.0.0/6
16.0.0.0/4
32.0.0.0/3
64.0.0.0/2
128.0.0.0/3
160.0.0.0/5
168.0.0.0/6
172.0.0.0/12
172.32.0.0/11
172.64.0.0/10
172.128.0.0/9
173.0.0.0/8
174.0.0.0/7
176.0.0.0/4
192.0.0.0/9
192.128.0.0/11
192.160.0.0/13
192.169.0.0/16
192.170.0.0/15
192.172.0.0/14
192.176.0.0/12
192.192.0.0/10
193.0.0.0/8
194.0.0.0/7
196.0.0.0/6
200.0.0.0/5
208.0.0.0/4
224.0.0.0/3
```