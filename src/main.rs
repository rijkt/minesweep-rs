mod board;

fn main() {
    let board = board::gen_board(10, 10, 10);
    assert_eq!(board.len(), 100);
    println!("{:?}", board)
}
