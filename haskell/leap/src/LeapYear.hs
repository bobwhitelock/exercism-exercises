module LeapYear
  ( isLeapYear
  ) where

isLeapYear :: Integer -> Bool
isLeapYear year =
  let isDivisibleBy' = isDivisibleBy year
   in if isDivisibleBy' 100 && not (isDivisibleBy' 400)
        then False
        else if isDivisibleBy' 4
               then True
               else False

isDivisibleBy :: Integer -> Integer -> Bool
isDivisibleBy value divisor = mod value divisor == 0
