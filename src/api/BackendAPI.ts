import { invoke } from "@tauri-apps/api";

namespace BackendAPI {
    
    export namespace Audio {

        export type AudioPlayerID = number;

        export const playSound = (audioFilepath: string) : Promise<AudioPlayerID> => {
            return invoke('play_sound', {
                audioPath: audioFilepath                
            });
        }

        export const pauseSound = (audioPlayerID: AudioPlayerID) : Promise<boolean> => {

            return invoke('pause_audio_player', {
                audioPlayerId: audioPlayerID
            });

        }

    }

}

export default BackendAPI;