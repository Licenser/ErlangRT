use emulator::heap::Heap;
use term::lterm::LTerm;
use fail::Error;
use fail::Hopefully;
use rt_defs::{Word};
use term::boxed::{BoxHeader, BoxTypeTag};
use term::boxed;


/// A fixed-size array which stores everything in its allocated memory on
/// process heap.
pub struct Tuple {
  header: BoxHeader,
}

impl Tuple {
  /// Size of a tuple in memory with the header word (used for allocations)
  #[inline]
  pub const fn storage_size(arity: Word) -> Word {
    arity + BoxHeader::storage_size()
  }


  pub fn get_arity(self) -> Word {
    self.header.t
  }


  /// Allocate `size+1` cells and form a tuple in memory, return the pointer.
  pub fn create_into(hp: &mut Heap, arity: Word) -> Hopefully<*mut Tuple> {
    let n = boxed::Tuple::storage_size(arity);
    //let p = hp.heap_allocate(n, false)?;
    boxed::Tuple::create_into(hp, arity)
  }


  /// Convert any p into *const Tuple + checking the header word to be Tule
  pub unsafe fn from_pointer<T>(p: *const T) -> Hopefully<*const Tuple> {
    let tp = p as *const Tuple;
    if (*tp).header.get_tag() != BoxTypeTag::Tuple {
      return Err(Error::BoxedIsNotATuple)
    }
    Ok(tp)
  }


  /// Convert any p into *mut Tuple + checking the header word to be Tule
  pub unsafe fn from_pointer_mut<T>(p: *mut T) -> Hopefully<*mut Tuple> {
    let tp = p as *mut Tuple;
    if (*tp).header.get_tag() != BoxTypeTag::Tuple {
      return Err(Error::BoxedIsNotATuple)
    }
    Ok(tp)
  }


  pub unsafe fn set_raw_word_base0(this: *mut Tuple, index: Word, val: Word) {
    debug_assert!(index < this.arity());
    let p = this as *mut Word;
    *p.offset(index as isize + 1) = val
  }


  pub unsafe fn get_element_base0(this: *const Tuple, i: Word) -> LTerm {
    let p = this as *const Word;
    *p.offset(i as isize + 1)
  }
}
