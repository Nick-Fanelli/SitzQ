import { invoke } from "@tauri-apps/api";
import AudioSystem from "./api/AudioSystem";

const App = () => {

    return (
        <>

            <button onClick={() => invoke('start_transmission_server')}>Transmit</button>
            <button>Receive</button>

        </>
    )

}

export default App;
