const rust = import('./md_website')

const btn = document.getElementById('parse')
const previewArea = document.getElementById('output')
const input = document.getElementById('markdown')

input.addEventListener('input', async(event) => {
  const test = await rust;
  previewArea.innerHTML = test.parse(input.value)
});
