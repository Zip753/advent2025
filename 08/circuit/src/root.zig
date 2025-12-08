//! By convention, root.zig is the root source file when making a library.
const std = @import("std");

pub const Point = struct {
    x: f32,
    y: f32,
    z: f32,
};

const Edge = struct { usize, usize };

const MAX = 1000;

pub fn count_circuits(points: []const Point) !usize {
    if (points.len <= 2) {
        return 1;
    }

    const num_pairs = @divExact(points.len * (points.len - 1), 2);

    const gpa = std.heap.page_allocator;

    var edges: []Edge = try gpa.alloc(Edge, num_pairs);

    var edges_idx: usize = 0;
    for (0..(points.len - 1)) |i| {
        for ((i + 1)..points.len) |j| {
            edges[edges_idx] = .{ i, j };
            edges_idx += 1;
        }
    }

    std.mem.sortUnstable(Edge, edges, points, distance_less_than);

    std.debug.print("{d} -> {d}, {d}\n", .{ edges[0].@"0", edges[0].@"1", distance(points[edges[0].@"0"], points[edges[0].@"1"]) });
    std.debug.print("{d} -> {d}, {d}\n", .{ edges[1].@"0", edges[1].@"1", distance(points[edges[1].@"0"], points[edges[1].@"1"]) });
    std.debug.print("{d} -> {d}, {d}\n", .{ edges[2].@"0", edges[2].@"1", distance(points[edges[2].@"0"], points[edges[2].@"1"]) });
    std.debug.print("{d} -> {d}, {d}\n", .{ edges[3].@"0", edges[3].@"1", distance(points[edges[3].@"0"], points[edges[3].@"1"]) });

    var conn_list: []std.ArrayList(usize) = try gpa.alloc(std.ArrayList(usize), points.len);
    @memset(conn_list, std.ArrayList(usize).empty);

    // std.debug.print("{d}\n", .{conn_list.len});

    for (edges[0..MAX]) |edge| {
        const from, const to = edge;
        // std.debug.print("{d} {d}\n", .{ from, to });
        try conn_list[from].append(gpa, to);
        try conn_list[to].append(gpa, from);
    }

    const visited: []bool = try gpa.alloc(bool, points.len);
    @memset(visited, false);

    var component_sizes: std.ArrayList(usize) = .empty;
    for (0..points.len) |from| {
        const size = bfs(visited, conn_list, from);
        if (size > 0) {
            try component_sizes.append(gpa, size);
        }
    }

    std.mem.sortUnstable(usize, component_sizes.items, {}, std.sort.desc(usize));
    std.debug.print("max size {d}\n", .{component_sizes.items[0]});
    std.debug.print("size {d}\n", .{component_sizes.items.len});

    var total: usize = 1;

    for (component_sizes.items[0..@min(3, component_sizes.items.len)]) |size| {
        total *= size;
    }

    return total;
}

pub fn find_min_connectivity(points: []const Point) !u64 {
    if (points.len <= 2) {
        return 1;
    }

    const num_pairs = @divExact(points.len * (points.len - 1), 2);

    const gpa = std.heap.page_allocator;

    var edges: []Edge = try gpa.alloc(Edge, num_pairs);

    var edges_idx: usize = 0;
    for (0..(points.len - 1)) |i| {
        for ((i + 1)..points.len) |j| {
            edges[edges_idx] = .{ i, j };
            edges_idx += 1;
        }
    }

    std.mem.sortUnstable(Edge, edges, points, distance_less_than);

    std.debug.print("{d} -> {d}, {d}\n", .{ edges[0].@"0", edges[0].@"1", distance(points[edges[0].@"0"], points[edges[0].@"1"]) });
    std.debug.print("{d} -> {d}, {d}\n", .{ edges[1].@"0", edges[1].@"1", distance(points[edges[1].@"0"], points[edges[1].@"1"]) });
    std.debug.print("{d} -> {d}, {d}\n", .{ edges[2].@"0", edges[2].@"1", distance(points[edges[2].@"0"], points[edges[2].@"1"]) });
    std.debug.print("{d} -> {d}, {d}\n", .{ edges[3].@"0", edges[3].@"1", distance(points[edges[3].@"0"], points[edges[3].@"1"]) });

    var conn_list: []std.ArrayList(usize) = try gpa.alloc(std.ArrayList(usize), points.len);

    const visited: []bool = try gpa.alloc(bool, points.len);

    var min_bound: usize = 1;
    var max_bound: usize = edges.len - 1;

    while (max_bound > min_bound + 1) {
        const num_edges = @divTrunc(max_bound + min_bound, 2);
        std.debug.print("trying {d} [{d} - {d})\n", .{ num_edges, min_bound, max_bound });

        @memset(conn_list, std.ArrayList(usize).empty);

        // std.debug.print("{d}\n", .{conn_list.len});

        for (edges[0..num_edges]) |edge| {
            const from, const to = edge;
            // std.debug.print("{d} {d}\n", .{ from, to });
            try conn_list[from].append(gpa, to);
            try conn_list[to].append(gpa, from);
        }

        @memset(visited, false);
        const size = bfs(visited, conn_list, 0);

        if (size < points.len) {
            min_bound = num_edges;
        } else {
            max_bound = num_edges;
        }
    }

    const from = edges[min_bound].@"0";
    const to = edges[min_bound].@"1";

    std.debug.print("{d} -> {d}, {d} * {d}\n", .{ edges[min_bound].@"0", edges[min_bound].@"1", points[from].x, points[to].x });

    const x1: u64 = @intFromFloat(std.math.round(points[from].x));
    const x2: u64 = @intFromFloat(std.math.round(points[to].x));
    const result = x1 * x2;
    std.debug.print("result {d}", .{result});
    return result;
}

fn bfs(visited: []bool, conn_list: []const std.ArrayList(usize), from: usize) usize {
    if (visited[from]) {
        // std.debug.print("quick un {d}\n", .{from});
        return 0;
    }

    var total: usize = 1;
    visited[from] = true;
    // std.debug.print("visited for real bruh {d}\n", .{from});

    for (conn_list[from].items) |to| {
        // std.debug.print("going to {d}\n", .{to});
        total += bfs(visited, conn_list, to);
    }

    // std.debug.print("done with this un: {d} {d}\n", .{ from, total });
    return total;
}

fn distance_less_than(points: []const Point, a: Edge, b: Edge) bool {
    return distance(points[a.@"0"], points[a.@"1"]) < distance(points[b.@"0"], points[b.@"1"]);
}

fn distance(a: Point, b: Point) f32 {
    return @sqrt(sqr(a.x - b.x) + sqr(a.y - b.y) + sqr(a.z - b.z));
}

inline fn sqr(x: anytype) @TypeOf(x) {
    return x * x;
}
