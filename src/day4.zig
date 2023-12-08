const std = @import("std");
const expect = std.testing.expect;

pub fn task1(input: []const u8) u64 {
    var sum: u64 = 0;
    var line_it = std.mem.split(u8, input, "\n");
    while (line_it.next()) |line| {
        if (line.len < 1) {
            continue;
        }
        var card_it = std.mem.split(u8, line, ":");
        const card = card_it.next().?;
        _ = card;
        const numbers = card_it.next().?;
        var numbers_it = std.mem.split(u8, numbers, "|");
        const winners = numbers_it.next().?;
        const yours = numbers_it.next().?;
        var winners_it = std.mem.tokenize(u8, winners, " ");
        var yours_it = std.mem.tokenize(u8, yours, " ");

        var card_value: u64 = 0;
        while (yours_it.next()) |your_number| {
            var this_winners_it = winners_it;
            while (this_winners_it.next()) |this_winner| {
                if (std.mem.eql(u8, your_number, this_winner)) {
                    if (card_value == 0) {
                        card_value = 1;
                    } else {
                        card_value *= 2;
                    }
                }
            }
        }
        sum += card_value;
    }
    return sum;
}

pub fn task2(input: []const u8) i64 {
    _ = input;
    return 0;
    //    var sum: i64 = 0;
    //    var line_it = std.mem.split(u8, input, "\n");
    //    while (line_it.next()) |line| {
    //        const first_digit: u8 = extractFirstDigit(line) orelse 0;
    //        const last_digit: u8 = extractLastDigit(line) orelse 0;
    //        sum += 10 * first_digit + last_digit;
    //    }
    //    return sum;
}

test "day 1 task 1" {
    const input =
        \\Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        \\Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        \\Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        \\Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        \\Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        \\Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
    ;
    const output = task1(input);
    try expect(output == 13);
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
