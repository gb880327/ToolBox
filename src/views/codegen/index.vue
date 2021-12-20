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
                <el-button type="success" round size="small">执行 <i class="el-icon-caret-right"></i></el-button>
                <el-divider></el-divider>
                <el-row>
                    <el-col :span="12">
                        <el-card class="box-card"></el-card>
                    </el-col>
                    <el-col :span="12">
                        <el-card class="box-card"></el-card>
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
            current: {id: 1}
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
</style>