export type Settings = {
    install_path: string | null,
    current_profile: number,
    download_sources: Array<string>,
    minimize_on_launch: boolean,
    minimize_on_close: boolean
}

export type Profile = {
    settings: ProfileSettings,
    has_jar: boolean
}

export type ProfileSettings = {
    id: number,
    name: string
}

export type Data = {
    profiles: Array<Profile>,
    settings: Settings
}

export type ExitData = {
    exit_code: number,
    profile_id: number
}

export type DownloadVersion = {
    name: string,
    asset_name: string,
    asset_url: string,
    asset_size: number
}