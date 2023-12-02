import { invoke } from "@tauri-apps/api";
import AudioSystem from "./api/AudioSystem";
import { useCallback, useRef, useState } from "react";

const App = () => {

    const [remote, setRemote] = useState<string>();

    const ipAddressRef = useRef<HTMLInputElement>(null);

    const connect = useCallback(() => {

        if(!ipAddressRef.current)
            return;
        
        invoke('generate_network_remote', {
            address: ipAddressRef.current.value
        }).then((res) => {
            console.log(res);
        }).catch((err) => {
            console.error(err);
        })

    }, [ipAddressRef.current, setRemote]);

    return (
        <>

            <input ref={ipAddressRef} type="text" name="IP Address" id="" placeholder="IP Address" />
            <br />
            <br />
            <button onClick={connect}>Connect</button>

        </>
    )

}

export default App;
