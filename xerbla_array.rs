use ::libc;
/* f2c.h  --  Standard Fortran to C header file */
/* *  barf  [ba:rf]  2.  "He suggested using FORTRAN, and everybody barfed."

	- From The Shogakukan DICTIONARY OF NEW ENGLISH (Second edition) */
pub type integer = libc::c_long;
pub type ftnlen = libc::c_long;
/* xerbla_array.f -- translated by f2c (version 20061008).
   You must link the resulting object file with libf2c:
	on Microsoft Windows system, link with libf2c.lib;
	on Linux or Unix systems, link with .../path/to/libf2c.a -lm
	or, if you install libf2c.a in a standard place, with -lf2c -lm
	-- in that order, at the end of the command line, as in
		cc *.o -lf2c -lm
	Source for libf2c is in /netlib/f2c/libf2c.zip, e.g.,

		http://www.netlib.org/f2c/libf2c.zip
*/
/* Subroutine */
#[no_mangle]
pub unsafe extern "C" fn xerbla_array__(mut srname_array__: *mut libc::c_char,
                                        mut srname_len__: *mut integer,
                                        mut info: *mut integer,
                                        mut srname_array_len: ftnlen)
 -> libc::c_int {
    /* System generated locals */
    let mut i__1: integer = 0;
    let mut i__2: integer = 0;
    let mut i__3: integer = 0;
    /* Builtin functions */
    /* Subroutine */
    extern "C" {
        #[link_name = "s_copy"]
        fn s_copy_0(_: *mut libc::c_char, _: *mut libc::c_char, _: ftnlen,
                    _: ftnlen) -> libc::c_int;
    }
    extern "C" {
        #[link_name = "i_len"]
        fn i_len_0(_: *mut libc::c_char, _: ftnlen) -> integer;
    }
    /* Local variables */
    let mut i__: integer = 0;
    extern "C" {
        #[link_name = "xerbla_"]
        fn xerbla__0(_: *mut libc::c_char, _: *mut integer) -> libc::c_int;
    }
    let mut srname: [libc::c_char; 32] = [0; 32];
    /*  -- LAPACK auxiliary routine (version 3.0) -- */
/*     Univ. of Tennessee, Univ. of California Berkeley, NAG Ltd., */
/*     September 19, 2006 */
    /*     .. Scalar Arguments .. */
/*     .. */
/*     .. Array Arguments .. */
/*     .. */
    /*  Purpose */
/*  ======= */
    /*  XERBLA_ARRAY assists other languages in calling XERBLA, the LAPACK */
/*  and BLAS error handler.  Rather than taking a Fortran string argument */
/*  as the function's name, XERBLA_ARRAY takes an array of single */
/*  characters along with the array's length.  XERBLA_ARRAY then copies */
/*  up to 32 characters of that array into a Fortran string and passes */
/*  that to XERBLA.  If called with a non-positive SRNAME_LEN, */
/*  XERBLA_ARRAY will call XERBLA with a string of all blank characters. */
    /*  Say some macro or other device makes XERBLA_ARRAY available to C99 */
/*  by a name lapack_xerbla and with a common Fortran calling convention. */
/*  Then a C99 program could invoke XERBLA via: */
/*     { */
/*       int flen = strlen(__func__); */
/*       lapack_xerbla(__func__, &flen, &info); */
/*     } */
    /*  Providing XERBLA_ARRAY is not necessary for intercepting LAPACK */
/*  errors.  XERBLA_ARRAY calls XERBLA. */
    /*  Arguments */
/*  ========= */
    /*  SRNAME_ARRAY (input) CHARACTER(1) array, dimension (SRNAME_LEN) */
/*          The name of the routine which called XERBLA_ARRAY. */
    /*  SRNAME_LEN (input) INTEGER */
/*          The length of the name in SRNAME_ARRAY. */
    /*  INFO    (input) INTEGER */
/*          The position of the invalid parameter in the parameter list */
/*          of the calling routine. */
    /* ===================================================================== */
    /*     .. */
/*     .. Local Scalars .. */
/*     .. */
/*     .. Local Arrays .. */
/*     .. */
/*     .. Intrinsic Functions .. */
/*     .. */
/*     .. External Functions .. */
/*     .. */
/*     .. Executable Statements .. */
    /* Parameter adjustments */
    srname_array__ = srname_array__.offset(-1);
    /* Function Body */
    s_copy_0(srname.as_mut_ptr(),
             b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
             32 as libc::c_int as ftnlen, 0 as libc::c_int as ftnlen);
    /* Computing MIN */
    i__2 = *srname_len__;
    i__3 = i_len_0(srname.as_mut_ptr(), 32 as libc::c_int as ftnlen);
    i__1 = if i__2 <= i__3 { i__2 } else { i__3 };
    i__ = 1 as libc::c_int as integer;
    while i__ <= i__1 {
        *(&mut *srname.as_mut_ptr().offset((i__ -
                                                1 as libc::c_int as
                                                    libc::c_long) as isize) as
              *mut libc::c_char as *mut libc::c_uchar) =
            *(&mut *srname_array__.offset(i__ as isize) as *mut libc::c_char
                  as *mut libc::c_uchar);
        i__ += 1
    }
    xerbla__0(srname.as_mut_ptr(), info);
    return 0 as libc::c_int;
}
/* xerbla_array__ */
