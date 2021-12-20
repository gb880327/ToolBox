<template>
    <el-dialog :visible.sync="dialog" :title="title" :width="width" :close-on-click-modal="false" @close="closeHandler" top="5vh">
        <slot name="content"></slot>
        <div style="text-align: center;margin-top: 15px;">
            <el-button @click="cancelHandler" style="margin-right: 5px;" :style="{width: btnWidth ? btnWidth : ''}" size="small">取消</el-button>
            <el-button type="primary" @click="submitHandler"  style="margin-left: 5px;" :style="{width: btnWidth ? btnWidth : ''}"  size="small">确定</el-button>
        </div>
    </el-dialog>
</template>
<script>
export default {
    name: 'myDialog',
    props: ['width', 'btnWidth', 'title'],
    data(){
        return {
            dialog: false
        }
    },
    methods: {
        show(){
            this.dialog = true
        },
        close(){
            this.dialog = false
        },
        closeHandler(){
            this.$emit('cancel')
        },
        cancelHandler(){
            this.close()
            this.$emit('cancel')
        },
        submitHandler(){
            this.$emit('ok')
        }
    },
}
</script>
<style scoped>
.el-dialog__title {
    color: black !important;
}
</style>