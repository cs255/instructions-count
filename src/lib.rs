#![feature(plugin_registrar, rustc_private, box_syntax)]

extern crate rustc;
extern crate rustc_plugin;
extern crate rustc_const_math;
extern crate syntax;

// These imports should be all you need, and you might not even need them all
use rustc::middle::const_val::ConstVal;
use rustc_const_math::ConstInt;

use rustc::mir::transform::{self, MirPass, MirSource};
use rustc::mir::{Mir, Operand, Constant, Literal, Lvalue, SourceInfo,
                 ARGUMENT_VISIBILITY_SCOPE, StatementKind, Statement, Rvalue,
                 BinOp};
use rustc::ty::TyCtxt;
use rustc_plugin::Registry;
use syntax::codemap::DUMMY_SP;

// Declare a dummy struct in which our pass will reside
struct Pass;

// Implement Pass for the struct
impl transform::Pass for Pass {}

// Implement MirPass for the struct
impl<'tcx> MirPass<'tcx> for Pass {
  fn run_pass<'a>(&mut self,
                  // The type context: very useful in determining type info
                  tcx: TyCtxt<'a, 'tcx, 'tcx>,
                  // What type of source is this Mir for?
                  src: MirSource,
                  // The representation of Mir
                  mir: &mut Mir<'tcx>) {

    // Only perform this pass on Functions
    // (Rust doesn't allow recursive static blocks)
    if let MirSource::Fn(_) = src {

      // Modify "X" to be whatever you global is named,
      // make sure this is right, and that it doesn't share
      // names with any other node, because we are doing this in
      // a very dumb way
      let suffix = ["X".to_string()];
      let nodeid = tcx.map.nodes_matching_suffix(&suffix).next().unwrap();
      let global_def_id = tcx.map.local_def_id(nodeid);

      // This is the rust compiler's structure representing the constant
      // 1 as a 32 bit unsigned integer. Use this when adding
      let add_const = Operand::Constant(Constant {
        span: DUMMY_SP,
        ty: tcx.types.u32,
        literal: Literal::Value {
          value: ConstVal::Integral(ConstInt::U32(1 as u32)),
        },
      });
      // Used in creating statements
      let dummy_source_info = SourceInfo {
        span: DUMMY_SP,
        scope: ARGUMENT_VISIBILITY_SCOPE,
      };
      // TODO: implement the dynamic counting pass
    }
  }
}

// Register the pass
#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
  reg.register_mir_pass(box Pass);
}
