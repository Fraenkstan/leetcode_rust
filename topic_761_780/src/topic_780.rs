

pub fn reaching_points(sx: i32, sy: i32, mut tx: i32, mut ty: i32) -> bool {
    while tx > sx && ty > sy && tx != ty {
        if tx > ty {
            tx %= ty;
        } else { ty %= tx; }
    }
    if tx == sx && ty == sy {
        return true;
    } else if tx == sx {
        return ty > sy && (ty - sy) % tx == 0;
    } else if ty == sy {
        return tx > sx && (tx - sx) % ty == 0;
    }
    false
}