const std = @import("std");
const expect = std.testing.expect;

fn insertionSort(arr: []*u32) void {
    const arr_len = arr.len;

    var i: u32 = 1;

    while (i < arr_len) : (i += 1) {
        const key = arr[i];

        var j = i - 1;

        while (j > 0 and arr[j] > key) : (j = j - 1) {
            arr[j + 1] = arr[j];
        }
        arr[j + 1] = key;
    }
}

test "insertion sort" {
    var list = [_]u32{ 9, 3, 7, 4, 69, 420, 42 };

    insertionSort(&list);

    const expect_result = [_]u32{ 3, 4, 7, 9, 42, 69, 420 };

    // try expect(list == expect_result);
    for (expect_result, list) |e, i| {
        try expect(e == i);
    }
}
