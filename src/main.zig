const std = @import("std");
const day1 = @import("day1.zig");

pub fn main() !void {
    // setup memory
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();

    const allocator = gpa.allocator();

    // parse command line arguments
    var args = std.process.args();
    _ = args.skip(); // skip binary name

    const day_str = args.next().?;
    const part_str = args.next().?;
    const input_file_path = args.next().?;

    const day = try std.fmt.parseInt(u8, day_str, 10);
    const part = try std.fmt.parseInt(u8, part_str, 10);

    // read provided input file into memory
    const file = try std.fs.cwd().openFile(input_file_path, .{ .mode = std.fs.File.OpenMode.read_only });
    defer file.close();

    const input = try file.readToEndAlloc(allocator, 1024 * 1024 * 25);
    defer allocator.free(input);

    // call requested challange
    const stdout = std.io.getStdOut().writer();
    if (day == 0 and part == 0) {
        try stdout.print("input file:\n{s}\n", .{input});
    } else if (day == 1 and part == 1) {
        try stdout.print("{}\n", .{try day1.task1(input)});
    } else if (day == 1 and part == 2) {
        try stdout.print("{}\n", .{try day1.task2(input)});
    } else {
        std.debug.panic("ERROR: No implementation found for day:{} part:{}\n", .{ day, part });
    }
}
