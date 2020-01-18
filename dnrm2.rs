use libc;
extern "C" {
    #[no_mangle]
    fn sqrt(_: libc::c_double) -> libc::c_double;
}
/* f2c.h  --  Standard Fortran to C header file */
/* *  barf  [ba:rf]  2.  "He suggested using FORTRAN, and everybody barfed."

- From The Shogakukan DICTIONARY OF NEW ENGLISH (Second edition) */
pub type integer = libc::c_long;
pub type doublereal = libc::c_double;
/* dnrm2.f -- translated by f2c (version 20061008).
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
pub unsafe extern "C" fn f2c_dnrm2(
    mut n: *mut integer,
    mut x: *mut doublereal,
    mut incx: *mut integer,
) -> doublereal {
    /* System generated locals */
    let mut i__1: integer = 0;
    let mut i__2: integer = 0;
    let mut ret_val: doublereal = 0.;
    let mut d__1: doublereal = 0.;
    /* Builtin functions */
    /* Local variables */
    let mut ix: integer = 0;
    let mut ssq: doublereal = 0.;
    let mut norm: doublereal = 0.;
    let mut scale: doublereal = 0.;
    let mut absxi: doublereal = 0.;
    /*     .. Scalar Arguments .. */
    /*     .. */
    /*     .. Array Arguments .. */
    /*     .. */
    /*  Purpose */
    /*  ======= */
    /*  DNRM2 returns the euclidean norm of a vector via the function */
    /*  name, so that */
    /*     DNRM2 := sqrt( x'*x ) */
    /*  -- This version written on 25-October-1982. */
    /*     Modified on 14-October-1993 to inline the call to DLASSQ. */
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
    if *n < 1 as libc::c_int as libc::c_long || *incx < 1 as libc::c_int as libc::c_long {
        norm = 0.0f64
    } else if *n == 1 as libc::c_int as libc::c_long {
        norm = if *x.offset(1 as libc::c_int as isize) >= 0 as libc::c_int as libc::c_double {
            *x.offset(1 as libc::c_int as isize)
        } else {
            -*x.offset(1 as libc::c_int as isize)
        }
    } else {
        scale = 0.0f64;
        ssq = 1.0f64;
        /*        The following loop is equivalent to this call to the LAPACK */
        /*        auxiliary routine: */
        /*        CALL DLASSQ( N, X, INCX, SCALE, SSQ ) */
        i__1 = (*n - 1 as libc::c_int as libc::c_long) * *incx + 1 as libc::c_int as libc::c_long;
        i__2 = *incx;
        ix = 1 as libc::c_int as integer;
        while if i__2 < 0 as libc::c_int as libc::c_long {
            (ix >= i__1) as libc::c_int
        } else {
            (ix <= i__1) as libc::c_int
        } != 0
        {
            if *x.offset(ix as isize) != 0.0f64 {
                d__1 = *x.offset(ix as isize);
                absxi = (if d__1 >= 0 as libc::c_int as libc::c_double {
                    d__1
                } else {
                    -d__1
                });
                if scale < absxi {
                    /* Computing 2nd power */
                    d__1 = scale / absxi;
                    ssq = ssq * (d__1 * d__1) + 1.0f64;
                    scale = absxi
                } else {
                    /* Computing 2nd power */
                    d__1 = absxi / scale;
                    ssq += d__1 * d__1
                }
            }
            ix += i__2
            /* L10: */
        }
        norm = scale * sqrt(ssq)
    }
    ret_val = norm;
    return ret_val;
    /*     End of DNRM2. */
}
/* dnrm2_ */
