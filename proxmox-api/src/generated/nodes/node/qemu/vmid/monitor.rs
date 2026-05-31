#[derive(Debug, Clone)]
pub struct MonitorClient<T> {
    client: T,
    path: String,
}
impl<T> MonitorClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/monitor"),
        }
    }
}
impl<T> MonitorClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Execute QEMU monitor commands."]
    #[doc = ""]
    #[doc = "Permission check: perm(\"/vms/{vmid}\", [\"Sys.Audit\", \"Sys.Modify\"], any)"]
    #[doc = "The following commands do not require any additional privilege: ?, help, info\n\nThe following commands require 'Sys.Modify': announce_self, backup_cancel, balloon, block_job_cancel, block_job_complete, block_job_pause, block_job_resume, block_job_set_speed, block_resize, block_set_io_throttle, boot_set, c, calc_dirty_rate, cancel_vcpu_dirty_limit, chardev-send-break, closefd, commit, cont, cpu, delvm, eject, exit_preconfig, expire_password, getfd, gpa2hpa, gpa2hva, gva2gpa, i, loadvm, log, migrate_cancel, migrate_continue, migrate_pause, migrate_set_capability, migrate_set_parameter, migrate_start_postcopy, mouse_button, mouse_move, mouse_set, one-insn-per-tb, p, print, q, qemu-io, qom-get, qom-list, quit, replay_break, replay_delete_break, replay_seek, ringbuf_read, ringbuf_write, s, savevm, sendkey, set_link, set_password, set_vcpu_dirty_limit, snapshot_blkdev_internal, snapshot_delete_blkdev_internal, stop, stopcapture, sum, sync-profile, system_powerdown, system_reset, system_wakeup, trace-event, x, x_colo_lost_heartbeat, xp\n\nThe following commands are root-only: backup, block_stream, change, chardev-add, chardev-change, chardev-remove, client_migrate_info, device_add, device_del, drive_add, drive_backup, drive_del, drive_mirror, dump-guest-memory, dumpdtb, gdbserver, hostfwd_add, hostfwd_remove, logfile, mce, memsave, migrate, migrate_incoming, migrate_recover, nbd_server_add, nbd_server_remove, nbd_server_start, nbd_server_stop, netdev_add, netdev_del, nmi, o, object_add, object_del, pcie_aer_inject_error, pmemsave, qom-set, savevm-end, savevm-start, screendump, snapshot_blkdev, watchdog_action, wavcapture, xen-event-inject, xen-event-list\n"]
    pub async fn post(&self, params: PostParams) -> Result<String, T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &params).await
    }
}
impl PostParams {
    pub fn new(command: String) -> Self {
        Self {
            command,
            additional_properties: ::std::default::Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PostParams {
    #[doc = "The monitor command."]
    #[doc = ""]
    pub command: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
