struct UrlMap<'a> {
    groups: Vec<String>,
    origins: Vec<String>,
    map: Vec<Vec<Either<String,Regex>>>,
    metadata: Vec<MetaData>,
}

impl <'a>UrlMap<'a> {

    fn from_file(path: &str) -> Self {

    }

    fn find_in_vec(vec: &Vec<Either<String,Regex>>, target: &str) -> Option<usize> {
        let mut i: usize = 0;
        for either in vec {
            match either {
                // Either we can have a string of our url piece, like "/api" or "/users"
                Either::This(a) => {
                    if a == target {
                        i = i + 1;
                        return Some(i)
                    }
                },
                // Or we have a regex because it is an optional type
                Either::That(b) => {
                    if b.captures(target).is_some() {
                        i = i + 1;
                        return Some(i)
                    }
                },
                _ => ()
            }
        }
        None
    }

    // fn get(&self, target: (usize,usize)) -> Either<String,Regex> {
    //     let index_of: Either<String,Regex> = match self.map.get(target.0) {
    //         Either::This(a) => Either::This(a),
    //         _ => Either::None,
    //     };
    //     Either::None
    // }

    // fn find(&self, taget: Vec<&str>) -> Vec<(usize,usize)> {
    //     let mut x =  0;
    //     let mut at: Vec<(usize,usize)> = Vec::new();
    //
    //     for iter in self.map.iter() {
    //         match UrlMap::find_in_vec(i, taget.get(i).unwrap()) {
    //             Some(y) => at.push((x,y)),
    //             _ => (),
    //         }
    //     }
    //     at
    // }
}