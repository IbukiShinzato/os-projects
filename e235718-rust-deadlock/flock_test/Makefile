CC = gcc
CFLAGS = -g

test:	all
	echo 1 > plane_reserve 
	echo 1 > hotel_reserve 
	( ./flock_test hotel_reserve plane_reserve & ./flock_test hotel_reserve plane_reserve )

test-deadlock:	all
	echo 1 > plane_reserve 
	echo 1 > hotel_reserve 
	( ./flock_test plane_reserve hotel_reserve & ./flock_test hotel_reserve plane_reserve )

pl :	
	echo 1 > plane_reserve 
	echo 1 > hotel_reserve 
	( perl flock_test.pl & perl flock_test.pl )

pl-deadlock :
	echo 1 > plane_reserve 
	echo 1 > hotel_reserve 
	( perl flock_test.pl & perl flock_test1.pl )

all: flock_test 

tar:
	(cd ..; tar cf flock_test.tar --exclude "*.class" --exclude CVS flock_test) 

flock_test : flock_test.c
	$(CC) $(CFLAGS) -o $@ $< 

clean:
	rm flock_test 
