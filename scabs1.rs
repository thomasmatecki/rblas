use ::libc;
pub type real = libc::c_float;
pub type doublereal = libc::c_double;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct complex {
    pub r: real,
    pub i: real,
}
/* scabs1.f -- translated by f2c (version 20061008).
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
pub unsafe extern "C" fn scabs1_(mut z__: *mut complex) -> doublereal {
    /* System generated locals */
    let mut ret_val: real = 0.;
    let mut r__1: real = 0.;
    let mut r__2: real = 0.;
    /* Builtin functions */
    extern "C" {
        #[link_name = "r_imag"]
        fn r_imag_0(_: *mut complex) -> libc::c_double;
    }
    /*     .. Scalar Arguments .. */
/*     .. */
    /*  Purpose */
/*  ======= */
    /*  SCABS1 computes absolute value of a complex number */
    /*     .. Intrinsic Functions .. */
/*     .. */
    r__1 = (*z__).r;
    r__2 = r_imag_0(z__) as real;
    ret_val =
        ((if r__1 >= 0 as libc::c_int as libc::c_float {
              r__1
          } else { -r__1 }) as doublereal +
             (if r__2 >= 0 as libc::c_int as libc::c_float {
                  r__2
              } else { -r__2 }) as doublereal) as real;
    return ret_val as doublereal;
}
/* scabs1_ */
