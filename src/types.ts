export type Settings = {
    install_path: String | null,
    current_profile: number,
    download_sources: Array<String>,
    minimize_on_launch: boolean,
    minimize_on_close: boolean
}

export type Profile = {
    settings: ProfileSettings,
    has_jar: boolean
}

export type ProfileSettings = {
    id: number,
    name: String
}

export type Data = {
    profiles: Array<Profile>,
    settings: Settings
}

export type ExitData = {
    exit_code: number,
    profile_id: number
}