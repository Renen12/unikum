# Server documentation
The server is a basic wrapper around the Unikum api.\
The auth token("bearer") can be obtained from the Authorization header supplied in most requests to the unikum internals, can be found by inspecting the requests.\
JSESSIONID, UNIHZID, and the shibsession pairs can be obtained by looking at the cookies of the current session.\
The pid is in the URL of the blog page.\
It can return blogs("lärloggar") from https://start.unikum.net/unikum/blog/getBlog.ajax.\
To get the blogs of a subject, make a request to\
[SERVER_URL]:7951/?jsess=[JSESSION_ID]&uni=[UNIHZ_ID]&shibn=[SHIBSESSION_NAME]&shibv=[SHIBSESSION_VALUE]&pid=[SUBJECT_BLOG_PID]\
To get the messages of a Unikum user, make a request to\
[SERVER_URL]:7951/?messages=true&userpid=[USER_PID_FROM_HOMEPAGE]&bearer=[AUTH_TOKEN_BEARER]
