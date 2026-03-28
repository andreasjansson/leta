local user = require("user")

local function test_create_user()
    local u = user.User.new("Alice", "alice@example.com", 30)
    assert(u.name == "Alice")
end

local function test_is_adult()
    local u = user.User.new("Bob", "bob@example.com", 25)
    assert(u:isAdult())
end

return { test_create_user = test_create_user, test_is_adult = test_is_adult }
