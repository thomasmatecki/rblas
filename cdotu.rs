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
/* cdotu.f -- translated by f2c (version 20061008).
   You must link the resulting object file with libf2c:
    on Microsoft Windows system, link with libf2c.lib;
    on Linux or Unix systems, link with .../path/to/libf2c.a -lm
    or, if you install libf2c.a in a standard place, with -lf2c -lm
    -- in that order, at the end of the command line, as in
        cc *.o -lf2c -lm
    Source for libf2c is in /netlib/f2c/libf2c.zip, e.g.,

        http://www.netlib.org/f2c/libf2c.zip
*/
/* Complex */
#[no_mangle]
pub unsafe extern "C" fn f2c_cdotu(
    mut ret_val: *mut complex,
    mut n: *mut integer,
    mut cx: *mut complex,
    mut incx: *mut integer,
    mut cy: *mut complex,
    mut incy: *mut integer,
) {
    /* System generated locals */
    let mut i__1: integer = 0;
    let mut i__2: integer = 0;
    let mut i__3: integer = 0;
    let mut q__1: complex = complex { r: 0., i: 0. };
    let mut q__2: complex = complex { r: 0., i: 0. };
    /* Local variables */
    let mut i__: integer = 0;
    let mut ix: integer = 0;
    let mut iy: integer = 0;
    let mut ctemp: complex = complex { r: 0., i: 0. };
    /*     .. Scalar Arguments .. */
    /*     .. */
    /*     .. Array Arguments .. */
    /*     .. */
    /*  Purpose */
    /*  ======= */
    /*     CDOTU forms the dot product of two vectors. */
    /*  Further Details */
    /*  =============== */
    /*     jack dongarra, linpack, 3/11/78. */
    /*     modified 12/3/93, array(1) declarations changed to array(*) */
    /*     .. Local Scalars .. */
    /*     .. */
    /* Parameter adjustments */
    cy = cy.offset(-1);
    cx = cx.offset(-1);
    /* Function Body */
    ctemp.r = 0.0f32;
    ctemp.i = 0.0f32;
    (*ret_val).r = 0.0f32;
    (*ret_val).i = 0.0f32;
    if *n <= 0 as libc::c_int as libc::c_long {
        return;
    }
    if *incx == 1 as libc::c_int as libc::c_long && *incy == 1 as libc::c_int as libc::c_long {
        /*        code for both increments equal to 1 */
        i__1 = *n;
        i__ = 1 as libc::c_int as integer;
        while i__ <= i__1 {
            i__2 = i__;
            i__3 = i__;
            q__2.r = (*cx.offset(i__2 as isize)).r * (*cy.offset(i__3 as isize)).r
                - (*cx.offset(i__2 as isize)).i * (*cy.offset(i__3 as isize)).i;
            q__2.i = (*cx.offset(i__2 as isize)).r * (*cy.offset(i__3 as isize)).i
                + (*cx.offset(i__2 as isize)).i * (*cy.offset(i__3 as isize)).r;
            q__1.r = ctemp.r + q__2.r;
            q__1.i = ctemp.i + q__2.i;
            ctemp.r = q__1.r;
            ctemp.i = q__1.i;
            i__ += 1
            /* L30: */
        }
        (*ret_val).r = ctemp.r;
        (*ret_val).i = ctemp.i;
        return;
    } else {
        /*        code for unequal increments or equal increments */
        /*          not equal to 1 */
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
            i__3 = iy;
            q__2.r = (*cx.offset(i__2 as isize)).r * (*cy.offset(i__3 as isize)).r
                - (*cx.offset(i__2 as isize)).i * (*cy.offset(i__3 as isize)).i;
            q__2.i = (*cx.offset(i__2 as isize)).r * (*cy.offset(i__3 as isize)).i
                + (*cx.offset(i__2 as isize)).i * (*cy.offset(i__3 as isize)).r;
            q__1.r = ctemp.r + q__2.r;
            q__1.i = ctemp.i + q__2.i;
            ctemp.r = q__1.r;
            ctemp.i = q__1.i;
            ix += *incx;
            iy += *incy;
            i__ += 1
            /* L10: */
        }
        (*ret_val).r = ctemp.r;
        (*ret_val).i = ctemp.i;
        return;
    };
}
/* cdotu_ */
