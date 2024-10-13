use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct Report {
    pub(crate) print: Print,
}

#[derive(Serialize, Deserialize)]
pub struct Print {
    pub(crate) ams: Ams,
    pub(crate) ams_rfid_status: i64,
    pub(crate) ams_status: i64,
    pub(crate) aux_part_fan: bool,
    pub(crate) bed_target_temper: f64,
    pub(crate) bed_temper: f64,
    pub(crate) big_fan1_speed: String,
    pub(crate) big_fan2_speed: String,
    pub(crate) chamber_temper: f64,
    pub(crate) command: String,
    pub(crate) cooling_fan_speed: String,
    pub(crate) fail_reason: String,
    pub(crate) fan_gear: i64,
    pub(crate) filam_bak: Vec<Option<serde_json::Value>>,
    pub(crate) force_upgrade: bool,
    pub(crate) gcode_file: String,
    pub(crate) gcode_file_prepare_percent: String,
    pub(crate) gcode_start_time: Option<String>,
    pub(crate) gcode_state: String,
    pub(crate) heatbreak_fan_speed: String,
    pub(crate) hms: Vec<Option<serde_json::Value>>,
    pub(crate) home_flag: i64,
    pub(crate) hw_switch_state: i64,
    pub(crate) ipcam: Ipcam,
    pub(crate) layer_num: i64,
    pub(crate) lifecycle: Option<String>,
    pub(crate) lights_report: Vec<LightsReport>,
    pub(crate) maintain: i64,
    pub(crate) mc_percent: i64,
    pub(crate) mc_print_error_code: String,
    pub(crate) mc_print_stage: String,
    pub(crate) mc_print_sub_stage: i64,
    pub(crate) mc_remaining_time: i64,
    pub(crate) mess_production_state: Option<String>,
    pub(crate) nozzle_diameter: String,
    pub(crate) nozzle_target_temper: f64,
    pub(crate) nozzle_temper: f64,
    pub(crate) online: Online,
    pub(crate) print_error: i64,
    pub(crate) print_gcode_action: i64,
    pub(crate) print_real_action: i64,
    pub(crate) print_type: String,
    pub(crate) profile_id: String,
    pub(crate) project_id: String,
    pub(crate) queue_number: i64,
    pub(crate) sdcard: bool,
    pub(crate) sequence_id: String,
    pub(crate) spd_lvl: i64,
    pub(crate) spd_mag: i64,
    pub(crate) stg: Vec<Option<serde_json::Value>>,
    pub(crate) stg_cur: i64,
    pub(crate) subtask_id: String,
    pub(crate) subtask_name: String,
    pub(crate) task_id: String,
    pub(crate) total_layer_num: i64,
    pub(crate) upgrade_state: UpgradeState,
    pub(crate) upload: Upload,
    pub(crate) vt_tray: Tray,
    pub(crate) wifi_signal: String,
    pub(crate) xcam: Xcam,
    pub(crate) xcam_status: String,
}

#[derive(Serialize, Deserialize)]
pub struct Ams {
    pub(crate) ams: Vec<Am>,
    pub(crate) ams_exist_bits: String,
    pub(crate) insert_flag: bool,
    pub(crate) power_on_flag: bool,
    pub(crate) tray_exist_bits: String,
    pub(crate) tray_is_bbl_bits: String,
    pub(crate) tray_now: String,
    pub(crate) tray_pre: String,
    pub(crate) tray_read_done_bits: String,
    pub(crate) tray_reading_bits: String,
    pub(crate) tray_tar: String,
    pub(crate) version: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Am {
    pub(crate) humidity: String,
    pub(crate) id: String,
    pub(crate) temp: String,
    pub(crate) tray: Vec<Tray>,
}

#[derive(Serialize, Deserialize)]
pub struct Tray {
    pub(crate) id: String,
    pub(crate) bed_temp: Option<String>,
    pub(crate) bed_temp_type: Option<String>,
    pub(crate) cols: Option<Vec<String>>,
    pub(crate) drying_temp: Option<String>,
    pub(crate) drying_time: Option<String>,
    pub(crate) nozzle_temp_max: Option<String>,
    pub(crate) nozzle_temp_min: Option<String>,
    pub(crate) remain: Option<i64>,
    pub(crate) tag_uid: Option<String>,
    pub(crate) tray_color: Option<String>,
    pub(crate) tray_diameter: Option<String>, //f64>,
    pub(crate) tray_id_name: Option<String>,
    pub(crate) tray_info_idx: Option<String>,
    pub(crate) tray_sub_brands: Option<String>,
    pub(crate) tray_type: Option<String>,
    pub(crate) tray_uuid: Option<String>,
    pub(crate) tray_weight: Option<String>,
    pub(crate) xcam_info: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Ipcam {
    pub(crate) ipcam_dev: String,
    pub(crate) ipcam_record: String,
    pub(crate) resolution: String,
    pub(crate) timelapse: String,
}

#[derive(Serialize, Deserialize)]
pub struct LightsReport {
    pub(crate) mode: String,
    pub(crate) node: String,
}

#[derive(Serialize, Deserialize)]
pub struct Online {
    pub(crate) ahb: bool,
    pub(crate) rfid: Option<bool>,
    pub(crate) version: i64,
}

#[derive(Serialize, Deserialize)]
pub struct UpgradeState {
    pub(crate) ahb_new_version_number: String,
    pub(crate) ams_new_version_number: String,
    pub(crate) consistency_request: bool,
    pub(crate) dis_state: i64,
    pub(crate) err_code: i64,
    pub(crate) force_upgrade: bool,
    pub(crate) message: String,
    pub(crate) module: String,
    pub(crate) new_version_state: i64,
    pub(crate) ota_new_version_number: String,
    pub(crate) progress: String,
    pub(crate) sequence_id: i64,
    pub(crate) status: String,
}

#[derive(Serialize, Deserialize)]
pub struct Upload {
    pub(crate) file_size: i64,
    pub(crate) finish_size: i64,
    pub(crate) message: String,
    pub(crate) oss_url: String,
    pub(crate) progress: i64,
    pub(crate) sequence_id: String,
    pub(crate) speed: i64,
    pub(crate) status: String,
    pub(crate) task_id: String,
    pub(crate) time_remaining: i64,
    pub(crate) trouble_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct Xcam {
    pub(crate) allow_skip_parts: bool,
    pub(crate) buildplate_marker_detector: bool,
    pub(crate) first_layer_inspector: bool,
    pub(crate) halt_print_sensitivity: String,
    pub(crate) print_halt: bool,
    pub(crate) printing_monitor: bool,
    pub(crate) spaghetti_detector: bool,
}
