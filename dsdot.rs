use libc;
/* f2c.h  --  Standard Fortran to C header file */
/* *  barf  [ba:rf]  2.  "He suggested using FORTRAN, and everybody barfed."

- From The Shogakukan DICTIONARY OF NEW ENGLISH (Second edition) */
pub type integer = libc::c_long;
pub type real = libc::c_float;
pub type doublereal = libc::c_double;
/* dsdot.f -- translated by f2c (version 20061008).
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
pub unsafe extern "C" fn dsdot_(
    mut n: *mut integer,
    mut sx: *mut real,
    mut incx: *mut integer,
    mut sy: *mut real,
    mut incy: *mut integer,
) -> doublereal {
    /* System generated locals */
    let mut i__1: integer = 0;
    let mut i__2: integer = 0;
    let mut ret_val: doublereal = 0.;
    /* Local variables */
    let mut i__: integer = 0;
    let mut ns: integer = 0;
    let mut kx: integer = 0;
    let mut ky: integer = 0;
    /*     .. Scalar Arguments .. */
    /*     .. */
    /*     .. Array Arguments .. */
    /*     .. */
    /*  AUTHORS */
    /*  ======= */
    /*  Lawson, C. L., (JPL), Hanson, R. J., (SNLA), */
    /*  Kincaid, D. R., (U. of Texas), Krogh, F. T., (JPL) */
    /*  Purpose */
    /*  ======= */
    /*  Compute the inner product of two vectors with extended */
    /*  precision accumulation and result. */
    /*  Returns D.P. dot product accumulated in D.P., for S.P. SX and SY */
    /*  DSDOT = sum for I = 0 to N-1 of  SX(LX+I*INCX) * SY(LY+I*INCY), */
    /*  where LX = 1 if INCX .GE. 0, else LX = 1+(1-N)*INCX, and LY is */
    /*  defined in a similar way using INCY. */
    /*  Arguments */
    /*  ========= */
    /*  N      (input) INTEGER */
    /*         number of elements in input vector(s) */
    /*  SX     (input) REAL array, dimension(N) */
    /*         single precision vector with N elements */
    /*  INCX   (input) INTEGER */
    /*          storage spacing between elements of SX */
    /*  SY     (input) REAL array, dimension(N) */
    /*         single precision vector with N elements */
    /*  INCY   (input) INTEGER */
    /*         storage spacing between elements of SY */
    /*  DSDOT  (output) DOUBLE PRECISION */
    /*         DSDOT  double precision dot product (zero if N.LE.0) */
    /*  REFERENCES */
    /*  ========== */
    /*  C. L. Lawson, R. J. Hanson, D. R. Kincaid and F. T. */
    /*  Krogh, Basic linear algebra subprograms for Fortran */
    /*  usage, Algorithm No. 539, Transactions on Mathematical */
    /*  Software 5, 3 (September 1979), pp. 308-323. */
    /*  REVISION HISTORY  (YYMMDD) */
    /*  ========================== */
    /*  791001  DATE WRITTEN */
    /*  890831  Modified array declarations.  (WRB) */
    /*  890831  REVISION DATE from Version 3.2 */
    /*  891214  Prologue converted to Version 4.0 format.  (BAB) */
    /*  920310  Corrected definition of LX in DESCRIPTION.  (WRB) */
    /*  920501  Reformatted the REFERENCES section.  (WRB) */
    /*  070118  Reformat to LAPACK style (JL) */
    /*  ===================================================================== */
    /*     .. Local Scalars .. */
    /*     .. */
    /*     .. Intrinsic Functions .. */
    /*     .. */
    /* Parameter adjustments */
    sy = sy.offset(-1);
    sx = sx.offset(-1);
    /* Function Body */
    ret_val = 0.0f64;
    if *n <= 0 as libc::c_int as libc::c_long {
        return ret_val;
    }
    if *incx == *incy && *incx > 0 as libc::c_int as libc::c_long {
        /*     Code for equal, positive, non-unit increments. */
        ns = *n * *incx;
        i__1 = ns;
        i__2 = *incx;
        i__ = 1 as libc::c_int as integer;
        while if i__2 < 0 as libc::c_int as libc::c_long {
            (i__ >= i__1) as libc::c_int
        } else {
            (i__ <= i__1) as libc::c_int
        } != 0
        {
            ret_val +=
                *sx.offset(i__ as isize) as doublereal * *sy.offset(i__ as isize) as doublereal;
            i__ += i__2
            /* L30: */
        }
        return ret_val;
    } else {
        /*     Code for unequal or nonpositive increments. */
        kx = 1 as libc::c_int as integer;
        ky = 1 as libc::c_int as integer;
        if *incx < 0 as libc::c_int as libc::c_long {
            kx = (1 as libc::c_int as libc::c_long - *n) * *incx + 1 as libc::c_int as libc::c_long
        }
        if *incy < 0 as libc::c_int as libc::c_long {
            ky = (1 as libc::c_int as libc::c_long - *n) * *incy + 1 as libc::c_int as libc::c_long
        }
        i__1 = *n;
        i__ = 1 as libc::c_int as integer;
        while i__ <= i__1 {
            ret_val +=
                *sx.offset(kx as isize) as doublereal * *sy.offset(ky as isize) as doublereal;
            kx += *incx;
            ky += *incy;
            i__ += 1
            /* L10: */
        }
        return ret_val;
    };
}
/* dsdot_ */
