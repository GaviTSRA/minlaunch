<script>
    import { invoke } from "@tauri-apps/api/tauri"

    export let profiles;
    export let currentProfile;

    let visible = false;
    $: currentProfileData = profiles.filter(profile => profile.id == currentProfile)[0]
    
    if (!currentProfileData) {
        currentProfileData = {
            "name": "No profile selected",
            "id": -1
        }
    }

    async function setProfile(id) {
        await invoke("set_profile", {id: id})
        visible = false;
    }
</script>

<main>
    {#if visible}
    <div class="selector">
        {#each profiles as profile (profile.id)}
            {#if profile.id != currentProfileData.id}
            <button class="selectorItem" on:click|preventDefault={async () => {await setProfile(profile.id)}}>
                {profile.name}
            </button>
            {/if}
        {/each}
    </div>
    {/if}
    <button class="current" style="--radius:{visible ? '0px' : '10px'}" on:click|preventDefault={() => visible=!visible}>
        <div class="currentProfile">
            {currentProfileData.name}
        </div>
        <img src="chevron-down.svg" alt="arrow down">
    </button>
</main>

<style>
    .selectorItem {
        border: none;
        width: 12rem;
        height: 3rem;
        background-color: #3C3C3C;
        border-radius: 0;
        box-shadow: none;
    }
    .selectorItem:hover {
        background-color: #444;
    }
    .selector {
        position: absolute;
        bottom: 3.5rem;
        left: 1rem;
    }
    .current {
        display: flex;
        align-items: center;
        background-color: #444;
        width: 12rem;
        height: 3rem;
        margin-left: 1rem;
        margin-top: .5rem;
        border-radius: 10px;
        border-top-left-radius: var(--radius);
        border-top-right-radius: var(--radius);
        padding: 0;
    }
    .currentProfile {
        width: 10rem;
    }
</style>