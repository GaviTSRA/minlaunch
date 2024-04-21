<script lang="ts">
    import { createEventDispatcher } from "svelte";
    import type { DownloadVersion, Profile } from "../types";
    import DropdownInput from "./DropdownInput.svelte";
    import { invoke } from "@tauri-apps/api";
    import { listen } from "@tauri-apps/api/event";

    export let profile: Profile;
    export let sources: Array<string>;

    $: versions = [] as Array<DownloadVersion>
    let selectedVersion: string
    let installing = false
    let progress = 0

    const dispatch = createEventDispatcher();

    async function selectSource(value: string) {
        versions = []
        versions = await invoke("get_source_versions", {source: value});
        selectedVersion = versions[0].name == "" ? versions[0].asset_name : versions[0].name
    }

    function close() {
        dispatch("close")
    }
    async function install() {
        if (!selectedVersion) {
            console.error("No version selected")
            return
        }
        if (installing) {
            console.error("Already installing a verrsion")
            return
        }
        let version = versions.find(el => el.name == "" ? el.asset_name == selectedVersion : el.name == selectedVersion);
        if (!version) {
            console.error("Failed to determin version")
            return
        }
        installing = true;
        await invoke("install_version_jar", {profileId: profile.settings.id, url: version.asset_url, size: version.asset_size})
    }

    listen<number>("downloadProgress", event => {
        progress = event.payload
    })
    listen<string>("downloadDone", async () => {
        await invoke("get_data_async")
        close();
    })
</script>

<main>
    <!-- svelte-ignore a11y-click-events-have-key-events -->
    <!-- svelte-ignore a11y-no-static-element-interactions -->
    <div on:click={close} class="container">
        <!-- svelte-ignore a11y-click-events-have-key-events -->
        <!-- svelte-ignore a11y-no-static-element-interactions -->
        <div class="window" on:click|stopPropagation={()=>{}}>
            <h1>Download Version</h1>
            <div class="dropdowns">
                <DropdownInput options={sources} on:select={value=>selectSource(value.detail)}/>
                {#await selectSource(sources[0])}
                    <p>loading...</p>
                {:then _} 
                    <DropdownInput options={versions.map(a=>a.name == "" ? a.asset_name : a.name)} on:select={value=>selectedVersion = value.detail}/>
                {/await}
            </div>
            <button class="installBtn" disabled={installing} on:click={install}>{installing ? progress+"%" : "Install"}</button>
        </div>
    </div>
</main>

<style>
    .dropdowns {
        margin-left: 1rem;
    }
    .installBtn {
        position: absolute;
        bottom: 1rem;
        margin: auto;
        left: 25vw;
        width: 20vw;
        background-color: darkgreen;
    }
    .installBtn:hover {
        background-color: green;
    }
    .installBtn:active {
        background-color: rgb(3, 154, 3);
    }
    .container {
        z-index: 3;
        background-color: #00000088;
        height: 100vh;
        width: 100vw;
        position: fixed;
        top: 0;
        left: 0;
    }
    .window {
        border-style: solid;
        border-color: #1f1f1f;
        border-width: 3px;
        position:relative;
        display: flex;
        flex-direction: column;
        align-items: center;
        margin-left: 15vw;
        width: 70vw;
        height: 90vh;
        background-color: #2f2f2f;
        z-index: 4;
        border-radius: 10px;
    }
</style>