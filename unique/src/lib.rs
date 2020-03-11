pub trait Unique<T> {
    fn unique<P>(&mut self, pred: P) -> Option<T>
    where
        P: FnMut(&T) -> bool;
}

impl<T: Clone, I> Unique<T> for I
where
    I: Iterator<Item = T>,
{
    fn unique<P>(&mut self, mut pred: P) -> Option<T>
    where
        P: FnMut(&T) -> bool,
    {
        let mut res = vec![];

        for i in self {
            if pred(&i) {
                res.push(i);
                if res.len() > 1 {
                    break;
                }
            }
        }

        if res.len() == 1 {
            Some(res[0].clone())
        } else {
            None
        }
    }
}
