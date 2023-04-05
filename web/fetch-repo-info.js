import axios from 'axios';
import * as fs from 'fs';
const url = 'https://api.github.com/repos/datafuselabs/askbend';
axios.get(url)
  .then(response => {
    fs.writeFileSync('src/assets/json/repo-info.json', JSON.stringify(response.data, null, 2));
  });

