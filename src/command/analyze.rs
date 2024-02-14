use core::fmt;
use std::collections::HashMap;
use tabled::builder::Builder;
use digest::Digest;
use md5::Md5;
use sha1::Sha1;
use sha2::{Sha256, Sha512};
use bytesize::ByteSize;
use hex::encode;

struct Report {
    size: String,
    entropy: String,
    md5: String,
    sha1: String,
    sha256: String,
    sha512: String
}

impl fmt::Display for Report {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let mut builder = Builder::new();

        builder.push_record(["Size", &self.size]);
        builder.push_record(["Entropy", &self.entropy]);
        builder.push_record(["MD5", &self.md5]);
        builder.push_record(["SHA1", &self.sha1]);
        builder.push_record(["SHA256", &self.sha256]);
        builder.push_record(["SHA512", &self.sha512]);

        write!(formatter, "{}", builder.build())
    }
}

pub fn analyze(buffer: Vec<u8>) -> String {
    let mut md5 = Md5::new();
    let mut sha1 = Sha1::new();
    let mut sha256 = Sha256::new();
    let mut sha512 = Sha512::new();

    md5.update(&buffer);
    sha1.update(&buffer);
    sha256.update(&buffer);
    sha512.update(&buffer);

    let report = Report {
        size: ByteSize::b(buffer.len() as u64).to_string(),
        entropy: shannon_entropy(&buffer).to_string(),
        md5: encode(md5.finalize()),
        sha1: encode(sha1.finalize()),
        sha256: encode(sha256.finalize()),
        sha512: encode(sha512.finalize())
    };

    report.to_string()
}

pub fn shannon_entropy(buffer: &Vec<u8>) -> f64 {
    let length = buffer.len() as f64;
    let histogram = buffer.iter().fold(
        HashMap::new(), |mut acc, e| {
            *acc.entry(e).or_insert(0) = *acc.entry(e).or_insert(0) + 1;
            acc
        }
    );

    histogram.values().fold(0f64, |ac, &x| {
        let f = x as f64 / length;
        ac - (f * f.log2())
    })
}
