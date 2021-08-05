use std::collections::HashMap;
use std::borrow::Borrow;

pub fn fraction_to_decimal(numerator: i32, denominator: i32) -> String {
    //转换为 i64类型 防止溢出
    let numerator = numerator as i64;
    let mut denominator = denominator as i64;
    let mut res = String::new();
    let mut quotient = numerator / denominator;
    let mut remainder = numerator % denominator;
    let mut isnav = false;
    //如果余数为 6 能被整除 ,那么直接返回
    if remainder == 0 {
        return quotient.to_string();
    }
    //处理负数的情况 如果 结果为 - 的 那我们这里标记下 后面还是按正数处理,最后返回的时候加上 -
    if (numerator * denominator) < 0 {
        quotient *= -1;
        isnav = true;
    }
    //把负数转化为正数处理
    if remainder < 0 {
        remainder *= -1;
    }
    //把负数转化为正数处理
    if denominator < 0 {
        denominator *= -1;
    }

    //小数点后面 索引 可以记录
    let mut i = 0;
    let gap = helper(quotient);
    let mut recoder = HashMap::new();
    loop {
        if i == 0 {
            res += (quotient.to_string() + ".").as_ref();
            i += 1;
        } else { res += quotient.to_string().as_ref() }
        if recoder.contains_key(remainder.borrow()) {
            let index = recoder.get(remainder.borrow()).unwrap();
            if remainder == 0 {
                res.remove(res.len() - 1);
                break;
            }
            //gap 是整数部分的打大小 *index + gap 是 整数部分 + 循环节从小数点后其实位置
            res.insert((*index + gap) as usize, '(');
            res.push(")".parse().unwrap());
            break;
        }
        recoder.insert(remainder, i);
        quotient = remainder * 10 / denominator;
        remainder = remainder * 10 % denominator;
        i += 1;
    }
    //如果是结果是负的 最后返回的值 加上 -
    if isnav {
        res.insert(0, "-".parse().unwrap());
        return res;
    }
    res
}

pub fn helper(num: i64) -> i32 {
    if num == 0 {
        return 1;
    }
    let mut num = num;
    let mut res = 0;
    while num >= 1 {
        num /= 10;
        res += 1;
    }
    res
}