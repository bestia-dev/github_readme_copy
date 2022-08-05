# docs on bestia.dev/docs

It looks like that google search is really bad for github readmes.  
Maybe it will work better as HTML files on my own domain.
For now it looks that SEO is better on non-platform domains.
Maybe it needs some more information for SEO. We will see.
On every HTML there is a link to bestia.dev and a link to the github repository.

## Copy manually

In chrome open the github readme. 
Right-click and Inspect the readme Header.
Then right-click on the "article" element and Copy OuterHTML. 

TODO: write a rust utility for that transformation?

## Paste manually

On the local computer copy the "0_template.html" with the file name of the new HTML.  
Find the "Paste HERE" comment and paste there Ctrl+v. And then Undo Ctrl+z to avoid the HTML code to be shifted. Because then long text are not aligned anymore.
Change the title and description.
Save.