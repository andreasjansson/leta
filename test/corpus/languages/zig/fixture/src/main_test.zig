const std = @import("std");
const user = @import("user.zig");

test "create user" {
    const u = user.User.init("Alice", "alice@example.com", 30);
    try std.testing.expectEqualStrings("Alice", u.name);
}

test "is adult" {
    const adult = user.User.init("Bob", "bob@example.com", 25);
    try std.testing.expect(adult.isAdult());
}
