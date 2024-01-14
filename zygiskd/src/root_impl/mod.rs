mod kernelsu;
mod apatch;

#[derive(Debug)]
pub enum RootImpl {
    None,
    TooOld,
    Abnormal,
    Multiple,
    KernelSU,
    APatch,
}

static mut ROOT_IMPL: RootImpl = RootImpl::None;

pub fn setup() {
    let ksu_version = kernelsu::get_kernel_su();
    let apatch_version = apatch::get_apatch();

    let impl_ = match (ksu_version, apatch_version) {
        (None, None ) => RootImpl::None,
        (Some(_), Some(_)) => RootImpl::Multiple,
        (Some(ksu_version), None) => {
            match ksu_version {
                kernelsu::Version::Supported => RootImpl::KernelSU,
                kernelsu::Version::TooOld => RootImpl::TooOld,
                kernelsu::Version::Abnormal => RootImpl::Abnormal,
            }
        }
        (None, Some(apatch_version)) => {
            match apatch_version {
                apatch::Version::Supported => RootImpl::APatch,
                apatch::Version::TooOld => RootImpl::TooOld,
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
        _ => panic!("uid_granted_root: unknown root impl {:?}", get_impl()),
    }
}

pub fn uid_should_umount(uid: i32) -> bool {
    match get_impl() {
        RootImpl::KernelSU => kernelsu::uid_should_umount(uid),
        RootImpl::APatch => apatch::uid_should_umount(uid),
        _ => panic!("uid_should_umount: unknown root impl {:?}", get_impl()),
    }
}
