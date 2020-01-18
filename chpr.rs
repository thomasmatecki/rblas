use ::libc;
/* f2c.h  --  Standard Fortran to C header file */
/* *  barf  [ba:rf]  2.  "He suggested using FORTRAN, and everybody barfed."

	- From The Shogakukan DICTIONARY OF NEW ENGLISH (Second edition) */
pub type integer = libc::c_long;
pub type real = libc::c_float;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct complex {
    pub r: real,
    pub i: real,
}
pub type logical = libc::c_long;
/* chpr.f -- translated by f2c (version 20061008).
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
pub unsafe extern "C" fn f2c_chpr(mut uplo: *mut libc::c_char,
                                  mut n: *mut integer, mut alpha: *mut real,
                                  mut x: *mut complex, mut incx: *mut integer,
                                  mut ap: *mut complex) -> libc::c_int {
    /* System generated locals */
    let mut i__1: integer = 0;
    let mut i__2: integer = 0;
    let mut i__3: integer = 0;
    let mut i__4: integer = 0;
    let mut i__5: integer = 0;
    let mut r__1: real = 0.;
    let mut q__1: complex = complex{r: 0., i: 0.,};
    let mut q__2: complex = complex{r: 0., i: 0.,};
    /* Builtin functions */
    extern "C" {
        #[link_name = "r_cnjg"]
        fn r_cnjg_0(_: *mut complex, _: *mut complex);
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
    let mut temp: complex = complex{r: 0., i: 0.,};
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
    /*  CHPR    performs the hermitian rank 1 operation */
    /*     A := alpha*x*conjg( x' ) + A, */
    /*  where alpha is a real scalar, x is an n element vector and A is an */
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
    /*  ALPHA  - REAL            . */
/*           On entry, ALPHA specifies the scalar alpha. */
/*           Unchanged on exit. */
    /*  X      - COMPLEX          array of dimension at least */
/*           ( 1 + ( n - 1 )*abs( INCX ) ). */
/*           Before entry, the incremented array X must contain the n */
/*           element vector x. */
/*           Unchanged on exit. */
    /*  INCX   - INTEGER. */
/*           On entry, INCX specifies the increment for the elements of */
/*           X. INCX must not be zero. */
/*           Unchanged on exit. */
    /*  AP     - COMPLEX          array of DIMENSION at least */
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
    x = x.offset(-1);
    /* Function Body */
    info = 0 as libc::c_int as integer;
    if lsame__0(uplo,
                b"U\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char) == 0 &&
           lsame__0(uplo,
                    b"L\x00" as *const u8 as *const libc::c_char as
                        *mut libc::c_char) == 0 {
        info = 1 as libc::c_int as integer
    } else if *n < 0 as libc::c_int as libc::c_long {
        info = 2 as libc::c_int as integer
    } else if *incx == 0 as libc::c_int as libc::c_long {
        info = 5 as libc::c_int as integer
    }
    if info != 0 as libc::c_int as libc::c_long {
        xerbla__0(b"CHPR  \x00" as *const u8 as *const libc::c_char as
                      *mut libc::c_char, &mut info);
        return 0 as libc::c_int
    }
    /*     Quick return if possible. */
    if *n == 0 as libc::c_int as libc::c_long || *alpha == 0.0f32 {
        return 0 as libc::c_int
    }
    /*     Set the start point in X if the increment is not unity. */
    if *incx <= 0 as libc::c_int as libc::c_long {
        kx =
            1 as libc::c_int as libc::c_long -
                (*n - 1 as libc::c_int as libc::c_long) * *incx
    } else if *incx != 1 as libc::c_int as libc::c_long {
        kx = 1 as libc::c_int as integer
    }
    /*     Start the operations. In this version the elements of the array AP */
/*     are accessed sequentially with one pass through AP. */
    kk = 1 as libc::c_int as integer;
    if lsame__0(uplo,
                b"U\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char) != 0 {
        /*        Form  A  when upper triangle is stored in AP. */
        if *incx == 1 as libc::c_int as libc::c_long {
            i__1 = *n;
            j = 1 as libc::c_int as integer;
            while j <= i__1 {
                i__2 = j;
                if (*x.offset(i__2 as isize)).r != 0.0f32 ||
                       (*x.offset(i__2 as isize)).i != 0.0f32 {
                    r_cnjg_0(&mut q__2, &mut *x.offset(j as isize));
                    q__1.r = *alpha * q__2.r;
                    q__1.i = *alpha * q__2.i;
                    temp.r = q__1.r;
                    temp.i = q__1.i;
                    k = kk;
                    i__2 = j - 1 as libc::c_int as libc::c_long;
                    i__ = 1 as libc::c_int as integer;
                    while i__ <= i__2 {
                        i__3 = k;
                        i__4 = k;
                        i__5 = i__;
                        q__2.r =
                            (*x.offset(i__5 as isize)).r * temp.r -
                                (*x.offset(i__5 as isize)).i * temp.i;
                        q__2.i =
                            (*x.offset(i__5 as isize)).r * temp.i +
                                (*x.offset(i__5 as isize)).i * temp.r;
                        q__1.r = (*ap.offset(i__4 as isize)).r + q__2.r;
                        q__1.i = (*ap.offset(i__4 as isize)).i + q__2.i;
                        (*ap.offset(i__3 as isize)).r = q__1.r;
                        (*ap.offset(i__3 as isize)).i = q__1.i;
                        k += 1;
                        i__ += 1
                        /* L20: */
                        /* L10: */
                    }
                    i__2 = kk + j - 1 as libc::c_int as libc::c_long;
                    i__3 = kk + j - 1 as libc::c_int as libc::c_long;
                    i__4 = j;
                    q__1.r =
                        (*x.offset(i__4 as isize)).r * temp.r -
                            (*x.offset(i__4 as isize)).i * temp.i;
                    q__1.i =
                        (*x.offset(i__4 as isize)).r * temp.i +
                            (*x.offset(i__4 as isize)).i * temp.r;
                    r__1 = (*ap.offset(i__3 as isize)).r + q__1.r;
                    (*ap.offset(i__2 as isize)).r = r__1;
                    (*ap.offset(i__2 as isize)).i = 0.0f32
                } else {
                    i__2 = kk + j - 1 as libc::c_int as libc::c_long;
                    i__3 = kk + j - 1 as libc::c_int as libc::c_long;
                    r__1 = (*ap.offset(i__3 as isize)).r;
                    (*ap.offset(i__2 as isize)).r = r__1;
                    (*ap.offset(i__2 as isize)).i = 0.0f32
                }
                kk += j;
                j += 1
            }
        } else {
            jx = kx;
            i__1 = *n;
            j = 1 as libc::c_int as integer;
            while j <= i__1 {
                i__2 = jx;
                if (*x.offset(i__2 as isize)).r != 0.0f32 ||
                       (*x.offset(i__2 as isize)).i != 0.0f32 {
                    r_cnjg_0(&mut q__2, &mut *x.offset(jx as isize));
                    q__1.r = *alpha * q__2.r;
                    q__1.i = *alpha * q__2.i;
                    temp.r = q__1.r;
                    temp.i = q__1.i;
                    ix = kx;
                    i__2 = kk + j - 2 as libc::c_int as libc::c_long;
                    k = kk;
                    while k <= i__2 {
                        i__3 = k;
                        i__4 = k;
                        i__5 = ix;
                        q__2.r =
                            (*x.offset(i__5 as isize)).r * temp.r -
                                (*x.offset(i__5 as isize)).i * temp.i;
                        q__2.i =
                            (*x.offset(i__5 as isize)).r * temp.i +
                                (*x.offset(i__5 as isize)).i * temp.r;
                        q__1.r = (*ap.offset(i__4 as isize)).r + q__2.r;
                        q__1.i = (*ap.offset(i__4 as isize)).i + q__2.i;
                        (*ap.offset(i__3 as isize)).r = q__1.r;
                        (*ap.offset(i__3 as isize)).i = q__1.i;
                        ix += *incx;
                        k += 1
                        /* L40: */
                        /* L30: */
                    }
                    i__2 = kk + j - 1 as libc::c_int as libc::c_long;
                    i__3 = kk + j - 1 as libc::c_int as libc::c_long;
                    i__4 = jx;
                    q__1.r =
                        (*x.offset(i__4 as isize)).r * temp.r -
                            (*x.offset(i__4 as isize)).i * temp.i;
                    q__1.i =
                        (*x.offset(i__4 as isize)).r * temp.i +
                            (*x.offset(i__4 as isize)).i * temp.r;
                    r__1 = (*ap.offset(i__3 as isize)).r + q__1.r;
                    (*ap.offset(i__2 as isize)).r = r__1;
                    (*ap.offset(i__2 as isize)).i = 0.0f32
                } else {
                    i__2 = kk + j - 1 as libc::c_int as libc::c_long;
                    i__3 = kk + j - 1 as libc::c_int as libc::c_long;
                    r__1 = (*ap.offset(i__3 as isize)).r;
                    (*ap.offset(i__2 as isize)).r = r__1;
                    (*ap.offset(i__2 as isize)).i = 0.0f32
                }
                jx += *incx;
                kk += j;
                j += 1
            }
        }
    } else if *incx == 1 as libc::c_int as libc::c_long {
        i__1 = *n;
        j = 1 as libc::c_int as integer;
        while j <= i__1 {
            i__2 = j;
            if (*x.offset(i__2 as isize)).r != 0.0f32 ||
                   (*x.offset(i__2 as isize)).i != 0.0f32 {
                r_cnjg_0(&mut q__2, &mut *x.offset(j as isize));
                q__1.r = *alpha * q__2.r;
                q__1.i = *alpha * q__2.i;
                temp.r = q__1.r;
                temp.i = q__1.i;
                i__2 = kk;
                i__3 = kk;
                i__4 = j;
                q__1.r =
                    temp.r * (*x.offset(i__4 as isize)).r -
                        temp.i * (*x.offset(i__4 as isize)).i;
                q__1.i =
                    temp.r * (*x.offset(i__4 as isize)).i +
                        temp.i * (*x.offset(i__4 as isize)).r;
                r__1 = (*ap.offset(i__3 as isize)).r + q__1.r;
                (*ap.offset(i__2 as isize)).r = r__1;
                (*ap.offset(i__2 as isize)).i = 0.0f32;
                k = kk + 1 as libc::c_int as libc::c_long;
                i__2 = *n;
                i__ = j + 1 as libc::c_int as libc::c_long;
                while i__ <= i__2 {
                    i__3 = k;
                    i__4 = k;
                    i__5 = i__;
                    q__2.r =
                        (*x.offset(i__5 as isize)).r * temp.r -
                            (*x.offset(i__5 as isize)).i * temp.i;
                    q__2.i =
                        (*x.offset(i__5 as isize)).r * temp.i +
                            (*x.offset(i__5 as isize)).i * temp.r;
                    q__1.r = (*ap.offset(i__4 as isize)).r + q__2.r;
                    q__1.i = (*ap.offset(i__4 as isize)).i + q__2.i;
                    (*ap.offset(i__3 as isize)).r = q__1.r;
                    (*ap.offset(i__3 as isize)).i = q__1.i;
                    k += 1;
                    i__ += 1
                    /*        Form  A  when lower triangle is stored in AP. */
                    /* L60: */
                    /* L50: */
                }
            } else {
                i__2 = kk;
                i__3 = kk;
                r__1 = (*ap.offset(i__3 as isize)).r;
                (*ap.offset(i__2 as isize)).r = r__1;
                (*ap.offset(i__2 as isize)).i = 0.0f32
            }
            kk = kk + *n - j + 1 as libc::c_int as libc::c_long;
            j += 1
        }
    } else {
        jx = kx;
        i__1 = *n;
        j = 1 as libc::c_int as integer;
        while j <= i__1 {
            i__2 = jx;
            if (*x.offset(i__2 as isize)).r != 0.0f32 ||
                   (*x.offset(i__2 as isize)).i != 0.0f32 {
                r_cnjg_0(&mut q__2, &mut *x.offset(jx as isize));
                q__1.r = *alpha * q__2.r;
                q__1.i = *alpha * q__2.i;
                temp.r = q__1.r;
                temp.i = q__1.i;
                i__2 = kk;
                i__3 = kk;
                i__4 = jx;
                q__1.r =
                    temp.r * (*x.offset(i__4 as isize)).r -
                        temp.i * (*x.offset(i__4 as isize)).i;
                q__1.i =
                    temp.r * (*x.offset(i__4 as isize)).i +
                        temp.i * (*x.offset(i__4 as isize)).r;
                r__1 = (*ap.offset(i__3 as isize)).r + q__1.r;
                (*ap.offset(i__2 as isize)).r = r__1;
                (*ap.offset(i__2 as isize)).i = 0.0f32;
                ix = jx;
                i__2 = kk + *n - j;
                k = kk + 1 as libc::c_int as libc::c_long;
                while k <= i__2 {
                    ix += *incx;
                    i__3 = k;
                    i__4 = k;
                    i__5 = ix;
                    q__2.r =
                        (*x.offset(i__5 as isize)).r * temp.r -
                            (*x.offset(i__5 as isize)).i * temp.i;
                    q__2.i =
                        (*x.offset(i__5 as isize)).r * temp.i +
                            (*x.offset(i__5 as isize)).i * temp.r;
                    q__1.r = (*ap.offset(i__4 as isize)).r + q__2.r;
                    q__1.i = (*ap.offset(i__4 as isize)).i + q__2.i;
                    (*ap.offset(i__3 as isize)).r = q__1.r;
                    (*ap.offset(i__3 as isize)).i = q__1.i;
                    k += 1
                    /* L80: */
                    /* L70: */
                }
            } else {
                i__2 = kk;
                i__3 = kk;
                r__1 = (*ap.offset(i__3 as isize)).r;
                (*ap.offset(i__2 as isize)).r = r__1;
                (*ap.offset(i__2 as isize)).i = 0.0f32
            }
            jx += *incx;
            kk = kk + *n - j + 1 as libc::c_int as libc::c_long;
            j += 1
        }
    }
    return 0 as libc::c_int;
    /*     End of CHPR  . */
}
/* chpr_ */
