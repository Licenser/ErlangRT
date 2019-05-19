ErlangRT - Runtime
==================

Erlang Replacement Therapy.
This is an attempt to make Erlang runtime (BEAM emulator) in Rust. This is not
the first attempt so I kinda know what I'm doing.

Progress to the Proof of Concept
--------------------------------

* Term library 80% (remaining 80% are in progress)
* External Term Format (decoder 70%, encoder 0%)
* BEAM Loader - mostly done
* VM and processes 40%
* VM loop and opcodes 45% (74 of 168)
* Some basic BIFs <15%
* Binaries, sub-binaries, binary heap, binary opcodes: <20%

.. figure:: http://imgur.com/H5qypZG.png
   :scale: 50%
   :alt: Trying to run ``init:boot/1``

Tests in ``priv/test2.erl`` work. Running ``make test`` tries to run ``init:boot/1`` and produces the output above.

Compiling
`````````

* The source assumes that you have Erlang OTP 22+ source in a Git submodule in ``otp/``,
  and the stdlib and preload BEAM files are compiled and ready. Makefile takes care of it.
* Install latest **Rust** and **Cargo** via `Rustup <http://doc.crates.io/>`_
* Run ``make`` and with the magic of Bash autocomplete see which targets it
  supports. You might like:
    * ``make run`` - runs the executable with test args, whatever set by the developer,
      do not expect it to show any magical tricks;
    * ``make doc`` - builds doc pages in ``target/doc/erlang_rt/``
    * ``make test`` - runs the tests
    * ``make build`` and ``make build-rel`` - builds but does not run the debug and
      the release target respectively
      
Currently the emulator expects to have preloaded BEAM modules from OTP 22+ located in `otp/`
Git submodule (Makefile takes care of it).

Editing and Code Navigation
```````````````````````````

I am using and strongly recommend IntelliJ IDEA CE (free version) with
IntelliJ-Rust plugin (available in repositories tab inside IntelliJ).

Reference Material
``````````````````

* `BEAM Wisdoms <http://beam-wisdoms.clau.se/>`_ (I run this one)
* `The BEAM book <https://github.com/happi/theBeamBook>`_
  (I am also one of the editors there)

Contributing
````````````

See ``CONTRIBUTING.rst``
