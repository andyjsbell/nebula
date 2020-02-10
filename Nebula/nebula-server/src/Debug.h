#pragma once
#define DEBUG_PRINT(type, fmt, ...) do {fprintf(stderr, type " %20s:%-4u | %-30s | " fmt "\n", __FILE__, __LINE__, __FUNCTION__, ##__VA_ARGS__);} while (0)
#define CHECK_HR(x) { HRESULT __hr__ = (x); if (FAILED(__hr__)) { printf("Operation Failed (%08x) %s:%d\n", __hr__, __FILE__, __LINE__); return false;} }
#define CHECK_HR_NULL(x) { HRESULT __hr__ = (x); if (FAILED(__hr__)) { printf("Operation Failed (%08x) %s:%d\n", __hr__, __FILE__, __LINE__); return NULL;} }

#define DEBUG_INFO(fmt, ...) DEBUG_PRINT("[I]", fmt, ##__VA_ARGS__)
#define DEBUG_WARN(fmt, ...) DEBUG_PRINT("[W]", fmt, ##__VA_ARGS__)
#define DEBUG_ERROR(fmt, ...) DEBUG_PRINT("[E]", fmt, ##__VA_ARGS__)
#define DEBUG_FIXME(fmt, ...) DEBUG_PRINT("[F]", fmt, ##__VA_ARGS__)