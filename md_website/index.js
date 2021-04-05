const rust = import('./md_website')


    const btn = document.getElementById('parse')
    const previewArea = document.getElementById('output')

    btn.addEventListener('click', async() => {
        const input = document.getElementById('markdown').value;
        rust.then(modulee => {
          console.log(input)
          previewArea.innerHTML = input;
          previewArea.innerHTML = modulee.parse(input);
      })
    })
