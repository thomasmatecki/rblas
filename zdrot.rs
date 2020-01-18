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
/* zdrot.f -- translated by f2c (version 20061008).
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
pub unsafe extern "C" fn zdrot_(
    mut n: *mut integer,
    mut cx: *mut doublecomplex,
    mut incx: *mut integer,
    mut cy: *mut doublecomplex,
    mut incy: *mut integer,
    mut c__: *mut doublereal,
    mut s: *mut doublereal,
) -> libc::c_int {
    /* System generated locals */
    let mut i__1: integer = 0;
    let mut i__2: integer = 0;
    let mut i__3: integer = 0;
    let mut i__4: integer = 0;
    let mut z__1: doublecomplex = doublecomplex { r: 0., i: 0. };
    let mut z__2: doublecomplex = doublecomplex { r: 0., i: 0. };
    let mut z__3: doublecomplex = doublecomplex { r: 0., i: 0. };
    /* Local variables */
    let mut i__: integer = 0;
    let mut ix: integer = 0;
    let mut iy: integer = 0;
    let mut ctemp: doublecomplex = doublecomplex { r: 0., i: 0. };
    /*     .. Scalar Arguments .. */
    /*     .. */
    /*     .. Array Arguments .. */
    /*     .. */
    /*  Purpose */
    /*  ======= */
    /*  Applies a plane rotation, where the cos and sin (c and s) are real */
    /*  and the vectors cx and cy are complex. */
    /*  jack dongarra, linpack, 3/11/78. */
    /*  Arguments */
    /*  ========== */
    /*  N        (input) INTEGER */
    /*           On entry, N specifies the order of the vectors cx and cy. */
    /*           N must be at least zero. */
    /*           Unchanged on exit. */
    /*  CX       (input) COMPLEX*16 array, dimension at least */
    /*           ( 1 + ( N - 1 )*abs( INCX ) ). */
    /*           Before entry, the incremented array CX must contain the n */
    /*           element vector cx. On exit, CX is overwritten by the updated */
    /*           vector cx. */
    /*  INCX     (input) INTEGER */
    /*           On entry, INCX specifies the increment for the elements of */
    /*           CX. INCX must not be zero. */
    /*           Unchanged on exit. */
    /*  CY       (input) COMPLEX*16 array, dimension at least */
    /*           ( 1 + ( N - 1 )*abs( INCY ) ). */
    /*           Before entry, the incremented array CY must contain the n */
    /*           element vector cy. On exit, CY is overwritten by the updated */
    /*           vector cy. */
    /*  INCY     (input) INTEGER */
    /*           On entry, INCY specifies the increment for the elements of */
    /*           CY. INCY must not be zero. */
    /*           Unchanged on exit. */
    /*  C        (input) DOUBLE PRECISION */
    /*           On entry, C specifies the cosine, cos. */
    /*           Unchanged on exit. */
    /*  S        (input) DOUBLE PRECISION */
    /*           On entry, S specifies the sine, sin. */
    /*           Unchanged on exit. */
    /* ===================================================================== */
    /*     .. Local Scalars .. */
    /*     .. */
    /*     .. Executable Statements .. */
    /* Parameter adjustments */
    cy = cy.offset(-1);
    cx = cx.offset(-1);
    /* Function Body */
    if *n <= 0 as libc::c_int as libc::c_long {
        return 0 as libc::c_int;
    }
    if *incx == 1 as libc::c_int as libc::c_long && *incy == 1 as libc::c_int as libc::c_long {
        /*        code for both increments equal to 1 */
        i__1 = *n;
        i__ = 1 as libc::c_int as integer;
        while i__ <= i__1 {
            i__2 = i__;
            z__2.r = *c__ * (*cx.offset(i__2 as isize)).r;
            z__2.i = *c__ * (*cx.offset(i__2 as isize)).i;
            i__3 = i__;
            z__3.r = *s * (*cy.offset(i__3 as isize)).r;
            z__3.i = *s * (*cy.offset(i__3 as isize)).i;
            z__1.r = z__2.r + z__3.r;
            z__1.i = z__2.i + z__3.i;
            ctemp.r = z__1.r;
            ctemp.i = z__1.i;
            i__2 = i__;
            i__3 = i__;
            z__2.r = *c__ * (*cy.offset(i__3 as isize)).r;
            z__2.i = *c__ * (*cy.offset(i__3 as isize)).i;
            i__4 = i__;
            z__3.r = *s * (*cx.offset(i__4 as isize)).r;
            z__3.i = *s * (*cx.offset(i__4 as isize)).i;
            z__1.r = z__2.r - z__3.r;
            z__1.i = z__2.i - z__3.i;
            (*cy.offset(i__2 as isize)).r = z__1.r;
            (*cy.offset(i__2 as isize)).i = z__1.i;
            i__2 = i__;
            (*cx.offset(i__2 as isize)).r = ctemp.r;
            (*cx.offset(i__2 as isize)).i = ctemp.i;
            i__ += 1
            /* L30: */
        }
        return 0 as libc::c_int;
    } else {
        /*        code for unequal increments or equal increments not equal */
        /*          to 1 */
        ix = 1 as libc::c_int as integer;
        iy = 1 as libc::c_int as integer;
        if *incx < 0 as libc::c_int as libc::c_long {
            ix = (-*n + 1 as libc::c_int as libc::c_long) * *incx + 1 as libc::c_int as libc::c_long
        }
        if *incy < 0 as libc::c_int as libc::c_long {
            iy = (-*n + 1 as libc::c_int as libc::c_long) * *incy + 1 as libc::c_int as libc::c_long
        }
        i__1 = *n;
        i__ = 1 as libc::c_int as integer;
        while i__ <= i__1 {
            i__2 = ix;
            z__2.r = *c__ * (*cx.offset(i__2 as isize)).r;
            z__2.i = *c__ * (*cx.offset(i__2 as isize)).i;
            i__3 = iy;
            z__3.r = *s * (*cy.offset(i__3 as isize)).r;
            z__3.i = *s * (*cy.offset(i__3 as isize)).i;
            z__1.r = z__2.r + z__3.r;
            z__1.i = z__2.i + z__3.i;
            ctemp.r = z__1.r;
            ctemp.i = z__1.i;
            i__2 = iy;
            i__3 = iy;
            z__2.r = *c__ * (*cy.offset(i__3 as isize)).r;
            z__2.i = *c__ * (*cy.offset(i__3 as isize)).i;
            i__4 = ix;
            z__3.r = *s * (*cx.offset(i__4 as isize)).r;
            z__3.i = *s * (*cx.offset(i__4 as isize)).i;
            z__1.r = z__2.r - z__3.r;
            z__1.i = z__2.i - z__3.i;
            (*cy.offset(i__2 as isize)).r = z__1.r;
            (*cy.offset(i__2 as isize)).i = z__1.i;
            i__2 = ix;
            (*cx.offset(i__2 as isize)).r = ctemp.r;
            (*cx.offset(i__2 as isize)).i = ctemp.i;
            ix += *incx;
            iy += *incy;
            i__ += 1
            /* L10: */
        }
        return 0 as libc::c_int;
    };
}
/* zdrot_ */
