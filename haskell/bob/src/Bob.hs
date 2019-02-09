module Bob
  ( responseFor
  ) where

import Data.Char (isSpace, toLower, toUpper)
import Data.List (dropWhileEnd)

responseFor :: String -> String
responseFor input =
  let trimmedInput = trim input
      upperInput = map toUpper trimmedInput
      lowerInput = map toLower trimmedInput
      someLetters = upperInput /= lowerInput
      isSilence = trimmedInput == ""
      isShout = someLetters && upperInput == trimmedInput
      isQuestion = last trimmedInput == '?'
   in if isSilence
        then "Fine. Be that way!"
        else if isShout && isQuestion
               then "Calm down, I know what I'm doing!"
               else if isQuestion
                      then "Sure."
                      else if isShout
                             then "Whoa, chill out!"
                             else "Whatever."

trim :: String -> String
trim = dropWhileEnd isSpace . dropWhile isSpace
