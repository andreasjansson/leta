(** Module type for storage backends. *)
module type Storage = sig
  type t
  val create : unit -> t
  val save : t -> string -> User.user -> unit
  val load : t -> string -> User.user option
  val delete : t -> string -> bool
  val list : t -> User.user list
end

(** In-memory storage implementation. *)
module MemoryStorage : Storage = struct
  type t = (string, User.user) Hashtbl.t

  let create () = Hashtbl.create 16

  let save tbl key user = Hashtbl.replace tbl key user

  let load tbl key =
    match Hashtbl.find_opt tbl key with
    | Some user -> Some user
    | None -> None

  let delete tbl key =
    if Hashtbl.mem tbl key then begin
      Hashtbl.remove tbl key;
      true
    end else false

  let list tbl =
    Hashtbl.fold (fun _ user acc -> user :: acc) tbl []
end

(** File storage implementation (stub). *)
module FileStorage : Storage = struct
  type t = {
    base_path : string;
    cache : (string, User.user) Hashtbl.t;
  }

  let create () = {
    base_path = ".";
    cache = Hashtbl.create 16;
  }

  let save storage key user =
    Hashtbl.replace storage.cache key user

  let load storage key =
    Hashtbl.find_opt storage.cache key

  let delete storage key =
    if Hashtbl.mem storage.cache key then begin
      Hashtbl.remove storage.cache key;
      true
    end else false

  let list storage =
    Hashtbl.fold (fun _ user acc -> user :: acc) storage.cache []
end
