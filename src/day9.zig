const std = @import("std");
const expect = std.testing.expect;

fn isLineZero(line: []const i64) bool {
    for (line) |element| {
        if (element != 0) {
            return false;
        }
    }
    return true;
}

pub fn task1(allocator: std.mem.Allocator, input: []const u8) !i64 {
    var sum: i64 = 0;

    var arena = std.heap.ArenaAllocator.init(allocator);
    defer arena.deinit();

    var arena_allocator = arena.allocator();

    var line_it = std.mem.split(u8, input, "\n");
    while (line_it.next()) |line| {
        if (line.len == 0) {
            continue;
        }

        var histories = std.ArrayList(std.ArrayList(i64)).init(arena_allocator);

        { // parse history on line
            try histories.append(std.ArrayList(i64).init(arena_allocator));
            //defer histories.getLast().deinit();
            var token_it = std.mem.tokenize(u8, line, " ");
            while (token_it.next()) |token| {
                const n = try std.fmt.parseInt(i64, token, 10);
                try histories.items[histories.items.len - 1].append(n);
            }
        }

        { // calculate deltas
            var i: usize = 0;
            while (!isLineZero(histories.items[i].items) or histories.items[i].items.len == 0) : (i += 1) {
                { // build next delta from current line
                    try histories.append(std.ArrayList(i64).init(arena_allocator));
                    var j: usize = 0;
                    while (j < (histories.items[i].items.len - 1)) : (j += 1) {
                        const delta = histories.items[i].items[j + 1] - histories.items[i].items[j];
                        try histories.items[i + 1].append(delta);
                    }
                }
            }
        }

        { // calculate next numbers
            var i: usize = histories.items.len - 2;
            while (true) : (i -= 1) {
                try histories.items[i].append(histories.items[i].items[histories.items[i].items.len - 1] + histories.items[i + 1].items[histories.items[i + 1].items.len - 1]);
                if (i == 0) {
                    break;
                }
            }
        }

        const next_number = histories.items[0].items[histories.items[0].items.len - 1]; // + histories.items[1].items[histories.items[1].items.len - 1];
        sum += next_number;

        //{ // debug
        //    std.debug.print("\n\n", .{});
        //    var i: usize = 0;
        //    while (i < histories.items.len) : (i += 1) {
        //        var j: usize = 0;
        //        while (j < histories.items[i].items.len) : (j += 1) {
        //            std.debug.print("{} ", .{histories.items[i].items[j]});
        //        }
        //        std.debug.print("\n", .{});
        //    }
        //}
    }
    return sum;
}

pub fn task2(allocator: std.mem.Allocator, input: []const u8) !i64 {
    var sum: i64 = 0;

    var arena = std.heap.ArenaAllocator.init(allocator);
    defer arena.deinit();

    var arena_allocator = arena.allocator();

    var line_it = std.mem.split(u8, input, "\n");
    while (line_it.next()) |line| {
        if (line.len == 0) {
            continue;
        }

        var histories = std.ArrayList(std.ArrayList(i64)).init(arena_allocator);

        { // parse history on line
            try histories.append(std.ArrayList(i64).init(arena_allocator));
            //defer histories.getLast().deinit();
            var token_it = std.mem.tokenize(u8, line, " ");
            while (token_it.next()) |token| {
                const n = try std.fmt.parseInt(i64, token, 10);
                try histories.items[histories.items.len - 1].append(n);
            }
        }

        { // calculate deltas
            var i: usize = 0;
            while (!isLineZero(histories.items[i].items) or histories.items[i].items.len == 0) : (i += 1) {
                { // build next delta from current line
                    try histories.append(std.ArrayList(i64).init(arena_allocator));
                    var j: usize = 0;
                    while (j < (histories.items[i].items.len - 1)) : (j += 1) {
                        const delta = histories.items[i].items[j + 1] - histories.items[i].items[j];
                        try histories.items[i + 1].append(delta);
                    }
                }
            }
        }

        var last_nr: i64 = 0;
        { // calculate prev numbers
            var i: usize = histories.items.len - 2;
            while (true) : (i -= 1) {
                last_nr = histories.items[i].items[0] - last_nr;

                if (i == 0) {
                    break;
                }
            }
        }

        // const next_number = histories.items[0].items[histories.items[0].items.len - 1]; // + histories.items[1].items[histories.items[1].items.len - 1];
        sum += last_nr;

        //{ // debug
        //    std.debug.print("\n\n", .{});
        //    var i: usize = 0;
        //    while (i < histories.items.len) : (i += 1) {
        //        var j: usize = 0;
        //        while (j < histories.items[i].items.len) : (j += 1) {
        //            std.debug.print("{} ", .{histories.items[i].items[j]});
        //        }
        //        std.debug.print("\n", .{});
        //    }
        //}
    }
    return sum;
}

test "day 9 task 1" {
    const input =
        \\0 3 6 9 12 15
        \\1 3 6 10 15 21
        \\10 13 16 21 30 45
    ;
    const actual = try task1(std.testing.allocator, input);
    std.debug.print("-> {}\n", .{actual});
    try expect(actual == 114);
}

test "day 9 task 2" {
    const input =
        \\0 3 6 9 12 15
        \\1 3 6 10 15 21
        \\10 13 16 21 30 45
    ;
    const actual = try task2(std.testing.allocator, input);
    std.debug.print("-> {}\n", .{actual});
    try expect(actual == 2);
}

//test "day 8 task 2" {
//    const input =
//        \\LR
//        \\
//        \\11A = (11B, XXX)
//        \\11B = (XXX, 11Z)
//        \\11Z = (11B, XXX)
//        \\22A = (22B, XXX)
//        \\22B = (22C, 22C)
//        \\22C = (22Z, 22Z)
//        \\22Z = (22B, 22B)
//        \\XXX = (XXX, XXX)
//    ;
//    const output = try task2(std.testing.allocator, input);
//    try expect(output == 6);
//}
