<script>
    import { invoke } from "@tauri-apps/api";

    export let profile;

    function openFolder() {
        invoke("open_profile_folder", {id: profile.id})
    }
</script>

<main>
    <div class="container">
        <p class="name">{profile.name} <span class="id">({profile.id})</span></p>
        <div class="end">
            <button class="imgContainer" on:click={openFolder}>
                <img class="openFolder" src="folder.svg" alt="folder">
            </button>
        </div>
    </div>
</main>

<style>
    .end {
        position:absolute;
        right: 1rem;
        display: block;
    }
    .container:not(:hover) .end { 
        display: none; 
    }
    .container {
        display: flex;
        flex-direction: row;
        align-items:center;
        justify-content: start;
        border-width: 0;
        border-bottom-width: 1px;
        border-color: #1f1f1f;
        border-style: solid;
    }
    .container:hover {
        background-color: #3f3f3f;
    }
    .name {
        padding-left: 1.5rem;
        font-weight: 550;
    }
    .id {
        font-weight: 400;
        color: #CCCCCC;
        font-size: .75rem;
    }
    .openFolder {
        filter: invert(1);
    }
    .imgContainer {
        height: 28px;
        width: 28px;
        border: none;
        padding: 0;
        box-shadow: none;
        background-color: #00000000;
        transform: rotate(0deg);
        transition: transform .2s ease;
    }
    .imgContainer:hover {
        transform: rotate(10deg);
    }
</style>