const std = @import("std");
const expect = std.testing.expect;

fn extractFirstDigit(s: []const u8) ?u8 {
    var i: u64 = 0;
    while (i < s.len) : (i += 1) {
        if (extractDigit(s[i..])) |digit| {
            return digit;
        }
    }
    return null;
}

fn extractLastDigit(s: []const u8) ?u8 {
    var i: u64 = 0;
    while (i < s.len) : (i += 1) {
        if (extractDigit(s[s.len - i - 1 ..])) |digit| {
            return digit;
        }
    }
    return null;
}

fn extractDigit(s: []const u8) ?u8 {
    if (std.ascii.isDigit(s[0])) {
        return s[0] - 0x30;
    } else if (std.mem.startsWith(u8, s, "zero")) {
        return 0;
    } else if (std.mem.startsWith(u8, s, "one")) {
        return 1;
    } else if (std.mem.startsWith(u8, s, "two")) {
        return 2;
    } else if (std.mem.startsWith(u8, s, "three")) {
        return 3;
    } else if (std.mem.startsWith(u8, s, "four")) {
        return 4;
    } else if (std.mem.startsWith(u8, s, "five")) {
        return 5;
    } else if (std.mem.startsWith(u8, s, "six")) {
        return 6;
    } else if (std.mem.startsWith(u8, s, "seven")) {
        return 7;
    } else if (std.mem.startsWith(u8, s, "eight")) {
        return 8;
    } else if (std.mem.startsWith(u8, s, "nine")) {
        return 9;
    }
    return null;
}

pub fn task1(input: []const u8) i64 {
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
        sum += 10 * first_digit + last_digit;
    }
    return sum;
}

pub fn task2(input: []const u8) i64 {
    var sum: i64 = 0;
    var line_it = std.mem.split(u8, input, "\n");
    while (line_it.next()) |line| {
        const first_digit: u8 = extractFirstDigit(line) orelse 0;
        const last_digit: u8 = extractLastDigit(line) orelse 0;
        sum += 10 * first_digit + last_digit;
    }
    return sum;
}

test "day 1 task 1" {
    const input =
        \\1abc2
        \\pqr3stu8vwx
        \\a1b2c3d4e5f
        \\treb7uchet
    ;
    const output = task1(input);
    try expect(output == 142);
}

test "day 1 task 2" {
    const input =
        \\ two1nine
        \\ eightwothree
        \\ abcone2threexyz
        \\ xtwone3four
        \\ 4nineeightseven2
        \\ zoneight234
        \\ 7pqrstsixteen
    ;
    const output = task2(input);
    try expect(output == 281);
}
