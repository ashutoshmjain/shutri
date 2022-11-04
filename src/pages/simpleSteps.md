- If you want to publish logseq to the web (like this website), the easiest and zero cost way is to publish it to git hub pages. If you don't know what github pages are , feel free to check out [[githubPages]]
- Here are simple steps to publish logseq to gh pages
	- Create a branch gh-pages in the github repo where you want to publish your graph. This may be a new repo (recommended) or an existing one, assuming you are not previously publishing anything.
	- Now , on your local machine, cd into the cloned repo and use following commands.
	- ```
	  git worktree add /tmp/graph/gh-pages
	  Export your logseq graph to some place on your machine - say ~/Documents/myGraph/
	  rm -rf /tmp/graph/* # this won't delete the .git directory
	  cp -rp ~/Documents/myGraph/*  /tmp/graph/
	  cd /tmp/graph
	  git add -A
	  git commit -m 'say deploy new graph'
	  git push
	  ```
	- If you want to publish frequently,  you may use some other folder than /tmp to avoid first step.
	- #+BEGIN_CAUTION
	  You will need to configure github repo that you are using for web publishing, based on where you want to publish eg domain name etc. 
	  #+END_CAUTION