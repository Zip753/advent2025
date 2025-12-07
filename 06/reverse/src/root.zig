//! By convention, root.zig is the root source file when making a library.
const std = @import("std");

pub fn transpose(writer: *std.Io.Writer, lines: []const []const u8) !void {
    for (0..lines[0].len) |col| {
        for (0..lines.len) |row| {
            try writer.print("{c}", .{lines[row][col]});
        }
        try writer.print("\n", .{});
        try writer.flush();
    }
}

test "transposes the text" {
    const lines = [_][]const u8{
        "91 ",
        "736",
        "347",
        "583",
    };

    var buf: [1000]u8 = undefined;
    var sliceWriter = std.Io.Writer.fixed(&buf);

    try transpose(&sliceWriter, &lines);

    try std.testing.expectEqualStrings("9735\n1348\n 673\n", sliceWriter.buffered());
}
