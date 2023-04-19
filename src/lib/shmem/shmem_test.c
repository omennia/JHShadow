#include "lib/shmem/buddy.h"
#include "lib/shmem/shmem_allocator.h"
#include "lib/shmem/shmem_file.h"
#include "lib/shmem/shmem_util.h"

#include <limits.h>
#include <math.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <glib.h>
#include <sys/types.h>
#include <sys/wait.h>
#include <unistd.h>

static void buddycontrolblock_testOrder() {
    BuddyControlBlock bcb;
    memset(&bcb, 0, sizeof(BuddyControlBlock));

    g_assert_cmpint(buddycontrolblock_order(&bcb), ==, 0);

    for (unsigned idx = 0; idx < shmem_util_uintPow2k(SHD_BUDDY_ORDER_BITS); ++idx) {
        buddycontrolblock_setOrder(&bcb, idx);
        g_assert_cmpint(buddycontrolblock_order(&bcb), ==, idx);
    }
}

static void buddycontrolblock_testOrderAndNxt() {
    enum { kNTests = 1000 };

    BuddyControlBlock bcb;
    memset(&bcb, 0, sizeof(BuddyControlBlock));

    uint32_t* nxt_values = calloc(kNTests, sizeof(uint32_t));

    for (size_t idx = 0; idx < kNTests; ++idx) {
        nxt_values[idx] = rand() % (SHD_BUDDY_ORDER_MASK + 1);
    }
    nxt_values[0] = 0;
    nxt_values[1] = SHD_BUDDY_ORDER_MASK;

    for (unsigned idx = 0; idx < 32; ++idx) {
        buddycontrolblock_setOrder(&bcb, idx);
        for (unsigned jdx = 0; jdx < kNTests; ++jdx) {

            uint32_t nxt_value = nxt_values[jdx];

            buddycontrolblock_setNxt(&bcb, nxt_value);
            g_assert_cmpint(buddycontrolblock_order(&bcb), ==, idx);
            g_assert_cmpint(buddycontrolblock_nxt(&bcb), ==, nxt_value);
        }
    }

    for (unsigned idx = 0; idx < kNTests; ++idx) {
        uint32_t nxt_value = nxt_values[idx];
        buddycontrolblock_setNxt(&bcb, nxt_value);
        for (unsigned jdx = 0; jdx < 32; ++jdx) {
            buddycontrolblock_setOrder(&bcb, jdx);
            g_assert_cmpint(buddycontrolblock_order(&bcb), ==, jdx);
            g_assert_cmpint(buddycontrolblock_nxt(&bcb), ==, nxt_value);
        }
    }

    free(nxt_values);
}

static void buddycontrolblock_testTagAndPrv() {
    enum { kNTests = 1000 };

    int rc = 0;

    BuddyControlBlock bcb;
    memset(&bcb, 0, sizeof(BuddyControlBlock));

    uint32_t* prv_values = calloc(kNTests, sizeof(uint32_t));

    for (size_t idx = 0; idx < kNTests; ++idx) {
        prv_values[idx] = rand() % ((uint32_t)SHD_BUDDY_TAG_MASK + 1);
    }
    prv_values[0] = 0;
    prv_values[1] = SHD_BUDDY_TAG_MASK;

    for (unsigned idx = 0; idx < 2; ++idx) {
        buddycontrolblock_setTag(&bcb, idx);
        for (unsigned jdx = 0; jdx < kNTests; ++jdx) {

            uint32_t prv_value = prv_values[jdx];

            buddycontrolblock_setPrv(&bcb, prv_value);
            g_assert_cmpint(buddycontrolblock_tag(&bcb), ==, idx);
            g_assert_cmpint(buddycontrolblock_prv(&bcb), ==, prv_value);
        }
    }

    for (unsigned idx = 0; idx < kNTests; ++idx) {
        uint32_t prv_value = prv_values[idx];
        buddycontrolblock_setPrv(&bcb, prv_value);
        for (unsigned jdx = 0; jdx < 2; ++jdx) {
            buddycontrolblock_setTag(&bcb, jdx);
            g_assert_cmpint(buddycontrolblock_tag(&bcb), ==, jdx);
            g_assert_cmpint(buddycontrolblock_prv(&bcb), ==, prv_value);
        }
    }

    free(prv_values);
}

static void buddycontrolblock_testGoodSizes() {
    g_assert_cmpint(buddy_goodPoolSizeNBytes(1), ==, 16);
    g_assert_cmpint(buddy_goodPoolSizeNBytes(33), ==, 64);
    g_assert_cmpint(buddy_goodPoolSizeNBytes(32), ==, 32);
    g_assert_cmpint(buddy_goodPoolSizeNBytes(INT32_MAX), ==, 0);
}

/*
 * tests that, if there is enough room in the pool for an
 * allocation to complete, then we malloc an identical block.
 * as we free things up, we make sure that the contents match.
 */
static void buddy_implTestVsMalloc(size_t pool_nbytes) {

    enum { kNAllocs = 100 };

    struct Alloc {
        size_t nbytes;
        void* bud;
        void* mal;
    };

    struct Alloc* allocs = calloc(kNAllocs, sizeof(struct Alloc));

    unsigned max_alloc = shmem_util_uintLog2(pool_nbytes);
    size_t n = (max_alloc - SHD_BUDDY_PART_MIN_ORDER + 1);

    size_t meta_nbytes = buddy_metaSizeNBytes(pool_nbytes);

    void* pool = calloc(1, pool_nbytes);
    void* meta = calloc(1, meta_nbytes);

    buddy_poolInit(pool, pool_nbytes);
    buddy_metaInit(meta, pool, pool_nbytes);

    for (size_t idx = 0; idx < kNAllocs; ++idx) {
        unsigned alloc_order = SHD_BUDDY_PART_MIN_ORDER + (rand() % n);
        size_t alloc_nbytes = shmem_util_uintPow2k(alloc_order) - sizeof(BuddyControlBlock);

        void* p = buddy_alloc(alloc_nbytes, meta, pool, pool_nbytes);

        if (p) {
            void* q = malloc(alloc_nbytes);
            g_assert_nonnull(q);
            *(uint32_t*)p = rand();
            *(uint32_t*)q = *(uint32_t*)p;

            struct Alloc a = {.nbytes = alloc_nbytes, .bud = p, .mal = q};
            allocs[idx] = a;
        } else {
            struct Alloc a = {.nbytes = 0, .bud = NULL, .mal = NULL};
            allocs[idx] = a;
        }
    }

    for (size_t idx = 0; idx < kNAllocs; ++idx) {
        if (allocs[idx].bud != NULL) {

            uint32_t* p = (uint32_t*)allocs[idx].bud;
            uint32_t* q = (uint32_t*)allocs[idx].mal;
            g_assert(*p == *q);

            buddy_free(allocs[idx].bud, meta, pool, pool_nbytes);
            free(allocs[idx].mal);
        }
    }

    free(pool);
    free(meta);
    free(allocs);
}

static void buddy_testVsMalloc() {

    enum { kNItrs = 1000 };

    buddy_implTestVsMalloc(32);
    buddy_implTestVsMalloc(64);
    buddy_implTestVsMalloc(4096);

    // each test is randomized, try a bunch at the max pool size.
    for (size_t idx = 0; idx < kNItrs; ++idx) {
        buddy_implTestVsMalloc(SHD_BUDDY_POOL_MAX_NBYTES);
    }
}

static void shmemfile_implTestGoodAlloc(size_t requested_nbytes) {

    size_t good_nbytes = shmemfile_goodSizeNBytes(requested_nbytes);

    ShMemFile shmf;
    int rc = shmemfile_alloc(good_nbytes, &shmf);
    g_assert_cmpint(rc, ==, 0);
    rc = shmemfile_free(&shmf);
    g_assert_cmpint(rc, ==, 0);
}

static void shmemfile_testGoodAlloc() {
    shmemfile_implTestGoodAlloc(100);
    shmemfile_implTestGoodAlloc(1000);
    shmemfile_implTestGoodAlloc(5000);
    shmemfile_implTestGoodAlloc(SHD_BUDDY_POOL_MAX_NBYTES);
}

static void shmemutil_testLog2() {
    for (uint32_t idx = 1; idx < 32000; ++idx) {
        uint32_t lhs = log2(idx);
        uint32_t rhs = shmem_util_uintLog2(idx);
        g_assert_cmpint(lhs, ==, rhs);
    }
}

static void shmemutil_testPow2k() {
    g_assert_cmpint(shmem_util_uintPow2k(0), ==, 1);
    g_assert_cmpint(shmem_util_uintPow2k(1), ==, 2);
    g_assert_cmpint(shmem_util_uintPow2k(2), ==, 4);
    g_assert_cmpint(shmem_util_uintPow2k(31), ==, 2147483648);
}

static void shmemallocator_implTestAlloc(ShMemAllocator* allocator, size_t nbytes) {
    ShMemBlock blk = shmemallocator_alloc(allocator, nbytes);
    g_assert_cmpint(blk.nbytes, ==, nbytes);
    g_assert_nonnull(blk.p);

    memset(blk.p, 255, nbytes);

    for (size_t idx = 0; idx < nbytes; ++idx) {
        g_assert_cmpint(((uint8_t*)blk.p)[idx], ==, 255);
    }

    shmemallocator_free(allocator, &blk);
}

enum { kNWarmups = 100 };
typedef ShMemBlock Blocks[2 + kNWarmups];

static ShMemAllocator* shmemallocator_getWarm(ShMemBlock* blks) {
    ShMemAllocator* allocator = shmemallocator_create();
    g_assert_nonnull(allocator);

    // two big allocations
    blks[0] = shmemallocator_alloc(allocator, 104857600);
    blks[1] = shmemallocator_alloc(allocator, 84857600);

    for (size_t idx = 2; idx < 2 + kNWarmups; ++idx) {
        blks[idx] = shmemallocator_alloc(allocator, rand() % 100000);
    }

    return allocator;
}

static void shmemallocator_freeWarm(ShMemAllocator* allocator, ShMemBlock* blks) {

    for (size_t idx = 0; idx < 2 + kNWarmups; ++idx) {
        shmemallocator_free(allocator, &blks[idx]);
    }

    shmemallocator_destroy(allocator);
}

static void shmemallocator_testAlloc() {

    enum { kNumTests = 6 };

    size_t alloc_sizes[kNumTests] = {1, 25, 100, 4096, 100000, 104857600};

    // first on a cold allocator
    for (size_t idx = 0; idx < kNumTests; ++idx) {
        ShMemAllocator* allocator = shmemallocator_create();
        g_assert_nonnull(allocator);
        shmemallocator_implTestAlloc(allocator, alloc_sizes[idx]);
        shmemallocator_destroy(allocator);
    }

    // then on a warm allocator
    ShMemBlock* blks = calloc(1, sizeof(Blocks));
    g_assert_nonnull(blks);
    ShMemAllocator* warm_allocator = shmemallocator_getWarm(blks);

    for (size_t idx = 0; idx < kNumTests; ++idx) {
        shmemallocator_implTestAlloc(warm_allocator, alloc_sizes[idx]);
    }

    shmemallocator_freeWarm(warm_allocator, blks);
    free(blks);
}

static void shmemallocator_implTestSerial(ShMemAllocator* allocator) {
    ShMemBlock x = shmemallocator_alloc(allocator, 1024);
    ShMemBlockSerialized serial = shmemallocator_blockSerialize(allocator, &x);
    ShMemBlock y = shmemallocator_blockDeserialize(allocator, &serial);

    g_assert_cmpmem(&x, sizeof(x), &y, sizeof(y));

    shmemallocator_free(allocator, &x);
}

static void shmemallocator_testSerial() {
    ShMemAllocator* allocator = shmemallocator_create();
    g_assert_nonnull(allocator);
    shmemallocator_implTestSerial(allocator);
    shmemallocator_destroy(allocator);

    ShMemBlock* blks = calloc(1, sizeof(Blocks));
    g_assert_nonnull(blks);
    allocator = shmemallocator_getWarm(blks);
    g_assert_nonnull(allocator);
    shmemallocator_implTestSerial(allocator);
    shmemallocator_freeWarm(allocator, blks);
    free(blks);
}

static ShMemSerializer* shmemserialzer_getWarm(ShMemAllocator* allocator, ShMemBlock* blks) {
    ShMemSerializer* serializer = shmemserializer_create();
    g_assert_nonnull(serializer);

    for (size_t idx = 0; idx < 2 + kNWarmups; ++idx) {
        ShMemBlockSerialized serial = shmemallocator_blockSerialize(allocator, &blks[idx]);

        shmemserializer_blockDeserialize(serializer, &serial);
    }

    return serializer;
}

static void shmemserializer_implTestDeserialize(ShMemAllocator* allocator,
                                                ShMemSerializer* serializer) {

    enum { kTestNBytes = 71 }; // arbitrary
    const char* test_str = "hello world";

    ShMemBlock blk = shmemallocator_alloc(allocator, kTestNBytes);
    strcpy(blk.p, test_str);

    ShMemBlockSerialized serial = shmemallocator_blockSerialize(allocator, &blk);

    ShMemBlock blk_2 = shmemserializer_blockDeserialize(serializer, &serial);

    g_assert_cmpint(blk.nbytes, ==, blk_2.nbytes);
    g_assert_cmpmem(blk.p, blk.nbytes, blk_2.p, blk_2.nbytes);

    shmemallocator_free(allocator, &blk);
}

static void shmemserializer_testDeserialize() {
    ShMemAllocator* allocator = shmemallocator_create();
    g_assert_nonnull(allocator);

    ShMemSerializer* serializer = shmemserializer_create();
    g_assert_nonnull(serializer);

    shmemserializer_implTestDeserialize(allocator, serializer);

    shmemserializer_destroy(serializer);
    shmemallocator_destroy(allocator);

    ShMemBlock* blks = calloc(1, sizeof(Blocks));
    g_assert_nonnull(blks);
    allocator = shmemallocator_getWarm(blks);
    g_assert_nonnull(allocator);
    serializer = shmemserialzer_getWarm(allocator, blks);

    shmemserializer_implTestDeserialize(allocator, serializer);

    shmemserializer_destroy(serializer);
    shmemallocator_freeWarm(allocator, blks);
    free(blks);
}

static void shmemserializer_testSerialize() {
    ShMemAllocator* allocator = shmemallocator_create();
    g_assert_nonnull(allocator);

    ShMemSerializer* serializer = shmemserializer_create();
    g_assert_nonnull(serializer);

    ShMemBlock blk = shmemallocator_alloc(allocator, 1);

    ShMemBlockSerialized serial = shmemallocator_blockSerialize(allocator, &blk);

    ShMemBlock blk_2 = shmemserializer_blockDeserialize(serializer, &serial);

    ShMemBlockSerialized serial_2 = shmemserializer_blockSerialize(serializer, &blk_2);

    g_assert_cmpmem(&serial, sizeof(ShMemBlockSerialized), &serial_2, sizeof(ShMemBlockSerialized));

    shmemallocator_free(allocator, &blk);

    shmemserializer_destroy(serializer);
    shmemallocator_destroy(allocator);
}

static void shmemblockserialized_testString() {
    ShMemBlockSerialized blk = {1, 2, 3, "blk"};

    char buf[SHD_SHMEM_BLOCK_SERIALIZED_MAX_STRLEN];

    shmemblockserialized_toString(&blk, buf);

    bool err = false;

    ShMemBlockSerialized blk_2 = shmemblockserialized_fromString(buf, &err);

    g_assert(!err);

    g_assert_cmpmem(&blk, sizeof(ShMemBlockSerialized), &blk_2, sizeof(ShMemBlockSerialized));

    blk.offset = ULLONG_MAX;
    blk.nbytes = ULLONG_MAX;
    blk.block_nbytes = ULLONG_MAX;
    memset(blk.name, 'x', SHD_SHMEM_FILE_NAME_NBYTES);
    blk.name[SHD_SHMEM_FILE_NAME_NBYTES - 1] = '\0';

    shmemblockserialized_toString(&blk, buf);

    blk_2 = shmemblockserialized_fromString(buf, &err);

    g_assert(!err);

    g_assert_cmpmem(&blk, sizeof(ShMemBlockSerialized), &blk_2, sizeof(ShMemBlockSerialized));
}

static const char* test_fork_key = "TEST_FORK";
static const char* test_fork_val1 = "hello";
static const char* test_fork_val2 = "goodbye";

static void shmemallocator_testForkExec(void* _, const void* user_data) {

    const char* path = user_data;

    ShMemAllocator* allocator = shmemallocator_create();
    g_assert_nonnull(allocator);

    ShMemBlock blk = shmemallocator_alloc(allocator, 32);

    strcpy(blk.p, test_fork_val1);

    ShMemBlockSerialized serial = shmemallocator_blockSerialize(allocator, &blk);

    pid_t pid = fork();

    if (pid == 0) { // child process

        shmemallocator_destroyNoShmDelete(allocator);

        const size_t buf_nbytes = 256;
        char offset_buf[buf_nbytes], nbytes_buf[buf_nbytes], block_nbytes_buf[buf_nbytes];

        snprintf(offset_buf, buf_nbytes, "%zu", serial.offset);
        snprintf(nbytes_buf, buf_nbytes, "%zu", serial.nbytes);
        snprintf(block_nbytes_buf, buf_nbytes, "%zu", serial.block_nbytes);

        int rc = execl(path, path, test_fork_key, offset_buf, nbytes_buf, block_nbytes_buf,
                       serial.name, (char*)NULL);

        if (rc != 0) {
            exit(rc);
        }

    } else { // parent process

        int status = 0;
        waitpid(pid, &status, 0);
        g_assert_cmpint(WIFEXITED(status), !=, 0);
        g_assert_cmpint(WEXITSTATUS(status), ==, 0);

        g_assert_cmpmem(test_fork_val2, strlen(test_fork_val2), blk.p, strlen(blk.p));

        shmemallocator_free(allocator, &blk);
        shmemallocator_destroy(allocator);
    }
}

// after fork + exec, the exec'd process executes this
static void shmemallocator_auxTestForkExec(int argc, char** argv) {
    if (argc != 6) {
        exit(-1);
    }

    ShMemBlockSerialized serial;
    sscanf(argv[2], "%zu", &serial.offset);
    sscanf(argv[3], "%zu", &serial.nbytes);
    sscanf(argv[4], "%zu", &serial.block_nbytes);
    strcpy(serial.name, argv[5]);

    ShMemSerializer* serializer = shmemserializer_create();
    ShMemBlock blk = shmemserializer_blockDeserialize(serializer, &serial);

    int rc = 0;

    if (strcmp(test_fork_val1, blk.p) == 0) {
        strcpy(blk.p, test_fork_val2);
    } else {
        rc = 1;
    }

    shmemserializer_destroy(serializer);

    exit(rc);
}

int main(int argc, char** argv) {

    if (argc > 2) {
        if (strcmp(argv[1], test_fork_key) == 0) {
            shmemallocator_auxTestForkExec(argc, argv);
        }
    }

    g_test_init(&argc, &argv, NULL);
    g_test_set_nonfatal_assertions();

    /* buddy tests */
    g_test_add(
        "/shmem/buddycontrolblock_testOrder", void, NULL, NULL, buddycontrolblock_testOrder, NULL);

    g_test_add("/shmem/buddycontrolblock_testOrderAndNxt", void, NULL, NULL,
               buddycontrolblock_testOrderAndNxt, NULL);

    g_test_add("/shmem/buddycontrolblock_testTagAndPrv", void, NULL, NULL,
               buddycontrolblock_testTagAndPrv, NULL);

    g_test_add("/shmem/buddycontrolblock_testGoodSizes", void, NULL, NULL,
               buddycontrolblock_testGoodSizes, NULL);

    g_test_add("/shmem/buddy_testVsMalloc", void, NULL, NULL, buddy_testVsMalloc, NULL);

    /* shmemfile tests */

    g_test_add("/shmem/shmemfile_testGoodAlloc", void, NULL, NULL, shmemfile_testGoodAlloc, NULL);

    /* shmemutil tests */

    g_test_add("/shmem/shmemutil_testLog2", void, NULL, NULL, shmemutil_testLog2, NULL);

    g_test_add("/shmem/shmemutil_testPow2k", void, NULL, NULL, shmemutil_testPow2k, NULL);

    /* shmemallocator tests */

    g_test_add("/shmem/shmemallocator_testAlloc", void, NULL, NULL, shmemallocator_testAlloc, NULL);

    g_test_add(
        "/shmem/shmemallocator_testSerial", void, NULL, NULL, shmemallocator_testSerial, NULL);

    g_test_add("/shmem/shmemblockserialized_testString", void, NULL, NULL,
               shmemblockserialized_testString, NULL);

    g_test_add("/shmem/shmemserializer_testDeserialize", void, NULL, NULL,
               shmemserializer_testDeserialize, NULL);

    g_test_add("/shmem/shmemserializer_testSerialize", void, NULL, NULL,
               shmemserializer_testSerialize, NULL);

    g_test_add("/shmem/shmemallocator_testForkExec", void, argv[0], NULL,
               shmemallocator_testForkExec, NULL);

    return g_test_run();
}
