module Anagram exposing (..)


detect : String -> List String -> List String
detect input potentials =
    List.filter (isAnagramOf input) potentials


isAnagramOf : String -> String -> Bool
isAnagramOf input potential =
    notIdentical input potential && normalized input == normalized potential


notIdentical : String -> String -> Bool
notIdentical string1 string2 =
    String.toLower string1 /= String.toLower string2


normalized : String -> String
normalized string =
    String.toLower string
        |> String.toList
        |> List.sort
        |> String.fromList
