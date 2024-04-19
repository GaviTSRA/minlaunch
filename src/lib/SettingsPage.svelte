<script lang="ts">
    import { invoke } from "@tauri-apps/api";
    import { open } from '@tauri-apps/api/dialog';
    import type { Settings } from "../types";

    export let settings: Settings

    let minimize_on_launch = settings.minimize_on_launch
    let minimize_on_close = settings.minimize_on_close
    
    function changeSetting(setting: String, value: boolean) {
        invoke("change_bool_setting", {"setting": setting, "value": value})
    }

    async function chooseInstallFolder() {
        const folder = await open({
            multiple: false,
            directory: true,
        });
        invoke("set_install_path", {path: folder})
    }
</script>

<main>
    <div class="checkboxes">
        <div class="stringSetting">
            <label>Install Path</label>
            <button on:click={chooseInstallFolder}>Set</button>
            <p class="stringSettingValue">{settings.install_path}</p>
        </div>
        <div>
            <input type="checkbox" bind:checked={minimize_on_launch} on:change={()=>changeSetting("minimize_on_launch", minimize_on_launch)} />
            <label>Minimize on launch</label>
        </div>
        <div>
            <input type="checkbox" bind:checked={minimize_on_close} on:change={()=>changeSetting("minimize_on_close", minimize_on_close)}/>
            <label>Minimize on close</label>
        </div>
    </div>
</main>

<style>
    .stringSetting > button {
        padding: .25rem .5rem;
        margin-left: 1rem;
    }
    .stringSetting > label {
        font-weight: 550;
    }
    .stringSetting {
        display:flex;
        align-items: center;
    }
    .stringSettingValue {
        margin-left: 1rem;
        text-decoration: underline;
    }
    .checkboxes {
        display: flex;
        align-items: start;
        flex-direction: column;
        margin: 1rem;
    }
</style>