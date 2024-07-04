const std = @import("std");
const expect = std.testing.expect;

fn linear_search(haystack: []u32, needle: u32) bool {
    for (haystack) |el| {
        if (el == needle) {
            return true;
        }
    }

    return false;
}

test "Linear search" {
    var list = [_]u32{ 9, 3, 7, 4, 69, 420, 42 };

    try expect(linear_search(&list, 42) == true);
    try expect(linear_search(&list, 1) == false);
}
