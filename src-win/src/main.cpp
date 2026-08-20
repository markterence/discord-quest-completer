#include <windows.h>
#include <shellapi.h>

#define ID_BTN_GITHUB     1002
#define WM_TRAY_MESSAGE   (WM_USER + 1)
#define ID_TRAY_EXIT      1003
#define ID_TRAY_TOGGLE    1004

typedef BOOL(WINAPI* P_Shell_NotifyIconW)(DWORD, PNOTIFYICONDATAW);
typedef HINSTANCE(WINAPI* P_ShellExecuteW)(HWND, LPCWSTR, LPCWSTR, LPCWSTR, LPCWSTR, INT);
typedef HFONT(WINAPI* P_CreateFontW)(int, int, int, int, int, DWORD, DWORD, DWORD, DWORD, DWORD, DWORD, DWORD, DWORD, LPCWSTR);
typedef BOOL(WINAPI* P_DeleteObject)(HGDIOBJ);
typedef HWND(WINAPI* P_CreateWindowExW)(DWORD, LPCWSTR, LPCWSTR, DWORD, int, int, int, int, HWND, HMENU, HINSTANCE, LPVOID);

P_Shell_NotifyIconW f_Shell_NotifyIconW;
P_ShellExecuteW f_ShellExecuteW;
P_CreateFontW f_CreateFontW;
P_DeleteObject f_DeleteObject;

wchar_t g_szGameName[256] = L"Discord Quest Completer";
const wchar_t* GITHUB_URL = L"https://github.com/markterence/discord-quest-completer";
NOTIFYICONDATAW nid = { 0 };
HFONT hFontTitle = NULL, hFontText = NULL;

wchar_t* FindStringW(const wchar_t* str, const wchar_t* substr) {
    while (*str) {
        const wchar_t* h = str, * n = substr;
        while (*h && *n && *h == *n) { h++; n++; }
        if (!*n) return (wchar_t*)str;
        str++;
    }
    return NULL;
}

void ToggleWindow(HWND hWnd) {
    if (IsWindowVisible(hWnd)) {
        ShowWindow(hWnd, SW_HIDE);
    }
    else {
        ShowWindow(hWnd, SW_SHOW);
        ShowWindow(hWnd, SW_RESTORE);
        SetForegroundWindow(hWnd);
    }
}

LRESULT CALLBACK WndProc(HWND hWnd, UINT message, WPARAM wParam, LPARAM lParam) {
    switch (message) {
    case WM_CREATE: {
        hFontTitle = f_CreateFontW(22, 0, 0, 0, FW_BOLD, 0, 0, 0, DEFAULT_CHARSET, 0, 0, 0, 0, L"Segoe UI");
        hFontText = f_CreateFontW(18, 0, 0, 0, FW_NORMAL, 0, 0, 0, DEFAULT_CHARSET, 0, 0, 0, 0, L"Segoe UI");

        HWND h1 = CreateWindowExW(0, L"STATIC", L"Discord Quest Completer", WS_VISIBLE | WS_CHILD | SS_CENTER, 0, 20, 400, 25, hWnd, NULL, NULL, NULL);
        HWND h2 = CreateWindowExW(0, L"STATIC", g_szGameName, WS_VISIBLE | WS_CHILD | SS_CENTER, 0, 55, 400, 25, hWnd, NULL, NULL, NULL);
        HWND h3 = CreateWindowExW(0, L"STATIC", L"This program is part of the Discord Quest Completer", WS_VISIBLE | WS_CHILD | SS_CENTER, 10, 100, 380, 40, hWnd, NULL, NULL, NULL);
        HWND h4 = CreateWindowExW(0, L"BUTTON", L"View on GitHub", WS_VISIBLE | WS_CHILD | BS_PUSHBUTTON, 125, 150, 150, 35, hWnd, (HMENU)ID_BTN_GITHUB, NULL, NULL);

        SendMessageW(h1, WM_SETFONT, (WPARAM)hFontTitle, TRUE);
        SendMessageW(h2, WM_SETFONT, (WPARAM)hFontText, TRUE);
        SendMessageW(h3, WM_SETFONT, (WPARAM)hFontText, TRUE);
        SendMessageW(h4, WM_SETFONT, (WPARAM)hFontText, TRUE);

        nid.cbSize = sizeof(NOTIFYICONDATAW);
        nid.hWnd = hWnd;
        nid.uID = 1;
        nid.uFlags = NIF_ICON | NIF_MESSAGE | NIF_TIP;
        nid.uCallbackMessage = WM_TRAY_MESSAGE;
        nid.hIcon = LoadIcon(NULL, IDI_APPLICATION);
        lstrcpynW(nid.szTip, g_szGameName, 128);
        f_Shell_NotifyIconW(NIM_ADD, &nid);
    } break;

    case WM_TRAY_MESSAGE:
        if (lParam == WM_RBUTTONUP) {
            POINT cur; GetCursorPos(&cur);
            HMENU hMenu = CreatePopupMenu();
            InsertMenuW(hMenu, 0, MF_BYPOSITION | MF_STRING, ID_TRAY_TOGGLE, IsWindowVisible(hWnd) ? L"Hide" : L"Show");
            InsertMenuW(hMenu, 2, MF_SEPARATOR, 0, NULL);
            InsertMenuW(hMenu, 3, MF_BYPOSITION | MF_STRING, ID_TRAY_EXIT, L"Exit");
            SetForegroundWindow(hWnd);
            TrackPopupMenu(hMenu, TPM_BOTTOMALIGN | TPM_LEFTALIGN, cur.x, cur.y, 0, hWnd, NULL);
            DestroyMenu(hMenu);
        }
        else if (lParam == WM_LBUTTONDBLCLK) ToggleWindow(hWnd);
        break;

    case WM_COMMAND:
        if (LOWORD(wParam) == ID_BTN_GITHUB) f_ShellExecuteW(NULL, L"open", GITHUB_URL, NULL, NULL, SW_SHOWNORMAL);
        if (LOWORD(wParam) == ID_TRAY_EXIT) DestroyWindow(hWnd);
        if (LOWORD(wParam) == ID_TRAY_TOGGLE) ToggleWindow(hWnd);
        break;

    case WM_CTLCOLORSTATIC:
        SetBkMode((HDC)wParam, TRANSPARENT);
        return (LRESULT)GetStockObject(WHITE_BRUSH);

    case WM_DESTROY:
        f_Shell_NotifyIconW(NIM_DELETE, &nid);
        if (hFontTitle) f_DeleteObject(hFontTitle);
        if (hFontText) f_DeleteObject(hFontText);
        PostQuitMessage(0);
        break;

    default: return DefWindowProcW(hWnd, message, wParam, lParam);
    }
    return 0;
}

extern "C" {
    #pragma function(memset)
    void* memset(void* dest, int c, size_t count) {
        char* bytes = (char*)dest;
        while (count--) {
            *bytes++ = (char)c;
        }
        return dest;
    }
}

extern "C" void mainEntryPoint() {
    HINSTANCE hInst = GetModuleHandleW(NULL);

    HMODULE hShell32 = LoadLibraryW(L"shell32.dll");
    HMODULE hGdi32 = LoadLibraryW(L"gdi32.dll");
    HMODULE hUser32 = LoadLibraryW(L"user32.dll");

    f_Shell_NotifyIconW = (P_Shell_NotifyIconW)GetProcAddress(hShell32, "Shell_NotifyIconW");
    f_ShellExecuteW = (P_ShellExecuteW)GetProcAddress(hShell32, "ShellExecuteW");
    f_CreateFontW = (P_CreateFontW)GetProcAddress(hGdi32, "CreateFontW");
    f_DeleteObject = (P_DeleteObject)GetProcAddress(hGdi32, "DeleteObject");
    P_CreateWindowExW f_CreateWindowExW = (P_CreateWindowExW)GetProcAddress(hUser32, "CreateWindowExW");

    LPWSTR lpCmdLine = GetCommandLineW();
    const wchar_t* flag = L"--title";
    wchar_t* pos = FindStringW(lpCmdLine, flag);

    if (pos) {
        pos += 7;
        while (*pos == L' ') pos++;

        if (*pos == L'"') {
            pos++; 
            int i = 0;
            while (*pos != L'"' && *pos != L'\0' && i < 255) {
                g_szGameName[i++] = *pos++;
            }
            g_szGameName[i] = L'\0';
        } else {
            int i = 0;
            while (*pos != L' ' && *pos != L'\0' && i < 255) {
                g_szGameName[i++] = *pos++;
            }
            g_szGameName[i] = L'\0';
        }
    }

    WNDCLASSW wc = { 0 };
    wc.lpfnWndProc = WndProc;
    wc.hInstance = hInst;
    wc.lpszClassName = L"DQCTray";
    wc.hbrBackground = (HBRUSH)(COLOR_WINDOW + 1);
    wc.hCursor = LoadCursor(NULL, IDC_ARROW);
    RegisterClassW(&wc);

    HWND hWnd = f_CreateWindowExW(0, wc.lpszClassName, g_szGameName, WS_OVERLAPPED | WS_CAPTION | WS_SYSMENU | WS_MINIMIZEBOX, CW_USEDEFAULT, CW_USEDEFAULT, 415, 240, NULL, NULL, hInst, NULL);
    ShowWindow(hWnd, SW_SHOWNORMAL);

    MSG msg;
    while (GetMessageW(&msg, NULL, 0, 0)) {
        TranslateMessage(&msg);
        DispatchMessageW(&msg);
    }
    ExitProcess(0);
}