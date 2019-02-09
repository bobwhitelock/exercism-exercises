module Anagram
  ( anagramsFor
  ) where

import Data.Char (toLower)
import Data.List (sort)

anagramsFor :: String -> [String] -> [String]
anagramsFor subject candidates = filter (isAnagram subject) candidates

isAnagram :: String -> String -> Bool
isAnagram subject candidate =
  let normalize = sort . lower
      lower = map toLower
   in normalize subject == normalize candidate &&
      lower subject /= lower candidate
