use ::libc;
/* f2c.h  --  Standard Fortran to C header file */
/* *  barf  [ba:rf]  2.  "He suggested using FORTRAN, and everybody barfed."

	- From The Shogakukan DICTIONARY OF NEW ENGLISH (Second edition) */
pub type integer = libc::c_long;
pub type real = libc::c_float;
pub type logical = libc::c_long;
/* ssbmv.f -- translated by f2c (version 20061008).
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
pub unsafe extern "C" fn f2c_ssbmv(mut uplo: *mut libc::c_char,
                                   mut n: *mut integer, mut k: *mut integer,
                                   mut alpha: *mut real, mut a: *mut real,
                                   mut lda: *mut integer, mut x: *mut real,
                                   mut incx: *mut integer,
                                   mut beta: *mut real, mut y: *mut real,
                                   mut incy: *mut integer) -> libc::c_int {
    /* System generated locals */
    let mut a_dim1: integer = 0;
    let mut a_offset: integer = 0;
    let mut i__1: integer = 0;
    let mut i__2: integer = 0;
    let mut i__3: integer = 0;
    let mut i__4: integer = 0;
    /* Local variables */
    let mut i__: integer = 0;
    let mut j: integer = 0;
    let mut l: integer = 0;
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
    let mut kplus1: integer = 0;
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
    /*  SSBMV  performs the matrix-vector  operation */
    /*     y := alpha*A*x + beta*y, */
    /*  where alpha and beta are scalars, x and y are n element vectors and */
/*  A is an n by n symmetric band matrix, with k super-diagonals. */
    /*  Arguments */
/*  ========== */
    /*  UPLO   - CHARACTER*1. */
/*           On entry, UPLO specifies whether the upper or lower */
/*           triangular part of the band matrix A is being supplied as */
/*           follows: */
    /*              UPLO = 'U' or 'u'   The upper triangular part of A is */
/*                                  being supplied. */
    /*              UPLO = 'L' or 'l'   The lower triangular part of A is */
/*                                  being supplied. */
    /*           Unchanged on exit. */
    /*  N      - INTEGER. */
/*           On entry, N specifies the order of the matrix A. */
/*           N must be at least zero. */
/*           Unchanged on exit. */
    /*  K      - INTEGER. */
/*           On entry, K specifies the number of super-diagonals of the */
/*           matrix A. K must satisfy  0 .le. K. */
/*           Unchanged on exit. */
    /*  ALPHA  - REAL            . */
/*           On entry, ALPHA specifies the scalar alpha. */
/*           Unchanged on exit. */
    /*  A      - REAL             array of DIMENSION ( LDA, n ). */
/*           Before entry with UPLO = 'U' or 'u', the leading ( k + 1 ) */
/*           by n part of the array A must contain the upper triangular */
/*           band part of the symmetric matrix, supplied column by */
/*           column, with the leading diagonal of the matrix in row */
/*           ( k + 1 ) of the array, the first super-diagonal starting at */
/*           position 2 in row k, and so on. The top left k by k triangle */
/*           of the array A is not referenced. */
/*           The following program segment will transfer the upper */
/*           triangular part of a symmetric band matrix from conventional */
/*           full matrix storage to band storage: */
    /*                 DO 20, J = 1, N */
/*                    M = K + 1 - J */
/*                    DO 10, I = MAX( 1, J - K ), J */
/*                       A( M + I, J ) = matrix( I, J ) */
/*              10    CONTINUE */
/*              20 CONTINUE */
    /*           Before entry with UPLO = 'L' or 'l', the leading ( k + 1 ) */
/*           by n part of the array A must contain the lower triangular */
/*           band part of the symmetric matrix, supplied column by */
/*           column, with the leading diagonal of the matrix in row 1 of */
/*           the array, the first sub-diagonal starting at position 1 in */
/*           row 2, and so on. The bottom right k by k triangle of the */
/*           array A is not referenced. */
/*           The following program segment will transfer the lower */
/*           triangular part of a symmetric band matrix from conventional */
/*           full matrix storage to band storage: */
    /*                 DO 20, J = 1, N */
/*                    M = 1 - J */
/*                    DO 10, I = J, MIN( N, J + K ) */
/*                       A( M + I, J ) = matrix( I, J ) */
/*              10    CONTINUE */
/*              20 CONTINUE */
    /*           Unchanged on exit. */
    /*  LDA    - INTEGER. */
/*           On entry, LDA specifies the first dimension of A as declared */
/*           in the calling (sub) program. LDA must be at least */
/*           ( k + 1 ). */
/*           Unchanged on exit. */
    /*  X      - REAL             array of DIMENSION at least */
/*           ( 1 + ( n - 1 )*abs( INCX ) ). */
/*           Before entry, the incremented array X must contain the */
/*           vector x. */
/*           Unchanged on exit. */
    /*  INCX   - INTEGER. */
/*           On entry, INCX specifies the increment for the elements of */
/*           X. INCX must not be zero. */
/*           Unchanged on exit. */
    /*  BETA   - REAL            . */
/*           On entry, BETA specifies the scalar beta. */
/*           Unchanged on exit. */
    /*  Y      - REAL             array of DIMENSION at least */
/*           ( 1 + ( n - 1 )*abs( INCY ) ). */
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
    if lsame__0(uplo,
                b"U\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char) == 0 &&
           lsame__0(uplo,
                    b"L\x00" as *const u8 as *const libc::c_char as
                        *mut libc::c_char) == 0 {
        info = 1 as libc::c_int as integer
    } else if *n < 0 as libc::c_int as libc::c_long {
        info = 2 as libc::c_int as integer
    } else if *k < 0 as libc::c_int as libc::c_long {
        info = 3 as libc::c_int as integer
    } else if *lda < *k + 1 as libc::c_int as libc::c_long {
        info = 6 as libc::c_int as integer
    } else if *incx == 0 as libc::c_int as libc::c_long {
        info = 8 as libc::c_int as integer
    } else if *incy == 0 as libc::c_int as libc::c_long {
        info = 11 as libc::c_int as integer
    }
    if info != 0 as libc::c_int as libc::c_long {
        xerbla__0(b"SSBMV \x00" as *const u8 as *const libc::c_char as
                      *mut libc::c_char, &mut info);
        return 0 as libc::c_int
    }
    /*     Quick return if possible. */
    if *n == 0 as libc::c_int as libc::c_long ||
           *alpha == 0.0f32 && *beta == 1.0f32 {
        return 0 as libc::c_int
    }
    /*     Set up the start points in  X  and  Y. */
    if *incx > 0 as libc::c_int as libc::c_long {
        kx = 1 as libc::c_int as integer
    } else {
        kx =
            1 as libc::c_int as libc::c_long -
                (*n - 1 as libc::c_int as libc::c_long) * *incx
    }
    if *incy > 0 as libc::c_int as libc::c_long {
        ky = 1 as libc::c_int as integer
    } else {
        ky =
            1 as libc::c_int as libc::c_long -
                (*n - 1 as libc::c_int as libc::c_long) * *incy
    }
    /*     Start the operations. In this version the elements of the array A */
/*     are accessed sequentially with one pass through A. */
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
    if *alpha == 0.0f32 { return 0 as libc::c_int }
    if lsame__0(uplo,
                b"U\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char) != 0 {
        /*        Form  y  when upper triangle of A is stored. */
        kplus1 = *k + 1 as libc::c_int as libc::c_long;
        if *incx == 1 as libc::c_int as libc::c_long &&
               *incy == 1 as libc::c_int as libc::c_long {
            i__1 = *n;
            j = 1 as libc::c_int as integer;
            while j <= i__1 {
                temp1 = *alpha * *x.offset(j as isize);
                temp2 = 0.0f32;
                l = kplus1 - j;
                /* L60: */
                i__2 = 1 as libc::c_int as integer;
                i__3 = j - *k;
                i__4 = j - 1 as libc::c_int as libc::c_long;
                i__ = if i__2 >= i__3 { i__2 } else { i__3 };
                while i__ <= i__4 {
                    let ref mut fresh0 = *y.offset(i__ as isize);
                    *fresh0 +=
                        temp1 * *a.offset((l + i__ + j * a_dim1) as isize);
                    temp2 +=
                        *a.offset((l + i__ + j * a_dim1) as isize) *
                            *x.offset(i__ as isize);
                    i__ += 1
                    /* Computing MAX */
                    /* L50: */
                }
                *y.offset(j as isize) =
                    *y.offset(j as isize) +
                        temp1 * *a.offset((kplus1 + j * a_dim1) as isize) +
                        *alpha * temp2;
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
                l = kplus1 - j;
                /* L80: */
                i__4 = 1 as libc::c_int as integer;
                i__2 = j - *k;
                i__3 = j - 1 as libc::c_int as libc::c_long;
                i__ = if i__4 >= i__2 { i__4 } else { i__2 };
                while i__ <= i__3 {
                    let ref mut fresh1 = *y.offset(iy as isize);
                    *fresh1 +=
                        temp1 * *a.offset((l + i__ + j * a_dim1) as isize);
                    temp2 +=
                        *a.offset((l + i__ + j * a_dim1) as isize) *
                            *x.offset(ix as isize);
                    ix += *incx;
                    iy += *incy;
                    i__ += 1
                    /* Computing MAX */
                    /* L70: */
                }
                *y.offset(jy as isize) =
                    *y.offset(jy as isize) +
                        temp1 * *a.offset((kplus1 + j * a_dim1) as isize) +
                        *alpha * temp2;
                jx += *incx;
                jy += *incy;
                if j > *k { kx += *incx; ky += *incy }
                j += 1
            }
        }
    } else if *incx == 1 as libc::c_int as libc::c_long &&
                  *incy == 1 as libc::c_int as libc::c_long {
        i__1 = *n;
        j = 1 as libc::c_int as integer;
        while j <= i__1 {
            temp1 = *alpha * *x.offset(j as isize);
            temp2 = 0.0f32;
            let ref mut fresh2 = *y.offset(j as isize);
            *fresh2 +=
                temp1 *
                    *a.offset((j * a_dim1 + 1 as libc::c_int as libc::c_long)
                                  as isize);
            l = 1 as libc::c_int as libc::c_long - j;
            /*        Form  y  when lower triangle of A is stored. */
            /* L100: */
            i__4 = *n;
            i__2 = j + *k;
            i__3 = if i__4 <= i__2 { i__4 } else { i__2 };
            i__ = j + 1 as libc::c_int as libc::c_long;
            while i__ <= i__3 {
                let ref mut fresh3 = *y.offset(i__ as isize);
                *fresh3 += temp1 * *a.offset((l + i__ + j * a_dim1) as isize);
                temp2 +=
                    *a.offset((l + i__ + j * a_dim1) as isize) *
                        *x.offset(i__ as isize);
                i__ += 1
                /* Computing MIN */
                /* L90: */
            }
            let ref mut fresh4 = *y.offset(j as isize);
            *fresh4 += *alpha * temp2;
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
            *fresh5 +=
                temp1 *
                    *a.offset((j * a_dim1 + 1 as libc::c_int as libc::c_long)
                                  as isize);
            l = 1 as libc::c_int as libc::c_long - j;
            ix = jx;
            iy = jy;
            /* L120: */
            i__4 = *n;
            i__2 = j + *k;
            i__3 = if i__4 <= i__2 { i__4 } else { i__2 };
            i__ = j + 1 as libc::c_int as libc::c_long;
            while i__ <= i__3 {
                ix += *incx;
                iy += *incy;
                let ref mut fresh6 = *y.offset(iy as isize);
                *fresh6 += temp1 * *a.offset((l + i__ + j * a_dim1) as isize);
                temp2 +=
                    *a.offset((l + i__ + j * a_dim1) as isize) *
                        *x.offset(ix as isize);
                i__ += 1
                /* Computing MIN */
                /* L110: */
            }
            let ref mut fresh7 = *y.offset(jy as isize);
            *fresh7 += *alpha * temp2;
            jx += *incx;
            jy += *incy;
            j += 1
        }
    }
    return 0 as libc::c_int;
    /*     End of SSBMV . */
}
/* ssbmv_ */
