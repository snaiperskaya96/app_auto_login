pub mod winapi;

pub mod auto_login {
    use std::{ffi::{c_int, OsString}, path::PathBuf, ptr::null_mut};

    use crate::winapi::{EnumWindows, GetWindowRect, GetWindowThreadProcessId, Rect, SetFocus, SetForegroundWindow, HWND, LPARAM};

    pub struct ImageSearchResult {
        pub window: HWND,
        pub rect: Rect,
        pub match_result: Vec<(u32, u32, f64)>,
    }

    pub fn init() {
        #[cfg(debug_assertions)]
        std::env::set_var("RUST_LOG", "DEBUG");
    
        pretty_env_logger::init();
    }

    pub fn kill_all_processes_by_name(name: &str) {
        log::info!("Killing all the processes called {}", name);
        let mut sys = sysinfo::System::new_all();
    
        loop {
            // Killing a process might terminate sibiling processes as well so we 
            // refresh the processes list and deal with them one at the time.
            sys.refresh_processes(sysinfo::ProcessesToUpdate::All);

            let os_name: OsString = name.to_string().into();
            for process in sys.processes_by_exact_name(os_name.as_os_str()) {
                log::info!("Killing process ID {}", process.pid());
                process.kill();
                break;
            }

            break;
        }
    }


    // this methods iterate through all the processes of a given name, (tries to) focuses them,
    // and checks whether a given picture shows up, until max_time_in_seconds is hit.
    pub fn find_picture_in_process_window(name: &str, img: &PathBuf, timeout: Option<std::time::Duration>) -> Option<ImageSearchResult> {
        let mut sys = sysinfo::System::new_all();

        let mut ignored_hwnd = Vec::new();

        let start_instant = std::time::Instant::now();
    
        loop {
            if let Some(max_time) = timeout {
                if start_instant.elapsed() >= max_time {
                    return None;
                }
            }

            sys.refresh_processes(sysinfo::ProcessesToUpdate::All);

            let os_name: OsString = name.to_string().into();
            for process in sys.processes_by_exact_name(os_name.as_os_str()) {
                struct EnumData {
                    target_pid: u32,
                    hwnd: HWND,
                }

                extern "system" fn enum_windows_proc(hwnd: HWND, lparam: LPARAM) -> c_int {
                    let data = unsafe { &mut *(lparam as *mut EnumData) };
                    let mut process_id: u32 = 0;
                    let thread_id = unsafe { GetWindowThreadProcessId(hwnd, &mut process_id) };

                    if thread_id != 0 && process_id == data.target_pid {
                        data.hwnd = hwnd;
                        return 0;
                    }
                    return 1;
                }
                
                let mut data = EnumData { target_pid: process.pid().as_u32(), hwnd: null_mut() };

                unsafe { EnumWindows(enum_windows_proc, (&mut data as *mut EnumData) as LPARAM ) };
            
                if !data.hwnd.is_null() && !ignored_hwnd.contains(&data.hwnd) {
                    unsafe { 
                        let foreground = SetForegroundWindow(data.hwnd);
                        let focus = SetFocus(data.hwnd);

                        log::info!("Found HWND for process {} ({}). SetForegroundWindow = {}, SetFocus = {:?}", 
                            name, 
                            process.pid().as_u32(),
                            foreground,
                            focus
                        );

                        let mut rect = Rect::default();
                        GetWindowRect(data.hwnd, &mut rect);
                        let region = (rect.left as _, rect.top as _, (rect.right - rect.left) as _, (rect.bottom - rect.top) as _);

                        if rect.coords_is_zero()
                        {
                            ignored_hwnd.push(data.hwnd);
                            continue;
                        }

                        log::info!("{:?}", region);

                        if foreground != 0 {
                            log::info!("{:?}", img);
                            let mut gui = rustautogui::RustAutoGui::new(false);
                            gui.load_and_prepare_template(img.to_str().unwrap(), Some(region), rustautogui::MatchMode::FFT, &None);
                            match gui.find_image_on_screen(0.8) {
                                Some(r) =>  { 
                                    log::info!("Image found!"); 
                                    return Some(ImageSearchResult { window: data.hwnd, rect: rect, match_result: r });
                                },
                                None => {},
                            }

                        }

                    }
                }
            }
        }
    }
}
