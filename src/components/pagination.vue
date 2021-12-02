<template>
  <div class="pagination">
      <el-pagination
        @size-change="handleSizeChange"
        @current-change="handleCurrentChange"
        @prev-click="prevPage"
        @next-click="nextPage"
        :current-page="pageNum"
        :page-sizes="[10, 20, 30, 40]"
        :page-size="pageSize"
        layout="total, sizes, prev, pager, next, jumper"
        :total="total"
    >
    </el-pagination>
  </div>
</template>
<script>
export default {
  name: "MyPagination",
  props: {
    total: {
      type: Number,
    },
    pageSize: {
      type: Number,
      default: 10,
    },
    pageNum: {
      type: Number,
      default: 1,
    },
  },
  methods: {
    nextPage(value) {
      this.$emit("pageChange", value, this.pageSize);
    },
    prevPage(value) {
      this.$emit("pageChange", value, this.pageSize);
    },
    handleCurrentChange(value) {
      this.$emit("pageChange", value, this.pageSize);
    },
    handleSizeChange(value) {
      this.$emit("pageChange", this.pageNum, value);
    },
  },
};
</script>
<style scoped>
.pagination {
    margin-top: 10px;
    text-align: right;
}
</style>