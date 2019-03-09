use crate::{
  beam::disp_result::DispatchResult,
  defs::BitSize,
  emulator::{process::Process, runtime_ctx::Context},
  fail::RtResult,
  term::{
    boxed::binary::{match_state::BinaryMatchState, slice::BinarySlice},
    lterm::LTerm,
  },
};

/// Having started binary matching, retrieve a binary piece.
/// Structure: bs_get_binary(Fail, MatchState, Live, Size, Unit, Flags, Dst)
define_opcode!(
  _vm, rt_ctx, proc, name: OpcodeBsGetBinary2, arity: 7,
  run: {unsafe {
    Self::bs_get_binary2_7(rt_ctx, proc, fail, match_state, live, size, unit, flags, dst)
  }},
  args: cp_or_nil(fail), binary_match_state(match_state),
        usize(live), load_usize(size), usize(unit), term(flags), term(dst),
);

impl OpcodeBsGetBinary2 {
  #[inline]
  unsafe fn bs_get_binary2_7(
    runtime_ctx: &mut Context,
    proc: &mut Process,
    _fail: LTerm,
    match_state: *mut BinaryMatchState,
    live: usize,
    size: usize,
    unit: usize,
    flags: LTerm,
    dst: LTerm,
  ) -> RtResult<DispatchResult> {
    println!(
      "bgb2: live={} size={} unit={} flags={}",
      live, size, unit, flags
    );

    // Allocate a sub-binary and possibly GC if does not fit?
    let bit_size = BitSize::with_unit(size, unit);
    let src_bin = (*match_state).get_src_binary();

    if bit_size.bit_count > 0 {
      let bit_offset = (*match_state).get_offset();
      let sub_bin =
        BinarySlice::create_into(src_bin, bit_offset, bit_size, &mut proc.heap)?;
      (*match_state).increase_offset(bit_size);

      println!("bgb2: created {}", (*sub_bin).make_term());
      // Return the sub-binary created
      runtime_ctx.store_value((*sub_bin).make_term(), dst, &mut proc.heap)?;
    } else {
      // ignore error here, can't fail
      println!("bgb2: created empty <<>>");
      runtime_ctx
        .store_value(LTerm::empty_binary(), dst, &mut proc.heap)
        .unwrap();
    }

    Ok(DispatchResult::Normal)
  }
}