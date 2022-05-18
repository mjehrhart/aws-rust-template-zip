# AWS Lambda & Rust Deployment Method

# Prerequisite
This guide is written for macOS users. Make sure you have these following items installed and configured if needed. Installing these items takes only a few minutes.

# Setup
Open your mac terminal to your projects home directory
 
- ```mkdir .cargo```
- ```touch .cargo/config.toml```
- Make any changes to Makefile
- Make any changes to template.yaml  

Once all this is done, go ahead and do your coding stuff.  As of the time of writing this, various rust crates do not work in aws lambda or at least I haven't been able to get them to work.  So be careful and take your time testing crates.  For me, it was a lot of trial and error.  

# Commands

- Add .cargo/config.toml to project root directory. This needs to be added only once.

- Add the following to the file. This will allow for cross compilation so this can run on AWS custom runtime (amazon linux 2)

- ```bash
[target.x86_64-unknown-linux-musl]
linker = "rust-lld"
```
 
# Build
- ```cargo build --release --target x86_64-unknown-linux-musl```
- ~~~```rust-musl-builder cargo build --release```~~~ (takes a few minutes)

- ```cp target/x86_64-unknown-linux-musl/release/{APP_NAME} build/bootstrap```
- ``` zip lambda.zip bootstrap```

# Helpful Links
- https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/building-custom-runtimes.html

- https://github.com/aws-samples/serverless-rust-demo

- https://github.com/awslabs/aws-lambda-rust-runtime/blob/main/lambda-http/examples/hello-http.rs

# Notes

# Screenshots  

#### Makefile


#### template.yaml
Note the CodeUri points to the build/ directory.  This is where the bootstrap file be put.  So the CodeUri points to the location of the bootstrap for that function.  

<img width="50%" alt="images/Screen Shot 2022-05-10 at 10.00.09 AM_NDCdNwvovL0Tjp0Y.png" src="https://raw.githubusercontent.com/mjehrhart/assets/main/images/Screen Shot 2022-05-10 at 10.00.09 AM_NDCdNwvovL0Tjp0Y.png">  

  

	  
Take a look at @build:  
 	cargo lambda build --release --target $(ARCH)
