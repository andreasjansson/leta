(** A user in the system. *)
type user = {
  name : string;
  email : string;
  age : int;
}

(** Creates a new user. *)
let create_user name email age = { name; email; age }

(** Returns the user's name. *)
let get_name user = user.name

(** Returns the user's email. *)
let get_email user = user.email

(** Returns the user's age. *)
let get_age user = user.age

(** Checks if the user is an adult. *)
let is_adult user = user.age >= 18

(** Returns a formatted display name. *)
let display_name user = Printf.sprintf "%s <%s>" user.name user.email

(** Validates a user and returns an error string option. *)
let validate_user user =
  if String.length user.name = 0 then Some "name is required"
  else if String.length user.email = 0 then Some "email is required"
  else None

(** Default ports for various services. *)
let default_ports = [|
  80;
  443;
  8080;
  8443;
  3000;
|]