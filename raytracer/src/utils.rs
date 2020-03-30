use crate::FloatTy;

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
