use libc;
/* f2c.h  --  Standard Fortran to C header file */
/* *  barf  [ba:rf]  2.  "He suggested using FORTRAN, and everybody barfed."

- From The Shogakukan DICTIONARY OF NEW ENGLISH (Second edition) */
pub type integer = libc::c_long;
pub type real = libc::c_float;
pub type doublereal = libc::c_double;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct complex {
    pub r: real,
    pub i: real,
}
/* scasum.f -- translated by f2c (version 20061008).
   You must link the resulting object file with libf2c:
    on Microsoft Windows system, link with libf2c.lib;
    on Linux or Unix systems, link with .../path/to/libf2c.a -lm
    or, if you install libf2c.a in a standard place, with -lf2c -lm
    -- in that order, at the end of the command line, as in
        cc *.o -lf2c -lm
    Source for libf2c is in /netlib/f2c/libf2c.zip, e.g.,

        http://www.netlib.org/f2c/libf2c.zip
*/
#[no_mangle]
pub unsafe extern "C" fn f2c_scasum(
    mut n: *mut integer,
    mut cx: *mut complex,
    mut incx: *mut integer,
) -> doublereal {
    /* System generated locals */
    let mut i__1: integer = 0;
    let mut i__2: integer = 0;
    let mut i__3: integer = 0;
    let mut ret_val: real = 0.;
    let mut r__1: real = 0.;
    let mut r__2: real = 0.;
    /* Builtin functions */
    extern "C" {
        #[link_name = "r_imag"]
        fn r_imag_0(_: *mut complex) -> libc::c_double;
    }
    /* Local variables */
    let mut i__: integer = 0;
    let mut nincx: integer = 0;
    let mut stemp: real = 0.;
    /*     .. Scalar Arguments .. */
    /*     .. */
    /*     .. Array Arguments .. */
    /*     .. */
    /*  Purpose */
    /*  ======= */
    /*     takes the sum of the absolute values of a complex vector and */
    /*     returns a single precision result. */
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
    ret_val = 0.0f32;
    stemp = 0.0f32;
    if *n <= 0 as libc::c_int as libc::c_long || *incx <= 0 as libc::c_int as libc::c_long {
        return ret_val as doublereal;
    }
    if *incx == 1 as libc::c_int as libc::c_long {
        /*        code for increment equal to 1 */
        i__2 = *n;
        i__ = 1 as libc::c_int as integer;
        while i__ <= i__2 {
            i__1 = i__;
            r__1 = (*cx.offset(i__1 as isize)).r;
            r__2 = r_imag_0(&mut *cx.offset(i__ as isize)) as real;
            stemp = (stemp as libc::c_double
                + (if r__1 >= 0 as libc::c_int as libc::c_float {
                    r__1
                } else {
                    -r__1
                }) as doublereal
                + (if r__2 >= 0 as libc::c_int as libc::c_float {
                    r__2
                } else {
                    -r__2
                }) as doublereal) as real;
            i__ += 1
            /* L30: */
        }
        ret_val = stemp;
        return ret_val as doublereal;
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
            r__1 = (*cx.offset(i__3 as isize)).r;
            r__2 = r_imag_0(&mut *cx.offset(i__ as isize)) as real;
            stemp = (stemp as libc::c_double
                + (if r__1 >= 0 as libc::c_int as libc::c_float {
                    r__1
                } else {
                    -r__1
                }) as doublereal
                + (if r__2 >= 0 as libc::c_int as libc::c_float {
                    r__2
                } else {
                    -r__2
                }) as doublereal) as real;
            i__ += i__2
            /* L10: */
        }
        ret_val = stemp;
        return ret_val as doublereal;
    };
}
/* scasum_ */
