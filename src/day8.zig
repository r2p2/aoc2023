const std = @import("std");
const expect = std.testing.expect;

const Node = struct {
    name: []const u8,
    left: []const u8,
    right: []const u8,
};

const Node2 = struct {
    lhs: []const u8,
    rhs: []const u8,
};

fn endReached(positions: *std.ArrayList(*[]const u8)) bool {
    var i: usize = 0;
    while (i < positions.items.len) : (i += 1) {
        if (positions.items[i].*[positions.items[i].len - 1] != 'Z') {
            return false;
        }
    }
    return true;
}

pub fn task1(allocator: std.mem.Allocator, input: []const u8) !u64 {
    var nodes = std.ArrayList(Node).init(allocator);
    defer nodes.deinit();

    var line_it = std.mem.split(u8, input, "\n");
    var steps = line_it.next().?;
    while (line_it.next()) |line| {
        if (line.len == 0) {
            continue;
        }
        var token_it = std.mem.tokenizeAny(u8, line, " =(),");
        try nodes.append(Node{ .name = token_it.next().?, .left = token_it.next().?, .right = token_it.next().? });
    }

    var step_cnt: u64 = 0;
    var pos: []const u8 = "AAA";
    while (!std.mem.eql(u8, pos, "ZZZ")) : ({
        step_cnt += 1;
    }) {
        var i: usize = 0;
        while (i < nodes.items.len) : (i += 1) {
            const node = &nodes.items[i];
            if (!std.mem.eql(u8, node.name, pos)) {
                continue;
            }

            if (steps[step_cnt % steps.len] == 'L') {
                pos = node.left;
            } else {
                pos = node.right;
            }
            break;
        }
    }
    return step_cnt;
}

pub fn task2(allocator: std.mem.Allocator, input: []const u8) !u64 {
    var node_names = std.ArrayList([]const u8).init(allocator);
    defer node_names.deinit();
    var nodes = std.StringHashMap(Node2).init(allocator);
    defer nodes.deinit();

    var line_it = std.mem.split(u8, input, "\n");
    var steps = line_it.next().?;
    while (line_it.next()) |line| {
        if (line.len == 0) {
            continue;
        }
        var token_it = std.mem.tokenizeAny(u8, line, " =(),");
        const name = token_it.next().?;
        const lhs = token_it.next().?;
        const rhs = token_it.next().?;
        try nodes.put(name, Node2{ .lhs = lhs, .rhs = rhs });
        try node_names.append(name);
    }

    var positions = std.ArrayList(*[]const u8).init(allocator);
    defer positions.deinit();
    var step_results = std.ArrayList(u64).init(allocator);
    defer step_results.deinit();
    { // find starting positions
        var i: usize = 0;
        while (i < node_names.items.len) : (i += 1) {
            const name = &node_names.items[i];
            if (name.*[name.len - 1] == 'A') {
                try positions.append(name);
                try step_results.append(0);
            }
        }
    }

    var pos_i: usize = 0;
    while (pos_i < positions.items.len) : (pos_i += 1) {
        var pos = &positions.items[pos_i];

        var step_cnt: usize = 0;
        while (pos.*.*[2] != 'Z') : (step_cnt += 1) {
            const node = nodes.getPtr(pos.*.*).?;

            if (steps[step_cnt % steps.len] == 'L') {
                pos.* = &node.lhs;
            } else {
                pos.* = &node.rhs;
            }
        }
        step_results.items[pos_i] = step_cnt;
    }

    //{ // debug
    //    var i: usize = 0;
    //    while (i < step_results.items.len) : (i += 1) {
    //        std.debug.print(">>{}\n", .{step_results.items[i]});
    //    }
    //}

    {
        var i: usize = 1;
        while (i < step_results.items.len) : (i += 1) {
            const a = step_results.items[i - 1];
            const b = step_results.items[i];

            const lcm = a * b / std.math.gcd(a, b);
            step_results.items[i] = lcm;
        }
    }

    return step_results.getLast();
}

test "day 8 task 1" {
    const input =
        \\LLR
        \\
        \\AAA = (BBB, BBB)
        \\BBB = (AAA, ZZZ)
        \\ZZZ = (ZZZ, ZZZ)
    ;
    try expect(try task1(std.testing.allocator, input) == 6);
}

test "day 8 task 2" {
    const input =
        \\LR
        \\
        \\11A = (11B, XXX)
        \\11B = (XXX, 11Z)
        \\11Z = (11B, XXX)
        \\22A = (22B, XXX)
        \\22B = (22C, 22C)
        \\22C = (22Z, 22Z)
        \\22Z = (22B, 22B)
        \\XXX = (XXX, XXX)
    ;
    const output = try task2(std.testing.allocator, input);
    try expect(output == 6);
}
