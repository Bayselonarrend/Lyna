-- Математический пакет для Lua

local math_utils = {}

-- Факториал
function math_utils.factorial(n)
    if type(n) ~= "number" or n < 0 or n ~= math.floor(n) then
        error("Факториал определен только для неотрицательных целых чисел")
    end
    
    if n <= 1 then
        return 1
    else
        return n * math_utils.factorial(n - 1)
    end
end

-- Числа Фибоначчи
function math_utils.fibonacci(n)
    if type(n) ~= "number" or n < 0 or n ~= math.floor(n) then
        error("Число Фибоначчи определено только для неотрицательных целых чисел")
    end
    
    if n <= 1 then
        return n
    else
        return math_utils.fibonacci(n - 1) + math_utils.fibonacci(n - 2)
    end
end

-- Проверка на простое число
function math_utils.is_prime(n)
    if type(n) ~= "number" or n < 2 or n ~= math.floor(n) then
        return false
    end
    
    for i = 2, math.sqrt(n) do
        if n % i == 0 then
            return false
        end
    end
    return true
end

-- НОД (наибольший общий делитель)
function math_utils.gcd(a, b)
    if type(a) ~= "number" or type(b) ~= "number" then
        error("НОД определен только для чисел")
    end
    
    a, b = math.abs(a), math.abs(b)
    while b ~= 0 do
        a, b = b, a % b
    end
    return a
end

-- НОК (наименьшее общее кратное)
function math_utils.lcm(a, b)
    if type(a) ~= "number" or type(b) ~= "number" then
        error("НОК определен только для чисел")
    end
    
    return math.abs(a * b) / math_utils.gcd(a, b)
end

-- Регистрируем пакет глобально
_G.math_utils = math_utils

-- Возвращаем пакет
return math_utils