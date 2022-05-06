# wdds_randomizer 
Character randomizer for **The Walking Dead: The Definitive Series**. This tool will **attempt** to properly swap character (sometimes even inanimate objects - to spice it up) models. The word "attempt" is doing a lot of work here because the end results are often eldritch in nature.

### Supported Seasons
Currently, one season 1 is supported. Contributions are welcome to add support for other seasons, all you need to do is add the names of the corresponding archive's d3dmesh files (without extension) which you want to allow swapping on, within `data.rs`, and then edit the iterator in the `randomize` function within `randomize.rs`

The archives where you find these will be called something like `WDC_pc_WalkingDead{season}0{episode}_txmesh.ttarch2`.

## Usage
Run the executable and you will be given options to either restore backup or randomize. Before randomizing happens, the tool will try to back up the archives that it will modify (it will append `.ORIG` to their file names). Randomizing might take a long time (and also some disk space).

## Examples
You can see some example abominations [here](/abominations). Probably not all of them will look like this.

## License
This repository is licensed under MIT.

This tool makes use of Luigi Auriemma's `ttarchext` tool which can be found [here](https://aluigi.altervista.org/papers.htm#ttarchext)