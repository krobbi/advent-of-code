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
    let Some(network) = parse_network(input) else {
        return Solution::ParseError;
    };

    network.count_paths("you", "out").into()
}

/// Solves part two.
pub fn part_two(input: &str) -> Solution {
    // The Elves have found the source of the problem: the server rack ("svr")
    // connects to the output ("out") through both a digital-to-analog converter
    // ("dac") and fast fourier transformer ("fft"). The Elves want to know how
    // many paths exist between the server and the output that pass through the
    // DAC and FFT.
    let Some(network) = parse_network(input) else {
        return Solution::ParseError;
    };

    // Because the network is (presumably) acyclic, either the DAC or FFT should
    // appear first, but not the other way around.
    let fft_to_dac = network.count_paths("fft", "dac");
    let dac_to_fft = network.count_paths("dac", "fft");
    let middle_path_count = fft_to_dac + dac_to_fft;

    let (first, second) = if fft_to_dac == 0 {
        ("dac", "fft")
    } else {
        ("fft", "dac")
    };

    let beginning_path_count = network.count_paths("svr", first);
    let end_path_count = network.count_paths(second, "out");
    (beginning_path_count * middle_path_count * end_path_count).into()
}

/// Returns the number of paths from a source device name to a target device
/// name in a [`Network`] to the "out" device with a memo.
fn count_paths_with_memo(
    network: &Network,
    source_device_name: &str,
    target_device_name: &str,
    memo: &mut HashMap<String, u64>,
) -> u64 {
    if source_device_name == target_device_name {
        return 1;
    } else if let Some(path_count) = memo.get(source_device_name).copied() {
        return path_count;
    }

    let mut path_count = 0;

    for connected_device_name in network.connections_from(source_device_name) {
        path_count +=
            count_paths_with_memo(network, connected_device_name, target_device_name, memo);
    }

    memo.insert(source_device_name.into(), path_count);
    path_count
}

/// A data network.
#[derive(Default)]
struct Network {
    /// The map of device names to [`Device`]s.
    devices: HashMap<String, Device>,
}

impl Network {
    /// Returns a slice of device names connected from another device name.
    fn connections_from(&self, device_name: &str) -> &[String] {
        match self.devices.get(device_name) {
            None => &[],
            Some(device) => &device.connections,
        }
    }

    /// Returns the number of paths from a source device name to a target device
    /// name.
    fn count_paths(&self, source_device_name: &str, target_device_name: &str) -> u64 {
        let mut memo = HashMap::new();
        count_paths_with_memo(self, source_device_name, target_device_name, &mut memo)
    }
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
