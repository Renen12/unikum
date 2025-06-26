// Experimental JS implementation
let pid = "";
let unihzsessid = "";
let jsessionid = "";
let shibsessionName = "";
let shibsessionValue = "";
const url = `https://start.unikum.net/unikum/blog/getBlog.ajax?pid=${pid}`;
let x = await fetch(url, {
    method: "GET",
    headers: {
        "Accept": "application/json",
        "Accept-Language": "en-GB,en;q=0.5",
        "Accept-Encoding": "gzip, deflate, br, zstd",
        "Referer": "https://start.unikum.net/unikum/viewBlog.html",
        "traceparent": "00-291d66d84e763a1df67e9a1e33315882-069c6d01336f7518-01",
        "DNT": "1",
        "Sec-GPC": "1",
        "Sec-Fetch-Dest": "empty",
        "Sec-Fetch-Mode": "cors",
        "Sec-Fetch-Site": "same-origin",
        "Priority": "u=4",
        "Pragma": "no-cache",
        "Cache-Control": "no-cache",
        "TE": "trailers",
        "Content-Type": "application/json",
        "User-Agent": "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:139.0) Gecko/20100101 Firefox/139.0",
        "Cookie": `JSESSIONID = "${jsessionid}"; UNIHZSESSID = "${unihzsessid}"; unilocale = sv; ${shibsessionName} = ${shibsessionValue}`
    },
    credentials: "include"
});
console.log(await x.text())