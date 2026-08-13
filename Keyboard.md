Вот готовый HTML: прозрачный слой, на месте каждой клавиши — тонкий прямоугольник только с границей (без заливки). Координаты подобраны по вашей картинке (холст 1160×600), поэтому слой можно класть поверх неё как оверлей.

```html
<!DOCTYPE html>
<html lang="ru">
<head>
<meta charset="utf-8">
<title>Keyboard overlay</title>
<style>
  :root{
    --kb-border: rgba(0,0,0,.55); /* цвет обводки клавиш */
    --kb-radius: 6px;
  }
  body{ margin:0; background:transparent; }

  .kb-wrap{ position:relative; width:1160px; height:600px; }
  .kb-wrap > img{ position:absolute; inset:0; width:100%; height:100%; display:block; }

  /* Сам оверлей — полностью прозрачный */
  .kb-overlay{ position:absolute; inset:0; background:transparent; pointer-events:none; }

  /* Клавиша = только тонкая рамка */
  .kb-overlay .k{
    position:absolute; box-sizing:border-box;
    background:transparent;
    border:1px solid var(--kb-border);
    border-radius:var(--kb-radius);
  }
  /* необязательно: подсветка при наведении/активации */
  .kb-overlay .k.on{ background:rgba(255,140,0,.35); }
</style>
</head>
<body>

<div class="kb-wrap">
  <!-- раскомментируйте и подставьте свою картинку -->
  <!-- <img src="keyboard.png" alt=""> -->

  <div class="kb-overlay">
    <!-- Ряд F -->
    <i class="k" data-code="Escape"      style="left:60px;  top:60px; width:55px; height:44px"></i>
    <i class="k" data-code="F1"          style="left:125px; top:60px; width:45px; height:44px"></i>
    <i class="k" data-code="F2"          style="left:172px; top:60px; width:45px; height:44px"></i>
    <i class="k" data-code="F3"          style="left:220px; top:60px; width:45px; height:44px"></i>
    <i class="k" data-code="F4"          style="left:267px; top:60px; width:45px; height:44px"></i>
    <i class="k" data-code="F5"          style="left:322px; top:60px; width:45px; height:44px"></i>
    <i class="k" data-code="F6"          style="left:369px; top:60px; width:45px; height:44px"></i>
    <i class="k" data-code="F7"          style="left:417px; top:60px; width:45px; height:44px"></i>
    <i class="k" data-code="F8"          style="left:464px; top:60px; width:45px; height:44px"></i>
    <i class="k" data-code="F9"          style="left:519px; top:60px; width:45px; height:44px"></i>
    <i class="k" data-code="F10"         style="left:566px; top:60px; width:45px; height:44px"></i>
    <i class="k" data-code="F11"         style="left:614px; top:60px; width:45px; height:44px"></i>
    <i class="k" data-code="F12"         style="left:661px; top:60px; width:45px; height:44px"></i>
    <i class="k" data-code="PrintScreen" style="left:716px; top:60px; width:50px; height:44px"></i>
    <i class="k" data-code="ScrollLock"  style="left:768px; top:60px; width:50px; height:44px"></i>
    <i class="k" data-code="Pause"       style="left:833px; top:60px; width:50px; height:44px"></i>
    <i class="k" data-code="Insert"      style="left:885px; top:60px; width:50px; height:44px"></i>

    <!-- Ряд цифр -->
    <i class="k" data-code="Backquote"   style="left:60px;  top:127px; width:45px;  height:52px"></i>
    <i class="k" data-code="Digit1"      style="left:107px; top:127px; width:45px;  height:52px"></i>
    <i class="k" data-code="Digit2"      style="left:154px; top:127px; width:45px;  height:52px"></i>
    <i class="k" data-code="Digit3"      style="left:201px; top:127px; width:45px;  height:52px"></i>
    <i class="k" data-code="Digit4"      style="left:248px; top:127px; width:45px;  height:52px"></i>
    <i class="k" data-code="Digit5"      style="left:295px; top:127px; width:45px;  height:52px"></i>
    <i class="k" data-code="Digit6"      style="left:342px; top:127px; width:45px;  height:52px"></i>
    <i class="k" data-code="Digit7"      style="left:389px; top:127px; width:45px;  height:52px"></i>
    <i class="k" data-code="Digit8"      style="left:436px; top:127px; width:45px;  height:52px"></i>
    <i class="k" data-code="Digit9"      style="left:483px; top:127px; width:45px;  height:52px"></i>
    <i class="k" data-code="Digit0"      style="left:530px; top:127px; width:45px;  height:52px"></i>
    <i class="k" data-code="Minus"       style="left:577px; top:127px; width:45px;  height:52px"></i>
    <i class="k" data-code="Equal"       style="left:624px; top:127px; width:45px;  height:52px"></i>
    <i class="k" data-code="Backspace"   style="left:716px; top:127px; width:102px; height:52px"></i>
    <i class="k" data-code="Home"        style="left:833px; top:127px; width:50px;  height:52px"></i>
    <i class="k" data-code="End"         style="left:885px; top:127px; width:50px;  height:52px"></i>

    <!-- Ряд Tab -->
    <i class="k" data-code="Tab"          style="left:60px;  top:195px; width:67px; height:52px"></i>
    <i class="k" data-code="KeyQ"         style="left:132px; top:195px; width:45px; height:52px"></i>
    <i class="k" data-code="KeyW"         style="left:179px; top:195px; width:45px; height:52px"></i>
    <i class="k" data-code="KeyE"         style="left:226px; top:195px; width:45px; height:52px"></i>
    <i class="k" data-code="KeyR"         style="left:273px; top:195px; width:45px; height:52px"></i>
    <i class="k" data-code="KeyT"         style="left:320px; top:195px; width:45px; height:52px"></i>
    <i class="k" data-code="KeyY"         style="left:367px; top:195px; width:45px; height:52px"></i>
    <i class="k" data-code="KeyU"         style="left:414px; top:195px; width:45px; height:52px"></i>
    <i class="k" data-code="KeyI"         style="left:461px; top:195px; width:45px; height:52px"></i>
    <i class="k" data-code="KeyO"         style="left:508px; top:195px; width:45px; height:52px"></i>
    <i class="k" data-code="KeyP"         style="left:555px; top:195px; width:45px; height:52px"></i>
    <i class="k" data-code="BracketLeft"  style="left:602px; top:195px; width:45px; height:52px"></i>
    <i class="k" data-code="BracketRight" style="left:649px; top:195px; width:45px; height:52px"></i>
    <i class="k" data-code="Backslash"    style="left:741px; top:195px; width:77px; height:52px"></i>
    <i class="k" data-code="Delete"       style="left:833px; top:195px; width:50px; height:118px"></i>
    <i class="k" data-code="PageUp"       style="left:885px; top:195px; width:50px; height:52px"></i>

    <!-- Ряд Caps -->
    <i class="k" data-code="CapsLock"   style="left:60px;  top:263px; width:82px;  height:52px"></i>
    <i class="k" data-code="KeyA"       style="left:147px; top:263px; width:45px;  height:52px"></i>
    <i class="k" data-code="KeyS"       style="left:194px; top:263px; width:45px;  height:52px"></i>
    <i class="k" data-code="KeyD"       style="left:241px; top:263px; width:45px;  height:52px"></i>
    <i class="k" data-code="KeyF"       style="left:288px; top:263px; width:45px;  height:52px"></i>
    <i class="k" data-code="KeyG"       style="left:335px; top:263px; width:45px;  height:52px"></i>
    <i class="k" data-code="KeyH"       style="left:382px; top:263px; width:45px;  height:52px"></i>
    <i class="k" data-code="KeyJ"       style="left:429px; top:263px; width:45px;  height:52px"></i>
    <i class="k" data-code="KeyK"       style="left:476px; top:263px; width:45px;  height:52px"></i>
    <i class="k" data-code="KeyL"       style="left:523px; top:263px; width:45px;  height:52px"></i>
    <i class="k" data-code="Semicolon"  style="left:570px; top:263px; width:45px;  height:52px"></i>
    <i class="k" data-code="Quote"      style="left:617px; top:263px; width:45px;  height:52px"></i>
    <i class="k" data-code="Enter"      style="left:700px; top:263px; width:118px; height:52px"></i>
    <i class="k" data-code="PageDown"   style="left:885px; top:263px; width:50px;  height:52px"></i>

    <!-- Ряд Shift -->
    <i class="k" data-code="ShiftLeft"  style="left:60px;  top:327px; width:105px; height:52px"></i>
    <i class="k" data-code="KeyZ"       style="left:170px; top:327px; width:45px;  height:52px"></i>
    <i class="k" data-code="KeyX"       style="left:217px; top:327px; width:45px;  height:52px"></i>
    <i class="k" data-code="KeyC"       style="left:264px; top:327px; width:45px;  height:52px"></i>
    <i class="k" data-code="KeyV"       style="left:311px; top:327px; width:45px;  height:52px"></i>
    <i class="k" data-code="KeyB"       style="left:358px; top:327px; width:45px;  height:52px"></i>
    <i class="k" data-code="KeyN"       style="left:405px; top:327px; width:45px;  height:52px"></i>
    <i class="k" data-code="KeyM"       style="left:452px; top:327px; width:45px;  height:52px"></i>
    <i class="k" data-code="Comma"      style="left:499px; top:327px; width:45px;  height:52px"></i>
    <i class="k" data-code="Period"     style="left:546px; top:327px; width:45px;  height:52px"></i>
    <i class="k" data-code="Slash"      style="left:593px; top:327px; width:45px;  height:52px"></i>
    <i class="k" data-code="ShiftRight" style="left:676px; top:327px; width:142px; height:52px"></i>
    <i class="k" data-code="ArrowUp"    style="left:833px; top:327px; width:50px;  height:52px"></i>

    <!-- Нижний ряд -->
    <i class="k" data-code="Fn"            style="left:60px;  top:390px; width:45px;  height:52px"></i>
    <i class="k" data-code="ControlLeft"   style="left:107px; top:390px; width:58px;  height:52px"></i>
    <i class="k" data-code="MetaLeft"      style="left:167px; top:390px; width:48px;  height:52px"></i>
    <i class="k" data-code="AltLeft"       style="left:217px; top:390px; width:48px;  height:52px"></i>
    <i class="k" data-code="Space"         style="left:267px; top:390px; width:280px; height:52px"></i>
    <i class="k" data-code="AltRight"      style="left:549px; top:390px; width:48px;  height:52px"></i>
    <i class="k" data-code="ContextMenu"   style="left:614px; top:390px; width:58px;  height:52px"></i>
    <i class="k" data-code="ControlRight"  style="left:674px; top:390px; width:76px;  height:52px"></i>
    <i class="k" data-code="ArrowLeft"     style="left:780px; top:390px; width:48px;  height:52px"></i>
    <i class="k" data-code="ArrowDown"     style="left:830px; top:390px; width:48px;  height:52px"></i>
    <i class="k" data-code="ArrowRight"    style="left:880px; top:390px; width:48px;  height:52px"></i>
  </div>
</div>

</body>
</html>
```

Как пользоваться:

- **Оверлей поверх картинки:** раскомментируйте `<img src="keyboard.png">` — слой `.kb-overlay` ляжет поверх, фон у него полностью прозрачный, видны только тонкие рамки клавиш.
- **Настройка рамки:** цвет и толщина меняются в `--kb-border` (и в правиле `.k { border: ... }`), скругление — `--kb-radius`.
- **Клик по клавишам:** сейчас `pointer-events:none`, чтобы оверлей не мешал кликам. Если нужно ловить клики/наведение — уберите эту строку.
- **Подсветка клавиш:** у каждой клавиши есть `data-code` (стандартные коды как у `KeyboardEvent.code`), поэтому легко подсвечивать нажатия: `document.querySelector('[data-code="KeyF"]').classList.add('on')`.
- **Масштаб:** если нужно другого размера — оберните `.kb-wrap` в контейнер с `transform: scale(0.8); transform-origin: top left;`.

Если нужно, могу сделать вариант в процентах (резиновый, тянется под любой размер картинки) или добавить подписи клавиш внутри рамок.

Хорошо, добавил раскраску зон печати: у каждой клавиши теперь цветная граница **и** цветные символы (латиница + кириллица, как на картинке). Цвет = палец/зона:

- 🟠 оранжевый — левый мизинец, 🟣 лавандовый — левый безымянный, 🟡 жёлтый — левый средний, 🟢 зелёный — левый указательный
- 🟤 бежевый — правый указательный, 💜 фиолетовый — правый средний, 🟧 песочный — правый безымянный, 🔵 голубой — правый мизинец
- большие пальцы: AltLeft — синий, AltRight — фиолетовый, пробел — градиент синий→фиолетовый

```html
<!DOCTYPE html>
<html lang="ru">
<head>
<meta charset="utf-8">
<title>Keyboard overlay — zones</title>
<style>
  body{ margin:0; background:transparent; }
  .kb-wrap{ position:relative; width:1160px; height:600px; }
  .kb-wrap > img{ position:absolute; inset:0; width:100%; height:100%; display:block; }
  .kb-overlay{ position:absolute; inset:0; background:transparent; pointer-events:none;
               font:11px/1.1 system-ui, sans-serif; }

  /* зоны: --c = цвет границы и символов */
  .z1{--c:#d2691e} /* левый мизинец   */
  .z2{--c:#c7aee2} /* левый безымянный*/
  .z3{--c:#ddd789} /* левый средний   */
  .z4{--c:#8cc41e} /* левый указательный*/
  .z5{--c:#e8cfa8} /* правый указательный*/
  .z6{--c:#b48ce2} /* правый средний  */
  .z7{--c:#ef9430} /* правый безымянный*/
  .z8{--c:#a6cedc} /* правый мизинец  */
  .ztl{--c:#5b78e0}/* большой левый   */
  .ztr{--c:#6c2fc0}/* большой правый  */
  .zn{--c:#b9bec7} /* вне зон (F-ряд, стрелки…) */

  .k{
    position:absolute; box-sizing:border-box; background:transparent;
    border:1px solid var(--c); border-radius:6px; color:var(--c);
    display:flex; flex-direction:column; align-items:center; justify-content:center;
    gap:1px; text-align:center; font-style:normal;
  }
  .k .s{ font-size:9px; opacity:.85; }
  /* пробел: градиентная рамка (фон прозрачный) */
  .k.sp{ border-width:1px; border-style:solid;
         border-image:linear-gradient(90deg,#5b78e0,#6c2fc0) 1; border-radius:0; }
</style>
</head>
<body>
<div class="kb-wrap">
  <!-- <img src="keyboard.png" alt=""> -->
  <div class="kb-overlay">

    <!-- Ряд F -->
    <i class="k zn" style="left:60px;top:60px;width:55px;height:44px">Esc</i>
    <i class="k zn" style="left:125px;top:60px;width:45px;height:44px">F1</i>
    <i class="k zn" style="left:172px;top:60px;width:45px;height:44px">F2</i>
    <i class="k zn" style="left:220px;top:60px;width:45px;height:44px">F3</i>
    <i class="k zn" style="left:267px;top:60px;width:45px;height:44px">F4</i>
    <i class="k zn" style="left:322px;top:60px;width:45px;height:44px">F5</i>
    <i class="k zn" style="left:369px;top:60px;width:45px;height:44px">F6</i>
    <i class="k zn" style="left:417px;top:60px;width:45px;height:44px">F7</i>
    <i class="k zn" style="left:464px;top:60px;width:45px;height:44px">F8</i>
    <i class="k zn" style="left:519px;top:60px;width:45px;height:44px">F9</i>
    <i class="k zn" style="left:566px;top:60px;width:45px;height:44px">F10</i>
    <i class="k zn" style="left:614px;top:60px;width:45px;height:44px">F11</i>
    <i class="k zn" style="left:661px;top:60px;width:45px;height:44px">F12</i>
    <i class="k zn" style="left:716px;top:60px;width:50px;height:44px"><span class="s">PrtSc</span></i>
    <i class="k zn" style="left:768px;top:60px;width:50px;height:44px"><span class="s">ScrLk</span></i>
    <i class="k zn" style="left:833px;top:60px;width:50px;height:44px"><span class="s">Pause</span></i>
    <i class="k zn" style="left:885px;top:60px;width:50px;height:44px"><span class="s">Insert</span></i>

    <!-- Ряд цифр -->
    <i class="k z1" style="left:60px;top:127px;width:45px;height:52px">~<span class="s">`</span></i>
    <i class="k z2" style="left:107px;top:127px;width:45px;height:52px">!<span class="s">1</span></i>
    <i class="k z2" style="left:154px;top:127px;width:45px;height:52px">@<span class="s">2</span></i>
    <i class="k z3" style="left:201px;top:127px;width:45px;height:52px">#<span class="s">3</span></i>
    <i class="k z3" style="left:248px;top:127px;width:45px;height:52px">$<span class="s">4</span></i>
    <i class="k z4" style="left:295px;top:127px;width:45px;height:52px">%<span class="s">5</span></i>
    <i class="k z4" style="left:342px;top:127px;width:45px;height:52px">^<span class="s">6</span></i>
    <i class="k z5" style="left:389px;top:127px;width:45px;height:52px">&amp;<span class="s">7</span></i>
    <i class="k z6" style="left:436px;top:127px;width:45px;height:52px">*<span class="s">8</span></i>
    <i class="k z6" style="left:483px;top:127px;width:45px;height:52px">(<span class="s">9</span></i>
    <i class="k z7" style="left:530px;top:127px;width:45px;height:52px">)<span class="s">0</span></i>
    <i class="k z8" style="left:577px;top:127px;width:45px;height:52px">—<span class="s">-</span></i>
    <i class="k z8" style="left:624px;top:127px;width:45px;height:52px">+<span class="s">=</span></i>
    <i class="k z8" style="left:716px;top:127px;width:102px;height:52px"><span class="s">Backspace</span></i>
    <i class="k zn" style="left:833px;top:127px;width:50px;height:52px"><span class="s">Home</span></i>
    <i class="k zn" style="left:885px;top:127px;width:50px;height:52px"><span class="s">End</span></i>

    <!-- Ряд Tab -->
    <i class="k z1" style="left:60px;top:195px;width:67px;height:52px"><span class="s">Tab</span></i>
    <i class="k z1" style="left:132px;top:195px;width:45px;height:52px">Q<span class="s">й</span></i>
    <i class="k z2" style="left:179px;top:195px;width:45px;height:52px">W<span class="s">ц</span></i>
    <i class="k z3" style="left:226px;top:195px;width:45px;height:52px">E<span class="s">у</span></i>
    <i class="k z4" style="left:273px;top:195px;width:45px;height:52px">R<span class="s">к</span></i>
    <i class="k z4" style="left:320px;top:195px;width:45px;height:52px">T<span class="s">е</span></i>
    <i class="k z5" style="left:367px;top:195px;width:45px;height:52px">Y<span class="s">н</span></i>
    <i class="k z5" style="left:414px;top:195px;width:45px;height:52px">U<span class="s">г</span></i>
    <i class="k z6" style="left:461px;top:195px;width:45px;height:52px">I<span class="s">ш</span></i>
    <i class="k z7" style="left:508px;top:195px;width:45px;height:52px">O<span class="s">щ</span></i>
    <i class="k z8" style="left:555px;top:195px;width:45px;height:52px">P<span class="s">з</span></i>
    <i class="k z8" style="left:602px;top:195px;width:45px;height:52px">[<span class="s">х</span></i>
    <i class="k z8" style="left:649px;top:195px;width:45px;height:52px">]<span class="s">ъ</span></i>
    <i class="k z8" style="left:741px;top:195px;width:77px;height:52px">\<span class="s">/</span></i>
    <i class="k zn" style="left:833px;top:195px;width:50px;height:118px"><span class="s">Delete</span></i>
    <i class="k zn" style="left:885px;top:195px;width:50px;height:52px"><span class="s">PgUp</span></i>

    <!-- Ряд Caps -->
    <i class="k z1" style="left:60px;top:263px;width:82px;height:52px"><span class="s">CapsLock</span></i>
    <i class="k z1" style="left:147px;top:263px;width:45px;height:52px">A<span class="s">ф</span></i>
    <i class="k z2" style="left:194px;top:263px;width:45px;height:52px">S<span class="s">ы</span></i>
    <i class="k z3" style="left:241px;top:263px;width:45px;height:52px">D<span class="s">в</span></i>
    <i class="k z4" style="left:288px;top:263px;width:45px;height:52px">F<span class="s">а</span></i>
    <i class="k z4" style="left:335px;top:263px;width:45px;height:52px">G<span class="s">п</span></i>
    <i class="k z5" style="left:382px;top:263px;width:45px;height:52px">H<span class="s">р</span></i>
    <i class="k z5" style="left:429px;top:263px;width:45px;height:52px">J<span class="s">о</span></i>
    <i class="k z6" style="left:476px;top:263px;width:45px;height:52px">K<span class="s">л</span></i>
    <i class="k z7" style="left:523px;top:263px;width:45px;height:52px">L<span class="s">д</span></i>
    <i class="k z8" style="left:570px;top:263px;width:45px;height:52px">;<span class="s">ж</span></i>
    <i class="k z8" style="left:617px;top:263px;width:45px;height:52px">'<span class="s">э</span></i>
    <i class="k z8" style="left:700px;top:263px;width:118px;height:52px"><span class="s">Enter</span></i>
    <i class="k zn" style="left:885px;top:263px;width:50px;height:52px"><span class="s">PgDn</span></i>

    <!-- Ряд Shift -->
    <i class="k z1" style="left:60px;top:327px;width:105px;height:52px"><span class="s">Shift</span></i>
    <i class="k z2" style="left:170px;top:327px;width:45px;height:52px">Z<span class="s">я</span></i>
    <i class="k z3" style="left:217px;top:327px;width:45px;height:52px">X<span class="s">ч</span></i>
    <i class="k z4" style="left:264px;top:327px;width:45px;height:52px">C<span class="s">с</span></i>
    <i class="k z4" style="left:311px;top:327px;width:45px;height:52px">V<span class="s">м</span></i>
    <i class="k z4" style="left:358px;top:327px;width:45px;height:52px">B<span class="s">и</span></i>
    <i class="k z5" style="left:405px;top:327px;width:45px;height:52px">N<span class="s">т</span></i>
    <i class="k z5" style="left:452px;top:327px;width:45px;height:52px">M<span class="s">ь</span></i>
    <i class="k z6" style="left:499px;top:327px;width:45px;height:52px">,<span class="s">б</span></i>
    <i class="k z7" style="left:546px;top:327px;width:45px;height:52px">.<span class="s">ю</span></i>
    <i class="k z8" style="left:593px;top:327px;width:45px;height:52px">/<span class="s">?</span></i>
    <i class="k z8" style="left:676px;top:327px;width:142px;height:52px"><span class="s">Shift</span></i>
    <i class="k zn" style="left:833px;top:327px;width:50px;height:52px">↑</i>

    <!-- Нижний ряд -->
    <i class="k z1"  style="left:60px;top:390px;width:45px;height:52px"><span class="s">Fn</span></i>
    <i class="k z1"  style="left:107px;top:390px;width:58px;height:52px"><span class="s">Ctrl</span></i>
    <i class="k zn"  style="left:167px;top:390px;width:48px;height:52px"><span class="s">Win</span></i>
    <i class="k ztl" style="left:217px;top:390px;width:48px;height:52px"><span class="s">Alt</span></i>
    <i class="k sp"  style="left:267px;top:390px;width:280px;height:52px"></i>
    <i class="k ztr" style="left:549px;top:390px;width:48px;height:52px"><span class="s">Alt</span></i>
    <i class="k zn"  style="left:614px;top:390px;width:58px;height:52px"><span class="s">Menu</span></i>
    <i class="k zn"  style="left:674px;top:390px;width:76px;height:52px"><span class="s">Ctrl</span></i>
    <i class="k zn"  style="left:780px;top:390px;width:48px;height:52px">←</i>
    <i class="k zn"  style="left:830px;top:390px;width:48px;height:52px">↓</i>
    <i class="k zn"  style="left:880px;top:390px;width:48px;height:52px">→</i>
  </div>
</div>
</body>
</html>
```

Замечания:

- Фон по‑прежнему полностью прозрачный — заливки у клавиш нет, только цветная рамка 1px и цветные символы (`color` = цвет зоны).
- Все цвета зон заданы переменными `--c` в классах `.z1…`.z8 / `.ztl / .ztr` — можно перекрасить всю зону, поменяв одну строку.
- Клавиши вне зон печати (F‑ряд, PrtSc, Home/End, стрелки, Win, Menu, правый Ctrl) — нейтрально‑серые (`.zn`), как на картинке без кружков.
- Если символы покажутся бледноватыми на светлом фоне — добавьте `.k{ text-shadow:0 0 1px currentColor; }` или замените цвета на чуть темнее в переменных.

Если нужно, могу сделать вариант, где цветные только рамки, а символы чёрные, или добавить лёгкую полупрозрачную заливку зон (например, `background: color-mix(in srgb, var(--c) 15%, transparent)`).