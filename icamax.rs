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
/* icamax.f -- translated by f2c (version 20061008).
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
pub unsafe extern "C" fn f2c_icamax(
    mut n: *mut integer,
    mut cx: *mut complex,
    mut incx: *mut integer,
) -> integer {
    /* System generated locals */
    let mut ret_val: integer = 0;
    let mut i__1: integer = 0;
    /* Local variables */
    let mut i__: integer = 0;
    let mut ix: integer = 0;
    let mut smax: real = 0.;
    extern "C" {
        #[link_name = "scabs1_"]
        fn scabs1__0(_: *mut complex) -> doublereal;
    }
    /*     .. Scalar Arguments .. */
    /*     .. */
    /*     .. Array Arguments .. */
    /*     .. */
    /*  Purpose */
    /*  ======= */
    /*     finds the index of element having max. absolute value. */
    /*     jack dongarra, linpack, 3/11/78. */
    /*     modified 3/93 to return if incx .le. 0. */
    /*     modified 12/3/93, array(1) declarations changed to array(*) */
    /*     .. Local Scalars .. */
    /*     .. */
    /*     .. External Functions .. */
    /*     .. */
    /* Parameter adjustments */
    cx = cx.offset(-1);
    /* Function Body */
    ret_val = 0 as libc::c_int as integer;
    if *n < 1 as libc::c_int as libc::c_long || *incx <= 0 as libc::c_int as libc::c_long {
        return ret_val;
    }
    ret_val = 1 as libc::c_int as integer;
    if *n == 1 as libc::c_int as libc::c_long {
        return ret_val;
    }
    if *incx == 1 as libc::c_int as libc::c_long {
        /*        code for increment equal to 1 */
        smax = scabs1__0(&mut *cx.offset(1 as libc::c_int as isize)) as real;
        i__1 = *n;
        i__ = 2 as libc::c_int as integer;
        while i__ <= i__1 {
            if !(scabs1__0(&mut *cx.offset(i__ as isize)) <= smax as libc::c_double) {
                ret_val = i__;
                smax = scabs1__0(&mut *cx.offset(i__ as isize)) as real
            }
            i__ += 1
        }
        return ret_val;
    } else {
        /*        code for increment not equal to 1 */
        ix = 1 as libc::c_int as integer;
        smax = scabs1__0(&mut *cx.offset(1 as libc::c_int as isize)) as real;
        ix += *incx;
        i__1 = *n;
        i__ = 2 as libc::c_int as integer;
        while i__ <= i__1 {
            if !(scabs1__0(&mut *cx.offset(ix as isize)) <= smax as libc::c_double) {
                ret_val = i__;
                smax = scabs1__0(&mut *cx.offset(ix as isize)) as real
            }
            ix += *incx;
            i__ += 1
            /* L10: */
        }
        return ret_val;
    };
}
/* icamax_ */
