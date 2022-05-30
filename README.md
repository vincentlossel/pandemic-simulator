# Pandemic Simulator
Simulate the spread of a disease amongst living entities. 

*This project was started as a way to learn more about Rust.*

## Installation
```
git clone https://github.com/vincentlossel/pandemic-simulator.git
cd pandemic-simulator
cargo run
```

## Dependencies
- [raylib-rs](https://crates.io/crates/raylib)
- [rand](https://crates.io/crates/rand)
- [chrono](https://crates.io/crates/chrono)

## TODOs
- [ ] Optimize the rendering loops
- [ ] Optimize the infection checks
- [ ] Allow for recovered humans to get infected again
- [ ] Add date and time in the simulation
    - [ ] Special events based on date/time (e.g. social activities in the week-end, work days, holidays...)
- [ ] Export stats at the end of the simulation (e.g. Time series)
- [ ] New features
    - [ ] More parameters
    - [ ] Social distancing
    - [ ] Quarantine zones
    - [ ] Lockdown
    - [ ] Weather
    - [ ] Virus mutations
    - [ ] Places: Households / Work / Market
    - [ ] Individual characteristics (e.g. age, weight, comorbidities...)
    - [ ] Cross-species transmission
- [ ] Scenarios

## Licence
This project is published under the MIT Licence. 
