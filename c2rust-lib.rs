#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(const_raw_ptr_to_usize_cast)]
#![feature(register_tool)]
#![register_tool(c2rust)]



extern crate libc;



pub mod caxpy;
pub mod ccopy;
pub mod cdotc;
pub mod cdotu;
pub mod cgbmv;
pub mod cgemm;
pub mod cgemv;
pub mod cgerc;
pub mod cgeru;
pub mod chbmv;
pub mod chemm;
pub mod chemv;
pub mod cher;
pub mod cher2;
pub mod cher2k;
pub mod cherk;
pub mod chpmv;
pub mod chpr;
pub mod chpr2;
pub mod crotg;
pub mod cscal;
pub mod csrot;
pub mod csscal;
pub mod cswap;
pub mod csymm;
pub mod csyr2k;
pub mod csyrk;
pub mod ctbmv;
pub mod ctbsv;
pub mod ctpmv;
pub mod ctpsv;
pub mod ctrmm;
pub mod ctrmv;
pub mod ctrsm;
pub mod ctrsv;
pub mod dasum;
pub mod daxpy;
pub mod dcabs1;
pub mod dcopy;
pub mod ddot;
pub mod dgbmv;
pub mod dgemm;
pub mod dgemv;
pub mod dger;
pub mod dnrm2;
pub mod drot;
pub mod drotg;
pub mod drotm;
pub mod drotmg;
pub mod dsbmv;
pub mod dscal;
pub mod dsdot;
pub mod dspmv;
pub mod dspr;
pub mod dspr2;
pub mod dswap;
pub mod dsymm;
pub mod dsymv;
pub mod dsyr;
pub mod dsyr2;
pub mod dsyr2k;
pub mod dsyrk;
pub mod dtbmv;
pub mod dtbsv;
pub mod dtpmv;
pub mod dtpsv;
pub mod dtrmm;
pub mod dtrmv;
pub mod dtrsm;
pub mod dtrsv;
pub mod dzasum;
pub mod dznrm2;
pub mod icamax;
pub mod idamax;
pub mod isamax;
pub mod izamax;
pub mod lsame;
pub mod sasum;
pub mod saxpy;
pub mod scabs1;
pub mod scasum;
pub mod scnrm2;
pub mod scopy;
pub mod sdot;
pub mod sdsdot;
pub mod sgbmv;
pub mod sgemm;
pub mod sgemv;
pub mod sger;
pub mod snrm2;
pub mod srot;
pub mod srotg;
pub mod srotm;
pub mod srotmg;
pub mod ssbmv;
pub mod sscal;
pub mod sspmv;
pub mod sspr;
pub mod sspr2;
pub mod sswap;
pub mod ssymm;
pub mod ssymv;
pub mod ssyr;
pub mod ssyr2;
pub mod ssyr2k;
pub mod ssyrk;
pub mod stbmv;
pub mod stbsv;
pub mod stpmv;
pub mod stpsv;
pub mod strmm;
pub mod strmv;
pub mod strsm;
pub mod strsv;
pub mod xerbla;
pub mod xerbla_array;
pub mod zaxpy;
pub mod zcopy;
pub mod zdotc;
pub mod zdotu;
pub mod zdrot;
pub mod zdscal;
pub mod zgbmv;
pub mod zgemm;
pub mod zgemv;
pub mod zgerc;
pub mod zgeru;
pub mod zhbmv;
pub mod zhemm;
pub mod zhemv;
pub mod zher;
pub mod zher2;
pub mod zher2k;
pub mod zherk;
pub mod zhpmv;
pub mod zhpr;
pub mod zhpr2;
pub mod zrotg;
pub mod zscal;
pub mod zswap;
pub mod zsymm;
pub mod zsyr2k;
pub mod zsyrk;
pub mod ztbmv;
pub mod ztbsv;
pub mod ztpmv;
pub mod ztpsv;
pub mod ztrmm;
pub mod ztrmv;
pub mod ztrsm;
pub mod ztrsv;

