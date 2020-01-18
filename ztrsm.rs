use libc;
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
/* ztrsm.f -- translated by f2c (version 20061008).
   You must link the resulting object file with libf2c:
    on Microsoft Windows system, link with libf2c.lib;
    on Linux or Unix systems, link with .../path/to/libf2c.a -lm
    or, if you install libf2c.a in a standard place, with -lf2c -lm
    -- in that order, at the end of the command line, as in
        cc *.o -lf2c -lm
    Source for libf2c is in /netlib/f2c/libf2c.zip, e.g.,

        http://www.netlib.org/f2c/libf2c.zip
*/
/* Table of constant values */
static mut c_b1: doublecomplex = {
    let mut init = doublecomplex {
        r: 1.0f64,
        i: 0.0f64,
    };
    init
};
/* Subroutine */
#[no_mangle]
pub unsafe extern "C" fn f2c_ztrsm(
    mut side: *mut libc::c_char,
    mut uplo: *mut libc::c_char,
    mut transa: *mut libc::c_char,
    mut diag: *mut libc::c_char,
    mut m: *mut integer,
    mut n: *mut integer,
    mut alpha: *mut doublecomplex,
    mut a: *mut doublecomplex,
    mut lda: *mut integer,
    mut b: *mut doublecomplex,
    mut ldb: *mut integer,
) -> libc::c_int {
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
    let mut i__7: integer = 0;
    let mut z__1: doublecomplex = doublecomplex { r: 0., i: 0. };
    let mut z__2: doublecomplex = doublecomplex { r: 0., i: 0. };
    let mut z__3: doublecomplex = doublecomplex { r: 0., i: 0. };
    /* Builtin functions */
    extern "C" {
        #[link_name = "z_div"]
        fn z_div_0(_: *mut doublecomplex, _: *mut doublecomplex, _: *mut doublecomplex);
    }
    extern "C" {
        #[link_name = "d_cnjg"]
        fn d_cnjg_0(_: *mut doublecomplex, _: *mut doublecomplex);
    }
    /* Local variables */
    let mut i__: integer = 0;
    let mut j: integer = 0;
    let mut k: integer = 0;
    let mut info: integer = 0;
    let mut temp: doublecomplex = doublecomplex { r: 0., i: 0. };
    let mut lside: logical = 0;
    extern "C" {
        #[link_name = "lsame_"]
        fn lsame__0(_: *mut libc::c_char, _: *mut libc::c_char) -> logical;
    }
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
    /*  ZTRSM  solves one of the matrix equations */
    /*     op( A )*X = alpha*B,   or   X*op( A ) = alpha*B, */
    /*  where alpha is a scalar, X and B are m by n matrices, A is a unit, or */
    /*  non-unit,  upper or lower triangular matrix  and  op( A )  is one  of */
    /*     op( A ) = A   or   op( A ) = A'   or   op( A ) = conjg( A' ). */
    /*  The matrix X is overwritten on B. */
    /*  Arguments */
    /*  ========== */
    /*  SIDE   - CHARACTER*1. */
    /*           On entry, SIDE specifies whether op( A ) appears on the left */
    /*           or right of X as follows: */
    /*              SIDE = 'L' or 'l'   op( A )*X = alpha*B. */
    /*              SIDE = 'R' or 'r'   X*op( A ) = alpha*B. */
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
    /*  ALPHA  - COMPLEX*16      . */
    /*           On entry,  ALPHA specifies the scalar  alpha. When  alpha is */
    /*           zero then  A is not referenced and  B need not be set before */
    /*           entry. */
    /*           Unchanged on exit. */
    /*  A      - COMPLEX*16       array of DIMENSION ( LDA, k ), where k is m */
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
    /*  B      - COMPLEX*16       array of DIMENSION ( LDB, n ). */
    /*           Before entry,  the leading  m by n part of the array  B must */
    /*           contain  the  right-hand  side  matrix  B,  and  on exit  is */
    /*           overwritten by the solution matrix  X. */
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
    lside = lsame__0(
        side,
        b"L\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if lside != 0 {
        nrowa = *m
    } else {
        nrowa = *n
    }
    noconj = lsame__0(
        transa,
        b"T\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    nounit = lsame__0(
        diag,
        b"N\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    upper = lsame__0(
        uplo,
        b"U\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    info = 0 as libc::c_int as integer;
    if lside == 0
        && lsame__0(
            side,
            b"R\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == 0
    {
        info = 1 as libc::c_int as integer
    } else if upper == 0
        && lsame__0(
            uplo,
            b"L\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == 0
    {
        info = 2 as libc::c_int as integer
    } else if lsame__0(
        transa,
        b"N\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 0
        && lsame__0(
            transa,
            b"T\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == 0
        && lsame__0(
            transa,
            b"C\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == 0
    {
        info = 3 as libc::c_int as integer
    } else if lsame__0(
        diag,
        b"U\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 0
        && lsame__0(
            diag,
            b"N\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == 0
    {
        info = 4 as libc::c_int as integer
    } else if *m < 0 as libc::c_int as libc::c_long {
        info = 5 as libc::c_int as integer
    } else if *n < 0 as libc::c_int as libc::c_long {
        info = 6 as libc::c_int as integer
    } else if *lda
        < (if 1 as libc::c_int as libc::c_long >= nrowa {
            1 as libc::c_int as libc::c_long
        } else {
            nrowa
        })
    {
        info = 9 as libc::c_int as integer
    } else if *ldb
        < (if 1 as libc::c_int as libc::c_long >= *m {
            1 as libc::c_int as libc::c_long
        } else {
            *m
        })
    {
        info = 11 as libc::c_int as integer
    }
    if info != 0 as libc::c_int as libc::c_long {
        xerbla__0(
            b"ZTRSM \x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            &mut info,
        );
        return 0 as libc::c_int;
    }
    /*     Quick return if possible. */
    if *m == 0 as libc::c_int as libc::c_long || *n == 0 as libc::c_int as libc::c_long {
        return 0 as libc::c_int;
    }
    /*     And when  alpha.eq.zero. */
    if (*alpha).r == 0.0f64 && (*alpha).i == 0.0f64 {
        i__1 = *n;
        j = 1 as libc::c_int as integer;
        while j <= i__1 {
            i__2 = *m;
            i__ = 1 as libc::c_int as integer;
            while i__ <= i__2 {
                i__3 = i__ + j * b_dim1;
                (*b.offset(i__3 as isize)).r = 0.0f64;
                (*b.offset(i__3 as isize)).i = 0.0f64;
                i__ += 1
                /* L20: */
                /* L10: */
            }
            j += 1
        }
        return 0 as libc::c_int;
    }
    /*     Start the operations. */
    if lside != 0 {
        if lsame__0(
            transa,
            b"N\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) != 0
        {
            /*           Form  B := alpha*inv( A )*B. */
            if upper != 0 {
                i__1 = *n;
                j = 1 as libc::c_int as integer;
                while j <= i__1 {
                    if (*alpha).r != 1.0f64 || (*alpha).i != 0.0f64 {
                        i__2 = *m;
                        i__ = 1 as libc::c_int as integer;
                        while i__ <= i__2 {
                            i__3 = i__ + j * b_dim1;
                            i__4 = i__ + j * b_dim1;
                            z__1.r = (*alpha).r * (*b.offset(i__4 as isize)).r
                                - (*alpha).i * (*b.offset(i__4 as isize)).i;
                            z__1.i = (*alpha).r * (*b.offset(i__4 as isize)).i
                                + (*alpha).i * (*b.offset(i__4 as isize)).r;
                            (*b.offset(i__3 as isize)).r = z__1.r;
                            (*b.offset(i__3 as isize)).i = z__1.i;
                            i__ += 1
                            /* L30: */
                        }
                    }
                    k = *m;
                    while k >= 1 as libc::c_int as libc::c_long {
                        i__2 = k + j * b_dim1;
                        if (*b.offset(i__2 as isize)).r != 0.0f64
                            || (*b.offset(i__2 as isize)).i != 0.0f64
                        {
                            if nounit != 0 {
                                i__2 = k + j * b_dim1;
                                z_div_0(
                                    &mut z__1,
                                    &mut *b.offset((k + j * b_dim1) as isize),
                                    &mut *a.offset((k + k * a_dim1) as isize),
                                );
                                (*b.offset(i__2 as isize)).r = z__1.r;
                                (*b.offset(i__2 as isize)).i = z__1.i
                            }
                            i__2 = k - 1 as libc::c_int as libc::c_long;
                            i__ = 1 as libc::c_int as integer;
                            while i__ <= i__2 {
                                i__3 = i__ + j * b_dim1;
                                i__4 = i__ + j * b_dim1;
                                i__5 = k + j * b_dim1;
                                i__6 = i__ + k * a_dim1;
                                z__2.r = (*b.offset(i__5 as isize)).r
                                    * (*a.offset(i__6 as isize)).r
                                    - (*b.offset(i__5 as isize)).i * (*a.offset(i__6 as isize)).i;
                                z__2.i = (*b.offset(i__5 as isize)).r
                                    * (*a.offset(i__6 as isize)).i
                                    + (*b.offset(i__5 as isize)).i * (*a.offset(i__6 as isize)).r;
                                z__1.r = (*b.offset(i__4 as isize)).r - z__2.r;
                                z__1.i = (*b.offset(i__4 as isize)).i - z__2.i;
                                (*b.offset(i__3 as isize)).r = z__1.r;
                                (*b.offset(i__3 as isize)).i = z__1.i;
                                i__ += 1
                                /* L60: */
                                /* L50: */
                                /* L40: */
                            }
                        }
                        k -= 1
                    }
                    j += 1
                }
            } else {
                i__1 = *n;
                j = 1 as libc::c_int as integer;
                while j <= i__1 {
                    if (*alpha).r != 1.0f64 || (*alpha).i != 0.0f64 {
                        i__2 = *m;
                        i__ = 1 as libc::c_int as integer;
                        while i__ <= i__2 {
                            i__3 = i__ + j * b_dim1;
                            i__4 = i__ + j * b_dim1;
                            z__1.r = (*alpha).r * (*b.offset(i__4 as isize)).r
                                - (*alpha).i * (*b.offset(i__4 as isize)).i;
                            z__1.i = (*alpha).r * (*b.offset(i__4 as isize)).i
                                + (*alpha).i * (*b.offset(i__4 as isize)).r;
                            (*b.offset(i__3 as isize)).r = z__1.r;
                            (*b.offset(i__3 as isize)).i = z__1.i;
                            i__ += 1
                            /* L70: */
                        }
                    }
                    i__2 = *m;
                    k = 1 as libc::c_int as integer;
                    while k <= i__2 {
                        i__3 = k + j * b_dim1;
                        if (*b.offset(i__3 as isize)).r != 0.0f64
                            || (*b.offset(i__3 as isize)).i != 0.0f64
                        {
                            if nounit != 0 {
                                i__3 = k + j * b_dim1;
                                z_div_0(
                                    &mut z__1,
                                    &mut *b.offset((k + j * b_dim1) as isize),
                                    &mut *a.offset((k + k * a_dim1) as isize),
                                );
                                (*b.offset(i__3 as isize)).r = z__1.r;
                                (*b.offset(i__3 as isize)).i = z__1.i
                            }
                            i__3 = *m;
                            i__ = k + 1 as libc::c_int as libc::c_long;
                            while i__ <= i__3 {
                                i__4 = i__ + j * b_dim1;
                                i__5 = i__ + j * b_dim1;
                                i__6 = k + j * b_dim1;
                                i__7 = i__ + k * a_dim1;
                                z__2.r = (*b.offset(i__6 as isize)).r
                                    * (*a.offset(i__7 as isize)).r
                                    - (*b.offset(i__6 as isize)).i * (*a.offset(i__7 as isize)).i;
                                z__2.i = (*b.offset(i__6 as isize)).r
                                    * (*a.offset(i__7 as isize)).i
                                    + (*b.offset(i__6 as isize)).i * (*a.offset(i__7 as isize)).r;
                                z__1.r = (*b.offset(i__5 as isize)).r - z__2.r;
                                z__1.i = (*b.offset(i__5 as isize)).i - z__2.i;
                                (*b.offset(i__4 as isize)).r = z__1.r;
                                (*b.offset(i__4 as isize)).i = z__1.i;
                                i__ += 1
                                /* L100: */
                                /* L90: */
                                /* L80: */
                            }
                        }
                        k += 1
                    }
                    j += 1
                }
            }
        } else if upper != 0 {
            i__1 = *n;
            j = 1 as libc::c_int as integer;
            while j <= i__1 {
                i__2 = *m;
                i__ = 1 as libc::c_int as integer;
                while i__ <= i__2 {
                    i__3 = i__ + j * b_dim1;
                    z__1.r = (*alpha).r * (*b.offset(i__3 as isize)).r
                        - (*alpha).i * (*b.offset(i__3 as isize)).i;
                    z__1.i = (*alpha).r * (*b.offset(i__3 as isize)).i
                        + (*alpha).i * (*b.offset(i__3 as isize)).r;
                    temp.r = z__1.r;
                    temp.i = z__1.i;
                    if noconj != 0 {
                        i__3 = i__ - 1 as libc::c_int as libc::c_long;
                        k = 1 as libc::c_int as integer;
                        while k <= i__3 {
                            i__4 = k + i__ * a_dim1;
                            i__5 = k + j * b_dim1;
                            z__2.r = (*a.offset(i__4 as isize)).r * (*b.offset(i__5 as isize)).r
                                - (*a.offset(i__4 as isize)).i * (*b.offset(i__5 as isize)).i;
                            z__2.i = (*a.offset(i__4 as isize)).r * (*b.offset(i__5 as isize)).i
                                + (*a.offset(i__4 as isize)).i * (*b.offset(i__5 as isize)).r;
                            z__1.r = temp.r - z__2.r;
                            z__1.i = temp.i - z__2.i;
                            temp.r = z__1.r;
                            temp.i = z__1.i;
                            k += 1
                            /*           Form  B := alpha*inv( A' )*B */
                            /*           or    B := alpha*inv( conjg( A' ) )*B. */
                            /* L140: */
                            /* L130: */
                            /* L110: */
                        }
                        if nounit != 0 {
                            z_div_0(
                                &mut z__1,
                                &mut temp,
                                &mut *a.offset((i__ + i__ * a_dim1) as isize),
                            );
                            temp.r = z__1.r;
                            temp.i = z__1.i
                        }
                    } else {
                        i__3 = i__ - 1 as libc::c_int as libc::c_long;
                        k = 1 as libc::c_int as integer;
                        while k <= i__3 {
                            d_cnjg_0(&mut z__3, &mut *a.offset((k + i__ * a_dim1) as isize));
                            i__4 = k + j * b_dim1;
                            z__2.r = z__3.r * (*b.offset(i__4 as isize)).r
                                - z__3.i * (*b.offset(i__4 as isize)).i;
                            z__2.i = z__3.r * (*b.offset(i__4 as isize)).i
                                + z__3.i * (*b.offset(i__4 as isize)).r;
                            z__1.r = temp.r - z__2.r;
                            z__1.i = temp.i - z__2.i;
                            temp.r = z__1.r;
                            temp.i = z__1.i;
                            k += 1
                            /* L120: */
                        }
                        if nounit != 0 {
                            d_cnjg_0(&mut z__2, &mut *a.offset((i__ + i__ * a_dim1) as isize));
                            z_div_0(&mut z__1, &mut temp, &mut z__2);
                            temp.r = z__1.r;
                            temp.i = z__1.i
                        }
                    }
                    i__3 = i__ + j * b_dim1;
                    (*b.offset(i__3 as isize)).r = temp.r;
                    (*b.offset(i__3 as isize)).i = temp.i;
                    i__ += 1
                }
                j += 1
            }
        } else {
            i__1 = *n;
            j = 1 as libc::c_int as integer;
            while j <= i__1 {
                i__ = *m;
                while i__ >= 1 as libc::c_int as libc::c_long {
                    i__2 = i__ + j * b_dim1;
                    z__1.r = (*alpha).r * (*b.offset(i__2 as isize)).r
                        - (*alpha).i * (*b.offset(i__2 as isize)).i;
                    z__1.i = (*alpha).r * (*b.offset(i__2 as isize)).i
                        + (*alpha).i * (*b.offset(i__2 as isize)).r;
                    temp.r = z__1.r;
                    temp.i = z__1.i;
                    if noconj != 0 {
                        i__2 = *m;
                        k = i__ + 1 as libc::c_int as libc::c_long;
                        while k <= i__2 {
                            i__3 = k + i__ * a_dim1;
                            i__4 = k + j * b_dim1;
                            z__2.r = (*a.offset(i__3 as isize)).r * (*b.offset(i__4 as isize)).r
                                - (*a.offset(i__3 as isize)).i * (*b.offset(i__4 as isize)).i;
                            z__2.i = (*a.offset(i__3 as isize)).r * (*b.offset(i__4 as isize)).i
                                + (*a.offset(i__3 as isize)).i * (*b.offset(i__4 as isize)).r;
                            z__1.r = temp.r - z__2.r;
                            z__1.i = temp.i - z__2.i;
                            temp.r = z__1.r;
                            temp.i = z__1.i;
                            k += 1
                            /* L170: */
                            /* L150: */
                        }
                        if nounit != 0 {
                            z_div_0(
                                &mut z__1,
                                &mut temp,
                                &mut *a.offset((i__ + i__ * a_dim1) as isize),
                            );
                            temp.r = z__1.r;
                            temp.i = z__1.i
                        }
                    } else {
                        i__2 = *m;
                        k = i__ + 1 as libc::c_int as libc::c_long;
                        while k <= i__2 {
                            d_cnjg_0(&mut z__3, &mut *a.offset((k + i__ * a_dim1) as isize));
                            i__3 = k + j * b_dim1;
                            z__2.r = z__3.r * (*b.offset(i__3 as isize)).r
                                - z__3.i * (*b.offset(i__3 as isize)).i;
                            z__2.i = z__3.r * (*b.offset(i__3 as isize)).i
                                + z__3.i * (*b.offset(i__3 as isize)).r;
                            z__1.r = temp.r - z__2.r;
                            z__1.i = temp.i - z__2.i;
                            temp.r = z__1.r;
                            temp.i = z__1.i;
                            k += 1
                            /* L160: */
                        }
                        if nounit != 0 {
                            d_cnjg_0(&mut z__2, &mut *a.offset((i__ + i__ * a_dim1) as isize));
                            z_div_0(&mut z__1, &mut temp, &mut z__2);
                            temp.r = z__1.r;
                            temp.i = z__1.i
                        }
                    }
                    i__2 = i__ + j * b_dim1;
                    (*b.offset(i__2 as isize)).r = temp.r;
                    (*b.offset(i__2 as isize)).i = temp.i;
                    i__ -= 1
                }
                j += 1
                /* L180: */
            }
        }
    } else if lsame__0(
        transa,
        b"N\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) != 0
    {
        /*           Form  B := alpha*B*inv( A ). */
        if upper != 0 {
            i__1 = *n;
            j = 1 as libc::c_int as integer;
            while j <= i__1 {
                if (*alpha).r != 1.0f64 || (*alpha).i != 0.0f64 {
                    i__2 = *m;
                    i__ = 1 as libc::c_int as integer;
                    while i__ <= i__2 {
                        i__3 = i__ + j * b_dim1;
                        i__4 = i__ + j * b_dim1;
                        z__1.r = (*alpha).r * (*b.offset(i__4 as isize)).r
                            - (*alpha).i * (*b.offset(i__4 as isize)).i;
                        z__1.i = (*alpha).r * (*b.offset(i__4 as isize)).i
                            + (*alpha).i * (*b.offset(i__4 as isize)).r;
                        (*b.offset(i__3 as isize)).r = z__1.r;
                        (*b.offset(i__3 as isize)).i = z__1.i;
                        i__ += 1
                        /* L190: */
                    }
                }
                i__2 = j - 1 as libc::c_int as libc::c_long;
                k = 1 as libc::c_int as integer;
                while k <= i__2 {
                    i__3 = k + j * a_dim1;
                    if (*a.offset(i__3 as isize)).r != 0.0f64
                        || (*a.offset(i__3 as isize)).i != 0.0f64
                    {
                        i__3 = *m;
                        i__ = 1 as libc::c_int as integer;
                        while i__ <= i__3 {
                            i__4 = i__ + j * b_dim1;
                            i__5 = i__ + j * b_dim1;
                            i__6 = k + j * a_dim1;
                            i__7 = i__ + k * b_dim1;
                            z__2.r = (*a.offset(i__6 as isize)).r * (*b.offset(i__7 as isize)).r
                                - (*a.offset(i__6 as isize)).i * (*b.offset(i__7 as isize)).i;
                            z__2.i = (*a.offset(i__6 as isize)).r * (*b.offset(i__7 as isize)).i
                                + (*a.offset(i__6 as isize)).i * (*b.offset(i__7 as isize)).r;
                            z__1.r = (*b.offset(i__5 as isize)).r - z__2.r;
                            z__1.i = (*b.offset(i__5 as isize)).i - z__2.i;
                            (*b.offset(i__4 as isize)).r = z__1.r;
                            (*b.offset(i__4 as isize)).i = z__1.i;
                            i__ += 1
                            /* L230: */
                            /* L210: */
                            /* L200: */
                        }
                    }
                    k += 1
                }
                if nounit != 0 {
                    z_div_0(
                        &mut z__1,
                        &mut c_b1,
                        &mut *a.offset((j + j * a_dim1) as isize),
                    );
                    temp.r = z__1.r;
                    temp.i = z__1.i;
                    i__2 = *m;
                    i__ = 1 as libc::c_int as integer;
                    while i__ <= i__2 {
                        i__3 = i__ + j * b_dim1;
                        i__4 = i__ + j * b_dim1;
                        z__1.r = temp.r * (*b.offset(i__4 as isize)).r
                            - temp.i * (*b.offset(i__4 as isize)).i;
                        z__1.i = temp.r * (*b.offset(i__4 as isize)).i
                            + temp.i * (*b.offset(i__4 as isize)).r;
                        (*b.offset(i__3 as isize)).r = z__1.r;
                        (*b.offset(i__3 as isize)).i = z__1.i;
                        i__ += 1
                        /* L220: */
                    }
                }
                j += 1
            }
        } else {
            j = *n;
            while j >= 1 as libc::c_int as libc::c_long {
                if (*alpha).r != 1.0f64 || (*alpha).i != 0.0f64 {
                    i__1 = *m;
                    i__ = 1 as libc::c_int as integer;
                    while i__ <= i__1 {
                        i__2 = i__ + j * b_dim1;
                        i__3 = i__ + j * b_dim1;
                        z__1.r = (*alpha).r * (*b.offset(i__3 as isize)).r
                            - (*alpha).i * (*b.offset(i__3 as isize)).i;
                        z__1.i = (*alpha).r * (*b.offset(i__3 as isize)).i
                            + (*alpha).i * (*b.offset(i__3 as isize)).r;
                        (*b.offset(i__2 as isize)).r = z__1.r;
                        (*b.offset(i__2 as isize)).i = z__1.i;
                        i__ += 1
                        /* L240: */
                    }
                }
                i__1 = *n;
                k = j + 1 as libc::c_int as libc::c_long;
                while k <= i__1 {
                    i__2 = k + j * a_dim1;
                    if (*a.offset(i__2 as isize)).r != 0.0f64
                        || (*a.offset(i__2 as isize)).i != 0.0f64
                    {
                        i__2 = *m;
                        i__ = 1 as libc::c_int as integer;
                        while i__ <= i__2 {
                            i__3 = i__ + j * b_dim1;
                            i__4 = i__ + j * b_dim1;
                            i__5 = k + j * a_dim1;
                            i__6 = i__ + k * b_dim1;
                            z__2.r = (*a.offset(i__5 as isize)).r * (*b.offset(i__6 as isize)).r
                                - (*a.offset(i__5 as isize)).i * (*b.offset(i__6 as isize)).i;
                            z__2.i = (*a.offset(i__5 as isize)).r * (*b.offset(i__6 as isize)).i
                                + (*a.offset(i__5 as isize)).i * (*b.offset(i__6 as isize)).r;
                            z__1.r = (*b.offset(i__4 as isize)).r - z__2.r;
                            z__1.i = (*b.offset(i__4 as isize)).i - z__2.i;
                            (*b.offset(i__3 as isize)).r = z__1.r;
                            (*b.offset(i__3 as isize)).i = z__1.i;
                            i__ += 1
                            /* L280: */
                            /* L260: */
                            /* L250: */
                        }
                    }
                    k += 1
                }
                if nounit != 0 {
                    z_div_0(
                        &mut z__1,
                        &mut c_b1,
                        &mut *a.offset((j + j * a_dim1) as isize),
                    );
                    temp.r = z__1.r;
                    temp.i = z__1.i;
                    i__1 = *m;
                    i__ = 1 as libc::c_int as integer;
                    while i__ <= i__1 {
                        i__2 = i__ + j * b_dim1;
                        i__3 = i__ + j * b_dim1;
                        z__1.r = temp.r * (*b.offset(i__3 as isize)).r
                            - temp.i * (*b.offset(i__3 as isize)).i;
                        z__1.i = temp.r * (*b.offset(i__3 as isize)).i
                            + temp.i * (*b.offset(i__3 as isize)).r;
                        (*b.offset(i__2 as isize)).r = z__1.r;
                        (*b.offset(i__2 as isize)).i = z__1.i;
                        i__ += 1
                        /* L270: */
                    }
                }
                j -= 1
            }
        }
    } else if upper != 0 {
        k = *n;
        while k >= 1 as libc::c_int as libc::c_long {
            if nounit != 0 {
                if noconj != 0 {
                    z_div_0(
                        &mut z__1,
                        &mut c_b1,
                        &mut *a.offset((k + k * a_dim1) as isize),
                    );
                    temp.r = z__1.r;
                    temp.i = z__1.i
                } else {
                    d_cnjg_0(&mut z__2, &mut *a.offset((k + k * a_dim1) as isize));
                    z_div_0(&mut z__1, &mut c_b1, &mut z__2);
                    temp.r = z__1.r;
                    temp.i = z__1.i
                }
                i__1 = *m;
                i__ = 1 as libc::c_int as integer;
                while i__ <= i__1 {
                    i__2 = i__ + k * b_dim1;
                    i__3 = i__ + k * b_dim1;
                    z__1.r = temp.r * (*b.offset(i__3 as isize)).r
                        - temp.i * (*b.offset(i__3 as isize)).i;
                    z__1.i = temp.r * (*b.offset(i__3 as isize)).i
                        + temp.i * (*b.offset(i__3 as isize)).r;
                    (*b.offset(i__2 as isize)).r = z__1.r;
                    (*b.offset(i__2 as isize)).i = z__1.i;
                    i__ += 1
                    /*           Form  B := alpha*B*inv( A' ) */
                    /*           or    B := alpha*B*inv( conjg( A' ) ). */
                    /* L290: */
                }
            }
            i__1 = k - 1 as libc::c_int as libc::c_long;
            j = 1 as libc::c_int as integer;
            while j <= i__1 {
                i__2 = j + k * a_dim1;
                if (*a.offset(i__2 as isize)).r != 0.0f64 || (*a.offset(i__2 as isize)).i != 0.0f64
                {
                    if noconj != 0 {
                        i__2 = j + k * a_dim1;
                        temp.r = (*a.offset(i__2 as isize)).r;
                        temp.i = (*a.offset(i__2 as isize)).i
                    } else {
                        d_cnjg_0(&mut z__1, &mut *a.offset((j + k * a_dim1) as isize));
                        temp.r = z__1.r;
                        temp.i = z__1.i
                    }
                    i__2 = *m;
                    i__ = 1 as libc::c_int as integer;
                    while i__ <= i__2 {
                        i__3 = i__ + j * b_dim1;
                        i__4 = i__ + j * b_dim1;
                        i__5 = i__ + k * b_dim1;
                        z__2.r = temp.r * (*b.offset(i__5 as isize)).r
                            - temp.i * (*b.offset(i__5 as isize)).i;
                        z__2.i = temp.r * (*b.offset(i__5 as isize)).i
                            + temp.i * (*b.offset(i__5 as isize)).r;
                        z__1.r = (*b.offset(i__4 as isize)).r - z__2.r;
                        z__1.i = (*b.offset(i__4 as isize)).i - z__2.i;
                        (*b.offset(i__3 as isize)).r = z__1.r;
                        (*b.offset(i__3 as isize)).i = z__1.i;
                        i__ += 1
                        /* L330: */
                        /* L310: */
                        /* L300: */
                    }
                }
                j += 1
            }
            if (*alpha).r != 1.0f64 || (*alpha).i != 0.0f64 {
                i__1 = *m;
                i__ = 1 as libc::c_int as integer;
                while i__ <= i__1 {
                    i__2 = i__ + k * b_dim1;
                    i__3 = i__ + k * b_dim1;
                    z__1.r = (*alpha).r * (*b.offset(i__3 as isize)).r
                        - (*alpha).i * (*b.offset(i__3 as isize)).i;
                    z__1.i = (*alpha).r * (*b.offset(i__3 as isize)).i
                        + (*alpha).i * (*b.offset(i__3 as isize)).r;
                    (*b.offset(i__2 as isize)).r = z__1.r;
                    (*b.offset(i__2 as isize)).i = z__1.i;
                    i__ += 1
                    /* L320: */
                }
            }
            k -= 1
        }
    } else {
        i__1 = *n;
        k = 1 as libc::c_int as integer;
        while k <= i__1 {
            if nounit != 0 {
                if noconj != 0 {
                    z_div_0(
                        &mut z__1,
                        &mut c_b1,
                        &mut *a.offset((k + k * a_dim1) as isize),
                    );
                    temp.r = z__1.r;
                    temp.i = z__1.i
                } else {
                    d_cnjg_0(&mut z__2, &mut *a.offset((k + k * a_dim1) as isize));
                    z_div_0(&mut z__1, &mut c_b1, &mut z__2);
                    temp.r = z__1.r;
                    temp.i = z__1.i
                }
                i__2 = *m;
                i__ = 1 as libc::c_int as integer;
                while i__ <= i__2 {
                    i__3 = i__ + k * b_dim1;
                    i__4 = i__ + k * b_dim1;
                    z__1.r = temp.r * (*b.offset(i__4 as isize)).r
                        - temp.i * (*b.offset(i__4 as isize)).i;
                    z__1.i = temp.r * (*b.offset(i__4 as isize)).i
                        + temp.i * (*b.offset(i__4 as isize)).r;
                    (*b.offset(i__3 as isize)).r = z__1.r;
                    (*b.offset(i__3 as isize)).i = z__1.i;
                    i__ += 1
                    /* L340: */
                }
            }
            i__2 = *n;
            j = k + 1 as libc::c_int as libc::c_long;
            while j <= i__2 {
                i__3 = j + k * a_dim1;
                if (*a.offset(i__3 as isize)).r != 0.0f64 || (*a.offset(i__3 as isize)).i != 0.0f64
                {
                    if noconj != 0 {
                        i__3 = j + k * a_dim1;
                        temp.r = (*a.offset(i__3 as isize)).r;
                        temp.i = (*a.offset(i__3 as isize)).i
                    } else {
                        d_cnjg_0(&mut z__1, &mut *a.offset((j + k * a_dim1) as isize));
                        temp.r = z__1.r;
                        temp.i = z__1.i
                    }
                    i__3 = *m;
                    i__ = 1 as libc::c_int as integer;
                    while i__ <= i__3 {
                        i__4 = i__ + j * b_dim1;
                        i__5 = i__ + j * b_dim1;
                        i__6 = i__ + k * b_dim1;
                        z__2.r = temp.r * (*b.offset(i__6 as isize)).r
                            - temp.i * (*b.offset(i__6 as isize)).i;
                        z__2.i = temp.r * (*b.offset(i__6 as isize)).i
                            + temp.i * (*b.offset(i__6 as isize)).r;
                        z__1.r = (*b.offset(i__5 as isize)).r - z__2.r;
                        z__1.i = (*b.offset(i__5 as isize)).i - z__2.i;
                        (*b.offset(i__4 as isize)).r = z__1.r;
                        (*b.offset(i__4 as isize)).i = z__1.i;
                        i__ += 1
                        /* L380: */
                        /* L360: */
                        /* L350: */
                    }
                }
                j += 1
            }
            if (*alpha).r != 1.0f64 || (*alpha).i != 0.0f64 {
                i__2 = *m;
                i__ = 1 as libc::c_int as integer;
                while i__ <= i__2 {
                    i__3 = i__ + k * b_dim1;
                    i__4 = i__ + k * b_dim1;
                    z__1.r = (*alpha).r * (*b.offset(i__4 as isize)).r
                        - (*alpha).i * (*b.offset(i__4 as isize)).i;
                    z__1.i = (*alpha).r * (*b.offset(i__4 as isize)).i
                        + (*alpha).i * (*b.offset(i__4 as isize)).r;
                    (*b.offset(i__3 as isize)).r = z__1.r;
                    (*b.offset(i__3 as isize)).i = z__1.i;
                    i__ += 1
                    /* L370: */
                }
            }
            k += 1
        }
    }
    return 0 as libc::c_int;
    /*     End of ZTRSM . */
}
/* ztrsm_ */
