<template>
    <myDialog title="模板编辑" ref="dialog" @ok="ok" @cancel="cancel" width="90%" btnWidth="48%">
        <template slot="content">
            <el-form size="small" inline>
                <el-form-item label="名称：">
                    <el-input v-model="form.name" placeholder="请输入模板名称！"></el-input>
                </el-form-item>
                <el-form-item label="模板语言：">
                    <el-select v-model="form.language" @change="langChange">
                        <el-option v-for="(item,i) of config.langs" :key="i" :label="item.label" :value="item.value"></el-option>
                    </el-select>
                </el-form-item>
                <el-form-item>
                    <el-popover placement="bottom" trigger="hover">
                         <div class="tips-content">
                            <span class="title">可用变量</span>
                            <ul class="helpTips">
                            <li><a href="https://shopify.github.io/liquid/basics/introduction/" target="_blank">模板语言参考</a></li>
                            <li>base_package - 基础包名 ( java )</li>
                            <li>package - 包名 ( java )</li>
                            <li>imports - 需要导入的包 ( java )</li>
                            <li>entity_name - 类名</li>
                            <li>table_name - 表名</li>
                            <li>primary_key - 主键</li>
                            <li>pri_type - 主键数据类型</li>
                            <li>remark - 表备注信息</li>
                            <li>date - 当前日期(yyyy-MM-dd)</li>
                            <li>
                                <span>fields - 表字段</span>
                                <div class="sub">
                                <span>field_name - 表字段名</span>
                                <span>name - 实体字段名</span>
                                <span>data_type - 数据类型</span>
                                <span>comment - 字段备注</span>
                                <!-- <span>key - 是否为主键</span> -->
                                </div>
                            </li>
                            </ul>
                        </div>
                        <i class="el-icon-question help-icon" slot="reference"></i>
                    </el-popover>
                </el-form-item>
            </el-form>
            <div class="editor">
                <div class="ace-editor" ref="ace" style="margin-top: 10px"></div>
            </div>
        </template>
    </myDialog>
</template>
<script>
import ace from "ace-builds";
import "ace-builds/webpack-resolver";

export default {
    data(){
        return {
            aceEditor: {},
            form: {
                name: '',
                category_id: '',
                language: '',
                content: ''
            }
        }
    },
    methods: {
        show(category, row){
            this.$refs.dialog.show()
            if(row){
                this.form = JSON.parse(JSON.stringify(row))
            }
            this.form.category_id = category
            this.init()
        },
        init(){
            this.$nextTick(() => {
                this.aceEditor = ace.edit(this.$refs.ace, {
                    maxLines: 100, // 最大行数，超过会自动出现滚动条
                    minLines: 32, // 最小行数，还未到最大行数时，编辑器会自动伸缩大小
                    fontSize: 14, // 编辑器内字体大小
                    theme: "ace/theme/dracula", // 默认设置的主题
                    mode: "ace/mode/text", // 默认设置的语言模式
                    tabSize: 4, // 制表符设置为 4 个空格大小
                })
                this.aceEditor.setValue(this.form.content ? this.form.content : '', -1)
                this.aceEditor.on('change', (data)=> {
                    this.form.content = this.aceEditor.getValue()
                })
                this.langChange(this.form.language)
            })
        },
        langChange(value){
            this.aceEditor.session.setMode("ace/mode/" + value)
        },
        ok(){
            if(!this.form.name){
                this.error('请填写模板名称！')
                return
            }
            if(!this.form.language){
                this.error('请选择模板语言！')
                return
            }
            if(!this.form.content){
                this.error('请填写模板内容！')
                return
            }
            this.invoke('SaveTemplate', (data)=> {
                this.success('保存成功！')
                this.$emit('update', this.form.category_id)
                this.cancel()
                this.$refs.dialog.close()
            }, {template: this.form})
        },
        cancel(){
            this.aceEditor.setValue('', -1)
            this.form = { name: '', category_id: '', language: '', content: '' }
        }
    }
}
</script>
<style scoped>
.editor {
  overflow-y: scroll;
  height: 500px;
}
.help-icon {
    font-size: 22px;
    vertical-align: middle;
    cursor: pointer;
}
.help-icon:hover {
    color: #409EFF
}
.tips-content {
  padding: 5px 0;
  font-size: 14px;
  color:#303133;
}
.tips-content .title{
  display: block;
  padding: 5px 0 10px 10px;
  border-bottom: 1px #8EACC5 solid;
}
.helpTips{
  list-style: none;
  padding: 0;
  margin: 0;
}
.helpTips li{
  list-style: none;
  padding: 0;
  margin: 0;
}
.helpTips a{
  color: black;
}
.helpTips .sub{
  margin-left:12px;
}
.helpTips .sub span{
  display: block;
  padding: 0 0;
}
</style>