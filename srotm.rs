use ::libc;
/* f2c.h  --  Standard Fortran to C header file */
/* *  barf  [ba:rf]  2.  "He suggested using FORTRAN, and everybody barfed."

	- From The Shogakukan DICTIONARY OF NEW ENGLISH (Second edition) */
pub type integer = libc::c_long;
pub type real = libc::c_float;
/* srotm.f -- translated by f2c (version 20061008).
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
pub unsafe extern "C" fn f2c_srotm(mut n: *mut integer, mut sx: *mut real,
                                   mut incx: *mut integer, mut sy: *mut real,
                                   mut incy: *mut integer,
                                   mut sparam: *mut real) -> libc::c_int {
    /* Initialized data */
    static mut zero: real = 0.0f32;
    static mut two: real = 2.0f32;
    /* System generated locals */
    let mut i__1: integer = 0;
    let mut i__2: integer = 0;
    /* Local variables */
    let mut i__: integer = 0;
    let mut w: real = 0.;
    let mut z__: real = 0.;
    let mut kx: integer = 0;
    let mut ky: integer = 0;
    let mut sh11: real = 0.;
    let mut sh12: real = 0.;
    let mut sh21: real = 0.;
    let mut sh22: real = 0.;
    let mut sflag: real = 0.;
    let mut nsteps: integer = 0;
    /*     .. Scalar Arguments .. */
/*     .. */
/*     .. Array Arguments .. */
/*     .. */
    /*  Purpose */
/*  ======= */
    /*     APPLY THE MODIFIED GIVENS TRANSFORMATION, H, TO THE 2 BY N MATRIX */
    /*     (SX**T) , WHERE **T INDICATES TRANSPOSE. THE ELEMENTS OF SX ARE IN */
/*     (DX**T) */
    /*     SX(LX+I*INCX), I = 0 TO N-1, WHERE LX = 1 IF INCX .GE. 0, ELSE */
/*     LX = (-INCX)*N, AND SIMILARLY FOR SY USING USING LY AND INCY. */
/*     WITH SPARAM(1)=SFLAG, H HAS ONE OF THE FOLLOWING FORMS.. */
    /*     SFLAG=-1.E0     SFLAG=0.E0        SFLAG=1.E0     SFLAG=-2.E0 */
    /*       (SH11  SH12)    (1.E0  SH12)    (SH11  1.E0)    (1.E0  0.E0) */
/*     H=(          )    (          )    (          )    (          ) */
/*       (SH21  SH22),   (SH21  1.E0),   (-1.E0 SH22),   (0.E0  1.E0). */
/*     SEE  SROTMG FOR A DESCRIPTION OF DATA STORAGE IN SPARAM. */
    /*  Arguments */
/*  ========= */
    /*  N      (input) INTEGER */
/*         number of elements in input vector(s) */
    /*  SX     (input/output) REAL array, dimension N */
/*         double precision vector with N elements */
    /*  INCX   (input) INTEGER */
/*         storage spacing between elements of SX */
    /*  SY     (input/output) REAL array, dimension N */
/*         double precision vector with N elements */
    /*  INCY   (input) INTEGER */
/*         storage spacing between elements of SY */
    /*  SPARAM (input/output)  REAL array, dimension 5 */
/*     SPARAM(1)=SFLAG */
/*     SPARAM(2)=SH11 */
/*     SPARAM(3)=SH21 */
/*     SPARAM(4)=SH12 */
/*     SPARAM(5)=SH22 */
    /*  ===================================================================== */
    /*     .. Local Scalars .. */
/*     .. */
/*     .. Data statements .. */
    /* Parameter adjustments */
    sparam = sparam.offset(-1);
    sy = sy.offset(-1);
    sx = sx.offset(-1);
    /* Function Body */
/*     .. */
    sflag = *sparam.offset(1 as libc::c_int as isize);
    if !(*n <= 0 as libc::c_int as libc::c_long || sflag + two == zero) {
        if !(*incx == *incy && *incx > 0 as libc::c_int as libc::c_long) {
            kx = 1 as libc::c_int as integer;
            ky = 1 as libc::c_int as integer;
            if *incx < 0 as libc::c_int as libc::c_long {
                kx =
                    (1 as libc::c_int as libc::c_long - *n) * *incx +
                        1 as libc::c_int as libc::c_long
            }
            if *incy < 0 as libc::c_int as libc::c_long {
                ky =
                    (1 as libc::c_int as libc::c_long - *n) * *incy +
                        1 as libc::c_int as libc::c_long
            }
            if sflag < 0.0f32 {
                sh11 = *sparam.offset(2 as libc::c_int as isize);
                sh12 = *sparam.offset(4 as libc::c_int as isize);
                sh21 = *sparam.offset(3 as libc::c_int as isize);
                sh22 = *sparam.offset(5 as libc::c_int as isize);
                i__2 = *n;
                i__ = 1 as libc::c_int as integer;
                while i__ <= i__2 {
                    w = *sx.offset(kx as isize);
                    z__ = *sy.offset(ky as isize);
                    *sx.offset(kx as isize) = w * sh11 + z__ * sh12;
                    *sy.offset(ky as isize) = w * sh21 + z__ * sh22;
                    kx += *incx;
                    ky += *incy;
                    i__ += 1
                    /* L130: */
                }
            } else if sflag == 0 as libc::c_int as libc::c_float {
                sh12 = *sparam.offset(4 as libc::c_int as isize);
                sh21 = *sparam.offset(3 as libc::c_int as isize);
                i__2 = *n;
                i__ = 1 as libc::c_int as integer;
                while i__ <= i__2 {
                    w = *sx.offset(kx as isize);
                    z__ = *sy.offset(ky as isize);
                    *sx.offset(kx as isize) = w + z__ * sh12;
                    *sy.offset(ky as isize) = w * sh21 + z__;
                    kx += *incx;
                    ky += *incy;
                    i__ += 1
                    /* L90: */
                }
            } else {
                sh11 = *sparam.offset(2 as libc::c_int as isize);
                sh22 = *sparam.offset(5 as libc::c_int as isize);
                i__2 = *n;
                i__ = 1 as libc::c_int as integer;
                while i__ <= i__2 {
                    w = *sx.offset(kx as isize);
                    z__ = *sy.offset(ky as isize);
                    *sx.offset(kx as isize) = w * sh11 + z__;
                    *sy.offset(ky as isize) = -w + sh22 * z__;
                    kx += *incx;
                    ky += *incy;
                    i__ += 1
                    /* L110: */
                }
            }
        } else {
            nsteps = *n * *incx;
            if sflag < 0.0f32 {
                sh11 = *sparam.offset(2 as libc::c_int as isize);
                sh12 = *sparam.offset(4 as libc::c_int as isize);
                sh21 = *sparam.offset(3 as libc::c_int as isize);
                sh22 = *sparam.offset(5 as libc::c_int as isize);
                i__1 = nsteps;
                i__2 = *incx;
                i__ = 1 as libc::c_int as integer;
                while if i__2 < 0 as libc::c_int as libc::c_long {
                          (i__ >= i__1) as libc::c_int
                      } else { (i__ <= i__1) as libc::c_int } != 0 {
                    w = *sx.offset(i__ as isize);
                    z__ = *sy.offset(i__ as isize);
                    *sx.offset(i__ as isize) = w * sh11 + z__ * sh12;
                    *sy.offset(i__ as isize) = w * sh21 + z__ * sh22;
                    i__ += i__2
                    /* L60: */
                }
            } else if sflag == 0 as libc::c_int as libc::c_float {
                sh12 = *sparam.offset(4 as libc::c_int as isize);
                sh21 = *sparam.offset(3 as libc::c_int as isize);
                i__1 = nsteps;
                i__2 = *incx;
                i__ = 1 as libc::c_int as integer;
                while if i__2 < 0 as libc::c_int as libc::c_long {
                          (i__ >= i__1) as libc::c_int
                      } else { (i__ <= i__1) as libc::c_int } != 0 {
                    w = *sx.offset(i__ as isize);
                    z__ = *sy.offset(i__ as isize);
                    *sx.offset(i__ as isize) = w + z__ * sh12;
                    *sy.offset(i__ as isize) = w * sh21 + z__;
                    i__ += i__2
                    /* L20: */
                }
            } else {
                sh11 = *sparam.offset(2 as libc::c_int as isize);
                sh22 = *sparam.offset(5 as libc::c_int as isize);
                i__2 = nsteps;
                i__1 = *incx;
                i__ = 1 as libc::c_int as integer;
                while if i__1 < 0 as libc::c_int as libc::c_long {
                          (i__ >= i__2) as libc::c_int
                      } else { (i__ <= i__2) as libc::c_int } != 0 {
                    w = *sx.offset(i__ as isize);
                    z__ = *sy.offset(i__ as isize);
                    *sx.offset(i__ as isize) = w * sh11 + z__;
                    *sy.offset(i__ as isize) = -w + sh22 * z__;
                    i__ += i__1
                    /* L40: */
                }
            }
        }
    }
    return 0 as libc::c_int;
}
/* srotm_ */
