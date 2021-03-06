mod support;
mod async_send_sync;
mod buffered;
mod fs_copy;
mod fs_dir;
//mod fs_file_mocked;
mod fs_file;
mod fs_link;
mod fs;
mod io_async_read;
mod io_chain;
mod io_copy;

mod io_driver_drop;
mod io_driver;
mod io_lines;
mod io_read_exact;
mod io_read_line;
mod io_read;
mod io_read_to_end;
mod io_read_to_string;
mod io_read_until;
mod io_split;
mod io_take;
mod io_write_all;
mod io_write_int;
mod io_write;
mod macros_join;
mod macros_pin;
mod macros_select;
mod macros_try_join;

mod net_bind_resource;
mod net_lookup_host;
mod no_rt;
// mod process_issue_2174;
// mod process_issue_42;
// mod process_kill_on_drop;
// mod process_smoke;

mod _require_full;
mod rt_basic;
mod rt_common;
mod rt_threaded;
mod signal_ctrl_c;
mod signal_drop_recv;
mod signal_drop_rt;
mod signal_drop_signal;
mod signal_multi_rt;
mod signal_no_rt;
mod signal_notify_both;
mod signal_twice;
mod signal_usr1;
mod stream_chain;
mod stream_collect;
mod stream_empty;
mod stream_fuse;
mod stream_iter;
mod stream_merge;
mod stream_once;
mod stream_pending;
mod stream_reader;
mod stream_stream_map;
mod stream_timeout;
mod sync_barrier;
mod sync_broadcast;
mod sync_cancellation_token;
mod sync_errors;
mod sync_mpsc;
mod sync_mutex_owned;
mod sync_mutex;
mod sync_notify;
mod sync_oneshot;
mod sync_rwlock;
mod sync_semaphore_owned;
mod sync_semaphore;
mod sync_watch;
mod task_blocking;
mod task_local;
mod task_local_set;
mod tcp_accept;
mod tcp_connect;
mod tcp_echo;
mod tcp_into_split;
mod tcp_peek;
mod tcp_shutdown;
mod tcp_split;
mod test_clock;
mod time_delay_queue;
mod time_delay;
mod time_interval;
mod time_rt;
mod time_throttle;
mod time_timeout;
mod udp;
//mod uds_cred;
mod uds_datagram;
mod uds_split;
mod uds_stream;

use std::prelude::v1::*;

use crates_unittest::run_inventory_tests;

pub fn run_tests() {
    run_inventory_tests!();
}