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
/* ctrmm.f -- translated by f2c (version 20061008).
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
pub unsafe extern "C" fn f2c_ctrmm(mut side: *mut libc::c_char,
                                   mut uplo: *mut libc::c_char,
                                   mut transa: *mut libc::c_char,
                                   mut diag: *mut libc::c_char,
                                   mut m: *mut integer, mut n: *mut integer,
                                   mut alpha: *mut complex,
                                   mut a: *mut complex, mut lda: *mut integer,
                                   mut b: *mut complex, mut ldb: *mut integer)
 -> libc::c_int {
    /* System generated locals */
    let mut a_dim1: integer = 0;
    let mut a_offset: integer = 0;
    let mut b_dim1: integer = 0;
    let mut b_offset: integer = 0;
    let mut i__1: integer = 0;
    let mut i__2: integer = 0;
    let mut i__3: integer = 0;
    let mut i__4: integer = 0;
    let mut i__5: integer = 0;
    let mut i__6: integer = 0;
    let mut q__1: complex = complex{r: 0., i: 0.,};
    let mut q__2: complex = complex{r: 0., i: 0.,};
    let mut q__3: complex = complex{r: 0., i: 0.,};
    /* Builtin functions */
    extern "C" {
        #[link_name = "r_cnjg"]
        fn r_cnjg_0(_: *mut complex, _: *mut complex);
    }
    /* Local variables */
    let mut i__: integer = 0;
    let mut j: integer = 0;
    let mut k: integer = 0;
    let mut info: integer = 0;
    let mut temp: complex = complex{r: 0., i: 0.,};
    extern "C" {
        #[link_name = "lsame_"]
        fn lsame__0(_: *mut libc::c_char, _: *mut libc::c_char) -> logical;
    }
    let mut lside: logical = 0;
    let mut nrowa: integer = 0;
    let mut upper: logical = 0;
    extern "C" {
        #[link_name = "xerbla_"]
        fn xerbla__0(_: *mut libc::c_char, _: *mut integer) -> libc::c_int;
    }
    let mut noconj: logical = 0;
    let mut nounit: logical = 0;
    /*     .. Scalar Arguments .. */
/*     .. */
/*     .. Array Arguments .. */
/*     .. */
    /*  Purpose */
/*  ======= */
    /*  CTRMM  performs one of the matrix-matrix operations */
    /*     B := alpha*op( A )*B,   or   B := alpha*B*op( A ) */
    /*  where  alpha  is a scalar,  B  is an m by n matrix,  A  is a unit, or */
/*  non-unit,  upper or lower triangular matrix  and  op( A )  is one  of */
    /*     op( A ) = A   or   op( A ) = A'   or   op( A ) = conjg( A' ). */
    /*  Arguments */
/*  ========== */
    /*  SIDE   - CHARACTER*1. */
/*           On entry,  SIDE specifies whether  op( A ) multiplies B from */
/*           the left or right as follows: */
    /*              SIDE = 'L' or 'l'   B := alpha*op( A )*B. */
    /*              SIDE = 'R' or 'r'   B := alpha*B*op( A ). */
    /*           Unchanged on exit. */
    /*  UPLO   - CHARACTER*1. */
/*           On entry, UPLO specifies whether the matrix A is an upper or */
/*           lower triangular matrix as follows: */
    /*              UPLO = 'U' or 'u'   A is an upper triangular matrix. */
    /*              UPLO = 'L' or 'l'   A is a lower triangular matrix. */
    /*           Unchanged on exit. */
    /*  TRANSA - CHARACTER*1. */
/*           On entry, TRANSA specifies the form of op( A ) to be used in */
/*           the matrix multiplication as follows: */
    /*              TRANSA = 'N' or 'n'   op( A ) = A. */
    /*              TRANSA = 'T' or 't'   op( A ) = A'. */
    /*              TRANSA = 'C' or 'c'   op( A ) = conjg( A' ). */
    /*           Unchanged on exit. */
    /*  DIAG   - CHARACTER*1. */
/*           On entry, DIAG specifies whether or not A is unit triangular */
/*           as follows: */
    /*              DIAG = 'U' or 'u'   A is assumed to be unit triangular. */
    /*              DIAG = 'N' or 'n'   A is not assumed to be unit */
/*                                  triangular. */
    /*           Unchanged on exit. */
    /*  M      - INTEGER. */
/*           On entry, M specifies the number of rows of B. M must be at */
/*           least zero. */
/*           Unchanged on exit. */
    /*  N      - INTEGER. */
/*           On entry, N specifies the number of columns of B.  N must be */
/*           at least zero. */
/*           Unchanged on exit. */
    /*  ALPHA  - COMPLEX         . */
/*           On entry,  ALPHA specifies the scalar  alpha. When  alpha is */
/*           zero then  A is not referenced and  B need not be set before */
/*           entry. */
/*           Unchanged on exit. */
    /*  A      - COMPLEX          array of DIMENSION ( LDA, k ), where k is m */
/*           when  SIDE = 'L' or 'l'  and is  n  when  SIDE = 'R' or 'r'. */
/*           Before entry  with  UPLO = 'U' or 'u',  the  leading  k by k */
/*           upper triangular part of the array  A must contain the upper */
/*           triangular matrix  and the strictly lower triangular part of */
/*           A is not referenced. */
/*           Before entry  with  UPLO = 'L' or 'l',  the  leading  k by k */
/*           lower triangular part of the array  A must contain the lower */
/*           triangular matrix  and the strictly upper triangular part of */
/*           A is not referenced. */
/*           Note that when  DIAG = 'U' or 'u',  the diagonal elements of */
/*           A  are not referenced either,  but are assumed to be  unity. */
/*           Unchanged on exit. */
    /*  LDA    - INTEGER. */
/*           On entry, LDA specifies the first dimension of A as declared */
/*           in the calling (sub) program.  When  SIDE = 'L' or 'l'  then */
/*           LDA  must be at least  max( 1, m ),  when  SIDE = 'R' or 'r' */
/*           then LDA must be at least max( 1, n ). */
/*           Unchanged on exit. */
    /*  B      - COMPLEX          array of DIMENSION ( LDB, n ). */
/*           Before entry,  the leading  m by n part of the array  B must */
/*           contain the matrix  B,  and  on exit  is overwritten  by the */
/*           transformed matrix. */
    /*  LDB    - INTEGER. */
/*           On entry, LDB specifies the first dimension of B as declared */
/*           in  the  calling  (sub)  program.   LDB  must  be  at  least */
/*           max( 1, m ). */
/*           Unchanged on exit. */
    /*  Level 3 Blas routine. */
    /*  -- Written on 8-February-1989. */
/*     Jack Dongarra, Argonne National Laboratory. */
/*     Iain Duff, AERE Harwell. */
/*     Jeremy Du Croz, Numerical Algorithms Group Ltd. */
/*     Sven Hammarling, Numerical Algorithms Group Ltd. */
    /*     .. External Functions .. */
/*     .. */
/*     .. External Subroutines .. */
/*     .. */
/*     .. Intrinsic Functions .. */
/*     .. */
/*     .. Local Scalars .. */
/*     .. */
/*     .. Parameters .. */
/*     .. */
    /*     Test the input parameters. */
    /* Parameter adjustments */
    a_dim1 = *lda;
    a_offset = 1 as libc::c_int as libc::c_long + a_dim1;
    a = a.offset(-(a_offset as isize));
    b_dim1 = *ldb;
    b_offset = 1 as libc::c_int as libc::c_long + b_dim1;
    b = b.offset(-(b_offset as isize));
    /* Function Body */
    lside =
        lsame__0(side,
                 b"L\x00" as *const u8 as *const libc::c_char as
                     *mut libc::c_char);
    if lside != 0 { nrowa = *m } else { nrowa = *n }
    noconj =
        lsame__0(transa,
                 b"T\x00" as *const u8 as *const libc::c_char as
                     *mut libc::c_char);
    nounit =
        lsame__0(diag,
                 b"N\x00" as *const u8 as *const libc::c_char as
                     *mut libc::c_char);
    upper =
        lsame__0(uplo,
                 b"U\x00" as *const u8 as *const libc::c_char as
                     *mut libc::c_char);
    info = 0 as libc::c_int as integer;
    if lside == 0 &&
           lsame__0(side,
                    b"R\x00" as *const u8 as *const libc::c_char as
                        *mut libc::c_char) == 0 {
        info = 1 as libc::c_int as integer
    } else if upper == 0 &&
                  lsame__0(uplo,
                           b"L\x00" as *const u8 as *const libc::c_char as
                               *mut libc::c_char) == 0 {
        info = 2 as libc::c_int as integer
    } else if lsame__0(transa,
                       b"N\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char) == 0 &&
                  lsame__0(transa,
                           b"T\x00" as *const u8 as *const libc::c_char as
                               *mut libc::c_char) == 0 &&
                  lsame__0(transa,
                           b"C\x00" as *const u8 as *const libc::c_char as
                               *mut libc::c_char) == 0 {
        info = 3 as libc::c_int as integer
    } else if lsame__0(diag,
                       b"U\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char) == 0 &&
                  lsame__0(diag,
                           b"N\x00" as *const u8 as *const libc::c_char as
                               *mut libc::c_char) == 0 {
        info = 4 as libc::c_int as integer
    } else if *m < 0 as libc::c_int as libc::c_long {
        info = 5 as libc::c_int as integer
    } else if *n < 0 as libc::c_int as libc::c_long {
        info = 6 as libc::c_int as integer
    } else if *lda <
                  (if 1 as libc::c_int as libc::c_long >= nrowa {
                       1 as libc::c_int as libc::c_long
                   } else { nrowa }) {
        info = 9 as libc::c_int as integer
    } else if *ldb <
                  (if 1 as libc::c_int as libc::c_long >= *m {
                       1 as libc::c_int as libc::c_long
                   } else { *m }) {
        info = 11 as libc::c_int as integer
    }
    if info != 0 as libc::c_int as libc::c_long {
        xerbla__0(b"CTRMM \x00" as *const u8 as *const libc::c_char as
                      *mut libc::c_char, &mut info);
        return 0 as libc::c_int
    }
    /*     Quick return if possible. */
    if *m == 0 as libc::c_int as libc::c_long ||
           *n == 0 as libc::c_int as libc::c_long {
        return 0 as libc::c_int
    }
    /*     And when  alpha.eq.zero. */
    if (*alpha).r == 0.0f32 && (*alpha).i == 0.0f32 {
        i__1 = *n;
        j = 1 as libc::c_int as integer;
        while j <= i__1 {
            i__2 = *m;
            i__ = 1 as libc::c_int as integer;
            while i__ <= i__2 {
                i__3 = i__ + j * b_dim1;
                (*b.offset(i__3 as isize)).r = 0.0f32;
                (*b.offset(i__3 as isize)).i = 0.0f32;
                i__ += 1
                /* L20: */
                /* L10: */
            }
            j += 1
        }
        return 0 as libc::c_int
    }
    /*     Start the operations. */
    if lside != 0 {
        if lsame__0(transa,
                    b"N\x00" as *const u8 as *const libc::c_char as
                        *mut libc::c_char) != 0 {
            /*           Form  B := alpha*A*B. */
            if upper != 0 {
                i__1 = *n;
                j = 1 as libc::c_int as integer;
                while j <= i__1 {
                    i__2 = *m;
                    k = 1 as libc::c_int as integer;
                    while k <= i__2 {
                        i__3 = k + j * b_dim1;
                        if (*b.offset(i__3 as isize)).r != 0.0f32 ||
                               (*b.offset(i__3 as isize)).i != 0.0f32 {
                            i__3 = k + j * b_dim1;
                            q__1.r =
                                (*alpha).r * (*b.offset(i__3 as isize)).r -
                                    (*alpha).i * (*b.offset(i__3 as isize)).i;
                            q__1.i =
                                (*alpha).r * (*b.offset(i__3 as isize)).i +
                                    (*alpha).i * (*b.offset(i__3 as isize)).r;
                            temp.r = q__1.r;
                            temp.i = q__1.i;
                            i__3 = k - 1 as libc::c_int as libc::c_long;
                            i__ = 1 as libc::c_int as integer;
                            while i__ <= i__3 {
                                i__4 = i__ + j * b_dim1;
                                i__5 = i__ + j * b_dim1;
                                i__6 = i__ + k * a_dim1;
                                q__2.r =
                                    temp.r * (*a.offset(i__6 as isize)).r -
                                        temp.i * (*a.offset(i__6 as isize)).i;
                                q__2.i =
                                    temp.r * (*a.offset(i__6 as isize)).i +
                                        temp.i * (*a.offset(i__6 as isize)).r;
                                q__1.r =
                                    (*b.offset(i__5 as isize)).r + q__2.r;
                                q__1.i =
                                    (*b.offset(i__5 as isize)).i + q__2.i;
                                (*b.offset(i__4 as isize)).r = q__1.r;
                                (*b.offset(i__4 as isize)).i = q__1.i;
                                i__ += 1
                                /* L50: */
                                /* L40: */
                                /* L30: */
                            }
                            if nounit != 0 {
                                i__3 = k + k * a_dim1;
                                q__1.r =
                                    temp.r * (*a.offset(i__3 as isize)).r -
                                        temp.i * (*a.offset(i__3 as isize)).i;
                                q__1.i =
                                    temp.r * (*a.offset(i__3 as isize)).i +
                                        temp.i * (*a.offset(i__3 as isize)).r;
                                temp.r = q__1.r;
                                temp.i = q__1.i
                            }
                            i__3 = k + j * b_dim1;
                            (*b.offset(i__3 as isize)).r = temp.r;
                            (*b.offset(i__3 as isize)).i = temp.i
                        }
                        k += 1
                    }
                    j += 1
                }
            } else {
                i__1 = *n;
                j = 1 as libc::c_int as integer;
                while j <= i__1 {
                    k = *m;
                    while k >= 1 as libc::c_int as libc::c_long {
                        i__2 = k + j * b_dim1;
                        if (*b.offset(i__2 as isize)).r != 0.0f32 ||
                               (*b.offset(i__2 as isize)).i != 0.0f32 {
                            i__2 = k + j * b_dim1;
                            q__1.r =
                                (*alpha).r * (*b.offset(i__2 as isize)).r -
                                    (*alpha).i * (*b.offset(i__2 as isize)).i;
                            q__1.i =
                                (*alpha).r * (*b.offset(i__2 as isize)).i +
                                    (*alpha).i * (*b.offset(i__2 as isize)).r;
                            temp.r = q__1.r;
                            temp.i = q__1.i;
                            i__2 = k + j * b_dim1;
                            (*b.offset(i__2 as isize)).r = temp.r;
                            (*b.offset(i__2 as isize)).i = temp.i;
                            if nounit != 0 {
                                i__2 = k + j * b_dim1;
                                i__3 = k + j * b_dim1;
                                i__4 = k + k * a_dim1;
                                q__1.r =
                                    (*b.offset(i__3 as isize)).r *
                                        (*a.offset(i__4 as isize)).r -
                                        (*b.offset(i__3 as isize)).i *
                                            (*a.offset(i__4 as isize)).i;
                                q__1.i =
                                    (*b.offset(i__3 as isize)).r *
                                        (*a.offset(i__4 as isize)).i +
                                        (*b.offset(i__3 as isize)).i *
                                            (*a.offset(i__4 as isize)).r;
                                (*b.offset(i__2 as isize)).r = q__1.r;
                                (*b.offset(i__2 as isize)).i = q__1.i
                            }
                            i__2 = *m;
                            i__ = k + 1 as libc::c_int as libc::c_long;
                            while i__ <= i__2 {
                                i__3 = i__ + j * b_dim1;
                                i__4 = i__ + j * b_dim1;
                                i__5 = i__ + k * a_dim1;
                                q__2.r =
                                    temp.r * (*a.offset(i__5 as isize)).r -
                                        temp.i * (*a.offset(i__5 as isize)).i;
                                q__2.i =
                                    temp.r * (*a.offset(i__5 as isize)).i +
                                        temp.i * (*a.offset(i__5 as isize)).r;
                                q__1.r =
                                    (*b.offset(i__4 as isize)).r + q__2.r;
                                q__1.i =
                                    (*b.offset(i__4 as isize)).i + q__2.i;
                                (*b.offset(i__3 as isize)).r = q__1.r;
                                (*b.offset(i__3 as isize)).i = q__1.i;
                                i__ += 1
                                /* L70: */
                                /* L60: */
                            }
                        }
                        k -= 1
                    }
                    j += 1
                    /* L80: */
                }
            }
        } else if upper != 0 {
            i__1 = *n;
            j = 1 as libc::c_int as integer;
            while j <= i__1 {
                i__ = *m;
                while i__ >= 1 as libc::c_int as libc::c_long {
                    i__2 = i__ + j * b_dim1;
                    temp.r = (*b.offset(i__2 as isize)).r;
                    temp.i = (*b.offset(i__2 as isize)).i;
                    if noconj != 0 {
                        if nounit != 0 {
                            i__2 = i__ + i__ * a_dim1;
                            q__1.r =
                                temp.r * (*a.offset(i__2 as isize)).r -
                                    temp.i * (*a.offset(i__2 as isize)).i;
                            q__1.i =
                                temp.r * (*a.offset(i__2 as isize)).i +
                                    temp.i * (*a.offset(i__2 as isize)).r;
                            temp.r = q__1.r;
                            temp.i = q__1.i
                        }
                        i__2 = i__ - 1 as libc::c_int as libc::c_long;
                        k = 1 as libc::c_int as integer;
                        while k <= i__2 {
                            i__3 = k + i__ * a_dim1;
                            i__4 = k + j * b_dim1;
                            q__2.r =
                                (*a.offset(i__3 as isize)).r *
                                    (*b.offset(i__4 as isize)).r -
                                    (*a.offset(i__3 as isize)).i *
                                        (*b.offset(i__4 as isize)).i;
                            q__2.i =
                                (*a.offset(i__3 as isize)).r *
                                    (*b.offset(i__4 as isize)).i +
                                    (*a.offset(i__3 as isize)).i *
                                        (*b.offset(i__4 as isize)).r;
                            q__1.r = temp.r + q__2.r;
                            q__1.i = temp.i + q__2.i;
                            temp.r = q__1.r;
                            temp.i = q__1.i;
                            k += 1
                            /*           Form  B := alpha*A'*B   or   B := alpha*conjg( A' )*B. */
                            /* L110: */
                            /* L90: */
                        }
                    } else {
                        if nounit != 0 {
                            r_cnjg_0(&mut q__2,
                                     &mut *a.offset((i__ + i__ * a_dim1) as
                                                        isize));
                            q__1.r = temp.r * q__2.r - temp.i * q__2.i;
                            q__1.i = temp.r * q__2.i + temp.i * q__2.r;
                            temp.r = q__1.r;
                            temp.i = q__1.i
                        }
                        i__2 = i__ - 1 as libc::c_int as libc::c_long;
                        k = 1 as libc::c_int as integer;
                        while k <= i__2 {
                            r_cnjg_0(&mut q__3,
                                     &mut *a.offset((k + i__ * a_dim1) as
                                                        isize));
                            i__3 = k + j * b_dim1;
                            q__2.r =
                                q__3.r * (*b.offset(i__3 as isize)).r -
                                    q__3.i * (*b.offset(i__3 as isize)).i;
                            q__2.i =
                                q__3.r * (*b.offset(i__3 as isize)).i +
                                    q__3.i * (*b.offset(i__3 as isize)).r;
                            q__1.r = temp.r + q__2.r;
                            q__1.i = temp.i + q__2.i;
                            temp.r = q__1.r;
                            temp.i = q__1.i;
                            k += 1
                            /* L100: */
                        }
                    }
                    i__2 = i__ + j * b_dim1;
                    q__1.r = (*alpha).r * temp.r - (*alpha).i * temp.i;
                    q__1.i = (*alpha).r * temp.i + (*alpha).i * temp.r;
                    (*b.offset(i__2 as isize)).r = q__1.r;
                    (*b.offset(i__2 as isize)).i = q__1.i;
                    i__ -= 1
                }
                j += 1
                /* L120: */
            }
        } else {
            i__1 = *n;
            j = 1 as libc::c_int as integer;
            while j <= i__1 {
                i__2 = *m;
                i__ = 1 as libc::c_int as integer;
                while i__ <= i__2 {
                    i__3 = i__ + j * b_dim1;
                    temp.r = (*b.offset(i__3 as isize)).r;
                    temp.i = (*b.offset(i__3 as isize)).i;
                    if noconj != 0 {
                        if nounit != 0 {
                            i__3 = i__ + i__ * a_dim1;
                            q__1.r =
                                temp.r * (*a.offset(i__3 as isize)).r -
                                    temp.i * (*a.offset(i__3 as isize)).i;
                            q__1.i =
                                temp.r * (*a.offset(i__3 as isize)).i +
                                    temp.i * (*a.offset(i__3 as isize)).r;
                            temp.r = q__1.r;
                            temp.i = q__1.i
                        }
                        i__3 = *m;
                        k = i__ + 1 as libc::c_int as libc::c_long;
                        while k <= i__3 {
                            i__4 = k + i__ * a_dim1;
                            i__5 = k + j * b_dim1;
                            q__2.r =
                                (*a.offset(i__4 as isize)).r *
                                    (*b.offset(i__5 as isize)).r -
                                    (*a.offset(i__4 as isize)).i *
                                        (*b.offset(i__5 as isize)).i;
                            q__2.i =
                                (*a.offset(i__4 as isize)).r *
                                    (*b.offset(i__5 as isize)).i +
                                    (*a.offset(i__4 as isize)).i *
                                        (*b.offset(i__5 as isize)).r;
                            q__1.r = temp.r + q__2.r;
                            q__1.i = temp.i + q__2.i;
                            temp.r = q__1.r;
                            temp.i = q__1.i;
                            k += 1
                            /* L160: */
                            /* L150: */
                            /* L130: */
                        }
                    } else {
                        if nounit != 0 {
                            r_cnjg_0(&mut q__2,
                                     &mut *a.offset((i__ + i__ * a_dim1) as
                                                        isize));
                            q__1.r = temp.r * q__2.r - temp.i * q__2.i;
                            q__1.i = temp.r * q__2.i + temp.i * q__2.r;
                            temp.r = q__1.r;
                            temp.i = q__1.i
                        }
                        i__3 = *m;
                        k = i__ + 1 as libc::c_int as libc::c_long;
                        while k <= i__3 {
                            r_cnjg_0(&mut q__3,
                                     &mut *a.offset((k + i__ * a_dim1) as
                                                        isize));
                            i__4 = k + j * b_dim1;
                            q__2.r =
                                q__3.r * (*b.offset(i__4 as isize)).r -
                                    q__3.i * (*b.offset(i__4 as isize)).i;
                            q__2.i =
                                q__3.r * (*b.offset(i__4 as isize)).i +
                                    q__3.i * (*b.offset(i__4 as isize)).r;
                            q__1.r = temp.r + q__2.r;
                            q__1.i = temp.i + q__2.i;
                            temp.r = q__1.r;
                            temp.i = q__1.i;
                            k += 1
                            /* L140: */
                        }
                    }
                    i__3 = i__ + j * b_dim1;
                    q__1.r = (*alpha).r * temp.r - (*alpha).i * temp.i;
                    q__1.i = (*alpha).r * temp.i + (*alpha).i * temp.r;
                    (*b.offset(i__3 as isize)).r = q__1.r;
                    (*b.offset(i__3 as isize)).i = q__1.i;
                    i__ += 1
                }
                j += 1
            }
        }
    } else if lsame__0(transa,
                       b"N\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char) != 0 {
        /*           Form  B := alpha*B*A. */
        if upper != 0 {
            j = *n;
            while j >= 1 as libc::c_int as libc::c_long {
                temp.r = (*alpha).r;
                temp.i = (*alpha).i;
                if nounit != 0 {
                    i__1 = j + j * a_dim1;
                    q__1.r =
                        temp.r * (*a.offset(i__1 as isize)).r -
                            temp.i * (*a.offset(i__1 as isize)).i;
                    q__1.i =
                        temp.r * (*a.offset(i__1 as isize)).i +
                            temp.i * (*a.offset(i__1 as isize)).r;
                    temp.r = q__1.r;
                    temp.i = q__1.i
                }
                i__1 = *m;
                i__ = 1 as libc::c_int as integer;
                while i__ <= i__1 {
                    i__2 = i__ + j * b_dim1;
                    i__3 = i__ + j * b_dim1;
                    q__1.r =
                        temp.r * (*b.offset(i__3 as isize)).r -
                            temp.i * (*b.offset(i__3 as isize)).i;
                    q__1.i =
                        temp.r * (*b.offset(i__3 as isize)).i +
                            temp.i * (*b.offset(i__3 as isize)).r;
                    (*b.offset(i__2 as isize)).r = q__1.r;
                    (*b.offset(i__2 as isize)).i = q__1.i;
                    i__ += 1
                    /* L200: */
                    /* L170: */
                }
                i__1 = j - 1 as libc::c_int as libc::c_long;
                k = 1 as libc::c_int as integer;
                while k <= i__1 {
                    i__2 = k + j * a_dim1;
                    if (*a.offset(i__2 as isize)).r != 0.0f32 ||
                           (*a.offset(i__2 as isize)).i != 0.0f32 {
                        i__2 = k + j * a_dim1;
                        q__1.r =
                            (*alpha).r * (*a.offset(i__2 as isize)).r -
                                (*alpha).i * (*a.offset(i__2 as isize)).i;
                        q__1.i =
                            (*alpha).r * (*a.offset(i__2 as isize)).i +
                                (*alpha).i * (*a.offset(i__2 as isize)).r;
                        temp.r = q__1.r;
                        temp.i = q__1.i;
                        i__2 = *m;
                        i__ = 1 as libc::c_int as integer;
                        while i__ <= i__2 {
                            i__3 = i__ + j * b_dim1;
                            i__4 = i__ + j * b_dim1;
                            i__5 = i__ + k * b_dim1;
                            q__2.r =
                                temp.r * (*b.offset(i__5 as isize)).r -
                                    temp.i * (*b.offset(i__5 as isize)).i;
                            q__2.i =
                                temp.r * (*b.offset(i__5 as isize)).i +
                                    temp.i * (*b.offset(i__5 as isize)).r;
                            q__1.r = (*b.offset(i__4 as isize)).r + q__2.r;
                            q__1.i = (*b.offset(i__4 as isize)).i + q__2.i;
                            (*b.offset(i__3 as isize)).r = q__1.r;
                            (*b.offset(i__3 as isize)).i = q__1.i;
                            i__ += 1
                            /* L190: */
                            /* L180: */
                        }
                    }
                    k += 1
                }
                j -= 1
            }
        } else {
            i__1 = *n;
            j = 1 as libc::c_int as integer;
            while j <= i__1 {
                temp.r = (*alpha).r;
                temp.i = (*alpha).i;
                if nounit != 0 {
                    i__2 = j + j * a_dim1;
                    q__1.r =
                        temp.r * (*a.offset(i__2 as isize)).r -
                            temp.i * (*a.offset(i__2 as isize)).i;
                    q__1.i =
                        temp.r * (*a.offset(i__2 as isize)).i +
                            temp.i * (*a.offset(i__2 as isize)).r;
                    temp.r = q__1.r;
                    temp.i = q__1.i
                }
                i__2 = *m;
                i__ = 1 as libc::c_int as integer;
                while i__ <= i__2 {
                    i__3 = i__ + j * b_dim1;
                    i__4 = i__ + j * b_dim1;
                    q__1.r =
                        temp.r * (*b.offset(i__4 as isize)).r -
                            temp.i * (*b.offset(i__4 as isize)).i;
                    q__1.i =
                        temp.r * (*b.offset(i__4 as isize)).i +
                            temp.i * (*b.offset(i__4 as isize)).r;
                    (*b.offset(i__3 as isize)).r = q__1.r;
                    (*b.offset(i__3 as isize)).i = q__1.i;
                    i__ += 1
                    /* L240: */
                    /* L210: */
                }
                i__2 = *n;
                k = j + 1 as libc::c_int as libc::c_long;
                while k <= i__2 {
                    i__3 = k + j * a_dim1;
                    if (*a.offset(i__3 as isize)).r != 0.0f32 ||
                           (*a.offset(i__3 as isize)).i != 0.0f32 {
                        i__3 = k + j * a_dim1;
                        q__1.r =
                            (*alpha).r * (*a.offset(i__3 as isize)).r -
                                (*alpha).i * (*a.offset(i__3 as isize)).i;
                        q__1.i =
                            (*alpha).r * (*a.offset(i__3 as isize)).i +
                                (*alpha).i * (*a.offset(i__3 as isize)).r;
                        temp.r = q__1.r;
                        temp.i = q__1.i;
                        i__3 = *m;
                        i__ = 1 as libc::c_int as integer;
                        while i__ <= i__3 {
                            i__4 = i__ + j * b_dim1;
                            i__5 = i__ + j * b_dim1;
                            i__6 = i__ + k * b_dim1;
                            q__2.r =
                                temp.r * (*b.offset(i__6 as isize)).r -
                                    temp.i * (*b.offset(i__6 as isize)).i;
                            q__2.i =
                                temp.r * (*b.offset(i__6 as isize)).i +
                                    temp.i * (*b.offset(i__6 as isize)).r;
                            q__1.r = (*b.offset(i__5 as isize)).r + q__2.r;
                            q__1.i = (*b.offset(i__5 as isize)).i + q__2.i;
                            (*b.offset(i__4 as isize)).r = q__1.r;
                            (*b.offset(i__4 as isize)).i = q__1.i;
                            i__ += 1
                            /* L230: */
                            /* L220: */
                        }
                    }
                    k += 1
                }
                j += 1
            }
        }
    } else if upper != 0 {
        i__1 = *n;
        k = 1 as libc::c_int as integer;
        while k <= i__1 {
            i__2 = k - 1 as libc::c_int as libc::c_long;
            j = 1 as libc::c_int as integer;
            while j <= i__2 {
                i__3 = j + k * a_dim1;
                if (*a.offset(i__3 as isize)).r != 0.0f32 ||
                       (*a.offset(i__3 as isize)).i != 0.0f32 {
                    if noconj != 0 {
                        i__3 = j + k * a_dim1;
                        q__1.r =
                            (*alpha).r * (*a.offset(i__3 as isize)).r -
                                (*alpha).i * (*a.offset(i__3 as isize)).i;
                        q__1.i =
                            (*alpha).r * (*a.offset(i__3 as isize)).i +
                                (*alpha).i * (*a.offset(i__3 as isize)).r;
                        temp.r = q__1.r;
                        temp.i = q__1.i
                    } else {
                        r_cnjg_0(&mut q__2,
                                 &mut *a.offset((j + k * a_dim1) as isize));
                        q__1.r = (*alpha).r * q__2.r - (*alpha).i * q__2.i;
                        q__1.i = (*alpha).r * q__2.i + (*alpha).i * q__2.r;
                        temp.r = q__1.r;
                        temp.i = q__1.i
                    }
                    i__3 = *m;
                    i__ = 1 as libc::c_int as integer;
                    while i__ <= i__3 {
                        i__4 = i__ + j * b_dim1;
                        i__5 = i__ + j * b_dim1;
                        i__6 = i__ + k * b_dim1;
                        q__2.r =
                            temp.r * (*b.offset(i__6 as isize)).r -
                                temp.i * (*b.offset(i__6 as isize)).i;
                        q__2.i =
                            temp.r * (*b.offset(i__6 as isize)).i +
                                temp.i * (*b.offset(i__6 as isize)).r;
                        q__1.r = (*b.offset(i__5 as isize)).r + q__2.r;
                        q__1.i = (*b.offset(i__5 as isize)).i + q__2.i;
                        (*b.offset(i__4 as isize)).r = q__1.r;
                        (*b.offset(i__4 as isize)).i = q__1.i;
                        i__ += 1
                        /*           Form  B := alpha*B*A'   or   B := alpha*B*conjg( A' ). */
                        /* L280: */
                        /* L260: */
                        /* L250: */
                    }
                }
                j += 1
            }
            temp.r = (*alpha).r;
            temp.i = (*alpha).i;
            if nounit != 0 {
                if noconj != 0 {
                    i__2 = k + k * a_dim1;
                    q__1.r =
                        temp.r * (*a.offset(i__2 as isize)).r -
                            temp.i * (*a.offset(i__2 as isize)).i;
                    q__1.i =
                        temp.r * (*a.offset(i__2 as isize)).i +
                            temp.i * (*a.offset(i__2 as isize)).r;
                    temp.r = q__1.r;
                    temp.i = q__1.i
                } else {
                    r_cnjg_0(&mut q__2,
                             &mut *a.offset((k + k * a_dim1) as isize));
                    q__1.r = temp.r * q__2.r - temp.i * q__2.i;
                    q__1.i = temp.r * q__2.i + temp.i * q__2.r;
                    temp.r = q__1.r;
                    temp.i = q__1.i
                }
            }
            if temp.r != 1.0f32 || temp.i != 0.0f32 {
                i__2 = *m;
                i__ = 1 as libc::c_int as integer;
                while i__ <= i__2 {
                    i__3 = i__ + k * b_dim1;
                    i__4 = i__ + k * b_dim1;
                    q__1.r =
                        temp.r * (*b.offset(i__4 as isize)).r -
                            temp.i * (*b.offset(i__4 as isize)).i;
                    q__1.i =
                        temp.r * (*b.offset(i__4 as isize)).i +
                            temp.i * (*b.offset(i__4 as isize)).r;
                    (*b.offset(i__3 as isize)).r = q__1.r;
                    (*b.offset(i__3 as isize)).i = q__1.i;
                    i__ += 1
                    /* L270: */
                }
            }
            k += 1
        }
    } else {
        k = *n;
        while k >= 1 as libc::c_int as libc::c_long {
            i__1 = *n;
            j = k + 1 as libc::c_int as libc::c_long;
            while j <= i__1 {
                i__2 = j + k * a_dim1;
                if (*a.offset(i__2 as isize)).r != 0.0f32 ||
                       (*a.offset(i__2 as isize)).i != 0.0f32 {
                    if noconj != 0 {
                        i__2 = j + k * a_dim1;
                        q__1.r =
                            (*alpha).r * (*a.offset(i__2 as isize)).r -
                                (*alpha).i * (*a.offset(i__2 as isize)).i;
                        q__1.i =
                            (*alpha).r * (*a.offset(i__2 as isize)).i +
                                (*alpha).i * (*a.offset(i__2 as isize)).r;
                        temp.r = q__1.r;
                        temp.i = q__1.i
                    } else {
                        r_cnjg_0(&mut q__2,
                                 &mut *a.offset((j + k * a_dim1) as isize));
                        q__1.r = (*alpha).r * q__2.r - (*alpha).i * q__2.i;
                        q__1.i = (*alpha).r * q__2.i + (*alpha).i * q__2.r;
                        temp.r = q__1.r;
                        temp.i = q__1.i
                    }
                    i__2 = *m;
                    i__ = 1 as libc::c_int as integer;
                    while i__ <= i__2 {
                        i__3 = i__ + j * b_dim1;
                        i__4 = i__ + j * b_dim1;
                        i__5 = i__ + k * b_dim1;
                        q__2.r =
                            temp.r * (*b.offset(i__5 as isize)).r -
                                temp.i * (*b.offset(i__5 as isize)).i;
                        q__2.i =
                            temp.r * (*b.offset(i__5 as isize)).i +
                                temp.i * (*b.offset(i__5 as isize)).r;
                        q__1.r = (*b.offset(i__4 as isize)).r + q__2.r;
                        q__1.i = (*b.offset(i__4 as isize)).i + q__2.i;
                        (*b.offset(i__3 as isize)).r = q__1.r;
                        (*b.offset(i__3 as isize)).i = q__1.i;
                        i__ += 1
                        /* L320: */
                        /* L300: */
                        /* L290: */
                    }
                }
                j += 1
            }
            temp.r = (*alpha).r;
            temp.i = (*alpha).i;
            if nounit != 0 {
                if noconj != 0 {
                    i__1 = k + k * a_dim1;
                    q__1.r =
                        temp.r * (*a.offset(i__1 as isize)).r -
                            temp.i * (*a.offset(i__1 as isize)).i;
                    q__1.i =
                        temp.r * (*a.offset(i__1 as isize)).i +
                            temp.i * (*a.offset(i__1 as isize)).r;
                    temp.r = q__1.r;
                    temp.i = q__1.i
                } else {
                    r_cnjg_0(&mut q__2,
                             &mut *a.offset((k + k * a_dim1) as isize));
                    q__1.r = temp.r * q__2.r - temp.i * q__2.i;
                    q__1.i = temp.r * q__2.i + temp.i * q__2.r;
                    temp.r = q__1.r;
                    temp.i = q__1.i
                }
            }
            if temp.r != 1.0f32 || temp.i != 0.0f32 {
                i__1 = *m;
                i__ = 1 as libc::c_int as integer;
                while i__ <= i__1 {
                    i__2 = i__ + k * b_dim1;
                    i__3 = i__ + k * b_dim1;
                    q__1.r =
                        temp.r * (*b.offset(i__3 as isize)).r -
                            temp.i * (*b.offset(i__3 as isize)).i;
                    q__1.i =
                        temp.r * (*b.offset(i__3 as isize)).i +
                            temp.i * (*b.offset(i__3 as isize)).r;
                    (*b.offset(i__2 as isize)).r = q__1.r;
                    (*b.offset(i__2 as isize)).i = q__1.i;
                    i__ += 1
                    /* L310: */
                }
            }
            k -= 1
        }
    }
    return 0 as libc::c_int;
    /*     End of CTRMM . */
}
/* ctrmm_ */
