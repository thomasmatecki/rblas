use ::libc;
extern "C" {
    #[no_mangle]
    fn sqrt(_: libc::c_double) -> libc::c_double;
}
pub type real = libc::c_float;
pub type doublereal = libc::c_double;
/* srotg.f -- translated by f2c (version 20061008).
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
static mut c_b4: real = 1.0f32;
/* Subroutine */
#[no_mangle]
pub unsafe extern "C" fn f2c_srotg(mut sa: *mut real, mut sb: *mut real,
                                   mut c__: *mut real, mut s: *mut real)
 -> libc::c_int {
    /* System generated locals */
    let mut r__1: real = 0.;
    let mut r__2: real = 0.;
    /* Builtin functions */
    extern "C" {
        #[link_name = "r_sign"]
        fn r_sign_0(_: *mut real, _: *mut real) -> libc::c_double;
    }
    /* Local variables */
    let mut r__: real = 0.;
    let mut z__: real = 0.;
    let mut roe: real = 0.;
    let mut scale: real = 0.;
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
    roe = *sb;
    if (if *sa >= 0 as libc::c_int as libc::c_float { *sa } else { -*sa }) as
           doublereal >
           (if *sb >= 0 as libc::c_int as libc::c_float { *sb } else { -*sb })
               as doublereal {
        roe = *sa
    }
    scale =
        ((if *sa >= 0 as libc::c_int as libc::c_float { *sa } else { -*sa })
             as doublereal +
             (if *sb >= 0 as libc::c_int as libc::c_float {
                  *sb
              } else { -*sb }) as doublereal) as real;
    if scale != 0.0f32 {
        /* Computing 2nd power */
        r__1 = *sa / scale;
        /* Computing 2nd power */
        r__2 = *sb / scale;
        r__ =
            (scale as libc::c_double *
                 sqrt((r__1 * r__1 + r__2 * r__2) as libc::c_double)) as real;
        r__ = (r_sign_0(&mut c_b4, &mut roe) * r__ as libc::c_double) as real;
        *c__ = *sa / r__;
        *s = *sb / r__;
        z__ = 1.0f32;
        if (if *sa >= 0 as libc::c_int as libc::c_float { *sa } else { -*sa })
               as doublereal >
               (if *sb >= 0 as libc::c_int as libc::c_float {
                    *sb
                } else { -*sb }) as doublereal {
            z__ = *s
        }
        if (if *sb >= 0 as libc::c_int as libc::c_float { *sb } else { -*sb })
               as doublereal >=
               (if *sa >= 0 as libc::c_int as libc::c_float {
                    *sa
                } else { -*sa }) as doublereal && *c__ != 0.0f32 {
            z__ = 1.0f32 / *c__
        }
    } else { *c__ = 1.0f32; *s = 0.0f32; r__ = 0.0f32; z__ = 0.0f32 }
    *sa = r__;
    *sb = z__;
    return 0 as libc::c_int;
}
/* srotg_ */
