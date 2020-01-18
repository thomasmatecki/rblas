use libc;
/* f2c.h  --  Standard Fortran to C header file */
/* *  barf  [ba:rf]  2.  "He suggested using FORTRAN, and everybody barfed."

- From The Shogakukan DICTIONARY OF NEW ENGLISH (Second edition) */
pub type integer = libc::c_long;
pub type doublereal = libc::c_double;
/* drotm.f -- translated by f2c (version 20061008).
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
pub unsafe extern "C" fn f2c_drotm(
    mut n: *mut integer,
    mut dx: *mut doublereal,
    mut incx: *mut integer,
    mut dy: *mut doublereal,
    mut incy: *mut integer,
    mut dparam: *mut doublereal,
) -> libc::c_int {
    /* Initialized data */
    static mut zero: doublereal = 0.0f64;
    static mut two: doublereal = 2.0f64;
    /* System generated locals */
    let mut i__1: integer = 0;
    let mut i__2: integer = 0;
    /* Local variables */
    let mut i__: integer = 0;
    let mut w: doublereal = 0.;
    let mut z__: doublereal = 0.;
    let mut kx: integer = 0;
    let mut ky: integer = 0;
    let mut dh11: doublereal = 0.;
    let mut dh12: doublereal = 0.;
    let mut dh21: doublereal = 0.;
    let mut dh22: doublereal = 0.;
    let mut dflag: doublereal = 0.;
    let mut nsteps: integer = 0;
    /*     .. Scalar Arguments .. */
    /*     .. */
    /*     .. Array Arguments .. */
    /*     .. */
    /*  Purpose */
    /*  ======= */
    /*     APPLY THE MODIFIED GIVENS TRANSFORMATION, H, TO THE 2 BY N MATRIX */
    /*     (DX**T) , WHERE **T INDICATES TRANSPOSE. THE ELEMENTS OF DX ARE IN */
    /*     (DY**T) */
    /*     DX(LX+I*INCX), I = 0 TO N-1, WHERE LX = 1 IF INCX .GE. 0, ELSE */
    /*     LX = (-INCX)*N, AND SIMILARLY FOR SY USING LY AND INCY. */
    /*     WITH DPARAM(1)=DFLAG, H HAS ONE OF THE FOLLOWING FORMS.. */
    /*     DFLAG=-1.D0     DFLAG=0.D0        DFLAG=1.D0     DFLAG=-2.D0 */
    /*       (DH11  DH12)    (1.D0  DH12)    (DH11  1.D0)    (1.D0  0.D0) */
    /*     H=(          )    (          )    (          )    (          ) */
    /*       (DH21  DH22),   (DH21  1.D0),   (-1.D0 DH22),   (0.D0  1.D0). */
    /*     SEE DROTMG FOR A DESCRIPTION OF DATA STORAGE IN DPARAM. */
    /*  Arguments */
    /*  ========= */
    /*  N      (input) INTEGER */
    /*         number of elements in input vector(s) */
    /*  DX     (input/output) DOUBLE PRECISION array, dimension N */
    /*         double precision vector with N elements */
    /*  INCX   (input) INTEGER */
    /*         storage spacing between elements of DX */
    /*  DY     (input/output) DOUBLE PRECISION array, dimension N */
    /*         double precision vector with N elements */
    /*  INCY   (input) INTEGER */
    /*         storage spacing between elements of DY */
    /*  DPARAM (input/output)  DOUBLE PRECISION array, dimension 5 */
    /*     DPARAM(1)=DFLAG */
    /*     DPARAM(2)=DH11 */
    /*     DPARAM(3)=DH21 */
    /*     DPARAM(4)=DH12 */
    /*     DPARAM(5)=DH22 */
    /*  ===================================================================== */
    /*     .. Local Scalars .. */
    /*     .. */
    /*     .. Data statements .. */
    /* Parameter adjustments */
    dparam = dparam.offset(-1);
    dy = dy.offset(-1);
    dx = dx.offset(-1);
    /* Function Body */
    /*     .. */
    dflag = *dparam.offset(1 as libc::c_int as isize);
    if !(*n <= 0 as libc::c_int as libc::c_long || dflag + two == zero) {
        if !(*incx == *incy && *incx > 0 as libc::c_int as libc::c_long) {
            kx = 1 as libc::c_int as integer;
            ky = 1 as libc::c_int as integer;
            if *incx < 0 as libc::c_int as libc::c_long {
                kx = (1 as libc::c_int as libc::c_long - *n) * *incx
                    + 1 as libc::c_int as libc::c_long
            }
            if *incy < 0 as libc::c_int as libc::c_long {
                ky = (1 as libc::c_int as libc::c_long - *n) * *incy
                    + 1 as libc::c_int as libc::c_long
            }
            if dflag < 0.0f64 {
                dh11 = *dparam.offset(2 as libc::c_int as isize);
                dh12 = *dparam.offset(4 as libc::c_int as isize);
                dh21 = *dparam.offset(3 as libc::c_int as isize);
                dh22 = *dparam.offset(5 as libc::c_int as isize);
                i__2 = *n;
                i__ = 1 as libc::c_int as integer;
                while i__ <= i__2 {
                    w = *dx.offset(kx as isize);
                    z__ = *dy.offset(ky as isize);
                    *dx.offset(kx as isize) = w * dh11 + z__ * dh12;
                    *dy.offset(ky as isize) = w * dh21 + z__ * dh22;
                    kx += *incx;
                    ky += *incy;
                    i__ += 1
                    /* L130: */
                }
            } else if dflag == 0 as libc::c_int as libc::c_double {
                dh12 = *dparam.offset(4 as libc::c_int as isize);
                dh21 = *dparam.offset(3 as libc::c_int as isize);
                i__2 = *n;
                i__ = 1 as libc::c_int as integer;
                while i__ <= i__2 {
                    w = *dx.offset(kx as isize);
                    z__ = *dy.offset(ky as isize);
                    *dx.offset(kx as isize) = w + z__ * dh12;
                    *dy.offset(ky as isize) = w * dh21 + z__;
                    kx += *incx;
                    ky += *incy;
                    i__ += 1
                    /* L90: */
                }
            } else {
                dh11 = *dparam.offset(2 as libc::c_int as isize);
                dh22 = *dparam.offset(5 as libc::c_int as isize);
                i__2 = *n;
                i__ = 1 as libc::c_int as integer;
                while i__ <= i__2 {
                    w = *dx.offset(kx as isize);
                    z__ = *dy.offset(ky as isize);
                    *dx.offset(kx as isize) = w * dh11 + z__;
                    *dy.offset(ky as isize) = -w + dh22 * z__;
                    kx += *incx;
                    ky += *incy;
                    i__ += 1
                    /* L110: */
                }
            }
        } else {
            nsteps = *n * *incx;
            if dflag < 0.0f64 {
                dh11 = *dparam.offset(2 as libc::c_int as isize);
                dh12 = *dparam.offset(4 as libc::c_int as isize);
                dh21 = *dparam.offset(3 as libc::c_int as isize);
                dh22 = *dparam.offset(5 as libc::c_int as isize);
                i__1 = nsteps;
                i__2 = *incx;
                i__ = 1 as libc::c_int as integer;
                while if i__2 < 0 as libc::c_int as libc::c_long {
                    (i__ >= i__1) as libc::c_int
                } else {
                    (i__ <= i__1) as libc::c_int
                } != 0
                {
                    w = *dx.offset(i__ as isize);
                    z__ = *dy.offset(i__ as isize);
                    *dx.offset(i__ as isize) = w * dh11 + z__ * dh12;
                    *dy.offset(i__ as isize) = w * dh21 + z__ * dh22;
                    i__ += i__2
                    /* L60: */
                }
            } else if dflag == 0 as libc::c_int as libc::c_double {
                dh12 = *dparam.offset(4 as libc::c_int as isize);
                dh21 = *dparam.offset(3 as libc::c_int as isize);
                i__1 = nsteps;
                i__2 = *incx;
                i__ = 1 as libc::c_int as integer;
                while if i__2 < 0 as libc::c_int as libc::c_long {
                    (i__ >= i__1) as libc::c_int
                } else {
                    (i__ <= i__1) as libc::c_int
                } != 0
                {
                    w = *dx.offset(i__ as isize);
                    z__ = *dy.offset(i__ as isize);
                    *dx.offset(i__ as isize) = w + z__ * dh12;
                    *dy.offset(i__ as isize) = w * dh21 + z__;
                    i__ += i__2
                    /* L20: */
                }
            } else {
                dh11 = *dparam.offset(2 as libc::c_int as isize);
                dh22 = *dparam.offset(5 as libc::c_int as isize);
                i__2 = nsteps;
                i__1 = *incx;
                i__ = 1 as libc::c_int as integer;
                while if i__1 < 0 as libc::c_int as libc::c_long {
                    (i__ >= i__2) as libc::c_int
                } else {
                    (i__ <= i__2) as libc::c_int
                } != 0
                {
                    w = *dx.offset(i__ as isize);
                    z__ = *dy.offset(i__ as isize);
                    *dx.offset(i__ as isize) = w * dh11 + z__;
                    *dy.offset(i__ as isize) = -w + dh22 * z__;
                    i__ += i__1
                    /* L40: */
                }
            }
        }
    }
    return 0 as libc::c_int;
}
/* drotm_ */
