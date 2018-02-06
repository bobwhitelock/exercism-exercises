module Isogram exposing (..)

import Dict
import Regex
import Tuple


isIsogram : String -> Bool
isIsogram word =
    let
        alphabeticalCharRegex =
            Regex.regex "[a-z]"

        updateCount =
            \char dict ->
                if Dict.member char dict then
                    Dict.update char (Maybe.map ((+) 1)) dict
                else
                    Dict.insert char 1 dict
    in
    String.toLower word
        |> String.filter (toString >> Regex.contains alphabeticalCharRegex)
        |> String.toList
        |> List.foldl updateCount Dict.empty
        |> Dict.toList
        |> List.map Tuple.second
        |> List.any (flip (>) 1)
        |> not
