(** Creates a sample user for testing. *)
let create_sample_user () =
  User.create_user "John Doe" "john@example.com" 30

(** Processes a list of users and prints their display names. *)
let process_users users =
  List.iter (fun u -> print_endline (User.display_name u)) users

(** Main entry point. *)
let main () =
  let storage = Storage.MemoryStorage.create () in
  let user = create_sample_user () in

  Storage.MemoryStorage.save storage (User.get_email user) user;

  begin match Storage.MemoryStorage.load storage "john@example.com" with
  | Some found ->
    Printf.printf "Found user: %s\n" (User.display_name found);
    Printf.printf "Is adult: %b\n" (User.is_adult found)
  | None ->
    print_endline "User not found"
  end;

  let users = Storage.MemoryStorage.list storage in
  process_users users

let () = main ()
