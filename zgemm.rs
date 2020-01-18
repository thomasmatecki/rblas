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
/* zgemm.f -- translated by f2c (version 20061008).
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
pub unsafe extern "C" fn f2c_zgemm(
    mut transa: *mut libc::c_char,
    mut transb: *mut libc::c_char,
    mut m: *mut integer,
    mut n: *mut integer,
    mut k: *mut integer,
    mut alpha: *mut doublecomplex,
    mut a: *mut doublecomplex,
    mut lda: *mut integer,
    mut b: *mut doublecomplex,
    mut ldb: *mut integer,
    mut beta: *mut doublecomplex,
    mut c__: *mut doublecomplex,
    mut ldc: *mut integer,
) -> libc::c_int {
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
    let mut z__1: doublecomplex = doublecomplex { r: 0., i: 0. };
    let mut z__2: doublecomplex = doublecomplex { r: 0., i: 0. };
    let mut z__3: doublecomplex = doublecomplex { r: 0., i: 0. };
    let mut z__4: doublecomplex = doublecomplex { r: 0., i: 0. };
    /* Builtin functions */
    extern "C" {
        #[link_name = "d_cnjg"]
        fn d_cnjg_0(_: *mut doublecomplex, _: *mut doublecomplex);
    }
    /* Local variables */
    let mut i__: integer = 0;
    let mut j: integer = 0;
    let mut l: integer = 0;
    let mut info: integer = 0;
    let mut nota: logical = 0;
    let mut notb: logical = 0;
    let mut temp: doublecomplex = doublecomplex { r: 0., i: 0. };
    let mut conja: logical = 0;
    let mut conjb: logical = 0;
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
    /*  ZGEMM  performs one of the matrix-matrix operations */
    /*     C := alpha*op( A )*op( B ) + beta*C, */
    /*  where  op( X ) is one of */
    /*     op( X ) = X   or   op( X ) = X'   or   op( X ) = conjg( X' ), */
    /*  alpha and beta are scalars, and A, B and C are matrices, with op( A ) */
    /*  an m by k matrix,  op( B )  a  k by n matrix and  C an m by n matrix. */
    /*  Arguments */
    /*  ========== */
    /*  TRANSA - CHARACTER*1. */
    /*           On entry, TRANSA specifies the form of op( A ) to be used in */
    /*           the matrix multiplication as follows: */
    /*              TRANSA = 'N' or 'n',  op( A ) = A. */
    /*              TRANSA = 'T' or 't',  op( A ) = A'. */
    /*              TRANSA = 'C' or 'c',  op( A ) = conjg( A' ). */
    /*           Unchanged on exit. */
    /*  TRANSB - CHARACTER*1. */
    /*           On entry, TRANSB specifies the form of op( B ) to be used in */
    /*           the matrix multiplication as follows: */
    /*              TRANSB = 'N' or 'n',  op( B ) = B. */
    /*              TRANSB = 'T' or 't',  op( B ) = B'. */
    /*              TRANSB = 'C' or 'c',  op( B ) = conjg( B' ). */
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
    /*  ALPHA  - COMPLEX*16      . */
    /*           On entry, ALPHA specifies the scalar alpha. */
    /*           Unchanged on exit. */
    /*  A      - COMPLEX*16       array of DIMENSION ( LDA, ka ), where ka is */
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
    /*  B      - COMPLEX*16       array of DIMENSION ( LDB, kb ), where kb is */
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
    /*  BETA   - COMPLEX*16      . */
    /*           On entry,  BETA  specifies the scalar  beta.  When  BETA  is */
    /*           supplied as zero then C need not be set on input. */
    /*           Unchanged on exit. */
    /*  C      - COMPLEX*16       array of DIMENSION ( LDC, n ). */
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
    /*     conjugated or transposed, set  CONJA and CONJB  as true if  A  and */
    /*     B  respectively are to be  transposed but  not conjugated  and set */
    /*     NROWA, NCOLA and  NROWB  as the number of rows and  columns  of  A */
    /*     and the number of rows of  B  respectively. */
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
    nota = lsame__0(
        transa,
        b"N\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    notb = lsame__0(
        transb,
        b"N\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    conja = lsame__0(
        transa,
        b"C\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    conjb = lsame__0(
        transb,
        b"C\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if nota != 0 {
        nrowa = *m;
        ncola = *k
    } else {
        nrowa = *k;
        ncola = *m
    }
    if notb != 0 {
        nrowb = *k
    } else {
        nrowb = *n
    }
    /*     Test the input parameters. */
    info = 0 as libc::c_int as integer;
    if nota == 0
        && conja == 0
        && lsame__0(
            transa,
            b"T\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == 0
    {
        info = 1 as libc::c_int as integer
    } else if notb == 0
        && conjb == 0
        && lsame__0(
            transb,
            b"T\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == 0
    {
        info = 2 as libc::c_int as integer
    } else if *m < 0 as libc::c_int as libc::c_long {
        info = 3 as libc::c_int as integer
    } else if *n < 0 as libc::c_int as libc::c_long {
        info = 4 as libc::c_int as integer
    } else if *k < 0 as libc::c_int as libc::c_long {
        info = 5 as libc::c_int as integer
    } else if *lda
        < (if 1 as libc::c_int as libc::c_long >= nrowa {
            1 as libc::c_int as libc::c_long
        } else {
            nrowa
        })
    {
        info = 8 as libc::c_int as integer
    } else if *ldb
        < (if 1 as libc::c_int as libc::c_long >= nrowb {
            1 as libc::c_int as libc::c_long
        } else {
            nrowb
        })
    {
        info = 10 as libc::c_int as integer
    } else if *ldc
        < (if 1 as libc::c_int as libc::c_long >= *m {
            1 as libc::c_int as libc::c_long
        } else {
            *m
        })
    {
        info = 13 as libc::c_int as integer
    }
    if info != 0 as libc::c_int as libc::c_long {
        xerbla__0(
            b"ZGEMM \x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            &mut info,
        );
        return 0 as libc::c_int;
    }
    /*     Quick return if possible. */
    if *m == 0 as libc::c_int as libc::c_long
        || *n == 0 as libc::c_int as libc::c_long
        || ((*alpha).r == 0.0f64 && (*alpha).i == 0.0f64 || *k == 0 as libc::c_int as libc::c_long)
            && ((*beta).r == 1.0f64 && (*beta).i == 0.0f64)
    {
        return 0 as libc::c_int;
    }
    /*     And when  alpha.eq.zero. */
    if (*alpha).r == 0.0f64 && (*alpha).i == 0.0f64 {
        if (*beta).r == 0.0f64 && (*beta).i == 0.0f64 {
            i__1 = *n;
            j = 1 as libc::c_int as integer;
            while j <= i__1 {
                i__2 = *m;
                i__ = 1 as libc::c_int as integer;
                while i__ <= i__2 {
                    i__3 = i__ + j * c_dim1;
                    (*c__.offset(i__3 as isize)).r = 0.0f64;
                    (*c__.offset(i__3 as isize)).i = 0.0f64;
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
                    z__1.r = (*beta).r * (*c__.offset(i__4 as isize)).r
                        - (*beta).i * (*c__.offset(i__4 as isize)).i;
                    z__1.i = (*beta).r * (*c__.offset(i__4 as isize)).i
                        + (*beta).i * (*c__.offset(i__4 as isize)).r;
                    (*c__.offset(i__3 as isize)).r = z__1.r;
                    (*c__.offset(i__3 as isize)).i = z__1.i;
                    i__ += 1
                    /* L40: */
                    /* L30: */
                }
                j += 1
            }
        }
        return 0 as libc::c_int;
    }
    /*     Start the operations. */
    if notb != 0 {
        if nota != 0 {
            /*           Form  C := alpha*A*B + beta*C. */
            i__1 = *n;
            j = 1 as libc::c_int as integer;
            while j <= i__1 {
                if (*beta).r == 0.0f64 && (*beta).i == 0.0f64 {
                    i__2 = *m;
                    i__ = 1 as libc::c_int as integer;
                    while i__ <= i__2 {
                        i__3 = i__ + j * c_dim1;
                        (*c__.offset(i__3 as isize)).r = 0.0f64;
                        (*c__.offset(i__3 as isize)).i = 0.0f64;
                        i__ += 1
                        /* L50: */
                    }
                } else if (*beta).r != 1.0f64 || (*beta).i != 0.0f64 {
                    i__2 = *m;
                    i__ = 1 as libc::c_int as integer;
                    while i__ <= i__2 {
                        i__3 = i__ + j * c_dim1;
                        i__4 = i__ + j * c_dim1;
                        z__1.r = (*beta).r * (*c__.offset(i__4 as isize)).r
                            - (*beta).i * (*c__.offset(i__4 as isize)).i;
                        z__1.i = (*beta).r * (*c__.offset(i__4 as isize)).i
                            + (*beta).i * (*c__.offset(i__4 as isize)).r;
                        (*c__.offset(i__3 as isize)).r = z__1.r;
                        (*c__.offset(i__3 as isize)).i = z__1.i;
                        i__ += 1
                        /* L60: */
                    }
                }
                i__2 = *k;
                l = 1 as libc::c_int as integer;
                while l <= i__2 {
                    i__3 = l + j * b_dim1;
                    if (*b.offset(i__3 as isize)).r != 0.0f64
                        || (*b.offset(i__3 as isize)).i != 0.0f64
                    {
                        i__3 = l + j * b_dim1;
                        z__1.r = (*alpha).r * (*b.offset(i__3 as isize)).r
                            - (*alpha).i * (*b.offset(i__3 as isize)).i;
                        z__1.i = (*alpha).r * (*b.offset(i__3 as isize)).i
                            + (*alpha).i * (*b.offset(i__3 as isize)).r;
                        temp.r = z__1.r;
                        temp.i = z__1.i;
                        i__3 = *m;
                        i__ = 1 as libc::c_int as integer;
                        while i__ <= i__3 {
                            i__4 = i__ + j * c_dim1;
                            i__5 = i__ + j * c_dim1;
                            i__6 = i__ + l * a_dim1;
                            z__2.r = temp.r * (*a.offset(i__6 as isize)).r
                                - temp.i * (*a.offset(i__6 as isize)).i;
                            z__2.i = temp.r * (*a.offset(i__6 as isize)).i
                                + temp.i * (*a.offset(i__6 as isize)).r;
                            z__1.r = (*c__.offset(i__5 as isize)).r + z__2.r;
                            z__1.i = (*c__.offset(i__5 as isize)).i + z__2.i;
                            (*c__.offset(i__4 as isize)).r = z__1.r;
                            (*c__.offset(i__4 as isize)).i = z__1.i;
                            i__ += 1
                            /* L90: */
                            /* L80: */
                            /* L70: */
                        }
                    }
                    l += 1
                }
                j += 1
            }
        } else if conja != 0 {
            /*           Form  C := alpha*conjg( A' )*B + beta*C. */
            i__1 = *n;
            j = 1 as libc::c_int as integer;
            while j <= i__1 {
                i__2 = *m;
                i__ = 1 as libc::c_int as integer;
                while i__ <= i__2 {
                    temp.r = 0.0f64;
                    temp.i = 0.0f64;
                    i__3 = *k;
                    l = 1 as libc::c_int as integer;
                    while l <= i__3 {
                        d_cnjg_0(&mut z__3, &mut *a.offset((l + i__ * a_dim1) as isize));
                        i__4 = l + j * b_dim1;
                        z__2.r = z__3.r * (*b.offset(i__4 as isize)).r
                            - z__3.i * (*b.offset(i__4 as isize)).i;
                        z__2.i = z__3.r * (*b.offset(i__4 as isize)).i
                            + z__3.i * (*b.offset(i__4 as isize)).r;
                        z__1.r = temp.r + z__2.r;
                        z__1.i = temp.i + z__2.i;
                        temp.r = z__1.r;
                        temp.i = z__1.i;
                        l += 1
                        /* L120: */
                        /* L110: */
                        /* L100: */
                    }
                    if (*beta).r == 0.0f64 && (*beta).i == 0.0f64 {
                        i__3 = i__ + j * c_dim1;
                        z__1.r = (*alpha).r * temp.r - (*alpha).i * temp.i;
                        z__1.i = (*alpha).r * temp.i + (*alpha).i * temp.r;
                        (*c__.offset(i__3 as isize)).r = z__1.r;
                        (*c__.offset(i__3 as isize)).i = z__1.i
                    } else {
                        i__3 = i__ + j * c_dim1;
                        z__2.r = (*alpha).r * temp.r - (*alpha).i * temp.i;
                        z__2.i = (*alpha).r * temp.i + (*alpha).i * temp.r;
                        i__4 = i__ + j * c_dim1;
                        z__3.r = (*beta).r * (*c__.offset(i__4 as isize)).r
                            - (*beta).i * (*c__.offset(i__4 as isize)).i;
                        z__3.i = (*beta).r * (*c__.offset(i__4 as isize)).i
                            + (*beta).i * (*c__.offset(i__4 as isize)).r;
                        z__1.r = z__2.r + z__3.r;
                        z__1.i = z__2.i + z__3.i;
                        (*c__.offset(i__3 as isize)).r = z__1.r;
                        (*c__.offset(i__3 as isize)).i = z__1.i
                    }
                    i__ += 1
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
                    temp.r = 0.0f64;
                    temp.i = 0.0f64;
                    i__3 = *k;
                    l = 1 as libc::c_int as integer;
                    while l <= i__3 {
                        i__4 = l + i__ * a_dim1;
                        i__5 = l + j * b_dim1;
                        z__2.r = (*a.offset(i__4 as isize)).r * (*b.offset(i__5 as isize)).r
                            - (*a.offset(i__4 as isize)).i * (*b.offset(i__5 as isize)).i;
                        z__2.i = (*a.offset(i__4 as isize)).r * (*b.offset(i__5 as isize)).i
                            + (*a.offset(i__4 as isize)).i * (*b.offset(i__5 as isize)).r;
                        z__1.r = temp.r + z__2.r;
                        z__1.i = temp.i + z__2.i;
                        temp.r = z__1.r;
                        temp.i = z__1.i;
                        l += 1
                        /* L150: */
                        /* L140: */
                        /* L130: */
                    }
                    if (*beta).r == 0.0f64 && (*beta).i == 0.0f64 {
                        i__3 = i__ + j * c_dim1;
                        z__1.r = (*alpha).r * temp.r - (*alpha).i * temp.i;
                        z__1.i = (*alpha).r * temp.i + (*alpha).i * temp.r;
                        (*c__.offset(i__3 as isize)).r = z__1.r;
                        (*c__.offset(i__3 as isize)).i = z__1.i
                    } else {
                        i__3 = i__ + j * c_dim1;
                        z__2.r = (*alpha).r * temp.r - (*alpha).i * temp.i;
                        z__2.i = (*alpha).r * temp.i + (*alpha).i * temp.r;
                        i__4 = i__ + j * c_dim1;
                        z__3.r = (*beta).r * (*c__.offset(i__4 as isize)).r
                            - (*beta).i * (*c__.offset(i__4 as isize)).i;
                        z__3.i = (*beta).r * (*c__.offset(i__4 as isize)).i
                            + (*beta).i * (*c__.offset(i__4 as isize)).r;
                        z__1.r = z__2.r + z__3.r;
                        z__1.i = z__2.i + z__3.i;
                        (*c__.offset(i__3 as isize)).r = z__1.r;
                        (*c__.offset(i__3 as isize)).i = z__1.i
                    }
                    i__ += 1
                }
                j += 1
            }
        }
    } else if nota != 0 {
        if conjb != 0 {
            /*           Form  C := alpha*A*conjg( B' ) + beta*C. */
            i__1 = *n;
            j = 1 as libc::c_int as integer;
            while j <= i__1 {
                if (*beta).r == 0.0f64 && (*beta).i == 0.0f64 {
                    i__2 = *m;
                    i__ = 1 as libc::c_int as integer;
                    while i__ <= i__2 {
                        i__3 = i__ + j * c_dim1;
                        (*c__.offset(i__3 as isize)).r = 0.0f64;
                        (*c__.offset(i__3 as isize)).i = 0.0f64;
                        i__ += 1
                        /* L160: */
                    }
                } else if (*beta).r != 1.0f64 || (*beta).i != 0.0f64 {
                    i__2 = *m;
                    i__ = 1 as libc::c_int as integer;
                    while i__ <= i__2 {
                        i__3 = i__ + j * c_dim1;
                        i__4 = i__ + j * c_dim1;
                        z__1.r = (*beta).r * (*c__.offset(i__4 as isize)).r
                            - (*beta).i * (*c__.offset(i__4 as isize)).i;
                        z__1.i = (*beta).r * (*c__.offset(i__4 as isize)).i
                            + (*beta).i * (*c__.offset(i__4 as isize)).r;
                        (*c__.offset(i__3 as isize)).r = z__1.r;
                        (*c__.offset(i__3 as isize)).i = z__1.i;
                        i__ += 1
                        /* L170: */
                    }
                }
                i__2 = *k;
                l = 1 as libc::c_int as integer;
                while l <= i__2 {
                    i__3 = j + l * b_dim1;
                    if (*b.offset(i__3 as isize)).r != 0.0f64
                        || (*b.offset(i__3 as isize)).i != 0.0f64
                    {
                        d_cnjg_0(&mut z__2, &mut *b.offset((j + l * b_dim1) as isize));
                        z__1.r = (*alpha).r * z__2.r - (*alpha).i * z__2.i;
                        z__1.i = (*alpha).r * z__2.i + (*alpha).i * z__2.r;
                        temp.r = z__1.r;
                        temp.i = z__1.i;
                        i__3 = *m;
                        i__ = 1 as libc::c_int as integer;
                        while i__ <= i__3 {
                            i__4 = i__ + j * c_dim1;
                            i__5 = i__ + j * c_dim1;
                            i__6 = i__ + l * a_dim1;
                            z__2.r = temp.r * (*a.offset(i__6 as isize)).r
                                - temp.i * (*a.offset(i__6 as isize)).i;
                            z__2.i = temp.r * (*a.offset(i__6 as isize)).i
                                + temp.i * (*a.offset(i__6 as isize)).r;
                            z__1.r = (*c__.offset(i__5 as isize)).r + z__2.r;
                            z__1.i = (*c__.offset(i__5 as isize)).i + z__2.i;
                            (*c__.offset(i__4 as isize)).r = z__1.r;
                            (*c__.offset(i__4 as isize)).i = z__1.i;
                            i__ += 1
                            /* L200: */
                            /* L190: */
                            /* L180: */
                        }
                    }
                    l += 1
                }
                j += 1
            }
        } else {
            /*           Form  C := alpha*A*B'          + beta*C */
            i__1 = *n;
            j = 1 as libc::c_int as integer;
            while j <= i__1 {
                if (*beta).r == 0.0f64 && (*beta).i == 0.0f64 {
                    i__2 = *m;
                    i__ = 1 as libc::c_int as integer;
                    while i__ <= i__2 {
                        i__3 = i__ + j * c_dim1;
                        (*c__.offset(i__3 as isize)).r = 0.0f64;
                        (*c__.offset(i__3 as isize)).i = 0.0f64;
                        i__ += 1
                        /* L210: */
                    }
                } else if (*beta).r != 1.0f64 || (*beta).i != 0.0f64 {
                    i__2 = *m;
                    i__ = 1 as libc::c_int as integer;
                    while i__ <= i__2 {
                        i__3 = i__ + j * c_dim1;
                        i__4 = i__ + j * c_dim1;
                        z__1.r = (*beta).r * (*c__.offset(i__4 as isize)).r
                            - (*beta).i * (*c__.offset(i__4 as isize)).i;
                        z__1.i = (*beta).r * (*c__.offset(i__4 as isize)).i
                            + (*beta).i * (*c__.offset(i__4 as isize)).r;
                        (*c__.offset(i__3 as isize)).r = z__1.r;
                        (*c__.offset(i__3 as isize)).i = z__1.i;
                        i__ += 1
                        /* L220: */
                    }
                }
                i__2 = *k;
                l = 1 as libc::c_int as integer;
                while l <= i__2 {
                    i__3 = j + l * b_dim1;
                    if (*b.offset(i__3 as isize)).r != 0.0f64
                        || (*b.offset(i__3 as isize)).i != 0.0f64
                    {
                        i__3 = j + l * b_dim1;
                        z__1.r = (*alpha).r * (*b.offset(i__3 as isize)).r
                            - (*alpha).i * (*b.offset(i__3 as isize)).i;
                        z__1.i = (*alpha).r * (*b.offset(i__3 as isize)).i
                            + (*alpha).i * (*b.offset(i__3 as isize)).r;
                        temp.r = z__1.r;
                        temp.i = z__1.i;
                        i__3 = *m;
                        i__ = 1 as libc::c_int as integer;
                        while i__ <= i__3 {
                            i__4 = i__ + j * c_dim1;
                            i__5 = i__ + j * c_dim1;
                            i__6 = i__ + l * a_dim1;
                            z__2.r = temp.r * (*a.offset(i__6 as isize)).r
                                - temp.i * (*a.offset(i__6 as isize)).i;
                            z__2.i = temp.r * (*a.offset(i__6 as isize)).i
                                + temp.i * (*a.offset(i__6 as isize)).r;
                            z__1.r = (*c__.offset(i__5 as isize)).r + z__2.r;
                            z__1.i = (*c__.offset(i__5 as isize)).i + z__2.i;
                            (*c__.offset(i__4 as isize)).r = z__1.r;
                            (*c__.offset(i__4 as isize)).i = z__1.i;
                            i__ += 1
                            /* L250: */
                            /* L240: */
                            /* L230: */
                        }
                    }
                    l += 1
                }
                j += 1
            }
        }
    } else if conja != 0 {
        if conjb != 0 {
            /*           Form  C := alpha*conjg( A' )*conjg( B' ) + beta*C. */
            i__1 = *n;
            j = 1 as libc::c_int as integer;
            while j <= i__1 {
                i__2 = *m;
                i__ = 1 as libc::c_int as integer;
                while i__ <= i__2 {
                    temp.r = 0.0f64;
                    temp.i = 0.0f64;
                    i__3 = *k;
                    l = 1 as libc::c_int as integer;
                    while l <= i__3 {
                        d_cnjg_0(&mut z__3, &mut *a.offset((l + i__ * a_dim1) as isize));
                        d_cnjg_0(&mut z__4, &mut *b.offset((j + l * b_dim1) as isize));
                        z__2.r = z__3.r * z__4.r - z__3.i * z__4.i;
                        z__2.i = z__3.r * z__4.i + z__3.i * z__4.r;
                        z__1.r = temp.r + z__2.r;
                        z__1.i = temp.i + z__2.i;
                        temp.r = z__1.r;
                        temp.i = z__1.i;
                        l += 1
                        /* L280: */
                        /* L270: */
                        /* L260: */
                    }
                    if (*beta).r == 0.0f64 && (*beta).i == 0.0f64 {
                        i__3 = i__ + j * c_dim1;
                        z__1.r = (*alpha).r * temp.r - (*alpha).i * temp.i;
                        z__1.i = (*alpha).r * temp.i + (*alpha).i * temp.r;
                        (*c__.offset(i__3 as isize)).r = z__1.r;
                        (*c__.offset(i__3 as isize)).i = z__1.i
                    } else {
                        i__3 = i__ + j * c_dim1;
                        z__2.r = (*alpha).r * temp.r - (*alpha).i * temp.i;
                        z__2.i = (*alpha).r * temp.i + (*alpha).i * temp.r;
                        i__4 = i__ + j * c_dim1;
                        z__3.r = (*beta).r * (*c__.offset(i__4 as isize)).r
                            - (*beta).i * (*c__.offset(i__4 as isize)).i;
                        z__3.i = (*beta).r * (*c__.offset(i__4 as isize)).i
                            + (*beta).i * (*c__.offset(i__4 as isize)).r;
                        z__1.r = z__2.r + z__3.r;
                        z__1.i = z__2.i + z__3.i;
                        (*c__.offset(i__3 as isize)).r = z__1.r;
                        (*c__.offset(i__3 as isize)).i = z__1.i
                    }
                    i__ += 1
                }
                j += 1
            }
        } else {
            /*           Form  C := alpha*conjg( A' )*B' + beta*C */
            i__1 = *n;
            j = 1 as libc::c_int as integer;
            while j <= i__1 {
                i__2 = *m;
                i__ = 1 as libc::c_int as integer;
                while i__ <= i__2 {
                    temp.r = 0.0f64;
                    temp.i = 0.0f64;
                    i__3 = *k;
                    l = 1 as libc::c_int as integer;
                    while l <= i__3 {
                        d_cnjg_0(&mut z__3, &mut *a.offset((l + i__ * a_dim1) as isize));
                        i__4 = j + l * b_dim1;
                        z__2.r = z__3.r * (*b.offset(i__4 as isize)).r
                            - z__3.i * (*b.offset(i__4 as isize)).i;
                        z__2.i = z__3.r * (*b.offset(i__4 as isize)).i
                            + z__3.i * (*b.offset(i__4 as isize)).r;
                        z__1.r = temp.r + z__2.r;
                        z__1.i = temp.i + z__2.i;
                        temp.r = z__1.r;
                        temp.i = z__1.i;
                        l += 1
                        /* L310: */
                        /* L300: */
                        /* L290: */
                    }
                    if (*beta).r == 0.0f64 && (*beta).i == 0.0f64 {
                        i__3 = i__ + j * c_dim1;
                        z__1.r = (*alpha).r * temp.r - (*alpha).i * temp.i;
                        z__1.i = (*alpha).r * temp.i + (*alpha).i * temp.r;
                        (*c__.offset(i__3 as isize)).r = z__1.r;
                        (*c__.offset(i__3 as isize)).i = z__1.i
                    } else {
                        i__3 = i__ + j * c_dim1;
                        z__2.r = (*alpha).r * temp.r - (*alpha).i * temp.i;
                        z__2.i = (*alpha).r * temp.i + (*alpha).i * temp.r;
                        i__4 = i__ + j * c_dim1;
                        z__3.r = (*beta).r * (*c__.offset(i__4 as isize)).r
                            - (*beta).i * (*c__.offset(i__4 as isize)).i;
                        z__3.i = (*beta).r * (*c__.offset(i__4 as isize)).i
                            + (*beta).i * (*c__.offset(i__4 as isize)).r;
                        z__1.r = z__2.r + z__3.r;
                        z__1.i = z__2.i + z__3.i;
                        (*c__.offset(i__3 as isize)).r = z__1.r;
                        (*c__.offset(i__3 as isize)).i = z__1.i
                    }
                    i__ += 1
                }
                j += 1
            }
        }
    } else if conjb != 0 {
        /*           Form  C := alpha*A'*conjg( B' ) + beta*C */
        i__1 = *n;
        j = 1 as libc::c_int as integer;
        while j <= i__1 {
            i__2 = *m;
            i__ = 1 as libc::c_int as integer;
            while i__ <= i__2 {
                temp.r = 0.0f64;
                temp.i = 0.0f64;
                i__3 = *k;
                l = 1 as libc::c_int as integer;
                while l <= i__3 {
                    i__4 = l + i__ * a_dim1;
                    d_cnjg_0(&mut z__3, &mut *b.offset((j + l * b_dim1) as isize));
                    z__2.r = (*a.offset(i__4 as isize)).r * z__3.r
                        - (*a.offset(i__4 as isize)).i * z__3.i;
                    z__2.i = (*a.offset(i__4 as isize)).r * z__3.i
                        + (*a.offset(i__4 as isize)).i * z__3.r;
                    z__1.r = temp.r + z__2.r;
                    z__1.i = temp.i + z__2.i;
                    temp.r = z__1.r;
                    temp.i = z__1.i;
                    l += 1
                    /* L340: */
                    /* L330: */
                    /* L320: */
                }
                if (*beta).r == 0.0f64 && (*beta).i == 0.0f64 {
                    i__3 = i__ + j * c_dim1;
                    z__1.r = (*alpha).r * temp.r - (*alpha).i * temp.i;
                    z__1.i = (*alpha).r * temp.i + (*alpha).i * temp.r;
                    (*c__.offset(i__3 as isize)).r = z__1.r;
                    (*c__.offset(i__3 as isize)).i = z__1.i
                } else {
                    i__3 = i__ + j * c_dim1;
                    z__2.r = (*alpha).r * temp.r - (*alpha).i * temp.i;
                    z__2.i = (*alpha).r * temp.i + (*alpha).i * temp.r;
                    i__4 = i__ + j * c_dim1;
                    z__3.r = (*beta).r * (*c__.offset(i__4 as isize)).r
                        - (*beta).i * (*c__.offset(i__4 as isize)).i;
                    z__3.i = (*beta).r * (*c__.offset(i__4 as isize)).i
                        + (*beta).i * (*c__.offset(i__4 as isize)).r;
                    z__1.r = z__2.r + z__3.r;
                    z__1.i = z__2.i + z__3.i;
                    (*c__.offset(i__3 as isize)).r = z__1.r;
                    (*c__.offset(i__3 as isize)).i = z__1.i
                }
                i__ += 1
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
                temp.r = 0.0f64;
                temp.i = 0.0f64;
                i__3 = *k;
                l = 1 as libc::c_int as integer;
                while l <= i__3 {
                    i__4 = l + i__ * a_dim1;
                    i__5 = j + l * b_dim1;
                    z__2.r = (*a.offset(i__4 as isize)).r * (*b.offset(i__5 as isize)).r
                        - (*a.offset(i__4 as isize)).i * (*b.offset(i__5 as isize)).i;
                    z__2.i = (*a.offset(i__4 as isize)).r * (*b.offset(i__5 as isize)).i
                        + (*a.offset(i__4 as isize)).i * (*b.offset(i__5 as isize)).r;
                    z__1.r = temp.r + z__2.r;
                    z__1.i = temp.i + z__2.i;
                    temp.r = z__1.r;
                    temp.i = z__1.i;
                    l += 1
                    /* L370: */
                    /* L360: */
                    /* L350: */
                }
                if (*beta).r == 0.0f64 && (*beta).i == 0.0f64 {
                    i__3 = i__ + j * c_dim1;
                    z__1.r = (*alpha).r * temp.r - (*alpha).i * temp.i;
                    z__1.i = (*alpha).r * temp.i + (*alpha).i * temp.r;
                    (*c__.offset(i__3 as isize)).r = z__1.r;
                    (*c__.offset(i__3 as isize)).i = z__1.i
                } else {
                    i__3 = i__ + j * c_dim1;
                    z__2.r = (*alpha).r * temp.r - (*alpha).i * temp.i;
                    z__2.i = (*alpha).r * temp.i + (*alpha).i * temp.r;
                    i__4 = i__ + j * c_dim1;
                    z__3.r = (*beta).r * (*c__.offset(i__4 as isize)).r
                        - (*beta).i * (*c__.offset(i__4 as isize)).i;
                    z__3.i = (*beta).r * (*c__.offset(i__4 as isize)).i
                        + (*beta).i * (*c__.offset(i__4 as isize)).r;
                    z__1.r = z__2.r + z__3.r;
                    z__1.i = z__2.i + z__3.i;
                    (*c__.offset(i__3 as isize)).r = z__1.r;
                    (*c__.offset(i__3 as isize)).i = z__1.i
                }
                i__ += 1
            }
            j += 1
        }
    }
    return 0 as libc::c_int;
    /*     End of ZGEMM . */
}
/* zgemm_ */
