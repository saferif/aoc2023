#![no_std]

mod allocation;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
mod error;
mod utils;

extern crate alloc;

use crate::error::AoCError;
use alloc::boxed::Box;
use alloc::format;
use alloc::string::String;

static SOLUTIONS: &[fn(String) -> Result<String, AoCError>] = &[
    day01::part1,
    day01::part2,
    day02::part1,
    day02::part2,
    day03::part1,
    day03::part2,
    day04::part1,
    day04::part2,
    day05::part1,
    day05::part2,
    day06::part1,
    day06::part2,
    day07::part1,
    day07::part2,
    day08::part1,
    day08::part2,
    day09::part1,
    day09::part2,
    day10::part1,
    day10::part2,
    day11::part1,
    day11::part2,
    day12::part1,
    day12::part2,
    day13::part1,
    day13::part2,
    day14::part1,
    day14::part2,
    day15::part1,
    day15::part2,
    day16::part1,
    day16::part2,
    day17::part1,
    day17::part2,
    day18::part1,
    day18::part2,
    day19::part1,
    day19::part2,
    day20::part1,
    day20::part2,
    day21::part1,
    day21::part2,
    day22::part1,
    day22::part2,
    day23::part1,
    day23::part2,
    day24::part1,
    day24::part2,
    day25::part1,
    day25::part2,
];

#[repr(C, packed)]
struct JSString {
    data: *mut u8,
    len: usize,
}

#[no_mangle]
extern "C" fn solve(index: usize, str: *mut JSString) -> bool {
    let input = unsafe { String::from_raw_parts((*str).data, (*str).len, (*str).len) };
    let (ok, output) = SOLUTIONS
        .get(index)
        .map_or(Err(format!("Invalid index {}", index).into()), |f| f(input))
        .map_or_else(|e| (false, String::from(e)), |r| (true, r));
    unsafe {
        (*str).len = output.len();
        (*str).data = Box::leak(output.into_boxed_str()).as_mut_ptr();
    }
    ok
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    core::arch::wasm32::unreachable()
}
