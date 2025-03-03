#!/usr/local/bin/perl

use Fcntl ':flock'; 

$plane="plane_reserve";
$hotel="hotel_reserve";

open(PLANE,"+<$plane") || die("$$ can't open $plane\n");
open(HOTEL,"+<$hotel") || die("$$ can't open $plane\n");

flock(HOTEL,LOCK_EX);
sleep 1;
&reserve('HOTEL',$hotel);
flock(PLANE,LOCK_EX);
sleep 1;
&reserve('PLANE',$plane);

close(HOTEL);   # and release lock
close(PLANE);   # and release lock

sub reserve {
    my ($fh,$file)=@_;
    if(($value = <$fh>)>0) {
	seek($fh, 0, 0);
	print $fh $value-1,"\n";
	print "$$ $file reserved\n";
	return 1;
    } else {
	die("$$ can't reserve $file\n");
    }
}
