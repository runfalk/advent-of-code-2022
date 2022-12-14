use anyhow::{anyhow, Result};
use chumsky::prelude::*;
use std::cmp::Ordering;
use std::fs::File;
use std::io::Read;
use std::iter;
use std::path::Path;

#[derive(Debug)]
enum Packet {
    Int(usize),
    List(Vec<Self>),
}

fn parser() -> impl Parser<char, Packet, Error = Simple<char>> {
    recursive(|p| {
        p.separated_by(just(','))
            .delimited_by(just('['), just(']'))
            .map(Packet::List)
            .or(text::int(10).from_str().unwrapped().map(Packet::Int))
    })
}

fn is_in_order(left: &Vec<Packet>, right: &Vec<Packet>) -> Ordering {
    for pair in left.iter().zip(right) {
        match pair {
            (Packet::Int(l), Packet::Int(r)) => match l.cmp(r) {
                Ordering::Equal => {}
                order => return order,
            },
            (Packet::List(l), Packet::List(r)) => {
                let order = is_in_order(l, r);
                if order.is_ne() {
                    return order;
                }
            }
            (Packet::List(l), Packet::Int(r)) => {
                let order = is_in_order(l, &vec![Packet::Int(*r)]);
                if order.is_ne() {
                    return order;
                }
            }
            (Packet::Int(l), Packet::List(r)) => {
                let order = is_in_order(&vec![Packet::Int(*l)], r);
                if order.is_ne() {
                    return order;
                }
            }
        }
    }
    left.len().cmp(&right.len())
}

fn part_a(pairs: &[(Vec<Packet>, Vec<Packet>)]) -> usize {
    let mut sum = 0;
    for (i, (left, right)) in pairs.iter().enumerate() {
        if is_in_order(left, right) == Ordering::Less {
            sum += i + 1;
        }
    }
    sum
}

fn part_b(pairs: &[(Vec<Packet>, Vec<Packet>)]) -> usize {
    let divider_1 = vec![Packet::List(vec![Packet::Int(2)])];
    let divider_2 = vec![Packet::List(vec![Packet::Int(6)])];
    let mut packets = pairs
        .iter()
        .flat_map(|(l, r)| iter::once(l).chain(iter::once(r)))
        .collect::<Vec<_>>();
    packets.push(&divider_1);
    packets.push(&divider_2);

    packets.sort_by(|a, b| is_in_order(a, b));

    let divider_1_idx = packets
        .iter()
        .position(|p| is_in_order(p, &divider_1).is_eq());
    let divider_2_idx = packets
        .iter()
        .position(|p| is_in_order(p, &divider_2).is_eq());

    // Unwrap is safe because we know dividers are in the list
    (divider_1_idx.unwrap() + 1) * (divider_2_idx.unwrap() + 1)
}

pub fn main(path: &Path) -> Result<(usize, Option<usize>)> {
    let mut input = String::new();
    File::open(path)?.read_to_string(&mut input)?;

    let mut pairs = Vec::new();
    let packet_parser = parser();
    for pair in input.split("\n\n").map(|pair_str| {
        pair_str
            .split_once('\n')
            .ok_or_else(|| anyhow!("Pair must have a single line break"))
    }) {
        let (left, right) = pair?;
        let Packet::List(left) = packet_parser.parse(left).unwrap() else { panic!(); };
        let Packet::List(right) = packet_parser.parse(right).unwrap() else { panic!(); };
        pairs.push((left, right));
    }
    Ok((part_a(&pairs), Some(part_b(&pairs))))
}
