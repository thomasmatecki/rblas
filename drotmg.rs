use ::libc;
/* f2c.h  --  Standard Fortran to C header file */
/* *  barf  [ba:rf]  2.  "He suggested using FORTRAN, and everybody barfed."

	- From The Shogakukan DICTIONARY OF NEW ENGLISH (Second edition) */
pub type integer = libc::c_long;
pub type doublereal = libc::c_double;
/* drotmg.f -- translated by f2c (version 20061008).
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
pub unsafe extern "C" fn f2c_drotmg(mut dd1: *mut doublereal,
                                    mut dd2: *mut doublereal,
                                    mut dx1: *mut doublereal,
                                    mut dy1: *mut doublereal,
                                    mut dparam: *mut doublereal)
 -> libc::c_int {
    let mut current_block: u64;
    /* Initialized data */
    static mut zero: doublereal = 0.0f64;
    static mut one: doublereal = 1.0f64;
    static mut two: doublereal = 2.0f64;
    static mut gam: doublereal = 4096.0f64;
    static mut gamsq: doublereal = 16777216.0f64;
    static mut rgamsq: doublereal = 5.9604645e-8f64;
    /* Format strings */
    static mut fmt_120: [libc::c_char; 1] = [0];
    static mut fmt_150: [libc::c_char; 1] = [0];
    static mut fmt_180: [libc::c_char; 1] = [0];
    static mut fmt_210: [libc::c_char; 1] = [0];
    /* System generated locals */
    let mut d__1: doublereal = 0.;
    /* Local variables */
    let mut du: doublereal = 0.;
    let mut dp1: doublereal = 0.;
    let mut dp2: doublereal = 0.;
    let mut dq1: doublereal = 0.;
    let mut dq2: doublereal = 0.;
    let mut dh11: doublereal = 0.;
    let mut dh12: doublereal = 0.;
    let mut dh21: doublereal = 0.;
    let mut dh22: doublereal = 0.;
    let mut igo: integer = 0;
    let mut dflag: doublereal = 0.;
    let mut dtemp: doublereal = 0.;
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
/*     THE SECOND COMPONENT OF THE 2-VECTOR  (DSQRT(DD1)*DX1,DSQRT(DD2)* */
/*     DY2)**T. */
/*     WITH DPARAM(1)=DFLAG, H HAS ONE OF THE FOLLOWING FORMS.. */
    /*     DFLAG=-1.D0     DFLAG=0.D0        DFLAG=1.D0     DFLAG=-2.D0 */
    /*       (DH11  DH12)    (1.D0  DH12)    (DH11  1.D0)    (1.D0  0.D0) */
/*     H=(          )    (          )    (          )    (          ) */
/*       (DH21  DH22),   (DH21  1.D0),   (-1.D0 DH22),   (0.D0  1.D0). */
/*     LOCATIONS 2-4 OF DPARAM CONTAIN DH11, DH21, DH12, AND DH22 */
/*     RESPECTIVELY. (VALUES OF 1.D0, -1.D0, OR 0.D0 IMPLIED BY THE */
/*     VALUE OF DPARAM(1) ARE NOT STORED IN DPARAM.) */
    /*     THE VALUES OF GAMSQ AND RGAMSQ SET IN THE DATA STATEMENT MAY BE */
/*     INEXACT.  THIS IS OK AS THEY ARE ONLY USED FOR TESTING THE SIZE */
/*     OF DD1 AND DD2.  ALL ACTUAL SCALING OF DATA IS DONE USING GAM. */
    /*  Arguments */
/*  ========= */
    /*  DD1    (input/output) DOUBLE PRECISION */
    /*  DD2    (input/output) DOUBLE PRECISION */
    /*  DX1    (input/output) DOUBLE PRECISION */
    /*  DY1    (input) DOUBLE PRECISION */
    /*  DPARAM (input/output)  DOUBLE PRECISION array, dimension 5 */
/*     DPARAM(1)=DFLAG */
/*     DPARAM(2)=DH11 */
/*     DPARAM(3)=DH21 */
/*     DPARAM(4)=DH12 */
/*     DPARAM(5)=DH22 */
    /*  ===================================================================== */
    /*     .. Local Scalars .. */
/*     .. */
/*     .. Intrinsic Functions .. */
/*     .. */
/*     .. Data statements .. */
    /* Parameter adjustments */
    dparam = dparam.offset(-1);
    /* Function Body */
/*     .. */
    if !(*dd1 < zero) {
        /*     CASE-DD1-NONNEGATIVE */
        dp2 = *dd2 * *dy1;
        if !(dp2 == zero) {
            /*     REGULAR-CASE.. */
            dp1 = *dd1 * *dx1;
            dq2 = dp2 * *dy1;
            dq1 = dp1 * *dx1;
            if !((if dq1 >= 0 as libc::c_int as libc::c_double {
                      dq1
                  } else { -dq1 }) >
                     (if dq2 >= 0 as libc::c_int as libc::c_double {
                          dq2
                      } else { -dq2 })) {
                if !(dq2 < zero) {
                    dflag = one;
                    dh11 = dp1 / dp2;
                    dh22 = *dx1 / *dy1;
                    du = one + dh11 * dh22;
                    dtemp = *dd2 / du;
                    *dd2 = *dd1 / du;
                    *dd1 = dtemp;
                    *dx1 = *dy1 * du;
                    /*         GO SCALE-CHECK */
                    current_block = 14316613746598039475;
                } else {
                    /*         GO ZERO-H-D-AND-DX1.. */
                    current_block = 1219181519538965718;
                }
            } else {
                dh21 = -*dy1 / *dx1;
                dh12 = dp2 / dp1;
                du = one - dh12 * dh21;
                if !(du <= zero) {
                    dflag = zero;
                    *dd1 /= du;
                    *dd2 /= du;
                    *dx1 *= du;
                    /*         GO SCALE-CHECK.. */
                    current_block = 14316613746598039475;
                } else {
                    /*         GO ZERO-H-D-AND-DX1.. */
                    current_block = 1219181519538965718;
                }
            }
            match current_block {
                1219181519538965718 => { }
                _ => {
                    'c_670:
                        loop 
                             /*     PROCEDURE..SCALE-CHECK */
                             {
                            if !(*dd1 <= rgamsq) {
                                current_block = 9361680084571800209;
                            } else if *dd1 == zero {
                                current_block = 9432909789281843485;
                            } else {
                                igo = 0 as libc::c_int as integer;
                                igo_fmt = fmt_120.as_mut_ptr();
                                /*              FIX-H.. */
                                current_block = 16457600364537029179;
                            }
                            loop  {
                                match current_block {
                                    9361680084571800209 => {
                                        if !(*dd1 >= gamsq) {
                                            current_block =
                                                9432909789281843485;
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
                                        if dflag >= zero {
                                            if !(dflag == zero) {
                                                dh21 = -one;
                                                dh12 = one;
                                                dflag = -one
                                            } else {
                                                dh11 = one;
                                                dh22 = one;
                                                dflag = -one
                                            }
                                        }
                                        match igo {
                                            0 => { break ; }
                                            1 => {
                                                /* Computing 2nd power */
                                                d__1 = gam;
                                                *dd1 /= d__1 * d__1;
                                                *dx1 *= gam;
                                                dh11 *= gam;
                                                dh12 *= gam;
                                                current_block =
                                                    9361680084571800209;
                                                continue ;
                                            }
                                            2 => {
                                                /* Computing 2nd power */
                                                d__1 = gam;
                                                *dd2 *= d__1 * d__1;
                                                dh21 /= gam;
                                                dh22 /= gam;
                                                current_block =
                                                    9432909789281843485;
                                                continue ;
                                            }
                                            3 => {
                                                /* Computing 2nd power */
                                                d__1 = gam;
                                                *dd2 /= d__1 * d__1;
                                                dh21 *= gam;
                                                dh22 *= gam
                                            }
                                            _ => { continue 'c_670 ; }
                                        }
                                    }
                                    _ => {
                                        if (if *dd2 >=
                                                   0 as libc::c_int as
                                                       libc::c_double {
                                                *dd2
                                            } else { -*dd2 }) <= rgamsq {
                                            if *dd2 == zero { break 'c_670 ; }
                                            igo = 2 as libc::c_int as integer;
                                            igo_fmt = fmt_180.as_mut_ptr();
                                            /*              FIX-H.. */
                                            current_block =
                                                16457600364537029179;
                                            continue ;
                                        }
                                    }
                                }
                                if !((if *dd2 >=
                                             0 as libc::c_int as
                                                 libc::c_double {
                                          *dd2
                                      } else { -*dd2 }) >= gamsq) {
                                    break 'c_670 ;
                                }
                                igo = 3 as libc::c_int as integer;
                                igo_fmt = fmt_210.as_mut_ptr();
                                current_block = 16457600364537029179;
                            }
                            /* Computing 2nd power */
                            d__1 = gam;
                            *dd1 *= d__1 * d__1;
                            *dx1 /= gam;
                            dh11 /= gam;
                            dh12 /= gam
                        }
                    current_block = 11662017948461917171;
                }
            }
        } else { dflag = -two; current_block = 17685814418459604237; }
    } else {
        /*       GO ZERO-H-D-AND-DX1.. */
        current_block = 1219181519538965718;
    }
    match current_block {
        1219181519538965718 =>
        /*     PROCEDURE..ZERO-H-D-AND-DX1.. */
        {
            dflag = -one;
            dh11 = zero;
            dh12 = zero;
            dh21 = zero;
            dh22 = zero;
            *dd1 = zero;
            *dd2 = zero;
            *dx1 = zero;
            current_block = 11662017948461917171;
        }
        _ => { }
    }
    match current_block {
        11662017948461917171 =>
        /*         RETURN.. */
        {
            if dflag < 0.0f64 {
                *dparam.offset(2 as libc::c_int as isize) = dh11;
                *dparam.offset(3 as libc::c_int as isize) = dh21;
                *dparam.offset(4 as libc::c_int as isize) = dh12;
                *dparam.offset(5 as libc::c_int as isize) = dh22
            } else if dflag == 0 as libc::c_int as libc::c_double {
                *dparam.offset(3 as libc::c_int as isize) = dh21;
                *dparam.offset(4 as libc::c_int as isize) = dh12
            } else {
                *dparam.offset(2 as libc::c_int as isize) = dh11;
                *dparam.offset(5 as libc::c_int as isize) = dh22
            }
        }
        _ => { }
    }
    *dparam.offset(1 as libc::c_int as isize) = dflag;
    return 0 as libc::c_int;
}
/* drotmg_ */
