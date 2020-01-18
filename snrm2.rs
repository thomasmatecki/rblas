use ::libc;
extern "C" {
    #[no_mangle]
    fn sqrt(_: libc::c_double) -> libc::c_double;
}
/* f2c.h  --  Standard Fortran to C header file */
/* *  barf  [ba:rf]  2.  "He suggested using FORTRAN, and everybody barfed."

	- From The Shogakukan DICTIONARY OF NEW ENGLISH (Second edition) */
pub type integer = libc::c_long;
pub type real = libc::c_float;
pub type doublereal = libc::c_double;
/* snrm2.f -- translated by f2c (version 20061008).
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
pub unsafe extern "C" fn f2c_snrm2(mut n: *mut integer, mut x: *mut real,
                                   mut incx: *mut integer) -> doublereal {
    /* System generated locals */
    let mut i__1: integer = 0;
    let mut i__2: integer = 0;
    let mut ret_val: real = 0.;
    let mut r__1: real = 0.;
    /* Builtin functions */
    /* Local variables */
    let mut ix: integer = 0;
    let mut ssq: real = 0.;
    let mut norm: real = 0.;
    let mut scale: real = 0.;
    let mut absxi: real = 0.;
    /*     .. Scalar Arguments .. */
/*     .. */
/*     .. Array Arguments .. */
/*     .. */
    /*  Purpose */
/*  ======= */
    /*  SNRM2 returns the euclidean norm of a vector via the function */
/*  name, so that */
    /*     SNRM2 := sqrt( x'*x ). */
    /*  Further Details */
/*  =============== */
    /*  -- This version written on 25-October-1982. */
/*     Modified on 14-October-1993 to inline the call to SLASSQ. */
/*     Sven Hammarling, Nag Ltd. */
    /*     .. Parameters .. */
/*     .. */
/*     .. Local Scalars .. */
/*     .. */
/*     .. Intrinsic Functions .. */
/*     .. */
    /* Parameter adjustments */
    x = x.offset(-1);
    /* Function Body */
    if *n < 1 as libc::c_int as libc::c_long ||
           *incx < 1 as libc::c_int as libc::c_long {
        norm = 0.0f32
    } else if *n == 1 as libc::c_int as libc::c_long {
        norm =
            if *x.offset(1 as libc::c_int as isize) >=
                   0 as libc::c_int as libc::c_float {
                *x.offset(1 as libc::c_int as isize)
            } else { -*x.offset(1 as libc::c_int as isize) } as doublereal as
                real
    } else {
        scale = 0.0f32;
        ssq = 1.0f32;
        /*        The following loop is equivalent to this call to the LAPACK */
/*        auxiliary routine: */
/*        CALL SLASSQ( N, X, INCX, SCALE, SSQ ) */
        i__1 =
            (*n - 1 as libc::c_int as libc::c_long) * *incx +
                1 as libc::c_int as libc::c_long;
        i__2 = *incx;
        ix = 1 as libc::c_int as integer;
        while if i__2 < 0 as libc::c_int as libc::c_long {
                  (ix >= i__1) as libc::c_int
              } else { (ix <= i__1) as libc::c_int } != 0 {
            if *x.offset(ix as isize) != 0.0f32 {
                r__1 = *x.offset(ix as isize);
                absxi =
                    (if r__1 >= 0 as libc::c_int as libc::c_float {
                         r__1
                     } else { -r__1 }) as doublereal as real;
                if scale < absxi {
                    /* Computing 2nd power */
                    r__1 = scale / absxi;
                    ssq = ssq * (r__1 * r__1) + 1.0f32;
                    scale = absxi
                } else {
                    /* Computing 2nd power */
                    r__1 = absxi / scale;
                    ssq += r__1 * r__1
                }
            }
            ix += i__2
            /* L10: */
        }
        norm = (scale as libc::c_double * sqrt(ssq as libc::c_double)) as real
    }
    ret_val = norm;
    return ret_val as doublereal;
    /*     End of SNRM2. */
}
/* snrm2_ */
