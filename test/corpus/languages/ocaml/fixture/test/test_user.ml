let test_create_user () =
  let u = User.create_user "Alice" "alice@example.com" 30 in
  assert (User.get_name u = "Alice")

let test_is_adult () =
  let u = User.create_user "Bob" "bob@example.com" 25 in
  assert (User.is_adult u)
