fn heap_sort_down(vec: &mut Vec<u32>, mut k: usize, size: usize, temp: u32) {
    /*
     { for (;;) { \
        size_t s = (k << 1); \
        if (s > size) break; \
        if (s < size && p[s + 1] > p[s]) s++; \
        if (temp >= p[s]) break; \
        p[k] = p[s]; k = s; \
      } p[k] = temp; }
     */
    loop {
        let mut s: usize = k << 1;
        if s > size {
            break;
        }

        if s < size && vec[s + 1] > vec[s] {
            s += 1
        }

        if temp >= vec[s] {
            break;
        }

        vec[k] = vec[s];
        k = s;
    }

    vec[k] = temp;
}

pub fn heap_sort(vec: &mut Vec<u32>) {
    let size = vec.len();
    if size <= 1 {
        return;
    }
    let mut i = size / 2;
    loop {
        let temp = vec[i];
        let k = temp;
        heap_sort_down(vec, k as usize, size, temp);
        i -= 1;
        if i == 0 {
            break;
        }
    }

    let mut size = size;
    while size > 3 {
        let temp = vec[size];
        let k: usize = if vec[3] > vec[2] { 3 } else { 2 };
        vec[size] = vec[1];
        size -= 1;
        vec[1] = vec[k];
        heap_sort_down(vec, k, size, temp)
    }

    let temp = p[size];
    p[size] = p[1];
    if size > 2 && p[2] < temp {
        p[1] = p[2];
        p[2] = temp;
    } else {
        p[1] = temp;
    }
}