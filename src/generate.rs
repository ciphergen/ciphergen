use std::sync::{atomic::{AtomicUsize, Ordering}, mpsc::Sender};

use rand::thread_rng;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::{generators::*, markov::Generator};

const LINE_FEED: u8 = b'\n';

pub enum UsernameKind {
    Simple,
    Complex
}

fn create_serial(sender: Sender<Vec<u8>>, closure: impl FnOnce() -> Vec<u8>) {
    let buffer = closure();

    sender.send(buffer).unwrap();
}

fn create_parallel(sender: Sender<Vec<u8>>, count: Option<usize>, closure: impl Fn() -> Vec<u8> + Send + Sync) {
    let max = count.unwrap_or(1);
    let range = 0..max;
    let counter = AtomicUsize::new(0);

    range.into_par_iter().for_each(|_| {
        let mut buffer = closure();

        if counter.fetch_add(1, Ordering::Relaxed) != max - 1 { buffer.push(LINE_FEED); }

        sender.send(buffer).unwrap();
    });
}

pub fn create_bytes(sender: Sender<Vec<u8>>, length: usize) {
    create_serial(sender, || generate_bytes(length));
}

pub fn create_hex(sender: Sender<Vec<u8>>, uppercase: bool, length: usize) {
    create_serial(sender, || generate_hex(uppercase, length));
}

pub fn create_base64(sender: Sender<Vec<u8>>, url_safe: bool, length: usize) {
    create_serial(sender, || generate_base64(url_safe, length));
}

pub fn create_password(sender: Sender<Vec<u8>>, character_set: &[char], length: usize, count: Option<usize>) {
    create_parallel(sender, count, || generate_password(character_set, length));
}

pub fn create_passphrase(sender: Sender<Vec<u8>>, wordlist: &[String], separator: &str, length: usize, count: Option<usize>) {
    create_parallel(sender, count, || generate_passphrase(wordlist, separator, length));
}

pub fn create_username(sender: Sender<Vec<u8>>, capitalize: bool, kind: UsernameKind, length: usize, count: Option<usize>) {
    match kind {
        UsernameKind::Simple => create_parallel(sender, count, || generate_simple_username(capitalize, length)),
        UsernameKind::Complex => create_parallel(sender, count, || generate_complex_username(capitalize, length))
    }
}

pub fn create_digits(sender: Sender<Vec<u8>>, length: usize, count: Option<usize>) {
    create_parallel(sender, count, || generate_digits(length));
}

pub fn create_number(sender: Sender<Vec<u8>>, minimum: usize, maximum: usize, count: Option<usize>) {
    create_parallel(sender, count, || generate_number(minimum, maximum));
}

pub fn create_markov(sender: Sender<Vec<u8>>, generator: &Generator, capitalize: bool, minimum: usize, maximum: usize, count: Option<usize>) {
    create_parallel(sender, count, || generate_markov(generator, capitalize, minimum, maximum, &mut thread_rng()));
}
