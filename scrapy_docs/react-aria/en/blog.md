# Blog

<PostList
  currentPage={props.currentPage}
  pages={props.pages.filter(page => page.name.startsWith('react-aria/blog/') && !page.name.endsWith('index')) ?? []}
/>
