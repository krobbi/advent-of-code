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
    // many paths exist between the server and the output which pass through
    // both the DAC and the FFT.
    let Some(network) = parse_network(input) else {
        return Solution::ParseError;
    };

    // Because the network is (presumably) acyclic, there should be paths from
    // the DAC to the FFT or paths from the FFT to the DAC, but not both. We may
    // need to check both directions depending on the input.
    let dac_to_fft_path_count = network.count_paths("dac", "fft");
    let fft_to_dac_path_count = network.count_paths("fft", "dac");

    // The paths between the DAC and the FFT form a 'bad section' which should
    // not be passed through between the server and the output. One of these
    // devices is the entry to this section and the other device is the exit.
    let bad_path_count = dac_to_fft_path_count + fft_to_dac_path_count;

    let (entry_device_name, exit_device_name) = if dac_to_fft_path_count != 0 {
        ("dac", "fft")
    } else {
        ("fft", "dac")
    };

    // Now we find the number of paths which can enter and exit this bad
    // section.
    let entry_path_count = network.count_paths("svr", entry_device_name);
    let exit_path_count = network.count_paths(exit_device_name, "out");

    // The server can reach the entry device with some number of paths, which
    // can reach the exit device with some number of paths, which can reach the
    // output with some number of paths.
    (entry_path_count * bad_path_count * exit_path_count).into()
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

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests part one.
    #[test]
    fn part_one_works() {
        static INPUT: &str = "\
            aaa: you hhh\n\
            you: bbb ccc\n\
            bbb: ddd eee\n\
            ccc: ddd eee fff\n\
            ddd: ggg\n\
            eee: out\n\
            fff: out\n\
            ggg: out\n\
            hhh: ccc fff iii\n\
            iii: out\n";

        assert_eq!(part_one(INPUT), 5.into());
    }

    /// Tests part two.
    #[test]
    fn part_two_works() {
        static INPUT: &str = "\
            svr: aaa bbb\n\
            aaa: fft\n\
            fft: ccc\n\
            bbb: tty\n\
            tty: ccc\n\
            ccc: ddd eee\n\
            ddd: hub\n\
            hub: fff\n\
            eee: dac\n\
            dac: fff\n\
            fff: ggg hhh\n\
            ggg: out\n\
            hhh: out\n";

        assert_eq!(part_two(INPUT), 2.into());
    }
}
