<script>
    import { invoke } from "@tauri-apps/api";

    export let profile;
    let editing = false;

    function openFolder() {
        invoke("open_profile_folder", {id: profile.settings.id})
    }
    function editName() {
        editing = true;
    }
    function saveName() {
        invoke("update_profile", {id: profile.settings.id, name: profile.settings.name})
        editing = false
    }
</script>

<main>
    <div class="container">
        {#if editing}
        <input autofocus on:blur={saveName} type="text" class="editing" bind:value={profile.settings.name}/>
        {:else}
        <p class="name">{profile.settings.name} <span class="id">({profile.settings.id})</span></p>
        {/if}
        <button class="imgContainer hide" on:click={editName}>
            <img class="editName" src="edit-2.svg" alt="edit name">
        </button>
        <div class="end hide">
            <button class="imgContainer" on:click={openFolder}>
                <img class="openFolder" src="folder.svg" alt="folder">
            </button>
        </div>
    </div>
    {#if !profile.has_jar}
        <div class="alertRow">
            <img src="alert-triangle.svg"/>
            <p>This profile is missing a desktop.jar!</p>
        </div>
    {/if}
</main>

<style>
    .alertRow {
        display: flex;
        align-items: top;
        margin-left: 1.5rem;
        margin-top: 0;
        justify-content: left;
    }
    .alertRow > img {
        margin-right: .5rem;
        filter: invert(55%) sepia(92%) saturate(3383%) hue-rotate(327deg) brightness(107%) contrast(124%);
    }
    .editName {
        margin-left: 1rem;
        filter: invert(1);
        width: 1rem;
    }
    .end {
        position:absolute;
        right: 1rem;
        display: block;
    }
    main:not(:hover) .hide { 
        display: none; 
    }
    main {
        border-width: 0;
        border-bottom-width: 1px;
        border-color: #1f1f1f;
        border-style: solid;
    }
    .container {
        display: flex;
        flex-direction: row;
        align-items:center;
        justify-content: start;
        height: 2rem;
        padding: .25rem 0;
    }
    main:hover, main:hover > .container > .editing {
        background-color: #3f3f3f;
    }
    .name {
        padding-left: 1.5rem;
        font-weight: 550;
    }
    .editing {
        background-color: #2f2f2f;
        border: none;
        padding: 0;
        height: 50%;
        padding: .5rem;
        width: 12rem;
        margin-left: 1rem;
        font-weight: 550;
        box-shadow: none;
    }
    .id {
        font-weight: 400;
        color: #CCCCCC;
        font-size: .75rem;
    }
    .openFolder {
        filter: invert(1);
        transform: rotate(0deg);
        transition: transform .2s ease;
    }
    .imgContainer {
        height: 28px;
        width: 28px;
        border: none;
        padding: 0;
        box-shadow: none;
        background-color: #00000000;
    }
    .openFolder:hover {
        transform: rotate(10deg);
    }
</style>