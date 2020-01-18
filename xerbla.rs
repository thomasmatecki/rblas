use libc;
extern "C" {
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
/* f2c.h  --  Standard Fortran to C header file */
/* *  barf  [ba:rf]  2.  "He suggested using FORTRAN, and everybody barfed."

- From The Shogakukan DICTIONARY OF NEW ENGLISH (Second edition) */
pub type integer = libc::c_long;
/* Adjust for integer*8. */
/* Extern is for use with -E */
/* I/O stuff */
pub type flag = libc::c_long;
pub type ftnlen = libc::c_long;
pub type ftnint = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cilist {
    pub cierr: flag,
    pub ciunit: ftnint,
    pub ciend: flag,
    pub cifmt: *mut libc::c_char,
    pub cirec: ftnint,
}
/* Subroutine */
#[no_mangle]
pub unsafe extern "C" fn xerbla_(
    mut srname: *mut libc::c_char,
    mut info: *mut integer,
) -> libc::c_int {
    /* Format strings */
    static mut fmt_9999: [libc::c_char; 74] = [
        40, 2, 32, 42, 42, 32, 79, 110, 32, 101, 110, 116, 114, 121, 32, 116, 111, 32, 2, 44, 97,
        44, 2, 32, 112, 97, 114, 97, 109, 101, 116, 101, 114, 32, 110, 117, 109, 98, 101, 114, 32,
        2, 44, 105, 50, 44, 2, 32, 104, 97, 100, 32, 2, 44, 2, 97, 110, 32, 105, 108, 108, 101,
        103, 97, 108, 32, 118, 97, 108, 117, 101, 2, 41, 0,
    ];
    /* Builtin functions */
    extern "C" {
        #[link_name = "s_wsfe"]
        fn s_wsfe_0(_: *mut cilist) -> integer;
    }
    extern "C" {
        #[link_name = "i_len_trim"]
        fn i_len_trim_0(_: *mut libc::c_char, _: ftnlen) -> integer;
    }
    extern "C" {
        #[link_name = "do_fio"]
        fn do_fio_0(_: *mut integer, _: *mut libc::c_char, _: ftnlen) -> integer;
    }
    extern "C" {
        #[link_name = "e_wsfe"]
        fn e_wsfe_0() -> integer;
    }
    /* Subroutine */
    extern "C" {
        #[link_name = "s_stop"]
        fn s_stop_0(_: *mut libc::c_char, _: ftnlen) -> libc::c_int;
    }
    /* Fortran I/O blocks */
    static mut io___1: cilist = unsafe {
        {
            let mut init = cilist {
                cierr: 0 as libc::c_int as flag,
                ciunit: 6 as libc::c_int as ftnint,
                ciend: 0 as libc::c_int as flag,
                cifmt: fmt_9999.as_ptr() as *mut _,
                cirec: 0 as libc::c_int as ftnint,
            };
            init
        }
    };
    /*  -- LAPACK auxiliary routine (preliminary version) -- */
    /*     Univ. of Tennessee, Univ. of California Berkeley and NAG Ltd.. */
    /*     November 2006 */
    /*     .. Scalar Arguments .. */
    /*     .. */
    /*  Purpose */
    /*  ======= */
    /*  XERBLA  is an error handler for the LAPACK routines. */
    /*  It is called by an LAPACK routine if an input parameter has an */
    /*  invalid value.  A message is printed and execution stops. */
    /*  Installers may consider modifying the STOP statement in order to */
    /*  call system-specific exception-handling facilities. */
    /*  Arguments */
    /*  ========= */
    /*  SRNAME  (input) CHARACTER*(*) */
    /*          The name of the routine which called XERBLA. */
    /*  INFO    (input) INTEGER */
    /*          The position of the invalid parameter in the parameter list */
    /*          of the calling routine. */
    /* ===================================================================== */
    /*     .. Intrinsic Functions .. */
    /*     .. */
    /*     .. Executable Statements .. */
    printf(
        b"** On entry to %6s, parameter number %2i had an illegal value\n\x00" as *const u8
            as *const libc::c_char,
        srname,
        *info,
    );
    /*     End of XERBLA */
    return 0 as libc::c_int;
}
/* xerbla_ */
