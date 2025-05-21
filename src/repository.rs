pub struct Repository {
    pub host: String,
    pub path: String,
}

pub fn parse_repo_string(repo_str: &str) -> Result<Repository, &str> {
    let part_after_proto = match repo_str.split_once("://") {
        None => repo_str,
        Some((_, p)) => p,
    };

    let (prefix, suffix) = match part_after_proto.split_once(':') {
        Some(("", _)) | Some((_, "")) | None => return Err("unexpected format"),
        Some((prefix, _)) if prefix.contains('/') => return Err("looks like local path"),
        Some(parts) => parts,
    };

    let host_part = match prefix.split_once('@') {
        None => prefix,
        Some((_, host)) => host,
    };

    let path_part_trimmed = suffix.trim_start_matches('/');

    let path_part = match path_part_trimmed.strip_suffix(".git") {
        None => path_part_trimmed,
        Some(p) => p,
    };

    Ok(Repository {
        host: host_part.to_owned(),
        path: path_part.to_owned(),
    })
}
