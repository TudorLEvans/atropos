# Atropos

Atropos, one of the Greek fates, was responsible for bringing mortals to their end. Like Atropos, I intend this rust-based tool to embody that sense of the inevitability of time. Specifically, I have yet to find the perfect metronome, so I thought I should build one myself.

Atropos is built on top of cpal, which offers fairly low level access to sound hardware.

The project is at a very early stage, and currently only supports generating a single sound - a square wave - produced with the command:

```
cargo run
```