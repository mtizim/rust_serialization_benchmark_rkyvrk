```
log/bitcode/size 703710
log/bitcode/zlib 288826
log/bitcode/zstd 229755
log/bitcode/zstd_time time:   [1.5383 ms 1.5383 ms 1.5383 ms] 149 MB/s
log/bitcode + rkyv/size 703720
log/bitcode + rkyv/zlib 288837
log/bitcode + rkyv/zstd 229764
log/bitcode + rkyv/zstd_time time:   [1.6039 ms 1.6039 ms 1.6039 ms] 143 MB/s
mesh/bitcode/size 6000006
mesh/bitcode/zlib 5182295
mesh/bitcode/zstd 4923880
mesh/bitcode/zstd_time time:   [7.7820 ms 7.7820 ms 7.7820 ms] 632 MB/s
mesh/bitcode + rkyv/size 6000016
mesh/bitcode + rkyv/zlib 5182303
mesh/bitcode + rkyv/zstd 4923911
mesh/bitcode + rkyv/zstd_time time:   [7.8955 ms 7.8955 ms 7.8955 ms] 623 MB/s
Benchmarking minecraft_savedata/bitcode/serialize: Collecting 100 samples in estimated 6.0127 s (20minecraft_savedata/bitcode/serialize
                        time:   [295.18 µs 295.36 µs 295.55 µs]
Found 23 outliers among 100 measurements (23.00%)
  12 (12.00%) low severe
  8 (8.00%) low mild
  3 (3.00%) high severe
Benchmarking minecraft_savedata/bitcode/deserialize: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 5.3s, enable flat sampling, or reduce sample count to 60.
Benchmarking minecraft_savedata/bitcode/deserialize: Collecting 100 samples in estimated 5.2590 s (minecraft_savedata/bitcode/deserialize
                        time:   [1.0409 ms 1.0416 ms 1.0426 ms]
Found 7 outliers among 100 measurements (7.00%)
  1 (1.00%) high mild
  6 (6.00%) high severe
minecraft_savedata/bitcode/size 327711
minecraft_savedata/bitcode/zlib 200813
minecraft_savedata/bitcode/zstd 183019
minecraft_savedata/bitcode/zstd_time time:   [437.7680 µs 437.7680 µs 437.7680 µs] 418 MB/s

minecraft_savedata/bitcode/serialize_rkyv
                        time:   [300.27 µs 300.63 µs 301.01 µs]
Found 6 outliers among 100 measurements (6.00%)
  1 (1.00%) high mild
  5 (5.00%) high severe
Benchmarking minecraft_savedata/bitcode/deserialize_rkyv: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 5.2s, enable flat sampling, or reduce sample count to 60.
minecraft_savedata/bitcode/deserialize_rkyv
                        time:   [1.0466 ms 1.0469 ms 1.0473 ms]
Found 12 outliers among 100 measurements (12.00%)
  1 (1.00%) low severe
  8 (8.00%) low mild
  3 (3.00%) high severe
minecraft_savedata/bitcode + rkyv/size 327720
minecraft_savedata/bitcode + rkyv/zlib 200822
minecraft_savedata/bitcode + rkyv/zstd 183024
minecraft_savedata/bitcode + rkyv/zstd_time time:   [446.3890 µs 446.3890 µs 446.3890 µs] 410 MB/s

mk48/bitcode/size 948502
mk48/bitcode/zlib 857324
mk48/bitcode/zstd 832628
mk48/bitcode/zstd_time time:   [1.9855 ms 1.9855 ms 1.9855 ms] 419 MB/s
mk48/bitcode + rkyv/size 948512
mk48/bitcode + rkyv/zlib 857333
mk48/bitcode + rkyv/zstd 832638
mk48/bitcode + rkyv/zstd_time time:   [1.9704 ms 1.9704 ms 1.9704 ms] 422 MB/s

```
