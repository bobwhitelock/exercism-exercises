module ScrabbleScore exposing (..)

import Dict exposing (Dict)


scoreWord : String -> Int
scoreWord word =
    String.toLower word
        |> String.toList
        |> List.map scoreLetter
        |> List.sum


scoreLetter : Char -> Int
scoreLetter letter =
    Dict.get letter lettersToScores
        |> Maybe.withDefault 0


lettersToScores : Dict Char Int
lettersToScores =
    List.concat
        [ (lettersWorthPoints [ 'a', 'e', 'i', 'o', 'u', 'l', 'n', 'r', 's', 't' ] 1)
        , (lettersWorthPoints [ 'd', 'g' ] 2)
        , (lettersWorthPoints [ 'b', 'c', 'm', 'p' ] 3)
        , (lettersWorthPoints [ 'f', 'h', 'v', 'w', 'y' ] 4)
        , (lettersWorthPoints [ 'k' ] 5)
        , (lettersWorthPoints [ 'j', 'x' ] 8)
        , (lettersWorthPoints [ 'q', 'z' ] 10)
        ]
        |> Dict.fromList


lettersWorthPoints : List Char -> Int -> List ( Char, Int )
lettersWorthPoints letters points =
    let
        letterPointsTuple =
            \points -> \letter -> ( letter, points )
    in
        List.map (letterPointsTuple points) letters
