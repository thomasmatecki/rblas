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
/* zhpr2.f -- translated by f2c (version 20061008).
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
pub unsafe extern "C" fn f2c_zhpr2(
    mut uplo: *mut libc::c_char,
    mut n: *mut integer,
    mut alpha: *mut doublecomplex,
    mut x: *mut doublecomplex,
    mut incx: *mut integer,
    mut y: *mut doublecomplex,
    mut incy: *mut integer,
    mut ap: *mut doublecomplex,
) -> libc::c_int {
    /* System generated locals */
    let mut i__1: integer = 0;
    let mut i__2: integer = 0;
    let mut i__3: integer = 0;
    let mut i__4: integer = 0;
    let mut i__5: integer = 0;
    let mut i__6: integer = 0;
    let mut d__1: doublereal = 0.;
    let mut z__1: doublecomplex = doublecomplex { r: 0., i: 0. };
    let mut z__2: doublecomplex = doublecomplex { r: 0., i: 0. };
    let mut z__3: doublecomplex = doublecomplex { r: 0., i: 0. };
    let mut z__4: doublecomplex = doublecomplex { r: 0., i: 0. };
    /* Builtin functions */
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
    let mut iy: integer = 0;
    let mut jx: integer = 0;
    let mut jy: integer = 0;
    let mut kx: integer = 0;
    let mut ky: integer = 0;
    let mut info: integer = 0;
    let mut temp1: doublecomplex = doublecomplex { r: 0., i: 0. };
    let mut temp2: doublecomplex = doublecomplex { r: 0., i: 0. };
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
    /*  ZHPR2  performs the hermitian rank 2 operation */
    /*     A := alpha*x*conjg( y' ) + conjg( alpha )*y*conjg( x' ) + A, */
    /*  where alpha is a scalar, x and y are n element vectors and A is an */
    /*  n by n hermitian matrix, supplied in packed form. */
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
    /*  ALPHA  - COMPLEX*16      . */
    /*           On entry, ALPHA specifies the scalar alpha. */
    /*           Unchanged on exit. */
    /*  X      - COMPLEX*16       array of dimension at least */
    /*           ( 1 + ( n - 1 )*abs( INCX ) ). */
    /*           Before entry, the incremented array X must contain the n */
    /*           element vector x. */
    /*           Unchanged on exit. */
    /*  INCX   - INTEGER. */
    /*           On entry, INCX specifies the increment for the elements of */
    /*           X. INCX must not be zero. */
    /*           Unchanged on exit. */
    /*  Y      - COMPLEX*16       array of dimension at least */
    /*           ( 1 + ( n - 1 )*abs( INCY ) ). */
    /*           Before entry, the incremented array Y must contain the n */
    /*           element vector y. */
    /*           Unchanged on exit. */
    /*  INCY   - INTEGER. */
    /*           On entry, INCY specifies the increment for the elements of */
    /*           Y. INCY must not be zero. */
    /*           Unchanged on exit. */
    /*  AP     - COMPLEX*16       array of DIMENSION at least */
    /*           ( ( n*( n + 1 ) )/2 ). */
    /*           Before entry with  UPLO = 'U' or 'u', the array AP must */
    /*           contain the upper triangular part of the hermitian matrix */
    /*           packed sequentially, column by column, so that AP( 1 ) */
    /*           contains a( 1, 1 ), AP( 2 ) and AP( 3 ) contain a( 1, 2 ) */
    /*           and a( 2, 2 ) respectively, and so on. On exit, the array */
    /*           AP is overwritten by the upper triangular part of the */
    /*           updated matrix. */
    /*           Before entry with UPLO = 'L' or 'l', the array AP must */
    /*           contain the lower triangular part of the hermitian matrix */
    /*           packed sequentially, column by column, so that AP( 1 ) */
    /*           contains a( 1, 1 ), AP( 2 ) and AP( 3 ) contain a( 2, 1 ) */
    /*           and a( 3, 1 ) respectively, and so on. On exit, the array */
    /*           AP is overwritten by the lower triangular part of the */
    /*           updated matrix. */
    /*           Note that the imaginary parts of the diagonal elements need */
    /*           not be set, they are assumed to be zero, and on exit they */
    /*           are set to zero. */
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
    ap = ap.offset(-1);
    y = y.offset(-1);
    x = x.offset(-1);
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
        info = 5 as libc::c_int as integer
    } else if *incy == 0 as libc::c_int as libc::c_long {
        info = 7 as libc::c_int as integer
    }
    if info != 0 as libc::c_int as libc::c_long {
        xerbla__0(
            b"ZHPR2 \x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            &mut info,
        );
        return 0 as libc::c_int;
    }
    /*     Quick return if possible. */
    if *n == 0 as libc::c_int as libc::c_long || (*alpha).r == 0.0f64 && (*alpha).i == 0.0f64 {
        return 0 as libc::c_int;
    }
    /*     Set up the start points in X and Y if the increments are not both */
    /*     unity. */
    if *incx != 1 as libc::c_int as libc::c_long || *incy != 1 as libc::c_int as libc::c_long {
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
        jx = kx;
        jy = ky
    }
    /*     Start the operations. In this version the elements of the array AP */
    /*     are accessed sequentially with one pass through AP. */
    kk = 1 as libc::c_int as integer;
    if lsame__0(
        uplo,
        b"U\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) != 0
    {
        /*        Form  A  when upper triangle is stored in AP. */
        if *incx == 1 as libc::c_int as libc::c_long && *incy == 1 as libc::c_int as libc::c_long {
            i__1 = *n;
            j = 1 as libc::c_int as integer;
            while j <= i__1 {
                i__2 = j;
                i__3 = j;
                if (*x.offset(i__2 as isize)).r != 0.0f64
                    || (*x.offset(i__2 as isize)).i != 0.0f64
                    || ((*y.offset(i__3 as isize)).r != 0.0f64
                        || (*y.offset(i__3 as isize)).i != 0.0f64)
                {
                    d_cnjg_0(&mut z__2, &mut *y.offset(j as isize));
                    z__1.r = (*alpha).r * z__2.r - (*alpha).i * z__2.i;
                    z__1.i = (*alpha).r * z__2.i + (*alpha).i * z__2.r;
                    temp1.r = z__1.r;
                    temp1.i = z__1.i;
                    i__2 = j;
                    z__2.r = (*alpha).r * (*x.offset(i__2 as isize)).r
                        - (*alpha).i * (*x.offset(i__2 as isize)).i;
                    z__2.i = (*alpha).r * (*x.offset(i__2 as isize)).i
                        + (*alpha).i * (*x.offset(i__2 as isize)).r;
                    d_cnjg_0(&mut z__1, &mut z__2);
                    temp2.r = z__1.r;
                    temp2.i = z__1.i;
                    k = kk;
                    i__2 = j - 1 as libc::c_int as libc::c_long;
                    i__ = 1 as libc::c_int as integer;
                    while i__ <= i__2 {
                        i__3 = k;
                        i__4 = k;
                        i__5 = i__;
                        z__3.r = (*x.offset(i__5 as isize)).r * temp1.r
                            - (*x.offset(i__5 as isize)).i * temp1.i;
                        z__3.i = (*x.offset(i__5 as isize)).r * temp1.i
                            + (*x.offset(i__5 as isize)).i * temp1.r;
                        z__2.r = (*ap.offset(i__4 as isize)).r + z__3.r;
                        z__2.i = (*ap.offset(i__4 as isize)).i + z__3.i;
                        i__6 = i__;
                        z__4.r = (*y.offset(i__6 as isize)).r * temp2.r
                            - (*y.offset(i__6 as isize)).i * temp2.i;
                        z__4.i = (*y.offset(i__6 as isize)).r * temp2.i
                            + (*y.offset(i__6 as isize)).i * temp2.r;
                        z__1.r = z__2.r + z__4.r;
                        z__1.i = z__2.i + z__4.i;
                        (*ap.offset(i__3 as isize)).r = z__1.r;
                        (*ap.offset(i__3 as isize)).i = z__1.i;
                        k += 1;
                        i__ += 1
                        /* L20: */
                        /* L10: */
                    }
                    i__2 = kk + j - 1 as libc::c_int as libc::c_long;
                    i__3 = kk + j - 1 as libc::c_int as libc::c_long;
                    i__4 = j;
                    z__2.r = (*x.offset(i__4 as isize)).r * temp1.r
                        - (*x.offset(i__4 as isize)).i * temp1.i;
                    z__2.i = (*x.offset(i__4 as isize)).r * temp1.i
                        + (*x.offset(i__4 as isize)).i * temp1.r;
                    i__5 = j;
                    z__3.r = (*y.offset(i__5 as isize)).r * temp2.r
                        - (*y.offset(i__5 as isize)).i * temp2.i;
                    z__3.i = (*y.offset(i__5 as isize)).r * temp2.i
                        + (*y.offset(i__5 as isize)).i * temp2.r;
                    z__1.r = z__2.r + z__3.r;
                    z__1.i = z__2.i + z__3.i;
                    d__1 = (*ap.offset(i__3 as isize)).r + z__1.r;
                    (*ap.offset(i__2 as isize)).r = d__1;
                    (*ap.offset(i__2 as isize)).i = 0.0f64
                } else {
                    i__2 = kk + j - 1 as libc::c_int as libc::c_long;
                    i__3 = kk + j - 1 as libc::c_int as libc::c_long;
                    d__1 = (*ap.offset(i__3 as isize)).r;
                    (*ap.offset(i__2 as isize)).r = d__1;
                    (*ap.offset(i__2 as isize)).i = 0.0f64
                }
                kk += j;
                j += 1
            }
        } else {
            i__1 = *n;
            j = 1 as libc::c_int as integer;
            while j <= i__1 {
                i__2 = jx;
                i__3 = jy;
                if (*x.offset(i__2 as isize)).r != 0.0f64
                    || (*x.offset(i__2 as isize)).i != 0.0f64
                    || ((*y.offset(i__3 as isize)).r != 0.0f64
                        || (*y.offset(i__3 as isize)).i != 0.0f64)
                {
                    d_cnjg_0(&mut z__2, &mut *y.offset(jy as isize));
                    z__1.r = (*alpha).r * z__2.r - (*alpha).i * z__2.i;
                    z__1.i = (*alpha).r * z__2.i + (*alpha).i * z__2.r;
                    temp1.r = z__1.r;
                    temp1.i = z__1.i;
                    i__2 = jx;
                    z__2.r = (*alpha).r * (*x.offset(i__2 as isize)).r
                        - (*alpha).i * (*x.offset(i__2 as isize)).i;
                    z__2.i = (*alpha).r * (*x.offset(i__2 as isize)).i
                        + (*alpha).i * (*x.offset(i__2 as isize)).r;
                    d_cnjg_0(&mut z__1, &mut z__2);
                    temp2.r = z__1.r;
                    temp2.i = z__1.i;
                    ix = kx;
                    iy = ky;
                    i__2 = kk + j - 2 as libc::c_int as libc::c_long;
                    k = kk;
                    while k <= i__2 {
                        i__3 = k;
                        i__4 = k;
                        i__5 = ix;
                        z__3.r = (*x.offset(i__5 as isize)).r * temp1.r
                            - (*x.offset(i__5 as isize)).i * temp1.i;
                        z__3.i = (*x.offset(i__5 as isize)).r * temp1.i
                            + (*x.offset(i__5 as isize)).i * temp1.r;
                        z__2.r = (*ap.offset(i__4 as isize)).r + z__3.r;
                        z__2.i = (*ap.offset(i__4 as isize)).i + z__3.i;
                        i__6 = iy;
                        z__4.r = (*y.offset(i__6 as isize)).r * temp2.r
                            - (*y.offset(i__6 as isize)).i * temp2.i;
                        z__4.i = (*y.offset(i__6 as isize)).r * temp2.i
                            + (*y.offset(i__6 as isize)).i * temp2.r;
                        z__1.r = z__2.r + z__4.r;
                        z__1.i = z__2.i + z__4.i;
                        (*ap.offset(i__3 as isize)).r = z__1.r;
                        (*ap.offset(i__3 as isize)).i = z__1.i;
                        ix += *incx;
                        iy += *incy;
                        k += 1
                        /* L40: */
                        /* L30: */
                    }
                    i__2 = kk + j - 1 as libc::c_int as libc::c_long;
                    i__3 = kk + j - 1 as libc::c_int as libc::c_long;
                    i__4 = jx;
                    z__2.r = (*x.offset(i__4 as isize)).r * temp1.r
                        - (*x.offset(i__4 as isize)).i * temp1.i;
                    z__2.i = (*x.offset(i__4 as isize)).r * temp1.i
                        + (*x.offset(i__4 as isize)).i * temp1.r;
                    i__5 = jy;
                    z__3.r = (*y.offset(i__5 as isize)).r * temp2.r
                        - (*y.offset(i__5 as isize)).i * temp2.i;
                    z__3.i = (*y.offset(i__5 as isize)).r * temp2.i
                        + (*y.offset(i__5 as isize)).i * temp2.r;
                    z__1.r = z__2.r + z__3.r;
                    z__1.i = z__2.i + z__3.i;
                    d__1 = (*ap.offset(i__3 as isize)).r + z__1.r;
                    (*ap.offset(i__2 as isize)).r = d__1;
                    (*ap.offset(i__2 as isize)).i = 0.0f64
                } else {
                    i__2 = kk + j - 1 as libc::c_int as libc::c_long;
                    i__3 = kk + j - 1 as libc::c_int as libc::c_long;
                    d__1 = (*ap.offset(i__3 as isize)).r;
                    (*ap.offset(i__2 as isize)).r = d__1;
                    (*ap.offset(i__2 as isize)).i = 0.0f64
                }
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
            i__2 = j;
            i__3 = j;
            if (*x.offset(i__2 as isize)).r != 0.0f64
                || (*x.offset(i__2 as isize)).i != 0.0f64
                || ((*y.offset(i__3 as isize)).r != 0.0f64
                    || (*y.offset(i__3 as isize)).i != 0.0f64)
            {
                d_cnjg_0(&mut z__2, &mut *y.offset(j as isize));
                z__1.r = (*alpha).r * z__2.r - (*alpha).i * z__2.i;
                z__1.i = (*alpha).r * z__2.i + (*alpha).i * z__2.r;
                temp1.r = z__1.r;
                temp1.i = z__1.i;
                i__2 = j;
                z__2.r = (*alpha).r * (*x.offset(i__2 as isize)).r
                    - (*alpha).i * (*x.offset(i__2 as isize)).i;
                z__2.i = (*alpha).r * (*x.offset(i__2 as isize)).i
                    + (*alpha).i * (*x.offset(i__2 as isize)).r;
                d_cnjg_0(&mut z__1, &mut z__2);
                temp2.r = z__1.r;
                temp2.i = z__1.i;
                i__2 = kk;
                i__3 = kk;
                i__4 = j;
                z__2.r =
                    (*x.offset(i__4 as isize)).r * temp1.r - (*x.offset(i__4 as isize)).i * temp1.i;
                z__2.i =
                    (*x.offset(i__4 as isize)).r * temp1.i + (*x.offset(i__4 as isize)).i * temp1.r;
                i__5 = j;
                z__3.r =
                    (*y.offset(i__5 as isize)).r * temp2.r - (*y.offset(i__5 as isize)).i * temp2.i;
                z__3.i =
                    (*y.offset(i__5 as isize)).r * temp2.i + (*y.offset(i__5 as isize)).i * temp2.r;
                z__1.r = z__2.r + z__3.r;
                z__1.i = z__2.i + z__3.i;
                d__1 = (*ap.offset(i__3 as isize)).r + z__1.r;
                (*ap.offset(i__2 as isize)).r = d__1;
                (*ap.offset(i__2 as isize)).i = 0.0f64;
                k = kk + 1 as libc::c_int as libc::c_long;
                i__2 = *n;
                i__ = j + 1 as libc::c_int as libc::c_long;
                while i__ <= i__2 {
                    i__3 = k;
                    i__4 = k;
                    i__5 = i__;
                    z__3.r = (*x.offset(i__5 as isize)).r * temp1.r
                        - (*x.offset(i__5 as isize)).i * temp1.i;
                    z__3.i = (*x.offset(i__5 as isize)).r * temp1.i
                        + (*x.offset(i__5 as isize)).i * temp1.r;
                    z__2.r = (*ap.offset(i__4 as isize)).r + z__3.r;
                    z__2.i = (*ap.offset(i__4 as isize)).i + z__3.i;
                    i__6 = i__;
                    z__4.r = (*y.offset(i__6 as isize)).r * temp2.r
                        - (*y.offset(i__6 as isize)).i * temp2.i;
                    z__4.i = (*y.offset(i__6 as isize)).r * temp2.i
                        + (*y.offset(i__6 as isize)).i * temp2.r;
                    z__1.r = z__2.r + z__4.r;
                    z__1.i = z__2.i + z__4.i;
                    (*ap.offset(i__3 as isize)).r = z__1.r;
                    (*ap.offset(i__3 as isize)).i = z__1.i;
                    k += 1;
                    i__ += 1
                    /*        Form  A  when lower triangle is stored in AP. */
                    /* L60: */
                    /* L50: */
                }
            } else {
                i__2 = kk;
                i__3 = kk;
                d__1 = (*ap.offset(i__3 as isize)).r;
                (*ap.offset(i__2 as isize)).r = d__1;
                (*ap.offset(i__2 as isize)).i = 0.0f64
            }
            kk = kk + *n - j + 1 as libc::c_int as libc::c_long;
            j += 1
        }
    } else {
        i__1 = *n;
        j = 1 as libc::c_int as integer;
        while j <= i__1 {
            i__2 = jx;
            i__3 = jy;
            if (*x.offset(i__2 as isize)).r != 0.0f64
                || (*x.offset(i__2 as isize)).i != 0.0f64
                || ((*y.offset(i__3 as isize)).r != 0.0f64
                    || (*y.offset(i__3 as isize)).i != 0.0f64)
            {
                d_cnjg_0(&mut z__2, &mut *y.offset(jy as isize));
                z__1.r = (*alpha).r * z__2.r - (*alpha).i * z__2.i;
                z__1.i = (*alpha).r * z__2.i + (*alpha).i * z__2.r;
                temp1.r = z__1.r;
                temp1.i = z__1.i;
                i__2 = jx;
                z__2.r = (*alpha).r * (*x.offset(i__2 as isize)).r
                    - (*alpha).i * (*x.offset(i__2 as isize)).i;
                z__2.i = (*alpha).r * (*x.offset(i__2 as isize)).i
                    + (*alpha).i * (*x.offset(i__2 as isize)).r;
                d_cnjg_0(&mut z__1, &mut z__2);
                temp2.r = z__1.r;
                temp2.i = z__1.i;
                i__2 = kk;
                i__3 = kk;
                i__4 = jx;
                z__2.r =
                    (*x.offset(i__4 as isize)).r * temp1.r - (*x.offset(i__4 as isize)).i * temp1.i;
                z__2.i =
                    (*x.offset(i__4 as isize)).r * temp1.i + (*x.offset(i__4 as isize)).i * temp1.r;
                i__5 = jy;
                z__3.r =
                    (*y.offset(i__5 as isize)).r * temp2.r - (*y.offset(i__5 as isize)).i * temp2.i;
                z__3.i =
                    (*y.offset(i__5 as isize)).r * temp2.i + (*y.offset(i__5 as isize)).i * temp2.r;
                z__1.r = z__2.r + z__3.r;
                z__1.i = z__2.i + z__3.i;
                d__1 = (*ap.offset(i__3 as isize)).r + z__1.r;
                (*ap.offset(i__2 as isize)).r = d__1;
                (*ap.offset(i__2 as isize)).i = 0.0f64;
                ix = jx;
                iy = jy;
                i__2 = kk + *n - j;
                k = kk + 1 as libc::c_int as libc::c_long;
                while k <= i__2 {
                    ix += *incx;
                    iy += *incy;
                    i__3 = k;
                    i__4 = k;
                    i__5 = ix;
                    z__3.r = (*x.offset(i__5 as isize)).r * temp1.r
                        - (*x.offset(i__5 as isize)).i * temp1.i;
                    z__3.i = (*x.offset(i__5 as isize)).r * temp1.i
                        + (*x.offset(i__5 as isize)).i * temp1.r;
                    z__2.r = (*ap.offset(i__4 as isize)).r + z__3.r;
                    z__2.i = (*ap.offset(i__4 as isize)).i + z__3.i;
                    i__6 = iy;
                    z__4.r = (*y.offset(i__6 as isize)).r * temp2.r
                        - (*y.offset(i__6 as isize)).i * temp2.i;
                    z__4.i = (*y.offset(i__6 as isize)).r * temp2.i
                        + (*y.offset(i__6 as isize)).i * temp2.r;
                    z__1.r = z__2.r + z__4.r;
                    z__1.i = z__2.i + z__4.i;
                    (*ap.offset(i__3 as isize)).r = z__1.r;
                    (*ap.offset(i__3 as isize)).i = z__1.i;
                    k += 1
                    /* L80: */
                    /* L70: */
                }
            } else {
                i__2 = kk;
                i__3 = kk;
                d__1 = (*ap.offset(i__3 as isize)).r;
                (*ap.offset(i__2 as isize)).r = d__1;
                (*ap.offset(i__2 as isize)).i = 0.0f64
            }
            jx += *incx;
            jy += *incy;
            kk = kk + *n - j + 1 as libc::c_int as libc::c_long;
            j += 1
        }
    }
    return 0 as libc::c_int;
    /*     End of ZHPR2 . */
}
/* zhpr2_ */
