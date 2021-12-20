<template>
    <div class="content">
        <el-row>
            <el-col :span="6" class="border-right" :style="{height: height + 'px'}">
                <el-form label-width="100px" :style="{height: (height-40) + 'px', maxHeight: (height-40) + 'px'}">
                    <el-form-item label="部署项目：" class="item">
                        <div class="items">
                            <el-radio-group v-model="projectId">
                                <el-radio :label="3">备选项</el-radio>
                                <el-radio :label="6">备选项</el-radio>
                                <el-radio :label="9">备选项</el-radio>
                            </el-radio-group>
                        </div>
                    </el-form-item>
                    <el-form-item label="部署服务器：" class="item">
                        <div class="items">
                            <el-checkbox-group v-model="serverIds" size="small">
                                <el-checkbox label="备选项1"></el-checkbox>
                                <el-checkbox label="备选项2"></el-checkbox>
                            </el-checkbox-group>
                        </div>
                    </el-form-item>
                    <el-form-item label="部署环境：" class="item">
                        <div class="items">
                            <el-radio-group v-model="profile">
                                <el-radio :label="3">备选项</el-radio>
                                <el-radio :label="6">备选项</el-radio>
                                <el-radio :label="9">备选项</el-radio>
                            </el-radio-group>
                        </div>
                    </el-form-item>
                </el-form>

                <el-button type="success" round style="width: 70%;margin: 0 10%;" @click="addLog('abcd', true)">部署</el-button>
            </el-col>
            <el-col :span="18">
                <div class="console" :style="{height: (height - 20) + 'px'}"></div>
            </el-col> 
        </el-row>
    </div>
</template>
<script>
export default {
    data(){
        return {
            height: window.innerHeight - 90,
            projects: [],
            projectId: '',
            profile: '',
            serverIds: []
        }
    },
    created(){
        
    },
    methods: {
        goToEnd(){
            let div = document.getElementsByClassName('console')[0]
            div.scrollTop = div.scrollHeight;
        },
        addLog(msg, error){
            let div = document.getElementsByClassName('console')[0]
            let span = document.createElement('span')
            span.setAttribute('style', 'margin: 2px 0; display: block;' + (error ? 'color: #F56C6C;' : ''))
            span.innerHTML = msg
            div.appendChild(span)
            this.goToEnd()
        }
    }
}
</script>
<style scoped>
.item {
    overflow-x: scroll;
}
.border-right {
    border-right: 1px solid #dcdfe6;
}
.items {
    max-height: 250px;
    overflow-y: scroll;
}
.el-radio {
    display: block !important;
    margin: 10px 0 !important;
}
.el-checkbox {
    display: block !important;
    line-height: 26px !important;
}
.console {
    margin-left: 10px;
    padding: 10px 10px;
    height: 100%;
    overflow-y: scroll;
    background-color: #171E29;
}
</style>