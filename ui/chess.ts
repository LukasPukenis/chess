"use strict";
    
class BoardElement {
    kind: string;
    piece: string;

    constructor(kind: string, piece: string) {
        this.kind = kind;
        this.piece = piece;
    }
}

class RenderElement {
    figure: BoardElement | null;
    selected: boolean;
    possible: boolean;

    constructor(figure: BoardElement | null, selected: boolean, possible: boolean) {
        this.figure = figure;
        this.selected = selected;
        this.possible = possible;
    }
};

class State {

};

class DraggingState extends State {
    from: string;

    constructor(from: string) {
        super();
        this.from = from;
    }
}

class SelectedState extends State {
    active: string;
    possible_moves: string[];

    constructor(active: string, possible_moves: string[]) {
        super();
        this.active = active;
        this.possible_moves = possible_moves;
    }
}

class IdleState extends State{
    constructor () {
        super();
    }
}

class Game {
    state: State;
    board: RenderElement[][] = [];

    constructor() {
        this.state = new IdleState();
    }

    setState(state: State) {
        this.state = state;
        this._refresh();
    }
    
    setBoard(board: (BoardElement | null)[][]) {        
        // transform board into internal representation
        this.board = [];
        for (let i = 0; i < 8; i++) {
            this.board.push([]);
            for (let j = 0; j < 8; j++) {
                let element = board[i][j];
                if (element != null) {
                    
                    let selected = false;
                    let possible = false;

                    // TODO: checking `this.state` doesnt work 
                    switch (this.state.constructor.name) {
                        case "IdleState":        
                            break;
                        case "SelectedState":
                            selected = (this.state.active[0] == i && this.state.active[1] == j);
                            console.log(this.state.active[0], this.state.active[1], i, j, element);
                            possible = this.state.possible_moves.find(x => x[0] == i && x[1] == j)
                            break;
                        case "DraggingState":
                            selected = (this.state.active[0] == i && this.state.active[1] == j);
                            possible = this.state.possible_moves.find(x => x[0] == i && x[1] == j)
                            break;                            
                    }                    

                    
                    this.board[i].push(
                        new RenderElement(new BoardElement(
                            element['kind'],
                            element['piece']
                        ),
                        selected,
                        possible
                        )
                    );

                } else {
                    this.board[i].push(new RenderElement(null, false, false));
                }
            }
        }
        
        this.renderBoard(this.board);
    }

    run() {
        this.setState(new IdleState());
    }

    private _refresh() {
        fetch("/board").then(response => response.json()).then(data => {
            this.setBoard(data);            
        });
    }
    
    select(pos: string) {
        fetch(`/possible/${pos}`).then(response => response.json()).then(data => {
            console.log(data);
            
            this.state
            this._refresh();

            let moves = data;
            this.setState(new SelectedState(pos, moves));
        });            
    }

    start_dragging(pos: string) {
        this.setState(new DraggingState(pos));
    }

    end_dragging(pos: string) {
        if (this.state instanceof DraggingState) {

        } else {
            throw new Error("Invalid state");
        }

        this.handle_move(this.state.from, pos);
        this.setState(new IdleState());
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
            console.log(response);
        })
    }
    
    private renderBoard(elements: RenderElement[][]) {
        for (let i = 0; i < 8; i++) {
            for (let j = 0; j < 8; j++) {
                let element = elements[i][j];
                let box = document.getElementsByClassName("box")[i * 8 + j];
                let elem = box.getElementsByClassName('figure')[0];

                if (element.figure != null) {
                    let kind = element.figure['kind'].toLowerCase();
                    let piece = element.figure['piece'].toLowerCase();
                    elem.innerHTML = pieces[kind][piece];
                    
                    if (element.selected) {
                        box.classList.add('selected');
                    }

                    if (element.possible) {
                        console.log('possible');
                        box.classList.add('possible');
                    }

                    if (element.selected && element.possible) {
                        throw new Error("Selected and possible can't be true at the same time");
                    }
                } else {
                    elem.innerHTML = '';
                }
            }
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
        <div draggable="true" class="box {color}" data-pos="{pos}">
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
            game.select(event.target!.dataset.pos);
        });
        
        element.addEventListener('dragstart', (event) => {
            // @ts-ignore
            game.start_dragging(event.target!.dataset.pos);
        });

        element.addEventListener('dragover', (event) => {
            event.preventDefault();
        });

        element.addEventListener('drop', (event) => {
            event.preventDefault();                    
            // @ts-ignore
            game.end_dragging(event.target.dataset.pos);                        
        });

    }


    game.run();
});