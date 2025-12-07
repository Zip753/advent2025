const std = @import("std");
const reverse = @import("reverse");

pub fn main() !void {
    const stdin = std.fs.File.stdin();
    const stdout = std.fs.File.stdout();

    var ioBuf: [1000]u8 = undefined;

    // var stdinReader = stdin.reader(&ioBuf);
    // while (stdinReader.interface.takeDelimiterExclusive('\n')) |input| {
    //     std.debug.print("{s}", .{input});
    // } else |err| switch (err) {
    //     error.EndOfStream => {},
    //     error.ReadFailed, error.StreamTooLong => unreachable,
    // }

    const bytesRead = try stdin.readAll(&ioBuf);
    var linesIter = std.mem.splitScalar(u8, ioBuf[0..bytesRead], '\n');

    var lines: std.ArrayList([]const u8) = .empty;
    defer lines.deinit(std.heap.page_allocator);

    while (linesIter.next()) |line| {
        if (line.len == 0) {
            continue;
        }
        try lines.append(std.heap.page_allocator, line);
    }

    var outBuf: [1000]u8 = undefined;
    var stdoutWriter = stdout.writer(&outBuf);
    try reverse.transpose(&stdoutWriter.interface, lines.items);
}
