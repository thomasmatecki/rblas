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
/* cher2k.f -- translated by f2c (version 20061008).
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
pub unsafe extern "C" fn f2c_cher2k(mut uplo: *mut libc::c_char,
                                    mut trans: *mut libc::c_char,
                                    mut n: *mut integer, mut k: *mut integer,
                                    mut alpha: *mut complex,
                                    mut a: *mut complex,
                                    mut lda: *mut integer,
                                    mut b: *mut complex,
                                    mut ldb: *mut integer,
                                    mut beta: *mut real,
                                    mut c__: *mut complex,
                                    mut ldc: *mut integer) -> libc::c_int {
    /* System generated locals */
    let mut a_dim1: integer = 0;
    let mut a_offset: integer = 0;
    let mut b_dim1: integer = 0;
    let mut b_offset: integer = 0;
    let mut c_dim1: integer = 0;
    let mut c_offset: integer = 0;
    let mut i__1: integer = 0;
    let mut i__2: integer = 0;
    let mut i__3: integer = 0;
    let mut i__4: integer = 0;
    let mut i__5: integer = 0;
    let mut i__6: integer = 0;
    let mut i__7: integer = 0;
    let mut r__1: real = 0.;
    let mut q__1: complex = complex{r: 0., i: 0.,};
    let mut q__2: complex = complex{r: 0., i: 0.,};
    let mut q__3: complex = complex{r: 0., i: 0.,};
    let mut q__4: complex = complex{r: 0., i: 0.,};
    let mut q__5: complex = complex{r: 0., i: 0.,};
    let mut q__6: complex = complex{r: 0., i: 0.,};
    /* Builtin functions */
    extern "C" {
        #[link_name = "r_cnjg"]
        fn r_cnjg_0(_: *mut complex, _: *mut complex);
    }
    /* Local variables */
    let mut i__: integer = 0;
    let mut j: integer = 0;
    let mut l: integer = 0;
    let mut info: integer = 0;
    let mut temp1: complex = complex{r: 0., i: 0.,};
    let mut temp2: complex = complex{r: 0., i: 0.,};
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
    /*     .. Scalar Arguments .. */
/*     .. */
/*     .. Array Arguments .. */
/*     .. */
    /*  Purpose */
/*  ======= */
    /*  CHER2K  performs one of the hermitian rank 2k operations */
    /*     C := alpha*A*conjg( B' ) + conjg( alpha )*B*conjg( A' ) + beta*C, */
    /*  or */
    /*     C := alpha*conjg( A' )*B + conjg( alpha )*conjg( B' )*A + beta*C, */
    /*  where  alpha and beta  are scalars with  beta  real,  C is an  n by n */
/*  hermitian matrix and  A and B  are  n by k matrices in the first case */
/*  and  k by n  matrices in the second case. */
    /*  Arguments */
/*  ========== */
    /*  UPLO   - CHARACTER*1. */
/*           On  entry,   UPLO  specifies  whether  the  upper  or  lower */
/*           triangular  part  of the  array  C  is to be  referenced  as */
/*           follows: */
    /*              UPLO = 'U' or 'u'   Only the  upper triangular part of  C */
/*                                  is to be referenced. */
    /*              UPLO = 'L' or 'l'   Only the  lower triangular part of  C */
/*                                  is to be referenced. */
    /*           Unchanged on exit. */
    /*  TRANS  - CHARACTER*1. */
/*           On entry,  TRANS  specifies the operation to be performed as */
/*           follows: */
    /*              TRANS = 'N' or 'n'    C := alpha*A*conjg( B' )          + */
/*                                         conjg( alpha )*B*conjg( A' ) + */
/*                                         beta*C. */
    /*              TRANS = 'C' or 'c'    C := alpha*conjg( A' )*B          + */
/*                                         conjg( alpha )*conjg( B' )*A + */
/*                                         beta*C. */
    /*           Unchanged on exit. */
    /*  N      - INTEGER. */
/*           On entry,  N specifies the order of the matrix C.  N must be */
/*           at least zero. */
/*           Unchanged on exit. */
    /*  K      - INTEGER. */
/*           On entry with  TRANS = 'N' or 'n',  K  specifies  the number */
/*           of  columns  of the  matrices  A and B,  and on  entry  with */
/*           TRANS = 'C' or 'c',  K  specifies  the number of rows of the */
/*           matrices  A and B.  K must be at least zero. */
/*           Unchanged on exit. */
    /*  ALPHA  - COMPLEX         . */
/*           On entry, ALPHA specifies the scalar alpha. */
/*           Unchanged on exit. */
    /*  A      - COMPLEX          array of DIMENSION ( LDA, ka ), where ka is */
/*           k  when  TRANS = 'N' or 'n',  and is  n  otherwise. */
/*           Before entry with  TRANS = 'N' or 'n',  the  leading  n by k */
/*           part of the array  A  must contain the matrix  A,  otherwise */
/*           the leading  k by n  part of the array  A  must contain  the */
/*           matrix A. */
/*           Unchanged on exit. */
    /*  LDA    - INTEGER. */
/*           On entry, LDA specifies the first dimension of A as declared */
/*           in  the  calling  (sub)  program.   When  TRANS = 'N' or 'n' */
/*           then  LDA must be at least  max( 1, n ), otherwise  LDA must */
/*           be at least  max( 1, k ). */
/*           Unchanged on exit. */
    /*  B      - COMPLEX          array of DIMENSION ( LDB, kb ), where kb is */
/*           k  when  TRANS = 'N' or 'n',  and is  n  otherwise. */
/*           Before entry with  TRANS = 'N' or 'n',  the  leading  n by k */
/*           part of the array  B  must contain the matrix  B,  otherwise */
/*           the leading  k by n  part of the array  B  must contain  the */
/*           matrix B. */
/*           Unchanged on exit. */
    /*  LDB    - INTEGER. */
/*           On entry, LDB specifies the first dimension of B as declared */
/*           in  the  calling  (sub)  program.   When  TRANS = 'N' or 'n' */
/*           then  LDB must be at least  max( 1, n ), otherwise  LDB must */
/*           be at least  max( 1, k ). */
/*           Unchanged on exit. */
    /*  BETA   - REAL            . */
/*           On entry, BETA specifies the scalar beta. */
/*           Unchanged on exit. */
    /*  C      - COMPLEX          array of DIMENSION ( LDC, n ). */
/*           Before entry  with  UPLO = 'U' or 'u',  the leading  n by n */
/*           upper triangular part of the array C must contain the upper */
/*           triangular part  of the  hermitian matrix  and the strictly */
/*           lower triangular part of C is not referenced.  On exit, the */
/*           upper triangular part of the array  C is overwritten by the */
/*           upper triangular part of the updated matrix. */
/*           Before entry  with  UPLO = 'L' or 'l',  the leading  n by n */
/*           lower triangular part of the array C must contain the lower */
/*           triangular part  of the  hermitian matrix  and the strictly */
/*           upper triangular part of C is not referenced.  On exit, the */
/*           lower triangular part of the array  C is overwritten by the */
/*           lower triangular part of the updated matrix. */
/*           Note that the imaginary parts of the diagonal elements need */
/*           not be set,  they are assumed to be zero,  and on exit they */
/*           are set to zero. */
    /*  LDC    - INTEGER. */
/*           On entry, LDC specifies the first dimension of C as declared */
/*           in  the  calling  (sub)  program.   LDC  must  be  at  least */
/*           max( 1, n ). */
/*           Unchanged on exit. */
    /*  Level 3 Blas routine. */
    /*  -- Written on 8-February-1989. */
/*     Jack Dongarra, Argonne National Laboratory. */
/*     Iain Duff, AERE Harwell. */
/*     Jeremy Du Croz, Numerical Algorithms Group Ltd. */
/*     Sven Hammarling, Numerical Algorithms Group Ltd. */
    /*  -- Modified 8-Nov-93 to set C(J,J) to REAL( C(J,J) ) when BETA = 1. */
/*     Ed Anderson, Cray Research Inc. */
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
    c_dim1 = *ldc;
    c_offset = 1 as libc::c_int as libc::c_long + c_dim1;
    c__ = c__.offset(-(c_offset as isize));
    /* Function Body */
    if lsame__0(trans,
                b"N\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char) != 0 {
        nrowa = *n
    } else { nrowa = *k }
    upper =
        lsame__0(uplo,
                 b"U\x00" as *const u8 as *const libc::c_char as
                     *mut libc::c_char);
    info = 0 as libc::c_int as integer;
    if upper == 0 &&
           lsame__0(uplo,
                    b"L\x00" as *const u8 as *const libc::c_char as
                        *mut libc::c_char) == 0 {
        info = 1 as libc::c_int as integer
    } else if lsame__0(trans,
                       b"N\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char) == 0 &&
                  lsame__0(trans,
                           b"C\x00" as *const u8 as *const libc::c_char as
                               *mut libc::c_char) == 0 {
        info = 2 as libc::c_int as integer
    } else if *n < 0 as libc::c_int as libc::c_long {
        info = 3 as libc::c_int as integer
    } else if *k < 0 as libc::c_int as libc::c_long {
        info = 4 as libc::c_int as integer
    } else if *lda <
                  (if 1 as libc::c_int as libc::c_long >= nrowa {
                       1 as libc::c_int as libc::c_long
                   } else { nrowa }) {
        info = 7 as libc::c_int as integer
    } else if *ldb <
                  (if 1 as libc::c_int as libc::c_long >= nrowa {
                       1 as libc::c_int as libc::c_long
                   } else { nrowa }) {
        info = 9 as libc::c_int as integer
    } else if *ldc <
                  (if 1 as libc::c_int as libc::c_long >= *n {
                       1 as libc::c_int as libc::c_long
                   } else { *n }) {
        info = 12 as libc::c_int as integer
    }
    if info != 0 as libc::c_int as libc::c_long {
        xerbla__0(b"CHER2K\x00" as *const u8 as *const libc::c_char as
                      *mut libc::c_char, &mut info);
        return 0 as libc::c_int
    }
    /*     Quick return if possible. */
    if *n == 0 as libc::c_int as libc::c_long ||
           ((*alpha).r == 0.0f32 && (*alpha).i == 0.0f32 ||
                *k == 0 as libc::c_int as libc::c_long) && *beta == 1.0f32 {
        return 0 as libc::c_int
    }
    /*     And when  alpha.eq.zero. */
    if (*alpha).r == 0.0f32 && (*alpha).i == 0.0f32 {
        if upper != 0 {
            if *beta == 0.0f32 {
                i__1 = *n;
                j = 1 as libc::c_int as integer;
                while j <= i__1 {
                    i__2 = j;
                    i__ = 1 as libc::c_int as integer;
                    while i__ <= i__2 {
                        i__3 = i__ + j * c_dim1;
                        (*c__.offset(i__3 as isize)).r = 0.0f32;
                        (*c__.offset(i__3 as isize)).i = 0.0f32;
                        i__ += 1
                        /* L20: */
                        /* L10: */
                    }
                    j += 1
                }
            } else {
                i__1 = *n;
                j = 1 as libc::c_int as integer;
                while j <= i__1 {
                    i__2 = j - 1 as libc::c_int as libc::c_long;
                    i__ = 1 as libc::c_int as integer;
                    while i__ <= i__2 {
                        i__3 = i__ + j * c_dim1;
                        i__4 = i__ + j * c_dim1;
                        q__1.r = *beta * (*c__.offset(i__4 as isize)).r;
                        q__1.i = *beta * (*c__.offset(i__4 as isize)).i;
                        (*c__.offset(i__3 as isize)).r = q__1.r;
                        (*c__.offset(i__3 as isize)).i = q__1.i;
                        i__ += 1
                        /* L40: */
                        /* L30: */
                    }
                    i__2 = j + j * c_dim1;
                    i__3 = j + j * c_dim1;
                    r__1 = *beta * (*c__.offset(i__3 as isize)).r;
                    (*c__.offset(i__2 as isize)).r = r__1;
                    (*c__.offset(i__2 as isize)).i = 0.0f32;
                    j += 1
                }
            }
        } else if *beta == 0.0f32 {
            i__1 = *n;
            j = 1 as libc::c_int as integer;
            while j <= i__1 {
                i__2 = *n;
                i__ = j;
                while i__ <= i__2 {
                    i__3 = i__ + j * c_dim1;
                    (*c__.offset(i__3 as isize)).r = 0.0f32;
                    (*c__.offset(i__3 as isize)).i = 0.0f32;
                    i__ += 1
                    /* L60: */
                    /* L50: */
                }
                j += 1
            }
        } else {
            i__1 = *n;
            j = 1 as libc::c_int as integer;
            while j <= i__1 {
                i__2 = j + j * c_dim1;
                i__3 = j + j * c_dim1;
                r__1 = *beta * (*c__.offset(i__3 as isize)).r;
                (*c__.offset(i__2 as isize)).r = r__1;
                (*c__.offset(i__2 as isize)).i = 0.0f32;
                i__2 = *n;
                i__ = j + 1 as libc::c_int as libc::c_long;
                while i__ <= i__2 {
                    i__3 = i__ + j * c_dim1;
                    i__4 = i__ + j * c_dim1;
                    q__1.r = *beta * (*c__.offset(i__4 as isize)).r;
                    q__1.i = *beta * (*c__.offset(i__4 as isize)).i;
                    (*c__.offset(i__3 as isize)).r = q__1.r;
                    (*c__.offset(i__3 as isize)).i = q__1.i;
                    i__ += 1
                    /* L80: */
                    /* L70: */
                }
                j += 1
            }
        }
        return 0 as libc::c_int
    }
    /*     Start the operations. */
    if lsame__0(trans,
                b"N\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char) != 0 {
        /*        Form  C := alpha*A*conjg( B' ) + conjg( alpha )*B*conjg( A' ) + */
/*                   C. */
        if upper != 0 {
            i__1 = *n;
            j = 1 as libc::c_int as integer;
            while j <= i__1 {
                if *beta == 0.0f32 {
                    i__2 = j;
                    i__ = 1 as libc::c_int as integer;
                    while i__ <= i__2 {
                        i__3 = i__ + j * c_dim1;
                        (*c__.offset(i__3 as isize)).r = 0.0f32;
                        (*c__.offset(i__3 as isize)).i = 0.0f32;
                        i__ += 1
                        /* L90: */
                    }
                } else if *beta != 1.0f32 {
                    i__2 = j - 1 as libc::c_int as libc::c_long;
                    i__ = 1 as libc::c_int as integer;
                    while i__ <= i__2 {
                        i__3 = i__ + j * c_dim1;
                        i__4 = i__ + j * c_dim1;
                        q__1.r = *beta * (*c__.offset(i__4 as isize)).r;
                        q__1.i = *beta * (*c__.offset(i__4 as isize)).i;
                        (*c__.offset(i__3 as isize)).r = q__1.r;
                        (*c__.offset(i__3 as isize)).i = q__1.i;
                        i__ += 1
                        /* L100: */
                    }
                    i__2 = j + j * c_dim1;
                    i__3 = j + j * c_dim1;
                    r__1 = *beta * (*c__.offset(i__3 as isize)).r;
                    (*c__.offset(i__2 as isize)).r = r__1;
                    (*c__.offset(i__2 as isize)).i = 0.0f32
                } else {
                    i__2 = j + j * c_dim1;
                    i__3 = j + j * c_dim1;
                    r__1 = (*c__.offset(i__3 as isize)).r;
                    (*c__.offset(i__2 as isize)).r = r__1;
                    (*c__.offset(i__2 as isize)).i = 0.0f32
                }
                i__2 = *k;
                l = 1 as libc::c_int as integer;
                while l <= i__2 {
                    i__3 = j + l * a_dim1;
                    i__4 = j + l * b_dim1;
                    if (*a.offset(i__3 as isize)).r != 0.0f32 ||
                           (*a.offset(i__3 as isize)).i != 0.0f32 ||
                           ((*b.offset(i__4 as isize)).r != 0.0f32 ||
                                (*b.offset(i__4 as isize)).i != 0.0f32) {
                        r_cnjg_0(&mut q__2,
                                 &mut *b.offset((j + l * b_dim1) as isize));
                        q__1.r = (*alpha).r * q__2.r - (*alpha).i * q__2.i;
                        q__1.i = (*alpha).r * q__2.i + (*alpha).i * q__2.r;
                        temp1.r = q__1.r;
                        temp1.i = q__1.i;
                        i__3 = j + l * a_dim1;
                        q__2.r =
                            (*alpha).r * (*a.offset(i__3 as isize)).r -
                                (*alpha).i * (*a.offset(i__3 as isize)).i;
                        q__2.i =
                            (*alpha).r * (*a.offset(i__3 as isize)).i +
                                (*alpha).i * (*a.offset(i__3 as isize)).r;
                        r_cnjg_0(&mut q__1, &mut q__2);
                        temp2.r = q__1.r;
                        temp2.i = q__1.i;
                        i__3 = j - 1 as libc::c_int as libc::c_long;
                        i__ = 1 as libc::c_int as integer;
                        while i__ <= i__3 {
                            i__4 = i__ + j * c_dim1;
                            i__5 = i__ + j * c_dim1;
                            i__6 = i__ + l * a_dim1;
                            q__3.r =
                                (*a.offset(i__6 as isize)).r * temp1.r -
                                    (*a.offset(i__6 as isize)).i * temp1.i;
                            q__3.i =
                                (*a.offset(i__6 as isize)).r * temp1.i +
                                    (*a.offset(i__6 as isize)).i * temp1.r;
                            q__2.r = (*c__.offset(i__5 as isize)).r + q__3.r;
                            q__2.i = (*c__.offset(i__5 as isize)).i + q__3.i;
                            i__7 = i__ + l * b_dim1;
                            q__4.r =
                                (*b.offset(i__7 as isize)).r * temp2.r -
                                    (*b.offset(i__7 as isize)).i * temp2.i;
                            q__4.i =
                                (*b.offset(i__7 as isize)).r * temp2.i +
                                    (*b.offset(i__7 as isize)).i * temp2.r;
                            q__1.r = q__2.r + q__4.r;
                            q__1.i = q__2.i + q__4.i;
                            (*c__.offset(i__4 as isize)).r = q__1.r;
                            (*c__.offset(i__4 as isize)).i = q__1.i;
                            i__ += 1
                            /* L130: */
                            /* L120: */
                            /* L110: */
                        }
                        i__3 = j + j * c_dim1;
                        i__4 = j + j * c_dim1;
                        i__5 = j + l * a_dim1;
                        q__2.r =
                            (*a.offset(i__5 as isize)).r * temp1.r -
                                (*a.offset(i__5 as isize)).i * temp1.i;
                        q__2.i =
                            (*a.offset(i__5 as isize)).r * temp1.i +
                                (*a.offset(i__5 as isize)).i * temp1.r;
                        i__6 = j + l * b_dim1;
                        q__3.r =
                            (*b.offset(i__6 as isize)).r * temp2.r -
                                (*b.offset(i__6 as isize)).i * temp2.i;
                        q__3.i =
                            (*b.offset(i__6 as isize)).r * temp2.i +
                                (*b.offset(i__6 as isize)).i * temp2.r;
                        q__1.r = q__2.r + q__3.r;
                        q__1.i = q__2.i + q__3.i;
                        r__1 = (*c__.offset(i__4 as isize)).r + q__1.r;
                        (*c__.offset(i__3 as isize)).r = r__1;
                        (*c__.offset(i__3 as isize)).i = 0.0f32
                    }
                    l += 1
                }
                j += 1
            }
        } else {
            i__1 = *n;
            j = 1 as libc::c_int as integer;
            while j <= i__1 {
                if *beta == 0.0f32 {
                    i__2 = *n;
                    i__ = j;
                    while i__ <= i__2 {
                        i__3 = i__ + j * c_dim1;
                        (*c__.offset(i__3 as isize)).r = 0.0f32;
                        (*c__.offset(i__3 as isize)).i = 0.0f32;
                        i__ += 1
                        /* L140: */
                    }
                } else if *beta != 1.0f32 {
                    i__2 = *n;
                    i__ = j + 1 as libc::c_int as libc::c_long;
                    while i__ <= i__2 {
                        i__3 = i__ + j * c_dim1;
                        i__4 = i__ + j * c_dim1;
                        q__1.r = *beta * (*c__.offset(i__4 as isize)).r;
                        q__1.i = *beta * (*c__.offset(i__4 as isize)).i;
                        (*c__.offset(i__3 as isize)).r = q__1.r;
                        (*c__.offset(i__3 as isize)).i = q__1.i;
                        i__ += 1
                        /* L150: */
                    }
                    i__2 = j + j * c_dim1;
                    i__3 = j + j * c_dim1;
                    r__1 = *beta * (*c__.offset(i__3 as isize)).r;
                    (*c__.offset(i__2 as isize)).r = r__1;
                    (*c__.offset(i__2 as isize)).i = 0.0f32
                } else {
                    i__2 = j + j * c_dim1;
                    i__3 = j + j * c_dim1;
                    r__1 = (*c__.offset(i__3 as isize)).r;
                    (*c__.offset(i__2 as isize)).r = r__1;
                    (*c__.offset(i__2 as isize)).i = 0.0f32
                }
                i__2 = *k;
                l = 1 as libc::c_int as integer;
                while l <= i__2 {
                    i__3 = j + l * a_dim1;
                    i__4 = j + l * b_dim1;
                    if (*a.offset(i__3 as isize)).r != 0.0f32 ||
                           (*a.offset(i__3 as isize)).i != 0.0f32 ||
                           ((*b.offset(i__4 as isize)).r != 0.0f32 ||
                                (*b.offset(i__4 as isize)).i != 0.0f32) {
                        r_cnjg_0(&mut q__2,
                                 &mut *b.offset((j + l * b_dim1) as isize));
                        q__1.r = (*alpha).r * q__2.r - (*alpha).i * q__2.i;
                        q__1.i = (*alpha).r * q__2.i + (*alpha).i * q__2.r;
                        temp1.r = q__1.r;
                        temp1.i = q__1.i;
                        i__3 = j + l * a_dim1;
                        q__2.r =
                            (*alpha).r * (*a.offset(i__3 as isize)).r -
                                (*alpha).i * (*a.offset(i__3 as isize)).i;
                        q__2.i =
                            (*alpha).r * (*a.offset(i__3 as isize)).i +
                                (*alpha).i * (*a.offset(i__3 as isize)).r;
                        r_cnjg_0(&mut q__1, &mut q__2);
                        temp2.r = q__1.r;
                        temp2.i = q__1.i;
                        i__3 = *n;
                        i__ = j + 1 as libc::c_int as libc::c_long;
                        while i__ <= i__3 {
                            i__4 = i__ + j * c_dim1;
                            i__5 = i__ + j * c_dim1;
                            i__6 = i__ + l * a_dim1;
                            q__3.r =
                                (*a.offset(i__6 as isize)).r * temp1.r -
                                    (*a.offset(i__6 as isize)).i * temp1.i;
                            q__3.i =
                                (*a.offset(i__6 as isize)).r * temp1.i +
                                    (*a.offset(i__6 as isize)).i * temp1.r;
                            q__2.r = (*c__.offset(i__5 as isize)).r + q__3.r;
                            q__2.i = (*c__.offset(i__5 as isize)).i + q__3.i;
                            i__7 = i__ + l * b_dim1;
                            q__4.r =
                                (*b.offset(i__7 as isize)).r * temp2.r -
                                    (*b.offset(i__7 as isize)).i * temp2.i;
                            q__4.i =
                                (*b.offset(i__7 as isize)).r * temp2.i +
                                    (*b.offset(i__7 as isize)).i * temp2.r;
                            q__1.r = q__2.r + q__4.r;
                            q__1.i = q__2.i + q__4.i;
                            (*c__.offset(i__4 as isize)).r = q__1.r;
                            (*c__.offset(i__4 as isize)).i = q__1.i;
                            i__ += 1
                            /* L180: */
                            /* L170: */
                            /* L160: */
                        }
                        i__3 = j + j * c_dim1;
                        i__4 = j + j * c_dim1;
                        i__5 = j + l * a_dim1;
                        q__2.r =
                            (*a.offset(i__5 as isize)).r * temp1.r -
                                (*a.offset(i__5 as isize)).i * temp1.i;
                        q__2.i =
                            (*a.offset(i__5 as isize)).r * temp1.i +
                                (*a.offset(i__5 as isize)).i * temp1.r;
                        i__6 = j + l * b_dim1;
                        q__3.r =
                            (*b.offset(i__6 as isize)).r * temp2.r -
                                (*b.offset(i__6 as isize)).i * temp2.i;
                        q__3.i =
                            (*b.offset(i__6 as isize)).r * temp2.i +
                                (*b.offset(i__6 as isize)).i * temp2.r;
                        q__1.r = q__2.r + q__3.r;
                        q__1.i = q__2.i + q__3.i;
                        r__1 = (*c__.offset(i__4 as isize)).r + q__1.r;
                        (*c__.offset(i__3 as isize)).r = r__1;
                        (*c__.offset(i__3 as isize)).i = 0.0f32
                    }
                    l += 1
                }
                j += 1
            }
        }
    } else if upper != 0 {
        i__1 = *n;
        j = 1 as libc::c_int as integer;
        while j <= i__1 {
            i__2 = j;
            i__ = 1 as libc::c_int as integer;
            while i__ <= i__2 {
                temp1.r = 0.0f32;
                temp1.i = 0.0f32;
                temp2.r = 0.0f32;
                temp2.i = 0.0f32;
                i__3 = *k;
                l = 1 as libc::c_int as integer;
                while l <= i__3 {
                    r_cnjg_0(&mut q__3,
                             &mut *a.offset((l + i__ * a_dim1) as isize));
                    i__4 = l + j * b_dim1;
                    q__2.r =
                        q__3.r * (*b.offset(i__4 as isize)).r -
                            q__3.i * (*b.offset(i__4 as isize)).i;
                    q__2.i =
                        q__3.r * (*b.offset(i__4 as isize)).i +
                            q__3.i * (*b.offset(i__4 as isize)).r;
                    q__1.r = temp1.r + q__2.r;
                    q__1.i = temp1.i + q__2.i;
                    temp1.r = q__1.r;
                    temp1.i = q__1.i;
                    r_cnjg_0(&mut q__3,
                             &mut *b.offset((l + i__ * b_dim1) as isize));
                    i__4 = l + j * a_dim1;
                    q__2.r =
                        q__3.r * (*a.offset(i__4 as isize)).r -
                            q__3.i * (*a.offset(i__4 as isize)).i;
                    q__2.i =
                        q__3.r * (*a.offset(i__4 as isize)).i +
                            q__3.i * (*a.offset(i__4 as isize)).r;
                    q__1.r = temp2.r + q__2.r;
                    q__1.i = temp2.i + q__2.i;
                    temp2.r = q__1.r;
                    temp2.i = q__1.i;
                    l += 1
                    /*        Form  C := alpha*conjg( A' )*B + conjg( alpha )*conjg( B' )*A + */
/*                   C. */
                    /* L210: */
                    /* L200: */
                    /* L190: */
                }
                if i__ == j {
                    if *beta == 0.0f32 {
                        i__3 = j + j * c_dim1;
                        q__2.r = (*alpha).r * temp1.r - (*alpha).i * temp1.i;
                        q__2.i = (*alpha).r * temp1.i + (*alpha).i * temp1.r;
                        r_cnjg_0(&mut q__4, alpha);
                        q__3.r = q__4.r * temp2.r - q__4.i * temp2.i;
                        q__3.i = q__4.r * temp2.i + q__4.i * temp2.r;
                        q__1.r = q__2.r + q__3.r;
                        q__1.i = q__2.i + q__3.i;
                        r__1 = q__1.r;
                        (*c__.offset(i__3 as isize)).r = r__1;
                        (*c__.offset(i__3 as isize)).i = 0.0f32
                    } else {
                        i__3 = j + j * c_dim1;
                        i__4 = j + j * c_dim1;
                        q__2.r = (*alpha).r * temp1.r - (*alpha).i * temp1.i;
                        q__2.i = (*alpha).r * temp1.i + (*alpha).i * temp1.r;
                        r_cnjg_0(&mut q__4, alpha);
                        q__3.r = q__4.r * temp2.r - q__4.i * temp2.i;
                        q__3.i = q__4.r * temp2.i + q__4.i * temp2.r;
                        q__1.r = q__2.r + q__3.r;
                        q__1.i = q__2.i + q__3.i;
                        r__1 =
                            *beta * (*c__.offset(i__4 as isize)).r + q__1.r;
                        (*c__.offset(i__3 as isize)).r = r__1;
                        (*c__.offset(i__3 as isize)).i = 0.0f32
                    }
                } else if *beta == 0.0f32 {
                    i__3 = i__ + j * c_dim1;
                    q__2.r = (*alpha).r * temp1.r - (*alpha).i * temp1.i;
                    q__2.i = (*alpha).r * temp1.i + (*alpha).i * temp1.r;
                    r_cnjg_0(&mut q__4, alpha);
                    q__3.r = q__4.r * temp2.r - q__4.i * temp2.i;
                    q__3.i = q__4.r * temp2.i + q__4.i * temp2.r;
                    q__1.r = q__2.r + q__3.r;
                    q__1.i = q__2.i + q__3.i;
                    (*c__.offset(i__3 as isize)).r = q__1.r;
                    (*c__.offset(i__3 as isize)).i = q__1.i
                } else {
                    i__3 = i__ + j * c_dim1;
                    i__4 = i__ + j * c_dim1;
                    q__3.r = *beta * (*c__.offset(i__4 as isize)).r;
                    q__3.i = *beta * (*c__.offset(i__4 as isize)).i;
                    q__4.r = (*alpha).r * temp1.r - (*alpha).i * temp1.i;
                    q__4.i = (*alpha).r * temp1.i + (*alpha).i * temp1.r;
                    q__2.r = q__3.r + q__4.r;
                    q__2.i = q__3.i + q__4.i;
                    r_cnjg_0(&mut q__6, alpha);
                    q__5.r = q__6.r * temp2.r - q__6.i * temp2.i;
                    q__5.i = q__6.r * temp2.i + q__6.i * temp2.r;
                    q__1.r = q__2.r + q__5.r;
                    q__1.i = q__2.i + q__5.i;
                    (*c__.offset(i__3 as isize)).r = q__1.r;
                    (*c__.offset(i__3 as isize)).i = q__1.i
                }
                i__ += 1
            }
            j += 1
        }
    } else {
        i__1 = *n;
        j = 1 as libc::c_int as integer;
        while j <= i__1 {
            i__2 = *n;
            i__ = j;
            while i__ <= i__2 {
                temp1.r = 0.0f32;
                temp1.i = 0.0f32;
                temp2.r = 0.0f32;
                temp2.i = 0.0f32;
                i__3 = *k;
                l = 1 as libc::c_int as integer;
                while l <= i__3 {
                    r_cnjg_0(&mut q__3,
                             &mut *a.offset((l + i__ * a_dim1) as isize));
                    i__4 = l + j * b_dim1;
                    q__2.r =
                        q__3.r * (*b.offset(i__4 as isize)).r -
                            q__3.i * (*b.offset(i__4 as isize)).i;
                    q__2.i =
                        q__3.r * (*b.offset(i__4 as isize)).i +
                            q__3.i * (*b.offset(i__4 as isize)).r;
                    q__1.r = temp1.r + q__2.r;
                    q__1.i = temp1.i + q__2.i;
                    temp1.r = q__1.r;
                    temp1.i = q__1.i;
                    r_cnjg_0(&mut q__3,
                             &mut *b.offset((l + i__ * b_dim1) as isize));
                    i__4 = l + j * a_dim1;
                    q__2.r =
                        q__3.r * (*a.offset(i__4 as isize)).r -
                            q__3.i * (*a.offset(i__4 as isize)).i;
                    q__2.i =
                        q__3.r * (*a.offset(i__4 as isize)).i +
                            q__3.i * (*a.offset(i__4 as isize)).r;
                    q__1.r = temp2.r + q__2.r;
                    q__1.i = temp2.i + q__2.i;
                    temp2.r = q__1.r;
                    temp2.i = q__1.i;
                    l += 1
                    /* L240: */
                    /* L230: */
                    /* L220: */
                }
                if i__ == j {
                    if *beta == 0.0f32 {
                        i__3 = j + j * c_dim1;
                        q__2.r = (*alpha).r * temp1.r - (*alpha).i * temp1.i;
                        q__2.i = (*alpha).r * temp1.i + (*alpha).i * temp1.r;
                        r_cnjg_0(&mut q__4, alpha);
                        q__3.r = q__4.r * temp2.r - q__4.i * temp2.i;
                        q__3.i = q__4.r * temp2.i + q__4.i * temp2.r;
                        q__1.r = q__2.r + q__3.r;
                        q__1.i = q__2.i + q__3.i;
                        r__1 = q__1.r;
                        (*c__.offset(i__3 as isize)).r = r__1;
                        (*c__.offset(i__3 as isize)).i = 0.0f32
                    } else {
                        i__3 = j + j * c_dim1;
                        i__4 = j + j * c_dim1;
                        q__2.r = (*alpha).r * temp1.r - (*alpha).i * temp1.i;
                        q__2.i = (*alpha).r * temp1.i + (*alpha).i * temp1.r;
                        r_cnjg_0(&mut q__4, alpha);
                        q__3.r = q__4.r * temp2.r - q__4.i * temp2.i;
                        q__3.i = q__4.r * temp2.i + q__4.i * temp2.r;
                        q__1.r = q__2.r + q__3.r;
                        q__1.i = q__2.i + q__3.i;
                        r__1 =
                            *beta * (*c__.offset(i__4 as isize)).r + q__1.r;
                        (*c__.offset(i__3 as isize)).r = r__1;
                        (*c__.offset(i__3 as isize)).i = 0.0f32
                    }
                } else if *beta == 0.0f32 {
                    i__3 = i__ + j * c_dim1;
                    q__2.r = (*alpha).r * temp1.r - (*alpha).i * temp1.i;
                    q__2.i = (*alpha).r * temp1.i + (*alpha).i * temp1.r;
                    r_cnjg_0(&mut q__4, alpha);
                    q__3.r = q__4.r * temp2.r - q__4.i * temp2.i;
                    q__3.i = q__4.r * temp2.i + q__4.i * temp2.r;
                    q__1.r = q__2.r + q__3.r;
                    q__1.i = q__2.i + q__3.i;
                    (*c__.offset(i__3 as isize)).r = q__1.r;
                    (*c__.offset(i__3 as isize)).i = q__1.i
                } else {
                    i__3 = i__ + j * c_dim1;
                    i__4 = i__ + j * c_dim1;
                    q__3.r = *beta * (*c__.offset(i__4 as isize)).r;
                    q__3.i = *beta * (*c__.offset(i__4 as isize)).i;
                    q__4.r = (*alpha).r * temp1.r - (*alpha).i * temp1.i;
                    q__4.i = (*alpha).r * temp1.i + (*alpha).i * temp1.r;
                    q__2.r = q__3.r + q__4.r;
                    q__2.i = q__3.i + q__4.i;
                    r_cnjg_0(&mut q__6, alpha);
                    q__5.r = q__6.r * temp2.r - q__6.i * temp2.i;
                    q__5.i = q__6.r * temp2.i + q__6.i * temp2.r;
                    q__1.r = q__2.r + q__5.r;
                    q__1.i = q__2.i + q__5.i;
                    (*c__.offset(i__3 as isize)).r = q__1.r;
                    (*c__.offset(i__3 as isize)).i = q__1.i
                }
                i__ += 1
            }
            j += 1
        }
    }
    return 0 as libc::c_int;
    /*     End of CHER2K. */
}
/* cher2k_ */
