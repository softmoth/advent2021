#! /usr/bin/env perl

use 5.025;
use strict;
use warnings;

$| = 1;

use Data::Dumper qw/Dumper/;
sub note { print STDERR @_, "\n"; }

my $grid = make_grid(4, 7);
$grid = <<EOF;
1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581
EOF
$grid =~ s/\n$//;
$grid = [split(/\n/, $grid)];
map { $_ = [ split '', $_ ] } @$grid;
print_grid($grid);
list_paths($grid, [0, 0], [1, 2]);

exit 0;

sub list_paths {
    my ($grid, $start, $end) = @_;

    my @paths;
    my @search = [$start];

    my $count = 0;
    OUTER: while (@search) {
        die "terminate" if ++$count > 100_000;
        dump_search($grid, \@search);
        print_grid($grid, [map { $_->[-1] } @search]);
        my $point = $search[-1][-1];

        my @neighbors = grep {
                ! this_path_visited(\@search, $_)
            } neighbors($grid, $point);
        say "   + ", show_path($grid, \@neighbors);

        if (grep { points_eq($end, $_) } @neighbors) {
            my @path = map { $_->[-1] } @search;
            push @path, $end;
            say ">>>> ", show_path($grid, \@path);
            print_grid($grid, [map { $_->[-1] } @search], $end);
            push @paths, \@path;
            # Don't search anything else with this prefix
            @neighbors = ();
        }

        if (@neighbors) {
            push @search, \@neighbors;
        } else {
            # Done with $point
            pop @{$search[-1]};
            while (@{$search[-1]} == 0) {
                # Done with this level of neighbors
                pop @search;
                last OUTER if @search == 0;
                pop @{$search[-1]};
            }
        }
    }

    say "Found ", scalar(@paths), " paths in $count iterations";
}

sub this_path_visited {
    my ($search, $point) = @_;

    # Prune points adjacent to previously visited points,
    # since going there from here will always be longer than
    # just going there directly
    for (@{$search}[0..$#$search-1]) {
        return 1 if
            $point->[0] == $_->[-1][0] && abs($point->[1] - $_->[-1][1]) <= 1
            or
            $point->[1] == $_->[-1][1] && abs($point->[0] - $_->[-1][0]) <= 1
            ;
    }

#    for (@$search) {
#        return 1 if points_eq($point, $_->[-1]);
#    }
    0
}

sub points_eq {
    my ($a, $b) = @_;
    $a->[0] == $b->[0] && $a->[1] == $b->[1]
}

sub neighbors {
    my ($grid, $point) = @_;
    my $width = @{$grid->[0]};
    my $height = @$grid;

    my ($x, $y) = @$point;
    my $left = $x > 0 ? -1 : 0;
    my $right = $x < $width - 1 ? 1 : 0;
    my $up = $y > 0 ? -1 : 0;
    my $down = $y < $height - 1 ? 1 : 0;

    my @neighbors;

    if ($up)    { push @neighbors, [$x, $y - 1]; }
    if ($down)  { push @neighbors, [$x, $y + 1]; }
    if ($left)  { push @neighbors, [$x - 1, $y]; }
    if ($right) { push @neighbors, [$x + 1, $y]; }

    @neighbors
}

sub print_grid {
    my ($grid, $path, $end) = @_;
    $path ||= [];
    my %path = map { join(',', @$_) => 1 } @$path;
    for my $y (0..$#$grid) {
        for my $x (0.. $#{$grid->[0]}) {
            my ($on, $off) = $path{"$x,$y"}
                    ? ("\e[33m", "\e[0m")
                    : $end && $end->[0] == $x && $end->[1] == $y
                    ? ("\e[36m", "\e[0m")
                    : ('', '');
            print $on, $grid->[$y][$x], $off;
        }
        print "\n";
    }
}

sub show_path {
    my ($grid, $path) = @_;
    join ' ', map { $grid->[$_->[1]][$_->[0]] } @$path
}

sub dump_search {
    my ($grid, $search) = @_;

    for (@$search) {
        say ": ", show_path($grid, $_);
    }
}

sub make_grid {
    my ($w, $h) = @_;
    my @vals = map { chr } 49..57, 48, 97..122, 65..90, 33..47, 91..96, 123..126;
    my @grid;
    for (my $i = 0; $i < $w * $h; ++$i) {
        push @grid, [] if $i % $w == 0;
        push @{$grid[-1]}, $vals[$i % @vals];
    }
    \@grid;
}
