# Local simplemad 0.9.0 patch

Original: Benjamin Dykstra, https://github.com/bendykst/simple-mad.rs (MIT).

Live Radio+ changes: track actual input length; preserve only valid undecoded bytes; refill incrementally instead of filling 32 KiB; replace recursive BufLen handling with a loop; handle Interrupted reads and real EOF guard bytes; reject an overfull undecodable buffer; modernize deprecated integer constants and iteration syntax.

The regression test in src/streams/mod.rs compares exact PCM output for complete input, 180/350/700/1200-byte fragments, single bytes and frame-boundary fragments.
