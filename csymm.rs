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
/* csymm.f -- translated by f2c (version 20061008).
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
pub unsafe extern "C" fn f2c_csymm(mut side: *mut libc::c_char,
                                   mut uplo: *mut libc::c_char,
                                   mut m: *mut integer, mut n: *mut integer,
                                   mut alpha: *mut complex,
                                   mut a: *mut complex, mut lda: *mut integer,
                                   mut b: *mut complex, mut ldb: *mut integer,
                                   mut beta: *mut complex,
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
    let mut q__1: complex = complex{r: 0., i: 0.,};
    let mut q__2: complex = complex{r: 0., i: 0.,};
    let mut q__3: complex = complex{r: 0., i: 0.,};
    let mut q__4: complex = complex{r: 0., i: 0.,};
    let mut q__5: complex = complex{r: 0., i: 0.,};
    /* Local variables */
    let mut i__: integer = 0;
    let mut j: integer = 0;
    let mut k: integer = 0;
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
    /*  CSYMM  performs one of the matrix-matrix operations */
    /*     C := alpha*A*B + beta*C, */
    /*  or */
    /*     C := alpha*B*A + beta*C, */
    /*  where  alpha and beta are scalars, A is a symmetric matrix and  B and */
/*  C are m by n matrices. */
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
    /*  ALPHA  - COMPLEX         . */
/*           On entry, ALPHA specifies the scalar alpha. */
/*           Unchanged on exit. */
    /*  A      - COMPLEX          array of DIMENSION ( LDA, ka ), where ka is */
/*           m  when  SIDE = 'L' or 'l'  and is n  otherwise. */
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
/*           in the  calling (sub) program. When  SIDE = 'L' or 'l'  then */
/*           LDA must be at least  max( 1, m ), otherwise  LDA must be at */
/*           least max( 1, n ). */
/*           Unchanged on exit. */
    /*  B      - COMPLEX          array of DIMENSION ( LDB, n ). */
/*           Before entry, the leading  m by n part of the array  B  must */
/*           contain the matrix B. */
/*           Unchanged on exit. */
    /*  LDB    - INTEGER. */
/*           On entry, LDB specifies the first dimension of B as declared */
/*           in  the  calling  (sub)  program.   LDB  must  be  at  least */
/*           max( 1, m ). */
/*           Unchanged on exit. */
    /*  BETA   - COMPLEX         . */
/*           On entry,  BETA  specifies the scalar  beta.  When  BETA  is */
/*           supplied as zero then C need not be set on input. */
/*           Unchanged on exit. */
    /*  C      - COMPLEX          array of DIMENSION ( LDC, n ). */
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
        xerbla__0(b"CSYMM \x00" as *const u8 as *const libc::c_char as
                      *mut libc::c_char, &mut info);
        return 0 as libc::c_int
    }
    /*     Quick return if possible. */
    if *m == 0 as libc::c_int as libc::c_long ||
           *n == 0 as libc::c_int as libc::c_long ||
           (*alpha).r == 0.0f32 && (*alpha).i == 0.0f32 &&
               ((*beta).r == 1.0f32 && (*beta).i == 0.0f32) {
        return 0 as libc::c_int
    }
    /*     And when  alpha.eq.zero. */
    if (*alpha).r == 0.0f32 && (*alpha).i == 0.0f32 {
        if (*beta).r == 0.0f32 && (*beta).i == 0.0f32 {
            i__1 = *n;
            j = 1 as libc::c_int as integer;
            while j <= i__1 {
                i__2 = *m;
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
                i__2 = *m;
                i__ = 1 as libc::c_int as integer;
                while i__ <= i__2 {
                    i__3 = i__ + j * c_dim1;
                    i__4 = i__ + j * c_dim1;
                    q__1.r =
                        (*beta).r * (*c__.offset(i__4 as isize)).r -
                            (*beta).i * (*c__.offset(i__4 as isize)).i;
                    q__1.i =
                        (*beta).r * (*c__.offset(i__4 as isize)).i +
                            (*beta).i * (*c__.offset(i__4 as isize)).r;
                    (*c__.offset(i__3 as isize)).r = q__1.r;
                    (*c__.offset(i__3 as isize)).i = q__1.i;
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
                    i__3 = i__ + j * b_dim1;
                    q__1.r =
                        (*alpha).r * (*b.offset(i__3 as isize)).r -
                            (*alpha).i * (*b.offset(i__3 as isize)).i;
                    q__1.i =
                        (*alpha).r * (*b.offset(i__3 as isize)).i +
                            (*alpha).i * (*b.offset(i__3 as isize)).r;
                    temp1.r = q__1.r;
                    temp1.i = q__1.i;
                    temp2.r = 0.0f32;
                    temp2.i = 0.0f32;
                    i__3 = i__ - 1 as libc::c_int as libc::c_long;
                    k = 1 as libc::c_int as integer;
                    while k <= i__3 {
                        i__4 = k + j * c_dim1;
                        i__5 = k + j * c_dim1;
                        i__6 = k + i__ * a_dim1;
                        q__2.r =
                            temp1.r * (*a.offset(i__6 as isize)).r -
                                temp1.i * (*a.offset(i__6 as isize)).i;
                        q__2.i =
                            temp1.r * (*a.offset(i__6 as isize)).i +
                                temp1.i * (*a.offset(i__6 as isize)).r;
                        q__1.r = (*c__.offset(i__5 as isize)).r + q__2.r;
                        q__1.i = (*c__.offset(i__5 as isize)).i + q__2.i;
                        (*c__.offset(i__4 as isize)).r = q__1.r;
                        (*c__.offset(i__4 as isize)).i = q__1.i;
                        i__4 = k + j * b_dim1;
                        i__5 = k + i__ * a_dim1;
                        q__2.r =
                            (*b.offset(i__4 as isize)).r *
                                (*a.offset(i__5 as isize)).r -
                                (*b.offset(i__4 as isize)).i *
                                    (*a.offset(i__5 as isize)).i;
                        q__2.i =
                            (*b.offset(i__4 as isize)).r *
                                (*a.offset(i__5 as isize)).i +
                                (*b.offset(i__4 as isize)).i *
                                    (*a.offset(i__5 as isize)).r;
                        q__1.r = temp2.r + q__2.r;
                        q__1.i = temp2.i + q__2.i;
                        temp2.r = q__1.r;
                        temp2.i = q__1.i;
                        k += 1
                        /* L70: */
                        /* L60: */
                        /* L50: */
                    }
                    if (*beta).r == 0.0f32 && (*beta).i == 0.0f32 {
                        i__3 = i__ + j * c_dim1;
                        i__4 = i__ + i__ * a_dim1;
                        q__2.r =
                            temp1.r * (*a.offset(i__4 as isize)).r -
                                temp1.i * (*a.offset(i__4 as isize)).i;
                        q__2.i =
                            temp1.r * (*a.offset(i__4 as isize)).i +
                                temp1.i * (*a.offset(i__4 as isize)).r;
                        q__3.r = (*alpha).r * temp2.r - (*alpha).i * temp2.i;
                        q__3.i = (*alpha).r * temp2.i + (*alpha).i * temp2.r;
                        q__1.r = q__2.r + q__3.r;
                        q__1.i = q__2.i + q__3.i;
                        (*c__.offset(i__3 as isize)).r = q__1.r;
                        (*c__.offset(i__3 as isize)).i = q__1.i
                    } else {
                        i__3 = i__ + j * c_dim1;
                        i__4 = i__ + j * c_dim1;
                        q__3.r =
                            (*beta).r * (*c__.offset(i__4 as isize)).r -
                                (*beta).i * (*c__.offset(i__4 as isize)).i;
                        q__3.i =
                            (*beta).r * (*c__.offset(i__4 as isize)).i +
                                (*beta).i * (*c__.offset(i__4 as isize)).r;
                        i__5 = i__ + i__ * a_dim1;
                        q__4.r =
                            temp1.r * (*a.offset(i__5 as isize)).r -
                                temp1.i * (*a.offset(i__5 as isize)).i;
                        q__4.i =
                            temp1.r * (*a.offset(i__5 as isize)).i +
                                temp1.i * (*a.offset(i__5 as isize)).r;
                        q__2.r = q__3.r + q__4.r;
                        q__2.i = q__3.i + q__4.i;
                        q__5.r = (*alpha).r * temp2.r - (*alpha).i * temp2.i;
                        q__5.i = (*alpha).r * temp2.i + (*alpha).i * temp2.r;
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
                i__ = *m;
                while i__ >= 1 as libc::c_int as libc::c_long {
                    i__2 = i__ + j * b_dim1;
                    q__1.r =
                        (*alpha).r * (*b.offset(i__2 as isize)).r -
                            (*alpha).i * (*b.offset(i__2 as isize)).i;
                    q__1.i =
                        (*alpha).r * (*b.offset(i__2 as isize)).i +
                            (*alpha).i * (*b.offset(i__2 as isize)).r;
                    temp1.r = q__1.r;
                    temp1.i = q__1.i;
                    temp2.r = 0.0f32;
                    temp2.i = 0.0f32;
                    i__2 = *m;
                    k = i__ + 1 as libc::c_int as libc::c_long;
                    while k <= i__2 {
                        i__3 = k + j * c_dim1;
                        i__4 = k + j * c_dim1;
                        i__5 = k + i__ * a_dim1;
                        q__2.r =
                            temp1.r * (*a.offset(i__5 as isize)).r -
                                temp1.i * (*a.offset(i__5 as isize)).i;
                        q__2.i =
                            temp1.r * (*a.offset(i__5 as isize)).i +
                                temp1.i * (*a.offset(i__5 as isize)).r;
                        q__1.r = (*c__.offset(i__4 as isize)).r + q__2.r;
                        q__1.i = (*c__.offset(i__4 as isize)).i + q__2.i;
                        (*c__.offset(i__3 as isize)).r = q__1.r;
                        (*c__.offset(i__3 as isize)).i = q__1.i;
                        i__3 = k + j * b_dim1;
                        i__4 = k + i__ * a_dim1;
                        q__2.r =
                            (*b.offset(i__3 as isize)).r *
                                (*a.offset(i__4 as isize)).r -
                                (*b.offset(i__3 as isize)).i *
                                    (*a.offset(i__4 as isize)).i;
                        q__2.i =
                            (*b.offset(i__3 as isize)).r *
                                (*a.offset(i__4 as isize)).i +
                                (*b.offset(i__3 as isize)).i *
                                    (*a.offset(i__4 as isize)).r;
                        q__1.r = temp2.r + q__2.r;
                        q__1.i = temp2.i + q__2.i;
                        temp2.r = q__1.r;
                        temp2.i = q__1.i;
                        k += 1
                        /* L90: */
                        /* L80: */
                    }
                    if (*beta).r == 0.0f32 && (*beta).i == 0.0f32 {
                        i__2 = i__ + j * c_dim1;
                        i__3 = i__ + i__ * a_dim1;
                        q__2.r =
                            temp1.r * (*a.offset(i__3 as isize)).r -
                                temp1.i * (*a.offset(i__3 as isize)).i;
                        q__2.i =
                            temp1.r * (*a.offset(i__3 as isize)).i +
                                temp1.i * (*a.offset(i__3 as isize)).r;
                        q__3.r = (*alpha).r * temp2.r - (*alpha).i * temp2.i;
                        q__3.i = (*alpha).r * temp2.i + (*alpha).i * temp2.r;
                        q__1.r = q__2.r + q__3.r;
                        q__1.i = q__2.i + q__3.i;
                        (*c__.offset(i__2 as isize)).r = q__1.r;
                        (*c__.offset(i__2 as isize)).i = q__1.i
                    } else {
                        i__2 = i__ + j * c_dim1;
                        i__3 = i__ + j * c_dim1;
                        q__3.r =
                            (*beta).r * (*c__.offset(i__3 as isize)).r -
                                (*beta).i * (*c__.offset(i__3 as isize)).i;
                        q__3.i =
                            (*beta).r * (*c__.offset(i__3 as isize)).i +
                                (*beta).i * (*c__.offset(i__3 as isize)).r;
                        i__4 = i__ + i__ * a_dim1;
                        q__4.r =
                            temp1.r * (*a.offset(i__4 as isize)).r -
                                temp1.i * (*a.offset(i__4 as isize)).i;
                        q__4.i =
                            temp1.r * (*a.offset(i__4 as isize)).i +
                                temp1.i * (*a.offset(i__4 as isize)).r;
                        q__2.r = q__3.r + q__4.r;
                        q__2.i = q__3.i + q__4.i;
                        q__5.r = (*alpha).r * temp2.r - (*alpha).i * temp2.i;
                        q__5.i = (*alpha).r * temp2.i + (*alpha).i * temp2.r;
                        q__1.r = q__2.r + q__5.r;
                        q__1.i = q__2.i + q__5.i;
                        (*c__.offset(i__2 as isize)).r = q__1.r;
                        (*c__.offset(i__2 as isize)).i = q__1.i
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
            i__2 = j + j * a_dim1;
            q__1.r =
                (*alpha).r * (*a.offset(i__2 as isize)).r -
                    (*alpha).i * (*a.offset(i__2 as isize)).i;
            q__1.i =
                (*alpha).r * (*a.offset(i__2 as isize)).i +
                    (*alpha).i * (*a.offset(i__2 as isize)).r;
            temp1.r = q__1.r;
            temp1.i = q__1.i;
            if (*beta).r == 0.0f32 && (*beta).i == 0.0f32 {
                i__2 = *m;
                i__ = 1 as libc::c_int as integer;
                while i__ <= i__2 {
                    i__3 = i__ + j * c_dim1;
                    i__4 = i__ + j * b_dim1;
                    q__1.r =
                        temp1.r * (*b.offset(i__4 as isize)).r -
                            temp1.i * (*b.offset(i__4 as isize)).i;
                    q__1.i =
                        temp1.r * (*b.offset(i__4 as isize)).i +
                            temp1.i * (*b.offset(i__4 as isize)).r;
                    (*c__.offset(i__3 as isize)).r = q__1.r;
                    (*c__.offset(i__3 as isize)).i = q__1.i;
                    i__ += 1
                    /* L170: */
                    /* L110: */
                }
            } else {
                i__2 = *m;
                i__ = 1 as libc::c_int as integer;
                while i__ <= i__2 {
                    i__3 = i__ + j * c_dim1;
                    i__4 = i__ + j * c_dim1;
                    q__2.r =
                        (*beta).r * (*c__.offset(i__4 as isize)).r -
                            (*beta).i * (*c__.offset(i__4 as isize)).i;
                    q__2.i =
                        (*beta).r * (*c__.offset(i__4 as isize)).i +
                            (*beta).i * (*c__.offset(i__4 as isize)).r;
                    i__5 = i__ + j * b_dim1;
                    q__3.r =
                        temp1.r * (*b.offset(i__5 as isize)).r -
                            temp1.i * (*b.offset(i__5 as isize)).i;
                    q__3.i =
                        temp1.r * (*b.offset(i__5 as isize)).i +
                            temp1.i * (*b.offset(i__5 as isize)).r;
                    q__1.r = q__2.r + q__3.r;
                    q__1.i = q__2.i + q__3.i;
                    (*c__.offset(i__3 as isize)).r = q__1.r;
                    (*c__.offset(i__3 as isize)).i = q__1.i;
                    i__ += 1
                    /* L120: */
                }
            }
            i__2 = j - 1 as libc::c_int as libc::c_long;
            k = 1 as libc::c_int as integer;
            while k <= i__2 {
                if upper != 0 {
                    i__3 = k + j * a_dim1;
                    q__1.r =
                        (*alpha).r * (*a.offset(i__3 as isize)).r -
                            (*alpha).i * (*a.offset(i__3 as isize)).i;
                    q__1.i =
                        (*alpha).r * (*a.offset(i__3 as isize)).i +
                            (*alpha).i * (*a.offset(i__3 as isize)).r;
                    temp1.r = q__1.r;
                    temp1.i = q__1.i
                } else {
                    i__3 = j + k * a_dim1;
                    q__1.r =
                        (*alpha).r * (*a.offset(i__3 as isize)).r -
                            (*alpha).i * (*a.offset(i__3 as isize)).i;
                    q__1.i =
                        (*alpha).r * (*a.offset(i__3 as isize)).i +
                            (*alpha).i * (*a.offset(i__3 as isize)).r;
                    temp1.r = q__1.r;
                    temp1.i = q__1.i
                }
                i__3 = *m;
                i__ = 1 as libc::c_int as integer;
                while i__ <= i__3 {
                    i__4 = i__ + j * c_dim1;
                    i__5 = i__ + j * c_dim1;
                    i__6 = i__ + k * b_dim1;
                    q__2.r =
                        temp1.r * (*b.offset(i__6 as isize)).r -
                            temp1.i * (*b.offset(i__6 as isize)).i;
                    q__2.i =
                        temp1.r * (*b.offset(i__6 as isize)).i +
                            temp1.i * (*b.offset(i__6 as isize)).r;
                    q__1.r = (*c__.offset(i__5 as isize)).r + q__2.r;
                    q__1.i = (*c__.offset(i__5 as isize)).i + q__2.i;
                    (*c__.offset(i__4 as isize)).r = q__1.r;
                    (*c__.offset(i__4 as isize)).i = q__1.i;
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
                    i__3 = j + k * a_dim1;
                    q__1.r =
                        (*alpha).r * (*a.offset(i__3 as isize)).r -
                            (*alpha).i * (*a.offset(i__3 as isize)).i;
                    q__1.i =
                        (*alpha).r * (*a.offset(i__3 as isize)).i +
                            (*alpha).i * (*a.offset(i__3 as isize)).r;
                    temp1.r = q__1.r;
                    temp1.i = q__1.i
                } else {
                    i__3 = k + j * a_dim1;
                    q__1.r =
                        (*alpha).r * (*a.offset(i__3 as isize)).r -
                            (*alpha).i * (*a.offset(i__3 as isize)).i;
                    q__1.i =
                        (*alpha).r * (*a.offset(i__3 as isize)).i +
                            (*alpha).i * (*a.offset(i__3 as isize)).r;
                    temp1.r = q__1.r;
                    temp1.i = q__1.i
                }
                i__3 = *m;
                i__ = 1 as libc::c_int as integer;
                while i__ <= i__3 {
                    i__4 = i__ + j * c_dim1;
                    i__5 = i__ + j * c_dim1;
                    i__6 = i__ + k * b_dim1;
                    q__2.r =
                        temp1.r * (*b.offset(i__6 as isize)).r -
                            temp1.i * (*b.offset(i__6 as isize)).i;
                    q__2.i =
                        temp1.r * (*b.offset(i__6 as isize)).i +
                            temp1.i * (*b.offset(i__6 as isize)).r;
                    q__1.r = (*c__.offset(i__5 as isize)).r + q__2.r;
                    q__1.i = (*c__.offset(i__5 as isize)).i + q__2.i;
                    (*c__.offset(i__4 as isize)).r = q__1.r;
                    (*c__.offset(i__4 as isize)).i = q__1.i;
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
    /*     End of CSYMM . */
}
/* csymm_ */
