use libc;
/* f2c.h  --  Standard Fortran to C header file */
/* *  barf  [ba:rf]  2.  "He suggested using FORTRAN, and everybody barfed."

- From The Shogakukan DICTIONARY OF NEW ENGLISH (Second edition) */
pub type integer = libc::c_long;
pub type real = libc::c_float;
pub type doublereal = libc::c_double;
/* sasum.f -- translated by f2c (version 20061008).
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
pub unsafe extern "C" fn f2c_sasum(
    mut n: *mut integer,
    mut sx: *mut real,
    mut incx: *mut integer,
) -> doublereal {
    let mut current_block: u64;
    /* System generated locals */
    let mut i__1: integer = 0;
    let mut i__2: integer = 0;
    let mut ret_val: real = 0.;
    let mut r__1: real = 0.;
    let mut r__2: real = 0.;
    let mut r__3: real = 0.;
    let mut r__4: real = 0.;
    let mut r__5: real = 0.;
    let mut r__6: real = 0.;
    /* Local variables */
    let mut i__: integer = 0;
    let mut m: integer = 0;
    let mut mp1: integer = 0;
    let mut nincx: integer = 0;
    let mut stemp: real = 0.;
    /*     .. Scalar Arguments .. */
    /*     .. */
    /*     .. Array Arguments .. */
    /*     .. */
    /*  Purpose */
    /*  ======= */
    /*     takes the sum of the absolute values. */
    /*     uses unrolled loops for increment equal to one. */
    /*     jack dongarra, linpack, 3/11/78. */
    /*     modified 3/93 to return if incx .le. 0. */
    /*     modified 12/3/93, array(1) declarations changed to array(*) */
    /*     .. Local Scalars .. */
    /*     .. */
    /*     .. Intrinsic Functions .. */
    /*     .. */
    /* Parameter adjustments */
    sx = sx.offset(-1);
    /* Function Body */
    ret_val = 0.0f32;
    stemp = 0.0f32;
    if *n <= 0 as libc::c_int as libc::c_long || *incx <= 0 as libc::c_int as libc::c_long {
        return ret_val as doublereal;
    }
    if *incx == 1 as libc::c_int as libc::c_long {
        /*        code for increment equal to 1 */
        /*        clean-up loop */
        m = *n % 6 as libc::c_int as libc::c_long;
        if m == 0 as libc::c_int as libc::c_long {
            current_block = 10493328989435379652;
        } else {
            i__2 = m;
            i__ = 1 as libc::c_int as integer;
            while i__ <= i__2 {
                r__1 = *sx.offset(i__ as isize);
                stemp = (stemp as libc::c_double
                    + (if r__1 >= 0 as libc::c_int as libc::c_float {
                        r__1
                    } else {
                        -r__1
                    }) as doublereal) as real;
                i__ += 1
                /* L30: */
            }
            if *n < 6 as libc::c_int as libc::c_long {
                current_block = 18055918414398801273;
            } else {
                current_block = 10493328989435379652;
            }
        }
        match current_block {
            10493328989435379652 => {
                mp1 = m + 1 as libc::c_int as libc::c_long;
                i__2 = *n;
                i__ = mp1;
                while i__ <= i__2 {
                    r__1 = *sx.offset(i__ as isize);
                    r__2 = *sx.offset((i__ + 1 as libc::c_int as libc::c_long) as isize);
                    r__3 = *sx.offset((i__ + 2 as libc::c_int as libc::c_long) as isize);
                    r__4 = *sx.offset((i__ + 3 as libc::c_int as libc::c_long) as isize);
                    r__5 = *sx.offset((i__ + 4 as libc::c_int as libc::c_long) as isize);
                    r__6 = *sx.offset((i__ + 5 as libc::c_int as libc::c_long) as isize);
                    stemp = (stemp as libc::c_double
                        + (if r__1 >= 0 as libc::c_int as libc::c_float {
                            r__1
                        } else {
                            -r__1
                        }) as doublereal
                        + (if r__2 >= 0 as libc::c_int as libc::c_float {
                            r__2
                        } else {
                            -r__2
                        }) as doublereal
                        + (if r__3 >= 0 as libc::c_int as libc::c_float {
                            r__3
                        } else {
                            -r__3
                        }) as doublereal
                        + (if r__4 >= 0 as libc::c_int as libc::c_float {
                            r__4
                        } else {
                            -r__4
                        }) as doublereal
                        + (if r__5 >= 0 as libc::c_int as libc::c_float {
                            r__5
                        } else {
                            -r__5
                        }) as doublereal
                        + (if r__6 >= 0 as libc::c_int as libc::c_float {
                            r__6
                        } else {
                            -r__6
                        }) as doublereal) as real;
                    i__ += 6 as libc::c_int as libc::c_long
                    /* L50: */
                }
            }
            _ => {}
        }
        ret_val = stemp;
        return ret_val as doublereal;
    } else {
        /*        code for increment not equal to 1 */
        nincx = *n * *incx;
        i__1 = nincx;
        i__2 = *incx;
        i__ = 1 as libc::c_int as integer;
        while if i__2 < 0 as libc::c_int as libc::c_long {
            (i__ >= i__1) as libc::c_int
        } else {
            (i__ <= i__1) as libc::c_int
        } != 0
        {
            r__1 = *sx.offset(i__ as isize);
            stemp = (stemp as libc::c_double
                + (if r__1 >= 0 as libc::c_int as libc::c_float {
                    r__1
                } else {
                    -r__1
                }) as doublereal) as real;
            i__ += i__2
            /* L10: */
        }
        ret_val = stemp;
        return ret_val as doublereal;
    };
}
/* sasum_ */
