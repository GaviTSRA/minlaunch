<script lang="ts">
    import ProfileSelector from "./lib/ProfileSelector.svelte";
    import { invoke } from "@tauri-apps/api/tauri"
    import { listen } from "@tauri-apps/api/event"
    import PlayPage from "./lib/PlayPage.svelte";
    import PageSelectorItem from "./lib/PageSelectorItem.svelte";
    import ProfilesPage from "./lib/ProfilesPage.svelte";
    import SettingsPage from "./lib/SettingsPage.svelte";
    import type { Data, ExitData, Profile, Settings } from "./types";
    
    let state: { [profile_id: number] : number} = {}
    let currentProfile: number
    let profiles: Array<Profile> = []
    let err_msg: string | null
    let settings: Settings
    
    $: launchBtnText = getText(state[currentProfile])
    $: launchBtnColor = getColor(state[currentProfile])
    $: disabled = state[currentProfile] == 1 || state[currentProfile] == undefined
    
    async function launch() {
        await invoke("launch_game")
    }
    
    async function loadData() {
        let data: Data = await invoke("get_data")
        currentProfile = data.settings.current_profile
        profiles = data.profiles
        profiles.forEach(profile => {
            if (!state[profile.settings.id]) state[profile.settings.id] = 0
        });
        settings = data.settings
    }
    
    listen<number>("start", event => {
        state[event.payload] = 1
    })
    listen<ExitData>("stop", event => {
        state[event.payload.profile_id] = 0
        if(event.payload.exit_code != 0) {
            state[event.payload.profile_id] = 2
        }
    })
    listen<string>("launch_err", event => {
        err_msg = event.payload
    })
    listen<Data>("set_data", event => {
        currentProfile = event.payload.settings.current_profile;
        profiles = event.payload.profiles;
        profiles.forEach(profile => {
            if (!state[profile.settings.id]) state[profile.settings.id] = 0
        });
        settings = event.payload.settings
    })
    
    function getColor(profileState: number) {
        if (profileState == undefined) return "red"
        if (profileState == 0) return "green"
        else if (profileState == 1) return "gray"
        else if (profileState == 2) return "red"
    }
    function getText(profileState: number) {
        if (profileState == undefined) return "No Profile"
        if (profileState == 0) return "Launch"
        else if (profileState == 1) return "Running"
        else if (profileState == 2) return "Crashed"
    }
    function selectPage(page: string) {
        selectedPage = page
    }
    
    $: selectedPage = "play";
</script>

<main class="container">
    {#await loadData()}
        <p>Loading...</p>
    {:then _}
        <div class="pageSelector">
            <PageSelectorItem title="Play" selected={selectedPage == "play"} on:selectPage={()=>selectPage("play")}/>
            <PageSelectorItem title="Profiles" selected={selectedPage == "profiles"} on:selectPage={()=>selectPage("profiles")}/>
            <PageSelectorItem title="Settings" selected={selectedPage == "settings"} on:selectPage={()=>selectPage("settings")}/>
        </div>
        <div class="page">
            {#if err_msg}
                <div class="errMsg">
                    <p>{err_msg}</p>
                    <button on:click={() => err_msg=null}>Ok</button>
                </div>
            {/if}

            {#if selectedPage=="play"}
                <PlayPage/>
            {:else if selectedPage=="profiles"}
                <ProfilesPage sources={settings.download_sources} profiles={profiles}/>
            {:else if selectedPage=="settings"}
                <SettingsPage settings={settings}/>
            {/if}
        </div>
        <div class="bottomRow">
            <form class="row" on:submit|preventDefault={launch}>
                <div class="selector">
                    <ProfileSelector profiles={profiles} currentProfile={currentProfile}/>
                </div>
                <button 
                    class="launchButton" 
                    type="submit" 
                    style="--bg-color:{launchBtnColor}"
                    disabled={disabled}
                >
                    {launchBtnText}
                </button>
            </form>
        </div>
    {/await}
</main>
        
<style>
    .pageSelector {
        width: 100%;
        background-color: #1f1f1f;
        height: 3rem;
        padding: 0;
        margin: 0;
        position: fixed;
        display:flex;
        align-items: left;
        top: 0;
    }
    .errMsg {
        border-radius: 10px;
        position:absolute;
        top: 0;
        background-color: #222222;
        padding: .5rem;
        width: 30vw;
        left: 35vw;
    }
    .container {
        height: 100%;
        overflow: hidden;
    }
    .page {
        max-height: 90vh;
        height: 100%;
        top: 3rem;
        position:absolute;
        width: 100%;
    }
    .bottomRow {
        position:absolute;
        width: 100%;
        height: 4rem;
        bottom: 0;
        background-color: #1f1f1f;
    }
    .launchButton {
        width: 15vw;
        height: 3rem;
        position: absolute;
        right: 1rem;
        bottom: .5rem;
        background-color: var(--bg-color);
    }
    .selector {
        position: absolute;
        left: 0;
        height: 100%;
    }
</style>
        