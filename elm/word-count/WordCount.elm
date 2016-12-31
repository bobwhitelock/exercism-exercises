module WordCount exposing (..)

import Dict exposing (Dict)
import Regex


wordCount : String -> Dict String Int
wordCount input =
    let
        keepOnlyValidChars =
            Regex.replace
                Regex.All
                (Regex.regex "[^ a-z0-9]")
                (\_ -> "")

        incrementCount =
            \previous ->
                Maybe.withDefault 0 previous
                    |> (+) 1
                    |> Just

        updateCount =
            \word ->
                \dict ->
                    Dict.update word incrementCount dict
    in
        String.toLower input
            |> keepOnlyValidChars
            |> String.words
            |> List.foldl updateCount Dict.empty
