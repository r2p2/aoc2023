const std = @import("std");
const expect = std.testing.expect;

const Star = struct {
    i: usize,
    x: i64,
    y: i64,
    x_start: i64,
    y_start: i64,
};

fn distance(s1: Star, s2: Star) u64 {
    const dx = @max(s1.x, s2.x) - @min(s1.x, s2.x);
    const dy = @max(s1.y, s2.y) - @min(s1.y, s2.y);

    return @bitCast(dx + dy);
}

pub fn task1(allocator: std.mem.Allocator, input: []const u8) !u64 {
    var stars = std.ArrayList(Star).init(allocator);
    defer stars.deinit();

    const width: usize = std.mem.indexOf(u8, input, "\n").? + 1;

    //    std.debug.print("\n0\n", .{});
    { // parse star positions
        var i: usize = 0;
        while (i < input.len) : (i += 1) {
            if (input[i] != '#') {
                continue;
            }

            try stars.append(Star{
                .i = stars.items.len + 1,
                .x = @as(i64, @bitCast(i % width)),
                .y = @as(i64, @bitCast(i / width)),
                .x_start = @as(i64, @bitCast(i % width)),
                .y_start = @as(i64, @bitCast(i / width)),
            });
        }
    }
    //    std.debug.print("\na\n", .{});

    var x_max: i64 = 0;
    var y_max: i64 = 0;
    { // find outer most positions
        var i: usize = 0;
        while (i < stars.items.len) : (i += 1) {
            if (stars.items[i].x > x_max) {
                x_max = stars.items[i].x;
            }
            if (stars.items[i].y > y_max) {
                y_max = stars.items[i].y;
            }
        }
    }
    //    std.debug.print("\nA\n", .{});

    { // debug starts
        var i: usize = 0;
        while (i < stars.items.len) : (i += 1) {
            const s1 = stars.items[i];
            _ = s1;
            var j: usize = i + 1;
            while (j < stars.items.len) : (j += 1) {
                const s2 = stars.items[j];
                _ = s2;

                //                std.debug.print("({}){},{} -> ({}){},{}\n", .{ s1.i, s1.x, s1.y, s2.i, s2.x, s2.y });
            }
        }
    }

    { // grow universe
        { // x
            var x: usize = 0;
            while (x < x_max) : (x += 1) {
                var line_has_star: bool = false;
                {
                    var i: usize = 0;
                    while (i < stars.items.len) : (i += 1) {
                        if (stars.items[i].x_start == x) {
                            line_has_star = true;
                            break;
                        }
                    }
                }
                if (!line_has_star) {
                    {
                        var i: usize = 0;
                        while (i < stars.items.len) : (i += 1) {
                            if (stars.items[i].x_start > x) {
                                stars.items[i].x += 1;
                                //                                std.debug.print("\ngrow {}-{},{}\n", .{ stars.items[i].i, stars.items[i].x, stars.items[i].y });
                            }
                        }
                    }

                    //      x += 1;
                }
            }
        }
        //       std.debug.print("\nB\n", .{});
        { // y
            var y: usize = 0;
            while (y < y_max) : (y += 1) {
                var line_has_star: bool = false;
                {
                    var i: usize = 0;
                    while (i < stars.items.len) : (i += 1) {
                        if (stars.items[i].y_start == y) {
                            line_has_star = true;
                            break;
                        }
                    }
                }
                if (!line_has_star) {
                    {
                        var i: usize = 0;
                        while (i < stars.items.len) : (i += 1) {
                            if (stars.items[i].y_start > y) {
                                stars.items[i].y += 1;
                                //       std.debug.print("\ngrow {}\n", .{stars.items[i].i});
                            }
                        }
                    }

                    //    y += 1;
                }
            }
        }
    }
    //    std.debug.print("\nC\n", .{});

    var sum_dist: u64 = 0;
    { // calculate distances
        var i: usize = 0;
        while (i < stars.items.len) : (i += 1) {
            const s1 = stars.items[i];
            var j: usize = i + 1;
            while (j < stars.items.len) : (j += 1) {
                const s2 = stars.items[j];

                //                std.debug.print("({}){},{} -> ({}){},{} = {}\n", .{ s1.i, s1.x, s1.y, s2.i, s2.x, s2.y, distance(s1, s2) });
                sum_dist += distance(s1, s2);
            }
        }
    }

    //   std.debug.print("\nSOL:{}\n", .{sum_dist});
    return sum_dist;
}

pub fn task2(allocator: std.mem.Allocator, input: []const u8) !u64 {
    var stars = std.ArrayList(Star).init(allocator);
    defer stars.deinit();

    const width: usize = std.mem.indexOf(u8, input, "\n").? + 1;

    //    std.debug.print("\n0\n", .{});
    { // parse star positions
        var i: usize = 0;
        while (i < input.len) : (i += 1) {
            if (input[i] != '#') {
                continue;
            }

            try stars.append(Star{
                .i = stars.items.len + 1,
                .x = @as(i64, @bitCast(i % width)),
                .y = @as(i64, @bitCast(i / width)),
                .x_start = @as(i64, @bitCast(i % width)),
                .y_start = @as(i64, @bitCast(i / width)),
            });
        }
    }
    //    std.debug.print("\na\n", .{});

    var x_max: i64 = 0;
    var y_max: i64 = 0;
    { // find outer most positions
        var i: usize = 0;
        while (i < stars.items.len) : (i += 1) {
            if (stars.items[i].x > x_max) {
                x_max = stars.items[i].x;
            }
            if (stars.items[i].y > y_max) {
                y_max = stars.items[i].y;
            }
        }
    }
    //    std.debug.print("\nA\n", .{});

    { // debug starts
        var i: usize = 0;
        while (i < stars.items.len) : (i += 1) {
            const s1 = stars.items[i];
            _ = s1;
            var j: usize = i + 1;
            while (j < stars.items.len) : (j += 1) {
                const s2 = stars.items[j];
                _ = s2;

                //                std.debug.print("({}){},{} -> ({}){},{}\n", .{ s1.i, s1.x, s1.y, s2.i, s2.x, s2.y });
            }
        }
    }

    { // grow universe
        { // x
            var x: usize = 0;
            while (x < x_max) : (x += 1) {
                var line_has_star: bool = false;
                {
                    var i: usize = 0;
                    while (i < stars.items.len) : (i += 1) {
                        if (stars.items[i].x_start == x) {
                            line_has_star = true;
                            break;
                        }
                    }
                }
                if (!line_has_star) {
                    {
                        var i: usize = 0;
                        while (i < stars.items.len) : (i += 1) {
                            if (stars.items[i].x_start > x) {
                                stars.items[i].x += 999999;
                                //                                std.debug.print("\ngrow {}-{},{}\n", .{ stars.items[i].i, stars.items[i].x, stars.items[i].y });
                            }
                        }
                    }

                    //      x += 1;
                }
            }
        }
        //       std.debug.print("\nB\n", .{});
        { // y
            var y: usize = 0;
            while (y < y_max) : (y += 1) {
                var line_has_star: bool = false;
                {
                    var i: usize = 0;
                    while (i < stars.items.len) : (i += 1) {
                        if (stars.items[i].y_start == y) {
                            line_has_star = true;
                            break;
                        }
                    }
                }
                if (!line_has_star) {
                    {
                        var i: usize = 0;
                        while (i < stars.items.len) : (i += 1) {
                            if (stars.items[i].y_start > y) {
                                stars.items[i].y += 999999;
                                //       std.debug.print("\ngrow {}\n", .{stars.items[i].i});
                            }
                        }
                    }

                    //    y += 1;
                }
            }
        }
    }
    //    std.debug.print("\nC\n", .{});

    var sum_dist: u64 = 0;
    { // calculate distances
        var i: usize = 0;
        while (i < stars.items.len) : (i += 1) {
            const s1 = stars.items[i];
            var j: usize = i + 1;
            while (j < stars.items.len) : (j += 1) {
                const s2 = stars.items[j];

                //                std.debug.print("({}){},{} -> ({}){},{} = {}\n", .{ s1.i, s1.x, s1.y, s2.i, s2.x, s2.y, distance(s1, s2) });
                sum_dist += distance(s1, s2);
            }
        }
    }

    //   std.debug.print("\nSOL:{}\n", .{sum_dist});
    return sum_dist;
}

//test "distance" {
//    std.debug.print("\n\n{}\n\n", .{distance(Star{ .i = 0, .x = 0, .y = 0 , .x_start = 0}, Star{ .i = 1, .x = 0, .y = 1 })});
//    std.debug.print("\n\n{}\n\n", .{distance(Star{ .i = 0, .x = 0, .y = 0 , .x_start = 0}, Star{ .i = 1, .x = 1, .y = 1 })});
//    std.debug.print("\n\n{}\n\n", .{distance(Star{ .i = 0, .x = 0, .y = 11, .x_start = 0 }, Star{ .i = 1, .x = 5, .y = 11 })});
//    //try expect(distance(Star{ .i = 0, .x = 0, .y = 0 }, Star{ .i = 1, .x = 1, .y = 1 }) == 2);
//    //try expect(distance(Star{ .i = 0, .x = 1, .y = 1 }, Star{ .i = 1, .x = 0, .y = 0 }) == 2);
//}
//
test "day 1 task 1" {
    const input =
        \\...#......
        \\.......#..
        \\#.........
        \\..........
        \\......#...
        \\.#........
        \\.........#
        \\..........
        \\.......#..
        \\#...#.....
    ;
    const output = try task1(std.testing.allocator, input);
    try expect(output == 374);
}

test "day 1 task 2" {
    //    const input =
    //        \\ two1nine
    //        \\ eightwothree
    //        \\ abcone2threexyz
    //        \\ xtwone3four
    //        \\ 4nineeightseven2
    //        \\ zoneight234
    //        \\ 7pqrstsixteen
    //    ;
    //    const output = try task2(input);
    //    try expect(output == 281);
}
