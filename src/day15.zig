const std = @import("std");
const expect = std.testing.expect;

fn hash(s: []const u8) u8 {
    var h: i64 = 0;
    for (s) |c| {
        h += c;
        h *= 17;
        h = @mod(h, 256);
    }

    return @intCast(h);
}

pub fn task1(input: []const u8) i64 {
    var sum: i64 = 0;

    var it = std.mem.splitAny(u8, input, ",\n");
    while (it.next()) |s| {
        if (s.len > 0) {
            sum += hash(s);
        }
    }

    return sum;
}

//pub fn task2(input: []const u8) i64 {
//    var sum: i64 = 0;
//    var line_it = std.mem.split(u8, input, "\n");
//    while (line_it.next()) |line| {
//        const first_digit: u8 = extractFirstDigit(line) orelse 0;
//        const last_digit: u8 = extractLastDigit(line) orelse 0;
//        sum += 10 * first_digit + last_digit;
//    }
//    return sum;
//}

test "hash" {
    try expect(hash("HASH") == 52);
}

test "day 15 task 1" {
    const input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7\n";
    try expect(task1(input) == 1320);
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
