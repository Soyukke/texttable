pub fn cell(inner_width: usize, inner_height: usize) -> String {
    let tl = "┌";
    let tr = "┐";
    let bl = "└";
    let br = "┘";
    let ve = "│";
    let ho = "─";
    let mut str = "".to_string();

    let width = inner_width + 2;
    let height = inner_height + 2;
    for y in 0..height {
        for x in 0..width {
            match (x, y) {
                (w, h) if w==width-1 && h==height-1 => str += br,
                (0, 0) => str += tl,
                (w, 0) if w==width-1 => str += &format!("{}\n", tr),
                (w, _) if w==width-1 => str += &format!("{}\n", ve),
                (0, h) if h==height-1=> str += bl,
                (0, _) => str += &format!("{}", ve),
                (_, 0) => str += ho,
                (_, h) if h==height-1 => str += ho,
                _ => str += " "
            }

        }
    }
    str
}

/// (n, m)のテーブルの(i, j)にあるセル
/// tl: tlが必要 
/// tr: trが必要
/// それ以外: 
pub fn inner_cell(inner_width: usize, inner_height: usize, i: usize, j: usize, n: usize, m: usize) -> String {
    let tl = "┌";
    let tr = "┐";
    let bl = "└";
    let br = "┘";
    let ve = "│";
    let ho = "─";
    // j==0
    let tc = "┬";
    // j==height
    let bc = "┴";
    let cp = "┼";

    let mut str = "".to_string();

    let width = inner_width + 2;
    let height = inner_height + 2;
    for y in 0..height {
        for x in 0..width {
            match (x, y) {
                (w, h) if w==width-1 && h==height-1 => str += br,
                (0, 0) => str += tl,
                (w, 0) if w==width-1 => str += &format!("{}\n", tr),
                (w, _) if w==width-1 => str += &format!("{}\n", ve),
                (0, h) if h==height-1=> str += bl,
                (0, _) => str += &format!("{}", ve),
                (_, 0) => str += ho,
                (_, h) if h==height-1 => str += ho,
                _ => str += " "
            }

        }
    }
    str
}

#[derive(Debug, Clone)]
pub struct Table {
    cells: Vec<Vec<Cell>>,
    nrows: usize,
    ncols: usize,
}

impl Table {
    pub fn new(n: usize, m: usize) -> Self {
        Self {
            cells: vec![vec![Cell::new(1, 1); m]; n],
            nrows: n,
            ncols: m,
        }
    }

    pub fn print(&self) {

        for i in 0..self.nrows {
            let mut sss = vec!["".to_string(); self.cells[i][0].inner_height+2];
            for j in 0..self.ncols {
                let ss = self.cells[i][j].printstr();
                for (k, s) in ss.iter().enumerate() {
                    let l = s.chars().count();
                    match k {
                        0 => sss[k] += &s,
                        _ => sss[k] += &s,
                        //0 => sss[k] += &s.get(0..6).unwrap_or(""),
                        //_ => sss[k] += &s.get(0..6).unwrap_or(""),
                        //_ => sss[k] += &s,
                    }
                    ;
                }
            }
            for s in sss {
                println!("{}", s);
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Cell {
    inner_width: usize,
    inner_height: usize,
    inner_text: String,
}

impl Cell {
    pub fn new(w: usize, h: usize) -> Self {
        Self {inner_width: w, inner_height: h, inner_text: "".to_string()}
    }

    pub fn printstr_range(&self, xr: std::ops::Range<usize>, yr: std::ops::Range<usize>) -> Vec<String> {
        let mut strs: Vec<String> = vec![];
        let tl = "┌";
        let tr = "┐";
        let bl = "└";
        let br = "┘";
        let ve = "│";
        let ho = "─";
        let mut str = "".to_string();

        let width = self.inner_width + 2;
        let height = self.inner_height + 2;
        for y in yr.clone() {
            let mut sstr = "".to_string();
            for x in xr.clone() {
                match (x, y) {
                    (w, h) if w==width-1 && h==height-1 => sstr += br,
                    (0, 0) => sstr += tl,
                    (w, 0) if w==width-1 => sstr += &format!("{}", tr),
                    (w, _) if w==width-1 => sstr += &format!("{}", ve),
                    (0, h) if h==height-1=> sstr += bl,
                    (0, _) => sstr += &format!("{}", ve),
                    (_, 0) => sstr += ho,
                    (_, h) if h==height-1 => sstr += ho,
                    _ => sstr += " "
                }
            }
            strs.push(sstr.clone().to_string());
        }
        strs
    }

    pub fn printstr(&self) -> Vec<String> {
        let mut strs: Vec<String> = vec![];
        let tl = "┌";
        let tr = "┐";
        let bl = "└";
        let br = "┘";
        let ve = "│";
        let ho = "─";
        let mut str = "".to_string();

        let width = self.inner_width + 2;
        let height = self.inner_height + 2;
        for y in 0..height {
            let mut sstr = "".to_string();
            for x in 0..width {
                match (x, y) {
                    (w, h) if w==width-1 && h==height-1 => sstr += br,
                    (0, 0) => sstr += tl,
                    (w, 0) if w==width-1 => sstr += &format!("{}", tr),
                    (w, _) if w==width-1 => sstr += &format!("{}", ve),
                    (0, h) if h==height-1=> sstr += bl,
                    (0, _) => sstr += &format!("{}", ve),
                    (_, 0) => sstr += ho,
                    (_, h) if h==height-1 => sstr += ho,
                    _ => sstr += " "
                }
            }
            strs.push(sstr.clone().to_string());
        }
        strs
    }


    pub fn printstr_tl(&self) -> Vec<String> {
        let mut strs: Vec<String> = vec![];
        let tl = "┌";
        let tr = "┐";
        let bl = "└";
        let br = "┘";
        let ve = "│";
        let ho = "─";
        // j==0
        let tc = "┬";
        // j==height
        let bc = "┴";
        let cp = "┼";

        let mut str = "".to_string();

        let width = self.inner_width + 2;
        let height = self.inner_height + 2;
        for y in 0..height {
            let mut sstr = "".to_string();
            for x in 0..width {
                match (x, y) {
                    (w, h) if w==width-1 && h==height-1 => sstr += cp,
                    (0, 0) => sstr += tl,
                    (w, 0) if w==width-1 => sstr += &format!("{}", tc),
                    (w, _) if w==width-1 => sstr += &format!("{}", ve),
                    (0, h) if h==height-1=> sstr += "├",
                    (0, _) => sstr += &format!("{}", ve),
                    (_, 0) => sstr += ho,
                    (_, h) if h==height-1 => sstr += ho,
                    _ => sstr += " "
                }
            }
            strs.push(sstr.clone().to_string());
        }

        for s in strs.iter() {
            println!("{}", s);
        }
        strs
    }

    pub fn printstr_bl(&self) -> Vec<String> {
        let mut strs: Vec<String> = vec![];
        let tl = "┌";
        let tr = "┐";
        let bl = "└";
        let br = "┘";
        let ve = "│";
        let ho = "─";
        // j==0
        let tc = "┬";
        // j==height
        let bc = "┴";
        let cp = "┼";

        let mut str = "".to_string();

        let width = self.inner_width + 2;
        let height = self.inner_height + 2;
        for y in 0..height {
            let mut sstr = "".to_string();
            for x in 0..width {
                match (x, y) {
                    (w, h) if w==width-1 && h==height-1 => sstr += "┴",
                    (0, 0) => sstr += "├",
                    (w, 0) if w==width-1 => sstr += &format!("{}", "┼"),
                    (w, _) if w==width-1 => sstr += &format!("{}", ve),
                    (0, h) if h==height-1=> sstr += "└",
                    (0, _) => sstr += &format!("{}", ve),
                    (_, 0) => sstr += ho,
                    (_, h) if h==height-1 => sstr += ho,
                    _ => sstr += " "
                }
            }
            strs.push(sstr.clone().to_string());
        }

        for s in strs.iter() {
            println!("{}", s);
        }
        strs
    }

    pub fn printstr_br(&self) -> Vec<String> {
        let mut strs: Vec<String> = vec![];
        let tl = "┌";
        let tr = "┐";
        let bl = "└";
        let br = "┘";
        let ve = "│";
        let ho = "─";
        // j==0
        let tc = "┬";
        // j==height
        let bc = "┴";
        let cp = "┼";

        let mut str = "".to_string();

        let width = self.inner_width + 2;
        let height = self.inner_height + 2;
        for y in 0..height {
            let mut sstr = "".to_string();
            for x in 0..width {
                match (x, y) {
                    (w, h) if w==width-1 && h==height-1 => sstr += "┘",
                    (0, 0) => sstr += "┼",
                    (w, 0) if w==width-1 => sstr += &format!("{}", "┤"),
                    (w, _) if w==width-1 => sstr += &format!("{}", ve),
                    (0, h) if h==height-1=> sstr += "┴",
                    (0, _) => sstr += &format!("{}", ve),
                    (_, 0) => sstr += ho,
                    (_, h) if h==height-1 => sstr += ho,
                    _ => sstr += " "
                }
            }
            strs.push(sstr.clone().to_string());
        }

        for s in strs.iter() {
            println!("{}", s);
        }
        strs
    }

    pub fn printstr_tr(&self) -> Vec<String> {
        let mut strs: Vec<String> = vec![];
        let tl = "┌";
        let tr = "┐";
        let bl = "└";
        let br = "┘";
        let ve = "│";
        let ho = "─";
        // j==0
        let tc = "┬";
        // j==height
        let bc = "┴";
        let cp = "┼";

        let mut str = "".to_string();

        let width = self.inner_width + 2;
        let height = self.inner_height + 2;
        for y in 0..height {
            let mut sstr = "".to_string();
            for x in 0..width {
                match (x, y) {
                    (w, h) if w==width-1 && h==height-1 => sstr += "┤",
                    (0, 0) => sstr += "┬",
                    (w, 0) if w==width-1 => sstr += &format!("{}", "┐"),
                    (w, _) if w==width-1 => sstr += &format!("{}", ve),
                    (0, h) if h==height-1=> sstr += "┼",
                    (0, _) => sstr += &format!("{}", ve),
                    (_, 0) => sstr += ho,
                    (_, h) if h==height-1 => sstr += ho,
                    _ => sstr += " "
                }
            }
            strs.push(sstr.clone().to_string());
        }

        for s in strs.iter() {
            println!("{}", s);
        }
        strs
    }

    pub fn print(&self) {
        let tl = "┌";
        let tr = "┐";
        let bl = "└";
        let br = "┘";
        let ve = "│";
        let ho = "─";
        let mut str = "".to_string();

        let width = self.inner_width + 2;
        let height = self.inner_height + 2;
        for y in 0..height {
            for x in 0..width {
                match (x, y) {
                    (w, h) if w==width-1 && h==height-1 => str += br,
                    (0, 0) => str += tl,
                    (w, 0) if w==width-1 => str += &format!("{}\n", tr),
                    (w, _) if w==width-1 => str += &format!("{}\n", ve),
                    (0, h) if h==height-1=> str += bl,
                    (0, _) => str += &format!("{}", ve),
                    (_, 0) => str += ho,
                    (_, h) if h==height-1 => str += ho,
                    _ => str += " "
                }

            }
        }
        println!("{}", str);
    }
}
