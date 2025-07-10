# Graphical Client


**Frontend architecture**:
- `GUI Macroquad` - observes state of GUI Manager, draw its state, match over GUI Manager state,
- `GUI Macroquad` - sends commands from user input to GUI Manager,
- `GUI Manager` - state machine, enumerated states, abstracting GUI screens/views,
- `GUI Manager` - receives events from user and internal logic, transition states
- `Account Connection` - 

Notes:
- `GUI Macroquad` + `GUI Manager` - one thread
- `Account Connection` - other thread
- Macroquad is based on not Tokio task
- GUI <-> Logic channel probably std
