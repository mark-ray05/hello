<script setup>
import { ref } from 'vue';
import { hello3_backend } from 'declarations/hello3_backend/index';

let greeting = ref('');
let submittedNames = ref([]); // 用于存储已提交的名字列表

async function handleSubmit(e) {
  e.preventDefault();
  const target = e.target;
  const name = target.querySelector('#name').value;

  // 调用 greet 方法
  await hello3_backend.greet(name).then((response) => {
    greeting.value = response;
  });

  // 获取所有已提交的名字
  await updateSubmittedNames();
}

async function updateSubmittedNames() {
  // 调用 get_submitted_names 方法并更新 submittedNames
  const names = await hello3_backend.get_submitted_names();
  submittedNames.value = names;
}

// 初始化加载已提交的名字列表
updateSubmittedNames();

</script>

<template>
  <main>
    <img src="/logo2.svg" alt="DFINITY logo" />
    <br />
    <br />
    <form action="#" @submit="handleSubmit">
      <label for="name">Enter your name: &nbsp;</label>
      <input id="name" alt="Name" type="text" />
      <button type="submit">Click Me!</button>
    </form>
    <section id="greeting">{{ greeting }}</section>

    <!-- 显示所有已提交的名字列表 -->
    <section id="submitted-names">
      <h2>All Greeted Names</h2>
      <ul>
        <li v-for="(name, index) in submittedNames" :key="index">{{ name }}</li>
      </ul>
    </section>
  </main>
</template>
