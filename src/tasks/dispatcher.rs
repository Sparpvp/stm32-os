// Todo? Should change with a LL instead, but since we don't have enough memory anyways it's pointless
static mut FUNCTION_TABLE: [Option<ProcessIdentifier>; 10] = [None; 10];
static mut TABLE_COUNT: usize = 0;

#[derive(Clone, Copy)]
pub struct ProcessIdentifier {
    pub name: &'static str,
    pub base_address: usize,
}

pub struct ProcessSaver;

// Determine which processes can be added/deleted by name
impl ProcessIdentifier {
    pub fn saver() -> ProcessSaver {
        ProcessSaver
    }

    pub fn retrieve_base_address(name: &'static str) -> Option<usize> {
        unsafe {
            FUNCTION_TABLE
                .iter()
                .find(|o| o.unwrap().name == name)
                .and_then(|x| Some(x.unwrap().base_address))
        }
    }

    pub(in crate::tasks) fn register_user_process(name: &'static str, base_address: usize) {
        unsafe {
            if TABLE_COUNT < FUNCTION_TABLE.len() {
                FUNCTION_TABLE[TABLE_COUNT] = Some(ProcessIdentifier {
                    name: name,
                    base_address: base_address,
                });
                TABLE_COUNT += 1;
            } else {
                panic!("Process list is full!");
            }
        }
    }
}

impl ProcessSaver {
    pub fn add(&self, name: &'static str, base_address: fn()) -> ProcessSaver {
        ProcessIdentifier::register_user_process(name, base_address as usize);
        ProcessSaver
    }
}
