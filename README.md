# OpenChess API Usage
OpenChess is a RESTful API that generates chessboard images in SVG format based on the board position provided in FEN notation.

## Example Request
To use the API, send a GET request with the desired board position in FEN notation using the fen query parameter:
``` bash
curl -X GET 'https://chess.boehnen.net?fen={fen}'
```
Replace {fen} with the board position in FEN notation.

## Query Parameters
The API supports the following query parameters:

| Parameter Name | Optionality | Description |
|----------------|-------------|----------------------------------------------| 
|'fen' | Required | The board position in FEN notation. |
| 'theme' | Optional | Specify a board theme (`classic` or `modern`). |
| 'rotation' | Optional | Rotate the board in 90 degree increments (`0`, `90`, `180` or `270`). | 
| 'labels' | Optional | Include coordinate labels on the board (`true` or `false`). |

## Example FEN
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

## Example Request with FEN
``` bash
curl -X GET 'https://chess.boehnen.net?fen=r1bqkbnr/pppppppp/2n5/8/8/2N5/PPPPPPPP/R1BQKBNR'
```

### Response
The `200 OK` response is an SVG image of the chessboard representing the specified FEN position. The image can be saved or embedded in a webpage, and scaled to any desired resolution.

## Example of saving the response:
``` bash
curl -X GET 'https://chess.boehnen.net?fen=r1bqkbnr/pppppppp/2n5/8/8/2N5/PPPPPPPP/R1BQKBNR' -o board.svg
```
