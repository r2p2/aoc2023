const std = @import("std");
const expect = std.testing.expect;

const Digit = struct {
    digit: u8,
    len: u8,
};

fn extractDigit(s: []const u8) ?Digit {
    if (std.ascii.isDigit(s[0])) {
        return Digit{ .digit = s[0] - 0x30, .len = 1 };
    } else if (std.mem.startsWith(u8, s, "zero")) {
        return Digit{ .digit = 0, .len = 1 };
    } else if (std.mem.startsWith(u8, s, "one")) {
        return Digit{ .digit = 1, .len = 1 };
    } else if (std.mem.startsWith(u8, s, "two")) {
        return Digit{ .digit = 2, .len = 1 };
    } else if (std.mem.startsWith(u8, s, "three")) {
        return Digit{ .digit = 3, .len = 1 };
    } else if (std.mem.startsWith(u8, s, "four")) {
        return Digit{ .digit = 4, .len = 1 };
    } else if (std.mem.startsWith(u8, s, "five")) {
        return Digit{ .digit = 5, .len = 1 };
    } else if (std.mem.startsWith(u8, s, "six")) {
        return Digit{ .digit = 6, .len = 1 };
    } else if (std.mem.startsWith(u8, s, "seven")) {
        return Digit{ .digit = 7, .len = 1 };
    } else if (std.mem.startsWith(u8, s, "eight")) {
        return Digit{ .digit = 8, .len = 1 };
    } else if (std.mem.startsWith(u8, s, "nine")) {
        return Digit{ .digit = 9, .len = 1 };
    }
    return null;
}

pub fn task1(input: []const u8) !i64 {
    var sum: i64 = 0;
    var line_it = std.mem.split(u8, input, "\n");
    while (line_it.next()) |line| {
        var first_digit_set = false;
        var first_digit: i64 = 0;
        var last_digit_set = false;
        var last_digit: i64 = 0;

        for (line) |c| {
            if (std.ascii.isDigit(c)) {
                if (first_digit_set) {
                    last_digit_set = true;
                    last_digit = c - 0x30;
                } else {
                    first_digit_set = true;
                    first_digit = c - 0x30;
                }
            }
        }
        if (!last_digit_set) {
            last_digit = first_digit;
        }

        //std.debug.print("\nXXX {} = 10 * {} + {}\n", .{ sum, first_digit, last_digit });

        sum += 10 * first_digit + last_digit;
    }
    return sum;
}

pub fn task2(input: []const u8) !i64 {
    var sum: i64 = 0;
    var line_it = std.mem.split(u8, input, "\n");
    while (line_it.next()) |line| {
        var first_digit_set = false;
        var first_digit: i64 = 0;
        var last_digit_set = false;
        var last_digit: i64 = 0;

        var i: u64 = 0;
        while (i < line.len) {
            const maybe_digit = extractDigit(line[i..]);
            if (maybe_digit) |digit| {
                if (first_digit_set) {
                    last_digit_set = true;
                    last_digit = digit.digit;
                } else {
                    first_digit_set = true;
                    first_digit = digit.digit;
                }
                i += digit.len;
            } else {
                i += 1;
            }
        }

        if (!last_digit_set) {
            last_digit = first_digit;
        }

        //std.debug.print("\nXXX {s}\n", .{line});
        //std.debug.print("\nXXX {} = 10 * {} + {}\n", .{ sum, first_digit, last_digit });

        sum += 10 * first_digit + last_digit;
    }

    //std.debug.print("\n\n\n>>{s}\n\n\n", .{input_copy});
    return sum;
}

test "day1 task 1" {
    const input =
        \\1abc2
        \\pqr3stu8vwx
        \\a1b2c3d4e5f
        \\treb7uchet
    ;
    const output = try task1(input);
    try expect(output == 142);
}

test "day1 task 2" {
    const input =
        \\ two1nine
        \\ eightwothree
        \\ abcone2threexyz
        \\ xtwone3four
        \\ 4nineeightseven2
        \\ zoneight234
        \\ 7pqrstsixteen
    ;
    const output = try task2(input);
    try expect(output == 281);
}
