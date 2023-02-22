"use strict";
    
class API_BoardElement {
    color: string;
    piece: string;

    constructor(color: string, piece: string) {
        this.color = color;
        this.piece = piece;
    }
}

interface RenderBoard {
    [pos: string]: RenderElement;
}

class RenderElement {
    figure: API_BoardElement | null;
    selected: boolean;
    possible: boolean;

    constructor(figure: API_BoardElement | null, selected: boolean, possible: boolean) {
        this.figure = figure;
        this.selected = selected;
        this.possible = possible;
    }
};

class State {

};

class SelectedState extends State {
    active: string;
    possible_moves: string[];

    constructor(active: string, possible_moves: string[]) {
        super();
        this.active = active; // TODO
        this.possible_moves = possible_moves;
    }
}

class IdleState extends State{
    constructor () {
        super();
    }
}

interface API_Board {
    [key: string]: API_BoardElement;
}

class Game {
    state: State;
    
    constructor() {
        this.state = new IdleState();
    }

    setState(state: State) {
        this.state = state;
        this._refresh();
    }
    
    setBoard(board: API_Board) {        
        // transform board into internal representation
        
        let render_board: RenderBoard = {};
        for (let i = 1; i < 9; i++ ) {
            for (let j = 0; j < 8; j++) {
                // map j to ascii
                let char = String.fromCharCode(j+97);
                let pos: string = char+i;

                let selected = false;
                let possible = false;
                
                // TODO: checking `this.state` doesnt work 
                switch (this.state.constructor.name) {
                    case "IdleState":        
                        break;
                    case "SelectedState":
                        selected = this.state.active == pos;
                        possible = this.state.possible_moves.find(p => p == pos);
                        break;                    
                }                    

                let element = board[pos];
                if (element) {                                        
                    let color = element.color;
                    let piece = element.piece;
                    
                    render_board[pos] = new RenderElement(new API_BoardElement(
                        color, piece
                    ), selected, possible);
                } else {
                    render_board[pos] = new RenderElement(null, selected, possible);
                }
            }
        }

        this.renderBoard(render_board);
    }

    run() {
        this.setState(new IdleState());
    }

    private _refresh() {
        fetch("/board").then(response => response.json()).then(data => {
            this.setBoard(data);            
        });
    }
    
    clicked(pos: string) {
        if (this.state instanceof SelectedState) {
            this.handle_move(this.state.active, pos);
            this.state = new IdleState();
        } else {
            // select

            this.select(pos);
        }
    }

    select(pos: string) {
        fetch(`/moves/${pos}`).then(response => response.json()).then(data => {            
            this.state
            this._refresh();

            let moves = data;
            this.setState(new SelectedState(pos, moves));
        });            
    }
    
    handle_move(from: string, to: string) {
        fetch("/move", {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({
                from: from,
                to: to
            })
        }).then(response => {
            this._refresh();
        })
    }
    
    private renderBoard(board: RenderBoard) {
        let boxes = document.getElementsByClassName("box");
        for (let i = 0; i < boxes.length; i++) {
            let box = boxes.item(i);
            // reset
            box?.classList.remove('selected');
            box?.classList.remove('possible');
            // @ts-ignore
            box?.getElementsByClassName("figure")[0].innerHTML = '';
            
            // @ts-ignore
            let box_pos = box.dataset.pos;            
            let item = board[box_pos];
            
            if (item.figure != null) {
                let fig = item.figure;
                // @ts-ignore
                box?.getElementsByClassName("figure")[0].innerHTML = pieces[fig.color.toLowerCase()][fig.piece.toLowerCase()];
            }

            if (item.selected) {
                box?.classList.add('selected');
            }

            if (item.possible) {
                box?.classList.add('possible');
            }            

            // console.assert(!(item.selected && item.possible));

        }                    
    }
};

let game = new Game();

// make a multiplication table out of table element with js

const ascii_pieces = ['♖','♘','♗','♕','♔','♙','♜','♞','♝','♛','♚','♟'];
const pieces: any = {
    "white": {
        "rook": ascii_pieces[0],
        "knight": ascii_pieces[1],
        "bishop": ascii_pieces[2],
        "queen": ascii_pieces[3],
        "king": ascii_pieces[4],
        "pawn": ascii_pieces[5]
    },
    "black": {
        "rook": ascii_pieces[6],
        "knight": ascii_pieces[7],
        "bishop": ascii_pieces[8],
        "queen": ascii_pieces[9],
        "king": ascii_pieces[10],
        "pawn": ascii_pieces[11]
    }
};


document.addEventListener('DOMContentLoaded', (event) => {
    const template = `
        <div class="box {color}" data-pos="{pos}">
            <div class="nopointer">
                <div class="indicator-top-left">{indicator-top-left}</div>
                <div class="indicator-bottom-right">{indicator-bottom-right}</div>
                <div class="figure">{figure}</div>                        
            </div>
        </div>
    `;

    let board = document.getElementById("board")!;
    let html = [];
    for (let i = 0; i < 8; i++) {
        html.push('<div class="row">');
        for (let j = 0; j < 8; j++) {
            let indicator_top_left = '';
            let indicator_bottom_right = '';
            if (j == 0) {
                indicator_top_left = (8-i).toString();
            }
            
            if (i == 7) {
                indicator_bottom_right = String.fromCharCode(97+j-0);
            }
            const color = (i+j) % 2 == 0 ? 'even': 'odd';
            const figure = '';
            const pos = `${String.fromCharCode(97+j-0)}${8-i}`;

            html.push(template
                .replace('{color}', color)
                .replace('{indicator-top-left}', indicator_top_left)
                .replace('{indicator-bottom-right}', indicator_bottom_right)
                .replace('{figure}', figure)
                .replace('{pos}', pos)
                );
        }
        html.push('</div>');
    }

    board.innerHTML = html.join(" ");

    let elements = document.getElementsByClassName('box');
    for (let i in elements) {
        let element = elements.item( parseInt(i))!;
        
        element.addEventListener('click', (event) => {
            // @ts-ignore
            game.clicked(event.target!.dataset.pos);
        });
    }


    game.run();
});