const std = @import("std");
const trace = @import("trace");

pub fn main() !void {
    // Create file handler
    var file = try std.fs.cwd().openFile("../input.txt", .{ .mode = .read_only });
    defer file.close();

    var buf: [1024]u8 = undefined;
    var file_reader = file.reader(&buf);
    const reader = &file_reader.interface;

    // const total_splits = try trace.trace(reader);
    const total_splits = try trace.traceAll(reader);

    // Output the counter.
    std.debug.print("{d}\n", .{total_splits});
}
