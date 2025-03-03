#include <stdio.h>
#include <sys/mman.h>
#include <sys/types.h>
#include <fcntl.h>
#include <sys/stat.h>
#include <stdlib.h>
#include <unistd.h>
#include <string.h>

void mmap_copy(const char *source_file, const char *target_file)
{
    int from = -1;
    int to = -1;
    caddr_t from_mmap, to_mmap;
    off_t size;
    struct stat sb;

    if ((from = open(source_file, O_RDONLY, 0666)) == 0) {
        fprintf(stderr, "can't open %s\n", source_file);
        perror(NULL);
        return;
    }

    if (fstat(from, &sb)) {
        fprintf(stderr, "can't fstat %s\n", source_file);
        return;
    }

    size = sb.st_size;
    from_mmap = mmap(NULL, size, PROT_READ, MAP_PRIVATE, from, (off_t)0);
    if (from_mmap == (caddr_t)-1) {
        fprintf(stderr, "Can't mmap from\n");
        perror(NULL);
        return;
    }

    if ((to = open(target_file, O_CREAT | O_RDWR, 0666)) == 0) {
        fprintf(stderr, "can't open %s\n", target_file);
        perror(NULL);
        return;
    }

    if (size == 0) {
        return;
    }

    lseek(to, size - sizeof(int), 0L);
    int i = 0;
    write(to, &i, sizeof(int));

    to_mmap = mmap(NULL, size, PROT_WRITE | PROT_READ, MAP_SHARED, to, (off_t)0);
    if (to_mmap == (caddr_t)-1) {
        fprintf(stderr, "Can't mmap to\n");
        perror(NULL);
        return;
    }

    memcpy(to_mmap, from_mmap, size);

    if (msync(to_mmap, size, MS_SYNC)) {
        fprintf(stderr, "msync failure\n");
        return;
    }

    ftruncate(to, size);
}
