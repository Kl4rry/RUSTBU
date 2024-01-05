#[allow(unused)]
pub (crate) struct Board
{
    color: Color,
    home: Color,
    state: Vec<Vec<Option<Stone>>>
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub (crate) struct Stone
{
    color: Color,
    position: (usize, usize) 
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Color 
{
    White,
    Black,
}

impl Board
{
    //Creates a new board
    pub fn new_board(board_color: Color, home_color: Color) -> Board
    {
        return Board
        {
            color: board_color,
            home: home_color,
            state: Board::new_state()
        }
    }

    //Create default board.
    fn new_state() -> Vec<Vec<Option<Stone>>>
    {
        //Creates a new empty boardstate.
        let mut board: Vec<Vec<Option<Stone>>> = 
        vec![
            vec![None, None, None, None],
            vec![None, None, None, None],
            vec![None, None, None, None],
            vec![None, None, None, None],
        ];

        //Add rocks
        for i in 0..4 
        {
            //Top row (White)
            board[0][i] = Some(Stone::new(Color::White,  (0, i)));

            //Bottom row (Black)
            board[3][i] = Some(Stone::new(Color::Black,  (0, i)));
        }

        /*
        Detta returnar:
        [W][W][W][W]
        [ ][ ][ ][ ]
        [ ][ ][ ][ ]
        [B][B][B][B]

        Boarden displayas
        flipped för white.
         */
        return board;
    }

    pub fn get_state(&self) -> &Vec<Vec<Option<Stone>>>
    {
        return &self.state;
    }
}

impl Stone
{
    pub (crate) fn new(c: Color, pos: (usize, usize)) -> Stone
    {
        return Stone
        {
            color: c,
            position: pos
        }
    }
    
    pub fn get_color(&self) -> Color
    {
        return self.color;
    }

    pub fn passive_move() -> ()
    {
        /*
        Detta kommer ge typ:
        [W][B][ ][B]
        [ ][W][ ][ ]
        [ ][ ][ ][ ]
        [ ][w][B][W]

        Om jag väljer lilla w ska jag ha movement i 
        1x vänster, 1x vänster upp, 1x upp, 2x upphöger, 0x resten.
        */
    }

    fn get_possible_moves(&self, b: &Board, aggr: bool) -> ()
    {
        let mut boardstate = &b.get_state();
        let cur_pos = self.position; //0 = x, 1 = y
        let mut movelist: Vec<(usize, usize)> = Vec::new();
    
        //todo: a move to the left is a [1, 0] 
        //that gets added to the rock pos if movement.

        //check North & South
        for i in -2..3 as i8
        {
            let newpos = (self.position.0 as i8, self.position.1 as i8 + i);

            //Check if in range.
            if newpos.1 < 0 || newpos.1 > 4
            {
                continue;
            }

            //If not empty.
            if boardstate[newpos.0 as usize][newpos.1 as usize] != None
            {
                continue;
            }
        }

        //check east & West

        //check NE & SW

        //check NW & SE
    }

    pub fn aggressive_move() -> ()
    {
        //todo: Somehow kolla vad färgen på tidigare boarden var.
        //todo: move buffer för passiv och aggressiv?
    }
}