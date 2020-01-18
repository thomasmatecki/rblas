use libc;
/* f2c.h  --  Standard Fortran to C header file */
/* *  barf  [ba:rf]  2.  "He suggested using FORTRAN, and everybody barfed."

- From The Shogakukan DICTIONARY OF NEW ENGLISH (Second edition) */
pub type integer = libc::c_long;
pub type doublereal = libc::c_double;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct doublecomplex {
    pub r: doublereal,
    pub i: doublereal,
}
/* dzasum.f -- translated by f2c (version 20061008).
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
pub unsafe extern "C" fn f2c_dzasum(
    mut n: *mut integer,
    mut zx: *mut doublecomplex,
    mut incx: *mut integer,
) -> doublereal {
    /* System generated locals */
    let mut i__1: integer = 0;
    let mut ret_val: doublereal = 0.;
    /* Local variables */
    let mut i__: integer = 0;
    let mut ix: integer = 0;
    let mut stemp: doublereal = 0.;
    extern "C" {
        #[link_name = "dcabs1_"]
        fn dcabs1__0(_: *mut doublecomplex) -> doublereal;
    }
    /*     .. Scalar Arguments .. */
    /*     .. */
    /*     .. Array Arguments .. */
    /*     .. */
    /*  Purpose */
    /*  ======= */
    /*     takes the sum of the absolute values. */
    /*     jack dongarra, 3/11/78. */
    /*     modified 3/93 to return if incx .le. 0. */
    /*     modified 12/3/93, array(1) declarations changed to array(*) */
    /*     .. Local Scalars .. */
    /*     .. */
    /*     .. External Functions .. */
    /*     .. */
    /* Parameter adjustments */
    zx = zx.offset(-1);
    /* Function Body */
    ret_val = 0.0f64;
    stemp = 0.0f64;
    if *n <= 0 as libc::c_int as libc::c_long || *incx <= 0 as libc::c_int as libc::c_long {
        return ret_val;
    }
    if *incx == 1 as libc::c_int as libc::c_long {
        /*        code for increment equal to 1 */
        i__1 = *n;
        i__ = 1 as libc::c_int as integer;
        while i__ <= i__1 {
            stemp += dcabs1__0(&mut *zx.offset(i__ as isize));
            i__ += 1
            /* L30: */
        }
        ret_val = stemp;
        return ret_val;
    } else {
        /*        code for increment not equal to 1 */
        ix = 1 as libc::c_int as integer;
        i__1 = *n;
        i__ = 1 as libc::c_int as integer;
        while i__ <= i__1 {
            stemp += dcabs1__0(&mut *zx.offset(ix as isize));
            ix += *incx;
            i__ += 1
            /* L10: */
        }
        ret_val = stemp;
        return ret_val;
    };
}
/* dzasum_ */
