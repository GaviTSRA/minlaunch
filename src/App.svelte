<script>
  import ProfileSelector from "./lib/ProfileSelector.svelte";
  import { invoke } from "@tauri-apps/api/tauri"
  import { listen } from "@tauri-apps/api/event"
  import StartPage from "./lib/StartPage.svelte";

  let state = {}
  let currentProfile
  let profiles = []
  let err_msg

  $: launchBtnText = getText(state[currentProfile])
  $: launchBtnColor = getColor(state[currentProfile])
  $: disabled = state[currentProfile] == 1

  async function launch() {
    await invoke("launch_game")
  }

  async function loadData() {
    let data = await invoke("get_data")
    currentProfile = data.current_profile
    profiles = data.profiles
    profiles.forEach(profile => {
      if (!state[profile.id]) state[profile.id] = 0
    });
  }
  
  listen("start", event => {
    state[event.payload] = 1
    console.log(state)
  })
  listen("stop", event => {
    state[event.payload.profile_id] = 0
    if(event.payload.exit_code != 0) {
      state[event.payload.profile_id] = 2
    }
    console.log(state)
  })
  listen("launch_err", event => {
    err_msg = event.payload
  })
  listen("set_data", event => {
    currentProfile = event.payload.current_profile;
    profiles = event.payload.profiles;
    profiles.forEach(profile => {
      if (!state[profile.id]) state[profile.id] = 0
    });
  })

  function getColor(s) {
    if (s == 0) return "green"
    else if (s == 1) return "gray"
    else if (s == 2) return "red"
  }
  function getText(s) {
    if (s == 0) return "Launch"
    else if (s == 1) return "Running"
    else if (s == 2) return "Crashed"
  }
</script>

<main class="container">
  {#await loadData()}
    <p>Loading...</p>
  {:then _}
  <div class="page">
    {#if err_msg}
      <div class="errMsg">
        <p>{err_msg}</p>
        <button on:click={() => err_msg=undefined}>Ok</button>
      </div>
    {/if}
    {#if true}
    <StartPage/>
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
    height: 100vh;
  }
  .page {
    max-height: 100vh;
    height: 100%;
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
