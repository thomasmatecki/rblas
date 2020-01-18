use libc;
/* f2c.h  --  Standard Fortran to C header file */
/* *  barf  [ba:rf]  2.  "He suggested using FORTRAN, and everybody barfed."

- From The Shogakukan DICTIONARY OF NEW ENGLISH (Second edition) */
pub type integer = libc::c_long;
pub type real = libc::c_float;
pub type logical = libc::c_long;
/* strmv.f -- translated by f2c (version 20061008).
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
pub unsafe extern "C" fn f2c_strmv(
    mut uplo: *mut libc::c_char,
    mut trans: *mut libc::c_char,
    mut diag: *mut libc::c_char,
    mut n: *mut integer,
    mut a: *mut real,
    mut lda: *mut integer,
    mut x: *mut real,
    mut incx: *mut integer,
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
    let mut jx: integer = 0;
    let mut kx: integer = 0;
    let mut info: integer = 0;
    let mut temp: real = 0.;
    extern "C" {
        #[link_name = "lsame_"]
        fn lsame__0(_: *mut libc::c_char, _: *mut libc::c_char) -> logical;
    }
    extern "C" {
        #[link_name = "xerbla_"]
        fn xerbla__0(_: *mut libc::c_char, _: *mut integer) -> libc::c_int;
    }
    let mut nounit: logical = 0;
    /*     .. Scalar Arguments .. */
    /*     .. */
    /*     .. Array Arguments .. */
    /*     .. */
    /*  Purpose */
    /*  ======= */
    /*  STRMV  performs one of the matrix-vector operations */
    /*     x := A*x,   or   x := A'*x, */
    /*  where x is an n element vector and  A is an n by n unit, or non-unit, */
    /*  upper or lower triangular matrix. */
    /*  Arguments */
    /*  ========== */
    /*  UPLO   - CHARACTER*1. */
    /*           On entry, UPLO specifies whether the matrix is an upper or */
    /*           lower triangular matrix as follows: */
    /*              UPLO = 'U' or 'u'   A is an upper triangular matrix. */
    /*              UPLO = 'L' or 'l'   A is a lower triangular matrix. */
    /*           Unchanged on exit. */
    /*  TRANS  - CHARACTER*1. */
    /*           On entry, TRANS specifies the operation to be performed as */
    /*           follows: */
    /*              TRANS = 'N' or 'n'   x := A*x. */
    /*              TRANS = 'T' or 't'   x := A'*x. */
    /*              TRANS = 'C' or 'c'   x := A'*x. */
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
    /*  A      - REAL             array of DIMENSION ( LDA, n ). */
    /*           Before entry with  UPLO = 'U' or 'u', the leading n by n */
    /*           upper triangular part of the array A must contain the upper */
    /*           triangular matrix and the strictly lower triangular part of */
    /*           A is not referenced. */
    /*           Before entry with UPLO = 'L' or 'l', the leading n by n */
    /*           lower triangular part of the array A must contain the lower */
    /*           triangular matrix and the strictly upper triangular part of */
    /*           A is not referenced. */
    /*           Note that when  DIAG = 'U' or 'u', the diagonal elements of */
    /*           A are not referenced either, but are assumed to be unity. */
    /*           Unchanged on exit. */
    /*  LDA    - INTEGER. */
    /*           On entry, LDA specifies the first dimension of A as declared */
    /*           in the calling (sub) program. LDA must be at least */
    /*           max( 1, n ). */
    /*           Unchanged on exit. */
    /*  X      - REAL             array of dimension at least */
    /*           ( 1 + ( n - 1 )*abs( INCX ) ). */
    /*           Before entry, the incremented array X must contain the n */
    /*           element vector x. On exit, X is overwritten with the */
    /*           tranformed vector x. */
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
    a_dim1 = *lda;
    a_offset = 1 as libc::c_int as libc::c_long + a_dim1;
    a = a.offset(-(a_offset as isize));
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
    } else if *lda
        < (if 1 as libc::c_int as libc::c_long >= *n {
            1 as libc::c_int as libc::c_long
        } else {
            *n
        })
    {
        info = 6 as libc::c_int as integer
    } else if *incx == 0 as libc::c_int as libc::c_long {
        info = 8 as libc::c_int as integer
    }
    if info != 0 as libc::c_int as libc::c_long {
        xerbla__0(
            b"STRMV \x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            &mut info,
        );
        return 0 as libc::c_int;
    }
    /*     Quick return if possible. */
    if *n == 0 as libc::c_int as libc::c_long {
        return 0 as libc::c_int;
    }
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
    /*     Start the operations. In this version the elements of A are */
    /*     accessed sequentially with one pass through A. */
    if lsame__0(
        trans,
        b"N\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) != 0
    {
        /*        Form  x := A*x. */
        if lsame__0(
            uplo,
            b"U\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) != 0
        {
            if *incx == 1 as libc::c_int as libc::c_long {
                i__1 = *n;
                j = 1 as libc::c_int as integer;
                while j <= i__1 {
                    if *x.offset(j as isize) != 0.0f32 {
                        temp = *x.offset(j as isize);
                        i__2 = j - 1 as libc::c_int as libc::c_long;
                        i__ = 1 as libc::c_int as integer;
                        while i__ <= i__2 {
                            let ref mut fresh0 = *x.offset(i__ as isize);
                            *fresh0 += temp * *a.offset((i__ + j * a_dim1) as isize);
                            i__ += 1
                            /* L10: */
                        }
                        if nounit != 0 {
                            let ref mut fresh1 = *x.offset(j as isize);
                            *fresh1 *= *a.offset((j + j * a_dim1) as isize)
                        }
                    }
                    j += 1
                    /* L20: */
                }
            } else {
                jx = kx;
                i__1 = *n;
                j = 1 as libc::c_int as integer;
                while j <= i__1 {
                    if *x.offset(jx as isize) != 0.0f32 {
                        temp = *x.offset(jx as isize);
                        ix = kx;
                        i__2 = j - 1 as libc::c_int as libc::c_long;
                        i__ = 1 as libc::c_int as integer;
                        while i__ <= i__2 {
                            let ref mut fresh2 = *x.offset(ix as isize);
                            *fresh2 += temp * *a.offset((i__ + j * a_dim1) as isize);
                            ix += *incx;
                            i__ += 1
                            /* L30: */
                        }
                        if nounit != 0 {
                            let ref mut fresh3 = *x.offset(jx as isize);
                            *fresh3 *= *a.offset((j + j * a_dim1) as isize)
                        }
                    }
                    jx += *incx;
                    j += 1
                    /* L40: */
                }
            }
        } else if *incx == 1 as libc::c_int as libc::c_long {
            j = *n;
            while j >= 1 as libc::c_int as libc::c_long {
                if *x.offset(j as isize) != 0.0f32 {
                    temp = *x.offset(j as isize);
                    i__1 = j + 1 as libc::c_int as libc::c_long;
                    i__ = *n;
                    while i__ >= i__1 {
                        let ref mut fresh4 = *x.offset(i__ as isize);
                        *fresh4 += temp * *a.offset((i__ + j * a_dim1) as isize);
                        i__ -= 1
                        /* L50: */
                    }
                    if nounit != 0 {
                        let ref mut fresh5 = *x.offset(j as isize);
                        *fresh5 *= *a.offset((j + j * a_dim1) as isize)
                    }
                }
                j -= 1
                /* L60: */
            }
        } else {
            kx += (*n - 1 as libc::c_int as libc::c_long) * *incx;
            jx = kx;
            j = *n;
            while j >= 1 as libc::c_int as libc::c_long {
                if *x.offset(jx as isize) != 0.0f32 {
                    temp = *x.offset(jx as isize);
                    ix = kx;
                    i__1 = j + 1 as libc::c_int as libc::c_long;
                    i__ = *n;
                    while i__ >= i__1 {
                        let ref mut fresh6 = *x.offset(ix as isize);
                        *fresh6 += temp * *a.offset((i__ + j * a_dim1) as isize);
                        ix -= *incx;
                        i__ -= 1
                        /* L70: */
                    }
                    if nounit != 0 {
                        let ref mut fresh7 = *x.offset(jx as isize);
                        *fresh7 *= *a.offset((j + j * a_dim1) as isize)
                    }
                }
                jx -= *incx;
                j -= 1
                /* L80: */
            }
        }
    } else if lsame__0(
        uplo,
        b"U\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) != 0
    {
        if *incx == 1 as libc::c_int as libc::c_long {
            j = *n;
            while j >= 1 as libc::c_int as libc::c_long {
                temp = *x.offset(j as isize);
                if nounit != 0 {
                    temp *= *a.offset((j + j * a_dim1) as isize)
                }
                i__ = j - 1 as libc::c_int as libc::c_long;
                while i__ >= 1 as libc::c_int as libc::c_long {
                    temp += *a.offset((i__ + j * a_dim1) as isize) * *x.offset(i__ as isize);
                    i__ -= 1
                    /*        Form  x := A'*x. */
                    /* L100: */
                    /* L90: */
                }
                *x.offset(j as isize) = temp;
                j -= 1
            }
        } else {
            jx = kx + (*n - 1 as libc::c_int as libc::c_long) * *incx;
            j = *n;
            while j >= 1 as libc::c_int as libc::c_long {
                temp = *x.offset(jx as isize);
                ix = jx;
                if nounit != 0 {
                    temp *= *a.offset((j + j * a_dim1) as isize)
                }
                i__ = j - 1 as libc::c_int as libc::c_long;
                while i__ >= 1 as libc::c_int as libc::c_long {
                    ix -= *incx;
                    temp += *a.offset((i__ + j * a_dim1) as isize) * *x.offset(ix as isize);
                    i__ -= 1
                    /* L120: */
                    /* L110: */
                }
                *x.offset(jx as isize) = temp;
                jx -= *incx;
                j -= 1
            }
        }
    } else if *incx == 1 as libc::c_int as libc::c_long {
        i__1 = *n;
        j = 1 as libc::c_int as integer;
        while j <= i__1 {
            temp = *x.offset(j as isize);
            if nounit != 0 {
                temp *= *a.offset((j + j * a_dim1) as isize)
            }
            i__2 = *n;
            i__ = j + 1 as libc::c_int as libc::c_long;
            while i__ <= i__2 {
                temp += *a.offset((i__ + j * a_dim1) as isize) * *x.offset(i__ as isize);
                i__ += 1
                /* L140: */
                /* L130: */
            }
            *x.offset(j as isize) = temp;
            j += 1
        }
    } else {
        jx = kx;
        i__1 = *n;
        j = 1 as libc::c_int as integer;
        while j <= i__1 {
            temp = *x.offset(jx as isize);
            ix = jx;
            if nounit != 0 {
                temp *= *a.offset((j + j * a_dim1) as isize)
            }
            i__2 = *n;
            i__ = j + 1 as libc::c_int as libc::c_long;
            while i__ <= i__2 {
                ix += *incx;
                temp += *a.offset((i__ + j * a_dim1) as isize) * *x.offset(ix as isize);
                i__ += 1
                /* L160: */
                /* L150: */
            }
            *x.offset(jx as isize) = temp;
            jx += *incx;
            j += 1
        }
    }
    return 0 as libc::c_int;
    /*     End of STRMV . */
}
/* strmv_ */
