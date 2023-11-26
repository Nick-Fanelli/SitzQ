import { invoke } from "@tauri-apps/api";
import { open } from "@tauri-apps/api/dialog"
import { useCallback, useState } from "react"

const App = () => {

    const [soundFile, setSoundFile] = useState<string | null>(null);

    const openSoundFile = useCallback(() => {

        open({
            multiple: false,
        }).then((res) => {
            if(res !== null) {
                setSoundFile(res.toString());
            }
        });

    }, [setSoundFile]);

    return (
        <>
            <h1>{soundFile}</h1>
            <button onClick={openSoundFile}>Open File</button>
            <button onClick={() => invoke('play_sound', { audioPath: soundFile })}>Play Sound File</button>
            <button onClick={() => invoke('print_stats')}>Print Stats</button>
        </>
    )

}

export default App;
