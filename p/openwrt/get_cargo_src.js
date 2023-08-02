// get_cargo_src.js
//
// symlink `cargo_src` -> ~/.cargo/registry/src/XXX/
//
// usage:
//   cargo metadata | deno run -A get_cargo_src.js
import { join as path_join } from "https://deno.land/std@0.196.0/path/posix.ts";
import { readAll } from "https://deno.land/std@0.196.0/streams/read_all.ts";

console.log("get_cargo_src.js");

// read stdin (json)
let b = await readAll(Deno.stdin);
let de = new TextDecoder();
let d = JSON.parse(de.decode(b));

function find_log_path(d) {
  for (let i of d.packages) {
    for (let j of i.targets) {
      if (j.name == "log") {
        return j.src_path;
      }
    }
  }
  return null;
}

let log_path = find_log_path(d);
if (log_path == null) {
  console.log("ERROR: not found log path");
  Deno.exit(1);
}

console.log(log_path);
let cargo_src = path_join(log_path, "../../../");

console.log("    " + cargo_src);
await Deno.symlink(cargo_src, "cargo_src");
