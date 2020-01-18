use libc;
/* f2c.h  --  Standard Fortran to C header file */
/* *  barf  [ba:rf]  2.  "He suggested using FORTRAN, and everybody barfed."

- From The Shogakukan DICTIONARY OF NEW ENGLISH (Second edition) */
pub type integer = libc::c_long;
pub type doublereal = libc::c_double;
pub type logical = libc::c_long;
/* dgemv.f -- translated by f2c (version 20061008).
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
pub unsafe extern "C" fn f2c_dgemv(
    mut trans: *mut libc::c_char,
    mut m: *mut integer,
    mut n: *mut integer,
    mut alpha: *mut doublereal,
    mut a: *mut doublereal,
    mut lda: *mut integer,
    mut x: *mut doublereal,
    mut incx: *mut integer,
    mut beta: *mut doublereal,
    mut y: *mut doublereal,
    mut incy: *mut integer,
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
    let mut temp: doublereal = 0.;
    let mut lenx: integer = 0;
    let mut leny: integer = 0;
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
    /*  DGEMV  performs one of the matrix-vector operations */
    /*     y := alpha*A*x + beta*y,   or   y := alpha*A'*x + beta*y, */
    /*  where alpha and beta are scalars, x and y are vectors and A is an */
    /*  m by n matrix. */
    /*  Arguments */
    /*  ========== */
    /*  TRANS  - CHARACTER*1. */
    /*           On entry, TRANS specifies the operation to be performed as */
    /*           follows: */
    /*              TRANS = 'N' or 'n'   y := alpha*A*x + beta*y. */
    /*              TRANS = 'T' or 't'   y := alpha*A'*x + beta*y. */
    /*              TRANS = 'C' or 'c'   y := alpha*A'*x + beta*y. */
    /*           Unchanged on exit. */
    /*  M      - INTEGER. */
    /*           On entry, M specifies the number of rows of the matrix A. */
    /*           M must be at least zero. */
    /*           Unchanged on exit. */
    /*  N      - INTEGER. */
    /*           On entry, N specifies the number of columns of the matrix A. */
    /*           N must be at least zero. */
    /*           Unchanged on exit. */
    /*  ALPHA  - DOUBLE PRECISION. */
    /*           On entry, ALPHA specifies the scalar alpha. */
    /*           Unchanged on exit. */
    /*  A      - DOUBLE PRECISION array of DIMENSION ( LDA, n ). */
    /*           Before entry, the leading m by n part of the array A must */
    /*           contain the matrix of coefficients. */
    /*           Unchanged on exit. */
    /*  LDA    - INTEGER. */
    /*           On entry, LDA specifies the first dimension of A as declared */
    /*           in the calling (sub) program. LDA must be at least */
    /*           max( 1, m ). */
    /*           Unchanged on exit. */
    /*  X      - DOUBLE PRECISION array of DIMENSION at least */
    /*           ( 1 + ( n - 1 )*abs( INCX ) ) when TRANS = 'N' or 'n' */
    /*           and at least */
    /*           ( 1 + ( m - 1 )*abs( INCX ) ) otherwise. */
    /*           Before entry, the incremented array X must contain the */
    /*           vector x. */
    /*           Unchanged on exit. */
    /*  INCX   - INTEGER. */
    /*           On entry, INCX specifies the increment for the elements of */
    /*           X. INCX must not be zero. */
    /*           Unchanged on exit. */
    /*  BETA   - DOUBLE PRECISION. */
    /*           On entry, BETA specifies the scalar beta. When BETA is */
    /*           supplied as zero then Y need not be set on input. */
    /*           Unchanged on exit. */
    /*  Y      - DOUBLE PRECISION array of DIMENSION at least */
    /*           ( 1 + ( m - 1 )*abs( INCY ) ) when TRANS = 'N' or 'n' */
    /*           and at least */
    /*           ( 1 + ( n - 1 )*abs( INCY ) ) otherwise. */
    /*           Before entry with BETA non-zero, the incremented array Y */
    /*           must contain the vector y. On exit, Y is overwritten by the */
    /*           updated vector y. */
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
    /*     .. Intrinsic Functions .. */
    /*     .. */
    /*     Test the input parameters. */
    /* Parameter adjustments */
    a_dim1 = *lda;
    a_offset = 1 as libc::c_int as libc::c_long + a_dim1;
    a = a.offset(-(a_offset as isize));
    x = x.offset(-1);
    y = y.offset(-1);
    /* Function Body */
    info = 0 as libc::c_int as integer;
    if lsame__0(
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
        info = 1 as libc::c_int as integer
    } else if *m < 0 as libc::c_int as libc::c_long {
        info = 2 as libc::c_int as integer
    } else if *n < 0 as libc::c_int as libc::c_long {
        info = 3 as libc::c_int as integer
    } else if *lda
        < (if 1 as libc::c_int as libc::c_long >= *m {
            1 as libc::c_int as libc::c_long
        } else {
            *m
        })
    {
        info = 6 as libc::c_int as integer
    } else if *incx == 0 as libc::c_int as libc::c_long {
        info = 8 as libc::c_int as integer
    } else if *incy == 0 as libc::c_int as libc::c_long {
        info = 11 as libc::c_int as integer
    }
    if info != 0 as libc::c_int as libc::c_long {
        xerbla__0(
            b"DGEMV \x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            &mut info,
        );
        return 0 as libc::c_int;
    }
    /*     Quick return if possible. */
    if *m == 0 as libc::c_int as libc::c_long
        || *n == 0 as libc::c_int as libc::c_long
        || *alpha == 0.0f64 && *beta == 1.0f64
    {
        return 0 as libc::c_int;
    }
    /*     Set  LENX  and  LENY, the lengths of the vectors x and y, and set */
    /*     up the start points in  X  and  Y. */
    if lsame__0(
        trans,
        b"N\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) != 0
    {
        lenx = *n;
        leny = *m
    } else {
        lenx = *m;
        leny = *n
    }
    if *incx > 0 as libc::c_int as libc::c_long {
        kx = 1 as libc::c_int as integer
    } else {
        kx = 1 as libc::c_int as libc::c_long - (lenx - 1 as libc::c_int as libc::c_long) * *incx
    }
    if *incy > 0 as libc::c_int as libc::c_long {
        ky = 1 as libc::c_int as integer
    } else {
        ky = 1 as libc::c_int as libc::c_long - (leny - 1 as libc::c_int as libc::c_long) * *incy
    }
    /*     Start the operations. In this version the elements of A are */
    /*     accessed sequentially with one pass through A. */
    /*     First form  y := beta*y. */
    if *beta != 1.0f64 {
        if *incy == 1 as libc::c_int as libc::c_long {
            if *beta == 0.0f64 {
                i__1 = leny;
                i__ = 1 as libc::c_int as integer;
                while i__ <= i__1 {
                    *y.offset(i__ as isize) = 0.0f64;
                    i__ += 1
                    /* L10: */
                }
            } else {
                i__1 = leny;
                i__ = 1 as libc::c_int as integer;
                while i__ <= i__1 {
                    *y.offset(i__ as isize) = *beta * *y.offset(i__ as isize);
                    i__ += 1
                    /* L20: */
                }
            }
        } else {
            iy = ky;
            if *beta == 0.0f64 {
                i__1 = leny;
                i__ = 1 as libc::c_int as integer;
                while i__ <= i__1 {
                    *y.offset(iy as isize) = 0.0f64;
                    iy += *incy;
                    i__ += 1
                    /* L30: */
                }
            } else {
                i__1 = leny;
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
    if *alpha == 0.0f64 {
        return 0 as libc::c_int;
    }
    if lsame__0(
        trans,
        b"N\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) != 0
    {
        /*        Form  y := alpha*A*x + y. */
        jx = kx;
        if *incy == 1 as libc::c_int as libc::c_long {
            i__1 = *n;
            j = 1 as libc::c_int as integer;
            while j <= i__1 {
                if *x.offset(jx as isize) != 0.0f64 {
                    temp = *alpha * *x.offset(jx as isize);
                    i__2 = *m;
                    i__ = 1 as libc::c_int as integer;
                    while i__ <= i__2 {
                        let ref mut fresh0 = *y.offset(i__ as isize);
                        *fresh0 += temp * *a.offset((i__ + j * a_dim1) as isize);
                        i__ += 1
                        /* L50: */
                    }
                }
                jx += *incx;
                j += 1
                /* L60: */
            }
        } else {
            i__1 = *n;
            j = 1 as libc::c_int as integer;
            while j <= i__1 {
                if *x.offset(jx as isize) != 0.0f64 {
                    temp = *alpha * *x.offset(jx as isize);
                    iy = ky;
                    i__2 = *m;
                    i__ = 1 as libc::c_int as integer;
                    while i__ <= i__2 {
                        let ref mut fresh1 = *y.offset(iy as isize);
                        *fresh1 += temp * *a.offset((i__ + j * a_dim1) as isize);
                        iy += *incy;
                        i__ += 1
                        /* L70: */
                    }
                }
                jx += *incx;
                j += 1
                /* L80: */
            }
        }
    } else {
        /*        Form  y := alpha*A'*x + y. */
        jy = ky;
        if *incx == 1 as libc::c_int as libc::c_long {
            i__1 = *n;
            j = 1 as libc::c_int as integer;
            while j <= i__1 {
                temp = 0.0f64;
                i__2 = *m;
                i__ = 1 as libc::c_int as integer;
                while i__ <= i__2 {
                    temp += *a.offset((i__ + j * a_dim1) as isize) * *x.offset(i__ as isize);
                    i__ += 1
                    /* L100: */
                    /* L90: */
                }
                let ref mut fresh2 = *y.offset(jy as isize);
                *fresh2 += *alpha * temp;
                jy += *incy;
                j += 1
            }
        } else {
            i__1 = *n;
            j = 1 as libc::c_int as integer;
            while j <= i__1 {
                temp = 0.0f64;
                ix = kx;
                i__2 = *m;
                i__ = 1 as libc::c_int as integer;
                while i__ <= i__2 {
                    temp += *a.offset((i__ + j * a_dim1) as isize) * *x.offset(ix as isize);
                    ix += *incx;
                    i__ += 1
                    /* L120: */
                    /* L110: */
                }
                let ref mut fresh3 = *y.offset(jy as isize);
                *fresh3 += *alpha * temp;
                jy += *incy;
                j += 1
            }
        }
    }
    return 0 as libc::c_int;
    /*     End of DGEMV . */
}
/* dgemv_ */
