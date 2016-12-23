module Pangram exposing (..)

import Char
import Regex
import Set


isPangram : String -> Bool
isPangram input =
    let
        asciiInputChars =
            String.toLower input
                |> Regex.replace Regex.All (Regex.regex "[^a-z]") (\_ -> "")
                |> String.toList
                |> Set.fromList

        aCharCode =
            Char.toCode 'a'

        zCharCode =
            Char.toCode 'z'

        asciiAlphabetChars =
            List.range aCharCode zCharCode
                |> List.map Char.fromCode
                |> Set.fromList
    in
        asciiInputChars == asciiAlphabetChars
