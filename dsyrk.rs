use ::libc;
/* f2c.h  --  Standard Fortran to C header file */
/* *  barf  [ba:rf]  2.  "He suggested using FORTRAN, and everybody barfed."

	- From The Shogakukan DICTIONARY OF NEW ENGLISH (Second edition) */
pub type integer = libc::c_long;
pub type doublereal = libc::c_double;
pub type logical = libc::c_long;
/* dsyrk.f -- translated by f2c (version 20061008).
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
pub unsafe extern "C" fn f2c_dsyrk(mut uplo: *mut libc::c_char,
                                   mut trans: *mut libc::c_char,
                                   mut n: *mut integer, mut k: *mut integer,
                                   mut alpha: *mut doublereal,
                                   mut a: *mut doublereal,
                                   mut lda: *mut integer,
                                   mut beta: *mut doublereal,
                                   mut c__: *mut doublereal,
                                   mut ldc: *mut integer) -> libc::c_int {
    /* System generated locals */
    let mut a_dim1: integer = 0;
    let mut a_offset: integer = 0;
    let mut c_dim1: integer = 0;
    let mut c_offset: integer = 0;
    let mut i__1: integer = 0;
    let mut i__2: integer = 0;
    let mut i__3: integer = 0;
    /* Local variables */
    let mut i__: integer = 0;
    let mut j: integer = 0;
    let mut l: integer = 0;
    let mut info: integer = 0;
    let mut temp: doublereal = 0.;
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
    /*  DSYRK  performs one of the symmetric rank k operations */
    /*     C := alpha*A*A' + beta*C, */
    /*  or */
    /*     C := alpha*A'*A + beta*C, */
    /*  where  alpha and beta  are scalars, C is an  n by n  symmetric matrix */
/*  and  A  is an  n by k  matrix in the first case and a  k by n  matrix */
/*  in the second case. */
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
    /*              TRANS = 'N' or 'n'   C := alpha*A*A' + beta*C. */
    /*              TRANS = 'T' or 't'   C := alpha*A'*A + beta*C. */
    /*              TRANS = 'C' or 'c'   C := alpha*A'*A + beta*C. */
    /*           Unchanged on exit. */
    /*  N      - INTEGER. */
/*           On entry,  N specifies the order of the matrix C.  N must be */
/*           at least zero. */
/*           Unchanged on exit. */
    /*  K      - INTEGER. */
/*           On entry with  TRANS = 'N' or 'n',  K  specifies  the number */
/*           of  columns   of  the   matrix   A,   and  on   entry   with */
/*           TRANS = 'T' or 't' or 'C' or 'c',  K  specifies  the  number */
/*           of rows of the matrix  A.  K must be at least zero. */
/*           Unchanged on exit. */
    /*  ALPHA  - DOUBLE PRECISION. */
/*           On entry, ALPHA specifies the scalar alpha. */
/*           Unchanged on exit. */
    /*  A      - DOUBLE PRECISION array of DIMENSION ( LDA, ka ), where ka is */
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
    /*  BETA   - DOUBLE PRECISION. */
/*           On entry, BETA specifies the scalar beta. */
/*           Unchanged on exit. */
    /*  C      - DOUBLE PRECISION array of DIMENSION ( LDC, n ). */
/*           Before entry  with  UPLO = 'U' or 'u',  the leading  n by n */
/*           upper triangular part of the array C must contain the upper */
/*           triangular part  of the  symmetric matrix  and the strictly */
/*           lower triangular part of C is not referenced.  On exit, the */
/*           upper triangular part of the array  C is overwritten by the */
/*           upper triangular part of the updated matrix. */
/*           Before entry  with  UPLO = 'L' or 'l',  the leading  n by n */
/*           lower triangular part of the array C must contain the lower */
/*           triangular part  of the  symmetric matrix  and the strictly */
/*           upper triangular part of C is not referenced.  On exit, the */
/*           lower triangular part of the array  C is overwritten by the */
/*           lower triangular part of the updated matrix. */
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
                           b"T\x00" as *const u8 as *const libc::c_char as
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
    } else if *ldc <
                  (if 1 as libc::c_int as libc::c_long >= *n {
                       1 as libc::c_int as libc::c_long
                   } else { *n }) {
        info = 10 as libc::c_int as integer
    }
    if info != 0 as libc::c_int as libc::c_long {
        xerbla__0(b"DSYRK \x00" as *const u8 as *const libc::c_char as
                      *mut libc::c_char, &mut info);
        return 0 as libc::c_int
    }
    /*     Quick return if possible. */
    if *n == 0 as libc::c_int as libc::c_long ||
           (*alpha == 0.0f64 || *k == 0 as libc::c_int as libc::c_long) &&
               *beta == 1.0f64 {
        return 0 as libc::c_int
    }
    /*     And when  alpha.eq.zero. */
    if *alpha == 0.0f64 {
        if upper != 0 {
            if *beta == 0.0f64 {
                i__1 = *n;
                j = 1 as libc::c_int as integer;
                while j <= i__1 {
                    i__2 = j;
                    i__ = 1 as libc::c_int as integer;
                    while i__ <= i__2 {
                        *c__.offset((i__ + j * c_dim1) as isize) = 0.0f64;
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
                    i__2 = j;
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
        } else if *beta == 0.0f64 {
            i__1 = *n;
            j = 1 as libc::c_int as integer;
            while j <= i__1 {
                i__2 = *n;
                i__ = j;
                while i__ <= i__2 {
                    *c__.offset((i__ + j * c_dim1) as isize) = 0.0f64;
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
                i__2 = *n;
                i__ = j;
                while i__ <= i__2 {
                    *c__.offset((i__ + j * c_dim1) as isize) =
                        *beta * *c__.offset((i__ + j * c_dim1) as isize);
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
        /*        Form  C := alpha*A*A' + beta*C. */
        if upper != 0 {
            i__1 = *n;
            j = 1 as libc::c_int as integer;
            while j <= i__1 {
                if *beta == 0.0f64 {
                    i__2 = j;
                    i__ = 1 as libc::c_int as integer;
                    while i__ <= i__2 {
                        *c__.offset((i__ + j * c_dim1) as isize) = 0.0f64;
                        i__ += 1
                        /* L90: */
                    }
                } else if *beta != 1.0f64 {
                    i__2 = j;
                    i__ = 1 as libc::c_int as integer;
                    while i__ <= i__2 {
                        *c__.offset((i__ + j * c_dim1) as isize) =
                            *beta * *c__.offset((i__ + j * c_dim1) as isize);
                        i__ += 1
                        /* L100: */
                    }
                }
                i__2 = *k;
                l = 1 as libc::c_int as integer;
                while l <= i__2 {
                    if *a.offset((j + l * a_dim1) as isize) != 0.0f64 {
                        temp = *alpha * *a.offset((j + l * a_dim1) as isize);
                        i__3 = j;
                        i__ = 1 as libc::c_int as integer;
                        while i__ <= i__3 {
                            let ref mut fresh0 =
                                *c__.offset((i__ + j * c_dim1) as isize);
                            *fresh0 +=
                                temp * *a.offset((i__ + l * a_dim1) as isize);
                            i__ += 1
                            /* L130: */
                            /* L110: */
                        }
                    }
                    l += 1
                    /* L120: */
                }
                j += 1
            }
        } else {
            i__1 = *n;
            j = 1 as libc::c_int as integer;
            while j <= i__1 {
                if *beta == 0.0f64 {
                    i__2 = *n;
                    i__ = j;
                    while i__ <= i__2 {
                        *c__.offset((i__ + j * c_dim1) as isize) = 0.0f64;
                        i__ += 1
                        /* L140: */
                    }
                } else if *beta != 1.0f64 {
                    i__2 = *n;
                    i__ = j;
                    while i__ <= i__2 {
                        *c__.offset((i__ + j * c_dim1) as isize) =
                            *beta * *c__.offset((i__ + j * c_dim1) as isize);
                        i__ += 1
                        /* L150: */
                    }
                }
                i__2 = *k;
                l = 1 as libc::c_int as integer;
                while l <= i__2 {
                    if *a.offset((j + l * a_dim1) as isize) != 0.0f64 {
                        temp = *alpha * *a.offset((j + l * a_dim1) as isize);
                        i__3 = *n;
                        i__ = j;
                        while i__ <= i__3 {
                            let ref mut fresh1 =
                                *c__.offset((i__ + j * c_dim1) as isize);
                            *fresh1 +=
                                temp * *a.offset((i__ + l * a_dim1) as isize);
                            i__ += 1
                            /* L180: */
                            /* L160: */
                        }
                    }
                    l += 1
                    /* L170: */
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
                temp = 0.0f64;
                i__3 = *k;
                l = 1 as libc::c_int as integer;
                while l <= i__3 {
                    temp +=
                        *a.offset((l + i__ * a_dim1) as isize) *
                            *a.offset((l + j * a_dim1) as isize);
                    l += 1
                    /*        Form  C := alpha*A'*A + beta*C. */
                    /* L210: */
                    /* L200: */
                    /* L190: */
                }
                if *beta == 0.0f64 {
                    *c__.offset((i__ + j * c_dim1) as isize) = *alpha * temp
                } else {
                    *c__.offset((i__ + j * c_dim1) as isize) =
                        *alpha * temp +
                            *beta * *c__.offset((i__ + j * c_dim1) as isize)
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
                temp = 0.0f64;
                i__3 = *k;
                l = 1 as libc::c_int as integer;
                while l <= i__3 {
                    temp +=
                        *a.offset((l + i__ * a_dim1) as isize) *
                            *a.offset((l + j * a_dim1) as isize);
                    l += 1
                    /* L240: */
                    /* L230: */
                    /* L220: */
                }
                if *beta == 0.0f64 {
                    *c__.offset((i__ + j * c_dim1) as isize) = *alpha * temp
                } else {
                    *c__.offset((i__ + j * c_dim1) as isize) =
                        *alpha * temp +
                            *beta * *c__.offset((i__ + j * c_dim1) as isize)
                }
                i__ += 1
            }
            j += 1
        }
    }
    return 0 as libc::c_int;
    /*     End of DSYRK . */
}
/* dsyrk_ */
