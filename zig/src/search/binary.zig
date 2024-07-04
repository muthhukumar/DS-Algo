const std = @import("std");
const expect = std.testing.expect;

fn binary_search(haystack: []i32, needle: u32) bool {
    var low: i32 = 0;
    var high: i32 = @intCast(haystack.len);

    while (low < high) {
        const cal: i32 = low + (high - low) / 2;

        const mid: usize = std.math.floor(@as(cal, f32));

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
    var list = [_]i32{ 9, 3, 7, 4, 69, 420, 42 };

    try expect(binary_search(&list, 42) == true);
    try expect(binary_search(&list, 1) == false);
}
