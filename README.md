# Unikum API/tools
A script to return lärloggar(blogs) from Unikum\
--help — display this message\
--shibsession_name=[your shibsession cookie's name]\
--shibsession_value=[your shibsession cookie's value]\
--jsessionid=[your jsessionid]\
 --unihzsessid=[your unihzsessid]\
  --pid=[the pid of the blog page, e.g 11349120275]\
   --html — output these into respective HTML files\
   --dry — show raw data\
  Instructions on how to obtain these is in the README.md file. This must be run in a directory with the proper return_json_posts.ps1 file.\
  Such a file is provided in this repository.\
Get these values from cookies->storage in your browser on start.unikum.net
Can act as a server with --server to serve content from unikum. Look at the source code for further explanation.