use rltk::{Rltk, GameState, Console};

struct State {}
impl GameState for State {
    fn tick(&mut self, ctx : &mut Rltk) {
        // clear the screen. Generally do at the beginning of a frame
        ctx.cls();
        // print the at location (1,1)
        ctx.print(1, 1, "Hello Rust World");
    }
}

fn main() {
    use rltk::RltkBuilder;
    // make a terminal that 80 characters wide x 50 characters high
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial") // title of the window
        .build();
    let gs = State{ }; // game state
    rltk::main_loop(context, gs);

    /*
    The main_loop function takes control and calls the tick function at every 
    cycle. That could be more than 60 times per second. This probably depends 
    on the framerate of the game.
    */
}
