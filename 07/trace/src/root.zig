//! By convention, root.zig is the root source file when making a library.
const std = @import("std");

pub fn trace(reader: *std.Io.Reader) !u32 {
    // Read first line from file
    const first_line = try reader.takeDelimiterExclusive('\n');
    reader.toss(1);

    // Allocate two buffers (prev and next) of booleans for storing state of the size of the input.
    // Initialize with false.
    const allocator = std.heap.page_allocator;
    var prev = try allocator.alloc(bool, first_line.len);
    var next = try allocator.alloc(bool, first_line.len);

    // Process the first line, set the position where we read S as true.
    @memset(prev, false);
    for (first_line, 0..) |c, i| {
        if (c == 'S') {
            prev[i] = true;
        }
    }

    // Read second line and skip it (ideally would just skip through it, but this is improvement for later).
    _ = try reader.takeDelimiterInclusive('\n');

    // Set up global counter to 0.
    var total_splits: u32 = 0;

    // Then keep reading and doing processing.
    // Skip every second line.
    var even = false;
    while (reader.takeDelimiterInclusive('\n')) |line| : (even = !even) {
        if (even) {
            continue;
        }

        // Initialise next to false
        @memset(next, false);

        for (line[0 .. line.len - 1], 0..) |c, i| {
            if (c == '^' and prev[i]) {
                total_splits += 1;

                if (i >= 1) {
                    next[i - 1] = true;
                }

                if (i + 1 < next.len) {
                    next[i + 1] = true;
                }
            } else if (prev[i]) {
                next[i] = true;
            }
        }

        // Swap pointers
        std.mem.swap([]bool, &prev, &next);
    } else |err| switch (err) {
        error.EndOfStream => {},
        error.ReadFailed, error.StreamTooLong => return err,
    }

    return total_splits;
}

pub fn traceAll(reader: *std.Io.Reader) !u64 {
    // Read first line from file
    const first_line = try reader.takeDelimiterExclusive('\n');
    reader.toss(1);

    // Allocate two buffers (prev and next) of booleans for storing state of the size of the input.
    // Initialize with false.
    const allocator = std.heap.page_allocator;
    var prev = try allocator.alloc(u64, first_line.len);
    var next = try allocator.alloc(u64, first_line.len);

    // Process the first line, set the position where we read S as true.
    @memset(prev, 0);
    for (first_line, 0..) |c, i| {
        if (c == 'S') {
            prev[i] = 1;
        }
    }

    // Read second line and skip it (ideally would just skip through it, but this is improvement for later).
    _ = try reader.takeDelimiterInclusive('\n');

    // Then keep reading and doing processing.
    // Skip every second line.
    var even = false;
    while (reader.takeDelimiterInclusive('\n')) |line| : (even = !even) {
        if (even) {
            continue;
        }

        // Initialise next to false
        @memset(next, 0);

        for (line[0 .. line.len - 1], 0..) |c, i| {
            if (c == '^' and prev[i] != 0) {
                if (i >= 1) {
                    next[i - 1] += prev[i];
                }

                if (i + 1 < next.len) {
                    next[i + 1] += prev[i];
                }
            } else if (prev[i] != 0) {
                next[i] += prev[i];
            }
        }

        // Swap pointers
        std.mem.swap([]u64, &prev, &next);
    } else |err| switch (err) {
        error.EndOfStream => {},
        error.ReadFailed, error.StreamTooLong => return err,
    }

    var total: u64 = 0;

    for (prev) |count| {
        total += count;
    }

    return total;
}

test "trace default" {
    const input =
        \\.......S.......
        \\...............
        \\.......^.......
        \\...............
        \\......^.^......
        \\...............
        \\.....^.^.^.....
        \\...............
        \\....^.^...^....
        \\...............
        \\...^.^...^.^...
        \\...............
        \\..^...^.....^..
        \\...............
        \\.^.^.^.^.^...^.
        \\...............
        \\
    ;

    // create a fixed reader
    var reader: std.Io.Reader = .fixed(input);

    // call trace
    const result = try trace(&reader);

    // expect trace to equal smth
    try std.testing.expectEqual(result, 21);
}

test "trace skip" {
    const input =
        \\.......S.......
        \\...............
        \\.......^.......
        \\...............
        \\......^........
        \\...............
        \\......^.^......
        \\...............
        \\
    ;

    // create a fixed reader
    var reader: std.Io.Reader = .fixed(input);

    // call trace
    const result = try trace(&reader);

    // expect trace to equal smth
    try std.testing.expectEqual(result, 3);
}

test "traceAll default" {
    const input =
        \\.......S.......
        \\...............
        \\.......^.......
        \\...............
        \\......^.^......
        \\...............
        \\.....^.^.^.....
        \\...............
        \\....^.^...^....
        \\...............
        \\...^.^...^.^...
        \\...............
        \\..^...^.....^..
        \\...............
        \\.^.^.^.^.^...^.
        \\...............
        \\
    ;

    // create a fixed reader
    var reader: std.Io.Reader = .fixed(input);

    // call trace
    const result = try traceAll(&reader);

    // expect trace to equal smth
    try std.testing.expectEqual(40, result);
}

test "traceAll skip" {
    const input =
        \\.......S.......
        \\...............
        \\.......^.......
        \\...............
        \\......^........
        \\...............
        \\......^.^......
        \\...............
        \\
    ;

    // create a fixed reader
    var reader: std.Io.Reader = .fixed(input);

    // call trace
    const result = try traceAll(&reader);

    // expect trace to equal smth
    try std.testing.expectEqual(4, result);
}
