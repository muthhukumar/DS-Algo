const std = @import("std");

pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);

    defer arena.deinit();
    const allocator = arena.allocator();

    const fileName = readArgs(allocator) catch {
        std.debug.panic("Failed to read filename", .{});
    } orelse "";

    if (std.mem.eql(u8, fileName, "")) {
        std.debug.panic("Filename not provided", .{});
    }

    const finalFileName = try formatTitle(allocator, fileName);

    const fullName = try std.fmt.allocPrint(allocator, "leetcode/{s}.ts", .{finalFileName});

    const file = try std.fs.cwd().createFile(fullName, .{});
    file.close();
}

fn formatTitle(allocator: std.mem.Allocator, title: []const u8) ![]u8 {
    var normalizedTitle = try allocator.alloc(u8, title.len * 2); // Allocate enough space

    var destIdx: usize = 0;

    for (title) |c| {
        if (c >= '0' and c <= '9') {
            normalizedTitle[destIdx] = c;
            destIdx += 1;
        } else if (c == '.') {
            continue;
        } else if (c == ' ') {
            normalizedTitle[destIdx] = '-';
            destIdx += 1;
        } else {
            normalizedTitle[destIdx] = std.ascii.toLower(c);
            destIdx += 1;
        }
    }

    return normalizedTitle[0..destIdx];
}

fn readArgs(allocator: std.mem.Allocator) !?[]const u8 {
    var iterator = try std.process.argsWithAllocator(allocator);
    defer iterator.deinit();

    _ = iterator.next();

    if (iterator.next()) |fileName| {
        return fileName;
    }

    return undefined;
}
