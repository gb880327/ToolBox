<template>
    <div class="content">
        <el-table></el-table>
        <myPagination :total="total" :pageSize="pageSize" :pageNum="pageNum" @pageChange="pageChange"></myPagination>
    </div>
</template>
<script>
export default {
    data(){
        return {
            total: 0,
            pageNum: 0,
            pageSize: 10
        }
    },
    methods: {
        pageChange(pageNum, pageSize){
            this.pageNum = pageNum;
            this.pageSize = pageSize;
        }
    }
}
</script>
