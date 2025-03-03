#!/bin/sh
echo 1 > hotel_reserve
echo 1 > plane_reserve 
( perl flock_test.pl kono &  perl flock_test.pl higa )

