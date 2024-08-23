
local function toBits(num)
	local bits = {}

	for i = 1, 8 do
		local b = bit.band(num, 1)
		bits[i] = b
		num = bit.arshift(num, 1)
	end

	return bits
end

local function printBits(bits)
	for b = 1, 8 do
		io.write(bits[b].." ")
	end
	print()
end

local num = tonumber(arg[1], 16)
for b = 1, 8 do
	printBits(toBits(bit.band(num, 0xFF)))
	num = bit.arshift(num, 8)
end
