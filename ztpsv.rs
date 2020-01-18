use libc;
/* f2c.h  --  Standard Fortran to C header file */
/* *  barf  [ba:rf]  2.  "He suggested using FORTRAN, and everybody barfed."

- From The Shogakukan DICTIONARY OF NEW ENGLISH (Second edition) */
pub type integer = libc::c_long;
pub type doublereal = libc::c_double;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct doublecomplex {
    pub r: doublereal,
    pub i: doublereal,
}
pub type logical = libc::c_long;
/* ztpsv.f -- translated by f2c (version 20061008).
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
pub unsafe extern "C" fn f2c_ztpsv(
    mut uplo: *mut libc::c_char,
    mut trans: *mut libc::c_char,
    mut diag: *mut libc::c_char,
    mut n: *mut integer,
    mut ap: *mut doublecomplex,
    mut x: *mut doublecomplex,
    mut incx: *mut integer,
) -> libc::c_int {
    /* System generated locals */
    let mut i__1: integer = 0;
    let mut i__2: integer = 0;
    let mut i__3: integer = 0;
    let mut i__4: integer = 0;
    let mut i__5: integer = 0;
    let mut z__1: doublecomplex = doublecomplex { r: 0., i: 0. };
    let mut z__2: doublecomplex = doublecomplex { r: 0., i: 0. };
    let mut z__3: doublecomplex = doublecomplex { r: 0., i: 0. };
    /* Builtin functions */
    extern "C" {
        #[link_name = "z_div"]
        fn z_div_0(_: *mut doublecomplex, _: *mut doublecomplex, _: *mut doublecomplex);
    }
    extern "C" {
        #[link_name = "d_cnjg"]
        fn d_cnjg_0(_: *mut doublecomplex, _: *mut doublecomplex);
    }
    /* Local variables */
    let mut i__: integer = 0;
    let mut j: integer = 0;
    let mut k: integer = 0;
    let mut kk: integer = 0;
    let mut ix: integer = 0;
    let mut jx: integer = 0;
    let mut kx: integer = 0;
    let mut info: integer = 0;
    let mut temp: doublecomplex = doublecomplex { r: 0., i: 0. };
    extern "C" {
        #[link_name = "lsame_"]
        fn lsame__0(_: *mut libc::c_char, _: *mut libc::c_char) -> logical;
    }
    extern "C" {
        #[link_name = "xerbla_"]
        fn xerbla__0(_: *mut libc::c_char, _: *mut integer) -> libc::c_int;
    }
    let mut noconj: logical = 0;
    let mut nounit: logical = 0;
    /*     .. Scalar Arguments .. */
    /*     .. */
    /*     .. Array Arguments .. */
    /*     .. */
    /*  Purpose */
    /*  ======= */
    /*  ZTPSV  solves one of the systems of equations */
    /*     A*x = b,   or   A'*x = b,   or   conjg( A' )*x = b, */
    /*  where b and x are n element vectors and A is an n by n unit, or */
    /*  non-unit, upper or lower triangular matrix, supplied in packed form. */
    /*  No test for singularity or near-singularity is included in this */
    /*  routine. Such tests must be performed before calling this routine. */
    /*  Arguments */
    /*  ========== */
    /*  UPLO   - CHARACTER*1. */
    /*           On entry, UPLO specifies whether the matrix is an upper or */
    /*           lower triangular matrix as follows: */
    /*              UPLO = 'U' or 'u'   A is an upper triangular matrix. */
    /*              UPLO = 'L' or 'l'   A is a lower triangular matrix. */
    /*           Unchanged on exit. */
    /*  TRANS  - CHARACTER*1. */
    /*           On entry, TRANS specifies the equations to be solved as */
    /*           follows: */
    /*              TRANS = 'N' or 'n'   A*x = b. */
    /*              TRANS = 'T' or 't'   A'*x = b. */
    /*              TRANS = 'C' or 'c'   conjg( A' )*x = b. */
    /*           Unchanged on exit. */
    /*  DIAG   - CHARACTER*1. */
    /*           On entry, DIAG specifies whether or not A is unit */
    /*           triangular as follows: */
    /*              DIAG = 'U' or 'u'   A is assumed to be unit triangular. */
    /*              DIAG = 'N' or 'n'   A is not assumed to be unit */
    /*                                  triangular. */
    /*           Unchanged on exit. */
    /*  N      - INTEGER. */
    /*           On entry, N specifies the order of the matrix A. */
    /*           N must be at least zero. */
    /*           Unchanged on exit. */
    /*  AP     - COMPLEX*16       array of DIMENSION at least */
    /*           ( ( n*( n + 1 ) )/2 ). */
    /*           Before entry with  UPLO = 'U' or 'u', the array AP must */
    /*           contain the upper triangular matrix packed sequentially, */
    /*           column by column, so that AP( 1 ) contains a( 1, 1 ), */
    /*           AP( 2 ) and AP( 3 ) contain a( 1, 2 ) and a( 2, 2 ) */
    /*           respectively, and so on. */
    /*           Before entry with UPLO = 'L' or 'l', the array AP must */
    /*           contain the lower triangular matrix packed sequentially, */
    /*           column by column, so that AP( 1 ) contains a( 1, 1 ), */
    /*           AP( 2 ) and AP( 3 ) contain a( 2, 1 ) and a( 3, 1 ) */
    /*           respectively, and so on. */
    /*           Note that when  DIAG = 'U' or 'u', the diagonal elements of */
    /*           A are not referenced, but are assumed to be unity. */
    /*           Unchanged on exit. */
    /*  X      - COMPLEX*16       array of dimension at least */
    /*           ( 1 + ( n - 1 )*abs( INCX ) ). */
    /*           Before entry, the incremented array X must contain the n */
    /*           element right-hand side vector b. On exit, X is overwritten */
    /*           with the solution vector x. */
    /*  INCX   - INTEGER. */
    /*           On entry, INCX specifies the increment for the elements of */
    /*           X. INCX must not be zero. */
    /*           Unchanged on exit. */
    /*  Level 2 Blas routine. */
    /*  -- Written on 22-October-1986. */
    /*     Jack Dongarra, Argonne National Lab. */
    /*     Jeremy Du Croz, Nag Central Office. */
    /*     Sven Hammarling, Nag Central Office. */
    /*     Richard Hanson, Sandia National Labs. */
    /*     .. Parameters .. */
    /*     .. */
    /*     .. Local Scalars .. */
    /*     .. */
    /*     .. External Functions .. */
    /*     .. */
    /*     .. External Subroutines .. */
    /*     .. */
    /*     .. Intrinsic Functions .. */
    /*     .. */
    /*     Test the input parameters. */
    /* Parameter adjustments */
    x = x.offset(-1);
    ap = ap.offset(-1);
    /* Function Body */
    info = 0 as libc::c_int as integer;
    if lsame__0(
        uplo,
        b"U\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 0
        && lsame__0(
            uplo,
            b"L\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == 0
    {
        info = 1 as libc::c_int as integer
    } else if lsame__0(
        trans,
        b"N\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 0
        && lsame__0(
            trans,
            b"T\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == 0
        && lsame__0(
            trans,
            b"C\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == 0
    {
        info = 2 as libc::c_int as integer
    } else if lsame__0(
        diag,
        b"U\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 0
        && lsame__0(
            diag,
            b"N\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == 0
    {
        info = 3 as libc::c_int as integer
    } else if *n < 0 as libc::c_int as libc::c_long {
        info = 4 as libc::c_int as integer
    } else if *incx == 0 as libc::c_int as libc::c_long {
        info = 7 as libc::c_int as integer
    }
    if info != 0 as libc::c_int as libc::c_long {
        xerbla__0(
            b"ZTPSV \x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            &mut info,
        );
        return 0 as libc::c_int;
    }
    /*     Quick return if possible. */
    if *n == 0 as libc::c_int as libc::c_long {
        return 0 as libc::c_int;
    }
    noconj = lsame__0(
        trans,
        b"T\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    nounit = lsame__0(
        diag,
        b"N\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    /*     Set up the start point in X if the increment is not unity. This */
    /*     will be  ( N - 1 )*INCX  too small for descending loops. */
    if *incx <= 0 as libc::c_int as libc::c_long {
        kx = 1 as libc::c_int as libc::c_long - (*n - 1 as libc::c_int as libc::c_long) * *incx
    } else if *incx != 1 as libc::c_int as libc::c_long {
        kx = 1 as libc::c_int as integer
    }
    /*     Start the operations. In this version the elements of AP are */
    /*     accessed sequentially with one pass through AP. */
    if lsame__0(
        trans,
        b"N\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) != 0
    {
        /*        Form  x := inv( A )*x. */
        if lsame__0(
            uplo,
            b"U\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) != 0
        {
            kk = *n * (*n + 1 as libc::c_int as libc::c_long) / 2 as libc::c_int as libc::c_long;
            if *incx == 1 as libc::c_int as libc::c_long {
                j = *n;
                while j >= 1 as libc::c_int as libc::c_long {
                    i__1 = j;
                    if (*x.offset(i__1 as isize)).r != 0.0f64
                        || (*x.offset(i__1 as isize)).i != 0.0f64
                    {
                        if nounit != 0 {
                            i__1 = j;
                            z_div_0(
                                &mut z__1,
                                &mut *x.offset(j as isize),
                                &mut *ap.offset(kk as isize),
                            );
                            (*x.offset(i__1 as isize)).r = z__1.r;
                            (*x.offset(i__1 as isize)).i = z__1.i
                        }
                        i__1 = j;
                        temp.r = (*x.offset(i__1 as isize)).r;
                        temp.i = (*x.offset(i__1 as isize)).i;
                        k = kk - 1 as libc::c_int as libc::c_long;
                        i__ = j - 1 as libc::c_int as libc::c_long;
                        while i__ >= 1 as libc::c_int as libc::c_long {
                            i__1 = i__;
                            i__2 = i__;
                            i__3 = k;
                            z__2.r = temp.r * (*ap.offset(i__3 as isize)).r
                                - temp.i * (*ap.offset(i__3 as isize)).i;
                            z__2.i = temp.r * (*ap.offset(i__3 as isize)).i
                                + temp.i * (*ap.offset(i__3 as isize)).r;
                            z__1.r = (*x.offset(i__2 as isize)).r - z__2.r;
                            z__1.i = (*x.offset(i__2 as isize)).i - z__2.i;
                            (*x.offset(i__1 as isize)).r = z__1.r;
                            (*x.offset(i__1 as isize)).i = z__1.i;
                            k -= 1;
                            i__ -= 1
                            /* L20: */
                            /* L10: */
                        }
                    }
                    kk -= j;
                    j -= 1
                }
            } else {
                jx = kx + (*n - 1 as libc::c_int as libc::c_long) * *incx;
                j = *n;
                while j >= 1 as libc::c_int as libc::c_long {
                    i__1 = jx;
                    if (*x.offset(i__1 as isize)).r != 0.0f64
                        || (*x.offset(i__1 as isize)).i != 0.0f64
                    {
                        if nounit != 0 {
                            i__1 = jx;
                            z_div_0(
                                &mut z__1,
                                &mut *x.offset(jx as isize),
                                &mut *ap.offset(kk as isize),
                            );
                            (*x.offset(i__1 as isize)).r = z__1.r;
                            (*x.offset(i__1 as isize)).i = z__1.i
                        }
                        i__1 = jx;
                        temp.r = (*x.offset(i__1 as isize)).r;
                        temp.i = (*x.offset(i__1 as isize)).i;
                        ix = jx;
                        i__1 = kk - j + 1 as libc::c_int as libc::c_long;
                        k = kk - 1 as libc::c_int as libc::c_long;
                        while k >= i__1 {
                            ix -= *incx;
                            i__2 = ix;
                            i__3 = ix;
                            i__4 = k;
                            z__2.r = temp.r * (*ap.offset(i__4 as isize)).r
                                - temp.i * (*ap.offset(i__4 as isize)).i;
                            z__2.i = temp.r * (*ap.offset(i__4 as isize)).i
                                + temp.i * (*ap.offset(i__4 as isize)).r;
                            z__1.r = (*x.offset(i__3 as isize)).r - z__2.r;
                            z__1.i = (*x.offset(i__3 as isize)).i - z__2.i;
                            (*x.offset(i__2 as isize)).r = z__1.r;
                            (*x.offset(i__2 as isize)).i = z__1.i;
                            k -= 1
                            /* L40: */
                            /* L30: */
                        }
                    }
                    jx -= *incx;
                    kk -= j;
                    j -= 1
                }
            }
        } else {
            kk = 1 as libc::c_int as integer;
            if *incx == 1 as libc::c_int as libc::c_long {
                i__1 = *n;
                j = 1 as libc::c_int as integer;
                while j <= i__1 {
                    i__2 = j;
                    if (*x.offset(i__2 as isize)).r != 0.0f64
                        || (*x.offset(i__2 as isize)).i != 0.0f64
                    {
                        if nounit != 0 {
                            i__2 = j;
                            z_div_0(
                                &mut z__1,
                                &mut *x.offset(j as isize),
                                &mut *ap.offset(kk as isize),
                            );
                            (*x.offset(i__2 as isize)).r = z__1.r;
                            (*x.offset(i__2 as isize)).i = z__1.i
                        }
                        i__2 = j;
                        temp.r = (*x.offset(i__2 as isize)).r;
                        temp.i = (*x.offset(i__2 as isize)).i;
                        k = kk + 1 as libc::c_int as libc::c_long;
                        i__2 = *n;
                        i__ = j + 1 as libc::c_int as libc::c_long;
                        while i__ <= i__2 {
                            i__3 = i__;
                            i__4 = i__;
                            i__5 = k;
                            z__2.r = temp.r * (*ap.offset(i__5 as isize)).r
                                - temp.i * (*ap.offset(i__5 as isize)).i;
                            z__2.i = temp.r * (*ap.offset(i__5 as isize)).i
                                + temp.i * (*ap.offset(i__5 as isize)).r;
                            z__1.r = (*x.offset(i__4 as isize)).r - z__2.r;
                            z__1.i = (*x.offset(i__4 as isize)).i - z__2.i;
                            (*x.offset(i__3 as isize)).r = z__1.r;
                            (*x.offset(i__3 as isize)).i = z__1.i;
                            k += 1;
                            i__ += 1
                            /* L60: */
                            /* L50: */
                        }
                    }
                    kk += *n - j + 1 as libc::c_int as libc::c_long;
                    j += 1
                }
            } else {
                jx = kx;
                i__1 = *n;
                j = 1 as libc::c_int as integer;
                while j <= i__1 {
                    i__2 = jx;
                    if (*x.offset(i__2 as isize)).r != 0.0f64
                        || (*x.offset(i__2 as isize)).i != 0.0f64
                    {
                        if nounit != 0 {
                            i__2 = jx;
                            z_div_0(
                                &mut z__1,
                                &mut *x.offset(jx as isize),
                                &mut *ap.offset(kk as isize),
                            );
                            (*x.offset(i__2 as isize)).r = z__1.r;
                            (*x.offset(i__2 as isize)).i = z__1.i
                        }
                        i__2 = jx;
                        temp.r = (*x.offset(i__2 as isize)).r;
                        temp.i = (*x.offset(i__2 as isize)).i;
                        ix = jx;
                        i__2 = kk + *n - j;
                        k = kk + 1 as libc::c_int as libc::c_long;
                        while k <= i__2 {
                            ix += *incx;
                            i__3 = ix;
                            i__4 = ix;
                            i__5 = k;
                            z__2.r = temp.r * (*ap.offset(i__5 as isize)).r
                                - temp.i * (*ap.offset(i__5 as isize)).i;
                            z__2.i = temp.r * (*ap.offset(i__5 as isize)).i
                                + temp.i * (*ap.offset(i__5 as isize)).r;
                            z__1.r = (*x.offset(i__4 as isize)).r - z__2.r;
                            z__1.i = (*x.offset(i__4 as isize)).i - z__2.i;
                            (*x.offset(i__3 as isize)).r = z__1.r;
                            (*x.offset(i__3 as isize)).i = z__1.i;
                            k += 1
                            /* L80: */
                            /* L70: */
                        }
                    }
                    jx += *incx;
                    kk += *n - j + 1 as libc::c_int as libc::c_long;
                    j += 1
                }
            }
        }
    } else if lsame__0(
        uplo,
        b"U\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) != 0
    {
        kk = 1 as libc::c_int as integer;
        if *incx == 1 as libc::c_int as libc::c_long {
            i__1 = *n;
            j = 1 as libc::c_int as integer;
            while j <= i__1 {
                i__2 = j;
                temp.r = (*x.offset(i__2 as isize)).r;
                temp.i = (*x.offset(i__2 as isize)).i;
                k = kk;
                if noconj != 0 {
                    i__2 = j - 1 as libc::c_int as libc::c_long;
                    i__ = 1 as libc::c_int as integer;
                    while i__ <= i__2 {
                        i__3 = k;
                        i__4 = i__;
                        z__2.r = (*ap.offset(i__3 as isize)).r * (*x.offset(i__4 as isize)).r
                            - (*ap.offset(i__3 as isize)).i * (*x.offset(i__4 as isize)).i;
                        z__2.i = (*ap.offset(i__3 as isize)).r * (*x.offset(i__4 as isize)).i
                            + (*ap.offset(i__3 as isize)).i * (*x.offset(i__4 as isize)).r;
                        z__1.r = temp.r - z__2.r;
                        z__1.i = temp.i - z__2.i;
                        temp.r = z__1.r;
                        temp.i = z__1.i;
                        k += 1;
                        i__ += 1
                        /*        Form  x := inv( A' )*x  or  x := inv( conjg( A' ) )*x. */
                        /* L110: */
                        /* L90: */
                    }
                    if nounit != 0 {
                        z_div_0(
                            &mut z__1,
                            &mut temp,
                            &mut *ap.offset((kk + j - 1 as libc::c_int as libc::c_long) as isize),
                        );
                        temp.r = z__1.r;
                        temp.i = z__1.i
                    }
                } else {
                    i__2 = j - 1 as libc::c_int as libc::c_long;
                    i__ = 1 as libc::c_int as integer;
                    while i__ <= i__2 {
                        d_cnjg_0(&mut z__3, &mut *ap.offset(k as isize));
                        i__3 = i__;
                        z__2.r = z__3.r * (*x.offset(i__3 as isize)).r
                            - z__3.i * (*x.offset(i__3 as isize)).i;
                        z__2.i = z__3.r * (*x.offset(i__3 as isize)).i
                            + z__3.i * (*x.offset(i__3 as isize)).r;
                        z__1.r = temp.r - z__2.r;
                        z__1.i = temp.i - z__2.i;
                        temp.r = z__1.r;
                        temp.i = z__1.i;
                        k += 1;
                        i__ += 1
                        /* L100: */
                    }
                    if nounit != 0 {
                        d_cnjg_0(
                            &mut z__2,
                            &mut *ap.offset((kk + j - 1 as libc::c_int as libc::c_long) as isize),
                        );
                        z_div_0(&mut z__1, &mut temp, &mut z__2);
                        temp.r = z__1.r;
                        temp.i = z__1.i
                    }
                }
                i__2 = j;
                (*x.offset(i__2 as isize)).r = temp.r;
                (*x.offset(i__2 as isize)).i = temp.i;
                kk += j;
                j += 1
            }
        } else {
            jx = kx;
            i__1 = *n;
            j = 1 as libc::c_int as integer;
            while j <= i__1 {
                i__2 = jx;
                temp.r = (*x.offset(i__2 as isize)).r;
                temp.i = (*x.offset(i__2 as isize)).i;
                ix = kx;
                if noconj != 0 {
                    i__2 = kk + j - 2 as libc::c_int as libc::c_long;
                    k = kk;
                    while k <= i__2 {
                        i__3 = k;
                        i__4 = ix;
                        z__2.r = (*ap.offset(i__3 as isize)).r * (*x.offset(i__4 as isize)).r
                            - (*ap.offset(i__3 as isize)).i * (*x.offset(i__4 as isize)).i;
                        z__2.i = (*ap.offset(i__3 as isize)).r * (*x.offset(i__4 as isize)).i
                            + (*ap.offset(i__3 as isize)).i * (*x.offset(i__4 as isize)).r;
                        z__1.r = temp.r - z__2.r;
                        z__1.i = temp.i - z__2.i;
                        temp.r = z__1.r;
                        temp.i = z__1.i;
                        ix += *incx;
                        k += 1
                        /* L140: */
                        /* L120: */
                    }
                    if nounit != 0 {
                        z_div_0(
                            &mut z__1,
                            &mut temp,
                            &mut *ap.offset((kk + j - 1 as libc::c_int as libc::c_long) as isize),
                        );
                        temp.r = z__1.r;
                        temp.i = z__1.i
                    }
                } else {
                    i__2 = kk + j - 2 as libc::c_int as libc::c_long;
                    k = kk;
                    while k <= i__2 {
                        d_cnjg_0(&mut z__3, &mut *ap.offset(k as isize));
                        i__3 = ix;
                        z__2.r = z__3.r * (*x.offset(i__3 as isize)).r
                            - z__3.i * (*x.offset(i__3 as isize)).i;
                        z__2.i = z__3.r * (*x.offset(i__3 as isize)).i
                            + z__3.i * (*x.offset(i__3 as isize)).r;
                        z__1.r = temp.r - z__2.r;
                        z__1.i = temp.i - z__2.i;
                        temp.r = z__1.r;
                        temp.i = z__1.i;
                        ix += *incx;
                        k += 1
                        /* L130: */
                    }
                    if nounit != 0 {
                        d_cnjg_0(
                            &mut z__2,
                            &mut *ap.offset((kk + j - 1 as libc::c_int as libc::c_long) as isize),
                        );
                        z_div_0(&mut z__1, &mut temp, &mut z__2);
                        temp.r = z__1.r;
                        temp.i = z__1.i
                    }
                }
                i__2 = jx;
                (*x.offset(i__2 as isize)).r = temp.r;
                (*x.offset(i__2 as isize)).i = temp.i;
                jx += *incx;
                kk += j;
                j += 1
            }
        }
    } else {
        kk = *n * (*n + 1 as libc::c_int as libc::c_long) / 2 as libc::c_int as libc::c_long;
        if *incx == 1 as libc::c_int as libc::c_long {
            j = *n;
            while j >= 1 as libc::c_int as libc::c_long {
                i__1 = j;
                temp.r = (*x.offset(i__1 as isize)).r;
                temp.i = (*x.offset(i__1 as isize)).i;
                k = kk;
                if noconj != 0 {
                    i__1 = j + 1 as libc::c_int as libc::c_long;
                    i__ = *n;
                    while i__ >= i__1 {
                        i__2 = k;
                        i__3 = i__;
                        z__2.r = (*ap.offset(i__2 as isize)).r * (*x.offset(i__3 as isize)).r
                            - (*ap.offset(i__2 as isize)).i * (*x.offset(i__3 as isize)).i;
                        z__2.i = (*ap.offset(i__2 as isize)).r * (*x.offset(i__3 as isize)).i
                            + (*ap.offset(i__2 as isize)).i * (*x.offset(i__3 as isize)).r;
                        z__1.r = temp.r - z__2.r;
                        z__1.i = temp.i - z__2.i;
                        temp.r = z__1.r;
                        temp.i = z__1.i;
                        k -= 1;
                        i__ -= 1
                        /* L170: */
                        /* L150: */
                    }
                    if nounit != 0 {
                        z_div_0(
                            &mut z__1,
                            &mut temp,
                            &mut *ap.offset((kk - *n + j) as isize),
                        );
                        temp.r = z__1.r;
                        temp.i = z__1.i
                    }
                } else {
                    i__1 = j + 1 as libc::c_int as libc::c_long;
                    i__ = *n;
                    while i__ >= i__1 {
                        d_cnjg_0(&mut z__3, &mut *ap.offset(k as isize));
                        i__2 = i__;
                        z__2.r = z__3.r * (*x.offset(i__2 as isize)).r
                            - z__3.i * (*x.offset(i__2 as isize)).i;
                        z__2.i = z__3.r * (*x.offset(i__2 as isize)).i
                            + z__3.i * (*x.offset(i__2 as isize)).r;
                        z__1.r = temp.r - z__2.r;
                        z__1.i = temp.i - z__2.i;
                        temp.r = z__1.r;
                        temp.i = z__1.i;
                        k -= 1;
                        i__ -= 1
                        /* L160: */
                    }
                    if nounit != 0 {
                        d_cnjg_0(&mut z__2, &mut *ap.offset((kk - *n + j) as isize));
                        z_div_0(&mut z__1, &mut temp, &mut z__2);
                        temp.r = z__1.r;
                        temp.i = z__1.i
                    }
                }
                i__1 = j;
                (*x.offset(i__1 as isize)).r = temp.r;
                (*x.offset(i__1 as isize)).i = temp.i;
                kk -= *n - j + 1 as libc::c_int as libc::c_long;
                j -= 1
            }
        } else {
            kx += (*n - 1 as libc::c_int as libc::c_long) * *incx;
            jx = kx;
            j = *n;
            while j >= 1 as libc::c_int as libc::c_long {
                i__1 = jx;
                temp.r = (*x.offset(i__1 as isize)).r;
                temp.i = (*x.offset(i__1 as isize)).i;
                ix = kx;
                if noconj != 0 {
                    i__1 = kk - (*n - (j + 1 as libc::c_int as libc::c_long));
                    k = kk;
                    while k >= i__1 {
                        i__2 = k;
                        i__3 = ix;
                        z__2.r = (*ap.offset(i__2 as isize)).r * (*x.offset(i__3 as isize)).r
                            - (*ap.offset(i__2 as isize)).i * (*x.offset(i__3 as isize)).i;
                        z__2.i = (*ap.offset(i__2 as isize)).r * (*x.offset(i__3 as isize)).i
                            + (*ap.offset(i__2 as isize)).i * (*x.offset(i__3 as isize)).r;
                        z__1.r = temp.r - z__2.r;
                        z__1.i = temp.i - z__2.i;
                        temp.r = z__1.r;
                        temp.i = z__1.i;
                        ix -= *incx;
                        k -= 1
                        /* L200: */
                        /* L180: */
                    }
                    if nounit != 0 {
                        z_div_0(
                            &mut z__1,
                            &mut temp,
                            &mut *ap.offset((kk - *n + j) as isize),
                        );
                        temp.r = z__1.r;
                        temp.i = z__1.i
                    }
                } else {
                    i__1 = kk - (*n - (j + 1 as libc::c_int as libc::c_long));
                    k = kk;
                    while k >= i__1 {
                        d_cnjg_0(&mut z__3, &mut *ap.offset(k as isize));
                        i__2 = ix;
                        z__2.r = z__3.r * (*x.offset(i__2 as isize)).r
                            - z__3.i * (*x.offset(i__2 as isize)).i;
                        z__2.i = z__3.r * (*x.offset(i__2 as isize)).i
                            + z__3.i * (*x.offset(i__2 as isize)).r;
                        z__1.r = temp.r - z__2.r;
                        z__1.i = temp.i - z__2.i;
                        temp.r = z__1.r;
                        temp.i = z__1.i;
                        ix -= *incx;
                        k -= 1
                        /* L190: */
                    }
                    if nounit != 0 {
                        d_cnjg_0(&mut z__2, &mut *ap.offset((kk - *n + j) as isize));
                        z_div_0(&mut z__1, &mut temp, &mut z__2);
                        temp.r = z__1.r;
                        temp.i = z__1.i
                    }
                }
                i__1 = jx;
                (*x.offset(i__1 as isize)).r = temp.r;
                (*x.offset(i__1 as isize)).i = temp.i;
                jx -= *incx;
                kk -= *n - j + 1 as libc::c_int as libc::c_long;
                j -= 1
            }
        }
    }
    return 0 as libc::c_int;
    /*     End of ZTPSV . */
}
/* ztpsv_ */
