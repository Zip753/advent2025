const std = @import("std");
const circuit = @import("circuit");

pub fn main() !void {
    const gpa = std.heap.page_allocator;
    var file = try std.fs.cwd().openFile("../input.txt", .{ .mode = .read_only });

    var buf: [256]u8 = undefined;
    var input_reader = file.reader(&buf);
    var reader = &input_reader.interface;

    var points: std.ArrayList(circuit.Point) = .empty;

    while (reader.takeDelimiterInclusive('\n')) |line| {
        const pos1 = std.mem.indexOfScalar(u8, line, ',') orelse return;
        const num1 = try std.fmt.parseFloat(f32, line[0..pos1]);

        var offset = pos1 + 1;
        std.debug.print("line: {s}", .{line});
        std.debug.print("remainder: {s}", .{line[offset..]});
        const pos2 = std.mem.indexOfScalar(u8, line[offset..], ',') orelse return;
        std.debug.print("pos2: {d}\n", .{pos2});
        const num2 = try std.fmt.parseFloat(f32, line[offset..(offset + pos2)]);

        offset += pos2 + 1;
        const num3 = try std.fmt.parseFloat(f32, line[offset..(line.len - 1)]);

        try points.append(gpa, .{ .x = num1, .y = num2, .z = num3 });
    } else |err| switch (err) {
        error.EndOfStream => {},
        error.ReadFailed, error.StreamTooLong => return err,
    }

    const result = try circuit.find_min_connectivity(points.items);
    // print answer
    std.debug.print("{d}\n", .{result});
}
