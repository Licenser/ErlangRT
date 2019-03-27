//! Generated by `codegen/create_vm_dispatch.py`
//! Dispatch for all opcode types.
//! Config used: OTP22
#![allow(dead_code)]

use crate::{
  beam::{disp_result::*, gen_op::*, opcodes::*},
  emulator::{code::opcode::RawOpcode, process::Process, runtime_ctx::Context, vm::VM},
  fail::RtResult,
};

#[inline]
pub fn dispatch_op_inline(vm: &mut VM, op: RawOpcode, ctx: &mut Context, curr_p: &mut Process) -> RtResult<DispatchResult> {
  match op {
    OPCODE_FUNC_INFO => {
      assert_arity(OPCODE_FUNC_INFO, OpcodeFuncInfo::ARITY);
      return OpcodeFuncInfo::__run(vm, ctx, curr_p);
    },

    OPCODE_CALL => {
      assert_arity(OPCODE_CALL, OpcodeCall::ARITY);
      return OpcodeCall::__run(vm, ctx, curr_p);
    },

    OPCODE_CALL_LAST => {
      assert_arity(OPCODE_CALL_LAST, OpcodeCallLast::ARITY);
      return OpcodeCallLast::__run(vm, ctx, curr_p);
    },

    OPCODE_CALL_ONLY => {
      assert_arity(OPCODE_CALL_ONLY, OpcodeCallOnly::ARITY);
      return OpcodeCallOnly::__run(vm, ctx, curr_p);
    },

    OPCODE_CALL_EXT => {
      assert_arity(OPCODE_CALL_EXT, OpcodeCallExt::ARITY);
      return OpcodeCallExt::__run(vm, ctx, curr_p);
    },

    OPCODE_CALL_EXT_LAST => {
      assert_arity(OPCODE_CALL_EXT_LAST, OpcodeCallExtLast::ARITY);
      return OpcodeCallExtLast::__run(vm, ctx, curr_p);
    },

    OPCODE_BIF0 => {
      assert_arity(OPCODE_BIF0, OpcodeBif0::ARITY);
      return OpcodeBif0::__run(vm, ctx, curr_p);
    },

    OPCODE_BIF1 => {
      assert_arity(OPCODE_BIF1, OpcodeBif1::ARITY);
      return OpcodeBif1::__run(vm, ctx, curr_p);
    },

    OPCODE_BIF2 => {
      assert_arity(OPCODE_BIF2, OpcodeBif2::ARITY);
      return OpcodeBif2::__run(vm, ctx, curr_p);
    },

    OPCODE_ALLOCATE => {
      assert_arity(OPCODE_ALLOCATE, OpcodeAllocate::ARITY);
      return OpcodeAllocate::__run(vm, ctx, curr_p);
    },

    OPCODE_ALLOCATE_HEAP => {
      assert_arity(OPCODE_ALLOCATE_HEAP, OpcodeAllocateHeap::ARITY);
      return OpcodeAllocateHeap::__run(vm, ctx, curr_p);
    },

    OPCODE_ALLOCATE_ZERO => {
      assert_arity(OPCODE_ALLOCATE_ZERO, OpcodeAllocateZero::ARITY);
      return OpcodeAllocateZero::__run(vm, ctx, curr_p);
    },

    OPCODE_ALLOCATE_HEAP_ZERO => {
      assert_arity(OPCODE_ALLOCATE_HEAP_ZERO, OpcodeAllocateHeapZero::ARITY);
      return OpcodeAllocateHeapZero::__run(vm, ctx, curr_p);
    },

    OPCODE_TEST_HEAP => {
      assert_arity(OPCODE_TEST_HEAP, OpcodeTestHeap::ARITY);
      return OpcodeTestHeap::__run(vm, ctx, curr_p);
    },

    OPCODE_INIT => {
      assert_arity(OPCODE_INIT, OpcodeInit::ARITY);
      return OpcodeInit::__run(vm, ctx, curr_p);
    },

    OPCODE_DEALLOCATE => {
      assert_arity(OPCODE_DEALLOCATE, OpcodeDeallocate::ARITY);
      return OpcodeDeallocate::__run(vm, ctx, curr_p);
    },

    OPCODE_RETURN => {
      assert_arity(OPCODE_RETURN, OpcodeReturn::ARITY);
      return OpcodeReturn::__run(vm, ctx, curr_p);
    },

    OPCODE_SEND => {
      assert_arity(OPCODE_SEND, OpcodeSend::ARITY);
      return OpcodeSend::__run(vm, ctx, curr_p);
    },

    OPCODE_REMOVE_MESSAGE => {
      assert_arity(OPCODE_REMOVE_MESSAGE, OpcodeRemoveMessage::ARITY);
      return OpcodeRemoveMessage::__run(vm, ctx, curr_p);
    },

    OPCODE_LOOP_REC => {
      assert_arity(OPCODE_LOOP_REC, OpcodeLoopRec::ARITY);
      return OpcodeLoopRec::__run(vm, ctx, curr_p);
    },

    OPCODE_LOOP_REC_END => {
      assert_arity(OPCODE_LOOP_REC_END, OpcodeLoopRecEnd::ARITY);
      return OpcodeLoopRecEnd::__run(vm, ctx, curr_p);
    },

    OPCODE_WAIT => {
      assert_arity(OPCODE_WAIT, OpcodeWait::ARITY);
      return OpcodeWait::__run(vm, ctx, curr_p);
    },

    OPCODE_IS_LT => {
      assert_arity(OPCODE_IS_LT, OpcodeIsLt::ARITY);
      return OpcodeIsLt::__run(vm, ctx, curr_p);
    },

    OPCODE_IS_GE => {
      assert_arity(OPCODE_IS_GE, OpcodeIsGe::ARITY);
      return OpcodeIsGe::__run(vm, ctx, curr_p);
    },

    OPCODE_IS_EQ => {
      assert_arity(OPCODE_IS_EQ, OpcodeIsEq::ARITY);
      return OpcodeIsEq::__run(vm, ctx, curr_p);
    },

    OPCODE_IS_EQ_EXACT => {
      assert_arity(OPCODE_IS_EQ_EXACT, OpcodeIsEqExact::ARITY);
      return OpcodeIsEqExact::__run(vm, ctx, curr_p);
    },

    OPCODE_IS_NE_EXACT => {
      assert_arity(OPCODE_IS_NE_EXACT, OpcodeIsNeExact::ARITY);
      return OpcodeIsNeExact::__run(vm, ctx, curr_p);
    },

    OPCODE_IS_INTEGER => {
      assert_arity(OPCODE_IS_INTEGER, OpcodeIsInteger::ARITY);
      return OpcodeIsInteger::__run(vm, ctx, curr_p);
    },

    OPCODE_IS_FLOAT => {
      assert_arity(OPCODE_IS_FLOAT, OpcodeIsFloat::ARITY);
      return OpcodeIsFloat::__run(vm, ctx, curr_p);
    },

    OPCODE_IS_NUMBER => {
      assert_arity(OPCODE_IS_NUMBER, OpcodeIsNumber::ARITY);
      return OpcodeIsNumber::__run(vm, ctx, curr_p);
    },

    OPCODE_IS_ATOM => {
      assert_arity(OPCODE_IS_ATOM, OpcodeIsAtom::ARITY);
      return OpcodeIsAtom::__run(vm, ctx, curr_p);
    },

    OPCODE_IS_PID => {
      assert_arity(OPCODE_IS_PID, OpcodeIsPid::ARITY);
      return OpcodeIsPid::__run(vm, ctx, curr_p);
    },

    OPCODE_IS_REFERENCE => {
      assert_arity(OPCODE_IS_REFERENCE, OpcodeIsReference::ARITY);
      return OpcodeIsReference::__run(vm, ctx, curr_p);
    },

    OPCODE_IS_PORT => {
      assert_arity(OPCODE_IS_PORT, OpcodeIsPort::ARITY);
      return OpcodeIsPort::__run(vm, ctx, curr_p);
    },

    OPCODE_IS_NIL => {
      assert_arity(OPCODE_IS_NIL, OpcodeIsNil::ARITY);
      return OpcodeIsNil::__run(vm, ctx, curr_p);
    },

    OPCODE_IS_BINARY => {
      assert_arity(OPCODE_IS_BINARY, OpcodeIsBinary::ARITY);
      return OpcodeIsBinary::__run(vm, ctx, curr_p);
    },

    OPCODE_IS_LIST => {
      assert_arity(OPCODE_IS_LIST, OpcodeIsList::ARITY);
      return OpcodeIsList::__run(vm, ctx, curr_p);
    },

    OPCODE_IS_NONEMPTY_LIST => {
      assert_arity(OPCODE_IS_NONEMPTY_LIST, OpcodeIsNonemptyList::ARITY);
      return OpcodeIsNonemptyList::__run(vm, ctx, curr_p);
    },

    OPCODE_IS_TUPLE => {
      assert_arity(OPCODE_IS_TUPLE, OpcodeIsTuple::ARITY);
      return OpcodeIsTuple::__run(vm, ctx, curr_p);
    },

    OPCODE_TEST_ARITY => {
      assert_arity(OPCODE_TEST_ARITY, OpcodeTestArity::ARITY);
      return OpcodeTestArity::__run(vm, ctx, curr_p);
    },

    OPCODE_SELECT_VAL => {
      assert_arity(OPCODE_SELECT_VAL, OpcodeSelectVal::ARITY);
      return OpcodeSelectVal::__run(vm, ctx, curr_p);
    },

    OPCODE_JUMP => {
      assert_arity(OPCODE_JUMP, OpcodeJump::ARITY);
      return OpcodeJump::__run(vm, ctx, curr_p);
    },

    OPCODE_MOVE => {
      assert_arity(OPCODE_MOVE, OpcodeMove::ARITY);
      return OpcodeMove::__run(vm, ctx, curr_p);
    },

    OPCODE_GET_LIST => {
      assert_arity(OPCODE_GET_LIST, OpcodeGetList::ARITY);
      return OpcodeGetList::__run(vm, ctx, curr_p);
    },

    OPCODE_GET_TUPLE_ELEMENT => {
      assert_arity(OPCODE_GET_TUPLE_ELEMENT, OpcodeGetTupleElement::ARITY);
      return OpcodeGetTupleElement::__run(vm, ctx, curr_p);
    },

    OPCODE_SET_TUPLE_ELEMENT => {
      assert_arity(OPCODE_SET_TUPLE_ELEMENT, OpcodeSetTupleElement::ARITY);
      return OpcodeSetTupleElement::__run(vm, ctx, curr_p);
    },

    OPCODE_PUT_LIST => {
      assert_arity(OPCODE_PUT_LIST, OpcodePutList::ARITY);
      return OpcodePutList::__run(vm, ctx, curr_p);
    },

    OPCODE_PUT_TUPLE => {
      assert_arity(OPCODE_PUT_TUPLE, OpcodePutTuple::ARITY);
      return OpcodePutTuple::__run(vm, ctx, curr_p);
    },

    OPCODE_BADMATCH => {
      assert_arity(OPCODE_BADMATCH, OpcodeBadmatch::ARITY);
      return OpcodeBadmatch::__run(vm, ctx, curr_p);
    },

    OPCODE_CALL_FUN => {
      assert_arity(OPCODE_CALL_FUN, OpcodeCallFun::ARITY);
      return OpcodeCallFun::__run(vm, ctx, curr_p);
    },

    OPCODE_IS_FUNCTION => {
      assert_arity(OPCODE_IS_FUNCTION, OpcodeIsFunction::ARITY);
      return OpcodeIsFunction::__run(vm, ctx, curr_p);
    },

    OPCODE_CALL_EXT_ONLY => {
      assert_arity(OPCODE_CALL_EXT_ONLY, OpcodeCallExtOnly::ARITY);
      return OpcodeCallExtOnly::__run(vm, ctx, curr_p);
    },

    OPCODE_BS_PUT_INTEGER => {
      assert_arity(OPCODE_BS_PUT_INTEGER, OpcodeBsPutInteger::ARITY);
      return OpcodeBsPutInteger::__run(vm, ctx, curr_p);
    },

    OPCODE_BS_PUT_BINARY => {
      assert_arity(OPCODE_BS_PUT_BINARY, OpcodeBsPutBinary::ARITY);
      return OpcodeBsPutBinary::__run(vm, ctx, curr_p);
    },

    OPCODE_MAKE_FUN2 => {
      assert_arity(OPCODE_MAKE_FUN2, OpcodeMakeFun2::ARITY);
      return OpcodeMakeFun2::__run(vm, ctx, curr_p);
    },

    OPCODE_TRY => {
      assert_arity(OPCODE_TRY, OpcodeTry::ARITY);
      return OpcodeTry::__run(vm, ctx, curr_p);
    },

    OPCODE_TRY_END => {
      assert_arity(OPCODE_TRY_END, OpcodeTryEnd::ARITY);
      return OpcodeTryEnd::__run(vm, ctx, curr_p);
    },

    OPCODE_TRY_CASE => {
      assert_arity(OPCODE_TRY_CASE, OpcodeTryCase::ARITY);
      return OpcodeTryCase::__run(vm, ctx, curr_p);
    },

    OPCODE_RAISE => {
      assert_arity(OPCODE_RAISE, OpcodeRaise::ARITY);
      return OpcodeRaise::__run(vm, ctx, curr_p);
    },

    OPCODE_BS_INIT2 => {
      assert_arity(OPCODE_BS_INIT2, OpcodeBsInit2::ARITY);
      return OpcodeBsInit2::__run(vm, ctx, curr_p);
    },

    OPCODE_BS_ADD => {
      assert_arity(OPCODE_BS_ADD, OpcodeBsAdd::ARITY);
      return OpcodeBsAdd::__run(vm, ctx, curr_p);
    },

    OPCODE_APPLY => {
      assert_arity(OPCODE_APPLY, OpcodeApply::ARITY);
      return OpcodeApply::__run(vm, ctx, curr_p);
    },

    OPCODE_APPLY_LAST => {
      assert_arity(OPCODE_APPLY_LAST, OpcodeApplyLast::ARITY);
      return OpcodeApplyLast::__run(vm, ctx, curr_p);
    },

    OPCODE_IS_FUNCTION2 => {
      assert_arity(OPCODE_IS_FUNCTION2, OpcodeIsFunction2::ARITY);
      return OpcodeIsFunction2::__run(vm, ctx, curr_p);
    },

    OPCODE_BS_GET_BINARY2 => {
      assert_arity(OPCODE_BS_GET_BINARY2, OpcodeBsGetBinary2::ARITY);
      return OpcodeBsGetBinary2::__run(vm, ctx, curr_p);
    },

    OPCODE_BS_TEST_TAIL2 => {
      assert_arity(OPCODE_BS_TEST_TAIL2, OpcodeBsTestTail2::ARITY);
      return OpcodeBsTestTail2::__run(vm, ctx, curr_p);
    },

    OPCODE_GC_BIF1 => {
      assert_arity(OPCODE_GC_BIF1, OpcodeGcBif1::ARITY);
      return OpcodeGcBif1::__run(vm, ctx, curr_p);
    },

    OPCODE_GC_BIF2 => {
      assert_arity(OPCODE_GC_BIF2, OpcodeGcBif2::ARITY);
      return OpcodeGcBif2::__run(vm, ctx, curr_p);
    },

    OPCODE_TRIM => {
      assert_arity(OPCODE_TRIM, OpcodeTrim::ARITY);
      return OpcodeTrim::__run(vm, ctx, curr_p);
    },

    OPCODE_GC_BIF3 => {
      assert_arity(OPCODE_GC_BIF3, OpcodeGcBif3::ARITY);
      return OpcodeGcBif3::__run(vm, ctx, curr_p);
    },

    OPCODE_IS_TAGGED_TUPLE => {
      assert_arity(OPCODE_IS_TAGGED_TUPLE, OpcodeIsTaggedTuple::ARITY);
      return OpcodeIsTaggedTuple::__run(vm, ctx, curr_p);
    },

    OPCODE_GET_HD => {
      assert_arity(OPCODE_GET_HD, OpcodeGetHd::ARITY);
      return OpcodeGetHd::__run(vm, ctx, curr_p);
    },

    OPCODE_GET_TL => {
      assert_arity(OPCODE_GET_TL, OpcodeGetTl::ARITY);
      return OpcodeGetTl::__run(vm, ctx, curr_p);
    },

    OPCODE_PUT_TUPLE2 => {
      assert_arity(OPCODE_PUT_TUPLE2, OpcodePutTuple2::ARITY);
      return OpcodePutTuple2::__run(vm, ctx, curr_p);
    },

    OPCODE_BS_START_MATCH3 => {
      assert_arity(OPCODE_BS_START_MATCH3, OpcodeBsStartMatch3::ARITY);
      return OpcodeBsStartMatch3::__run(vm, ctx, curr_p);
    },

    other => unknown_opcode(other, ctx),
  }
  Ok(DispatchResult::Yield(YieldType::EndOfTheQueue))
}

