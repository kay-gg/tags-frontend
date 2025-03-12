# Tagging project frontend
This is the GUI for my tagging project. 
![Base app](/img/base.png)


# Features
Allows users to

- See all of their tagged files
- Filter by tags 
- Open file in the default file manager (soon)

# How to use
You'll have to have my [tags-backend](https://github.com/kay-gg/tags-backend) project downloaded, and the executable put in a directory where you can call it from a terminal. 

If you're on linux, you can put it in `/usr/local/bin` 

If you're on windows, you'll have to put it in a directory that is included in your `PATH` system variable, or add a new directory to your `PATH` and then put it in there.

After that, you can just run the frontend. It will be blank until you add tags and files to the tags.

# How to build
Since this project was built with rust, you'll need to have rust and cargo installed. After installing rust, you can do

```
git clone https://github.com/kay-gg/tags-frontend
```

```
cd ./tags-frontend
```

```
cargo run
```