- github is a free service to host your git repos. It also offers a free service that allows you to publish a static website to the web. They call this service - github pages.
- The concept is simple
	- Let us say you have some sort of web application in github repo (or elsewhere). Let us say, this web application for example [[mdbook]] or [[logseq]] , generate a bunch of html files and an index file that may be served to the web as a static website.
	- You can push this entire generated content to a branch of the repo say gh-pages.  (or a name of your choice). You will obviously need to create that branch. Even gh-pages is NOT automatically created.
	- #+BEGIN_TIP
	  Keep in mind - on this new branch (gh-pages or whatever name you chose), you don't wanna put your application. You only push the generated static content to this branch.
	  #+END_TIP
	- github has a jekyll based webserver, that takes your content (from the branch you specify) and serves it to the web
	- The normal url is  userName.github.io/repoName ; but you can configure it to point to your own custom domain or subdomain. If you use a custom domain , you will obviously need to pay for that domain name (to say Go Daddy or whoever you bought your domain name) but nothing goes to github.
	- The sad part is github pages still don't support [[.eth]] domains ..wish they did ..
-