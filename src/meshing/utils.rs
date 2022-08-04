pub trait ArrayExt<'a, T: 'a>
where
    T: Sized,
{
    fn combs(&'a self, size: usize) -> Vec<Vec<&'a T>>;
}

impl<'a, T: 'a> ArrayExt<'a, T> for [T] {
    fn combs(&'a self, size: usize) -> Vec<Vec<&'a T>> {
        let mut combs = Vec::new();
        let mut tmp = Vec::new();
        combs_util(&mut combs, &mut tmp, self.len(), 0, size);

        let mut res = Vec::new();
        for comb in combs {
            let mut tmp = Vec::new();
            for i in comb {
                tmp.push(&self[i])
            }
            res.push(tmp);
        }
        res
    }
}

fn combs_util(res: &mut Vec<Vec<usize>>, tmp: &mut Vec<usize>, n: usize, left: usize, k: usize) {
    if k == 0 {
        res.push(tmp.clone());
        return;
    }

    for i in left..n {
        tmp.push(i);
        combs_util(res, tmp, n, i + 1, k - 1);
        tmp.pop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn combs_util() {
        let mut v = Vec::new();
        let mut tmp = Vec::new();
        super::combs_util(&mut v, &mut tmp, 3, 0, 2);
        assert_eq!(v.len(), 3);
        assert_eq!(v[0], vec![0, 1]);
        assert_eq!(v[1], vec![0, 2]);
        assert_eq!(v[2], vec![1, 2]);
    }

    #[test]
    fn pairs() {
        let v = [1, 2, 3].combs(2);
        assert_eq!(v.len(), 3);
        assert_eq!(v[0], vec![&1, &2]);
        assert_eq!(v[1], vec![&1, &3]);
        assert_eq!(v[2], vec![&2, &3]);
    }
}
