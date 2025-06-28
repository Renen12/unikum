# This script can be used instead of the rust wrapper, but that is not recommended, as the rust script provides a better user experience.
[Console]::OutputEncoding = [System.Text.Encoding]::UTF8
$OutputEncoding = [System.Text.Encoding]::UTF8
$a = $args[0];
$b = $args[1];
$c = $args[2];
$d = $args[3];
$e = $args[4];
$session = New-Object Microsoft.PowerShell.Commands.WebRequestSession
$session.Cookies.Add((New-Object System.Net.Cookie("JSESSIONID", $a, "/", "start.unikum.net")))
$session.Cookies.Add((New-Object System.Net.Cookie("UNIHZSESSID", $b, "/", "start.unikum.net")))
$session.Cookies.Add((New-Object System.Net.Cookie("unilocale", "sv", "/", "start.unikum.net")))
$session.Cookies.Add((New-Object System.Net.Cookie($c, $d, "/", "start.unikum.net")))
Invoke-WebRequest -UseBasicParsing -Uri "https://start.unikum.net/unikum/blog/getBlog.ajax?pid=$($e)" `
    -WebSession $session `
    -UserAgent "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:139.0) Gecko/20100101 Firefox/139.0" `
    -Headers @{
    "Accept"          = "application/json"
    "Accept-Language" = "en-GB,en;q=0.5"
    "Accept-Encoding" = "gzip, deflate, br, zstd"
    "Referer"         = "https://start.unikum.net/unikum/viewBlog.html"
    "traceparent"     = "00-291d66d84e763a1df67e9a1e33315882-069c6d01336f7518-01"
    "DNT"             = "1"
    "Sec-GPC"         = "1"
    "Sec-Fetch-Dest"  = "empty"
    "Sec-Fetch-Mode"  = "cors"
    "Sec-Fetch-Site"  = "same-origin"
    "Priority"        = "u=4"
    "Pragma"          = "no-cache"
    "Cache-Control"   = "no-cache"
    "TE"              = "trailers"
} `
    -ContentType "application/json" | Select-Object -Expand Content 