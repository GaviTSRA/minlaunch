<script>
    import { invoke } from "@tauri-apps/api";

    export let settings

    let minimize_on_launch = settings.minimize_on_launch
    let minimize_on_close = settings.minimize_on_close
    
    function changeSetting(setting, value) {
        invoke("change_bool_setting", {"setting": setting, "value": value})
    }
</script>

<main>
    <div class="checkboxes">
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
    .checkboxes {
        display: flex;
        align-items: start;
        flex-direction: column;
        margin: 1rem;
    }
</style>