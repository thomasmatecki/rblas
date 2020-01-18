use ::libc;
/* f2c.h  --  Standard Fortran to C header file */
/* *  barf  [ba:rf]  2.  "He suggested using FORTRAN, and everybody barfed."

	- From The Shogakukan DICTIONARY OF NEW ENGLISH (Second edition) */
pub type integer = libc::c_long;
pub type real = libc::c_float;
pub type doublereal = libc::c_double;
/* srotmg.f -- translated by f2c (version 20061008).
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
pub unsafe extern "C" fn f2c_srotmg(mut sd1: *mut real, mut sd2: *mut real,
                                    mut sx1: *mut real, mut sy1: *mut real,
                                    mut sparam: *mut real) -> libc::c_int {
    let mut current_block: u64;
    /* Initialized data */
    static mut zero: real = 0.0f32;
    static mut one: real = 1.0f32;
    static mut two: real = 2.0f32;
    static mut gam: real = 4096.0f32;
    static mut gamsq: real = 16777200.0f32;
    static mut rgamsq: real = 5.96046e-8f32;
    /* Format strings */
    static mut fmt_120: [libc::c_char; 1] = [0];
    static mut fmt_150: [libc::c_char; 1] = [0];
    static mut fmt_180: [libc::c_char; 1] = [0];
    static mut fmt_210: [libc::c_char; 1] = [0];
    /* System generated locals */
    let mut r__1: real = 0.;
    /* Local variables */
    let mut su: real = 0.;
    let mut sp1: real = 0.;
    let mut sp2: real = 0.;
    let mut sq1: real = 0.;
    let mut sq2: real = 0.;
    let mut sh11: real = 0.;
    let mut sh12: real = 0.;
    let mut sh21: real = 0.;
    let mut sh22: real = 0.;
    let mut igo: integer = 0;
    let mut sflag: real = 0.;
    let mut stemp: real = 0.;
    /* Assigned format variables */
    static mut igo_fmt: *mut libc::c_char =
        0 as *const libc::c_char as *mut libc::c_char;
    /*     .. Scalar Arguments .. */
/*     .. */
/*     .. Array Arguments .. */
/*     .. */
    /*  Purpose */
/*  ======= */
    /*     CONSTRUCT THE MODIFIED GIVENS TRANSFORMATION MATRIX H WHICH ZEROS */
/*     THE SECOND COMPONENT OF THE 2-VECTOR  (SQRT(SD1)*SX1,SQRT(SD2)* */
/*     SY2)**T. */
/*     WITH SPARAM(1)=SFLAG, H HAS ONE OF THE FOLLOWING FORMS.. */
    /*     SFLAG=-1.E0     SFLAG=0.E0        SFLAG=1.E0     SFLAG=-2.E0 */
    /*       (SH11  SH12)    (1.E0  SH12)    (SH11  1.E0)    (1.E0  0.E0) */
/*     H=(          )    (          )    (          )    (          ) */
/*       (SH21  SH22),   (SH21  1.E0),   (-1.E0 SH22),   (0.E0  1.E0). */
/*     LOCATIONS 2-4 OF SPARAM CONTAIN SH11,SH21,SH12, AND SH22 */
/*     RESPECTIVELY. (VALUES OF 1.E0, -1.E0, OR 0.E0 IMPLIED BY THE */
/*     VALUE OF SPARAM(1) ARE NOT STORED IN SPARAM.) */
    /*     THE VALUES OF GAMSQ AND RGAMSQ SET IN THE DATA STATEMENT MAY BE */
/*     INEXACT.  THIS IS OK AS THEY ARE ONLY USED FOR TESTING THE SIZE */
/*     OF SD1 AND SD2.  ALL ACTUAL SCALING OF DATA IS DONE USING GAM. */
    /*  Arguments */
/*  ========= */
    /*  SD1    (input/output) REAL */
    /*  SD2    (input/output) REAL */
    /*  SX1    (input/output) REAL */
    /*  SY1    (input) REAL */
    /*  SPARAM (input/output)  REAL array, dimension 5 */
/*     SPARAM(1)=SFLAG */
/*     SPARAM(2)=SH11 */
/*     SPARAM(3)=SH21 */
/*     SPARAM(4)=SH12 */
/*     SPARAM(5)=SH22 */
    /*  ===================================================================== */
    /*     .. Local Scalars .. */
/*     .. */
/*     .. Intrinsic Functions .. */
/*     .. */
/*     .. Data statements .. */
    /* Parameter adjustments */
    sparam = sparam.offset(-1);
    /* Function Body */
/*     .. */
    if !(*sd1 < zero) {
        /*     CASE-SD1-NONNEGATIVE */
        sp2 = *sd2 * *sy1;
        if !(sp2 == zero) {
            /*     REGULAR-CASE.. */
            sp1 = *sd1 * *sx1;
            sq2 = sp2 * *sy1;
            sq1 = sp1 * *sx1;
            if !((if sq1 >= 0 as libc::c_int as libc::c_float {
                      sq1
                  } else { -sq1 }) as doublereal >
                     (if sq2 >= 0 as libc::c_int as libc::c_float {
                          sq2
                      } else { -sq2 }) as doublereal) {
                if !(sq2 < zero) {
                    sflag = one;
                    sh11 = sp1 / sp2;
                    sh22 = *sx1 / *sy1;
                    su = one + sh11 * sh22;
                    stemp = *sd2 / su;
                    *sd2 = *sd1 / su;
                    *sd1 = stemp;
                    *sx1 = *sy1 * su;
                    /*         GO SCALE-CHECK */
                    current_block = 16648156689382489725;
                } else {
                    /*         GO ZERO-H-D-AND-SX1.. */
                    current_block = 1219181519538965718;
                }
            } else {
                sh21 = -*sy1 / *sx1;
                sh12 = sp2 / sp1;
                su = one - sh12 * sh21;
                if !(su <= zero) {
                    sflag = zero;
                    *sd1 /= su;
                    *sd2 /= su;
                    *sx1 *= su;
                    /*         GO SCALE-CHECK.. */
                    current_block = 16648156689382489725;
                } else {
                    /*         GO ZERO-H-D-AND-SX1.. */
                    current_block = 1219181519538965718;
                }
            }
            match current_block {
                1219181519538965718 => { }
                _ => {
                    'c_674:
                        loop 
                             /*     PROCEDURE..SCALE-CHECK */
                             {
                            if !(*sd1 <= rgamsq) {
                                current_block = 5996064384694841776;
                            } else if *sd1 == zero {
                                current_block = 17542745072233027991;
                            } else {
                                igo = 0 as libc::c_int as integer;
                                igo_fmt = fmt_120.as_mut_ptr();
                                /*              FIX-H.. */
                                current_block = 16457600364537029179;
                            }
                            loop  {
                                match current_block {
                                    5996064384694841776 => {
                                        if !(*sd1 >= gamsq) {
                                            current_block =
                                                17542745072233027991;
                                            continue ;
                                        }
                                        igo = 1 as libc::c_int as integer;
                                        igo_fmt = fmt_150.as_mut_ptr();
                                        /*              FIX-H.. */
                                        current_block = 16457600364537029179;
                                        continue ;
                                    }
                                    16457600364537029179 =>
                                    /*              FIX-H.. */
                                    {
                                        if sflag >= zero {
                                            if !(sflag == zero) {
                                                sh21 = -one;
                                                sh12 = one;
                                                sflag = -one
                                            } else {
                                                sh11 = one;
                                                sh22 = one;
                                                sflag = -one
                                            }
                                        }
                                        match igo {
                                            0 => { break ; }
                                            1 => {
                                                /* Computing 2nd power */
                                                r__1 = gam;
                                                *sd1 /= r__1 * r__1;
                                                *sx1 *= gam;
                                                sh11 *= gam;
                                                sh12 *= gam;
                                                current_block =
                                                    5996064384694841776;
                                                continue ;
                                            }
                                            2 => {
                                                /* Computing 2nd power */
                                                r__1 = gam;
                                                *sd2 *= r__1 * r__1;
                                                sh21 /= gam;
                                                sh22 /= gam;
                                                current_block =
                                                    17542745072233027991;
                                                continue ;
                                            }
                                            3 => {
                                                /* Computing 2nd power */
                                                r__1 = gam;
                                                *sd2 /= r__1 * r__1;
                                                sh21 *= gam;
                                                sh22 *= gam
                                            }
                                            _ => { continue 'c_674 ; }
                                        }
                                    }
                                    _ => {
                                        if (if *sd2 >=
                                                   0 as libc::c_int as
                                                       libc::c_float {
                                                *sd2
                                            } else { -*sd2 }) as doublereal <=
                                               rgamsq as libc::c_double {
                                            if *sd2 == zero { break 'c_674 ; }
                                            igo = 2 as libc::c_int as integer;
                                            igo_fmt = fmt_180.as_mut_ptr();
                                            /*              FIX-H.. */
                                            current_block =
                                                16457600364537029179;
                                            continue ;
                                        }
                                    }
                                }
                                if !((if *sd2 >=
                                             0 as libc::c_int as libc::c_float
                                         {
                                          *sd2
                                      } else { -*sd2 }) as doublereal >=
                                         gamsq as libc::c_double) {
                                    break 'c_674 ;
                                }
                                igo = 3 as libc::c_int as integer;
                                igo_fmt = fmt_210.as_mut_ptr();
                                current_block = 16457600364537029179;
                            }
                            /* Computing 2nd power */
                            r__1 = gam;
                            *sd1 *= r__1 * r__1;
                            *sx1 /= gam;
                            sh11 /= gam;
                            sh12 /= gam
                        }
                    current_block = 11662017948461917171;
                }
            }
        } else { sflag = -two; current_block = 17685814418459604237; }
    } else {
        /*       GO ZERO-H-D-AND-SX1.. */
        current_block = 1219181519538965718;
    }
    match current_block {
        1219181519538965718 =>
        /*     PROCEDURE..ZERO-H-D-AND-SX1.. */
        {
            sflag = -one;
            sh11 = zero;
            sh12 = zero;
            sh21 = zero;
            sh22 = zero;
            *sd1 = zero;
            *sd2 = zero;
            *sx1 = zero;
            current_block = 11662017948461917171;
        }
        _ => { }
    }
    match current_block {
        11662017948461917171 =>
        /*         RETURN.. */
        {
            if sflag < 0.0f32 {
                *sparam.offset(2 as libc::c_int as isize) = sh11;
                *sparam.offset(3 as libc::c_int as isize) = sh21;
                *sparam.offset(4 as libc::c_int as isize) = sh12;
                *sparam.offset(5 as libc::c_int as isize) = sh22
            } else if sflag == 0 as libc::c_int as libc::c_float {
                *sparam.offset(3 as libc::c_int as isize) = sh21;
                *sparam.offset(4 as libc::c_int as isize) = sh12
            } else {
                *sparam.offset(2 as libc::c_int as isize) = sh11;
                *sparam.offset(5 as libc::c_int as isize) = sh22
            }
        }
        _ => { }
    }
    *sparam.offset(1 as libc::c_int as isize) = sflag;
    return 0 as libc::c_int;
}
/* srotmg_ */
