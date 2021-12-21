<template>
    <div class="content">
        <el-row>
            <el-col :span="6" class="border-right" :style="{height: height + 'px'}">
                <ul class="list">
                    <li v-for="(item, i) of projects" :key="i" :class="current.id === item.id ? 'selected' :''">
                        <span @click="select(item)">{{item.name}}</span>
                    </li>
                </ul>
            </el-col>
            <el-col :span="18" style="padding: 0 10px;">
                <el-button type="success" round size="small" style="width: 120px;">执行 <i class="el-icon-caret-right"></i></el-button>
                <el-divider></el-divider>
                <el-row>
                    <el-col :span="11">
                        <el-card class="box-card">
                            <div slot="header">
                                <el-checkbox v-model="allDbChecked" style="display: inline !important;"></el-checkbox>
                                <span style="padding-left:10px;">数据表</span>
                            </div>
                            <el-checkbox-group v-model="tables" size="small">
                                <el-checkbox label="备选项1"></el-checkbox>
                                <el-checkbox label="备选项2"></el-checkbox>
                            </el-checkbox-group>
                        </el-card>
                    </el-col>
                    <el-col :span="1">&nbsp;</el-col>
                    <el-col :span="12">
                        <el-card class="box-card">
                            <div slot="header">
                                <el-checkbox v-model="allTemplateChecked" style="display: inline !important;"></el-checkbox>
                                <span style="padding-left:10px;">项目模板</span>
                            </div>
                            <el-checkbox-group v-model="templates" size="small">
                                <el-checkbox label="备选项1"></el-checkbox>
                                <el-checkbox label="备选项2"></el-checkbox>
                            </el-checkbox-group>
                        </el-card>
                    </el-col>
                </el-row>
            </el-col>
        </el-row>
    </div>
</template>
<script>
export default {
    data(){
        return {
            height: window.innerHeight - 90,
            projects: [{id: 1, name: 'Demo'}, {id: 2, name: 'Test'}],
            current: {id: 1},
            allDbChecked: false,
            allTemplateChecked: false,
            tables: [],
            templates: []
        }
    },
    methods: {
        select(item){
            this.current = item
        }
    }
}
</script>
<style scoped>
.border-right {
    border-right: 1px solid #dcdfe6;
}
.list{
  list-style: none;
  margin: 0;
  padding: 0;
  width: 95%;
}
.list li{
  list-style: none;
  margin: 8px 0;
  padding: 0 10px;
  border-radius: 5px;
  border: 1px solid #303133;
}
.list li span{
  vertical-align: middle;
  line-height: 32px;
  font-size: 12px;
  display: inline-block;
  width: calc(100% - 50px);
  cursor: pointer;
}
.selected {
  background-color: #25374F;
  border: 1px solid #FFFFFF !important;
}
.el-checkbox {
    display: block !important;
    line-height: 26px !important;
}
.box-card {
    height: 100%;
    overflow-y: scroll;
}
</style>