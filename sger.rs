use ::libc;
/* f2c.h  --  Standard Fortran to C header file */
/* *  barf  [ba:rf]  2.  "He suggested using FORTRAN, and everybody barfed."

	- From The Shogakukan DICTIONARY OF NEW ENGLISH (Second edition) */
pub type integer = libc::c_long;
pub type real = libc::c_float;
/* sger.f -- translated by f2c (version 20061008).
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
pub unsafe extern "C" fn f2c_sger(mut m: *mut integer, mut n: *mut integer,
                                  mut alpha: *mut real, mut x: *mut real,
                                  mut incx: *mut integer, mut y: *mut real,
                                  mut incy: *mut integer, mut a: *mut real,
                                  mut lda: *mut integer) -> libc::c_int {
    /* System generated locals */
    let mut a_dim1: integer = 0;
    let mut a_offset: integer = 0;
    let mut i__1: integer = 0;
    let mut i__2: integer = 0;
    /* Local variables */
    let mut i__: integer = 0;
    let mut j: integer = 0;
    let mut ix: integer = 0;
    let mut jy: integer = 0;
    let mut kx: integer = 0;
    let mut info: integer = 0;
    let mut temp: real = 0.;
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
    /*  SGER   performs the rank 1 operation */
    /*     A := alpha*x*y' + A, */
    /*  where alpha is a scalar, x is an m element vector, y is an n element */
/*  vector and A is an m by n matrix. */
    /*  Arguments */
/*  ========== */
    /*  M      - INTEGER. */
/*           On entry, M specifies the number of rows of the matrix A. */
/*           M must be at least zero. */
/*           Unchanged on exit. */
    /*  N      - INTEGER. */
/*           On entry, N specifies the number of columns of the matrix A. */
/*           N must be at least zero. */
/*           Unchanged on exit. */
    /*  ALPHA  - REAL            . */
/*           On entry, ALPHA specifies the scalar alpha. */
/*           Unchanged on exit. */
    /*  X      - REAL             array of dimension at least */
/*           ( 1 + ( m - 1 )*abs( INCX ) ). */
/*           Before entry, the incremented array X must contain the m */
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
/*           Before entry, the leading m by n part of the array A must */
/*           contain the matrix of coefficients. On exit, A is */
/*           overwritten by the updated matrix. */
    /*  LDA    - INTEGER. */
/*           On entry, LDA specifies the first dimension of A as declared */
/*           in the calling (sub) program. LDA must be at least */
/*           max( 1, m ). */
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
    if *m < 0 as libc::c_int as libc::c_long {
        info = 1 as libc::c_int as integer
    } else if *n < 0 as libc::c_int as libc::c_long {
        info = 2 as libc::c_int as integer
    } else if *incx == 0 as libc::c_int as libc::c_long {
        info = 5 as libc::c_int as integer
    } else if *incy == 0 as libc::c_int as libc::c_long {
        info = 7 as libc::c_int as integer
    } else if *lda <
                  (if 1 as libc::c_int as libc::c_long >= *m {
                       1 as libc::c_int as libc::c_long
                   } else { *m }) {
        info = 9 as libc::c_int as integer
    }
    if info != 0 as libc::c_int as libc::c_long {
        xerbla__0(b"SGER  \x00" as *const u8 as *const libc::c_char as
                      *mut libc::c_char, &mut info);
        return 0 as libc::c_int
    }
    /*     Quick return if possible. */
    if *m == 0 as libc::c_int as libc::c_long ||
           *n == 0 as libc::c_int as libc::c_long || *alpha == 0.0f32 {
        return 0 as libc::c_int
    }
    /*     Start the operations. In this version the elements of A are */
/*     accessed sequentially with one pass through A. */
    if *incy > 0 as libc::c_int as libc::c_long {
        jy = 1 as libc::c_int as integer
    } else {
        jy =
            1 as libc::c_int as libc::c_long -
                (*n - 1 as libc::c_int as libc::c_long) * *incy
    }
    if *incx == 1 as libc::c_int as libc::c_long {
        i__1 = *n;
        j = 1 as libc::c_int as integer;
        while j <= i__1 {
            if *y.offset(jy as isize) != 0.0f32 {
                temp = *alpha * *y.offset(jy as isize);
                i__2 = *m;
                i__ = 1 as libc::c_int as integer;
                while i__ <= i__2 {
                    let ref mut fresh0 =
                        *a.offset((i__ + j * a_dim1) as isize);
                    *fresh0 += *x.offset(i__ as isize) * temp;
                    i__ += 1
                    /* L10: */
                }
            }
            jy += *incy;
            j += 1
            /* L20: */
        }
    } else {
        if *incx > 0 as libc::c_int as libc::c_long {
            kx = 1 as libc::c_int as integer
        } else {
            kx =
                1 as libc::c_int as libc::c_long -
                    (*m - 1 as libc::c_int as libc::c_long) * *incx
        }
        i__1 = *n;
        j = 1 as libc::c_int as integer;
        while j <= i__1 {
            if *y.offset(jy as isize) != 0.0f32 {
                temp = *alpha * *y.offset(jy as isize);
                ix = kx;
                i__2 = *m;
                i__ = 1 as libc::c_int as integer;
                while i__ <= i__2 {
                    let ref mut fresh1 =
                        *a.offset((i__ + j * a_dim1) as isize);
                    *fresh1 += *x.offset(ix as isize) * temp;
                    ix += *incx;
                    i__ += 1
                    /* L30: */
                }
            }
            jy += *incy;
            j += 1
            /* L40: */
        }
    }
    return 0 as libc::c_int;
    /*     End of SGER  . */
}
/* sger_ */
