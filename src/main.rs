use texttable::cell::*;

fn main() {
    let s = cell(5, 5);
    let t = Table::new(5, 20);
    //t.print();
    let c = Cell::new(1, 1);
    c.printstr_tl();
    c.printstr_bl();
    c.printstr_tr();
    c.printstr_br();
}
