const std = @import("std");
const expect = std.testing.expect;

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

pub fn task2(allocator: std.mem.Allocator, input: []const u8) !i64 {
    const input_copy = try allocator.alloc(u8, input.len);
    defer allocator.free(input_copy);
    std.mem.copy(u8, input_copy, input);

    var i: u64 = 0;
    while (i < input_copy.len) : (i += 1) {
        const substr = input_copy[i..];
        //std.debug.print("\n\n\n>>>>>{s}\n\n\n", .{substr});
        if (std.mem.eql(u8, substr, "zero")) {
            input_copy[i] = 0x30;
        } else if (std.mem.indexOf(u8, substr, "one") == 0) {
            input_copy[i] = 0x31;
        } else if (std.mem.indexOf(u8, substr, "two") == 0) {
            input_copy[i] = 0x32;
        } else if (std.mem.indexOf(u8, substr, "three") == 0) {
            input_copy[i] = 0x33;
        } else if (std.mem.indexOf(u8, substr, "four") == 0) {
            input_copy[i] = 0x34;
        } else if (std.mem.indexOf(u8, substr, "five") == 0) {
            input_copy[i] = 0x35;
        } else if (std.mem.indexOf(u8, substr, "six") == 0) {
            input_copy[i] = 0x36;
        } else if (std.mem.indexOf(u8, substr, "seven") == 0) {
            input_copy[i] = 0x37;
        } else if (std.mem.indexOf(u8, substr, "eight") == 0) {
            input_copy[i] = 0x38;
        } else if (std.mem.indexOf(u8, substr, "nine") == 0) {
            input_copy[i] = 0x39;
        }
    }
    //std.debug.print("\n\n\n>>{s}\n\n\n", .{input_copy});
    return task1(input_copy);
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
    const allocator = std.testing.allocator;
    const input =
        \\ two1nine
        \\ eightwothree
        \\ abcone2threexyz
        \\ xtwone3four
        \\ 4nineeightseven2
        \\ zoneight234
        \\ 7pqrstsixteen
    ;
    const output = try task2(allocator, input);
    try expect(output == 281);
}
