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
pub fn merge_sort(vec: &Vec<Qpixel>) -> Vec<Qpixel> {
    if vec.len() < 2 {
        return vec.to_vec();
    } else { 
        let size = vec.len() / 2;
        let left = merge_sort(&vec[0..size].to_vec());
        let right = merge_sort(&vec[size..].to_vec());

        return merge(&left, &right);
    }
}

fn merge(left: &Vec<Qpixel>, right: &Vec<Qpixel>) -> Vec<Qpixel> {
    let mut i = 0;
    let mut j = 0;
    let mut merged: Vec<Qpixel> = Vec::new();

    while i < left.len() && j < right.len() {
        if left[i].hue < right[j].hue {
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