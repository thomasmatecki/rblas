use libc;
/* f2c.h  --  Standard Fortran to C header file */
/* *  barf  [ba:rf]  2.  "He suggested using FORTRAN, and everybody barfed."

- From The Shogakukan DICTIONARY OF NEW ENGLISH (Second edition) */
pub type integer = libc::c_long;
pub type logical = libc::c_long;
/* lsame.f -- translated by f2c (version 20061008).
   You must link the resulting object file with libf2c:
    on Microsoft Windows system, link with libf2c.lib;
    on Linux or Unix systems, link with .../path/to/libf2c.a -lm
    or, if you install libf2c.a in a standard place, with -lf2c -lm
    -- in that order, at the end of the command line, as in
        cc *.o -lf2c -lm
    Source for libf2c is in /netlib/f2c/libf2c.zip, e.g.,

        http://www.netlib.org/f2c/libf2c.zip
*/
#[no_mangle]
pub unsafe extern "C" fn lsame_(mut ca: *mut libc::c_char, mut cb: *mut libc::c_char) -> logical {
    /* System generated locals */
    let mut ret_val: logical = 0;
    /* Local variables */
    let mut inta: integer = 0;
    let mut intb: integer = 0;
    let mut zcode: integer = 0;
    /*  -- LAPACK auxiliary routine (version 3.1) -- */
    /*     Univ. of Tennessee, Univ. of California Berkeley and NAG Ltd.. */
    /*     November 2006 */
    /*     .. Scalar Arguments .. */
    /*     .. */
    /*  Purpose */
    /*  ======= */
    /*  LSAME returns .TRUE. if CA is the same letter as CB regardless of */
    /*  case. */
    /*  Arguments */
    /*  ========= */
    /*  CA      (input) CHARACTER*1 */
    /*  CB      (input) CHARACTER*1 */
    /*          CA and CB specify the single characters to be compared. */
    /* ===================================================================== */
    /*     .. Intrinsic Functions .. */
    /*     .. */
    /*     .. Local Scalars .. */
    /*     .. */
    /*     Test if the characters are equal */
    ret_val = (*(ca as *mut libc::c_uchar) as libc::c_int
        == *(cb as *mut libc::c_uchar) as libc::c_int) as libc::c_int as logical;
    if ret_val != 0 {
        return ret_val;
    }
    /*     Now test for equivalence if both characters are alphabetic. */
    zcode = 'Z' as i32 as integer;
    /*     Use 'Z' rather than 'A' so that ASCII can be detected on Prime */
    /*     machines, on which ICHAR returns a value with bit 8 set. */
    /*     ICHAR('A') on Prime machines returns 193 which is the same as */
    /*     ICHAR('A') on an EBCDIC machine. */
    inta = *(ca as *mut libc::c_uchar) as integer;
    intb = *(cb as *mut libc::c_uchar) as integer;
    if zcode == 90 as libc::c_int as libc::c_long || zcode == 122 as libc::c_int as libc::c_long {
        /*        ASCII is assumed - ZCODE is the ASCII code of either lower or */
        /*        upper case 'Z'. */
        if inta >= 97 as libc::c_int as libc::c_long && inta <= 122 as libc::c_int as libc::c_long {
            inta += -(32 as libc::c_int) as libc::c_long
        }
        if intb >= 97 as libc::c_int as libc::c_long && intb <= 122 as libc::c_int as libc::c_long {
            intb += -(32 as libc::c_int) as libc::c_long
        }
    } else if zcode == 233 as libc::c_int as libc::c_long
        || zcode == 169 as libc::c_int as libc::c_long
    {
        /*        EBCDIC is assumed - ZCODE is the EBCDIC code of either lower or */
        /*        upper case 'Z'. */
        if inta >= 129 as libc::c_int as libc::c_long && inta <= 137 as libc::c_int as libc::c_long
            || inta >= 145 as libc::c_int as libc::c_long
                && inta <= 153 as libc::c_int as libc::c_long
            || inta >= 162 as libc::c_int as libc::c_long
                && inta <= 169 as libc::c_int as libc::c_long
        {
            inta += 64 as libc::c_int as libc::c_long
        }
        if intb >= 129 as libc::c_int as libc::c_long && intb <= 137 as libc::c_int as libc::c_long
            || intb >= 145 as libc::c_int as libc::c_long
                && intb <= 153 as libc::c_int as libc::c_long
            || intb >= 162 as libc::c_int as libc::c_long
                && intb <= 169 as libc::c_int as libc::c_long
        {
            intb += 64 as libc::c_int as libc::c_long
        }
    } else if zcode == 218 as libc::c_int as libc::c_long
        || zcode == 250 as libc::c_int as libc::c_long
    {
        /*        ASCII is assumed, on Prime machines - ZCODE is the ASCII code */
        /*        plus 128 of either lower or upper case 'Z'. */
        if inta >= 225 as libc::c_int as libc::c_long && inta <= 250 as libc::c_int as libc::c_long
        {
            inta += -(32 as libc::c_int) as libc::c_long
        }
        if intb >= 225 as libc::c_int as libc::c_long && intb <= 250 as libc::c_int as libc::c_long
        {
            intb += -(32 as libc::c_int) as libc::c_long
        }
    }
    ret_val = (inta == intb) as libc::c_int as logical;
    /*     RETURN */
    /*     End of LSAME */
    return ret_val;
}
/* lsame_ */
