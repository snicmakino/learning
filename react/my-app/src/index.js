import React from 'react';
import main from "./main";

document.body.innerHTML = `
      <style>
      body {
  font: 14px "Century Gothic", Futura, sans-serif;
  margin: 20px;
}
ol, ul {
  padding-left: 30px;
}
.board-row:after {
  clear: both;
  content: "";
  display: table;
}
.status {
  margin-bottom: 10px;
}
.square {
  background: #fff;
  border: 1px solid #999;
  float: left;
  font-size: 24px;
  font-weight: bold;
  line-height: 34px;
  height: 34px;
  margin-right: -1px;
  margin-top: -1px;
  padding: 0;
  text-align: center;
  width: 34px;
}
.square:focus {
  outline: none;
}
.kbd-navigation .square:focus {
  background: #ddd;
}
.game {
  display: flex;
  flex-direction: row;
}
.game-info {
  margin-left: 20px;
}
      </style>
<div id="errors" style="
  background: #c00;
  color: #fff;
  display: none;
  margin: -20px -20px 20px;
  padding: 20px;
  white-space: pre-wrap;
"></div>
<div id="container"></div>
`

window.addEventListener('mousedown', function(e) {
  document.body.classList.add('mouse-navigation');
  document.body.classList.remove('kbd-navigation');
});
window.addEventListener('keydown', function(e) {
  if (e.keyCode === 9) {
    document.body.classList.add('kbd-navigation');
    document.body.classList.remove('mouse-navigation');
  }
});
window.addEventListener('click', function(e) {
  if (e.target.tagName === 'A' && e.target.getAttribute('href') === '#') {
    e.preventDefault();
  }
});

main();
