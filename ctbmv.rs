use libc;
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
/* ctbmv.f -- translated by f2c (version 20061008).
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
pub unsafe extern "C" fn f2c_ctbmv(
    mut uplo: *mut libc::c_char,
    mut trans: *mut libc::c_char,
    mut diag: *mut libc::c_char,
    mut n: *mut integer,
    mut k: *mut integer,
    mut a: *mut complex,
    mut lda: *mut integer,
    mut x: *mut complex,
    mut incx: *mut integer,
) -> libc::c_int {
    /* System generated locals */
    let mut a_dim1: integer = 0;
    let mut a_offset: integer = 0;
    let mut i__1: integer = 0;
    let mut i__2: integer = 0;
    let mut i__3: integer = 0;
    let mut i__4: integer = 0;
    let mut i__5: integer = 0;
    let mut q__1: complex = complex { r: 0., i: 0. };
    let mut q__2: complex = complex { r: 0., i: 0. };
    let mut q__3: complex = complex { r: 0., i: 0. };
    /* Builtin functions */
    extern "C" {
        #[link_name = "r_cnjg"]
        fn r_cnjg_0(_: *mut complex, _: *mut complex);
    }
    /* Local variables */
    let mut i__: integer = 0;
    let mut j: integer = 0;
    let mut l: integer = 0;
    let mut ix: integer = 0;
    let mut jx: integer = 0;
    let mut kx: integer = 0;
    let mut info: integer = 0;
    let mut temp: complex = complex { r: 0., i: 0. };
    extern "C" {
        #[link_name = "lsame_"]
        fn lsame__0(_: *mut libc::c_char, _: *mut libc::c_char) -> logical;
    }
    let mut kplus1: integer = 0;
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
    /*  CTBMV  performs one of the matrix-vector operations */
    /*     x := A*x,   or   x := A'*x,   or   x := conjg( A' )*x, */
    /*  where x is an n element vector and  A is an n by n unit, or non-unit, */
    /*  upper or lower triangular band matrix, with ( k + 1 ) diagonals. */
    /*  Arguments */
    /*  ========== */
    /*  UPLO   - CHARACTER*1. */
    /*           On entry, UPLO specifies whether the matrix is an upper or */
    /*           lower triangular matrix as follows: */
    /*              UPLO = 'U' or 'u'   A is an upper triangular matrix. */
    /*              UPLO = 'L' or 'l'   A is a lower triangular matrix. */
    /*           Unchanged on exit. */
    /*  TRANS  - CHARACTER*1. */
    /*           On entry, TRANS specifies the operation to be performed as */
    /*           follows: */
    /*              TRANS = 'N' or 'n'   x := A*x. */
    /*              TRANS = 'T' or 't'   x := A'*x. */
    /*              TRANS = 'C' or 'c'   x := conjg( A' )*x. */
    /*           Unchanged on exit. */
    /*  DIAG   - CHARACTER*1. */
    /*           On entry, DIAG specifies whether or not A is unit */
    /*           triangular as follows: */
    /*              DIAG = 'U' or 'u'   A is assumed to be unit triangular. */
    /*              DIAG = 'N' or 'n'   A is not assumed to be unit */
    /*                                  triangular. */
    /*           Unchanged on exit. */
    /*  N      - INTEGER. */
    /*           On entry, N specifies the order of the matrix A. */
    /*           N must be at least zero. */
    /*           Unchanged on exit. */
    /*  K      - INTEGER. */
    /*           On entry with UPLO = 'U' or 'u', K specifies the number of */
    /*           super-diagonals of the matrix A. */
    /*           On entry with UPLO = 'L' or 'l', K specifies the number of */
    /*           sub-diagonals of the matrix A. */
    /*           K must satisfy  0 .le. K. */
    /*           Unchanged on exit. */
    /*  A      - COMPLEX          array of DIMENSION ( LDA, n ). */
    /*           Before entry with UPLO = 'U' or 'u', the leading ( k + 1 ) */
    /*           by n part of the array A must contain the upper triangular */
    /*           band part of the matrix of coefficients, supplied column by */
    /*           column, with the leading diagonal of the matrix in row */
    /*           ( k + 1 ) of the array, the first super-diagonal starting at */
    /*           position 2 in row k, and so on. The top left k by k triangle */
    /*           of the array A is not referenced. */
    /*           The following program segment will transfer an upper */
    /*           triangular band matrix from conventional full matrix storage */
    /*           to band storage: */
    /*                 DO 20, J = 1, N */
    /*                    M = K + 1 - J */
    /*                    DO 10, I = MAX( 1, J - K ), J */
    /*                       A( M + I, J ) = matrix( I, J ) */
    /*              10    CONTINUE */
    /*              20 CONTINUE */
    /*           Before entry with UPLO = 'L' or 'l', the leading ( k + 1 ) */
    /*           by n part of the array A must contain the lower triangular */
    /*           band part of the matrix of coefficients, supplied column by */
    /*           column, with the leading diagonal of the matrix in row 1 of */
    /*           the array, the first sub-diagonal starting at position 1 in */
    /*           row 2, and so on. The bottom right k by k triangle of the */
    /*           array A is not referenced. */
    /*           The following program segment will transfer a lower */
    /*           triangular band matrix from conventional full matrix storage */
    /*           to band storage: */
    /*                 DO 20, J = 1, N */
    /*                    M = 1 - J */
    /*                    DO 10, I = J, MIN( N, J + K ) */
    /*                       A( M + I, J ) = matrix( I, J ) */
    /*              10    CONTINUE */
    /*              20 CONTINUE */
    /*           Note that when DIAG = 'U' or 'u' the elements of the array A */
    /*           corresponding to the diagonal elements of the matrix are not */
    /*           referenced, but are assumed to be unity. */
    /*           Unchanged on exit. */
    /*  LDA    - INTEGER. */
    /*           On entry, LDA specifies the first dimension of A as declared */
    /*           in the calling (sub) program. LDA must be at least */
    /*           ( k + 1 ). */
    /*           Unchanged on exit. */
    /*  X      - COMPLEX          array of dimension at least */
    /*           ( 1 + ( n - 1 )*abs( INCX ) ). */
    /*           Before entry, the incremented array X must contain the n */
    /*           element vector x. On exit, X is overwritten with the */
    /*           tranformed vector x. */
    /*  INCX   - INTEGER. */
    /*           On entry, INCX specifies the increment for the elements of */
    /*           X. INCX must not be zero. */
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
    /*     .. External Functions .. */
    /*     .. */
    /*     .. External Subroutines .. */
    /*     .. */
    /*     .. Intrinsic Functions .. */
    /*     .. */
    /*     Test the input parameters. */
    /* Parameter adjustments */
    a_dim1 = *lda;
    a_offset = 1 as libc::c_int as libc::c_long + a_dim1;
    a = a.offset(-(a_offset as isize));
    x = x.offset(-1);
    /* Function Body */
    info = 0 as libc::c_int as integer;
    if lsame__0(
        uplo,
        b"U\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 0
        && lsame__0(
            uplo,
            b"L\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == 0
    {
        info = 1 as libc::c_int as integer
    } else if lsame__0(
        trans,
        b"N\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 0
        && lsame__0(
            trans,
            b"T\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == 0
        && lsame__0(
            trans,
            b"C\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == 0
    {
        info = 2 as libc::c_int as integer
    } else if lsame__0(
        diag,
        b"U\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 0
        && lsame__0(
            diag,
            b"N\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == 0
    {
        info = 3 as libc::c_int as integer
    } else if *n < 0 as libc::c_int as libc::c_long {
        info = 4 as libc::c_int as integer
    } else if *k < 0 as libc::c_int as libc::c_long {
        info = 5 as libc::c_int as integer
    } else if *lda < *k + 1 as libc::c_int as libc::c_long {
        info = 7 as libc::c_int as integer
    } else if *incx == 0 as libc::c_int as libc::c_long {
        info = 9 as libc::c_int as integer
    }
    if info != 0 as libc::c_int as libc::c_long {
        xerbla__0(
            b"CTBMV \x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            &mut info,
        );
        return 0 as libc::c_int;
    }
    /*     Quick return if possible. */
    if *n == 0 as libc::c_int as libc::c_long {
        return 0 as libc::c_int;
    }
    noconj = lsame__0(
        trans,
        b"T\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    nounit = lsame__0(
        diag,
        b"N\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    /*     Set up the start point in X if the increment is not unity. This */
    /*     will be  ( N - 1 )*INCX   too small for descending loops. */
    if *incx <= 0 as libc::c_int as libc::c_long {
        kx = 1 as libc::c_int as libc::c_long - (*n - 1 as libc::c_int as libc::c_long) * *incx
    } else if *incx != 1 as libc::c_int as libc::c_long {
        kx = 1 as libc::c_int as integer
    }
    /*     Start the operations. In this version the elements of A are */
    /*     accessed sequentially with one pass through A. */
    if lsame__0(
        trans,
        b"N\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) != 0
    {
        /*         Form  x := A*x. */
        if lsame__0(
            uplo,
            b"U\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) != 0
        {
            kplus1 = *k + 1 as libc::c_int as libc::c_long;
            if *incx == 1 as libc::c_int as libc::c_long {
                i__1 = *n;
                j = 1 as libc::c_int as integer;
                while j <= i__1 {
                    i__2 = j;
                    if (*x.offset(i__2 as isize)).r != 0.0f32
                        || (*x.offset(i__2 as isize)).i != 0.0f32
                    {
                        i__2 = j;
                        temp.r = (*x.offset(i__2 as isize)).r;
                        temp.i = (*x.offset(i__2 as isize)).i;
                        l = kplus1 - j;
                        /* L20: */
                        /* Computing MAX */
                        i__2 = 1 as libc::c_int as integer;
                        i__3 = j - *k;
                        i__4 = j - 1 as libc::c_int as libc::c_long;
                        i__ = if i__2 >= i__3 { i__2 } else { i__3 };
                        while i__ <= i__4 {
                            i__2 = i__;
                            i__3 = i__;
                            i__5 = l + i__ + j * a_dim1;
                            q__2.r = temp.r * (*a.offset(i__5 as isize)).r
                                - temp.i * (*a.offset(i__5 as isize)).i;
                            q__2.i = temp.r * (*a.offset(i__5 as isize)).i
                                + temp.i * (*a.offset(i__5 as isize)).r;
                            q__1.r = (*x.offset(i__3 as isize)).r + q__2.r;
                            q__1.i = (*x.offset(i__3 as isize)).i + q__2.i;
                            (*x.offset(i__2 as isize)).r = q__1.r;
                            (*x.offset(i__2 as isize)).i = q__1.i;
                            i__ += 1
                            /* L10: */
                        }
                        if nounit != 0 {
                            i__4 = j;
                            i__2 = j;
                            i__3 = kplus1 + j * a_dim1;
                            q__1.r = (*x.offset(i__2 as isize)).r * (*a.offset(i__3 as isize)).r
                                - (*x.offset(i__2 as isize)).i * (*a.offset(i__3 as isize)).i;
                            q__1.i = (*x.offset(i__2 as isize)).r * (*a.offset(i__3 as isize)).i
                                + (*x.offset(i__2 as isize)).i * (*a.offset(i__3 as isize)).r;
                            (*x.offset(i__4 as isize)).r = q__1.r;
                            (*x.offset(i__4 as isize)).i = q__1.i
                        }
                    }
                    j += 1
                }
            } else {
                jx = kx;
                i__1 = *n;
                j = 1 as libc::c_int as integer;
                while j <= i__1 {
                    i__4 = jx;
                    if (*x.offset(i__4 as isize)).r != 0.0f32
                        || (*x.offset(i__4 as isize)).i != 0.0f32
                    {
                        i__4 = jx;
                        temp.r = (*x.offset(i__4 as isize)).r;
                        temp.i = (*x.offset(i__4 as isize)).i;
                        ix = kx;
                        l = kplus1 - j;
                        /* L40: */
                        /* Computing MAX */
                        i__4 = 1 as libc::c_int as integer;
                        i__2 = j - *k;
                        i__3 = j - 1 as libc::c_int as libc::c_long;
                        i__ = if i__4 >= i__2 { i__4 } else { i__2 };
                        while i__ <= i__3 {
                            i__4 = ix;
                            i__2 = ix;
                            i__5 = l + i__ + j * a_dim1;
                            q__2.r = temp.r * (*a.offset(i__5 as isize)).r
                                - temp.i * (*a.offset(i__5 as isize)).i;
                            q__2.i = temp.r * (*a.offset(i__5 as isize)).i
                                + temp.i * (*a.offset(i__5 as isize)).r;
                            q__1.r = (*x.offset(i__2 as isize)).r + q__2.r;
                            q__1.i = (*x.offset(i__2 as isize)).i + q__2.i;
                            (*x.offset(i__4 as isize)).r = q__1.r;
                            (*x.offset(i__4 as isize)).i = q__1.i;
                            ix += *incx;
                            i__ += 1
                            /* L30: */
                        }
                        if nounit != 0 {
                            i__3 = jx;
                            i__4 = jx;
                            i__2 = kplus1 + j * a_dim1;
                            q__1.r = (*x.offset(i__4 as isize)).r * (*a.offset(i__2 as isize)).r
                                - (*x.offset(i__4 as isize)).i * (*a.offset(i__2 as isize)).i;
                            q__1.i = (*x.offset(i__4 as isize)).r * (*a.offset(i__2 as isize)).i
                                + (*x.offset(i__4 as isize)).i * (*a.offset(i__2 as isize)).r;
                            (*x.offset(i__3 as isize)).r = q__1.r;
                            (*x.offset(i__3 as isize)).i = q__1.i
                        }
                    }
                    jx += *incx;
                    if j > *k {
                        kx += *incx
                    }
                    j += 1
                }
            }
        } else if *incx == 1 as libc::c_int as libc::c_long {
            j = *n;
            while j >= 1 as libc::c_int as libc::c_long {
                i__1 = j;
                if (*x.offset(i__1 as isize)).r != 0.0f32 || (*x.offset(i__1 as isize)).i != 0.0f32
                {
                    i__1 = j;
                    temp.r = (*x.offset(i__1 as isize)).r;
                    temp.i = (*x.offset(i__1 as isize)).i;
                    l = 1 as libc::c_int as libc::c_long - j;
                    /* L60: */
                    /* Computing MIN */
                    i__1 = *n;
                    i__3 = j + *k;
                    i__4 = j + 1 as libc::c_int as libc::c_long;
                    i__ = if i__1 <= i__3 { i__1 } else { i__3 };
                    while i__ >= i__4 {
                        i__1 = i__;
                        i__3 = i__;
                        i__2 = l + i__ + j * a_dim1;
                        q__2.r = temp.r * (*a.offset(i__2 as isize)).r
                            - temp.i * (*a.offset(i__2 as isize)).i;
                        q__2.i = temp.r * (*a.offset(i__2 as isize)).i
                            + temp.i * (*a.offset(i__2 as isize)).r;
                        q__1.r = (*x.offset(i__3 as isize)).r + q__2.r;
                        q__1.i = (*x.offset(i__3 as isize)).i + q__2.i;
                        (*x.offset(i__1 as isize)).r = q__1.r;
                        (*x.offset(i__1 as isize)).i = q__1.i;
                        i__ -= 1
                        /* L50: */
                    }
                    if nounit != 0 {
                        i__4 = j;
                        i__1 = j;
                        i__3 = j * a_dim1 + 1 as libc::c_int as libc::c_long;
                        q__1.r = (*x.offset(i__1 as isize)).r * (*a.offset(i__3 as isize)).r
                            - (*x.offset(i__1 as isize)).i * (*a.offset(i__3 as isize)).i;
                        q__1.i = (*x.offset(i__1 as isize)).r * (*a.offset(i__3 as isize)).i
                            + (*x.offset(i__1 as isize)).i * (*a.offset(i__3 as isize)).r;
                        (*x.offset(i__4 as isize)).r = q__1.r;
                        (*x.offset(i__4 as isize)).i = q__1.i
                    }
                }
                j -= 1
            }
        } else {
            kx += (*n - 1 as libc::c_int as libc::c_long) * *incx;
            jx = kx;
            j = *n;
            while j >= 1 as libc::c_int as libc::c_long {
                i__4 = jx;
                if (*x.offset(i__4 as isize)).r != 0.0f32 || (*x.offset(i__4 as isize)).i != 0.0f32
                {
                    i__4 = jx;
                    temp.r = (*x.offset(i__4 as isize)).r;
                    temp.i = (*x.offset(i__4 as isize)).i;
                    ix = kx;
                    l = 1 as libc::c_int as libc::c_long - j;
                    /* L80: */
                    /* Computing MIN */
                    i__4 = *n;
                    i__1 = j + *k;
                    i__3 = j + 1 as libc::c_int as libc::c_long;
                    i__ = if i__4 <= i__1 { i__4 } else { i__1 };
                    while i__ >= i__3 {
                        i__4 = ix;
                        i__1 = ix;
                        i__2 = l + i__ + j * a_dim1;
                        q__2.r = temp.r * (*a.offset(i__2 as isize)).r
                            - temp.i * (*a.offset(i__2 as isize)).i;
                        q__2.i = temp.r * (*a.offset(i__2 as isize)).i
                            + temp.i * (*a.offset(i__2 as isize)).r;
                        q__1.r = (*x.offset(i__1 as isize)).r + q__2.r;
                        q__1.i = (*x.offset(i__1 as isize)).i + q__2.i;
                        (*x.offset(i__4 as isize)).r = q__1.r;
                        (*x.offset(i__4 as isize)).i = q__1.i;
                        ix -= *incx;
                        i__ -= 1
                        /* L70: */
                    }
                    if nounit != 0 {
                        i__3 = jx;
                        i__4 = jx;
                        i__1 = j * a_dim1 + 1 as libc::c_int as libc::c_long;
                        q__1.r = (*x.offset(i__4 as isize)).r * (*a.offset(i__1 as isize)).r
                            - (*x.offset(i__4 as isize)).i * (*a.offset(i__1 as isize)).i;
                        q__1.i = (*x.offset(i__4 as isize)).r * (*a.offset(i__1 as isize)).i
                            + (*x.offset(i__4 as isize)).i * (*a.offset(i__1 as isize)).r;
                        (*x.offset(i__3 as isize)).r = q__1.r;
                        (*x.offset(i__3 as isize)).i = q__1.i
                    }
                }
                jx -= *incx;
                if *n - j >= *k {
                    kx -= *incx
                }
                j -= 1
            }
        }
    } else if lsame__0(
        uplo,
        b"U\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) != 0
    {
        kplus1 = *k + 1 as libc::c_int as libc::c_long;
        if *incx == 1 as libc::c_int as libc::c_long {
            j = *n;
            while j >= 1 as libc::c_int as libc::c_long {
                i__3 = j;
                temp.r = (*x.offset(i__3 as isize)).r;
                temp.i = (*x.offset(i__3 as isize)).i;
                l = kplus1 - j;
                if noconj != 0 {
                    if nounit != 0 {
                        i__3 = kplus1 + j * a_dim1;
                        q__1.r = temp.r * (*a.offset(i__3 as isize)).r
                            - temp.i * (*a.offset(i__3 as isize)).i;
                        q__1.i = temp.r * (*a.offset(i__3 as isize)).i
                            + temp.i * (*a.offset(i__3 as isize)).r;
                        temp.r = q__1.r;
                        temp.i = q__1.i
                    }
                    /*        Form  x := A'*x  or  x := conjg( A' )*x. */
                    /* L110: */
                    /* Computing MAX */
                    i__4 = 1 as libc::c_int as integer;
                    i__1 = j - *k;
                    i__3 = if i__4 >= i__1 { i__4 } else { i__1 };
                    i__ = j - 1 as libc::c_int as libc::c_long;
                    while i__ >= i__3 {
                        i__4 = l + i__ + j * a_dim1;
                        i__1 = i__;
                        q__2.r = (*a.offset(i__4 as isize)).r * (*x.offset(i__1 as isize)).r
                            - (*a.offset(i__4 as isize)).i * (*x.offset(i__1 as isize)).i;
                        q__2.i = (*a.offset(i__4 as isize)).r * (*x.offset(i__1 as isize)).i
                            + (*a.offset(i__4 as isize)).i * (*x.offset(i__1 as isize)).r;
                        q__1.r = temp.r + q__2.r;
                        q__1.i = temp.i + q__2.i;
                        temp.r = q__1.r;
                        temp.i = q__1.i;
                        i__ -= 1
                        /* L90: */
                    }
                } else {
                    if nounit != 0 {
                        r_cnjg_0(&mut q__2, &mut *a.offset((kplus1 + j * a_dim1) as isize));
                        q__1.r = temp.r * q__2.r - temp.i * q__2.i;
                        q__1.i = temp.r * q__2.i + temp.i * q__2.r;
                        temp.r = q__1.r;
                        temp.i = q__1.i
                    }
                    /* Computing MAX */
                    i__4 = 1 as libc::c_int as integer;
                    i__1 = j - *k;
                    i__3 = if i__4 >= i__1 { i__4 } else { i__1 };
                    i__ = j - 1 as libc::c_int as libc::c_long;
                    while i__ >= i__3 {
                        r_cnjg_0(&mut q__3, &mut *a.offset((l + i__ + j * a_dim1) as isize));
                        i__4 = i__;
                        q__2.r = q__3.r * (*x.offset(i__4 as isize)).r
                            - q__3.i * (*x.offset(i__4 as isize)).i;
                        q__2.i = q__3.r * (*x.offset(i__4 as isize)).i
                            + q__3.i * (*x.offset(i__4 as isize)).r;
                        q__1.r = temp.r + q__2.r;
                        q__1.i = temp.i + q__2.i;
                        temp.r = q__1.r;
                        temp.i = q__1.i;
                        i__ -= 1
                        /* L100: */
                    }
                }
                i__3 = j;
                (*x.offset(i__3 as isize)).r = temp.r;
                (*x.offset(i__3 as isize)).i = temp.i;
                j -= 1
            }
        } else {
            kx += (*n - 1 as libc::c_int as libc::c_long) * *incx;
            jx = kx;
            j = *n;
            while j >= 1 as libc::c_int as libc::c_long {
                i__3 = jx;
                temp.r = (*x.offset(i__3 as isize)).r;
                temp.i = (*x.offset(i__3 as isize)).i;
                kx -= *incx;
                ix = kx;
                l = kplus1 - j;
                if noconj != 0 {
                    if nounit != 0 {
                        i__3 = kplus1 + j * a_dim1;
                        q__1.r = temp.r * (*a.offset(i__3 as isize)).r
                            - temp.i * (*a.offset(i__3 as isize)).i;
                        q__1.i = temp.r * (*a.offset(i__3 as isize)).i
                            + temp.i * (*a.offset(i__3 as isize)).r;
                        temp.r = q__1.r;
                        temp.i = q__1.i
                    }
                    /* L140: */
                    /* Computing MAX */
                    i__4 = 1 as libc::c_int as integer;
                    i__1 = j - *k;
                    i__3 = if i__4 >= i__1 { i__4 } else { i__1 };
                    i__ = j - 1 as libc::c_int as libc::c_long;
                    while i__ >= i__3 {
                        i__4 = l + i__ + j * a_dim1;
                        i__1 = ix;
                        q__2.r = (*a.offset(i__4 as isize)).r * (*x.offset(i__1 as isize)).r
                            - (*a.offset(i__4 as isize)).i * (*x.offset(i__1 as isize)).i;
                        q__2.i = (*a.offset(i__4 as isize)).r * (*x.offset(i__1 as isize)).i
                            + (*a.offset(i__4 as isize)).i * (*x.offset(i__1 as isize)).r;
                        q__1.r = temp.r + q__2.r;
                        q__1.i = temp.i + q__2.i;
                        temp.r = q__1.r;
                        temp.i = q__1.i;
                        ix -= *incx;
                        i__ -= 1
                        /* L120: */
                    }
                } else {
                    if nounit != 0 {
                        r_cnjg_0(&mut q__2, &mut *a.offset((kplus1 + j * a_dim1) as isize));
                        q__1.r = temp.r * q__2.r - temp.i * q__2.i;
                        q__1.i = temp.r * q__2.i + temp.i * q__2.r;
                        temp.r = q__1.r;
                        temp.i = q__1.i
                    }
                    /* Computing MAX */
                    i__4 = 1 as libc::c_int as integer;
                    i__1 = j - *k;
                    i__3 = if i__4 >= i__1 { i__4 } else { i__1 };
                    i__ = j - 1 as libc::c_int as libc::c_long;
                    while i__ >= i__3 {
                        r_cnjg_0(&mut q__3, &mut *a.offset((l + i__ + j * a_dim1) as isize));
                        i__4 = ix;
                        q__2.r = q__3.r * (*x.offset(i__4 as isize)).r
                            - q__3.i * (*x.offset(i__4 as isize)).i;
                        q__2.i = q__3.r * (*x.offset(i__4 as isize)).i
                            + q__3.i * (*x.offset(i__4 as isize)).r;
                        q__1.r = temp.r + q__2.r;
                        q__1.i = temp.i + q__2.i;
                        temp.r = q__1.r;
                        temp.i = q__1.i;
                        ix -= *incx;
                        i__ -= 1
                        /* L130: */
                    }
                }
                i__3 = jx;
                (*x.offset(i__3 as isize)).r = temp.r;
                (*x.offset(i__3 as isize)).i = temp.i;
                jx -= *incx;
                j -= 1
            }
        }
    } else if *incx == 1 as libc::c_int as libc::c_long {
        i__3 = *n;
        j = 1 as libc::c_int as integer;
        while j <= i__3 {
            i__4 = j;
            temp.r = (*x.offset(i__4 as isize)).r;
            temp.i = (*x.offset(i__4 as isize)).i;
            l = 1 as libc::c_int as libc::c_long - j;
            if noconj != 0 {
                if nounit != 0 {
                    i__4 = j * a_dim1 + 1 as libc::c_int as libc::c_long;
                    q__1.r = temp.r * (*a.offset(i__4 as isize)).r
                        - temp.i * (*a.offset(i__4 as isize)).i;
                    q__1.i = temp.r * (*a.offset(i__4 as isize)).i
                        + temp.i * (*a.offset(i__4 as isize)).r;
                    temp.r = q__1.r;
                    temp.i = q__1.i
                }
                /* L170: */
                /* Computing MIN */
                i__1 = *n;
                i__2 = j + *k;
                i__4 = if i__1 <= i__2 { i__1 } else { i__2 };
                i__ = j + 1 as libc::c_int as libc::c_long;
                while i__ <= i__4 {
                    i__1 = l + i__ + j * a_dim1;
                    i__2 = i__;
                    q__2.r = (*a.offset(i__1 as isize)).r * (*x.offset(i__2 as isize)).r
                        - (*a.offset(i__1 as isize)).i * (*x.offset(i__2 as isize)).i;
                    q__2.i = (*a.offset(i__1 as isize)).r * (*x.offset(i__2 as isize)).i
                        + (*a.offset(i__1 as isize)).i * (*x.offset(i__2 as isize)).r;
                    q__1.r = temp.r + q__2.r;
                    q__1.i = temp.i + q__2.i;
                    temp.r = q__1.r;
                    temp.i = q__1.i;
                    i__ += 1
                    /* L150: */
                }
            } else {
                if nounit != 0 {
                    r_cnjg_0(
                        &mut q__2,
                        &mut *a.offset((j * a_dim1 + 1 as libc::c_int as libc::c_long) as isize),
                    );
                    q__1.r = temp.r * q__2.r - temp.i * q__2.i;
                    q__1.i = temp.r * q__2.i + temp.i * q__2.r;
                    temp.r = q__1.r;
                    temp.i = q__1.i
                }
                /* Computing MIN */
                i__1 = *n;
                i__2 = j + *k;
                i__4 = if i__1 <= i__2 { i__1 } else { i__2 };
                i__ = j + 1 as libc::c_int as libc::c_long;
                while i__ <= i__4 {
                    r_cnjg_0(&mut q__3, &mut *a.offset((l + i__ + j * a_dim1) as isize));
                    i__1 = i__;
                    q__2.r = q__3.r * (*x.offset(i__1 as isize)).r
                        - q__3.i * (*x.offset(i__1 as isize)).i;
                    q__2.i = q__3.r * (*x.offset(i__1 as isize)).i
                        + q__3.i * (*x.offset(i__1 as isize)).r;
                    q__1.r = temp.r + q__2.r;
                    q__1.i = temp.i + q__2.i;
                    temp.r = q__1.r;
                    temp.i = q__1.i;
                    i__ += 1
                    /* L160: */
                }
            }
            i__4 = j;
            (*x.offset(i__4 as isize)).r = temp.r;
            (*x.offset(i__4 as isize)).i = temp.i;
            j += 1
        }
    } else {
        jx = kx;
        i__3 = *n;
        j = 1 as libc::c_int as integer;
        while j <= i__3 {
            i__4 = jx;
            temp.r = (*x.offset(i__4 as isize)).r;
            temp.i = (*x.offset(i__4 as isize)).i;
            kx += *incx;
            ix = kx;
            l = 1 as libc::c_int as libc::c_long - j;
            if noconj != 0 {
                if nounit != 0 {
                    i__4 = j * a_dim1 + 1 as libc::c_int as libc::c_long;
                    q__1.r = temp.r * (*a.offset(i__4 as isize)).r
                        - temp.i * (*a.offset(i__4 as isize)).i;
                    q__1.i = temp.r * (*a.offset(i__4 as isize)).i
                        + temp.i * (*a.offset(i__4 as isize)).r;
                    temp.r = q__1.r;
                    temp.i = q__1.i
                }
                /* L200: */
                /* Computing MIN */
                i__1 = *n;
                i__2 = j + *k;
                i__4 = if i__1 <= i__2 { i__1 } else { i__2 };
                i__ = j + 1 as libc::c_int as libc::c_long;
                while i__ <= i__4 {
                    i__1 = l + i__ + j * a_dim1;
                    i__2 = ix;
                    q__2.r = (*a.offset(i__1 as isize)).r * (*x.offset(i__2 as isize)).r
                        - (*a.offset(i__1 as isize)).i * (*x.offset(i__2 as isize)).i;
                    q__2.i = (*a.offset(i__1 as isize)).r * (*x.offset(i__2 as isize)).i
                        + (*a.offset(i__1 as isize)).i * (*x.offset(i__2 as isize)).r;
                    q__1.r = temp.r + q__2.r;
                    q__1.i = temp.i + q__2.i;
                    temp.r = q__1.r;
                    temp.i = q__1.i;
                    ix += *incx;
                    i__ += 1
                    /* L180: */
                }
            } else {
                if nounit != 0 {
                    r_cnjg_0(
                        &mut q__2,
                        &mut *a.offset((j * a_dim1 + 1 as libc::c_int as libc::c_long) as isize),
                    );
                    q__1.r = temp.r * q__2.r - temp.i * q__2.i;
                    q__1.i = temp.r * q__2.i + temp.i * q__2.r;
                    temp.r = q__1.r;
                    temp.i = q__1.i
                }
                /* Computing MIN */
                i__1 = *n;
                i__2 = j + *k;
                i__4 = if i__1 <= i__2 { i__1 } else { i__2 };
                i__ = j + 1 as libc::c_int as libc::c_long;
                while i__ <= i__4 {
                    r_cnjg_0(&mut q__3, &mut *a.offset((l + i__ + j * a_dim1) as isize));
                    i__1 = ix;
                    q__2.r = q__3.r * (*x.offset(i__1 as isize)).r
                        - q__3.i * (*x.offset(i__1 as isize)).i;
                    q__2.i = q__3.r * (*x.offset(i__1 as isize)).i
                        + q__3.i * (*x.offset(i__1 as isize)).r;
                    q__1.r = temp.r + q__2.r;
                    q__1.i = temp.i + q__2.i;
                    temp.r = q__1.r;
                    temp.i = q__1.i;
                    ix += *incx;
                    i__ += 1
                    /* L190: */
                }
            }
            i__4 = jx;
            (*x.offset(i__4 as isize)).r = temp.r;
            (*x.offset(i__4 as isize)).i = temp.i;
            jx += *incx;
            j += 1
        }
    }
    return 0 as libc::c_int;
    /*     End of CTBMV . */
}
/* ctbmv_ */
