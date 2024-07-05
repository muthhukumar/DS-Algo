const std = @import("std");
const expect = std.testing.expect;

fn binary_search(haystack: []const i32, needle: u32) bool {
    var low: usize = 0;
    var high = haystack.len;

    while (low < high) {
        const mid: usize = low + @divFloor(high - low, 2);

        const val = haystack[mid];

        if (val == needle) {
            return true;
        } else if (needle > val) {
            low = mid + 1;
        } else {
            high = mid;
        }
    }

    return false;
}

test "Binary search" {
    var list = [_]i32{ 3, 4, 7, 9, 42, 69, 420 };

    try expect(binary_search(&list, 42) == true);
    try expect(binary_search(&list, 1) == false);
}
