# Probot, claim you daily credits
it's a simple code that emulate to claim `probot` daily credits by using headless_chrome crate with **rust language**

# How to use
You can install it from releases tab in github or you can build the project with `cargo`

1. add your tokens inside `probot_tokens.txt`
2. run the `.exe` file

NOTE: this app is still **under development** so it may crashes after you pass an `invalid token` https://github.com/z7pz/probot_claim/issues/4#issue-1449681190

# Requirements to build
1. rust langauge 
2. cargo package manager

# How to build
1. run `python ./NopeCHA/build.py`
2. run `cargo run` or `cargo build`
 
NOTE: `NopeCHA` directory and `probot_tokens.txt` file need to be in the same directory as project or compiled app

# How to get probot token
1. login into `probot dashboard`
2. open `dev-tools`
3. go into `Application bar`
4. go to `localstorage`
5. go to `https://probot.io`
6. you will find `ac` feild just copy the value of it and this is the `probot token`

or 

1. login into `probot dashboard`
2. open `dev-tools`
3. go to console bar
4. and write `localStorage.getItem("ac")` and press enter
