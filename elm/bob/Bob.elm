module Bob exposing (..)

import Regex exposing (regex, contains)


type InputType
    = Statement
    | Shout
    | Question
    | Silence


hey : String -> String
hey input =
    let
        inputType =
            classify input
    in
        responseFor inputType


classify : String -> InputType
classify input =
    if
        (contains (regex "[a-zA-Z]") input)
            && (String.toUpper input == input)
    then
        Shout
    else if String.trim input == "" then
        Silence
    else if String.endsWith "?" input then
        Question
    else
        Statement


responseFor : InputType -> String
responseFor inputType =
    case inputType of
        Statement ->
            "Whatever."

        Shout ->
            "Whoa, chill out!"

        Question ->
            "Sure."

        Silence ->
            "Fine. Be that way!"
