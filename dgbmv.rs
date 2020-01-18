use ::libc;
/* f2c.h  --  Standard Fortran to C header file */
/* *  barf  [ba:rf]  2.  "He suggested using FORTRAN, and everybody barfed."

	- From The Shogakukan DICTIONARY OF NEW ENGLISH (Second edition) */
pub type integer = libc::c_long;
pub type doublereal = libc::c_double;
pub type logical = libc::c_long;
/* dgbmv.f -- translated by f2c (version 20061008).
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
pub unsafe extern "C" fn f2c_dgbmv(mut trans: *mut libc::c_char,
                                   mut m: *mut integer, mut n: *mut integer,
                                   mut kl: *mut integer, mut ku: *mut integer,
                                   mut alpha: *mut doublereal,
                                   mut a: *mut doublereal,
                                   mut lda: *mut integer,
                                   mut x: *mut doublereal,
                                   mut incx: *mut integer,
                                   mut beta: *mut doublereal,
                                   mut y: *mut doublereal,
                                   mut incy: *mut integer) -> libc::c_int {
    /* System generated locals */
    let mut a_dim1: integer = 0;
    let mut a_offset: integer = 0;
    let mut i__1: integer = 0;
    let mut i__2: integer = 0;
    let mut i__3: integer = 0;
    let mut i__4: integer = 0;
    let mut i__5: integer = 0;
    let mut i__6: integer = 0;
    /* Local variables */
    let mut i__: integer = 0;
    let mut j: integer = 0;
    let mut k: integer = 0;
    let mut ix: integer = 0;
    let mut iy: integer = 0;
    let mut jx: integer = 0;
    let mut jy: integer = 0;
    let mut kx: integer = 0;
    let mut ky: integer = 0;
    let mut kup1: integer = 0;
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
    /*  DGBMV  performs one of the matrix-vector operations */
    /*     y := alpha*A*x + beta*y,   or   y := alpha*A'*x + beta*y, */
    /*  where alpha and beta are scalars, x and y are vectors and A is an */
/*  m by n band matrix, with kl sub-diagonals and ku super-diagonals. */
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
    /*  KL     - INTEGER. */
/*           On entry, KL specifies the number of sub-diagonals of the */
/*           matrix A. KL must satisfy  0 .le. KL. */
/*           Unchanged on exit. */
    /*  KU     - INTEGER. */
/*           On entry, KU specifies the number of super-diagonals of the */
/*           matrix A. KU must satisfy  0 .le. KU. */
/*           Unchanged on exit. */
    /*  ALPHA  - DOUBLE PRECISION. */
/*           On entry, ALPHA specifies the scalar alpha. */
/*           Unchanged on exit. */
    /*  A      - DOUBLE PRECISION array of DIMENSION ( LDA, n ). */
/*           Before entry, the leading ( kl + ku + 1 ) by n part of the */
/*           array A must contain the matrix of coefficients, supplied */
/*           column by column, with the leading diagonal of the matrix in */
/*           row ( ku + 1 ) of the array, the first super-diagonal */
/*           starting at position 2 in row ku, the first sub-diagonal */
/*           starting at position 1 in row ( ku + 2 ), and so on. */
/*           Elements in the array A that do not correspond to elements */
/*           in the band matrix (such as the top left ku by ku triangle) */
/*           are not referenced. */
/*           The following program segment will transfer a band matrix */
/*           from conventional full matrix storage to band storage: */
    /*                 DO 20, J = 1, N */
/*                    K = KU + 1 - J */
/*                    DO 10, I = MAX( 1, J - KU ), MIN( M, J + KL ) */
/*                       A( K + I, J ) = matrix( I, J ) */
/*              10    CONTINUE */
/*              20 CONTINUE */
    /*           Unchanged on exit. */
    /*  LDA    - INTEGER. */
/*           On entry, LDA specifies the first dimension of A as declared */
/*           in the calling (sub) program. LDA must be at least */
/*           ( kl + ku + 1 ). */
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
/*           Before entry, the incremented array Y must contain the */
/*           vector y. On exit, Y is overwritten by the updated vector y. */
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
    if lsame__0(trans,
                b"N\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char) == 0 &&
           lsame__0(trans,
                    b"T\x00" as *const u8 as *const libc::c_char as
                        *mut libc::c_char) == 0 &&
           lsame__0(trans,
                    b"C\x00" as *const u8 as *const libc::c_char as
                        *mut libc::c_char) == 0 {
        info = 1 as libc::c_int as integer
    } else if *m < 0 as libc::c_int as libc::c_long {
        info = 2 as libc::c_int as integer
    } else if *n < 0 as libc::c_int as libc::c_long {
        info = 3 as libc::c_int as integer
    } else if *kl < 0 as libc::c_int as libc::c_long {
        info = 4 as libc::c_int as integer
    } else if *ku < 0 as libc::c_int as libc::c_long {
        info = 5 as libc::c_int as integer
    } else if *lda < *kl + *ku + 1 as libc::c_int as libc::c_long {
        info = 8 as libc::c_int as integer
    } else if *incx == 0 as libc::c_int as libc::c_long {
        info = 10 as libc::c_int as integer
    } else if *incy == 0 as libc::c_int as libc::c_long {
        info = 13 as libc::c_int as integer
    }
    if info != 0 as libc::c_int as libc::c_long {
        xerbla__0(b"DGBMV \x00" as *const u8 as *const libc::c_char as
                      *mut libc::c_char, &mut info);
        return 0 as libc::c_int
    }
    /*     Quick return if possible. */
    if *m == 0 as libc::c_int as libc::c_long ||
           *n == 0 as libc::c_int as libc::c_long ||
           *alpha == 0.0f64 && *beta == 1.0f64 {
        return 0 as libc::c_int
    }
    /*     Set  LENX  and  LENY, the lengths of the vectors x and y, and set */
/*     up the start points in  X  and  Y. */
    if lsame__0(trans,
                b"N\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char) != 0 {
        lenx = *n;
        leny = *m
    } else { lenx = *m; leny = *n }
    if *incx > 0 as libc::c_int as libc::c_long {
        kx = 1 as libc::c_int as integer
    } else {
        kx =
            1 as libc::c_int as libc::c_long -
                (lenx - 1 as libc::c_int as libc::c_long) * *incx
    }
    if *incy > 0 as libc::c_int as libc::c_long {
        ky = 1 as libc::c_int as integer
    } else {
        ky =
            1 as libc::c_int as libc::c_long -
                (leny - 1 as libc::c_int as libc::c_long) * *incy
    }
    /*     Start the operations. In this version the elements of A are */
/*     accessed sequentially with one pass through the band part of A. */
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
    if *alpha == 0.0f64 { return 0 as libc::c_int }
    kup1 = *ku + 1 as libc::c_int as libc::c_long;
    if lsame__0(trans,
                b"N\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char) != 0 {
        /*        Form  y := alpha*A*x + y. */
        jx = kx;
        if *incy == 1 as libc::c_int as libc::c_long {
            i__1 = *n;
            j = 1 as libc::c_int as integer;
            while j <= i__1 {
                if *x.offset(jx as isize) != 0.0f64 {
                    temp = *alpha * *x.offset(jx as isize);
                    k = kup1 - j;
                    /* Computing MAX */
                    i__2 = 1 as libc::c_int as integer;
                    i__3 = j - *ku;
                    /* Computing MIN */
                    i__5 = *m;
                    i__6 = j + *kl;
                    i__4 = if i__5 <= i__6 { i__5 } else { i__6 };
                    i__ = if i__2 >= i__3 { i__2 } else { i__3 };
                    while i__ <= i__4 {
                        let ref mut fresh0 = *y.offset(i__ as isize);
                        *fresh0 +=
                            temp * *a.offset((k + i__ + j * a_dim1) as isize);
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
                    k = kup1 - j;
                    /* Computing MAX */
                    i__4 = 1 as libc::c_int as integer;
                    i__2 = j - *ku;
                    /* Computing MIN */
                    i__5 = *m;
                    i__6 = j + *kl;
                    i__3 = if i__5 <= i__6 { i__5 } else { i__6 };
                    i__ = if i__4 >= i__2 { i__4 } else { i__2 };
                    while i__ <= i__3 {
                        let ref mut fresh1 = *y.offset(iy as isize);
                        *fresh1 +=
                            temp * *a.offset((k + i__ + j * a_dim1) as isize);
                        iy += *incy;
                        i__ += 1
                        /* L70: */
                    }
                }
                jx += *incx;
                if j > *ku { ky += *incy }
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
                k = kup1 - j;
                /* L100: */
                i__3 = 1 as libc::c_int as integer;
                i__4 = j - *ku;
                i__5 = *m;
                i__6 = j + *kl;
                i__2 = if i__5 <= i__6 { i__5 } else { i__6 };
                i__ = if i__3 >= i__4 { i__3 } else { i__4 };
                while i__ <= i__2 {
                    temp +=
                        *a.offset((k + i__ + j * a_dim1) as isize) *
                            *x.offset(i__ as isize);
                    i__ += 1
                    /* Computing MAX */
                    /* Computing MIN */
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
                k = kup1 - j;
                /* L120: */
                i__2 = 1 as libc::c_int as integer;
                i__3 = j - *ku;
                i__5 = *m;
                i__6 = j + *kl;
                i__4 = if i__5 <= i__6 { i__5 } else { i__6 };
                i__ = if i__2 >= i__3 { i__2 } else { i__3 };
                while i__ <= i__4 {
                    temp +=
                        *a.offset((k + i__ + j * a_dim1) as isize) *
                            *x.offset(ix as isize);
                    ix += *incx;
                    i__ += 1
                    /* Computing MAX */
                    /* Computing MIN */
                    /* L110: */
                }
                let ref mut fresh3 = *y.offset(jy as isize);
                *fresh3 += *alpha * temp;
                jy += *incy;
                if j > *ku { kx += *incx }
                j += 1
            }
        }
    }
    return 0 as libc::c_int;
    /*     End of DGBMV . */
}
/* dgbmv_ */
