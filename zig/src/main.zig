const std = @import("std");
const testing = std.testing;

test "Main" {
    _ = @import("sort/insertion.zig");
    _ = @import("search/linear.zig");
    _ = @import("search/binary.zig");
}

pub fn main() !void {}
