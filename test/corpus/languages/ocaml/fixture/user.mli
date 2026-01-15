(** A user in the system. *)
type user = {
  name : string;
  email : string;
  age : int;
}

(** Creates a new user. *)
val create_user : string -> string -> int -> user

(** Returns the user's name. *)
val get_name : user -> string

(** Returns the user's email. *)
val get_email : user -> string

(** Returns the user's age. *)
val get_age : user -> int

(** Checks if the user is an adult. *)
val is_adult : user -> bool

(** Returns a formatted display name. *)
val display_name : user -> string

(** Validates a user and returns an error string option. *)
val validate_user : user -> string option

(** Default ports for various services. *)
val default_ports : int array
