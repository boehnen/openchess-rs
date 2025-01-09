# Running the OpenChess API
The OpenChess API is implemented in Rust. To pull the code and run the API locally, follow these steps:

### Prerequisites
Ensure the following are installed on your system:
- Rust and Cargo: Install Rust via [rustup](https://rustup.rs/).
- Git: Install Git to clone the repository.

### Clone the Repository
Use Git to clone the API repository:
``` bash
git clone https://github.com/boehnen/openchess-rs.git
```
``` bash
cd openchess-rs
```

### Build and Run the API
Build the project: 
``` bash
cargo build --release
```

### Run the API:
``` bash
cargo run --release
```
By default, the API will start on `http://127.0.0.1:8080`. You can adjust the port or host in the configuration file or by setting environment variables.

### Test the API
Once the API is running, test it using a curl command:
``` bash
curl -X GET 'http://127.0.0.1:8080?fen=r1bqkbnr/pppppppp/2n5/8/8/2N5/PPPPPPPP/R1BQKBNR'
```
You should receive an SVG image of the chessboard as a response.

# Public API Usage
OpenChess is a RESTful API that generates chessboard images in SVG format based on the board position provided in FEN notation.

### Example Request
To use the API, send a GET request with the desired board position in FEN notation using the fen query parameter:
``` bash
curl -X GET 'https://chess.boehnen.net?fen=r1bqkbnr/pppppppp/2n5/8/8/2N5/PPPPPPPP/R1BQKBNR'
```
Replace {fen} with the board position in FEN notation.

### Response
The `200 OK` response is an SVG image of the chessboard representing the specified FEN position. The image can be saved or embedded in a webpage, and scaled to any desired resolution.

### Query Parameters
The API supports the following query parameters:

| Parameter Name | Optionality | Description |
|----------------|-------------|----------------------------------------------| 
|'fen' | Required | The board position in FEN notation. |
| 'theme' | Optional | Specify a board theme (`classic` or `modern`). |
| 'rotation' | Optional | Rotate the board in 90 degree increments (`0`, `90`, `180` or `270`). | 
| 'labels' | Optional | Include coordinate labels on the board (`true` or `false`). |

### FEN
```
r1bqkbnr/pppppppp/2n5/8/8/2N5/PPPPPPPP/R1BQKBNR
```
This represents the following chess position:
- Black knight on `c6`
- White knight on `c3`
- Pawns in their initial positions.

The FEN string can be simplified to include only the piece positions, omitting the following components:
- Active player (w or b)
- Castling availability (KQkq)
- En passant target square (e.g., e3)
- Halfmove clock and fullmove number (0 1)
While these components can still be included in the fen query, they are ignored by the API as they are not relevant to rendering the chessboard.

### Example of saving the response:
``` bash
curl -X GET 'https://chess.boehnen.net?fen=r1bqkbnr/pppppppp/2n5/8/8/2N5/PPPPPPPP/R1BQKBNR' -o board.svg
```

# Contributing
- Found a bug? Have an idea for improvement? [Open an issue](https://github.com/boehnen/openchess-rs/issues) to let us know.
- Want to contribute code? Fork the repository, make your changes, and [submit a pull request](https://github.com/boehnen/openchess-rs/pulls).
- Have a feature in mind? [Request it](https://github.com/boehnen/openchess-rs/issues) through our issue tracker.
