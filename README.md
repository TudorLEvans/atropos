# Atropos

Atropos, one of the Greek fates, was responsible for bringing mortals to their end. Like Atropos, I intend this rust-based tool to embody that sense of the inevitability of time. Specifically, I have yet to find the perfect metronome, so I thought I should build one myself.

Atropos is built on top of cpal, which offers fairly low level access to sound hardware.

The tool can be run with the command:

```
cargo run
```

This will run the metronome for one minute with hardcoded configuration.