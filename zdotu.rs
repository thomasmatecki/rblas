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
/* zdotu.f -- translated by f2c (version 20061008).
   You must link the resulting object file with libf2c:
	on Microsoft Windows system, link with libf2c.lib;
	on Linux or Unix systems, link with .../path/to/libf2c.a -lm
	or, if you install libf2c.a in a standard place, with -lf2c -lm
	-- in that order, at the end of the command line, as in
		cc *.o -lf2c -lm
	Source for libf2c is in /netlib/f2c/libf2c.zip, e.g.,

		http://www.netlib.org/f2c/libf2c.zip
*/
/* Double Complex */
#[no_mangle]
pub unsafe extern "C" fn f2c_zdotu(mut ret_val: *mut doublecomplex,
                                   mut n: *mut integer,
                                   mut zx: *mut doublecomplex,
                                   mut incx: *mut integer,
                                   mut zy: *mut doublecomplex,
                                   mut incy: *mut integer) {
    /* System generated locals */
    let mut i__1: integer = 0;
    let mut i__2: integer = 0;
    let mut i__3: integer = 0;
    let mut z__1: doublecomplex = doublecomplex{r: 0., i: 0.,};
    let mut z__2: doublecomplex = doublecomplex{r: 0., i: 0.,};
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
    /*     ZDOTU forms the dot product of two vectors. */
    /*  Further Details */
/*  =============== */
    /*     jack dongarra, 3/11/78. */
/*     modified 12/3/93, array(1) declarations changed to array(*) */
    /*     .. Local Scalars .. */
/*     .. */
    /* Parameter adjustments */
    zy = zy.offset(-1);
    zx = zx.offset(-1);
    /* Function Body */
    ztemp.r = 0.0f64;
    ztemp.i = 0.0f64;
    (*ret_val).r = 0.0f64;
    (*ret_val).i = 0.0f64;
    if *n <= 0 as libc::c_int as libc::c_long { return }
    if *incx == 1 as libc::c_int as libc::c_long &&
           *incy == 1 as libc::c_int as libc::c_long {
        /*        code for both increments equal to 1 */
        i__1 = *n;
        i__ = 1 as libc::c_int as integer;
        while i__ <= i__1 {
            i__2 = i__;
            i__3 = i__;
            z__2.r =
                (*zx.offset(i__2 as isize)).r * (*zy.offset(i__3 as isize)).r
                    -
                    (*zx.offset(i__2 as isize)).i *
                        (*zy.offset(i__3 as isize)).i;
            z__2.i =
                (*zx.offset(i__2 as isize)).r * (*zy.offset(i__3 as isize)).i
                    +
                    (*zx.offset(i__2 as isize)).i *
                        (*zy.offset(i__3 as isize)).r;
            z__1.r = ztemp.r + z__2.r;
            z__1.i = ztemp.i + z__2.i;
            ztemp.r = z__1.r;
            ztemp.i = z__1.i;
            i__ += 1
            /* L30: */
        }
        (*ret_val).r = ztemp.r;
        (*ret_val).i = ztemp.i;
        return
    } else {
        /*        code for unequal increments or equal increments */
/*          not equal to 1 */
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
            i__3 = iy;
            z__2.r =
                (*zx.offset(i__2 as isize)).r * (*zy.offset(i__3 as isize)).r
                    -
                    (*zx.offset(i__2 as isize)).i *
                        (*zy.offset(i__3 as isize)).i;
            z__2.i =
                (*zx.offset(i__2 as isize)).r * (*zy.offset(i__3 as isize)).i
                    +
                    (*zx.offset(i__2 as isize)).i *
                        (*zy.offset(i__3 as isize)).r;
            z__1.r = ztemp.r + z__2.r;
            z__1.i = ztemp.i + z__2.i;
            ztemp.r = z__1.r;
            ztemp.i = z__1.i;
            ix += *incx;
            iy += *incy;
            i__ += 1
            /* L10: */
        }
        (*ret_val).r = ztemp.r;
        (*ret_val).i = ztemp.i;
        return
    };
}
/* zdotu_ */
