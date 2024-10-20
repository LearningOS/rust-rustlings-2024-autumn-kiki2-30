/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn sort<T: Ord + Clone>(array: &mut [T]) {
    let len = array.len();
    for i in 1..len {
        let key = array[i].clone(); // 克隆当前元素
        let mut j = i as isize - 1; // 先前的索引
        // 将比 key 大的元素向右移动
        while j >= 0 && array[j as usize] > key {
            array[j as usize + 1] = array[j as usize].clone(); // 克隆并移动元素
            j -= 1; // 继续向左
        }
        // 在安全范围内插入 key
        if j + 1 < len as isize {
            array[(j + 1) as usize] = key; // 将 key 插入到适当的位置
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }

    #[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }

    #[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}