use ::libc;
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
/* zhbmv.f -- translated by f2c (version 20061008).
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
pub unsafe extern "C" fn f2c_zhbmv(mut uplo: *mut libc::c_char,
                                   mut n: *mut integer, mut k: *mut integer,
                                   mut alpha: *mut doublecomplex,
                                   mut a: *mut doublecomplex,
                                   mut lda: *mut integer,
                                   mut x: *mut doublecomplex,
                                   mut incx: *mut integer,
                                   mut beta: *mut doublecomplex,
                                   mut y: *mut doublecomplex,
                                   mut incy: *mut integer) -> libc::c_int {
    /* System generated locals */
    let mut a_dim1: integer = 0;
    let mut a_offset: integer = 0;
    let mut i__1: integer = 0;
    let mut i__2: integer = 0;
    let mut i__3: integer = 0;
    let mut i__4: integer = 0;
    let mut i__5: integer = 0;
    let mut d__1: doublereal = 0.;
    let mut z__1: doublecomplex = doublecomplex{r: 0., i: 0.,};
    let mut z__2: doublecomplex = doublecomplex{r: 0., i: 0.,};
    let mut z__3: doublecomplex = doublecomplex{r: 0., i: 0.,};
    let mut z__4: doublecomplex = doublecomplex{r: 0., i: 0.,};
    /* Builtin functions */
    extern "C" {
        #[link_name = "d_cnjg"]
        fn d_cnjg_0(_: *mut doublecomplex, _: *mut doublecomplex);
    }
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
    let mut temp1: doublecomplex = doublecomplex{r: 0., i: 0.,};
    let mut temp2: doublecomplex = doublecomplex{r: 0., i: 0.,};
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
    /*  ZHBMV  performs the matrix-vector  operation */
    /*     y := alpha*A*x + beta*y, */
    /*  where alpha and beta are scalars, x and y are n element vectors and */
/*  A is an n by n hermitian band matrix, with k super-diagonals. */
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
    /*  ALPHA  - COMPLEX*16      . */
/*           On entry, ALPHA specifies the scalar alpha. */
/*           Unchanged on exit. */
    /*  A      - COMPLEX*16       array of DIMENSION ( LDA, n ). */
/*           Before entry with UPLO = 'U' or 'u', the leading ( k + 1 ) */
/*           by n part of the array A must contain the upper triangular */
/*           band part of the hermitian matrix, supplied column by */
/*           column, with the leading diagonal of the matrix in row */
/*           ( k + 1 ) of the array, the first super-diagonal starting at */
/*           position 2 in row k, and so on. The top left k by k triangle */
/*           of the array A is not referenced. */
/*           The following program segment will transfer the upper */
/*           triangular part of a hermitian band matrix from conventional */
/*           full matrix storage to band storage: */
    /*                 DO 20, J = 1, N */
/*                    M = K + 1 - J */
/*                    DO 10, I = MAX( 1, J - K ), J */
/*                       A( M + I, J ) = matrix( I, J ) */
/*              10    CONTINUE */
/*              20 CONTINUE */
    /*           Before entry with UPLO = 'L' or 'l', the leading ( k + 1 ) */
/*           by n part of the array A must contain the lower triangular */
/*           band part of the hermitian matrix, supplied column by */
/*           column, with the leading diagonal of the matrix in row 1 of */
/*           the array, the first sub-diagonal starting at position 1 in */
/*           row 2, and so on. The bottom right k by k triangle of the */
/*           array A is not referenced. */
/*           The following program segment will transfer the lower */
/*           triangular part of a hermitian band matrix from conventional */
/*           full matrix storage to band storage: */
    /*                 DO 20, J = 1, N */
/*                    M = 1 - J */
/*                    DO 10, I = J, MIN( N, J + K ) */
/*                       A( M + I, J ) = matrix( I, J ) */
/*              10    CONTINUE */
/*              20 CONTINUE */
    /*           Note that the imaginary parts of the diagonal elements need */
/*           not be set and are assumed to be zero. */
/*           Unchanged on exit. */
    /*  LDA    - INTEGER. */
/*           On entry, LDA specifies the first dimension of A as declared */
/*           in the calling (sub) program. LDA must be at least */
/*           ( k + 1 ). */
/*           Unchanged on exit. */
    /*  X      - COMPLEX*16       array of DIMENSION at least */
/*           ( 1 + ( n - 1 )*abs( INCX ) ). */
/*           Before entry, the incremented array X must contain the */
/*           vector x. */
/*           Unchanged on exit. */
    /*  INCX   - INTEGER. */
/*           On entry, INCX specifies the increment for the elements of */
/*           X. INCX must not be zero. */
/*           Unchanged on exit. */
    /*  BETA   - COMPLEX*16      . */
/*           On entry, BETA specifies the scalar beta. */
/*           Unchanged on exit. */
    /*  Y      - COMPLEX*16       array of DIMENSION at least */
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
        xerbla__0(b"ZHBMV \x00" as *const u8 as *const libc::c_char as
                      *mut libc::c_char, &mut info);
        return 0 as libc::c_int
    }
    /*     Quick return if possible. */
    if *n == 0 as libc::c_int as libc::c_long ||
           (*alpha).r == 0.0f64 && (*alpha).i == 0.0f64 &&
               ((*beta).r == 1.0f64 && (*beta).i == 0.0f64) {
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
    if (*beta).r != 1.0f64 || (*beta).i != 0.0f64 {
        if *incy == 1 as libc::c_int as libc::c_long {
            if (*beta).r == 0.0f64 && (*beta).i == 0.0f64 {
                i__1 = *n;
                i__ = 1 as libc::c_int as integer;
                while i__ <= i__1 {
                    i__2 = i__;
                    (*y.offset(i__2 as isize)).r = 0.0f64;
                    (*y.offset(i__2 as isize)).i = 0.0f64;
                    i__ += 1
                    /* L10: */
                }
            } else {
                i__1 = *n;
                i__ = 1 as libc::c_int as integer;
                while i__ <= i__1 {
                    i__2 = i__;
                    i__3 = i__;
                    z__1.r =
                        (*beta).r * (*y.offset(i__3 as isize)).r -
                            (*beta).i * (*y.offset(i__3 as isize)).i;
                    z__1.i =
                        (*beta).r * (*y.offset(i__3 as isize)).i +
                            (*beta).i * (*y.offset(i__3 as isize)).r;
                    (*y.offset(i__2 as isize)).r = z__1.r;
                    (*y.offset(i__2 as isize)).i = z__1.i;
                    i__ += 1
                    /* L20: */
                }
            }
        } else {
            iy = ky;
            if (*beta).r == 0.0f64 && (*beta).i == 0.0f64 {
                i__1 = *n;
                i__ = 1 as libc::c_int as integer;
                while i__ <= i__1 {
                    i__2 = iy;
                    (*y.offset(i__2 as isize)).r = 0.0f64;
                    (*y.offset(i__2 as isize)).i = 0.0f64;
                    iy += *incy;
                    i__ += 1
                    /* L30: */
                }
            } else {
                i__1 = *n;
                i__ = 1 as libc::c_int as integer;
                while i__ <= i__1 {
                    i__2 = iy;
                    i__3 = iy;
                    z__1.r =
                        (*beta).r * (*y.offset(i__3 as isize)).r -
                            (*beta).i * (*y.offset(i__3 as isize)).i;
                    z__1.i =
                        (*beta).r * (*y.offset(i__3 as isize)).i +
                            (*beta).i * (*y.offset(i__3 as isize)).r;
                    (*y.offset(i__2 as isize)).r = z__1.r;
                    (*y.offset(i__2 as isize)).i = z__1.i;
                    iy += *incy;
                    i__ += 1
                    /* L40: */
                }
            }
        }
    }
    if (*alpha).r == 0.0f64 && (*alpha).i == 0.0f64 {
        return 0 as libc::c_int
    }
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
                i__2 = j;
                z__1.r =
                    (*alpha).r * (*x.offset(i__2 as isize)).r -
                        (*alpha).i * (*x.offset(i__2 as isize)).i;
                z__1.i =
                    (*alpha).r * (*x.offset(i__2 as isize)).i +
                        (*alpha).i * (*x.offset(i__2 as isize)).r;
                temp1.r = z__1.r;
                temp1.i = z__1.i;
                temp2.r = 0.0f64;
                temp2.i = 0.0f64;
                l = kplus1 - j;
                /* L60: */
                i__2 = 1 as libc::c_int as integer;
                i__3 = j - *k;
                i__4 = j - 1 as libc::c_int as libc::c_long;
                i__ = if i__2 >= i__3 { i__2 } else { i__3 };
                while i__ <= i__4 {
                    i__2 = i__;
                    i__3 = i__;
                    i__5 = l + i__ + j * a_dim1;
                    z__2.r =
                        temp1.r * (*a.offset(i__5 as isize)).r -
                            temp1.i * (*a.offset(i__5 as isize)).i;
                    z__2.i =
                        temp1.r * (*a.offset(i__5 as isize)).i +
                            temp1.i * (*a.offset(i__5 as isize)).r;
                    z__1.r = (*y.offset(i__3 as isize)).r + z__2.r;
                    z__1.i = (*y.offset(i__3 as isize)).i + z__2.i;
                    (*y.offset(i__2 as isize)).r = z__1.r;
                    (*y.offset(i__2 as isize)).i = z__1.i;
                    d_cnjg_0(&mut z__3,
                             &mut *a.offset((l + i__ + j * a_dim1) as isize));
                    i__2 = i__;
                    z__2.r =
                        z__3.r * (*x.offset(i__2 as isize)).r -
                            z__3.i * (*x.offset(i__2 as isize)).i;
                    z__2.i =
                        z__3.r * (*x.offset(i__2 as isize)).i +
                            z__3.i * (*x.offset(i__2 as isize)).r;
                    z__1.r = temp2.r + z__2.r;
                    z__1.i = temp2.i + z__2.i;
                    temp2.r = z__1.r;
                    temp2.i = z__1.i;
                    i__ += 1
                    /* Computing MAX */
                    /* L50: */
                }
                i__4 = j;
                i__2 = j;
                i__3 = kplus1 + j * a_dim1;
                d__1 = (*a.offset(i__3 as isize)).r;
                z__3.r = d__1 * temp1.r;
                z__3.i = d__1 * temp1.i;
                z__2.r = (*y.offset(i__2 as isize)).r + z__3.r;
                z__2.i = (*y.offset(i__2 as isize)).i + z__3.i;
                z__4.r = (*alpha).r * temp2.r - (*alpha).i * temp2.i;
                z__4.i = (*alpha).r * temp2.i + (*alpha).i * temp2.r;
                z__1.r = z__2.r + z__4.r;
                z__1.i = z__2.i + z__4.i;
                (*y.offset(i__4 as isize)).r = z__1.r;
                (*y.offset(i__4 as isize)).i = z__1.i;
                j += 1
            }
        } else {
            jx = kx;
            jy = ky;
            i__1 = *n;
            j = 1 as libc::c_int as integer;
            while j <= i__1 {
                i__4 = jx;
                z__1.r =
                    (*alpha).r * (*x.offset(i__4 as isize)).r -
                        (*alpha).i * (*x.offset(i__4 as isize)).i;
                z__1.i =
                    (*alpha).r * (*x.offset(i__4 as isize)).i +
                        (*alpha).i * (*x.offset(i__4 as isize)).r;
                temp1.r = z__1.r;
                temp1.i = z__1.i;
                temp2.r = 0.0f64;
                temp2.i = 0.0f64;
                ix = kx;
                iy = ky;
                l = kplus1 - j;
                /* L80: */
                i__4 = 1 as libc::c_int as integer;
                i__2 = j - *k;
                i__3 = j - 1 as libc::c_int as libc::c_long;
                i__ = if i__4 >= i__2 { i__4 } else { i__2 };
                while i__ <= i__3 {
                    i__4 = iy;
                    i__2 = iy;
                    i__5 = l + i__ + j * a_dim1;
                    z__2.r =
                        temp1.r * (*a.offset(i__5 as isize)).r -
                            temp1.i * (*a.offset(i__5 as isize)).i;
                    z__2.i =
                        temp1.r * (*a.offset(i__5 as isize)).i +
                            temp1.i * (*a.offset(i__5 as isize)).r;
                    z__1.r = (*y.offset(i__2 as isize)).r + z__2.r;
                    z__1.i = (*y.offset(i__2 as isize)).i + z__2.i;
                    (*y.offset(i__4 as isize)).r = z__1.r;
                    (*y.offset(i__4 as isize)).i = z__1.i;
                    d_cnjg_0(&mut z__3,
                             &mut *a.offset((l + i__ + j * a_dim1) as isize));
                    i__4 = ix;
                    z__2.r =
                        z__3.r * (*x.offset(i__4 as isize)).r -
                            z__3.i * (*x.offset(i__4 as isize)).i;
                    z__2.i =
                        z__3.r * (*x.offset(i__4 as isize)).i +
                            z__3.i * (*x.offset(i__4 as isize)).r;
                    z__1.r = temp2.r + z__2.r;
                    z__1.i = temp2.i + z__2.i;
                    temp2.r = z__1.r;
                    temp2.i = z__1.i;
                    ix += *incx;
                    iy += *incy;
                    i__ += 1
                    /* Computing MAX */
                    /* L70: */
                }
                i__3 = jy;
                i__4 = jy;
                i__2 = kplus1 + j * a_dim1;
                d__1 = (*a.offset(i__2 as isize)).r;
                z__3.r = d__1 * temp1.r;
                z__3.i = d__1 * temp1.i;
                z__2.r = (*y.offset(i__4 as isize)).r + z__3.r;
                z__2.i = (*y.offset(i__4 as isize)).i + z__3.i;
                z__4.r = (*alpha).r * temp2.r - (*alpha).i * temp2.i;
                z__4.i = (*alpha).r * temp2.i + (*alpha).i * temp2.r;
                z__1.r = z__2.r + z__4.r;
                z__1.i = z__2.i + z__4.i;
                (*y.offset(i__3 as isize)).r = z__1.r;
                (*y.offset(i__3 as isize)).i = z__1.i;
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
            i__3 = j;
            z__1.r =
                (*alpha).r * (*x.offset(i__3 as isize)).r -
                    (*alpha).i * (*x.offset(i__3 as isize)).i;
            z__1.i =
                (*alpha).r * (*x.offset(i__3 as isize)).i +
                    (*alpha).i * (*x.offset(i__3 as isize)).r;
            temp1.r = z__1.r;
            temp1.i = z__1.i;
            temp2.r = 0.0f64;
            temp2.i = 0.0f64;
            i__3 = j;
            i__4 = j;
            i__2 = j * a_dim1 + 1 as libc::c_int as libc::c_long;
            d__1 = (*a.offset(i__2 as isize)).r;
            z__2.r = d__1 * temp1.r;
            z__2.i = d__1 * temp1.i;
            z__1.r = (*y.offset(i__4 as isize)).r + z__2.r;
            z__1.i = (*y.offset(i__4 as isize)).i + z__2.i;
            (*y.offset(i__3 as isize)).r = z__1.r;
            (*y.offset(i__3 as isize)).i = z__1.i;
            l = 1 as libc::c_int as libc::c_long - j;
            /*        Form  y  when lower triangle of A is stored. */
            /* L100: */
            i__4 = *n;
            i__2 = j + *k;
            i__3 = if i__4 <= i__2 { i__4 } else { i__2 };
            i__ = j + 1 as libc::c_int as libc::c_long;
            while i__ <= i__3 {
                i__4 = i__;
                i__2 = i__;
                i__5 = l + i__ + j * a_dim1;
                z__2.r =
                    temp1.r * (*a.offset(i__5 as isize)).r -
                        temp1.i * (*a.offset(i__5 as isize)).i;
                z__2.i =
                    temp1.r * (*a.offset(i__5 as isize)).i +
                        temp1.i * (*a.offset(i__5 as isize)).r;
                z__1.r = (*y.offset(i__2 as isize)).r + z__2.r;
                z__1.i = (*y.offset(i__2 as isize)).i + z__2.i;
                (*y.offset(i__4 as isize)).r = z__1.r;
                (*y.offset(i__4 as isize)).i = z__1.i;
                d_cnjg_0(&mut z__3,
                         &mut *a.offset((l + i__ + j * a_dim1) as isize));
                i__4 = i__;
                z__2.r =
                    z__3.r * (*x.offset(i__4 as isize)).r -
                        z__3.i * (*x.offset(i__4 as isize)).i;
                z__2.i =
                    z__3.r * (*x.offset(i__4 as isize)).i +
                        z__3.i * (*x.offset(i__4 as isize)).r;
                z__1.r = temp2.r + z__2.r;
                z__1.i = temp2.i + z__2.i;
                temp2.r = z__1.r;
                temp2.i = z__1.i;
                i__ += 1
                /* Computing MIN */
                /* L90: */
            }
            i__3 = j;
            i__4 = j;
            z__2.r = (*alpha).r * temp2.r - (*alpha).i * temp2.i;
            z__2.i = (*alpha).r * temp2.i + (*alpha).i * temp2.r;
            z__1.r = (*y.offset(i__4 as isize)).r + z__2.r;
            z__1.i = (*y.offset(i__4 as isize)).i + z__2.i;
            (*y.offset(i__3 as isize)).r = z__1.r;
            (*y.offset(i__3 as isize)).i = z__1.i;
            j += 1
        }
    } else {
        jx = kx;
        jy = ky;
        i__1 = *n;
        j = 1 as libc::c_int as integer;
        while j <= i__1 {
            i__3 = jx;
            z__1.r =
                (*alpha).r * (*x.offset(i__3 as isize)).r -
                    (*alpha).i * (*x.offset(i__3 as isize)).i;
            z__1.i =
                (*alpha).r * (*x.offset(i__3 as isize)).i +
                    (*alpha).i * (*x.offset(i__3 as isize)).r;
            temp1.r = z__1.r;
            temp1.i = z__1.i;
            temp2.r = 0.0f64;
            temp2.i = 0.0f64;
            i__3 = jy;
            i__4 = jy;
            i__2 = j * a_dim1 + 1 as libc::c_int as libc::c_long;
            d__1 = (*a.offset(i__2 as isize)).r;
            z__2.r = d__1 * temp1.r;
            z__2.i = d__1 * temp1.i;
            z__1.r = (*y.offset(i__4 as isize)).r + z__2.r;
            z__1.i = (*y.offset(i__4 as isize)).i + z__2.i;
            (*y.offset(i__3 as isize)).r = z__1.r;
            (*y.offset(i__3 as isize)).i = z__1.i;
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
                i__4 = iy;
                i__2 = iy;
                i__5 = l + i__ + j * a_dim1;
                z__2.r =
                    temp1.r * (*a.offset(i__5 as isize)).r -
                        temp1.i * (*a.offset(i__5 as isize)).i;
                z__2.i =
                    temp1.r * (*a.offset(i__5 as isize)).i +
                        temp1.i * (*a.offset(i__5 as isize)).r;
                z__1.r = (*y.offset(i__2 as isize)).r + z__2.r;
                z__1.i = (*y.offset(i__2 as isize)).i + z__2.i;
                (*y.offset(i__4 as isize)).r = z__1.r;
                (*y.offset(i__4 as isize)).i = z__1.i;
                d_cnjg_0(&mut z__3,
                         &mut *a.offset((l + i__ + j * a_dim1) as isize));
                i__4 = ix;
                z__2.r =
                    z__3.r * (*x.offset(i__4 as isize)).r -
                        z__3.i * (*x.offset(i__4 as isize)).i;
                z__2.i =
                    z__3.r * (*x.offset(i__4 as isize)).i +
                        z__3.i * (*x.offset(i__4 as isize)).r;
                z__1.r = temp2.r + z__2.r;
                z__1.i = temp2.i + z__2.i;
                temp2.r = z__1.r;
                temp2.i = z__1.i;
                i__ += 1
                /* Computing MIN */
                /* L110: */
            }
            i__3 = jy;
            i__4 = jy;
            z__2.r = (*alpha).r * temp2.r - (*alpha).i * temp2.i;
            z__2.i = (*alpha).r * temp2.i + (*alpha).i * temp2.r;
            z__1.r = (*y.offset(i__4 as isize)).r + z__2.r;
            z__1.i = (*y.offset(i__4 as isize)).i + z__2.i;
            (*y.offset(i__3 as isize)).r = z__1.r;
            (*y.offset(i__3 as isize)).i = z__1.i;
            jx += *incx;
            jy += *incy;
            j += 1
        }
    }
    return 0 as libc::c_int;
    /*     End of ZHBMV . */
}
/* zhbmv_ */
