use crate::FloatTy;

#[inline]
pub(crate) fn is_in_range(t: FloatTy, tmin: FloatTy, tmax: Option<FloatTy>) -> bool {
    if t < tmin {
        return false;
    }

    if let Some(tmax) = tmax {
        t < tmax
    } else {
        true
    }
}

#[inline]
pub(crate) fn fmin(a: FloatTy, b: FloatTy) -> FloatTy {
    if a <= b {
        a
    } else {
        b
    }
}

#[inline]
pub(crate) fn fmax(a: FloatTy, b: FloatTy) -> FloatTy {
    if a >= b {
        a
    } else {
        b
    }
}
