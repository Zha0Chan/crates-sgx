initSidebarItems({"fn":[["rsgx_cpuid","The rsgx_cpuid function performs the equivalent of a cpuid() function call or intrinisic which executes the CPUID instruction to query the host processor for the information about supported features."],["rsgx_cpuidex","The rsgx_cpuidex function performs the equivalent of a cpuid_ex() function call or intrinisic which executes the CPUID instruction to query the host processor for the information about supported features."]],"macro":[["__cfg_if_apply",""],["__cfg_if_items",""],["__thread_local_inner",""],["assert_eq","Asserts that two expressions are equal to each other (using [`PartialEq`])."],["assert_ne","Asserts that two expressions are not equal to each other (using [`PartialEq`])."],["cfg_if",""],["dbg",""],["debug_assert","Asserts that a boolean expression is `true` at runtime."],["debug_assert_eq","Asserts that two expressions are equal to each other."],["debug_assert_ne","Asserts that two expressions are not equal to each other."],["eprint","Prints to the standard error."],["eprintln","Prints to the standard error, with a newline."],["format","Creates a `String` using interpolation of runtime expressions."],["global_ctors_object","global_ctors_object is the base macro of implementing constructors."],["global_dtors_object",""],["is_cpu_feature_supported",""],["is_x86_feature_detected",""],["panic","Panics the current thread."],["print","Prints to the standard output."],["println","Prints to the standard output, with a newline."],["thread_local","Declare a new thread local storage key of type [`std::thread::LocalKey`]."],["todo","Indicates unfinished code."],["try","Unwraps a result or propagates its error."],["unimplemented","Indicates unimplemented code by panicking with a message of \"not implemented\"."],["unreachable","Indicates unreachable code."],["vec","Creates a [`Vec`] containing the arguments."],["write","Writes formatted data into a buffer."],["writeln","Write formatted data into a buffer, with a newline appended."]],"mod":[["alloc","Memory allocation APIs"],["any","This module implements the `Any` trait, which enables dynamic typing of any `'static` type through runtime reflection."],["array","Implementations of things like `Eq` for fixed-length arrays up to a certain length. Eventually, we should be able to generalize to all lengths."],["ascii",""],["borrow","A module for working with borrowed data."],["boxed","A pointer type for heap allocation."],["cell","Shareable mutable containers."],["char","A character type."],["clone","The `Clone` trait for types that cannot be 'implicitly copied'."],["cmp","Functionality for ordering and comparison."],["collections","Collection types."],["convert","Traits for conversions between types."],["debug",""],["default","The `Default` trait for types which may have meaningful default values."],["enclave",""],["env","Inspection and manipulation of the process's environment."],["error",""],["f32","This module provides constants which are specific to the implementation of the `f32` floating point data type."],["f64","This module provides constants which are specific to the implementation of the `f64` floating point data type."],["ffi","Utilities related to FFI bindings."],["fmt","Utilities for formatting and printing `String`s."],["future","Asynchronous values."],["hash","Generic hashing support."],["hint","Hints to compiler that affects how code should be emitted or optimized."],["i128","The 128-bit signed integer type."],["i16","The 16-bit signed integer type."],["i32","The 32-bit signed integer type."],["i64","The 64-bit signed integer type."],["i8","The 8-bit signed integer type."],["intrinsics","Compiler intrinsics."],["io",""],["isize","The pointer-sized signed integer type."],["iter","Composable external iteration."],["marker","Primitive traits and types representing basic properties of types."],["mem","Basic functions for dealing with memory."],["net","Networking primitives for TCP/UDP communication."],["num","Additional functionality for numerics."],["ops","Overloadable operators."],["option","Optional values."],["os",""],["panic","Panic support in the standard library"],["path","Cross-platform path manipulation."],["pin","Types that pin data to its location in memory."],["prelude",""],["ptr","Manually manage memory through raw pointers."],["raw","Contains struct definitions for the layout of compiler built-in types."],["rc","Single-threaded reference-counting pointers. 'Rc' stands for 'Reference Counted'."],["result","Error handling with the `Result` type."],["rt","Runtime services"],["sgxfs","Filesystem manipulation operations."],["slice","A dynamically-sized view into a contiguous sequence, `[T]`."],["str","Unicode string slices."],["string","A UTF-8 encoded, growable string."],["sync","The Intel(R) Software Guard Extensions SDK already supports mutex and conditional variable synchronization mechanisms by means of the following API and data types defined in the Types and Enumerations section. Some functions included in the trusted Thread Synchronization library may make calls outside the enclave (OCALLs). If you use any of the APIs below, you must first import the needed OCALL functions from sgx_tstd.edl. Otherwise, you will get a linker error when the enclave is being built; see Calling Functions outside the Enclave for additional details. The table below illustrates the primitives that the Intel(R) SGX Thread Synchronization library supports, as well as the OCALLs that each API function needs."],["task","Types and Traits for working with asynchronous tasks."],["thread","Native threads."],["time","Temporal quantification."],["u128","The 128-bit unsigned integer type."],["u16","The 16-bit unsigned integer type."],["u32","The 32-bit unsigned integer type."],["u64","The 64-bit unsigned integer type."],["u8","The 8-bit unsigned integer type."],["untrusted",""],["usize","The pointer-sized unsigned integer type."],["vec","A contiguous growable array type with heap-allocated contents, written `Vec<T>`."]]});