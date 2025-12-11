//! [Day 11: Reactor][link]
//!
//! [link]: https://adventofcode.com/2025/day/11

use std::collections::HashMap;

use crate::Solution;

/// Solves part one.
pub fn part_one(input: &str) -> Solution {
    // The Elves are having some trouble with the reactor. They have installed a
    // new server rack, but it can't communicate with the reactor. The server
    // connects to the reactor through a network of devices connected with data
    // cables. The data cables are one-way, the data cannot flow backwards. One
    // specific path between the device we are looking at (labelled "you") and
    // the main output to the reactor (labelled "out") is causing an issue. The
    // Elves want to know how many paths exist between this device and the main
    // output.

    // This sounds like traversing a directed (hopefully acyclic) graph.
    let Some(_network) = parse_network(input) else {
        return Solution::ParseError;
    };

    Solution::default()
}

/// Solves part two.
pub fn part_two(input: &str) -> Solution {
    let _ = input;
    Solution::default()
}

/// A data network.
#[derive(Default)]
struct Network {
    /// The map of device names to `Device`s.
    devices: HashMap<String, Device>,
}

/// A device in a [`Network`].
#[derive(Default)]
struct Device {
    /// The connections to other device names from the `Device`.
    connections: Vec<String>,
}

/// Parses a [`Network`] from input. This function returns [`None`] if a
/// [`Network`] could not be parsed.
fn parse_network(input: &str) -> Option<Network> {
    let mut network = Network::default();

    for line in input.lines().map(str::trim) {
        let (device_name, device) = parse_device(line)?;
        network.devices.insert(device_name, device);
    }

    Some(network)
}

/// Parses a device name and [`Device`] from a line of input. This function
/// returns [`None`] if a [`Device`] could not be parsed.
fn parse_device(line: &str) -> Option<(String, Device)> {
    let mut device_names = line.split_whitespace();
    let device_name = device_names.next()?.trim_end_matches(':');

    if device_name.is_empty() {
        return None;
    }

    let mut device = Device::default();

    for connected_device_name in device_names {
        device.connections.push(connected_device_name.into());
    }

    Some((device_name.into(), device))
}

/*
#[cfg(test)]
mod tests {
    use super::*;

    /// Tests part one.
    #[test]
    fn part_one_works() {}

    /// Tests part two.
    #[test]
    fn part_two_works() {}
}
*/
