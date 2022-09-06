<template>
    <button
        class="btn leading-relaxed font-medium flex items-center cursor-pointer border shadow hover:shadow-lg focus:outline-none focus:shadow-outline rounded"
        :class="{
            [`bg-${color}-600 text-${color}-100 border-${color}-600 hover:bg-${color}-700 hover:border-${color}-700 hover:text-white`]: color && !outline,
            [`border-${color}-600 bg-white text-${color}-600 hover:bg-${color}-600 hover:border-${color}-600 hover:text-white`]: outline,
            [`h-8 text-sm ${icon ? 'px-2' : 'px-4'}`]: size === 'sm',
            [`h-10 ${icon ? 'px-3' : 'px-6'}`]: size === 'md',
            [`h-12 text-lg ${icon ? 'px-4' : 'px-12'}`]: size === 'lg',
            'rounded-full': round,
        }" v-bind="$attrs" v-on="$listeners">
        <slot></slot>
    </button>
</template>
  <script>
  export default {
      inheritAttrs: false,
      props: {
          color: {
              type: String,
              default: "teal"
          },
          size: {
              type: String,
              default: "md" //sm, md, lg
          },
          outline: Boolean,
          icon: Boolean,
          round: Boolean
      }
  };
  </script>