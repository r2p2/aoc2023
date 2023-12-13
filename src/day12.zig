const std = @import("std");
const expect = std.testing.expect;

pub fn task1(allocator: std.mem.Allocator, input: []const u8) !i64 {
    var sum: i64 = 0;
    var line_it = std.mem.split(u8, input, "\n");
    while (line_it.next()) |line| {
        if (line.len == 0) {
            continue;
        }

        var part_it = std.mem.split(u8, line, " ");
        var task = part_it.next().?;

        var mutable_task = std.ArrayList(u8).init(allocator);
        defer mutable_task.deinit();
        try mutable_task.appendSlice(task);

        var rules_str = part_it.next().?;
        var rules = std.ArrayList(usize).init(allocator); // here are our expected groups
        defer rules.deinit();

        var rule_str_it = std.mem.split(u8, rules_str, ",");
        while (rule_str_it.next()) |rule_str| {
            try rules.append(try std.fmt.parseUnsigned(usize, rule_str, 10));
        }

        const count = std.mem.count(u8, task, "?");
        //        std.debug.print("\n-> TASK:'{s}'\n", .{task});
        var i: usize = 0;
        while (i < std.math.pow(usize, 2, count)) : (i += 1) {
            var j: usize = 0;
            while (j < mutable_task.items.len) : (j += 1) {
                if (mutable_task.items[j] == '?') {
                    mutable_task.items[j] = '*';
                    break;
                }
                if (mutable_task.items[j] == '*') {
                    mutable_task.items[j] = '?';
                    continue;
                }
            }

            var group_sizes = std.ArrayList(usize).init(allocator);
            defer group_sizes.deinit();
            var group_it = std.mem.tokenizeAny(u8, mutable_task.items, ".?");
            while (group_it.next()) |group| {
                //                std.debug.print("- group:{}\n", .{group.len});
                try group_sizes.append(group.len);
            }

            //            std.debug.print("'{s}'\n", .{mutable_task.items});
            if (group_sizes.items.len != rules.items.len) {
                continue;
            }

            var rule_match: bool = true;
            var idx: usize = 0;
            while (idx < group_sizes.items.len) : (idx += 1) {
                if (group_sizes.items[idx] != rules.items[idx]) {
                    rule_match = false;
                    break;
                }
            }
            if (rule_match) {
                sum += 1;
            }
        }
    }
    return sum;
}

pub fn task2(input: []const u8) i64 {
    _ = input;
    return 0;
}

test "day 12 task 1" {
    const input =
        \\???.### 1,1,3
        \\.??..??...?##. 1,1,3
        \\?#?#?#?#?#?#?#? 1,3,1,6
        \\????.#...#... 4,1,1
        \\????.######..#####. 1,6,5
        \\?###???????? 3,2,1
    ;
    const output = try task1(std.testing.allocator, input);
    std.debug.print("\n{}\n", .{output});
    try expect(output == 21);
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
