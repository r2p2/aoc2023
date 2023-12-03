const std = @import("std");
const expect = std.testing.expect;

const Number = struct {
    value: i64,
    valid: bool,
};
const Position = struct {
    number: *Number,
};

fn width(n: i64) u8 {
    var number = n;
    var w: u8 = 0;
    while (number > 0) {
        w += 1;
        number = @divTrunc(number, 10);
    }
    return w;
}

fn isChar(c: u8) bool {
    if (std.ascii.isDigit(c)) {
        return false;
    }

    if (c == '.') {
        return false;
    }

    if (c == '\n') {
        return false;
    }

    return true;
}

fn clear(a: []u8, i: u64, w: u64) void {
    if (std.ascii.isDigit(a[i])) {
        a[i] = '.';
    } else {
        return;
    }

    var l = i;
    while ((l % w) != 0) {
        l -= 1;
        if (!std.ascii.isDigit(a[l])) {
            break;
        }
        a[l] = '.';
    }
    var r = i;
    while ((r % w) != (w - 2)) {
        r += 1;
        if (!std.ascii.isDigit(a[r])) {
            break;
        }
        a[r] = '.';
    }
}

pub fn task1(allocator: std.mem.Allocator, input: []const u8) !u64 {
    const input_copy = try allocator.alloc(u8, input.len);
    defer allocator.free(input_copy);
    std.mem.copy(u8, input_copy, input);

    const w: u64 = std.mem.indexOf(u8, input, "\n").? + 1;
    std.debug.print("{}", .{w});

    for (input_copy, 0..) |c, i| {
        if (!isChar(c)) {
            continue;
        }

        const up = i < w;
        const lhs = (i % w) == 0;
        const rhs = (i % w) == (w - 2);
        const dn = (i + w) >= input.len;
        if (!up) {
            if (!lhs) {
                clear(input_copy, i - w - 1, w);
            }
            clear(input_copy, i - w, w);
            if (!rhs) {
                clear(input_copy, i - w + 1, w);
            }
        }

        if (!lhs) {
            clear(input_copy, i - 1, w);
        }

        if (!rhs) {
            clear(input_copy, i + 1, w);
        }

        if (!dn) {
            if (!lhs) {
                clear(input_copy, i + w - 1, w);
            }
            clear(input_copy, i + w, w);
            if (!rhs) {
                clear(input_copy, i + w + 1, w);
            }
        }
    }

    var wrong_sum: u64 = 0;
    {
        var num_it = std.mem.tokenizeAny(u8, input_copy, ".\n#+*$@=/%-&");
        while (num_it.next()) |num| {
            //std.debug.print("\n{s}\n", .{num});
            wrong_sum += try std.fmt.parseInt(u64, num, 10);
        }
    }

    var all_sum: u64 = 0;
    {
        var num_it = std.mem.tokenizeAny(u8, input, ".\n#+*$@=/%-&");
        while (num_it.next()) |num| {
            all_sum += try std.fmt.parseInt(u64, num, 10);
        }
    }

    std.debug.print("\n{s}\n", .{input_copy});
    return (all_sum - wrong_sum);
}

pub fn task2(input: []const u8) i64 {
    _ = input;
}

test "width" {
    try expect(width(0) == 0);
    try expect(width(1) == 1);
    try expect(width(3) == 1);
    try expect(width(12) == 2);
    try expect(width(442) == 3);
}

test "day 1 task 1" {
    const allocator = std.testing.allocator;
    const input =
        \\467..114..
        \\...*......
        \\..35..633.
        \\......#...
        \\617*......
        \\.....+.58.
        \\..592.....
        \\......755.
        \\...$.*....
        \\.664.598..
    ;
    const output = try task1(allocator, input);
    try expect(output == 4361);
}

//test "day 1 task 2" {
//    const input =
//        \\ two1nine
//        \\ eightwothree
//        \\ abcone2threexyz
//        \\ xtwone3four
//        \\ 4nineeightseven2
//        \\ zoneight234
//        \\ 7pqrstsixteen
//    ;
//    const output = task2(input);
//    try expect(output == 281);
//}
