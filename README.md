# Autonomous Self-Piloting Relay Nodes (ASPRN)

## About
This is the project repository for Autonomous Self-Piloting Relay Nodes, a TigerHacks 2022 submission.

The goal of this projec is to enable computer scientists and astronauts alike to communicate in space, with the goal of enabling interplanetary travel.
NASA is currently developing an interplanetary protocol for communications, known as Delay-Tolerant Networking (DTN). Our project aims to implement this protocol, and develop a vast web of DTN capable "routers" that we deploy throughout the solar system. This enhances communication infrastructure in space, and aims to be a factor driving humanity towards space travel.

## Current Goals


|      Goal      | Implemented? |
| -------------- | ------------ |
| Auto DTN Routing |     ✅     |
| Dockerization  |      ✅      |
| Protocol Layer Extensibility | ✅ |
| Server-Response Architecture | ✅ |
| 3D Model | ⚛️ WIP |
| Self-Piloting | ⚛️ WIP |
| Sensor Data Aquisition | ⚛️ WIP |

## How to get started
A couple of the main goals of ASPRN is ease of use and ease of setup, so we built a docker container that automatically sets up a node for you!

Simply start up your RaspberryPi, or any other capable module that you intend to launch in space, download docker and git, and paste the following into your terminal:
```
git clone https://github.com/Dom-1/ASPRN.git
cd ASPRN
```

If you'd like to build the Docker image locally, run
```
docker build -t asprn:v1 .
```

Or if you'd rather use the uploaded cloud image we generated, run
```
docker pull f0rked/asprn:v1
```

Either way, you're now able to build and run your container:
```
$ docker images
REPOSITORY      TAG       IMAGE ID       CREATED         SIZE
f0rked/asprn    v1        91fe3a9ecd7f   2 minutes ago   1.27GB                                                                                                      

$ docker create --name asprn f0rked/asprn:v1
...
$ docker container ls -a
CONTAINER ID   IMAGE             COMMAND          CREATED         STATUS         PORTS     NAMES
3ae81b55af56   f0rked/asprn:v1   "./run.sh"       5 seconds ago   Created                  asprn
$ docker start asprn
asprn
$ docker ps 
CONTAINER ID   IMAGE             COMMAND      CREATED              STATUS        PORTS     NAMES
3ae81b55af56   f0rked/asprn:v1   "./run.sh"   About a minute ago   Up 3 seconds            asprn
```

And your instance is all set! If the node can talk to other DTN-accessible nodes, it will automatically do so.

## The Node's Theoretical Space Suit

![Picture of ship](https://github.com/Dom-1/ASPRN/blob/main/ASPRNModule.webp?raw=true)

## Future Possibilities
 * Add sensors to ASPRN node to collect space data
 * Auto update scripts w/ git hooks
 * Space data aquisition (Google maps, but the solar system (Google Solar))
 * Dispatch reposition coordinates to drones (ground piloting, advanced route planning)
 * Use SpinLaunch platform for individual node launches
 
