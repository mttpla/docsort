use crate::setup::config::Config;
use notify::{
    Event, EventKind,
    event::{MetadataKind, ModifyKind},
};

pub(crate) fn filter_supported_modify_metadata_extended_file_event(
    cfg: &Config,
    event: Event,
) -> Option<Event> {
    let EventKind::Modify(ModifyKind::Metadata(MetadataKind::Extended)) = event.kind else {
        return None;
    };

    let has_supported_file = event
        .paths
        .iter()
        .any(|p| p.is_file() && is_supported_extension(cfg, p));

    if has_supported_file {
        Some(event)
    } else {
        None
    }
}

fn is_supported_extension(cfg: &Config, path: &std::path::Path) -> bool {
    let Some(ext) = path.extension().and_then(|e| e.to_str()) else {
        return false;
    };

    let ext = ext.to_ascii_lowercase();
    cfg.allowed_file_extensions
        .iter()
        .any(|allowed| allowed.eq_ignore_ascii_case(ext.as_str()))
}
