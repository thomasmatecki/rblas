use libc;
/* f2c.h  --  Standard Fortran to C header file */
/* *  barf  [ba:rf]  2.  "He suggested using FORTRAN, and everybody barfed."

- From The Shogakukan DICTIONARY OF NEW ENGLISH (Second edition) */
pub type integer = libc::c_long;
pub type real = libc::c_float;
pub type logical = libc::c_long;
/* ssyr2.f -- translated by f2c (version 20061008).
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
pub unsafe extern "C" fn f2c_ssyr2(
    mut uplo: *mut libc::c_char,
    mut n: *mut integer,
    mut alpha: *mut real,
    mut x: *mut real,
    mut incx: *mut integer,
    mut y: *mut real,
    mut incy: *mut integer,
    mut a: *mut real,
    mut lda: *mut integer,
) -> libc::c_int {
    /* System generated locals */
    let mut a_dim1: integer = 0;
    let mut a_offset: integer = 0;
    let mut i__1: integer = 0;
    let mut i__2: integer = 0;
    /* Local variables */
    let mut i__: integer = 0;
    let mut j: integer = 0;
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
    /*  SSYR2  performs the symmetric rank 2 operation */
    /*     A := alpha*x*y' + alpha*y*x' + A, */
    /*  where alpha is a scalar, x and y are n element vectors and A is an n */
    /*  by n symmetric matrix. */
    /*  Arguments */
    /*  ========== */
    /*  UPLO   - CHARACTER*1. */
    /*           On entry, UPLO specifies whether the upper or lower */
    /*           triangular part of the array A is to be referenced as */
    /*           follows: */
    /*              UPLO = 'U' or 'u'   Only the upper triangular part of A */
    /*                                  is to be referenced. */
    /*              UPLO = 'L' or 'l'   Only the lower triangular part of A */
    /*                                  is to be referenced. */
    /*           Unchanged on exit. */
    /*  N      - INTEGER. */
    /*           On entry, N specifies the order of the matrix A. */
    /*           N must be at least zero. */
    /*           Unchanged on exit. */
    /*  ALPHA  - REAL            . */
    /*           On entry, ALPHA specifies the scalar alpha. */
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
    /*  Y      - REAL             array of dimension at least */
    /*           ( 1 + ( n - 1 )*abs( INCY ) ). */
    /*           Before entry, the incremented array Y must contain the n */
    /*           element vector y. */
    /*           Unchanged on exit. */
    /*  INCY   - INTEGER. */
    /*           On entry, INCY specifies the increment for the elements of */
    /*           Y. INCY must not be zero. */
    /*           Unchanged on exit. */
    /*  A      - REAL             array of DIMENSION ( LDA, n ). */
    /*           Before entry with  UPLO = 'U' or 'u', the leading n by n */
    /*           upper triangular part of the array A must contain the upper */
    /*           triangular part of the symmetric matrix and the strictly */
    /*           lower triangular part of A is not referenced. On exit, the */
    /*           upper triangular part of the array A is overwritten by the */
    /*           upper triangular part of the updated matrix. */
    /*           Before entry with UPLO = 'L' or 'l', the leading n by n */
    /*           lower triangular part of the array A must contain the lower */
    /*           triangular part of the symmetric matrix and the strictly */
    /*           upper triangular part of A is not referenced. On exit, the */
    /*           lower triangular part of the array A is overwritten by the */
    /*           lower triangular part of the updated matrix. */
    /*  LDA    - INTEGER. */
    /*           On entry, LDA specifies the first dimension of A as declared */
    /*           in the calling (sub) program. LDA must be at least */
    /*           max( 1, n ). */
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
    y = y.offset(-1);
    a_dim1 = *lda;
    a_offset = 1 as libc::c_int as libc::c_long + a_dim1;
    a = a.offset(-(a_offset as isize));
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
    } else if *lda
        < (if 1 as libc::c_int as libc::c_long >= *n {
            1 as libc::c_int as libc::c_long
        } else {
            *n
        })
    {
        info = 9 as libc::c_int as integer
    }
    if info != 0 as libc::c_int as libc::c_long {
        xerbla__0(
            b"SSYR2 \x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            &mut info,
        );
        return 0 as libc::c_int;
    }
    /*     Quick return if possible. */
    if *n == 0 as libc::c_int as libc::c_long || *alpha == 0.0f32 {
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
    /*     Start the operations. In this version the elements of A are */
    /*     accessed sequentially with one pass through the triangular part */
    /*     of A. */
    if lsame__0(
        uplo,
        b"U\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) != 0
    {
        /*        Form  A  when A is stored in the upper triangle. */
        if *incx == 1 as libc::c_int as libc::c_long && *incy == 1 as libc::c_int as libc::c_long {
            i__1 = *n;
            j = 1 as libc::c_int as integer;
            while j <= i__1 {
                if *x.offset(j as isize) != 0.0f32 || *y.offset(j as isize) != 0.0f32 {
                    temp1 = *alpha * *y.offset(j as isize);
                    temp2 = *alpha * *x.offset(j as isize);
                    i__2 = j;
                    i__ = 1 as libc::c_int as integer;
                    while i__ <= i__2 {
                        *a.offset((i__ + j * a_dim1) as isize) = *a
                            .offset((i__ + j * a_dim1) as isize)
                            + *x.offset(i__ as isize) * temp1
                            + *y.offset(i__ as isize) * temp2;
                        i__ += 1
                        /* L10: */
                    }
                }
                j += 1
                /* L20: */
            }
        } else {
            i__1 = *n;
            j = 1 as libc::c_int as integer;
            while j <= i__1 {
                if *x.offset(jx as isize) != 0.0f32 || *y.offset(jy as isize) != 0.0f32 {
                    temp1 = *alpha * *y.offset(jy as isize);
                    temp2 = *alpha * *x.offset(jx as isize);
                    ix = kx;
                    iy = ky;
                    i__2 = j;
                    i__ = 1 as libc::c_int as integer;
                    while i__ <= i__2 {
                        *a.offset((i__ + j * a_dim1) as isize) = *a
                            .offset((i__ + j * a_dim1) as isize)
                            + *x.offset(ix as isize) * temp1
                            + *y.offset(iy as isize) * temp2;
                        ix += *incx;
                        iy += *incy;
                        i__ += 1
                        /* L30: */
                    }
                }
                jx += *incx;
                jy += *incy;
                j += 1
                /* L40: */
            }
        }
    } else if *incx == 1 as libc::c_int as libc::c_long && *incy == 1 as libc::c_int as libc::c_long
    {
        i__1 = *n;
        j = 1 as libc::c_int as integer;
        while j <= i__1 {
            if *x.offset(j as isize) != 0.0f32 || *y.offset(j as isize) != 0.0f32 {
                temp1 = *alpha * *y.offset(j as isize);
                temp2 = *alpha * *x.offset(j as isize);
                i__2 = *n;
                i__ = j;
                while i__ <= i__2 {
                    *a.offset((i__ + j * a_dim1) as isize) = *a.offset((i__ + j * a_dim1) as isize)
                        + *x.offset(i__ as isize) * temp1
                        + *y.offset(i__ as isize) * temp2;
                    i__ += 1
                    /*        Form  A  when A is stored in the lower triangle. */
                    /* L50: */
                }
            }
            j += 1
            /* L60: */
        }
    } else {
        i__1 = *n;
        j = 1 as libc::c_int as integer;
        while j <= i__1 {
            if *x.offset(jx as isize) != 0.0f32 || *y.offset(jy as isize) != 0.0f32 {
                temp1 = *alpha * *y.offset(jy as isize);
                temp2 = *alpha * *x.offset(jx as isize);
                ix = jx;
                iy = jy;
                i__2 = *n;
                i__ = j;
                while i__ <= i__2 {
                    *a.offset((i__ + j * a_dim1) as isize) = *a.offset((i__ + j * a_dim1) as isize)
                        + *x.offset(ix as isize) * temp1
                        + *y.offset(iy as isize) * temp2;
                    ix += *incx;
                    iy += *incy;
                    i__ += 1
                    /* L70: */
                }
            }
            jx += *incx;
            jy += *incy;
            j += 1
            /* L80: */
        }
    }
    return 0 as libc::c_int;
    /*     End of SSYR2 . */
}
/* ssyr2_ */
