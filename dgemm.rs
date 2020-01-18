use ::libc;
/* f2c.h  --  Standard Fortran to C header file */
/* *  barf  [ba:rf]  2.  "He suggested using FORTRAN, and everybody barfed."

	- From The Shogakukan DICTIONARY OF NEW ENGLISH (Second edition) */
pub type integer = libc::c_long;
pub type doublereal = libc::c_double;
pub type logical = libc::c_long;
/* dgemm.f -- translated by f2c (version 20061008).
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
pub unsafe extern "C" fn f2c_dgemm(mut transa: *mut libc::c_char,
                                   mut transb: *mut libc::c_char,
                                   mut m: *mut integer, mut n: *mut integer,
                                   mut k: *mut integer,
                                   mut alpha: *mut doublereal,
                                   mut a: *mut doublereal,
                                   mut lda: *mut integer,
                                   mut b: *mut doublereal,
                                   mut ldb: *mut integer,
                                   mut beta: *mut doublereal,
                                   mut c__: *mut doublereal,
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
    /* Local variables */
    let mut i__: integer = 0;
    let mut j: integer = 0;
    let mut l: integer = 0;
    let mut info: integer = 0;
    let mut nota: logical = 0;
    let mut notb: logical = 0;
    let mut temp: doublereal = 0.;
    let mut ncola: integer = 0;
    extern "C" {
        #[link_name = "lsame_"]
        fn lsame__0(_: *mut libc::c_char, _: *mut libc::c_char) -> logical;
    }
    let mut nrowa: integer = 0;
    let mut nrowb: integer = 0;
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
    /*  DGEMM  performs one of the matrix-matrix operations */
    /*     C := alpha*op( A )*op( B ) + beta*C, */
    /*  where  op( X ) is one of */
    /*     op( X ) = X   or   op( X ) = X', */
    /*  alpha and beta are scalars, and A, B and C are matrices, with op( A ) */
/*  an m by k matrix,  op( B )  a  k by n matrix and  C an m by n matrix. */
    /*  Arguments */
/*  ========== */
    /*  TRANSA - CHARACTER*1. */
/*           On entry, TRANSA specifies the form of op( A ) to be used in */
/*           the matrix multiplication as follows: */
    /*              TRANSA = 'N' or 'n',  op( A ) = A. */
    /*              TRANSA = 'T' or 't',  op( A ) = A'. */
    /*              TRANSA = 'C' or 'c',  op( A ) = A'. */
    /*           Unchanged on exit. */
    /*  TRANSB - CHARACTER*1. */
/*           On entry, TRANSB specifies the form of op( B ) to be used in */
/*           the matrix multiplication as follows: */
    /*              TRANSB = 'N' or 'n',  op( B ) = B. */
    /*              TRANSB = 'T' or 't',  op( B ) = B'. */
    /*              TRANSB = 'C' or 'c',  op( B ) = B'. */
    /*           Unchanged on exit. */
    /*  M      - INTEGER. */
/*           On entry,  M  specifies  the number  of rows  of the  matrix */
/*           op( A )  and of the  matrix  C.  M  must  be at least  zero. */
/*           Unchanged on exit. */
    /*  N      - INTEGER. */
/*           On entry,  N  specifies the number  of columns of the matrix */
/*           op( B ) and the number of columns of the matrix C. N must be */
/*           at least zero. */
/*           Unchanged on exit. */
    /*  K      - INTEGER. */
/*           On entry,  K  specifies  the number of columns of the matrix */
/*           op( A ) and the number of rows of the matrix op( B ). K must */
/*           be at least  zero. */
/*           Unchanged on exit. */
    /*  ALPHA  - DOUBLE PRECISION. */
/*           On entry, ALPHA specifies the scalar alpha. */
/*           Unchanged on exit. */
    /*  A      - DOUBLE PRECISION array of DIMENSION ( LDA, ka ), where ka is */
/*           k  when  TRANSA = 'N' or 'n',  and is  m  otherwise. */
/*           Before entry with  TRANSA = 'N' or 'n',  the leading  m by k */
/*           part of the array  A  must contain the matrix  A,  otherwise */
/*           the leading  k by m  part of the array  A  must contain  the */
/*           matrix A. */
/*           Unchanged on exit. */
    /*  LDA    - INTEGER. */
/*           On entry, LDA specifies the first dimension of A as declared */
/*           in the calling (sub) program. When  TRANSA = 'N' or 'n' then */
/*           LDA must be at least  max( 1, m ), otherwise  LDA must be at */
/*           least  max( 1, k ). */
/*           Unchanged on exit. */
    /*  B      - DOUBLE PRECISION array of DIMENSION ( LDB, kb ), where kb is */
/*           n  when  TRANSB = 'N' or 'n',  and is  k  otherwise. */
/*           Before entry with  TRANSB = 'N' or 'n',  the leading  k by n */
/*           part of the array  B  must contain the matrix  B,  otherwise */
/*           the leading  n by k  part of the array  B  must contain  the */
/*           matrix B. */
/*           Unchanged on exit. */
    /*  LDB    - INTEGER. */
/*           On entry, LDB specifies the first dimension of B as declared */
/*           in the calling (sub) program. When  TRANSB = 'N' or 'n' then */
/*           LDB must be at least  max( 1, k ), otherwise  LDB must be at */
/*           least  max( 1, n ). */
/*           Unchanged on exit. */
    /*  BETA   - DOUBLE PRECISION. */
/*           On entry,  BETA  specifies the scalar  beta.  When  BETA  is */
/*           supplied as zero then C need not be set on input. */
/*           Unchanged on exit. */
    /*  C      - DOUBLE PRECISION array of DIMENSION ( LDC, n ). */
/*           Before entry, the leading  m by n  part of the array  C must */
/*           contain the matrix  C,  except when  beta  is zero, in which */
/*           case C need not be set on entry. */
/*           On exit, the array  C  is overwritten by the  m by n  matrix */
/*           ( alpha*op( A )*op( B ) + beta*C ). */
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
    /*     Set  NOTA  and  NOTB  as  true if  A  and  B  respectively are not */
/*     transposed and set  NROWA, NCOLA and  NROWB  as the number of rows */
/*     and  columns of  A  and the  number of  rows  of  B  respectively. */
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
    nota =
        lsame__0(transa,
                 b"N\x00" as *const u8 as *const libc::c_char as
                     *mut libc::c_char);
    notb =
        lsame__0(transb,
                 b"N\x00" as *const u8 as *const libc::c_char as
                     *mut libc::c_char);
    if nota != 0 { nrowa = *m; ncola = *k } else { nrowa = *k; ncola = *m }
    if notb != 0 { nrowb = *k } else { nrowb = *n }
    /*     Test the input parameters. */
    info = 0 as libc::c_int as integer;
    if nota == 0 &&
           lsame__0(transa,
                    b"C\x00" as *const u8 as *const libc::c_char as
                        *mut libc::c_char) == 0 &&
           lsame__0(transa,
                    b"T\x00" as *const u8 as *const libc::c_char as
                        *mut libc::c_char) == 0 {
        info = 1 as libc::c_int as integer
    } else if notb == 0 &&
                  lsame__0(transb,
                           b"C\x00" as *const u8 as *const libc::c_char as
                               *mut libc::c_char) == 0 &&
                  lsame__0(transb,
                           b"T\x00" as *const u8 as *const libc::c_char as
                               *mut libc::c_char) == 0 {
        info = 2 as libc::c_int as integer
    } else if *m < 0 as libc::c_int as libc::c_long {
        info = 3 as libc::c_int as integer
    } else if *n < 0 as libc::c_int as libc::c_long {
        info = 4 as libc::c_int as integer
    } else if *k < 0 as libc::c_int as libc::c_long {
        info = 5 as libc::c_int as integer
    } else if *lda <
                  (if 1 as libc::c_int as libc::c_long >= nrowa {
                       1 as libc::c_int as libc::c_long
                   } else { nrowa }) {
        info = 8 as libc::c_int as integer
    } else if *ldb <
                  (if 1 as libc::c_int as libc::c_long >= nrowb {
                       1 as libc::c_int as libc::c_long
                   } else { nrowb }) {
        info = 10 as libc::c_int as integer
    } else if *ldc <
                  (if 1 as libc::c_int as libc::c_long >= *m {
                       1 as libc::c_int as libc::c_long
                   } else { *m }) {
        info = 13 as libc::c_int as integer
    }
    if info != 0 as libc::c_int as libc::c_long {
        xerbla__0(b"DGEMM \x00" as *const u8 as *const libc::c_char as
                      *mut libc::c_char, &mut info);
        return 0 as libc::c_int
    }
    /*     Quick return if possible. */
    if *m == 0 as libc::c_int as libc::c_long ||
           *n == 0 as libc::c_int as libc::c_long ||
           (*alpha == 0.0f64 || *k == 0 as libc::c_int as libc::c_long) &&
               *beta == 1.0f64 {
        return 0 as libc::c_int
    }
    /*     And if  alpha.eq.zero. */
    if *alpha == 0.0f64 {
        if *beta == 0.0f64 {
            i__1 = *n;
            j = 1 as libc::c_int as integer;
            while j <= i__1 {
                i__2 = *m;
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
    if notb != 0 {
        if nota != 0 {
            /*           Form  C := alpha*A*B + beta*C. */
            i__1 = *n;
            j = 1 as libc::c_int as integer;
            while j <= i__1 {
                if *beta == 0.0f64 {
                    i__2 = *m;
                    i__ = 1 as libc::c_int as integer;
                    while i__ <= i__2 {
                        *c__.offset((i__ + j * c_dim1) as isize) = 0.0f64;
                        i__ += 1
                        /* L50: */
                    }
                } else if *beta != 1.0f64 {
                    i__2 = *m;
                    i__ = 1 as libc::c_int as integer;
                    while i__ <= i__2 {
                        *c__.offset((i__ + j * c_dim1) as isize) =
                            *beta * *c__.offset((i__ + j * c_dim1) as isize);
                        i__ += 1
                        /* L60: */
                    }
                }
                i__2 = *k;
                l = 1 as libc::c_int as integer;
                while l <= i__2 {
                    if *b.offset((l + j * b_dim1) as isize) != 0.0f64 {
                        temp = *alpha * *b.offset((l + j * b_dim1) as isize);
                        i__3 = *m;
                        i__ = 1 as libc::c_int as integer;
                        while i__ <= i__3 {
                            let ref mut fresh0 =
                                *c__.offset((i__ + j * c_dim1) as isize);
                            *fresh0 +=
                                temp * *a.offset((i__ + l * a_dim1) as isize);
                            i__ += 1
                            /* L90: */
                            /* L70: */
                        }
                    }
                    l += 1
                    /* L80: */
                }
                j += 1
            }
        } else {
            /*           Form  C := alpha*A'*B + beta*C */
            i__1 = *n;
            j = 1 as libc::c_int as integer;
            while j <= i__1 {
                i__2 = *m;
                i__ = 1 as libc::c_int as integer;
                while i__ <= i__2 {
                    temp = 0.0f64;
                    i__3 = *k;
                    l = 1 as libc::c_int as integer;
                    while l <= i__3 {
                        temp +=
                            *a.offset((l + i__ * a_dim1) as isize) *
                                *b.offset((l + j * b_dim1) as isize);
                        l += 1
                        /* L120: */
                        /* L110: */
                        /* L100: */
                    }
                    if *beta == 0.0f64 {
                        *c__.offset((i__ + j * c_dim1) as isize) =
                            *alpha * temp
                    } else {
                        *c__.offset((i__ + j * c_dim1) as isize) =
                            *alpha * temp +
                                *beta *
                                    *c__.offset((i__ + j * c_dim1) as isize)
                    }
                    i__ += 1
                }
                j += 1
            }
        }
    } else if nota != 0 {
        /*           Form  C := alpha*A*B' + beta*C */
        i__1 = *n;
        j = 1 as libc::c_int as integer;
        while j <= i__1 {
            if *beta == 0.0f64 {
                i__2 = *m;
                i__ = 1 as libc::c_int as integer;
                while i__ <= i__2 {
                    *c__.offset((i__ + j * c_dim1) as isize) = 0.0f64;
                    i__ += 1
                    /* L130: */
                }
            } else if *beta != 1.0f64 {
                i__2 = *m;
                i__ = 1 as libc::c_int as integer;
                while i__ <= i__2 {
                    *c__.offset((i__ + j * c_dim1) as isize) =
                        *beta * *c__.offset((i__ + j * c_dim1) as isize);
                    i__ += 1
                    /* L140: */
                }
            }
            i__2 = *k;
            l = 1 as libc::c_int as integer;
            while l <= i__2 {
                if *b.offset((j + l * b_dim1) as isize) != 0.0f64 {
                    temp = *alpha * *b.offset((j + l * b_dim1) as isize);
                    i__3 = *m;
                    i__ = 1 as libc::c_int as integer;
                    while i__ <= i__3 {
                        let ref mut fresh1 =
                            *c__.offset((i__ + j * c_dim1) as isize);
                        *fresh1 +=
                            temp * *a.offset((i__ + l * a_dim1) as isize);
                        i__ += 1
                        /* L170: */
                        /* L150: */
                    }
                }
                l += 1
                /* L160: */
            }
            j += 1
        }
    } else {
        /*           Form  C := alpha*A'*B' + beta*C */
        i__1 = *n;
        j = 1 as libc::c_int as integer;
        while j <= i__1 {
            i__2 = *m;
            i__ = 1 as libc::c_int as integer;
            while i__ <= i__2 {
                temp = 0.0f64;
                i__3 = *k;
                l = 1 as libc::c_int as integer;
                while l <= i__3 {
                    temp +=
                        *a.offset((l + i__ * a_dim1) as isize) *
                            *b.offset((j + l * b_dim1) as isize);
                    l += 1
                    /* L200: */
                    /* L190: */
                    /* L180: */
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
    /*     End of DGEMM . */
}
/* dgemm_ */
