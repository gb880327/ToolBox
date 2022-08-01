import { ElLoading, ElMessageBox, ElNotification } from "element-plus";
import { open } from "@tauri-apps/api/dialog";
import { appDir } from "@tauri-apps/api/path";

let loadingInstance;
const loading = () => {
  loadingInstance = ElLoading.service({
    text: "系统处理中....",
  });
};
const stopLoading = () => {
  if (loadingInstance) {
    loadingInstance.close();
  }
};
const confirm = (msg, cb: Function = () => {}, cancel: Function = () => {}) => {
  ElMessageBox.confirm(msg, "操作提示", {
    confirmButtonText: "确定",
    cancelButtonText: "取消",
    type: "warning",
  })
    .then(() => {
      if (cb) cb();
    })
    .catch(() => {
      if (cancel) cancel();
    });
};
const success = (msg) => {
  ElNotification({
    title: "操作成功",
    message: msg,
    type: "success",
  });
};
const error = (msg) => {
  ElNotification({
    title: "操作失败",
    message: msg,
    type: "error",
  });
};
const chooseDir = async (dir = "") => {
  return await open({
    multiple: false,
    directory: true,
    defaultPath: dir ? dir : await appDir(),
  });
};
const chooseFile = async (dir = "") => {
  return await open({
    directory: false,
    multiple: false,
    defaultPath: dir ? dir : await appDir(),
  });
};
export default {
  loading,
  stopLoading,
  confirm,
  success,
  error,
  chooseDir,
  chooseFile,
};
