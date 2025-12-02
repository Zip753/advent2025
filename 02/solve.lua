function GetPrev(n, k)
	if k > #n then
		return 0
	end
	-- get length
	local len = #n
	local kLen = math.floor(len / k)
	if len % k ~= 0 then
		local nines = math.pow(10, kLen) - 1
		return nines
	end

	local firstK = string.sub(n, 1, kLen)
	local firstKNum = tonumber(firstK)
	-- print(firstK)

	local smallerThanFirst = false
	for i = kLen + 1, len, kLen do
		local nextI = math.min(n, i + kLen - 1)
		local nextK = string.sub(n, i, nextI)
		-- print(i, nextI, nextK, kLen)
		local nextKNum = tonumber(nextK)
		if nextKNum < firstKNum then
			return firstKNum - 1
		end
		if nextKNum > firstKNum then
			return firstKNum
		end
	end

	return firstK
end

-- print(GetPrev("12341234", 4))
-- print(GetPrev("12341211", 4))
-- print(GetPrev("12341234", 3))
-- print(GetPrev("12341234", 2))
-- print(GetPrev("12341234", 8))
-- print(GetPrev("12341234", 9))

function GetSum(s)
	local first = string.match(s, "%d+")
	local second = string.match(s, "%d+", #first + 2)

	local sum = 0
	for i = first + 1, second, 1 do
		sum = sum + i + i * math.pow(10, #string.format("%d", i))
	end
	-- print(first, second, sum)
	return sum
end

function GetTotal(from, to)
	local all = {}
	for k = 2, 10, 1 do
		local fromMinusOne = string.format("%d", tonumber(from) - 1)
		local fromK = GetPrev(fromMinusOne, k)
		local toK = GetPrev(to, k)
		print("k", k, fromK + 1, toK)
		for i = fromK + 1, toK, 1 do
			local iS = string.format("%d", i)
			-- print(k, fromK, toK, iS)
			all[tonumber(string.rep(iS, k))] = true
		end
	end

	local total = 0
	local totalN = 0
	local allValues = {}
	for key in pairs(all) do
		table.insert(allValues, key)
		total = total + key
		totalN = totalN + 1
	end

	print("inputs", from, to)
	table.sort(allValues)
	for _, value in pairs(allValues) do
		print("all", value)
	end

	return total
end

-- print(GetTotal("111", "111"))
print(GetPrev("687053", 3))
print(GetTotal("687053", "834889"))
