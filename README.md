BattleBit Offset Dumper
==========================

## Overview
Robust offset dumper for the game *BattleBit: Remastered*

## Credits
Massive credits to [CasualX](https://github.com/CasualX) for [pelite](https://github.com/CasualX/pelite) and his [offset dumper](https://github.com/CasualX/apexdream/tree/master/offsets) for Apex Legends.

## Setup
1. Clone this repository
2. Install `cargo` and `rust`
3. Run one of the commands below...

## Build and Run
To dump the game offsets simply replace `/PATH_TO_BATTLEBIT/` with your own path.

### Dump offsets:
```bash
cargo run --release -- "/PATH_TO_BATTLEBIT/GameAssembly.dll" ini > stdout.ini
```
### Human readable format:
```bash
cargo run --release -- "/PATH_TO_BATTLEBIT/GameAssembly.dll" human > stdout.md
```

## Contribution
Contributions are highly valued! Simply submit a pull request, and I will review it promptly.
