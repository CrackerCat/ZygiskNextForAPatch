mod kernelsu;
mod magisk;
mod apatch;

#[derive(Debug)]
pub enum RootImpl {
    None,
    TooOld,
    Multiple,
    Abnormal,
    KernelSU,
    Magisk,
    APatch,
}

static mut ROOT_IMPL: RootImpl = RootImpl::None;

pub fn setup() {
    let ksu_version = kernelsu::get_kernel_su();
    let apatch_version = apatch::get_apatch();
    let magisk_version = magisk::get_magisk();

    let impl_ = match (ksu_version, apatch_version, magisk_version) {
        (None, None, None) => RootImpl::None,
        (Some(_), Some(_), _) => RootImpl::Multiple,
        (Some(ksu_version), None, _) => {
            match ksu_version {
                kernelsu::Version::Supported => RootImpl::KernelSU,
                kernelsu::Version::TooOld => RootImpl::TooOld,
                kernelsu::Version::Abnormal => RootImpl::Abnormal,
            }
        }
        (None, Some(apatch_version), _) => {
            match apatch_version {
                apatch::Version::Supported => RootImpl::APatch,
                apatch::Version::TooOld => RootImpl::TooOld,
            }
        },
        (None, None, Some(magisk_version)) => {
            match magisk_version {
                magisk::Version::Supported => RootImpl::Magisk,
                magisk::Version::TooOld => RootImpl::TooOld,
            }
        }
    };
    unsafe { ROOT_IMPL = impl_; }
}

pub fn get_impl() -> &'static RootImpl {
    unsafe { &ROOT_IMPL }
}

pub fn uid_granted_root(uid: i32) -> bool {
    match get_impl() {
        RootImpl::KernelSU => kernelsu::uid_granted_root(uid),
        RootImpl::APatch => apatch::uid_granted_root(uid),
        RootImpl::Magisk => magisk::uid_granted_root(uid),
        _ => panic!("uid_granted_root: unknown root impl {:?}", get_impl()),
    }
}

pub fn uid_should_umount(uid: i32) -> bool {
    match get_impl() {
        RootImpl::KernelSU => kernelsu::uid_should_umount(uid),
        RootImpl::APatch => apatch::uid_should_umount(uid),
        RootImpl::Magisk => magisk::uid_should_umount(uid),
        _ => panic!("uid_should_umount: unknown root impl {:?}", get_impl()),
    }
}

pub fn uid_is_manager(uid: i32) -> bool {
    match get_impl() {
        RootImpl::KernelSU => kernelsu::uid_is_manager(uid),
        RootImpl::Magisk => magisk::uid_is_manager(uid),
        RootImpl::APatch => apatch::uid_is_manager(uid),
        _ => panic!("uid_is_manager: unknown root impl {:?}", get_impl()),
    }
}