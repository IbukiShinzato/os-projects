#!/bin/sh
echo 2 > hotel_reserve
echo 2 > plane_reserve 
( perl flock_test.pl kono &  perl flock_test1.pl higa )

