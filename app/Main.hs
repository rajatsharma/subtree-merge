module Main where

import Control.Monad (forM_)
import System.Environment (getArgs)
import System.Process (callProcess)

main :: IO ()
main = do
  args <- getArgs
  forM_ args $ \arg -> do
    callProcess "git" ["subtree", "add", "--prefix=" ++ arg, "git@github.com:rajatsharma/" ++ arg ++ ".git", "master"]
