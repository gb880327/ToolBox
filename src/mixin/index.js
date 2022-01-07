import { open } from "@tauri-apps/api/dialog";

const mixin = {
    methods: {
        chooseDir: (dir) => {
            return new Promise((resole, reject) => {
                open({ directory: true, defaultPath: dir }).then(rep => {
                    resole(rep)
                }).catch(err => {
                    reject(err)
                })
            })
        },
        chooseFile: (dir) => {
            return new Promise((resole, reject) => {
                open({ directory: false, defaultPath: dir }).then(rep => {
                    resole(rep)
                }).catch(err => {
                    reject(err)
                })
            })
        }
    }
}
export default mixin