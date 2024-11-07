use bitcode::{DecodeOwned, Encode};
use criterion::{black_box, Criterion};
use rkyv::{rancor::Error, util::AlignedVec, Archive};

pub fn bench<T>(name: &'static str, c: &mut Criterion, data: &T)
where
    T: serde::Serialize + for<'a> serde::Deserialize<'a> + PartialEq,
{
    let mut group = c.benchmark_group(format!("{}/bitcode", name));
    let mut buffer = bitcode::Buffer::new();

    group.bench_function("serialize", |b| {
        b.iter(|| {
            black_box(bitcode::serialize(black_box(data)));
        })
    });

    let encoded = bitcode::serialize(data).unwrap();

    group.bench_function("deserialize", |b| {
        b.iter(|| {
            black_box(bitcode::deserialize::<T>(black_box(&encoded)));
        })
    });

    crate::bench_size(name, "bitcode", &encoded);

    assert!(bitcode::deserialize::<T>(&encoded).unwrap() == *data);

    group.finish();
}

pub fn bench2<T>(name: &'static str, c: &mut Criterion, data: &T)
where
    T: serde::Serialize + for<'a> serde::Deserialize<'a> + PartialEq + Archive,
{
    let mut group = c.benchmark_group(format!("{}/bitcode", name));
    let mut buffer = bitcode::Buffer::new();

    group.bench_function("serialize_rkyv", |b| {
        b.iter(|| {
            black_box(ser_archive(black_box(data)));
        })
    });

    let encoded = ser_archive(data);

    group.bench_function("deserialize_rkyv", |b| {
        b.iter(|| {
            black_box(de_archive::<T>(black_box(&encoded)));
        })
    });

    crate::bench_size(name, "bitcode + rkyv", &encoded);

    assert!(de_archive::<T>(&encoded) == *data);

    group.finish();
}

fn ser_archive<T>(data: &T) -> AlignedVec
where
    T: serde::Serialize + for<'a> serde::Deserialize<'a> + PartialEq + Archive,
{
    let b_enc: Bytes = bitcode::serialize(data).unwrap().into();
    rkyv::to_bytes::<Error>(&b_enc).unwrap()
}

fn de_archive<T>(bytes: &AlignedVec) -> T
where
    T: serde::Serialize + for<'a> serde::Deserialize<'a> + PartialEq + Archive,
{
    let bytes = &rkyv::access::<ArchivedBytes, Error>(&bytes).unwrap().bytes;
    bitcode::deserialize(bytes).unwrap()
}

#[derive(rkyv::Serialize, rkyv::Deserialize, rkyv::Archive)]
struct Bytes {
    pub bytes: Vec<u8>,
}

impl From<Vec<u8>> for Bytes {
    fn from(value: Vec<u8>) -> Self {
        Bytes { bytes: value }
    }
}

impl Into<Vec<u8>> for Bytes {
    fn into(self) -> Vec<u8> {
        self.bytes
    }
}
