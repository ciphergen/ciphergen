use bytesize::ByteSize;
use core::fmt;
use digest::Digest;
use hex::encode;
use md5::Md5;
use sha1::Sha1;
use sha2::{Sha256, Sha512};
use tabled::builder::Builder;

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
    let length = buffer.len();
    let mut md5 = Md5::new();
    let mut sha1 = Sha1::new();
    let mut sha256 = Sha256::new();
    let mut sha512 = Sha512::new();

    md5.update(&buffer);
    sha1.update(&buffer);
    sha256.update(&buffer);
    sha512.update(&buffer);

    let shannon_entropy = shannon_entropy(&buffer);
    let absolute_entropy = normalized_absolute_entropy(&buffer);

    let report = Report {
        size: ByteSize::b(length as u64).to_string(),
        entropy: format!("{} (Shannon) | {} (Absolute)", shannon_entropy, absolute_entropy),
        md5: encode(md5.finalize()),
        sha1: encode(sha1.finalize()),
        sha256: encode(sha256.finalize()),
        sha512: encode(sha512.finalize())
    };

    report.to_string()
}

/// Calculate the Shannon entropy.
fn shannon_entropy(buffer: &[u8]) -> f64 {
    let length = buffer.len();
    let mut entropy = 0.0_f64;
    let mut counts = [0_u64; 256];

    // Create a histogram of the number of times each symbol occurred.
    for byte in buffer { counts[*byte as usize] += 1; }

    // Calculate the Shannon entropy.
    for count in counts {
        if count == 0 { continue; }

        let value = (count as f64) / (length as f64);

        entropy -= value * value.log2();
    }

    entropy
}

/// Calculate the normalized absolute entropy.
fn normalized_absolute_entropy(buffer: &[u8]) -> f64 {
    let length = buffer.len();

    // Calculate the Shannon entropy.
    let entropy = shannon_entropy(buffer);

    // Calculate and return the normalized absolute entropy.
    (length as f64) * entropy / 8.0
}

#[cfg(test)]
mod tests {
    use super::{shannon_entropy, normalized_absolute_entropy};

    #[test]
    fn zero_bytes_has_zero_shannon_entropy() {
        assert_eq!(shannon_entropy(b""), 0.0);
    }

    #[test]
    fn equal_distribution_has_full_shannon_entropy() {
        let mut bytes = [0_u8; 256];

        for index in 0..256 { bytes[index] = index as u8; }

        assert_eq!(shannon_entropy(&bytes), 8.0);
    }

    #[test]
    fn zero_bytes_has_zero_absolute_entropy() {
        assert_eq!(normalized_absolute_entropy(b""), 0.0);
    }

    #[test]
    fn equal_distribution_has_full_absolute_entropy() {
        let mut bytes = [0_u8; 256];

        for index in 0..256 { bytes[index] = index as u8; }

        assert_eq!(normalized_absolute_entropy(&bytes), 256.0);
    }
}
