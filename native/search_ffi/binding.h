#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

int64_t add(int64_t a, int64_t b);

int32_t error_message_utf8(char *buf, int32_t length);

int32_t last_error_length(void);

int32_t se_delete_by_str(int64_t port, const char *field, const char *value);

int32_t se_delete_by_u64(int64_t port, const char *field, uint64_t value);

int32_t se_exists(int64_t port);

int32_t se_get_by_i64(int64_t port, const char *field, int64_t value);

int32_t se_get_by_u64(int64_t port, const char *field, uint64_t value);

int32_t se_index(int64_t port, const char *doc);

int32_t se_open_or_create(const char *path, const char *schema);

int32_t se_search(int64_t port,
                  const char *query,
                  const char *fields,
                  uint32_t page_start,
                  uint32_t page_size);

int64_t se_update_by_str(int64_t port, const char *field, const char *value, const char *doc);

int32_t se_update_by_u64(int64_t port, const char *field, uint64_t value, const char *doc);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus
