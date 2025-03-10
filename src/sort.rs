use crate::qpixel::qpixel::Qpixel;

/*
    Sort three values of type T (fast)
*/
#[allow(dead_code)]
pub fn sort3<'a, T>(mut x: &'a mut T, mut y: &'a mut T, mut z: &'a mut T) -> [&'a mut T; 3] where T: PartialOrd, T: Copy {

    if x > y {
        let buf: &mut T = y;
        y = x;
        x = buf;
    }
    if y > z {
        let buf: &mut T = z;
        z = y;
        y = buf;
    }
    if x > y {
        let buf: &mut T = y;
        y = x;
        x = buf;
    }

    return [x, y, z]    
}

#[allow(dead_code)]
pub fn merge_sort<'a, T>(vec: &'a Vec<T>, get_value: &dyn Fn(&T) -> f32) -> Vec<T> where T: Copy {
    if vec.len() < 2 {
        return vec.to_vec();
    } else { 
        let size = vec.len() / 2;
        let left = merge_sort(&vec[0..size].to_vec(), get_value);
        let right = merge_sort(&vec[size..].to_vec(), get_value);

        return merge(left, right, &get_value);
    }
}

fn merge<'a, T>(left: Vec<T>, right: Vec<T>, get_value: &dyn Fn(&T) -> f32) -> Vec<T> where T: Copy {
    let mut i = 0;
    let mut j = 0;
    let mut merged: Vec<T> = Vec::new();

    while i < left.len() && j < right.len() {
        if get_value(&left[i]) < get_value(&right[j]) {
            merged.push(left[i]);
            i = i + 1;
        } else {
            merged.push(right[j]);
            j = j + 1;
        }
    }

    if i < left.len() {
        while i < left.len() {
            merged.push(left[i]);
            i = i + 1;
        }
    }

    if j < right.len() {
        while j < right.len() {
            merged.push(right[j]);
            j = j + 1;
        }
    }

    merged
}

#[allow(dead_code)]
pub fn get_hue(value: &&Qpixel) -> f32 {
    return value.hue;
}

#[allow(dead_code)]
pub fn to_ref_vec<'a, T>(vec: &'a Vec<T>) -> Vec<&'a T> {
    vec.iter().collect()
}

#[allow(dead_code)]
pub fn from_ref_vec<'a, T>(vec: Vec<&'a T>) -> Vec<T> where T: Copy {
    vec.iter().map(|elem| **elem).collect()
}
