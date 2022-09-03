#! /usr/bin/perl
use 5.025;

my %names;
my @edges;
for (<DATA>) {
    chomp;
    my ($a, $b) = split('-');
    ++$names{$a};
    ++$names{$b};
    push @edges, [$a, $b], [$b, $a];
}

#note 0+%names;
#foreach my $k (keys %names) { note "$k: $names{$k}" }

my @solutions;

sub walk {
    my ($cave, $path) = @_;

    $path = [@$path, $cave];

    die "RECURSION ERROR [@$path]" if @$path > 100;

    if ($cave eq "end") {
        # If we see end, we're done; no double-visits
        push @solutions, $path;
        return;
    }

    if ($cave !~ /^[A-Z]/) {
        # PART 1
        # Little room make sure we haven't been there before
        #foreach my $room (@$path) {
        #    if ($room eq $cave) {
        #        return;
        #    }
        #}

        # PART 2
        if ($cave eq "start" && @$path > 1) {
            # No double-visits to start room
            return;
        }

        # Filter out paths that visit too many small rooms
        my %small;
        my $special;
        for (grep { !/^[A-Z]/ } @$path) {
            ++$small{$_};
            if ($small{$_} > 2) {
                #note "# >2  @$path ...";
                return;
            } elsif ($small{$_} == 2) {
                #note "# ==  @$path ...";
                if ($special) {
                    return;
                }

                $special = $_;
            }
        }
    }

    walk($_, $path) for map { $_->[1] } grep { $_->[0] eq $cave } @edges;
}

walk("start", []);

#sort @solutions;
#say "@$_" for sort @solutions;

say scalar(@solutions);

exit 0;

__DATA__
XW-ed
cc-tk
eq-ed
ns-eq
cc-ed
LA-kl
II-tk
LA-end
end-II
SQ-kl
cc-kl
XW-eq
ed-LA
XW-tk
cc-II
tk-LA
eq-II
SQ-start
LA-start
XW-end
ed-tk
eq-JR
start-kl
ed-II
SQ-tk
