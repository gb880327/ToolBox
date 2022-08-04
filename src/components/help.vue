<template>
    <el-tooltip placement="bottom">
        <el-icon :size="18" class="help-icon"><QuestionFilled /></el-icon>
        <template #content>
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
        </template>
    </el-tooltip>
</template>
<script lang="ts" setup>
import { QuestionFilled } from '@element-plus/icons-vue'

</script>
<style scoped>
.help-icon {
    vertical-align: text-bottom;
    cursor: pointer;
    margin-left: 10px;
}
.help-icon:hover {
    color: #409EFF
}
.tips-content {
  padding: 5px 0;
  font-size: 14px;
  /* color:#FFFFFF; */
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
.helpTips .sub{
  margin-left:12px;
}
.helpTips .sub span{
  display: block;
  padding: 0 0;
}
</style>