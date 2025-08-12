pub mod bindings;

#[cfg(test)]
mod tests {
    use std::ptr::{null, null_mut};

    use crate::bindings;
    use bindings::*;

    #[test]
    fn generic() {
        unsafe {
            let mut status = amdsmi_status_t_AMDSMI_STATUS_SUCCESS;

            status = amdsmi_init((amdsmi_init_flags_t_AMDSMI_INIT_AMD_GPUS | amdsmi_init_flags_t_AMDSMI_INIT_AMD_GPUS).into());
            if status != amdsmi_status_t_AMDSMI_STATUS_SUCCESS {
                panic!("{}", status)
            }
            
            let mut socket_count = 0;
            
            status = amdsmi_get_socket_handles(&mut socket_count, null_mut());
            if status != amdsmi_status_t_AMDSMI_STATUS_SUCCESS {
                panic!("{}", status)
            }

            assert!(socket_count > 0);

            amdsmi_shut_down();
        }
    }
}
