use libc;
/* f2c.h  --  Standard Fortran to C header file */
/* *  barf  [ba:rf]  2.  "He suggested using FORTRAN, and everybody barfed."

- From The Shogakukan DICTIONARY OF NEW ENGLISH (Second edition) */
pub type integer = libc::c_long;
pub type real = libc::c_float;
pub type logical = libc::c_long;
/* sspmv.f -- translated by f2c (version 20061008).
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
pub unsafe extern "C" fn f2c_sspmv(
    mut uplo: *mut libc::c_char,
    mut n: *mut integer,
    mut alpha: *mut real,
    mut ap: *mut real,
    mut x: *mut real,
    mut incx: *mut integer,
    mut beta: *mut real,
    mut y: *mut real,
    mut incy: *mut integer,
) -> libc::c_int {
    /* System generated locals */
    let mut i__1: integer = 0;
    let mut i__2: integer = 0;
    /* Local variables */
    let mut i__: integer = 0;
    let mut j: integer = 0;
    let mut k: integer = 0;
    let mut kk: integer = 0;
    let mut ix: integer = 0;
    let mut iy: integer = 0;
    let mut jx: integer = 0;
    let mut jy: integer = 0;
    let mut kx: integer = 0;
    let mut ky: integer = 0;
    let mut info: integer = 0;
    let mut temp1: real = 0.;
    let mut temp2: real = 0.;
    extern "C" {
        #[link_name = "lsame_"]
        fn lsame__0(_: *mut libc::c_char, _: *mut libc::c_char) -> logical;
    }
    extern "C" {
        #[link_name = "xerbla_"]
        fn xerbla__0(_: *mut libc::c_char, _: *mut integer) -> libc::c_int;
    }
    /*     .. Scalar Arguments .. */
    /*     .. */
    /*     .. Array Arguments .. */
    /*     .. */
    /*  Purpose */
    /*  ======= */
    /*  SSPMV  performs the matrix-vector operation */
    /*     y := alpha*A*x + beta*y, */
    /*  where alpha and beta are scalars, x and y are n element vectors and */
    /*  A is an n by n symmetric matrix, supplied in packed form. */
    /*  Arguments */
    /*  ========== */
    /*  UPLO   - CHARACTER*1. */
    /*           On entry, UPLO specifies whether the upper or lower */
    /*           triangular part of the matrix A is supplied in the packed */
    /*           array AP as follows: */
    /*              UPLO = 'U' or 'u'   The upper triangular part of A is */
    /*                                  supplied in AP. */
    /*              UPLO = 'L' or 'l'   The lower triangular part of A is */
    /*                                  supplied in AP. */
    /*           Unchanged on exit. */
    /*  N      - INTEGER. */
    /*           On entry, N specifies the order of the matrix A. */
    /*           N must be at least zero. */
    /*           Unchanged on exit. */
    /*  ALPHA  - REAL            . */
    /*           On entry, ALPHA specifies the scalar alpha. */
    /*           Unchanged on exit. */
    /*  AP     - REAL             array of DIMENSION at least */
    /*           ( ( n*( n + 1 ) )/2 ). */
    /*           Before entry with UPLO = 'U' or 'u', the array AP must */
    /*           contain the upper triangular part of the symmetric matrix */
    /*           packed sequentially, column by column, so that AP( 1 ) */
    /*           contains a( 1, 1 ), AP( 2 ) and AP( 3 ) contain a( 1, 2 ) */
    /*           and a( 2, 2 ) respectively, and so on. */
    /*           Before entry with UPLO = 'L' or 'l', the array AP must */
    /*           contain the lower triangular part of the symmetric matrix */
    /*           packed sequentially, column by column, so that AP( 1 ) */
    /*           contains a( 1, 1 ), AP( 2 ) and AP( 3 ) contain a( 2, 1 ) */
    /*           and a( 3, 1 ) respectively, and so on. */
    /*           Unchanged on exit. */
    /*  X      - REAL             array of dimension at least */
    /*           ( 1 + ( n - 1 )*abs( INCX ) ). */
    /*           Before entry, the incremented array X must contain the n */
    /*           element vector x. */
    /*           Unchanged on exit. */
    /*  INCX   - INTEGER. */
    /*           On entry, INCX specifies the increment for the elements of */
    /*           X. INCX must not be zero. */
    /*           Unchanged on exit. */
    /*  BETA   - REAL            . */
    /*           On entry, BETA specifies the scalar beta. When BETA is */
    /*           supplied as zero then Y need not be set on input. */
    /*           Unchanged on exit. */
    /*  Y      - REAL             array of dimension at least */
    /*           ( 1 + ( n - 1 )*abs( INCY ) ). */
    /*           Before entry, the incremented array Y must contain the n */
    /*           element vector y. On exit, Y is overwritten by the updated */
    /*           vector y. */
    /*  INCY   - INTEGER. */
    /*           On entry, INCY specifies the increment for the elements of */
    /*           Y. INCY must not be zero. */
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
    /*     Test the input parameters. */
    /* Parameter adjustments */
    y = y.offset(-1);
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
    } else if *n < 0 as libc::c_int as libc::c_long {
        info = 2 as libc::c_int as integer
    } else if *incx == 0 as libc::c_int as libc::c_long {
        info = 6 as libc::c_int as integer
    } else if *incy == 0 as libc::c_int as libc::c_long {
        info = 9 as libc::c_int as integer
    }
    if info != 0 as libc::c_int as libc::c_long {
        xerbla__0(
            b"SSPMV \x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            &mut info,
        );
        return 0 as libc::c_int;
    }
    /*     Quick return if possible. */
    if *n == 0 as libc::c_int as libc::c_long || *alpha == 0.0f32 && *beta == 1.0f32 {
        return 0 as libc::c_int;
    }
    /*     Set up the start points in  X  and  Y. */
    if *incx > 0 as libc::c_int as libc::c_long {
        kx = 1 as libc::c_int as integer
    } else {
        kx = 1 as libc::c_int as libc::c_long - (*n - 1 as libc::c_int as libc::c_long) * *incx
    }
    if *incy > 0 as libc::c_int as libc::c_long {
        ky = 1 as libc::c_int as integer
    } else {
        ky = 1 as libc::c_int as libc::c_long - (*n - 1 as libc::c_int as libc::c_long) * *incy
    }
    /*     Start the operations. In this version the elements of the array AP */
    /*     are accessed sequentially with one pass through AP. */
    /*     First form  y := beta*y. */
    if *beta != 1.0f32 {
        if *incy == 1 as libc::c_int as libc::c_long {
            if *beta == 0.0f32 {
                i__1 = *n;
                i__ = 1 as libc::c_int as integer;
                while i__ <= i__1 {
                    *y.offset(i__ as isize) = 0.0f32;
                    i__ += 1
                    /* L10: */
                }
            } else {
                i__1 = *n;
                i__ = 1 as libc::c_int as integer;
                while i__ <= i__1 {
                    *y.offset(i__ as isize) = *beta * *y.offset(i__ as isize);
                    i__ += 1
                    /* L20: */
                }
            }
        } else {
            iy = ky;
            if *beta == 0.0f32 {
                i__1 = *n;
                i__ = 1 as libc::c_int as integer;
                while i__ <= i__1 {
                    *y.offset(iy as isize) = 0.0f32;
                    iy += *incy;
                    i__ += 1
                    /* L30: */
                }
            } else {
                i__1 = *n;
                i__ = 1 as libc::c_int as integer;
                while i__ <= i__1 {
                    *y.offset(iy as isize) = *beta * *y.offset(iy as isize);
                    iy += *incy;
                    i__ += 1
                    /* L40: */
                }
            }
        }
    }
    if *alpha == 0.0f32 {
        return 0 as libc::c_int;
    }
    kk = 1 as libc::c_int as integer;
    if lsame__0(
        uplo,
        b"U\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) != 0
    {
        /*        Form  y  when AP contains the upper triangle. */
        if *incx == 1 as libc::c_int as libc::c_long && *incy == 1 as libc::c_int as libc::c_long {
            i__1 = *n;
            j = 1 as libc::c_int as integer;
            while j <= i__1 {
                temp1 = *alpha * *x.offset(j as isize);
                temp2 = 0.0f32;
                k = kk;
                i__2 = j - 1 as libc::c_int as libc::c_long;
                i__ = 1 as libc::c_int as integer;
                while i__ <= i__2 {
                    let ref mut fresh0 = *y.offset(i__ as isize);
                    *fresh0 += temp1 * *ap.offset(k as isize);
                    temp2 += *ap.offset(k as isize) * *x.offset(i__ as isize);
                    k += 1;
                    i__ += 1
                    /* L60: */
                    /* L50: */
                }
                *y.offset(j as isize) = *y.offset(j as isize)
                    + temp1 * *ap.offset((kk + j - 1 as libc::c_int as libc::c_long) as isize)
                    + *alpha * temp2;
                kk += j;
                j += 1
            }
        } else {
            jx = kx;
            jy = ky;
            i__1 = *n;
            j = 1 as libc::c_int as integer;
            while j <= i__1 {
                temp1 = *alpha * *x.offset(jx as isize);
                temp2 = 0.0f32;
                ix = kx;
                iy = ky;
                i__2 = kk + j - 2 as libc::c_int as libc::c_long;
                k = kk;
                while k <= i__2 {
                    let ref mut fresh1 = *y.offset(iy as isize);
                    *fresh1 += temp1 * *ap.offset(k as isize);
                    temp2 += *ap.offset(k as isize) * *x.offset(ix as isize);
                    ix += *incx;
                    iy += *incy;
                    k += 1
                    /* L80: */
                    /* L70: */
                }
                *y.offset(jy as isize) = *y.offset(jy as isize)
                    + temp1 * *ap.offset((kk + j - 1 as libc::c_int as libc::c_long) as isize)
                    + *alpha * temp2;
                jx += *incx;
                jy += *incy;
                kk += j;
                j += 1
            }
        }
    } else if *incx == 1 as libc::c_int as libc::c_long && *incy == 1 as libc::c_int as libc::c_long
    {
        i__1 = *n;
        j = 1 as libc::c_int as integer;
        while j <= i__1 {
            temp1 = *alpha * *x.offset(j as isize);
            temp2 = 0.0f32;
            let ref mut fresh2 = *y.offset(j as isize);
            *fresh2 += temp1 * *ap.offset(kk as isize);
            k = kk + 1 as libc::c_int as libc::c_long;
            i__2 = *n;
            i__ = j + 1 as libc::c_int as libc::c_long;
            while i__ <= i__2 {
                let ref mut fresh3 = *y.offset(i__ as isize);
                *fresh3 += temp1 * *ap.offset(k as isize);
                temp2 += *ap.offset(k as isize) * *x.offset(i__ as isize);
                k += 1;
                i__ += 1
                /*        Form  y  when AP contains the lower triangle. */
                /* L100: */
                /* L90: */
            }
            let ref mut fresh4 = *y.offset(j as isize);
            *fresh4 += *alpha * temp2;
            kk += *n - j + 1 as libc::c_int as libc::c_long;
            j += 1
        }
    } else {
        jx = kx;
        jy = ky;
        i__1 = *n;
        j = 1 as libc::c_int as integer;
        while j <= i__1 {
            temp1 = *alpha * *x.offset(jx as isize);
            temp2 = 0.0f32;
            let ref mut fresh5 = *y.offset(jy as isize);
            *fresh5 += temp1 * *ap.offset(kk as isize);
            ix = jx;
            iy = jy;
            i__2 = kk + *n - j;
            k = kk + 1 as libc::c_int as libc::c_long;
            while k <= i__2 {
                ix += *incx;
                iy += *incy;
                let ref mut fresh6 = *y.offset(iy as isize);
                *fresh6 += temp1 * *ap.offset(k as isize);
                temp2 += *ap.offset(k as isize) * *x.offset(ix as isize);
                k += 1
                /* L120: */
                /* L110: */
            }
            let ref mut fresh7 = *y.offset(jy as isize);
            *fresh7 += *alpha * temp2;
            jx += *incx;
            jy += *incy;
            kk += *n - j + 1 as libc::c_int as libc::c_long;
            j += 1
        }
    }
    return 0 as libc::c_int;
    /*     End of SSPMV . */
}
/* sspmv_ */
