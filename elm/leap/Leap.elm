module Leap exposing (..)


isLeapYear : Int -> Bool
isLeapYear year =
    let
        yearDivisibleBy =
            divisibleBy year
    in
        yearDivisibleBy 400
            || (not (yearDivisibleBy 100) && yearDivisibleBy 4)


divisibleBy : Int -> Int -> Bool
divisibleBy dividend divisor =
    dividend % divisor == 0
