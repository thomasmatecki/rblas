use libc;
/* f2c.h  --  Standard Fortran to C header file */
/* *  barf  [ba:rf]  2.  "He suggested using FORTRAN, and everybody barfed."

- From The Shogakukan DICTIONARY OF NEW ENGLISH (Second edition) */
pub type integer = libc::c_long;
pub type real = libc::c_float;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct complex {
    pub r: real,
    pub i: real,
}
/* csscal.f -- translated by f2c (version 20061008).
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
pub unsafe extern "C" fn f2c_csscal(
    mut n: *mut integer,
    mut sa: *mut real,
    mut cx: *mut complex,
    mut incx: *mut integer,
) -> libc::c_int {
    /* System generated locals */
    let mut i__1: integer = 0;
    let mut i__2: integer = 0;
    let mut i__3: integer = 0;
    let mut i__4: integer = 0;
    let mut r__1: real = 0.;
    let mut r__2: real = 0.;
    let mut q__1: complex = complex { r: 0., i: 0. };
    /* Builtin functions */
    extern "C" {
        #[link_name = "r_imag"]
        fn r_imag_0(_: *mut complex) -> libc::c_double;
    }
    /* Local variables */
    let mut i__: integer = 0;
    let mut nincx: integer = 0;
    /*     .. Scalar Arguments .. */
    /*     .. */
    /*     .. Array Arguments .. */
    /*     .. */
    /*  Purpose */
    /*  ======= */
    /*     scales a complex vector by a real constant. */
    /*     jack dongarra, linpack, 3/11/78. */
    /*     modified 3/93 to return if incx .le. 0. */
    /*     modified 12/3/93, array(1) declarations changed to array(*) */
    /*     .. Local Scalars .. */
    /*     .. */
    /*     .. Intrinsic Functions .. */
    /*     .. */
    /* Parameter adjustments */
    cx = cx.offset(-1);
    /* Function Body */
    if *n <= 0 as libc::c_int as libc::c_long || *incx <= 0 as libc::c_int as libc::c_long {
        return 0 as libc::c_int;
    }
    if *incx == 1 as libc::c_int as libc::c_long {
        /*        code for increment equal to 1 */
        i__2 = *n;
        i__ = 1 as libc::c_int as integer;
        while i__ <= i__2 {
            i__1 = i__;
            i__3 = i__;
            r__1 = *sa * (*cx.offset(i__3 as isize)).r;
            r__2 = (*sa as libc::c_double * r_imag_0(&mut *cx.offset(i__ as isize))) as real;
            q__1.r = r__1;
            q__1.i = r__2;
            (*cx.offset(i__1 as isize)).r = q__1.r;
            (*cx.offset(i__1 as isize)).i = q__1.i;
            i__ += 1
            /* L30: */
        }
        return 0 as libc::c_int;
    } else {
        /*        code for increment not equal to 1 */
        nincx = *n * *incx;
        i__1 = nincx;
        i__2 = *incx;
        i__ = 1 as libc::c_int as integer;
        while if i__2 < 0 as libc::c_int as libc::c_long {
            (i__ >= i__1) as libc::c_int
        } else {
            (i__ <= i__1) as libc::c_int
        } != 0
        {
            i__3 = i__;
            i__4 = i__;
            r__1 = *sa * (*cx.offset(i__4 as isize)).r;
            r__2 = (*sa as libc::c_double * r_imag_0(&mut *cx.offset(i__ as isize))) as real;
            q__1.r = r__1;
            q__1.i = r__2;
            (*cx.offset(i__3 as isize)).r = q__1.r;
            (*cx.offset(i__3 as isize)).i = q__1.i;
            i__ += i__2
            /* L10: */
        }
        return 0 as libc::c_int;
    };
}
/* csscal_ */
