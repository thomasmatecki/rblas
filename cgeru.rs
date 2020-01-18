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
/* cgeru.f -- translated by f2c (version 20061008).
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
pub unsafe extern "C" fn f2c_cgeru(mut m: *mut integer, mut n: *mut integer,
                                   mut alpha: *mut complex,
                                   mut x: *mut complex,
                                   mut incx: *mut integer,
                                   mut y: *mut complex,
                                   mut incy: *mut integer,
                                   mut a: *mut complex, mut lda: *mut integer)
 -> libc::c_int {
    /* System generated locals */
    let mut a_dim1: integer = 0;
    let mut a_offset: integer = 0;
    let mut i__1: integer = 0;
    let mut i__2: integer = 0;
    let mut i__3: integer = 0;
    let mut i__4: integer = 0;
    let mut i__5: integer = 0;
    let mut q__1: complex = complex{r: 0., i: 0.,};
    let mut q__2: complex = complex{r: 0., i: 0.,};
    /* Local variables */
    let mut i__: integer = 0;
    let mut j: integer = 0;
    let mut ix: integer = 0;
    let mut jy: integer = 0;
    let mut kx: integer = 0;
    let mut info: integer = 0;
    let mut temp: complex = complex{r: 0., i: 0.,};
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
    /*  CGERU  performs the rank 1 operation */
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
    /*  ALPHA  - COMPLEX         . */
/*           On entry, ALPHA specifies the scalar alpha. */
/*           Unchanged on exit. */
    /*  X      - COMPLEX          array of dimension at least */
/*           ( 1 + ( m - 1 )*abs( INCX ) ). */
/*           Before entry, the incremented array X must contain the m */
/*           element vector x. */
/*           Unchanged on exit. */
    /*  INCX   - INTEGER. */
/*           On entry, INCX specifies the increment for the elements of */
/*           X. INCX must not be zero. */
/*           Unchanged on exit. */
    /*  Y      - COMPLEX          array of dimension at least */
/*           ( 1 + ( n - 1 )*abs( INCY ) ). */
/*           Before entry, the incremented array Y must contain the n */
/*           element vector y. */
/*           Unchanged on exit. */
    /*  INCY   - INTEGER. */
/*           On entry, INCY specifies the increment for the elements of */
/*           Y. INCY must not be zero. */
/*           Unchanged on exit. */
    /*  A      - COMPLEX          array of DIMENSION ( LDA, n ). */
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
        xerbla__0(b"CGERU \x00" as *const u8 as *const libc::c_char as
                      *mut libc::c_char, &mut info);
        return 0 as libc::c_int
    }
    /*     Quick return if possible. */
    if *m == 0 as libc::c_int as libc::c_long ||
           *n == 0 as libc::c_int as libc::c_long ||
           (*alpha).r == 0.0f32 && (*alpha).i == 0.0f32 {
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
            i__2 = jy;
            if (*y.offset(i__2 as isize)).r != 0.0f32 ||
                   (*y.offset(i__2 as isize)).i != 0.0f32 {
                i__2 = jy;
                q__1.r =
                    (*alpha).r * (*y.offset(i__2 as isize)).r -
                        (*alpha).i * (*y.offset(i__2 as isize)).i;
                q__1.i =
                    (*alpha).r * (*y.offset(i__2 as isize)).i +
                        (*alpha).i * (*y.offset(i__2 as isize)).r;
                temp.r = q__1.r;
                temp.i = q__1.i;
                i__2 = *m;
                i__ = 1 as libc::c_int as integer;
                while i__ <= i__2 {
                    i__3 = i__ + j * a_dim1;
                    i__4 = i__ + j * a_dim1;
                    i__5 = i__;
                    q__2.r =
                        (*x.offset(i__5 as isize)).r * temp.r -
                            (*x.offset(i__5 as isize)).i * temp.i;
                    q__2.i =
                        (*x.offset(i__5 as isize)).r * temp.i +
                            (*x.offset(i__5 as isize)).i * temp.r;
                    q__1.r = (*a.offset(i__4 as isize)).r + q__2.r;
                    q__1.i = (*a.offset(i__4 as isize)).i + q__2.i;
                    (*a.offset(i__3 as isize)).r = q__1.r;
                    (*a.offset(i__3 as isize)).i = q__1.i;
                    i__ += 1
                    /* L20: */
                    /* L10: */
                }
            }
            jy += *incy;
            j += 1
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
            i__2 = jy;
            if (*y.offset(i__2 as isize)).r != 0.0f32 ||
                   (*y.offset(i__2 as isize)).i != 0.0f32 {
                i__2 = jy;
                q__1.r =
                    (*alpha).r * (*y.offset(i__2 as isize)).r -
                        (*alpha).i * (*y.offset(i__2 as isize)).i;
                q__1.i =
                    (*alpha).r * (*y.offset(i__2 as isize)).i +
                        (*alpha).i * (*y.offset(i__2 as isize)).r;
                temp.r = q__1.r;
                temp.i = q__1.i;
                ix = kx;
                i__2 = *m;
                i__ = 1 as libc::c_int as integer;
                while i__ <= i__2 {
                    i__3 = i__ + j * a_dim1;
                    i__4 = i__ + j * a_dim1;
                    i__5 = ix;
                    q__2.r =
                        (*x.offset(i__5 as isize)).r * temp.r -
                            (*x.offset(i__5 as isize)).i * temp.i;
                    q__2.i =
                        (*x.offset(i__5 as isize)).r * temp.i +
                            (*x.offset(i__5 as isize)).i * temp.r;
                    q__1.r = (*a.offset(i__4 as isize)).r + q__2.r;
                    q__1.i = (*a.offset(i__4 as isize)).i + q__2.i;
                    (*a.offset(i__3 as isize)).r = q__1.r;
                    (*a.offset(i__3 as isize)).i = q__1.i;
                    ix += *incx;
                    i__ += 1
                    /* L40: */
                    /* L30: */
                }
            }
            jy += *incy;
            j += 1
        }
    }
    return 0 as libc::c_int;
    /*     End of CGERU . */
}
/* cgeru_ */
