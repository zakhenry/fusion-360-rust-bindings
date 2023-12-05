# Instructions

Run
```shell
cargo build && cp target/debug/liboxidised_fusion_360_addin.dylib addin/oxidised-fusion-360-addin/oxidised-fusion-360-addin.dylib
```

And in Fusion 360 add addin by selecting the addin/oxidised-fusion-360-addin dir.

The current build does nothing but write to file, view this with

```shell
tail -f /tmp/debug.log
```
