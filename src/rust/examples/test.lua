-- Пример Lua скрипта для тестирования

-- Простая функция
function greet(name)
    return "Привет, " .. (name or "Мир") .. "!"
end

-- Математические операции
function calculate(a, b, operation)
    if operation == "add" then
        return a + b
    elseif operation == "subtract" then
        return a - b
    elseif operation == "multiply" then
        return a * b
    elseif operation == "divide" then
        if b ~= 0 then
            return a / b
        else
            error("Деление на ноль!")
        end
    else
        error("Неизвестная операция: " .. operation)
    end
end

-- Работа с таблицами
function process_array(arr)
    local result = {}
    for i, v in ipairs(arr) do
        result[i] = v * 2
    end
    return result
end

-- Работа с объектами
function process_object(obj)
    local result = {}
    for k, v in pairs(obj) do
        if type(v) == "number" then
            result[k] = v + 10
        elseif type(v) == "string" then
            result[k] = "processed_" .. v
        else
            result[k] = v
        end
    end
    return result
end

-- Глобальные переменные для тестирования
test_number = 42
test_string = "Тестовая строка"
test_boolean = true

-- Возвращаем результат выполнения
return {
    message = "Скрипт успешно загружен",
    functions = {"greet", "calculate", "process_array", "process_object"},
    globals = {"test_number", "test_string", "test_boolean"}
}