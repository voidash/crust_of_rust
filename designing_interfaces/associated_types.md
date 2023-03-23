One method to implement interfaces
```rust
use std::marker::PhantomData;
pub trait Sort{
    type Output;
    fn sort(data: &mut [Self::Output]);
}

struct BubbleSort<T: PartialEq+PartialOrd> (PhantomData<T>);

impl<T> Sort for BubbleSort<T>
where T: PartialEq+PartialOrd
{
type Output = T;
fn sort(data: &mut [T]) -> &mut [T] {
        println!("test");
        data
    }

}
```


Another Method. Main difference is that the previous one was using associated types, whereas this method uses generic trait. 

```rust
pub trait Sort<T: PartialEq+PartialOrd> { 
    fn sort(data: &mut [T]);
}



struct BubbleSort<T> (PhantomData<T>);

impl<T> Sort<T> for BubbleSort<T>
where T: PartialEq+PartialOrd {
    fn sort(data: &mut [T]) {
        todo!()
    }
}
```

Using generic trait you can implement a trait multiple times for a single struct. which might or might not be desirable.For example

```rust
pub trait Iterator<T> {
    fn next(&self) -> Option<T>;
}
pub struct Counter {
    data : Vec<u32>,
}

impl Iterator<u32> for Counter {
    fn next(&self) -> Option<u32> {
        if self.data.len() > 0 {
            return Some(self.data[0]) 
        }
        return None
    }
}
impl Iterator<String> for Counter {
    fn next(&self) -> Option<String> {
        Some(format!("this is test"))
    }
}

#[cfg(test)]
mod test {
   

    use super::*;

    #[test]
    fn check_multiple_associated_types(){
        let c = Counter{data: vec![3,4,3]};
        // assert_eq!(c.next(),Some(3));
        // assert_eq!(c.next(),Some(format!("this is test")));

        // assert_eq!(c.next(),Some(3));
        println!("{:?}",<Counter as Iterator<String>>::next(&c));
     }

}
```
