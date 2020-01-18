use ::libc;
/* f2c.h  --  Standard Fortran to C header file */
/* *  barf  [ba:rf]  2.  "He suggested using FORTRAN, and everybody barfed."

	- From The Shogakukan DICTIONARY OF NEW ENGLISH (Second edition) */
pub type integer = libc::c_long;
pub type real = libc::c_float;
pub type logical = libc::c_long;
/* ssymm.f -- translated by f2c (version 20061008).
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
pub unsafe extern "C" fn f2c_ssymm(mut side: *mut libc::c_char,
                                   mut uplo: *mut libc::c_char,
                                   mut m: *mut integer, mut n: *mut integer,
                                   mut alpha: *mut real, mut a: *mut real,
                                   mut lda: *mut integer, mut b: *mut real,
                                   mut ldb: *mut integer, mut beta: *mut real,
                                   mut c__: *mut real, mut ldc: *mut integer)
 -> libc::c_int {
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
    /* Local variables */
    let mut i__: integer = 0;
    let mut j: integer = 0;
    let mut k: integer = 0;
    let mut info: integer = 0;
    let mut temp1: real = 0.;
    let mut temp2: real = 0.;
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
    /*  SSYMM  performs one of the matrix-matrix operations */
    /*     C := alpha*A*B + beta*C, */
    /*  or */
    /*     C := alpha*B*A + beta*C, */
    /*  where alpha and beta are scalars,  A is a symmetric matrix and  B and */
/*  C are  m by n matrices. */
    /*  Arguments */
/*  ========== */
    /*  SIDE   - CHARACTER*1. */
/*           On entry,  SIDE  specifies whether  the  symmetric matrix  A */
/*           appears on the  left or right  in the  operation as follows: */
    /*              SIDE = 'L' or 'l'   C := alpha*A*B + beta*C, */
    /*              SIDE = 'R' or 'r'   C := alpha*B*A + beta*C, */
    /*           Unchanged on exit. */
    /*  UPLO   - CHARACTER*1. */
/*           On  entry,   UPLO  specifies  whether  the  upper  or  lower */
/*           triangular  part  of  the  symmetric  matrix   A  is  to  be */
/*           referenced as follows: */
    /*              UPLO = 'U' or 'u'   Only the upper triangular part of the */
/*                                  symmetric matrix is to be referenced. */
    /*              UPLO = 'L' or 'l'   Only the lower triangular part of the */
/*                                  symmetric matrix is to be referenced. */
    /*           Unchanged on exit. */
    /*  M      - INTEGER. */
/*           On entry,  M  specifies the number of rows of the matrix  C. */
/*           M  must be at least zero. */
/*           Unchanged on exit. */
    /*  N      - INTEGER. */
/*           On entry, N specifies the number of columns of the matrix C. */
/*           N  must be at least zero. */
/*           Unchanged on exit. */
    /*  ALPHA  - REAL            . */
/*           On entry, ALPHA specifies the scalar alpha. */
/*           Unchanged on exit. */
    /*  A      - REAL             array of DIMENSION ( LDA, ka ), where ka is */
/*           m  when  SIDE = 'L' or 'l'  and is  n otherwise. */
/*           Before entry  with  SIDE = 'L' or 'l',  the  m by m  part of */
/*           the array  A  must contain the  symmetric matrix,  such that */
/*           when  UPLO = 'U' or 'u', the leading m by m upper triangular */
/*           part of the array  A  must contain the upper triangular part */
/*           of the  symmetric matrix and the  strictly  lower triangular */
/*           part of  A  is not referenced,  and when  UPLO = 'L' or 'l', */
/*           the leading  m by m  lower triangular part  of the  array  A */
/*           must  contain  the  lower triangular part  of the  symmetric */
/*           matrix and the  strictly upper triangular part of  A  is not */
/*           referenced. */
/*           Before entry  with  SIDE = 'R' or 'r',  the  n by n  part of */
/*           the array  A  must contain the  symmetric matrix,  such that */
/*           when  UPLO = 'U' or 'u', the leading n by n upper triangular */
/*           part of the array  A  must contain the upper triangular part */
/*           of the  symmetric matrix and the  strictly  lower triangular */
/*           part of  A  is not referenced,  and when  UPLO = 'L' or 'l', */
/*           the leading  n by n  lower triangular part  of the  array  A */
/*           must  contain  the  lower triangular part  of the  symmetric */
/*           matrix and the  strictly upper triangular part of  A  is not */
/*           referenced. */
/*           Unchanged on exit. */
    /*  LDA    - INTEGER. */
/*           On entry, LDA specifies the first dimension of A as declared */
/*           in the calling (sub) program.  When  SIDE = 'L' or 'l'  then */
/*           LDA must be at least  max( 1, m ), otherwise  LDA must be at */
/*           least  max( 1, n ). */
/*           Unchanged on exit. */
    /*  B      - REAL             array of DIMENSION ( LDB, n ). */
/*           Before entry, the leading  m by n part of the array  B  must */
/*           contain the matrix B. */
/*           Unchanged on exit. */
    /*  LDB    - INTEGER. */
/*           On entry, LDB specifies the first dimension of B as declared */
/*           in  the  calling  (sub)  program.   LDB  must  be  at  least */
/*           max( 1, m ). */
/*           Unchanged on exit. */
    /*  BETA   - REAL            . */
/*           On entry,  BETA  specifies the scalar  beta.  When  BETA  is */
/*           supplied as zero then C need not be set on input. */
/*           Unchanged on exit. */
    /*  C      - REAL             array of DIMENSION ( LDC, n ). */
/*           Before entry, the leading  m by n  part of the array  C must */
/*           contain the matrix  C,  except when  beta  is zero, in which */
/*           case C need not be set on entry. */
/*           On exit, the array  C  is overwritten by the  m by n updated */
/*           matrix. */
    /*  LDC    - INTEGER. */
/*           On entry, LDC specifies the first dimension of C as declared */
/*           in  the  calling  (sub)  program.   LDC  must  be  at  least */
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
    /*     Set NROWA as the number of rows of A. */
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
    if lsame__0(side,
                b"L\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char) != 0 {
        nrowa = *m
    } else { nrowa = *n }
    upper =
        lsame__0(uplo,
                 b"U\x00" as *const u8 as *const libc::c_char as
                     *mut libc::c_char);
    /*     Test the input parameters. */
    info = 0 as libc::c_int as integer;
    if lsame__0(side,
                b"L\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char) == 0 &&
           lsame__0(side,
                    b"R\x00" as *const u8 as *const libc::c_char as
                        *mut libc::c_char) == 0 {
        info = 1 as libc::c_int as integer
    } else if upper == 0 &&
                  lsame__0(uplo,
                           b"L\x00" as *const u8 as *const libc::c_char as
                               *mut libc::c_char) == 0 {
        info = 2 as libc::c_int as integer
    } else if *m < 0 as libc::c_int as libc::c_long {
        info = 3 as libc::c_int as integer
    } else if *n < 0 as libc::c_int as libc::c_long {
        info = 4 as libc::c_int as integer
    } else if *lda <
                  (if 1 as libc::c_int as libc::c_long >= nrowa {
                       1 as libc::c_int as libc::c_long
                   } else { nrowa }) {
        info = 7 as libc::c_int as integer
    } else if *ldb <
                  (if 1 as libc::c_int as libc::c_long >= *m {
                       1 as libc::c_int as libc::c_long
                   } else { *m }) {
        info = 9 as libc::c_int as integer
    } else if *ldc <
                  (if 1 as libc::c_int as libc::c_long >= *m {
                       1 as libc::c_int as libc::c_long
                   } else { *m }) {
        info = 12 as libc::c_int as integer
    }
    if info != 0 as libc::c_int as libc::c_long {
        xerbla__0(b"SSYMM \x00" as *const u8 as *const libc::c_char as
                      *mut libc::c_char, &mut info);
        return 0 as libc::c_int
    }
    /*     Quick return if possible. */
    if *m == 0 as libc::c_int as libc::c_long ||
           *n == 0 as libc::c_int as libc::c_long ||
           *alpha == 0.0f32 && *beta == 1.0f32 {
        return 0 as libc::c_int
    }
    /*     And when  alpha.eq.zero. */
    if *alpha == 0.0f32 {
        if *beta == 0.0f32 {
            i__1 = *n;
            j = 1 as libc::c_int as integer;
            while j <= i__1 {
                i__2 = *m;
                i__ = 1 as libc::c_int as integer;
                while i__ <= i__2 {
                    *c__.offset((i__ + j * c_dim1) as isize) = 0.0f32;
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
                i__2 = *m;
                i__ = 1 as libc::c_int as integer;
                while i__ <= i__2 {
                    *c__.offset((i__ + j * c_dim1) as isize) =
                        *beta * *c__.offset((i__ + j * c_dim1) as isize);
                    i__ += 1
                    /* L40: */
                    /* L30: */
                }
                j += 1
            }
        }
        return 0 as libc::c_int
    }
    /*     Start the operations. */
    if lsame__0(side,
                b"L\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char) != 0 {
        /*        Form  C := alpha*A*B + beta*C. */
        if upper != 0 {
            i__1 = *n;
            j = 1 as libc::c_int as integer;
            while j <= i__1 {
                i__2 = *m;
                i__ = 1 as libc::c_int as integer;
                while i__ <= i__2 {
                    temp1 = *alpha * *b.offset((i__ + j * b_dim1) as isize);
                    temp2 = 0.0f32;
                    i__3 = i__ - 1 as libc::c_int as libc::c_long;
                    k = 1 as libc::c_int as integer;
                    while k <= i__3 {
                        let ref mut fresh0 =
                            *c__.offset((k + j * c_dim1) as isize);
                        *fresh0 +=
                            temp1 * *a.offset((k + i__ * a_dim1) as isize);
                        temp2 +=
                            *b.offset((k + j * b_dim1) as isize) *
                                *a.offset((k + i__ * a_dim1) as isize);
                        k += 1
                        /* L70: */
                        /* L60: */
                        /* L50: */
                    }
                    if *beta == 0.0f32 {
                        *c__.offset((i__ + j * c_dim1) as isize) =
                            temp1 * *a.offset((i__ + i__ * a_dim1) as isize) +
                                *alpha * temp2
                    } else {
                        *c__.offset((i__ + j * c_dim1) as isize) =
                            *beta * *c__.offset((i__ + j * c_dim1) as isize) +
                                temp1 *
                                    *a.offset((i__ + i__ * a_dim1) as isize) +
                                *alpha * temp2
                    }
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
                    temp1 = *alpha * *b.offset((i__ + j * b_dim1) as isize);
                    temp2 = 0.0f32;
                    i__2 = *m;
                    k = i__ + 1 as libc::c_int as libc::c_long;
                    while k <= i__2 {
                        let ref mut fresh1 =
                            *c__.offset((k + j * c_dim1) as isize);
                        *fresh1 +=
                            temp1 * *a.offset((k + i__ * a_dim1) as isize);
                        temp2 +=
                            *b.offset((k + j * b_dim1) as isize) *
                                *a.offset((k + i__ * a_dim1) as isize);
                        k += 1
                        /* L90: */
                        /* L80: */
                    }
                    if *beta == 0.0f32 {
                        *c__.offset((i__ + j * c_dim1) as isize) =
                            temp1 * *a.offset((i__ + i__ * a_dim1) as isize) +
                                *alpha * temp2
                    } else {
                        *c__.offset((i__ + j * c_dim1) as isize) =
                            *beta * *c__.offset((i__ + j * c_dim1) as isize) +
                                temp1 *
                                    *a.offset((i__ + i__ * a_dim1) as isize) +
                                *alpha * temp2
                    }
                    i__ -= 1
                }
                j += 1
                /* L100: */
            }
        }
    } else {
        /*        Form  C := alpha*B*A + beta*C. */
        i__1 = *n;
        j = 1 as libc::c_int as integer;
        while j <= i__1 {
            temp1 = *alpha * *a.offset((j + j * a_dim1) as isize);
            if *beta == 0.0f32 {
                i__2 = *m;
                i__ = 1 as libc::c_int as integer;
                while i__ <= i__2 {
                    *c__.offset((i__ + j * c_dim1) as isize) =
                        temp1 * *b.offset((i__ + j * b_dim1) as isize);
                    i__ += 1
                    /* L170: */
                    /* L110: */
                }
            } else {
                i__2 = *m;
                i__ = 1 as libc::c_int as integer;
                while i__ <= i__2 {
                    *c__.offset((i__ + j * c_dim1) as isize) =
                        *beta * *c__.offset((i__ + j * c_dim1) as isize) +
                            temp1 * *b.offset((i__ + j * b_dim1) as isize);
                    i__ += 1
                    /* L120: */
                }
            }
            i__2 = j - 1 as libc::c_int as libc::c_long;
            k = 1 as libc::c_int as integer;
            while k <= i__2 {
                if upper != 0 {
                    temp1 = *alpha * *a.offset((k + j * a_dim1) as isize)
                } else {
                    temp1 = *alpha * *a.offset((j + k * a_dim1) as isize)
                }
                i__3 = *m;
                i__ = 1 as libc::c_int as integer;
                while i__ <= i__3 {
                    let ref mut fresh2 =
                        *c__.offset((i__ + j * c_dim1) as isize);
                    *fresh2 += temp1 * *b.offset((i__ + k * b_dim1) as isize);
                    i__ += 1
                    /* L140: */
                    /* L130: */
                }
                k += 1
            }
            i__2 = *n;
            k = j + 1 as libc::c_int as libc::c_long;
            while k <= i__2 {
                if upper != 0 {
                    temp1 = *alpha * *a.offset((j + k * a_dim1) as isize)
                } else {
                    temp1 = *alpha * *a.offset((k + j * a_dim1) as isize)
                }
                i__3 = *m;
                i__ = 1 as libc::c_int as integer;
                while i__ <= i__3 {
                    let ref mut fresh3 =
                        *c__.offset((i__ + j * c_dim1) as isize);
                    *fresh3 += temp1 * *b.offset((i__ + k * b_dim1) as isize);
                    i__ += 1
                    /* L160: */
                    /* L150: */
                }
                k += 1
            }
            j += 1
        }
    }
    return 0 as libc::c_int;
    /*     End of SSYMM . */
}
/* ssymm_ */
