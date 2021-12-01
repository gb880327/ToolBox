<template>
  <div id="app">
  </div>
</template>

<script>

export default {
  name: 'App',
  components: {},
  created(){
    this.invoke('ProjectList',this.getProjects)
    this.invoke('DeployProject', this.getProjects, {projectId: 1})
  },
  methods:{
    getProjects(data){
      console.info("=================",data)
    }
  }
}
</script>

<style>

</style>
