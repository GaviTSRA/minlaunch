<script>
    import { invoke } from "@tauri-apps/api";

    export let profile;
    let editing = false;
    let nameInput

    function openFolder() {
        invoke("open_profile_folder", {id: profile.id})
    }
    function editName() {
        editing = true;
    }
    function saveName() {
        invoke("update_profile", {id: profile.id, name: profile.name})
        editing = false
    }
</script>

<main>
    <div class="container">
        {#if editing}
        <input autofocus on:blur={saveName} bind:this={nameInput} type="text" class="edit" bind:value={profile.name}/>
        {:else}
        <p class="name">{profile.name} <span class="id">({profile.id})</span></p>
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
</main>

<style>
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
    .container:not(:hover) .hide { 
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
        height: 3rem;
    }
    .container:hover {
        background-color: #3f3f3f;
    }
    .name {
        padding-left: 1.5rem;
        font-weight: 550;
    }
    .editing {
        background-color: #222222;
        border: none;
        padding: 0;
        height: 50%;
        padding: .5rem;
        width: 12rem;
        margin-left: 1rem;
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