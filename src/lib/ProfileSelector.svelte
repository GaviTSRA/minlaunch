<script>
    import { invoke } from "@tauri-apps/api/tauri"

    export let profiles;
    export let currentProfile;

    let visible = false;
    $: currentProfileData = profiles.filter(profile => profile.settings.id == currentProfile)[0] == undefined ? {
        settings: {
            name: "No profile selected",
            id: -1
        },
        has_jar: false
    } : profiles.filter(profile => profile.settings.id == currentProfile)[0]

    async function setProfile(id) {
        await invoke("set_profile", {id: id})
        visible = false;
    }
</script>

<main>
    <div class={visible ? "selector" : "hide"}>
        {#each profiles as profile (profile.settings.id)}
            {#if profile.settings.id != currentProfileData.settings.id}
            <button class="selectorItem" on:click|preventDefault={async () => {await setProfile(profile.settings.id)}}>
                {profile.settings.name}
            </button>
            {/if}
        {/each}
    </div>
    <button class="current" style="--radius:{visible ? '0px' : '10px'}" on:click|preventDefault={() => visible=!visible}>
        <div class="currentProfile">
            {currentProfileData.settings.name}
        </div>
        <img src="chevron-down.svg" alt="arrow down" class={visible ? "rotate" : ""}>
    </button>
</main>

<style>
    .hide {
        display: none;
    }
    img {
        transition: transform .3s ease;
    }
    .rotate {
        transform:rotate(180deg);
    }
    .selectorItem {
        border: none;
        width: 12rem;
        height: 3rem;
        background-color: #3C3C3C;
        border-radius: 0;
        box-shadow: none;
        transition: background-color .3s ease;
    }
    .selectorItem:hover {
        background-color: #444;
    }
    .selector {
        transition: visibility 0s, opacity 0.5s linear;
        position: absolute;
        bottom: 3.5rem;
        left: 1rem;
        animation: animateUp .5s ease;
    }
    .selectorItem:first-child {
        border-top-left-radius: 10px;
        border-top-right-radius: 10px;
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
    @keyframes animateUp {
        0% {
            transform: translateY(3rem);
        }
        100% {
            transform: translateY(0);
        }
    }
</style>