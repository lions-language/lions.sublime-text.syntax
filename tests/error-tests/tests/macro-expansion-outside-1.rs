#[macro_use]
extern crate dcrate;

// This is an example of an error in a macro from an external crate.  These
// messages do not have a file_name value, and thus will only be displayed in
// the console (when building).  On-save syntax highlighting will display them
// at the bottom of the "root" source file.

/*BEGIN*/example_bad_syntax!{}/*END*/
//       ^^^^^^^^^^^^^^^^^^^^^HELP(>=1.44.0-beta) in this macro invocation
//       ^^^^^^^^^^^^^^^^^^^^^MSG(>=1.44.0-beta) See Primary: lib.rs:20
// ~ERR(>=1.20.0,<1.44.0-beta) /expected one of .*, found `:`/
// ~ERR(>=1.20.0,<1.44.0-beta) this error originates in a macro outside of the current crate
// ~ERR(>=1.20.0,<1.41.0-beta) /expected one of .* here/
// ~ERR(>=1.41.0-beta,<1.44.0-beta) /expected one of .* tokens/
// ~ERR(>=1.20.0,<1.24.0-beta) unexpected token
// ~ERR(>=1.20.0,<1.34.0-beta) /expected one of .*, found `:`/
// ~ERR(>=1.20.0,<1.34.0-beta) this error originates in a macro outside of the current crate
// ~ERR(>=1.20.0,<1.34.0-beta) expected one of
// ~ERR(>=1.32.0,<1.34.0-beta) in this macro
// ~ERR(>=1.20.0,<1.24.0) unexpected token
// end-msg: ERR(>=1.19.0,<1.20.0) /expected one of .*, found `:`/
// end-msg: ERR(>=1.19.0,<1.20.0) Errors occurred in macro <example_bad_syntax macros> from external crate
// end-msg: ERR(>=1.19.0,<1.20.0) Macro text: (  ) => { enum E { Kind ( x : u32 ) } }
// end-msg: ERR(>=1.19.0,<1.20.0) /expected one of .* here/
// end-msg: ERR(>=1.19.0,<1.20.0) unexpected token
// end-msg: ERR(>=1.19.0,<1.20.0) /expected one of .*, found `:`/
// end-msg: ERR(>=1.19.0,<1.20.0) Errors occurred in macro <example_bad_syntax macros> from external crate
// end-msg: ERR(>=1.19.0,<1.20.0) Macro text: (  ) => { enum E { Kind ( x : u32 ) } }
// end-msg: ERR(>=1.19.0,<1.20.0) expected one of 7 possible tokens here
// end-msg: ERR(>=1.19.0,<1.20.0) unexpected token
// end-msg: ERR(<1.19.0) /expected one of .*, found `:`/
// end-msg: ERR(<1.19.0) Errors occurred in macro <example_bad_syntax macros> from external crate
// end-msg: ERR(<1.19.0) Macro text: (  ) => { enum E { Kind ( x : u32 ) } }
// end-msg: ERR(>=1.18.0,<1.19.0) /expected one of .* here/
// end-msg: ERR(>=1.18.0,<1.19.0) unexpected token
// end-msg: ERR(<1.19.0) /expected one of .*, found `:`/
// end-msg: ERR(<1.19.0) Errors occurred in macro <example_bad_syntax macros> from external crate
// end-msg: ERR(<1.19.0) Macro text: (  ) => { enum E { Kind ( x : u32 ) } }
// end-msg: ERR(>=1.18.0,<1.19.0) expected one of 7 possible tokens here
// end-msg: ERR(>=1.18.0,<1.19.0) unexpected token
