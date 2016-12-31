module SumOfMultiples exposing (..)

import Set


sumOfMultiples : List Int -> Int -> Int
sumOfMultiples numbers limit =
    List.map (multiplesUpToLimit limit) numbers
        |> List.concat
        -- Convert to/from Set to get rid of duplicates.
        |>
            Set.fromList
        |> Set.toList
        |> List.sum


multiplesUpToLimit : Int -> Int -> List Int
multiplesUpToLimit limit number =
    let
        possibleMaxMultiplier =
            floor (toFloat limit / toFloat number)

        maxMultiplier =
            if multipleOf possibleMaxMultiplier == limit then
                -- We don't want to include the limit in the multipliers.
                possibleMaxMultiplier - 1
            else
                possibleMaxMultiplier

        multipliers =
            List.range 1 maxMultiplier

        multipleOf =
            \n -> n * number
    in
        List.map multipleOf multipliers
