use ::libc;
/* f2c.h  --  Standard Fortran to C header file */
/* *  barf  [ba:rf]  2.  "He suggested using FORTRAN, and everybody barfed."

	- From The Shogakukan DICTIONARY OF NEW ENGLISH (Second edition) */
pub type integer = libc::c_long;
pub type doublereal = libc::c_double;
/* idamax.f -- translated by f2c (version 20061008).
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
pub unsafe extern "C" fn f2c_idamax(mut n: *mut integer,
                                    mut dx: *mut doublereal,
                                    mut incx: *mut integer) -> integer {
    /* System generated locals */
    let mut ret_val: integer = 0;
    let mut i__1: integer = 0;
    let mut d__1: doublereal = 0.;
    /* Local variables */
    let mut i__: integer = 0;
    let mut ix: integer = 0;
    let mut dmax__: doublereal = 0.;
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
/*     .. Intrinsic Functions .. */
/*     .. */
    /* Parameter adjustments */
    dx = dx.offset(-1);
    /* Function Body */
    ret_val = 0 as libc::c_int as integer;
    if *n < 1 as libc::c_int as libc::c_long ||
           *incx <= 0 as libc::c_int as libc::c_long {
        return ret_val
    }
    ret_val = 1 as libc::c_int as integer;
    if *n == 1 as libc::c_int as libc::c_long { return ret_val }
    if *incx == 1 as libc::c_int as libc::c_long {
        /*        code for increment equal to 1 */
        dmax__ =
            if *dx.offset(1 as libc::c_int as isize) >=
                   0 as libc::c_int as libc::c_double {
                *dx.offset(1 as libc::c_int as isize)
            } else { -*dx.offset(1 as libc::c_int as isize) };
        i__1 = *n;
        i__ = 2 as libc::c_int as integer;
        while i__ <= i__1 {
            d__1 = *dx.offset(i__ as isize);
            if !((if d__1 >= 0 as libc::c_int as libc::c_double {
                      d__1
                  } else { -d__1 }) <= dmax__) {
                ret_val = i__;
                d__1 = *dx.offset(i__ as isize);
                dmax__ =
                    (if d__1 >= 0 as libc::c_int as libc::c_double {
                         d__1
                     } else { -d__1 })
            }
            i__ += 1
        }
        return ret_val
    } else {
        /*        code for increment not equal to 1 */
        ix = 1 as libc::c_int as integer;
        dmax__ =
            if *dx.offset(1 as libc::c_int as isize) >=
                   0 as libc::c_int as libc::c_double {
                *dx.offset(1 as libc::c_int as isize)
            } else { -*dx.offset(1 as libc::c_int as isize) };
        ix += *incx;
        i__1 = *n;
        i__ = 2 as libc::c_int as integer;
        while i__ <= i__1 {
            d__1 = *dx.offset(ix as isize);
            if !((if d__1 >= 0 as libc::c_int as libc::c_double {
                      d__1
                  } else { -d__1 }) <= dmax__) {
                ret_val = i__;
                d__1 = *dx.offset(ix as isize);
                dmax__ =
                    (if d__1 >= 0 as libc::c_int as libc::c_double {
                         d__1
                     } else { -d__1 })
            }
            ix += *incx;
            i__ += 1
            /* L10: */
        }
        return ret_val
    };
}
/* idamax_ */
