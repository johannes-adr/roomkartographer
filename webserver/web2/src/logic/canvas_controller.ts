import { writable, type Writable } from "svelte/store";
import type { SmithJS } from "../../public/pkg/smith_js";
import { WebPacket, type WebPoint } from "./smith_types";

let rotations: {rotation: number,time: number}[] = [];

export let rotation: Writable<number> = writable(0);
let rot = 0;

export async function handleWebsocket(msg: MessageEvent<any>, smith: SmithJS, render: (pts: WebPoint[],rot: number)=>void,) {
    
    let data = msg.data;
    if (data instanceof Blob) {
        let binary = new Uint8Array(await data.arrayBuffer());
        let packet: WebPacket = smith.deserialize(binary, "WebPacket");
        Object.setPrototypeOf(packet, WebPacket.prototype);
        if (packet.getTag() == "LidarScan") {
            let lidarscan = packet.as_LidarScan();
            let time_ms = Number.parseInt(lidarscan.time);

            //Find closest rotation
            if (rotations.length > 0) {
                let min = rotations[0];
                let mindiff = Math.abs(min.time - time_ms)
                for (let i = 1; i < rotations.length; i++) {
                    let loc = rotations[i];
                    let diff = Math.abs(loc.time - time_ms);
                    if (diff < mindiff) {
                        mindiff = diff;
                        min = loc;
                    }
                }
                rot = min.rotation;
            }
            rotation.set(rot)
            render(lidarscan.points, rot);
        } else if (packet.getTag() == "IMURotation") {
            let pack = packet.as_IMURotation();
            rotations.push({ rotation: pack.rotation, time: Number.parseInt(pack.time) });
            if (rotations.length > 20) {
                rotations = [...rotations.slice(1)];
            }
        }

        return packet;
    }   
}