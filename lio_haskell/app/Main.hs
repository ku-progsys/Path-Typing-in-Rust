{-# LANGUAGE OverloadedStrings #-}
module Main where 

import LIO
import LIO.TCB
import LIO.DCLabel
import System.Environment (getArgs)
import Data.Maybe (fromMaybe)


-- | Simple secrecy component example
s :: CNF
s =  ("Alice" \/ "Bob") /\  "Carla"

-- | Simple integrity component example
i :: CNF
i = "Alice" /\ "Carla"

-- | Simple label
l1 :: DCLabel
l1 = s %% i

-- | Simple label
l2 :: DCLabel
l2 = "Djon" %% "Alice"

-- | Creating privilege using constructor from TCB
p :: DCPriv
p = PrivTCB $ "Alice" /\ "Carla"

-- | Sanitize function (example transformation: strip integrity)
sanitize :: DCLabel -> DCLabel
sanitize (DCLabel s' _) = DCLabel s' cTrue -- keep secrecy, discard integrity

-- | Declassify function (example: reduce secrecy using privileges)
declassify :: DCPriv -> DCLabel -> DCLabel
declassify = downgradeP



main :: IO ()
main = do
  args <- getArgs
  let shouldSanitize = case args of
                         ("sanitize":_) -> True
                         _              -> False

  let labelBefore = l1
  let labelUsed = if shouldSanitize then sanitize labelBefore else labelBefore

  let declassifiedLabel =
        if shouldSanitize
           then declassify p labelUsed  -- Declassify only if sanitize path taken
           else labelUsed               -- No declassification allowed

  putStrLn $ "Declassified label: " ++ show declassifiedLabel
