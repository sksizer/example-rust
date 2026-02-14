// #[derive(Debug)]
// enum AppErrorStates {
//     NoConfigFile,
//     InvalidConfigFile,
//     FailedToStart,
// }
//
// #[derive(Debug)]
// enum AppState {
//     New,
//     Initializing,
//     FirstRun,
//     Running,
//     Error(AppErrorStates),
// }
//
//
// #[derive(Debug)]
// enum AppEvent {
//     Start,
//     Stop,
//     Restart,
// }
//
// fn new() -> AppState {
//     AppState::New
// }
//
// fn startApp(_state: AppState::New) -> AppState {
//     AppState::Initializing
// }

mod fsm;

fn main() {
    fsm::typestate::test();
    // println!("Hello, world!");
    // let mut app_state = new();
    // println!("App State: {:?}", app_state);
    //
    // app_state = startApp(app_state);
    // println!("App State: {:?}", app_state);
}
