module Raindrops exposing (..)

import List


type alias Sound =
    { divisor : Int
    , sound : String
    }


raindrops : Int -> String
raindrops numberRaindrops =
    let
        sounds =
            [ Sound 3 "Pling"
            , Sound 5 "Plang"
            , Sound 7 "Plong"
            ]

        raindropsMakeSound =
            makesSound numberRaindrops

        soundsMade =
            List.filter raindropsMakeSound sounds
                |> List.map (\s -> s.sound)
    in
        if List.isEmpty soundsMade then
            toString numberRaindrops
        else
            String.concat soundsMade


makesSound : Int -> Sound -> Bool
makesSound raindrops sound =
    raindrops % sound.divisor == 0
