use ::libc;
pub type doublereal = libc::c_double;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct doublecomplex {
    pub r: doublereal,
    pub i: doublereal,
}
/* dcabs1.f -- translated by f2c (version 20061008).
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
pub unsafe extern "C" fn dcabs1_(mut z__: *mut doublecomplex) -> doublereal {
    /* System generated locals */
    let mut ret_val: doublereal = 0.;
    let mut d__1: doublereal = 0.;
    let mut d__2: doublereal = 0.;
    /* Builtin functions */
    extern "C" {
        #[link_name = "d_imag"]
        fn d_imag_0(_: *mut doublecomplex) -> libc::c_double;
    }
    /*     .. Scalar Arguments .. */
/*     .. */
/*     .. */
/*  Purpose */
/*  ======= */
    /*  DCABS1 computes absolute value of a double complex number */
    /*     .. Intrinsic Functions .. */
    d__1 = (*z__).r;
    d__2 = d_imag_0(z__);
    ret_val =
        (if d__1 >= 0 as libc::c_int as libc::c_double {
             d__1
         } else { -d__1 }) +
            (if d__2 >= 0 as libc::c_int as libc::c_double {
                 d__2
             } else { -d__2 });
    return ret_val;
}
/* dcabs1_ */
