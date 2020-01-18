use ::libc;
extern "C" {
    #[no_mangle]
    fn sqrt(_: libc::c_double) -> libc::c_double;
}
pub type real = libc::c_float;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct complex {
    pub r: real,
    pub i: real,
}
/* crotg.f -- translated by f2c (version 20061008).
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
pub unsafe extern "C" fn f2c_crotg(mut ca: *mut complex, mut cb: *mut complex,
                                   mut c__: *mut real, mut s: *mut complex)
 -> libc::c_int {
    /* System generated locals */
    let mut r__1: real = 0.;
    let mut r__2: real = 0.;
    let mut q__1: complex = complex{r: 0., i: 0.,};
    let mut q__2: complex = complex{r: 0., i: 0.,};
    let mut q__3: complex = complex{r: 0., i: 0.,};
    /* Builtin functions */
    extern "C" {
        #[link_name = "c_abs"]
        fn c_abs_0(_: *mut complex) -> libc::c_double;
    }
    extern "C" {
        #[link_name = "r_cnjg"]
        fn r_cnjg_0(_: *mut complex, _: *mut complex);
    }
    /* Local variables */
    let mut norm: real = 0.;
    let mut alpha: complex = complex{r: 0., i: 0.,};
    let mut scale: real = 0.;
    /*     .. Scalar Arguments .. */
/*     .. */
    /*  Purpose */
/*  ======= */
    /*  CROTG determines a complex Givens rotation. */
    /*     .. Local Scalars .. */
/*     .. */
/*     .. Intrinsic Functions .. */
/*     .. */
    if c_abs_0(ca) != 0.0f32 as libc::c_double {
        scale = (c_abs_0(ca) + c_abs_0(cb)) as real;
        q__1.r = (*ca).r / scale;
        q__1.i = (*ca).i / scale;
        /* Computing 2nd power */
        r__1 = c_abs_0(&mut q__1) as real;
        q__2.r = (*cb).r / scale;
        q__2.i = (*cb).i / scale;
        /* Computing 2nd power */
        r__2 = c_abs_0(&mut q__2) as real;
        norm =
            (scale as libc::c_double *
                 sqrt((r__1 * r__1 + r__2 * r__2) as libc::c_double)) as real;
        r__1 = c_abs_0(ca) as real;
        q__1.r = (*ca).r / r__1;
        q__1.i = (*ca).i / r__1;
        alpha.r = q__1.r;
        alpha.i = q__1.i;
        *c__ = (c_abs_0(ca) / norm as libc::c_double) as real;
        r_cnjg_0(&mut q__3, cb);
        q__2.r = alpha.r * q__3.r - alpha.i * q__3.i;
        q__2.i = alpha.r * q__3.i + alpha.i * q__3.r;
        q__1.r = q__2.r / norm;
        q__1.i = q__2.i / norm;
        (*s).r = q__1.r;
        (*s).i = q__1.i;
        q__1.r = norm * alpha.r;
        q__1.i = norm * alpha.i;
        (*ca).r = q__1.r;
        (*ca).i = q__1.i
    } else {
        *c__ = 0.0f32;
        (*s).r = 1.0f32;
        (*s).i = 0.0f32;
        (*ca).r = (*cb).r;
        (*ca).i = (*cb).i
    }
    return 0 as libc::c_int;
}
/* crotg_ */
