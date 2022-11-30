$date = [System.TimeZoneInfo]::ConvertTimeBySystemTimeZoneId([DateTime]::Now, "Eastern Standard Time")
$day = $date.Day;

$inputFile = "src/input/{0:d2}.txt" -f $day

# Download input
aoc d --input-only --input-file $inputFile

# Copy skeleton
$rs = "src/days/day{0:d2}.rs" -f $day
$skeleton = Get-Content skeleton.rs
$skeleton = $skeleton -replace 'REPLACEME', $inputFile
$skeleton | Set-Content -Path $rs

# Fix days.rs
$daysRs = Get-Content src/days.rs
$mod = "pub mod day{0:d2};" -f $day
if (-not ($daysRs.Contains($mod))) {
    Add-Content src/days.rs $mod
}

# Fix main.rs
$main = Get-Content src/main.rs
$main = $main -replace 'days::day([0-9]*)::', ("days::day{0:d2}::" -f $day)
$main | Set-Content src/main.rs
