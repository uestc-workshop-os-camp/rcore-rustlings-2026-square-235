/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn sort<T:PartialOrd>(array: &mut [T]){
	//TODO
    if array.len() > 1{
        _quick_shot(array,0,array.len() - 1);
    } 
}

fn _quick_shot<T:PartialOrd>(array:&mut [T],start:usize,end:usize){
    if start < end {
        //执行本层排序
        let mut index = start;
        //双指针，选末位元素为基准，扫到小的往计数指针位置放
        for cur in start..end {
            if array[cur] < array[end] {
                array.swap(cur, index);
                index += 1;
            }
        }
        array.swap(index,end);

        //排两侧 
        if index > start{
            _quick_shot(array,start,index-1);
        }
        _quick_shot(array,index+1,end);
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