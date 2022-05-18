STACK_NAME ?= stack-aws-rust-template
NAME ?= aws-rust-template
  
ARCH := aarch64-unknown-linux-gnu
ARCH_SPLIT = $(subst -, ,$(ARCH))
 
setup:
	mkdir build
ifeq (,$(shell which rustc))
	$(error "Could not found Rust compiler, please install it")
endif
ifeq (,$(shell which cargo))
	$(error "Could not found Cargo, please install it")
endif   

bootstrap:
	cp ./target/x86_64-unknown-linux-musl/release/$(NAME) ./build/bootstrap  

b:
	cargo build --release --target x86_64-unknown-linux-musl

#  cargo build --release --target x86_64-unknown-linux-musl
#  cp target/x86_64-unknown-linux-musl/release/aws-rust-template build/bootstrap
#  cd build 
#  zip lambda.zip bootstrap 
#  cd ..

 