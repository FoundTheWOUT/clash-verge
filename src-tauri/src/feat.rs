use crate::core::*;
use crate::data::*;
use crate::log_if_err;

// 切换模式
pub fn change_clash_mode(mode: &str) {
  let core = Core::global();
  log_if_err!(core.update_mode(mode));
}

// 切换系统代理
pub fn toggle_system_proxy() {
  let core = Core::global();
  let data = Data::global();

  let verge = data.verge.lock();
  let enable = !verge.enable_system_proxy.clone().unwrap_or(false);
  drop(verge);

  log_if_err!(core.patch_verge(Verge {
    enable_system_proxy: Some(enable),
    ..Verge::default()
  }));
}

// 打开系统代理
pub fn enable_system_proxy() {
  let core = Core::global();
  log_if_err!(core.patch_verge(Verge {
    enable_system_proxy: Some(true),
    ..Verge::default()
  }));
}

// 关闭系统代理
pub fn disable_system_proxy() {
  let core = Core::global();
  log_if_err!(core.patch_verge(Verge {
    enable_system_proxy: Some(false),
    ..Verge::default()
  }));
}

// 切换tun模式
pub fn toggle_tun_mode() {
  let core = Core::global();
  let data = Data::global();

  let verge = data.verge.lock();
  let enable = !verge.enable_tun_mode.clone().unwrap_or(false);
  drop(verge);

  log_if_err!(core.patch_verge(Verge {
    enable_tun_mode: Some(enable),
    ..Verge::default()
  }));
}

// 打开tun模式
pub fn enable_tun_mode() {
  let core = Core::global();
  log_if_err!(core.patch_verge(Verge {
    enable_tun_mode: Some(true),
    ..Verge::default()
  }));
}

// 关闭tun模式
pub fn disable_tun_mode() {
  let core = Core::global();
  log_if_err!(core.patch_verge(Verge {
    enable_tun_mode: Some(false),
    ..Verge::default()
  }));
}
