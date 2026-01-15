(** Custom error types for the application. *)

exception Not_found_error of string
exception Validation_error of string
exception Storage_error of string

(** Raises a not found error. *)
let not_found msg = raise (Not_found_error msg)

(** Raises a validation error. *)
let validation_error msg = raise (Validation_error msg)

(** Raises a storage error. *)
let storage_error msg = raise (Storage_error msg)

(** Error code constants. *)
let error_not_found = 404
let error_validation = 400
let error_storage = 500
