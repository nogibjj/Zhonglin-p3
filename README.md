# Zhonglin Project2 Microservice Rust

## Key Objectives of Project
In project 2, the main purpose is to build a functional Web Microservice using Rust based on Kubernetes and deploy and host it on AWS.
I create a simple actix Microservice for top 10 US universities recommendation

This actix Microservice has multiple routes:

A. type: `/` that returns a message : "This is a random best university of the US generator!"

B. type: `/university` that returns a random university in the list of the US top 10 best universities

C. type: `/version` that returns the version of the package 

## 1. `cargo run`
<img width="675" alt="image" src="https://user-images.githubusercontent.com/112585430/221992977-b85337d9-3a34-42ab-b561-2bb50bbe581f.png">
Initiating message:  

<img width="683" alt="image" src="https://user-images.githubusercontent.com/112585430/221993156-9243a1c6-fc8f-49f4-affc-ff5439f5a48c.png">
<img width="715" alt="image" src="https://user-images.githubusercontent.com/112585430/221993315-e8633cc8-0f87-45af-90f3-a34b2c253b9b.png">
<img width="715" alt="image" src="https://user-images.githubusercontent.com/112585430/221994784-b69fc902-6ec2-4ce2-8220-c7b3228325cc.png">

## 2. clone repo on aws clound9, run the app on AWS APP runner.
`git clone https://github.com/nogibjj/Zhonglin-p2.git`
<img width="590" alt="image" src="https://user-images.githubusercontent.com/112585430/222000683-017503b2-e6c2-4284-89ee-0bffa0b71f86.png">

reinstall rust on cloud9 environment.  

<img width="630" alt="image" src="https://user-images.githubusercontent.com/112585430/222000347-129d8bc9-e886-4245-9b53-e008c2d63256.png">

build docker.

<img width="590" alt="image" src="https://user-images.githubusercontent.com/112585430/222003691-fda2657a-8508-4440-9803-7c0ee0822aad.png">. 

push the image to the repository. 

<img width="871" alt="image" src="https://user-images.githubusercontent.com/112585430/222004342-b582194d-e7be-4932-bfb5-06e04f73ed7d.png">

Go to AWS APP runner

deploy the app from the image 

<img width="1574" alt="image" src="https://user-images.githubusercontent.com/112585430/222012910-c46f48fa-4096-46fb-a7d8-f2c35962e494.png">
