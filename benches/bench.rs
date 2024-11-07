#[allow(unused_imports)]
use criterion::{black_box, criterion_main, Criterion};
use rand_pcg::Lcg64Xsh32;

#[cfg(feature = "bitcode")]
use rust_serialization_benchmark::bench_bitcode;

use rust_serialization_benchmark::generate_vec;

fn bench_log(c: &mut Criterion) {
    use rust_serialization_benchmark::datasets::log::{Log, Logs};

    const BENCH: &'static str = "log";

    // nothing up our sleeves, state and stream are first 20 digits of pi
    const STATE: u64 = 3141592653;
    const STREAM: u64 = 5897932384;

    let mut rng = Lcg64Xsh32::new(STATE, STREAM);

    const LOGS: usize = 10_000;
    let data = Logs {
        logs: generate_vec::<_, Log>(&mut rng, LOGS..LOGS + 1),
    };

    #[cfg(feature = "bitcode")]
    bench_bitcode::bench(BENCH, c, &data);

    #[cfg(feature = "bitcode")]
    bench_bitcode::bench2(BENCH, c, &data);
}

fn bench_mesh(c: &mut Criterion) {
    use rust_serialization_benchmark::datasets::mesh::{Mesh, Triangle};

    const BENCH: &'static str = "mesh";

    // nothing up our sleeves, state and stream are first 20 digits of pi
    const STATE: u64 = 3141592653;
    const STREAM: u64 = 5897932384;

    let mut rng = Lcg64Xsh32::new(STATE, STREAM);

    const TRIANGLES: usize = 125_000;
    let data = Mesh {
        triangles: generate_vec::<_, Triangle>(&mut rng, TRIANGLES..TRIANGLES + 1),
    };

    #[cfg(feature = "bitcode")]
    bench_bitcode::bench(BENCH, c, &data);

    #[cfg(feature = "bitcode")]
    bench_bitcode::bench2(BENCH, c, &data);
}

fn bench_minecraft_savedata(c: &mut Criterion) {
    use rust_serialization_benchmark::datasets::minecraft_savedata::{Player, Players};

    const BENCH: &'static str = "minecraft_savedata";

    // nothing up our sleeves, state and stream are first 20 digits of pi
    const STATE: u64 = 3141592653;
    const STREAM: u64 = 5897932384;

    let mut rng = Lcg64Xsh32::new(STATE, STREAM);

    const PLAYERS: usize = 500;
    let data = Players {
        players: generate_vec::<_, Player>(&mut rng, PLAYERS..PLAYERS + 1),
    };

    #[cfg(feature = "bilrost")]
    bench_bilrost::bench(BENCH, c, &data);

    #[cfg(feature = "bincode1")]
    bench_bincode1::bench(BENCH, c, &data);

    #[cfg(feature = "bincode")]
    bench_bincode::bench(BENCH, c, &data);

    #[cfg(feature = "bitcode")]
    bench_bitcode::bench(BENCH, c, &data);
    #[cfg(feature = "bitcode")]
    bench_bitcode::bench2(BENCH, c, &data);
}

fn bench_mk48(c: &mut Criterion) {
    use rust_serialization_benchmark::datasets::mk48::Updates;

    const BENCH: &'static str = "mk48";

    // nothing up our sleeves, state and stream are first 20 digits of pi
    const STATE: u64 = 3141592653;
    const STREAM: u64 = 5897932384;

    let mut rng = Lcg64Xsh32::new(STATE, STREAM);

    const UPDATES: usize = 1000;
    let data = Updates {
        updates: generate_vec(&mut rng, UPDATES..UPDATES + 1),
    };

    #[cfg(feature = "bitcode")]
    bench_bitcode::bench(BENCH, c, &data);
    #[cfg(feature = "bitcode")]
    bench_bitcode::bench2(BENCH, c, &data);
}

#[cfg(feature = "pprof")]
mod profiling {
    use criterion::profiler::Profiler;
    use pprof::ProfilerGuard;
    use std::ffi::c_int;
    use std::fs::File;
    use std::path::Path;

    pub struct FlamegraphProfiler<'a> {
        frequency: c_int,
        active_profiler: Option<ProfilerGuard<'a>>,
    }

    impl<'a> FlamegraphProfiler<'a> {
        pub fn new(frequency: c_int) -> Self {
            FlamegraphProfiler {
                frequency,
                active_profiler: None,
            }
        }
    }

    impl<'a> Profiler for FlamegraphProfiler<'a> {
        fn start_profiling(&mut self, _benchmark_id: &str, _benchmark_dir: &Path) {
            self.active_profiler = Some(ProfilerGuard::new(self.frequency).unwrap());
        }

        fn stop_profiling(&mut self, _benchmark_id: &str, benchmark_dir: &Path) {
            std::fs::create_dir_all(benchmark_dir).unwrap();
            let flamegraph_path = benchmark_dir.join("flamegraph.svg");
            let flamegraph_file = File::create(&flamegraph_path)
                .expect("File system error while creating flamegraph.svg");
            if let Some(profiler) = self.active_profiler.take() {
                profiler
                    .report()
                    .build()
                    .unwrap()
                    .flamegraph(flamegraph_file)
                    .expect("Error writing flamegraph");
            }
        }
    }
}

pub fn criterion_benchmark(c: &mut Criterion) {
    bench_log(c);
    bench_mesh(c);
    bench_minecraft_savedata(c);
    bench_mk48(c);
}

pub fn benches() {
    let criterion = Criterion::default();
    #[cfg(feature = "pprof")]
    let criterion = criterion.with_profiler(profiling::FlamegraphProfiler::new(100));
    let mut criterion = criterion.configure_from_args();
    criterion_benchmark(&mut criterion);
}

criterion_main!(benches);
