use ::libc;
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
/* zswap.f -- translated by f2c (version 20061008).
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
pub unsafe extern "C" fn f2c_zswap(mut n: *mut integer,
                                   mut zx: *mut doublecomplex,
                                   mut incx: *mut integer,
                                   mut zy: *mut doublecomplex,
                                   mut incy: *mut integer) -> libc::c_int {
    /* System generated locals */
    let mut i__1: integer = 0;
    let mut i__2: integer = 0;
    let mut i__3: integer = 0;
    /* Local variables */
    let mut i__: integer = 0;
    let mut ix: integer = 0;
    let mut iy: integer = 0;
    let mut ztemp: doublecomplex = doublecomplex{r: 0., i: 0.,};
    /*     .. Scalar Arguments .. */
/*     .. */
/*     .. Array Arguments .. */
/*     .. */
    /*  Purpose */
/*  ======= */
    /*     interchanges two vectors. */
/*     jack dongarra, 3/11/78. */
/*     modified 12/3/93, array(1) declarations changed to array(*) */
    /*     .. Local Scalars .. */
/*     .. */
    /* Parameter adjustments */
    zy = zy.offset(-1);
    zx = zx.offset(-1);
    /* Function Body */
    if *n <= 0 as libc::c_int as libc::c_long { return 0 as libc::c_int }
    if *incx == 1 as libc::c_int as libc::c_long &&
           *incy == 1 as libc::c_int as libc::c_long {
        /*       code for both increments equal to 1 */
        i__1 = *n;
        i__ = 1 as libc::c_int as integer;
        while i__ <= i__1 {
            i__2 = i__;
            ztemp.r = (*zx.offset(i__2 as isize)).r;
            ztemp.i = (*zx.offset(i__2 as isize)).i;
            i__2 = i__;
            i__3 = i__;
            (*zx.offset(i__2 as isize)).r = (*zy.offset(i__3 as isize)).r;
            (*zx.offset(i__2 as isize)).i = (*zy.offset(i__3 as isize)).i;
            i__2 = i__;
            (*zy.offset(i__2 as isize)).r = ztemp.r;
            (*zy.offset(i__2 as isize)).i = ztemp.i;
            i__ += 1
            /* L30: */
        }
        return 0 as libc::c_int
    } else {
        /*       code for unequal increments or equal increments not equal */
/*         to 1 */
        ix = 1 as libc::c_int as integer;
        iy = 1 as libc::c_int as integer;
        if *incx < 0 as libc::c_int as libc::c_long {
            ix =
                (-*n + 1 as libc::c_int as libc::c_long) * *incx +
                    1 as libc::c_int as libc::c_long
        }
        if *incy < 0 as libc::c_int as libc::c_long {
            iy =
                (-*n + 1 as libc::c_int as libc::c_long) * *incy +
                    1 as libc::c_int as libc::c_long
        }
        i__1 = *n;
        i__ = 1 as libc::c_int as integer;
        while i__ <= i__1 {
            i__2 = ix;
            ztemp.r = (*zx.offset(i__2 as isize)).r;
            ztemp.i = (*zx.offset(i__2 as isize)).i;
            i__2 = ix;
            i__3 = iy;
            (*zx.offset(i__2 as isize)).r = (*zy.offset(i__3 as isize)).r;
            (*zx.offset(i__2 as isize)).i = (*zy.offset(i__3 as isize)).i;
            i__2 = iy;
            (*zy.offset(i__2 as isize)).r = ztemp.r;
            (*zy.offset(i__2 as isize)).i = ztemp.i;
            ix += *incx;
            iy += *incy;
            i__ += 1
            /* L10: */
        }
        return 0 as libc::c_int
    };
}
/* zswap_ */
