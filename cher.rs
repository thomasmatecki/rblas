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
/* cher.f -- translated by f2c (version 20061008).
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
pub unsafe extern "C" fn f2c_cher(
    mut uplo: *mut libc::c_char,
    mut n: *mut integer,
    mut alpha: *mut real,
    mut x: *mut complex,
    mut incx: *mut integer,
    mut a: *mut complex,
    mut lda: *mut integer,
) -> libc::c_int {
    /* System generated locals */
    let mut a_dim1: integer = 0;
    let mut a_offset: integer = 0;
    let mut i__1: integer = 0;
    let mut i__2: integer = 0;
    let mut i__3: integer = 0;
    let mut i__4: integer = 0;
    let mut i__5: integer = 0;
    let mut r__1: real = 0.;
    let mut q__1: complex = complex { r: 0., i: 0. };
    let mut q__2: complex = complex { r: 0., i: 0. };
    /* Builtin functions */
    extern "C" {
        #[link_name = "r_cnjg"]
        fn r_cnjg_0(_: *mut complex, _: *mut complex);
    }
    /* Local variables */
    let mut i__: integer = 0;
    let mut j: integer = 0;
    let mut ix: integer = 0;
    let mut jx: integer = 0;
    let mut kx: integer = 0;
    let mut info: integer = 0;
    let mut temp: complex = complex { r: 0., i: 0. };
    extern "C" {
        #[link_name = "lsame_"]
        fn lsame__0(_: *mut libc::c_char, _: *mut libc::c_char) -> logical;
    }
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
    /*  CHER   performs the hermitian rank 1 operation */
    /*     A := alpha*x*conjg( x' ) + A, */
    /*  where alpha is a real scalar, x is an n element vector and A is an */
    /*  n by n hermitian matrix. */
    /*  Arguments */
    /*  ========== */
    /*  UPLO   - CHARACTER*1. */
    /*           On entry, UPLO specifies whether the upper or lower */
    /*           triangular part of the array A is to be referenced as */
    /*           follows: */
    /*              UPLO = 'U' or 'u'   Only the upper triangular part of A */
    /*                                  is to be referenced. */
    /*              UPLO = 'L' or 'l'   Only the lower triangular part of A */
    /*                                  is to be referenced. */
    /*           Unchanged on exit. */
    /*  N      - INTEGER. */
    /*           On entry, N specifies the order of the matrix A. */
    /*           N must be at least zero. */
    /*           Unchanged on exit. */
    /*  ALPHA  - REAL            . */
    /*           On entry, ALPHA specifies the scalar alpha. */
    /*           Unchanged on exit. */
    /*  X      - COMPLEX          array of dimension at least */
    /*           ( 1 + ( n - 1 )*abs( INCX ) ). */
    /*           Before entry, the incremented array X must contain the n */
    /*           element vector x. */
    /*           Unchanged on exit. */
    /*  INCX   - INTEGER. */
    /*           On entry, INCX specifies the increment for the elements of */
    /*           X. INCX must not be zero. */
    /*           Unchanged on exit. */
    /*  A      - COMPLEX          array of DIMENSION ( LDA, n ). */
    /*           Before entry with  UPLO = 'U' or 'u', the leading n by n */
    /*           upper triangular part of the array A must contain the upper */
    /*           triangular part of the hermitian matrix and the strictly */
    /*           lower triangular part of A is not referenced. On exit, the */
    /*           upper triangular part of the array A is overwritten by the */
    /*           upper triangular part of the updated matrix. */
    /*           Before entry with UPLO = 'L' or 'l', the leading n by n */
    /*           lower triangular part of the array A must contain the lower */
    /*           triangular part of the hermitian matrix and the strictly */
    /*           upper triangular part of A is not referenced. On exit, the */
    /*           lower triangular part of the array A is overwritten by the */
    /*           lower triangular part of the updated matrix. */
    /*           Note that the imaginary parts of the diagonal elements need */
    /*           not be set, they are assumed to be zero, and on exit they */
    /*           are set to zero. */
    /*  LDA    - INTEGER. */
    /*           On entry, LDA specifies the first dimension of A as declared */
    /*           in the calling (sub) program. LDA must be at least */
    /*           max( 1, n ). */
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
    x = x.offset(-1);
    a_dim1 = *lda;
    a_offset = 1 as libc::c_int as libc::c_long + a_dim1;
    a = a.offset(-(a_offset as isize));
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
    } else if *n < 0 as libc::c_int as libc::c_long {
        info = 2 as libc::c_int as integer
    } else if *incx == 0 as libc::c_int as libc::c_long {
        info = 5 as libc::c_int as integer
    } else if *lda
        < (if 1 as libc::c_int as libc::c_long >= *n {
            1 as libc::c_int as libc::c_long
        } else {
            *n
        })
    {
        info = 7 as libc::c_int as integer
    }
    if info != 0 as libc::c_int as libc::c_long {
        xerbla__0(
            b"CHER  \x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            &mut info,
        );
        return 0 as libc::c_int;
    }
    /*     Quick return if possible. */
    if *n == 0 as libc::c_int as libc::c_long || *alpha == 0.0f32 {
        return 0 as libc::c_int;
    }
    /*     Set the start point in X if the increment is not unity. */
    if *incx <= 0 as libc::c_int as libc::c_long {
        kx = 1 as libc::c_int as libc::c_long - (*n - 1 as libc::c_int as libc::c_long) * *incx
    } else if *incx != 1 as libc::c_int as libc::c_long {
        kx = 1 as libc::c_int as integer
    }
    /*     Start the operations. In this version the elements of A are */
    /*     accessed sequentially with one pass through the triangular part */
    /*     of A. */
    if lsame__0(
        uplo,
        b"U\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) != 0
    {
        /*        Form  A  when A is stored in upper triangle. */
        if *incx == 1 as libc::c_int as libc::c_long {
            i__1 = *n;
            j = 1 as libc::c_int as integer;
            while j <= i__1 {
                i__2 = j;
                if (*x.offset(i__2 as isize)).r != 0.0f32 || (*x.offset(i__2 as isize)).i != 0.0f32
                {
                    r_cnjg_0(&mut q__2, &mut *x.offset(j as isize));
                    q__1.r = *alpha * q__2.r;
                    q__1.i = *alpha * q__2.i;
                    temp.r = q__1.r;
                    temp.i = q__1.i;
                    i__2 = j - 1 as libc::c_int as libc::c_long;
                    i__ = 1 as libc::c_int as integer;
                    while i__ <= i__2 {
                        i__3 = i__ + j * a_dim1;
                        i__4 = i__ + j * a_dim1;
                        i__5 = i__;
                        q__2.r = (*x.offset(i__5 as isize)).r * temp.r
                            - (*x.offset(i__5 as isize)).i * temp.i;
                        q__2.i = (*x.offset(i__5 as isize)).r * temp.i
                            + (*x.offset(i__5 as isize)).i * temp.r;
                        q__1.r = (*a.offset(i__4 as isize)).r + q__2.r;
                        q__1.i = (*a.offset(i__4 as isize)).i + q__2.i;
                        (*a.offset(i__3 as isize)).r = q__1.r;
                        (*a.offset(i__3 as isize)).i = q__1.i;
                        i__ += 1
                        /* L20: */
                        /* L10: */
                    }
                    i__2 = j + j * a_dim1;
                    i__3 = j + j * a_dim1;
                    i__4 = j;
                    q__1.r = (*x.offset(i__4 as isize)).r * temp.r
                        - (*x.offset(i__4 as isize)).i * temp.i;
                    q__1.i = (*x.offset(i__4 as isize)).r * temp.i
                        + (*x.offset(i__4 as isize)).i * temp.r;
                    r__1 = (*a.offset(i__3 as isize)).r + q__1.r;
                    (*a.offset(i__2 as isize)).r = r__1;
                    (*a.offset(i__2 as isize)).i = 0.0f32
                } else {
                    i__2 = j + j * a_dim1;
                    i__3 = j + j * a_dim1;
                    r__1 = (*a.offset(i__3 as isize)).r;
                    (*a.offset(i__2 as isize)).r = r__1;
                    (*a.offset(i__2 as isize)).i = 0.0f32
                }
                j += 1
            }
        } else {
            jx = kx;
            i__1 = *n;
            j = 1 as libc::c_int as integer;
            while j <= i__1 {
                i__2 = jx;
                if (*x.offset(i__2 as isize)).r != 0.0f32 || (*x.offset(i__2 as isize)).i != 0.0f32
                {
                    r_cnjg_0(&mut q__2, &mut *x.offset(jx as isize));
                    q__1.r = *alpha * q__2.r;
                    q__1.i = *alpha * q__2.i;
                    temp.r = q__1.r;
                    temp.i = q__1.i;
                    ix = kx;
                    i__2 = j - 1 as libc::c_int as libc::c_long;
                    i__ = 1 as libc::c_int as integer;
                    while i__ <= i__2 {
                        i__3 = i__ + j * a_dim1;
                        i__4 = i__ + j * a_dim1;
                        i__5 = ix;
                        q__2.r = (*x.offset(i__5 as isize)).r * temp.r
                            - (*x.offset(i__5 as isize)).i * temp.i;
                        q__2.i = (*x.offset(i__5 as isize)).r * temp.i
                            + (*x.offset(i__5 as isize)).i * temp.r;
                        q__1.r = (*a.offset(i__4 as isize)).r + q__2.r;
                        q__1.i = (*a.offset(i__4 as isize)).i + q__2.i;
                        (*a.offset(i__3 as isize)).r = q__1.r;
                        (*a.offset(i__3 as isize)).i = q__1.i;
                        ix += *incx;
                        i__ += 1
                        /* L40: */
                        /* L30: */
                    }
                    i__2 = j + j * a_dim1;
                    i__3 = j + j * a_dim1;
                    i__4 = jx;
                    q__1.r = (*x.offset(i__4 as isize)).r * temp.r
                        - (*x.offset(i__4 as isize)).i * temp.i;
                    q__1.i = (*x.offset(i__4 as isize)).r * temp.i
                        + (*x.offset(i__4 as isize)).i * temp.r;
                    r__1 = (*a.offset(i__3 as isize)).r + q__1.r;
                    (*a.offset(i__2 as isize)).r = r__1;
                    (*a.offset(i__2 as isize)).i = 0.0f32
                } else {
                    i__2 = j + j * a_dim1;
                    i__3 = j + j * a_dim1;
                    r__1 = (*a.offset(i__3 as isize)).r;
                    (*a.offset(i__2 as isize)).r = r__1;
                    (*a.offset(i__2 as isize)).i = 0.0f32
                }
                jx += *incx;
                j += 1
            }
        }
    } else if *incx == 1 as libc::c_int as libc::c_long {
        i__1 = *n;
        j = 1 as libc::c_int as integer;
        while j <= i__1 {
            i__2 = j;
            if (*x.offset(i__2 as isize)).r != 0.0f32 || (*x.offset(i__2 as isize)).i != 0.0f32 {
                r_cnjg_0(&mut q__2, &mut *x.offset(j as isize));
                q__1.r = *alpha * q__2.r;
                q__1.i = *alpha * q__2.i;
                temp.r = q__1.r;
                temp.i = q__1.i;
                i__2 = j + j * a_dim1;
                i__3 = j + j * a_dim1;
                i__4 = j;
                q__1.r =
                    temp.r * (*x.offset(i__4 as isize)).r - temp.i * (*x.offset(i__4 as isize)).i;
                q__1.i =
                    temp.r * (*x.offset(i__4 as isize)).i + temp.i * (*x.offset(i__4 as isize)).r;
                r__1 = (*a.offset(i__3 as isize)).r + q__1.r;
                (*a.offset(i__2 as isize)).r = r__1;
                (*a.offset(i__2 as isize)).i = 0.0f32;
                i__2 = *n;
                i__ = j + 1 as libc::c_int as libc::c_long;
                while i__ <= i__2 {
                    i__3 = i__ + j * a_dim1;
                    i__4 = i__ + j * a_dim1;
                    i__5 = i__;
                    q__2.r = (*x.offset(i__5 as isize)).r * temp.r
                        - (*x.offset(i__5 as isize)).i * temp.i;
                    q__2.i = (*x.offset(i__5 as isize)).r * temp.i
                        + (*x.offset(i__5 as isize)).i * temp.r;
                    q__1.r = (*a.offset(i__4 as isize)).r + q__2.r;
                    q__1.i = (*a.offset(i__4 as isize)).i + q__2.i;
                    (*a.offset(i__3 as isize)).r = q__1.r;
                    (*a.offset(i__3 as isize)).i = q__1.i;
                    i__ += 1
                    /*        Form  A  when A is stored in lower triangle. */
                    /* L60: */
                    /* L50: */
                }
            } else {
                i__2 = j + j * a_dim1;
                i__3 = j + j * a_dim1;
                r__1 = (*a.offset(i__3 as isize)).r;
                (*a.offset(i__2 as isize)).r = r__1;
                (*a.offset(i__2 as isize)).i = 0.0f32
            }
            j += 1
        }
    } else {
        jx = kx;
        i__1 = *n;
        j = 1 as libc::c_int as integer;
        while j <= i__1 {
            i__2 = jx;
            if (*x.offset(i__2 as isize)).r != 0.0f32 || (*x.offset(i__2 as isize)).i != 0.0f32 {
                r_cnjg_0(&mut q__2, &mut *x.offset(jx as isize));
                q__1.r = *alpha * q__2.r;
                q__1.i = *alpha * q__2.i;
                temp.r = q__1.r;
                temp.i = q__1.i;
                i__2 = j + j * a_dim1;
                i__3 = j + j * a_dim1;
                i__4 = jx;
                q__1.r =
                    temp.r * (*x.offset(i__4 as isize)).r - temp.i * (*x.offset(i__4 as isize)).i;
                q__1.i =
                    temp.r * (*x.offset(i__4 as isize)).i + temp.i * (*x.offset(i__4 as isize)).r;
                r__1 = (*a.offset(i__3 as isize)).r + q__1.r;
                (*a.offset(i__2 as isize)).r = r__1;
                (*a.offset(i__2 as isize)).i = 0.0f32;
                ix = jx;
                i__2 = *n;
                i__ = j + 1 as libc::c_int as libc::c_long;
                while i__ <= i__2 {
                    ix += *incx;
                    i__3 = i__ + j * a_dim1;
                    i__4 = i__ + j * a_dim1;
                    i__5 = ix;
                    q__2.r = (*x.offset(i__5 as isize)).r * temp.r
                        - (*x.offset(i__5 as isize)).i * temp.i;
                    q__2.i = (*x.offset(i__5 as isize)).r * temp.i
                        + (*x.offset(i__5 as isize)).i * temp.r;
                    q__1.r = (*a.offset(i__4 as isize)).r + q__2.r;
                    q__1.i = (*a.offset(i__4 as isize)).i + q__2.i;
                    (*a.offset(i__3 as isize)).r = q__1.r;
                    (*a.offset(i__3 as isize)).i = q__1.i;
                    i__ += 1
                    /* L80: */
                    /* L70: */
                }
            } else {
                i__2 = j + j * a_dim1;
                i__3 = j + j * a_dim1;
                r__1 = (*a.offset(i__3 as isize)).r;
                (*a.offset(i__2 as isize)).r = r__1;
                (*a.offset(i__2 as isize)).i = 0.0f32
            }
            jx += *incx;
            j += 1
        }
    }
    return 0 as libc::c_int;
    /*     End of CHER  . */
}
/* cher_ */
