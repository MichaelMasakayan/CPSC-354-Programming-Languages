{-let ezample_eveny =  ["a","b","c","d","e"] -}
{-yelect_eveny (z:zy) n =  z !!n if z !! n/2 == % 0 then b:a!! n 
-}
select_evens::  [a] -> [a]
select_evens [] = []
select_evens [x] = []
select_evens (x1:x2:xy) = x2:select_evens xy
-- selects all the odd elements in a list
select_odds ::  [a] -> [a]
select_odds [] = []
select_odds [x] = [x]
select_odds (x1:x2:xy) = x2:select_odds xy
-- member takes an int and a list and checks if the int is in the list

member:: Eq a=> a-> [a] -> Bool
member _ [] = False
member x (z:zs)=  (x `elem` zs) == True



-- combines two lists into one list
append ::  [a] -> [a] -> [a]
append [] xy = xy
append (z:zy) xy = z:append xy zy
-- reverses the order of a list
revert :: [a] -> [a]
revert xs = rev [] xs where
  rev :: [a] -> [a] -> [a] 
  rev acc    []  =       acc
  rev acc (x:xs) = rev (x:acc) xs
--compares the values of the lists
less_equal:: Ord a=> [a] -> [a] -> Bool
less_equal  [][] = True
less_equal [x][b] = (x>=b)
-- less_equal (x:xs) (b:bz) = x<b:less_equal xs bz



main::IO ()
main =
   -- print(select_evens ["e","f","g","h","i"])
    -- print(select_odds ["e","f","g","h","i"])

-- print(select_odds ["a","b","c","d","e"])
-- print (revert [1,2,3]) 
     print(append [1,2] [3,4,5])
-- print(less_equal [][])
-- print(select_odds ["a","b","c","d","e"])
-- print (revert [1,2,3])
