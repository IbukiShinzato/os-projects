#include <stdio.h>
#include <fcntl.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sys/file.h>
#include <errno.h>

#define TRUE 1
#define FALSE 0

// extern int errno;

char *plane;
char *hotel;

void
usage(char *myself)
{
    fprintf(stderr,"Usage: %s -f file",myself);
}

int reserve(FILE *fp);

int
main(int ac,char *av[])
{
    int plane_fd,hotel_fd,result;
    pid_t pid;
    FILE *plane_fp,*hotel_fp;
    hotel = av[1];
    plane = av[2];

    pid = getpid();
    if ((plane_fp = fopen(plane, "r+")) == NULL) {
	(void)fprintf(stderr, "%d can't open %s %s\n", pid, plane, strerror(errno));
	 exit(1);
    }
    plane_fd = fileno(plane_fp);
    if ((hotel_fp = fopen(hotel, "r+") )== NULL ) {
	(void)fprintf(stderr, "%d can't open %s %s\n", pid, hotel, strerror(errno));
	 exit(1);
    }
    hotel_fd = fileno(hotel_fp);
/*
//           LOCK_SH  shared file lock 
//           LOCK_EX  exclusive file lock 
//           LOCK_NB  don't block when locking 
//           LOCK_UN  unlock file 
*/

    if( flock(plane_fd, LOCK_EX) != 0) {
	fprintf(stderr,"%d flock error: %s\n",pid, strerror(errno));
	exit(1);
    }
    result = reserve(plane_fp);
    // sleep(2);
    if(result) {
	fprintf(stderr,"%d reserve %s\n",pid,plane);
    } else {
	fprintf(stderr,"%d cannot reserve %s\n",pid,plane);
	exit(1);
    }
    if( flock(hotel_fd, LOCK_EX) != 0) {
	fprintf(stderr,"%d flock error: %s\n",pid, strerror(errno));
	exit(1);
    }

    result = reserve(hotel_fp);
    if(result) {
	fprintf(stderr,"%d reserve %s\n",pid,hotel);
    } else {
	fprintf(stderr,"%d cannot reserve %s\n",pid,hotel);
	exit(1);
    }
    fclose(plane_fp);   /* and release lock */
    fclose(hotel_fp);   /* and release lock */
    exit(0);
}


int 
reserve(FILE *fp)
{
    int remain;
    char buf[BUFSIZ];
    if(fgets(buf,BUFSIZ,fp)) {
	remain = atoi(buf);
	if(remain>0) {
	    fseek(fp, 0L, SEEK_SET);
	    fprintf(fp,"%d\n",--remain);
	    return TRUE;
	} 
    }
    return FALSE;
}
