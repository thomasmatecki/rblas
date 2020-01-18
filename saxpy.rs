use ::libc;
/* f2c.h  --  Standard Fortran to C header file */
/* *  barf  [ba:rf]  2.  "He suggested using FORTRAN, and everybody barfed."

	- From The Shogakukan DICTIONARY OF NEW ENGLISH (Second edition) */
pub type integer = libc::c_long;
pub type real = libc::c_float;
/* saxpy.f -- translated by f2c (version 20061008).
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
pub unsafe extern "C" fn f2c_saxpy(mut n: *mut integer, mut sa: *mut real,
                                   mut sx: *mut real, mut incx: *mut integer,
                                   mut sy: *mut real, mut incy: *mut integer)
 -> libc::c_int {
    /* System generated locals */
    let mut i__1: integer = 0;
    /* Local variables */
    let mut i__: integer = 0;
    let mut m: integer = 0;
    let mut ix: integer = 0;
    let mut iy: integer = 0;
    let mut mp1: integer = 0;
    /*     .. Scalar Arguments .. */
/*     .. */
/*     .. Array Arguments .. */
/*     .. */
    /*  Purpose */
/*  ======= */
    /*     SAXPY constant times a vector plus a vector. */
/*     uses unrolled loop for increments equal to one. */
/*     jack dongarra, linpack, 3/11/78. */
/*     modified 12/3/93, array(1) declarations changed to array(*) */
    /*     .. Local Scalars .. */
/*     .. */
/*     .. Intrinsic Functions .. */
/*     .. */
    /* Parameter adjustments */
    sy = sy.offset(-1);
    sx = sx.offset(-1);
    /* Function Body */
    if *n <= 0 as libc::c_int as libc::c_long { return 0 as libc::c_int }
    if *sa == 0.0f32 { return 0 as libc::c_int }
    if *incx == 1 as libc::c_int as libc::c_long &&
           *incy == 1 as libc::c_int as libc::c_long {
        /*        code for both increments equal to 1 */
        /*        clean-up loop */
        m = *n % 4 as libc::c_int as libc::c_long;
        if !(m == 0 as libc::c_int as libc::c_long) {
            i__1 = m;
            i__ = 1 as libc::c_int as integer;
            while i__ <= i__1 {
                let ref mut fresh1 = *sy.offset(i__ as isize);
                *fresh1 += *sa * *sx.offset(i__ as isize);
                i__ += 1
                /* L30: */
            }
            if *n < 4 as libc::c_int as libc::c_long {
                return 0 as libc::c_int
            }
        }
        mp1 = m + 1 as libc::c_int as libc::c_long;
        i__1 = *n;
        i__ = mp1;
        while i__ <= i__1 {
            let ref mut fresh2 = *sy.offset(i__ as isize);
            *fresh2 += *sa * *sx.offset(i__ as isize);
            let ref mut fresh3 =
                *sy.offset((i__ + 1 as libc::c_int as libc::c_long) as isize);
            *fresh3 +=
                *sa *
                    *sx.offset((i__ + 1 as libc::c_int as libc::c_long) as
                                   isize);
            let ref mut fresh4 =
                *sy.offset((i__ + 2 as libc::c_int as libc::c_long) as isize);
            *fresh4 +=
                *sa *
                    *sx.offset((i__ + 2 as libc::c_int as libc::c_long) as
                                   isize);
            let ref mut fresh5 =
                *sy.offset((i__ + 3 as libc::c_int as libc::c_long) as isize);
            *fresh5 +=
                *sa *
                    *sx.offset((i__ + 3 as libc::c_int as libc::c_long) as
                                   isize);
            i__ += 4 as libc::c_int as libc::c_long
            /* L50: */
        }
        return 0 as libc::c_int
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
            let ref mut fresh0 = *sy.offset(iy as isize);
            *fresh0 += *sa * *sx.offset(ix as isize);
            ix += *incx;
            iy += *incy;
            i__ += 1
            /* L10: */
        }
        return 0 as libc::c_int
    };
}
/* saxpy_ */
