# Introduction

Simple Tokio based runtime for virtual actors.

#  How runtime works

## Moving parts

1. LocalExecutor - starts thread with tokio::task::LocalSet to host actors on it.
2. LocalSpawner - runs on LocalExecutor thread and spawn actors in LocalSet.
3. LocalActor - spawned by LocalSpawner, runs as local tasks on LocalSet.

