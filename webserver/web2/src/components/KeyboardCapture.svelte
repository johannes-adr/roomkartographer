<script lang="ts">
    import type { SmithJS } from "../../public/pkg/smith_js";
    import { WebMovement, WebPacket } from "../logic/smith_types";
    export let ws: WebSocket;
    export let smith: SmithJS
     function sendPacket(packet: WebPacket) {
        ws.send(smith.serialize(packet, "WebPacket"));
    }

    let last = "_";

</script>
<div></div>
<svelte:body
    on:keyup={() => {
        last = "_";
        sendPacket(WebPacket.Movement(WebMovement.Stop()));
    }}
    on:keydown={(ev) => {
        let k = ev.key;
        if (k == last) {
            return;
        }
        last = k;
        if (k == "w") {
            sendPacket(WebPacket.Movement(WebMovement.Forward()));
        } else if (k == "a") {
            sendPacket(WebPacket.Movement(WebMovement.Left()));
        } else if (k == "s") {
            sendPacket(WebPacket.Movement(WebMovement.Stop()));
        } else if (k == "d") {
            sendPacket(WebPacket.Movement(WebMovement.Right()));
        }
    }}
/>
