BattleBit Offset Dumper
==========================

## Overview
This is a simple and robust offset dumper for the game BattleBit: Remastered

## Credits
Massive credits to [CasualX](https://github.com/CasualX) for [pelite](https://github.com/CasualX/pelite) and his [offset dumper](https://github.com/CasualX/apexdream/offsets/) for Apex Legends.


## Setup
1. Clone this repository
2. Install `cargo` and `rust`
3. Run one of the commands below...

## Build and Run
To dump the game offsets simply replace `/PATH_TO_BATTLEBIT/` whith your own path.
### Dump offsets:
```bash
cargo run --release -- "/PATH_TO_BATTLEBIT/GameAssembly.dll" ini > stdout.ini
```
### Human readable format:
```bash
cargo run --release -- "/PATH_TO_BATTLEBIT/GameAssembly.dll" human > stdout.md
```

## Contribution
Contributing is very appreciated! Just send a PR and I'll look at it