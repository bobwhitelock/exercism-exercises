module RomanNumerals exposing (toRoman)


toRoman : Int -> String
toRoman number =
    let
        digits =
            toString number
                |> String.padLeft 4 '0'
                |> String.toList
                |> List.reverse

        ( ones, tens, hundreds, thousands ) =
            case digits of
                ones :: tens :: hundreds :: thousands :: rest ->
                    ( ones, tens, hundreds, thousands )

                _ ->
                    -- Should never happen, but this is something reasonable in
                    -- case it somehow did.
                    ( '0', '0', '0', '0' )
    in
    String.join ""
        [ thousandsToRoman thousands
        , hundredsToRoman hundreds
        , tensToRoman tens
        , onesToRoman ones
        ]


thousandsToRoman : Char -> String
thousandsToRoman =
    -- Placeholders for 'five' and 'ten' symbols for thousands, since don't
    -- need to handle more than ~3000.
    unitToRoman ( 'M', '0', '0' )


hundredsToRoman : Char -> String
hundredsToRoman =
    unitToRoman ( 'C', 'D', 'M' )


tensToRoman : Char -> String
tensToRoman =
    unitToRoman ( 'X', 'L', 'C' )


onesToRoman : Char -> String
onesToRoman =
    unitToRoman ( 'I', 'V', 'X' )


unitToRoman : ( Char, Char, Char ) -> Char -> String
unitToRoman ( oneSymbol, fiveSymbol, tenSymbol ) unit =
    let
        symbols =
            case unit of
                '1' ->
                    one

                '2' ->
                    two

                '3' ->
                    three

                '4' ->
                    List.append one five

                '5' ->
                    five

                '6' ->
                    List.append five one

                '7' ->
                    List.append five two

                '8' ->
                    List.append five three

                '9' ->
                    List.append one ten

                _ ->
                    []

        one =
            [ oneSymbol ]

        two =
            [ oneSymbol, oneSymbol ]

        three =
            [ oneSymbol, oneSymbol, oneSymbol ]

        five =
            [ fiveSymbol ]

        ten =
            [ tenSymbol ]
    in
    String.fromList symbols
