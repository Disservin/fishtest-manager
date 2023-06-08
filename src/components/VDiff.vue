<template>
    <highlightjs class="h-80 bg-stone-700 overflow-y-scroll" language="js" :code="diffText" />
</template>

<script setup lang="ts">
import { ref } from 'vue'

const diffText = ref(`diff --git a/src/evaluate.cpp b/src/evaluate.cpp
index bf6dd69a..35d05427 100644
--- a/src/evaluate.cpp
+++ b/src/evaluate.cpp
@@ -1071,8 +1071,8 @@ Value Eval::evaluate(const Position& pos) {
       Value nnue = NNUE::evaluate(pos, true, &nnueComplexity);

       // Blend optimism with nnue complexity and (semi)classical complexity
-      optimism += 25 * optimism * (nnueComplexity + abs(psq - nnue)) / 16384;
-      v = (nnue * (945 + npm) + optimism * (174 + npm)) / 1024;
+      optimism += optimism * (nnueComplexity + abs(psq - nnue)) / 512;
+      v = (nnue * (945 + npm) + optimism * (150 + npm)) / 1024;
   }

   // Damp down the evaluation linearly when shuffling
   // Damp down the evaluation linearly when shuffling
   // Damp down the evaluation linearly when shuffling
   // Damp down the evaluation linearly when shuffling
   // Damp down the evaluation linearly when shuffling
   // Damp down the evaluation linearly when shuffling
   // Damp down the evaluation linearly when shuffling
   // Damp down the evaluation linearly when shuffling

   // Damp down the evaluation linearly when shuffling
   // Damp down the evaluation linearly when shuffling
   // Damp down the evaluation linearly when shuffling
   // Damp down the evaluation linearly when shuffling

`)
</script>

<style scoped></style>
