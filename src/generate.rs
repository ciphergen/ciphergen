use std::sync::{atomic::{AtomicUsize, Ordering}, mpsc::Sender};

use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::generators::*;

const LINE_FEED: u8 = b'\n';

pub enum UsernameKind {
    Simple,
    Complex
}

pub fn create_bytes(sender: Sender<Vec<u8>>, length: usize) {
    let buffer = generate_bytes(length);

    sender.send(buffer).unwrap();
}

pub fn create_hex(sender: Sender<Vec<u8>>, uppercase: bool, length: usize) {
    let buffer = generate_hex(uppercase, length);

    sender.send(buffer).unwrap();
}

pub fn create_base64(sender: Sender<Vec<u8>>, url_safe: bool, length: usize) {
    let buffer = generate_base64(url_safe, length);

    sender.send(buffer).unwrap();
}

pub fn create_password(sender: Sender<Vec<u8>>, character_set: &[char], length: usize, count: Option<usize>) {
    let max = count.unwrap_or(1);
    let range = 0..max;
    let counter = AtomicUsize::new(0);

    range.into_par_iter().for_each(|_| {
        let mut buffer = generate_password(character_set, length);

        if counter.fetch_add(1, Ordering::Relaxed) != max - 1 { buffer.push(LINE_FEED); }

        sender.send(buffer).unwrap();
    });
}

pub fn create_passphrase(sender: Sender<Vec<u8>>, wordlist: &[String], separator: &str, length: usize, count: Option<usize>) {
    let max = count.unwrap_or(1);
    let range = 0..max;
    let counter = AtomicUsize::new(0);

    range.into_par_iter().for_each(|_| {
        let mut buffer = generate_passphrase(wordlist, separator, length);

        if counter.fetch_add(1, Ordering::Relaxed) != max - 1 { buffer.push(LINE_FEED); }

        sender.send(buffer).unwrap();
    });
}

pub fn create_username(sender: Sender<Vec<u8>>, capitalize: bool, kind: UsernameKind,  length: usize, count: Option<usize>) {
    match kind {
        UsernameKind::Simple => {
            let max = count.unwrap_or(1);
            let range = 0..max;
            let counter = AtomicUsize::new(0);

            range.into_par_iter().for_each(|_| {
                let mut buffer = generate_simple_username(capitalize, length);

                if counter.fetch_add(1, Ordering::Relaxed) != max - 1 { buffer.push(LINE_FEED); }

                sender.send(buffer).unwrap();
            });
        }
        UsernameKind::Complex => {
            let max = count.unwrap_or(1);
            let range = 0..max;
            let counter = AtomicUsize::new(0);

            range.into_par_iter().for_each(|_| {
                let mut buffer = generate_complex_username(capitalize, length);

                if counter.fetch_add(1, Ordering::Relaxed) != max - 1 { buffer.push(LINE_FEED); }

                sender.send(buffer).unwrap();
            });
        }
    }
}

pub fn create_digits(sender: Sender<Vec<u8>>, length: usize, count: Option<usize>) {
    let max = count.unwrap_or(1);
    let range = 0..max;
    let counter = AtomicUsize::new(0);

    range.into_par_iter().for_each(|_| {
        let mut buffer = generate_digits(length);

        if counter.fetch_add(1, Ordering::Relaxed) != max - 1 { buffer.push(LINE_FEED); }

        sender.send(buffer).unwrap();
    });
}

pub fn create_number(sender: Sender<Vec<u8>>, minimum: usize, maximum: usize, count: Option<usize>) {
    let max = count.unwrap_or(1);
    let range = 0..max;
    let counter = AtomicUsize::new(0);

    range.into_par_iter().for_each(|_| {
        let mut buffer = generate_number(minimum, maximum);

        if counter.fetch_add(1, Ordering::Relaxed) != max - 1 { buffer.push(LINE_FEED); }

        sender.send(buffer).unwrap();
    });
}
