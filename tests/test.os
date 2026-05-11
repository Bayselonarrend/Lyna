
	#Использовать "../src/os"

	Lua = Новый Lua("LuaJIT");
	
	Код =
	"local ffi = require(""ffi"")
	|ffi.cdef[[
	|typedef void* HWND;
	|typedef unsigned int UINT;
	|typedef unsigned long DWORD;
	|typedef const char* LPCCH;
	|typedef wchar_t WCHAR;
	|typedef WCHAR* LPWSTR;
	|typedef const WCHAR* LPCWSTR;
	|int MultiByteToWideChar(UINT CodePage, DWORD dwFlags, LPCCH lpMultiByteStr, int cbMultiByte, LPWSTR lpWideCharStr, int cchWideChar);
	|int MessageBoxW(HWND hWnd, LPCWSTR lpText, LPCWSTR lpCaption, UINT uType);
	|]]
	|local CP_UTF8 = 65001
	|local k32 = ffi.load(""kernel32"")
	|local u32 = ffi.load(""user32"")
	|local function utf8_to_wide(s)
	|  local n = k32.MultiByteToWideChar(CP_UTF8, 0, s, -1, nil, 0)
	|  assert(n > 0)
	|  local buf = ffi.new(""wchar_t[?]"", n)
	|  assert(k32.MultiByteToWideChar(CP_UTF8, 0, s, -1, buf, n) > 0)
	|  return buf
	|end
	|u32.MessageBoxW(nil, utf8_to_wide(""Привет из Lua!""), utf8_to_wide(""Моё окно""), 0)";
	
	Lua.ВыполнитьКодИзСтроки(Код);
