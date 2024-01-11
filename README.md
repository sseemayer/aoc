# Advent of Code, in Rust

My Rust solutions for several years of [AoC](https://adventofcode.com).

## Automation Guideline Compliance

This repository complies with the [Advent of Code automation guidelines](https://old.reddit.com/r/adventofcode/wiki/faqs/automation). Specifically:

  * Outbound calls are throttled to every 5 minutes - see [aoc::config::Config](aoc/src/config.rs)
  * Once inputs are downloaded, they are cached locally - see [aoc::input](aoc/src/input.rs)
  * The `User-Agent` header is set to an appropriate value - see [aoc::input](aoc/src/input.rs)
