module Triangle exposing (..)

import Debug
import Set


type Triangle
    = Equilateral
    | Isosceles
    | Scalene


triangleKind : Float -> Float -> Float -> Result String Triangle
triangleKind side1 side2 side3 =
    let
        sides =
            [ side1, side2, side3 ]

        numberEqualSides =
            equalSides sides
    in
        if invalidLengths sides then
            Err "Invalid lengths"
        else if violatesTriangleInequality sides then
            Err "Violates inequality"
        else if numberEqualSides 3 then
            Ok Equilateral
        else if numberEqualSides 2 then
            Ok Isosceles
        else
            Ok Scalene


invalidLengths : List Float -> Bool
invalidLengths sides =
    List.any (\s -> s <= 0) sides


violatesTriangleInequality : List Float -> Bool
violatesTriangleInequality sides =
    let
        inequalityHoldsForSide =
            \s -> True

        longestSide =
            List.maximum sides
                |> Maybe.withDefault 0

        otherSides =
            List.sort sides
                |> List.reverse
                |> List.tail
                |> Maybe.withDefault []
    in
        not (longestSide <= (List.sum otherSides))


equalSides : List Float -> Int -> Bool
equalSides sides amountShouldBeEqual =
    let
        numberUniqueSideLengths =
            Set.size (Set.fromList sides)

        numberEqualSides =
            4 - numberUniqueSideLengths
    in
        amountShouldBeEqual == numberEqualSides
