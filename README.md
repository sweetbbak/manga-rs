## MANGA-RS

A very simple CLI tool that is very much a work in progress with the intended use case to parse manga websites 
without relying heavily on html tags which are prone to breaking and do not allow for much diversity of options.
Many web scraping tools require constant maintenance to ensure they are not broken and it can be quite the annoying
endeavor to try and debug a tool that you didn't write and have no idea where along the line things broke.

# Installing
```git clone https://github.com/sweetbbak/manga-rs.git 
 cd manga-rs

 # Returns a list of images in the html
 cargo run <url> --list-images

 # Dumps the raw html to be parsed
 cargo run <url> 

 # Download images (right now is kinda wonky since you have to create the directory first for this to work (?) )
 cargo run <url> --download-images <directory>

 # You could also run to get an executable version that you can put in your path 
 cargo build
```
# Why though?

I find myself writing a stupid amount of scrapers and automators to achieve little tasks like this.
I would like a nice and easy to use tool that can be integrated into shell scripts or the CLI so that
you could potentially use this tool to download any media online easily like manga or novels (maybe even anime)

The problem with existing solutions is that it is slow and feels like patchwork. It often requires complex
sed commands, the use of 'pup' html parser, wget, curl, zip, fzf and a manga reading application. On top of that
its not very reusable or versatile and once the website changes, everything breaks.

# solutions
I will be trying to implement the ability to pass flags to the program that would allow you to pass html tags
that could then be parsed.
ex:
```manga-rs https://website.com --parse-block h3.reader-container>a --download-images
```
This would download and zip all images that are within the specified code block.
this works surprisingly well as there arent usually alot of images on a manga page and the images that we want are
usuall within a specific code-block. At that point it becomes easy to iterate over selected chapter links and download
manga or even novels in text form.

the idea is to allow the user to supply simple patterns to build their own scrapers with ease and reusability.
it would be amazing to implement the use of TOML files that would allow the user to pass those rules that way. 
You could easily provide it with what html blocks you want the program to parse and the action that you want to 
apply to those tags. Throw in some simple logic like page iteration, handling errors, managing directories and files

as well as the ability to parse all links within specific html blocks or iterate over page numbers and download
all images, zip them into a cbz so they can easily be read.

of course that is a LOT and it is currently far from that but I believe that this could be an extremely useful
tool for a lot more than just parsing html to download images. If this has piqued your interest, feel free to 
hit me up with some ideas or send a pull request. Im pretty new to Rust so taking this all on will be alot for me
but I'd like to even implement 30% of this as a way to learn Rust and create something useful.
