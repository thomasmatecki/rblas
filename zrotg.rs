use ::libc;
extern "C" {
    #[no_mangle]
    fn sqrt(_: libc::c_double) -> libc::c_double;
}
pub type doublereal = libc::c_double;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct doublecomplex {
    pub r: doublereal,
    pub i: doublereal,
}
/* zrotg.f -- translated by f2c (version 20061008).
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
pub unsafe extern "C" fn f2c_zrotg(mut ca: *mut doublecomplex,
                                   mut cb: *mut doublecomplex,
                                   mut c__: *mut doublereal,
                                   mut s: *mut doublecomplex) -> libc::c_int {
    /* System generated locals */
    let mut d__1: doublereal = 0.;
    let mut d__2: doublereal = 0.;
    let mut z__1: doublecomplex = doublecomplex{r: 0., i: 0.,};
    let mut z__2: doublecomplex = doublecomplex{r: 0., i: 0.,};
    let mut z__3: doublecomplex = doublecomplex{r: 0., i: 0.,};
    let mut z__4: doublecomplex = doublecomplex{r: 0., i: 0.,};
    /* Builtin functions */
    extern "C" {
        #[link_name = "z_abs"]
        fn z_abs_0(_: *mut doublecomplex) -> libc::c_double;
    }
    extern "C" {
        #[link_name = "z_div"]
        fn z_div_0(_: *mut doublecomplex, _: *mut doublecomplex,
                   _: *mut doublecomplex);
    }
    extern "C" {
        #[link_name = "d_cnjg"]
        fn d_cnjg_0(_: *mut doublecomplex, _: *mut doublecomplex);
    }
    /* Local variables */
    let mut norm: doublereal = 0.;
    let mut alpha: doublecomplex = doublecomplex{r: 0., i: 0.,};
    let mut scale: doublereal = 0.;
    /*     .. Scalar Arguments .. */
/*     .. */
    /*  Purpose */
/*  ======= */
    /*     determines a double complex Givens rotation. */
    /*     .. Local Scalars .. */
/*     .. */
/*     .. Intrinsic Functions .. */
/*     .. */
    if z_abs_0(ca) != 0.0f64 {
        scale = z_abs_0(ca) + z_abs_0(cb);
        z__2.r = scale;
        z__2.i = 0.0f64;
        z_div_0(&mut z__1, ca, &mut z__2);
        /* Computing 2nd power */
        d__1 = z_abs_0(&mut z__1);
        z__4.r = scale;
        z__4.i = 0.0f64;
        z_div_0(&mut z__3, cb, &mut z__4);
        /* Computing 2nd power */
        d__2 = z_abs_0(&mut z__3);
        norm = scale * sqrt(d__1 * d__1 + d__2 * d__2);
        d__1 = z_abs_0(ca);
        z__1.r = (*ca).r / d__1;
        z__1.i = (*ca).i / d__1;
        alpha.r = z__1.r;
        alpha.i = z__1.i;
        *c__ = z_abs_0(ca) / norm;
        d_cnjg_0(&mut z__3, cb);
        z__2.r = alpha.r * z__3.r - alpha.i * z__3.i;
        z__2.i = alpha.r * z__3.i + alpha.i * z__3.r;
        z__1.r = z__2.r / norm;
        z__1.i = z__2.i / norm;
        (*s).r = z__1.r;
        (*s).i = z__1.i;
        z__1.r = norm * alpha.r;
        z__1.i = norm * alpha.i;
        (*ca).r = z__1.r;
        (*ca).i = z__1.i
    } else {
        *c__ = 0.0f64;
        (*s).r = 1.0f64;
        (*s).i = 0.0f64;
        (*ca).r = (*cb).r;
        (*ca).i = (*cb).i
    }
    return 0 as libc::c_int;
}
/* zrotg_ */
