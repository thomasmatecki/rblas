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
/* zgemv.f -- translated by f2c (version 20061008).
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
pub unsafe extern "C" fn f2c_zgemv(mut trans: *mut libc::c_char,
                                   mut m: *mut integer, mut n: *mut integer,
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
    let mut z__1: doublecomplex = doublecomplex{r: 0., i: 0.,};
    let mut z__2: doublecomplex = doublecomplex{r: 0., i: 0.,};
    let mut z__3: doublecomplex = doublecomplex{r: 0., i: 0.,};
    /* Builtin functions */
    extern "C" {
        #[link_name = "d_cnjg"]
        fn d_cnjg_0(_: *mut doublecomplex, _: *mut doublecomplex);
    }
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
    let mut temp: doublecomplex = doublecomplex{r: 0., i: 0.,};
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
    let mut noconj: logical = 0;
    /*     .. Scalar Arguments .. */
/*     .. */
/*     .. Array Arguments .. */
/*     .. */
    /*  Purpose */
/*  ======= */
    /*  ZGEMV  performs one of the matrix-vector operations */
    /*     y := alpha*A*x + beta*y,   or   y := alpha*A'*x + beta*y,   or */
    /*     y := alpha*conjg( A' )*x + beta*y, */
    /*  where alpha and beta are scalars, x and y are vectors and A is an */
/*  m by n matrix. */
    /*  Arguments */
/*  ========== */
    /*  TRANS  - CHARACTER*1. */
/*           On entry, TRANS specifies the operation to be performed as */
/*           follows: */
    /*              TRANS = 'N' or 'n'   y := alpha*A*x + beta*y. */
    /*              TRANS = 'T' or 't'   y := alpha*A'*x + beta*y. */
    /*              TRANS = 'C' or 'c'   y := alpha*conjg( A' )*x + beta*y. */
    /*           Unchanged on exit. */
    /*  M      - INTEGER. */
/*           On entry, M specifies the number of rows of the matrix A. */
/*           M must be at least zero. */
/*           Unchanged on exit. */
    /*  N      - INTEGER. */
/*           On entry, N specifies the number of columns of the matrix A. */
/*           N must be at least zero. */
/*           Unchanged on exit. */
    /*  ALPHA  - COMPLEX*16      . */
/*           On entry, ALPHA specifies the scalar alpha. */
/*           Unchanged on exit. */
    /*  A      - COMPLEX*16       array of DIMENSION ( LDA, n ). */
/*           Before entry, the leading m by n part of the array A must */
/*           contain the matrix of coefficients. */
/*           Unchanged on exit. */
    /*  LDA    - INTEGER. */
/*           On entry, LDA specifies the first dimension of A as declared */
/*           in the calling (sub) program. LDA must be at least */
/*           max( 1, m ). */
/*           Unchanged on exit. */
    /*  X      - COMPLEX*16       array of DIMENSION at least */
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
    /*  BETA   - COMPLEX*16      . */
/*           On entry, BETA specifies the scalar beta. When BETA is */
/*           supplied as zero then Y need not be set on input. */
/*           Unchanged on exit. */
    /*  Y      - COMPLEX*16       array of DIMENSION at least */
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
    } else if *lda <
                  (if 1 as libc::c_int as libc::c_long >= *m {
                       1 as libc::c_int as libc::c_long
                   } else { *m }) {
        info = 6 as libc::c_int as integer
    } else if *incx == 0 as libc::c_int as libc::c_long {
        info = 8 as libc::c_int as integer
    } else if *incy == 0 as libc::c_int as libc::c_long {
        info = 11 as libc::c_int as integer
    }
    if info != 0 as libc::c_int as libc::c_long {
        xerbla__0(b"ZGEMV \x00" as *const u8 as *const libc::c_char as
                      *mut libc::c_char, &mut info);
        return 0 as libc::c_int
    }
    /*     Quick return if possible. */
    if *m == 0 as libc::c_int as libc::c_long ||
           *n == 0 as libc::c_int as libc::c_long ||
           (*alpha).r == 0.0f64 && (*alpha).i == 0.0f64 &&
               ((*beta).r == 1.0f64 && (*beta).i == 0.0f64) {
        return 0 as libc::c_int
    }
    noconj =
        lsame__0(trans,
                 b"T\x00" as *const u8 as *const libc::c_char as
                     *mut libc::c_char);
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
/*     accessed sequentially with one pass through A. */
    /*     First form  y := beta*y. */
    if (*beta).r != 1.0f64 || (*beta).i != 0.0f64 {
        if *incy == 1 as libc::c_int as libc::c_long {
            if (*beta).r == 0.0f64 && (*beta).i == 0.0f64 {
                i__1 = leny;
                i__ = 1 as libc::c_int as integer;
                while i__ <= i__1 {
                    i__2 = i__;
                    (*y.offset(i__2 as isize)).r = 0.0f64;
                    (*y.offset(i__2 as isize)).i = 0.0f64;
                    i__ += 1
                    /* L10: */
                }
            } else {
                i__1 = leny;
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
                i__1 = leny;
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
                i__1 = leny;
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
    if lsame__0(trans,
                b"N\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char) != 0 {
        /*        Form  y := alpha*A*x + y. */
        jx = kx;
        if *incy == 1 as libc::c_int as libc::c_long {
            i__1 = *n;
            j = 1 as libc::c_int as integer;
            while j <= i__1 {
                i__2 = jx;
                if (*x.offset(i__2 as isize)).r != 0.0f64 ||
                       (*x.offset(i__2 as isize)).i != 0.0f64 {
                    i__2 = jx;
                    z__1.r =
                        (*alpha).r * (*x.offset(i__2 as isize)).r -
                            (*alpha).i * (*x.offset(i__2 as isize)).i;
                    z__1.i =
                        (*alpha).r * (*x.offset(i__2 as isize)).i +
                            (*alpha).i * (*x.offset(i__2 as isize)).r;
                    temp.r = z__1.r;
                    temp.i = z__1.i;
                    i__2 = *m;
                    i__ = 1 as libc::c_int as integer;
                    while i__ <= i__2 {
                        i__3 = i__;
                        i__4 = i__;
                        i__5 = i__ + j * a_dim1;
                        z__2.r =
                            temp.r * (*a.offset(i__5 as isize)).r -
                                temp.i * (*a.offset(i__5 as isize)).i;
                        z__2.i =
                            temp.r * (*a.offset(i__5 as isize)).i +
                                temp.i * (*a.offset(i__5 as isize)).r;
                        z__1.r = (*y.offset(i__4 as isize)).r + z__2.r;
                        z__1.i = (*y.offset(i__4 as isize)).i + z__2.i;
                        (*y.offset(i__3 as isize)).r = z__1.r;
                        (*y.offset(i__3 as isize)).i = z__1.i;
                        i__ += 1
                        /* L60: */
                        /* L50: */
                    }
                }
                jx += *incx;
                j += 1
            }
        } else {
            i__1 = *n;
            j = 1 as libc::c_int as integer;
            while j <= i__1 {
                i__2 = jx;
                if (*x.offset(i__2 as isize)).r != 0.0f64 ||
                       (*x.offset(i__2 as isize)).i != 0.0f64 {
                    i__2 = jx;
                    z__1.r =
                        (*alpha).r * (*x.offset(i__2 as isize)).r -
                            (*alpha).i * (*x.offset(i__2 as isize)).i;
                    z__1.i =
                        (*alpha).r * (*x.offset(i__2 as isize)).i +
                            (*alpha).i * (*x.offset(i__2 as isize)).r;
                    temp.r = z__1.r;
                    temp.i = z__1.i;
                    iy = ky;
                    i__2 = *m;
                    i__ = 1 as libc::c_int as integer;
                    while i__ <= i__2 {
                        i__3 = iy;
                        i__4 = iy;
                        i__5 = i__ + j * a_dim1;
                        z__2.r =
                            temp.r * (*a.offset(i__5 as isize)).r -
                                temp.i * (*a.offset(i__5 as isize)).i;
                        z__2.i =
                            temp.r * (*a.offset(i__5 as isize)).i +
                                temp.i * (*a.offset(i__5 as isize)).r;
                        z__1.r = (*y.offset(i__4 as isize)).r + z__2.r;
                        z__1.i = (*y.offset(i__4 as isize)).i + z__2.i;
                        (*y.offset(i__3 as isize)).r = z__1.r;
                        (*y.offset(i__3 as isize)).i = z__1.i;
                        iy += *incy;
                        i__ += 1
                        /* L80: */
                        /* L70: */
                    }
                }
                jx += *incx;
                j += 1
            }
        }
    } else {
        /*        Form  y := alpha*A'*x + y  or  y := alpha*conjg( A' )*x + y. */
        jy = ky;
        if *incx == 1 as libc::c_int as libc::c_long {
            i__1 = *n;
            j = 1 as libc::c_int as integer;
            while j <= i__1 {
                temp.r = 0.0f64;
                temp.i = 0.0f64;
                if noconj != 0 {
                    i__2 = *m;
                    i__ = 1 as libc::c_int as integer;
                    while i__ <= i__2 {
                        i__3 = i__ + j * a_dim1;
                        i__4 = i__;
                        z__2.r =
                            (*a.offset(i__3 as isize)).r *
                                (*x.offset(i__4 as isize)).r -
                                (*a.offset(i__3 as isize)).i *
                                    (*x.offset(i__4 as isize)).i;
                        z__2.i =
                            (*a.offset(i__3 as isize)).r *
                                (*x.offset(i__4 as isize)).i +
                                (*a.offset(i__3 as isize)).i *
                                    (*x.offset(i__4 as isize)).r;
                        z__1.r = temp.r + z__2.r;
                        z__1.i = temp.i + z__2.i;
                        temp.r = z__1.r;
                        temp.i = z__1.i;
                        i__ += 1
                        /* L110: */
                        /* L90: */
                    }
                } else {
                    i__2 = *m;
                    i__ = 1 as libc::c_int as integer;
                    while i__ <= i__2 {
                        d_cnjg_0(&mut z__3,
                                 &mut *a.offset((i__ + j * a_dim1) as isize));
                        i__3 = i__;
                        z__2.r =
                            z__3.r * (*x.offset(i__3 as isize)).r -
                                z__3.i * (*x.offset(i__3 as isize)).i;
                        z__2.i =
                            z__3.r * (*x.offset(i__3 as isize)).i +
                                z__3.i * (*x.offset(i__3 as isize)).r;
                        z__1.r = temp.r + z__2.r;
                        z__1.i = temp.i + z__2.i;
                        temp.r = z__1.r;
                        temp.i = z__1.i;
                        i__ += 1
                        /* L100: */
                    }
                }
                i__2 = jy;
                i__3 = jy;
                z__2.r = (*alpha).r * temp.r - (*alpha).i * temp.i;
                z__2.i = (*alpha).r * temp.i + (*alpha).i * temp.r;
                z__1.r = (*y.offset(i__3 as isize)).r + z__2.r;
                z__1.i = (*y.offset(i__3 as isize)).i + z__2.i;
                (*y.offset(i__2 as isize)).r = z__1.r;
                (*y.offset(i__2 as isize)).i = z__1.i;
                jy += *incy;
                j += 1
            }
        } else {
            i__1 = *n;
            j = 1 as libc::c_int as integer;
            while j <= i__1 {
                temp.r = 0.0f64;
                temp.i = 0.0f64;
                ix = kx;
                if noconj != 0 {
                    i__2 = *m;
                    i__ = 1 as libc::c_int as integer;
                    while i__ <= i__2 {
                        i__3 = i__ + j * a_dim1;
                        i__4 = ix;
                        z__2.r =
                            (*a.offset(i__3 as isize)).r *
                                (*x.offset(i__4 as isize)).r -
                                (*a.offset(i__3 as isize)).i *
                                    (*x.offset(i__4 as isize)).i;
                        z__2.i =
                            (*a.offset(i__3 as isize)).r *
                                (*x.offset(i__4 as isize)).i +
                                (*a.offset(i__3 as isize)).i *
                                    (*x.offset(i__4 as isize)).r;
                        z__1.r = temp.r + z__2.r;
                        z__1.i = temp.i + z__2.i;
                        temp.r = z__1.r;
                        temp.i = z__1.i;
                        ix += *incx;
                        i__ += 1
                        /* L140: */
                        /* L120: */
                    }
                } else {
                    i__2 = *m;
                    i__ = 1 as libc::c_int as integer;
                    while i__ <= i__2 {
                        d_cnjg_0(&mut z__3,
                                 &mut *a.offset((i__ + j * a_dim1) as isize));
                        i__3 = ix;
                        z__2.r =
                            z__3.r * (*x.offset(i__3 as isize)).r -
                                z__3.i * (*x.offset(i__3 as isize)).i;
                        z__2.i =
                            z__3.r * (*x.offset(i__3 as isize)).i +
                                z__3.i * (*x.offset(i__3 as isize)).r;
                        z__1.r = temp.r + z__2.r;
                        z__1.i = temp.i + z__2.i;
                        temp.r = z__1.r;
                        temp.i = z__1.i;
                        ix += *incx;
                        i__ += 1
                        /* L130: */
                    }
                }
                i__2 = jy;
                i__3 = jy;
                z__2.r = (*alpha).r * temp.r - (*alpha).i * temp.i;
                z__2.i = (*alpha).r * temp.i + (*alpha).i * temp.r;
                z__1.r = (*y.offset(i__3 as isize)).r + z__2.r;
                z__1.i = (*y.offset(i__3 as isize)).i + z__2.i;
                (*y.offset(i__2 as isize)).r = z__1.r;
                (*y.offset(i__2 as isize)).i = z__1.i;
                jy += *incy;
                j += 1
            }
        }
    }
    return 0 as libc::c_int;
    /*     End of ZGEMV . */
}
/* zgemv_ */
