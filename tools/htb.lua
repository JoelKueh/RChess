
local function toBits(num1)
	local bits = {}

	for i = 1, 8 do
		local b = bit.band(num1, 1)
		bits[i] = b
		num1 = bit.rshift(num1, 1)
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
	print(num)
	printBits(toBits(bit.band(num, 0xFF)))
	num = bit.rshift(num, 8)
end
