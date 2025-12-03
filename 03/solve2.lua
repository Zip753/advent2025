function GetBestJoltage(s)
	local startIdx = 0

	local solution = ""

	for i = 1, 12, 1 do
		local endIdx = #s - 12 + i
		local maxIdx = startIdx + 1

		for j = startIdx + 1, endIdx do
			-- print(s:sub(j, j), s:sub(maxIdx, maxIdx))
			if tonumber(s:sub(j, j)) > tonumber(s:sub(maxIdx, maxIdx)) then
				maxIdx = j
			end
		end

		solution = solution .. s:sub(maxIdx, maxIdx)
		startIdx = maxIdx
	end

	return solution
end

print(GetBestJoltage("1234123412341234123412341234123412341234"))

print(GetBestJoltage("987654321111111"))
print(GetBestJoltage("811111111111119"))
print(GetBestJoltage("234234234234278"))
print(GetBestJoltage("818181911112111"))
