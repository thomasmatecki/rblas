use ::libc;
extern "C" {
    #[no_mangle]
    fn sqrt(_: libc::c_double) -> libc::c_double;
}
pub type doublereal = libc::c_double;
/* drotg.f -- translated by f2c (version 20061008).
   You must link the resulting object file with libf2c:
	on Microsoft Windows system, link with libf2c.lib;
	on Linux or Unix systems, link with .../path/to/libf2c.a -lm
	or, if you install libf2c.a in a standard place, with -lf2c -lm
	-- in that order, at the end of the command line, as in
		cc *.o -lf2c -lm
	Source for libf2c is in /netlib/f2c/libf2c.zip, e.g.,

		http://www.netlib.org/f2c/libf2c.zip
*/
/* Table of constant values */
static mut c_b4: doublereal = 1.0f64;
/* Subroutine */
#[no_mangle]
pub unsafe extern "C" fn f2c_drotg(mut da: *mut doublereal,
                                   mut db: *mut doublereal,
                                   mut c__: *mut doublereal,
                                   mut s: *mut doublereal) -> libc::c_int {
    /* System generated locals */
    let mut d__1: doublereal = 0.;
    let mut d__2: doublereal = 0.;
    /* Builtin functions */
    extern "C" {
        #[link_name = "d_sign"]
        fn d_sign_0(_: *mut doublereal, _: *mut doublereal) -> libc::c_double;
    }
    /* Local variables */
    let mut r__: doublereal = 0.;
    let mut z__: doublereal = 0.;
    let mut roe: doublereal = 0.;
    let mut scale: doublereal = 0.;
    /*     .. Scalar Arguments .. */
/*     .. */
    /*  Purpose */
/*  ======= */
    /*     construct givens plane rotation. */
/*     jack dongarra, linpack, 3/11/78. */
    /*     .. Local Scalars .. */
/*     .. */
/*     .. Intrinsic Functions .. */
/*     .. */
    roe = *db;
    if (if *da >= 0 as libc::c_int as libc::c_double { *da } else { -*da }) >
           (if *db >= 0 as libc::c_int as libc::c_double {
                *db
            } else { -*db }) {
        roe = *da
    }
    scale =
        (if *da >= 0 as libc::c_int as libc::c_double { *da } else { -*da }) +
            (if *db >= 0 as libc::c_int as libc::c_double {
                 *db
             } else { -*db });
    if scale != 0.0f64 {
        /* Computing 2nd power */
        d__1 = *da / scale;
        /* Computing 2nd power */
        d__2 = *db / scale;
        r__ = scale * sqrt(d__1 * d__1 + d__2 * d__2);
        r__ = d_sign_0(&mut c_b4, &mut roe) * r__;
        *c__ = *da / r__;
        *s = *db / r__;
        z__ = 1.0f64;
        if (if *da >= 0 as libc::c_int as libc::c_double {
                *da
            } else { -*da }) >
               (if *db >= 0 as libc::c_int as libc::c_double {
                    *db
                } else { -*db }) {
            z__ = *s
        }
        if (if *db >= 0 as libc::c_int as libc::c_double {
                *db
            } else { -*db }) >=
               (if *da >= 0 as libc::c_int as libc::c_double {
                    *da
                } else { -*da }) && *c__ != 0.0f64 {
            z__ = 1.0f64 / *c__
        }
    } else { *c__ = 1.0f64; *s = 0.0f64; r__ = 0.0f64; z__ = 0.0f64 }
    *da = r__;
    *db = z__;
    return 0 as libc::c_int;
}
/* drotg_ */
