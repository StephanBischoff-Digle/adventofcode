import           System.IO

parse :: [String] -> [Int]
parse = map read

windowedReduce :: [Int] -> Int
windowedReduce [a, b] = if a < b then 1 else 0
windowedReduce (a : b : xs) =
  let v = if a < b then 1 else 0 in v + windowedReduce (b : xs)

main :: IO ()
main = do
  let list = []
  handle   <- openFile "input.txt" ReadMode
  contents <- hGetContents handle
  let singlewords = words contents
      list        = parse singlewords
  print (windowedReduce list)
  hClose handle
