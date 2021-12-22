import { open } from "@tauri-apps/api/dialog";

const mixin = {
    methods: {
        chooseDir: () => {
            return new Promise((resole, reject) => {
                open({ directory: true }).then(rep => {
                    resole(rep)
                }).catch(err => {
                    reject(err)
                })
            })
        },
        chooseFile: () => {
            return new Promise((resole, reject) => {
                open({ directory: false }).then(rep => {
                    resole(rep)
                }).catch(err => {
                    reject(err)
                })
            })
        }
    }
}
export default mixin