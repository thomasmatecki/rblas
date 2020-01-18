use libc;
/* f2c.h  --  Standard Fortran to C header file */
/* *  barf  [ba:rf]  2.  "He suggested using FORTRAN, and everybody barfed."

- From The Shogakukan DICTIONARY OF NEW ENGLISH (Second edition) */
pub type integer = libc::c_long;
pub type doublereal = libc::c_double;
/* dasum.f -- translated by f2c (version 20061008).
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
pub unsafe extern "C" fn f2c_dasum(
    mut n: *mut integer,
    mut dx: *mut doublereal,
    mut incx: *mut integer,
) -> doublereal {
    let mut current_block: u64;
    /* System generated locals */
    let mut i__1: integer = 0;
    let mut i__2: integer = 0;
    let mut ret_val: doublereal = 0.;
    let mut d__1: doublereal = 0.;
    let mut d__2: doublereal = 0.;
    let mut d__3: doublereal = 0.;
    let mut d__4: doublereal = 0.;
    let mut d__5: doublereal = 0.;
    let mut d__6: doublereal = 0.;
    /* Local variables */
    let mut i__: integer = 0;
    let mut m: integer = 0;
    let mut mp1: integer = 0;
    let mut dtemp: doublereal = 0.;
    let mut nincx: integer = 0;
    /*     .. Scalar Arguments .. */
    /*     .. */
    /*     .. Array Arguments .. */
    /*     .. */
    /*  Purpose */
    /*  ======= */
    /*     takes the sum of the absolute values. */
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
    ret_val = 0.0f64;
    dtemp = 0.0f64;
    if *n <= 0 as libc::c_int as libc::c_long || *incx <= 0 as libc::c_int as libc::c_long {
        return ret_val;
    }
    if *incx == 1 as libc::c_int as libc::c_long {
        /*        code for increment equal to 1 */
        /*        clean-up loop */
        m = *n % 6 as libc::c_int as libc::c_long;
        if m == 0 as libc::c_int as libc::c_long {
            current_block = 18405009441636945901;
        } else {
            i__2 = m;
            i__ = 1 as libc::c_int as integer;
            while i__ <= i__2 {
                d__1 = *dx.offset(i__ as isize);
                dtemp += (if d__1 >= 0 as libc::c_int as libc::c_double {
                    d__1
                } else {
                    -d__1
                });
                i__ += 1
                /* L30: */
            }
            if *n < 6 as libc::c_int as libc::c_long {
                current_block = 6784927079672828863;
            } else {
                current_block = 18405009441636945901;
            }
        }
        match current_block {
            18405009441636945901 => {
                mp1 = m + 1 as libc::c_int as libc::c_long;
                i__2 = *n;
                i__ = mp1;
                while i__ <= i__2 {
                    d__1 = *dx.offset(i__ as isize);
                    d__2 = *dx.offset((i__ + 1 as libc::c_int as libc::c_long) as isize);
                    d__3 = *dx.offset((i__ + 2 as libc::c_int as libc::c_long) as isize);
                    d__4 = *dx.offset((i__ + 3 as libc::c_int as libc::c_long) as isize);
                    d__5 = *dx.offset((i__ + 4 as libc::c_int as libc::c_long) as isize);
                    d__6 = *dx.offset((i__ + 5 as libc::c_int as libc::c_long) as isize);
                    dtemp = dtemp
                        + (if d__1 >= 0 as libc::c_int as libc::c_double {
                            d__1
                        } else {
                            -d__1
                        })
                        + (if d__2 >= 0 as libc::c_int as libc::c_double {
                            d__2
                        } else {
                            -d__2
                        })
                        + (if d__3 >= 0 as libc::c_int as libc::c_double {
                            d__3
                        } else {
                            -d__3
                        })
                        + (if d__4 >= 0 as libc::c_int as libc::c_double {
                            d__4
                        } else {
                            -d__4
                        })
                        + (if d__5 >= 0 as libc::c_int as libc::c_double {
                            d__5
                        } else {
                            -d__5
                        })
                        + (if d__6 >= 0 as libc::c_int as libc::c_double {
                            d__6
                        } else {
                            -d__6
                        });
                    i__ += 6 as libc::c_int as libc::c_long
                    /* L50: */
                }
            }
            _ => {}
        }
        ret_val = dtemp;
        return ret_val;
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
            d__1 = *dx.offset(i__ as isize);
            dtemp += (if d__1 >= 0 as libc::c_int as libc::c_double {
                d__1
            } else {
                -d__1
            });
            i__ += i__2
            /* L10: */
        }
        ret_val = dtemp;
        return ret_val;
    };
}
/* dasum_ */
