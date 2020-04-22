# 样式

## 安装

```bash
npm install tailwindcss --save-dev
./node_modules/.bin/tailwind init
```
# Container

A component for fixing an element's width to the current breakpoint. 

class|Properties
---|---:
.container |width: 100%;

# Box Sizing

Utilities for controlling how the browser should calculate an element's total size. 

class|Properties
.box-border |box-sizing: border-box;
.box-content |box-sizing: content-box;

## Display

Utilities for controlling the display box type of an element.

class|Properties
---|---:
.hidden |display: none;
.block |display: block;
.inline-block |display: inline-block;
.inline |display: inline;
.flex |display: flex;
.inline-flex |display: inline-flex;
.grid |display: grid;
.table |display: table;
.table-caption |display: table-caption;
.table-cell |display: table-cell;
.table-column |display: table-column;
.table-column-group |display: table-column-group;
.table-footer-group |display: table-footer-group;
.table-header-group |display: table-header-group;
.table-row-group |display: table-row-group;
.table-row |display: table-row;

# Floats

Utilities for controlling the wrapping of content around an element.

class|Properties
.float-right |float: right;
.float-left |float: left;
.float-none |float: none;
.clearfix |&::after {content: ""; display: table; clear: both;}

# Clear

Utilities for controlling the wrapping of content around an element.

class|Properties
.clear-left |clear: left;
.clear-right |clear: right;
.clear-both |clear: both;
.clear-none |clear: none;

## Object Fit

Utilities for controlling how a replaced element's content should be resized.

class|Properties
---|---:
.object-contain |object-fit: contain;
.object-cover |object-fit: cover;
.object-fill |object-fit: fill;
.object-none |object-fit: none;
.object-scale-down |object-fit: scale-down;

## Object Position

Utilities for controlling how a replaced element's content should be positioned within its container.

class|Properties
---|---:
.object-bottom |object-position: bottom;
.object-center |object-position: center;
.object-left |object-position: left;
.object-left-bottom |object-position: left bottom;
.object-left-top |object-position: left top;
.object-right |object-position: right;
.object-right-bottom |object-position: right bottom;
.object-right-top |object-position: right top;
.object-top |object-position: top;

## Overflow

Utilities for controlling how an element handles content that is too large for the container.

class|Properties
---|---:
.overflow-auto |overflow: auto;
.overflow-hidden |overflow: hidden;
.overflow-visible |overflow: visible;
.overflow-scroll |overflow: scroll;
.overflow-x-auto |overflow-x: auto;
.overflow-y-auto |overflow-y: auto;
.overflow-x-hidden |overflow-x: hidden;
.overflow-y-hidden |overflow-y: hidden;
.overflow-x-visible |overflow-x: visible;
.overflow-y-visible |overflow-y: visible;
.overflow-x-scroll |overflow-x: scroll;
.overflow-y-scroll |overflow-y: scroll;
.scrolling-touch |-webkit-overflow-scrolling: touch;
.scrolling-auto |-webkit-overflow-scrolling: auto;

## Position

Utilities for controlling how an element is positioned in the DOM.

class|Properties
---|---:
.static |position: static;
.fixed |position: fixed;
.absolute |position: absolute;
.relative |position: relative;
.sticky |position: sticky;

## Top / Right / Bottom / Left

Utilities for controlling the placement of positioned elements.

class|Properties
---|---:
.inset-0 |top: 0; right: 0; bottom: 0; left: 0;
.inset-y-0 |top: 0; bottom: 0;
.inset-x-0 |right: 0; left: 0;
.top-0 |top: 0;
.right-0 |right: 0;
.bottom-0 |bottom: 0;
.left-0 |left: 0;
.inset-auto |top: auto; right: auto; bottom: auto; left: auto;
.inset-y-auto |top: auto; bottom: auto;
.inset-x-auto |left: auto; right: auto;
.top-auto |top: auto;
.bottom-auto |bottom: auto;
.left-auto |left: auto;
.right-auto |right: auto;

## Visibility

Utilities for controlling the visibility of an element.

class|Properties
---|---:
.visible |visibility: visible;
.invisible |visibility: hidden;

## Z-Index

class|Properties
---|---:
.z-0 |z-index: 0;
.z-10 |z-index: 10;
.z-20 |z-index: 20;
.z-30 |z-index: 30;
.z-40 |z-index: 40;
.z-50 |z-index: 50;
.z-auto |z-index: auto;

## Font

### Text Decoration

Utilities for controlling the decoration of text.

class|Properties
---|---:
.underline |text-decoration: underline;
.line-through |text-decoration: line-through;
.no-underline |text-decoration: none;

### Text Transform

Utilities for controlling the transformation of text.

class|Properties
---|---:
.uppercase |text-transform: uppercase;
.lowercase |text-transform: lowercase;
.capitalize |text-transform: capitalize;
.normal-case |text-transform: none;

### Vertical Alignment

Utilities for controlling the vertical alignment of an inline or table-cell box.

class|Properties
---|---:
.align-baseline |vertical-align: baseline;
.align-top |vertical-align: top;
.align-middle |vertical-align: middle;
.align-bottom |vertical-align: bottom;
.align-text-top |vertical-align: text-top;
.align-text-bottom |vertical-align: text-bottom;

### Text Color

Utilities for controlling the text color of an element.

```js
.text-transparent
.text-black
.text-white

// 100 -> 900
.text-gray-
.text-red-
.text-orange-
.text-yellow-
.text-green-
.text-teal-
.text-blue-
.text-indigo-
.text-purple-
.text-pink-
```

### Text Alignment

Utilities for controlling the alignment of text.

class|Properties
---|---:
.text-left |text-align: left;
.text-center |text-align: center;
.text-right |text-align: right;
.text-justify |text-align: justify;

### List Style Position

Utilities for controlling the position of bullets/numbers in lists.

class|Properties
---|---:
.list-inside |list-style-position: inside;
.list-outside |list-style-position: outside;

### List Style Type

Utilities for controlling the bullet/number style of a list.

class|Properties
---|---:
.list-none |list-style-type: none;
.list-disc |list-style-type: disc;
.list-decimal |list-style-type: decimal;

### Line Height

Utilities for controlling the leading (line height) of an element.

class|Properties
---|---:
.leading-none |line-height: 1;
.leading-tight |line-height: 1.25;
.leading-snug |line-height: 1.375;
.leading-normal |line-height: 1.5;
.leading-relaxed |line-height: 1.625;
.leading-loose |line-height: 2;

### Font Weight

class|Properties
---|---:
.font-hairline |font-weight: 100;
.font-thin |font-weight: 200;
.font-light |font-weight: 300;
.font-normal |font-weight: 400;
.font-medium |font-weight: 500;
.font-semibold |font-weight: 600;
.font-bold |font-weight: 700;
.font-extrabold |font-weight: 800;
.font-black |font-weight: 900;

### Font Style

Utilities for controlling the style of text.

class|Properties
---|---:
.italic |font-style: italic;
.not-italic |font-style: normal;

### Font Size

class|Properties
---|---:
.text-xs |font-size: .75rem;
.text-sm |font-size: .875rem;
.text-base |font-size: 1rem;
.text-lg |font-size: 1.125rem;
.text-xl |font-size: 1.25rem;
.text-2xl |font-size: 1.5rem;
.text-3xl |font-size: 1.875rem;
.text-4xl |font-size: 2.25rem;
.text-5xl |font-size: 3rem;
.text-6xl |font-size: 4rem;

## Backgrounds

### Background Size

Utilities for controlling the background size of an element's background image.

class|Properties
---|---:
.bg-auto |background-size: auto;
.bg-cover |background-size: cover;
.bg-contain |background-size: contain;

### Background Repeat

Utilities for controlling the repetition of an element's background image.

class|Properties
---|---:
.bg-repeat |background-repeat: repeat;
.bg-no-repeat |background-repeat: no-repeat;
.bg-repeat-x |background-repeat: repeat-x;
.bg-repeat-y |background-repeat: repeat-y;
.bg-repeat-round |background-repeat: round;
.bg-repeat-space |background-repeat: space;

### Background Position

Utilities for controlling the position of an element's background image.

class|Properties
---|---:
.bg-bottom |background-position: bottom;
.bg-center |background-position: center;
.bg-left |background-position: left;
.bg-left-bottom |background-position: left bottom;
.bg-left-top |background-position: left top;
.bg-right |background-position: right;
.bg-right-bottom |background-position: right bottom;
.bg-right-top |background-position: right top;
.bg-top |background-position: top;

### Background Attachment

Utilities for controlling how a background image behaves when scrolling.

class|Properties
---|---:
.bg-fixed |background-attachment: fixed;
.bg-local |background-attachment: local;
.bg-scroll |background-attachment: scroll;

### Background Color

```js
.bg-transparent
.bg-black
.bg-white
// 100 -> 900
.bg-gray-
.bg-red-
.bg-orange-
.bg-yellow-
.bg-green-
.bg-teal-
.bg-blue-
.bg-indigo-
.bg-purple-
.bg-pink-
```

## Border

### Border Radius

Utilities for controlling the border radius of an element.

class|Properties
---|---:
.rounded-none |border-radius: 0;
.rounded-sm |border-radius: .125rem;
.rounded |border-radius: .25rem;
.rounded-lg |border-radius: .5rem;
.rounded-full |border-radius: 9999px;
.rounded-t-none |border-top-left-radius: 0; border-top-right-radius: 0;
.rounded-r-none |border-top-right-radius: 0; border-bottom-right-radius: 0;
.rounded-b-none |border-bottom-right-radius: 0; border-bottom-left-radius: 0;
.rounded-l-none |border-top-left-radius: 0; border-bottom-left-radius: 0;
.rounded-t-sm |border-top-left-radius: .125rem; border-top-right-radius: .125rem;
.rounded-r-sm |border-top-right-radius: .125rem; border-bottom-right-radius: .125rem;
.rounded-b-sm |border-bottom-right-radius: .125rem; border-bottom-left-radius: .125rem;
.rounded-l-sm |border-top-left-radius: .125rem; border-bottom-left-radius: .125rem;
.rounded-t |border-top-left-radius: .25rem; border-top-right-radius: .25rem;
.rounded-r |border-top-right-radius: .25rem; border-bottom-right-radius: .25rem;
.rounded-b |border-bottom-right-radius: .25rem; border-bottom-left-radius: .25rem;
.rounded-l |border-top-left-radius: .25rem; border-bottom-left-radius: .25rem;
.rounded-t-lg |border-top-left-radius: .5rem; border-top-right-radius: .5rem;
.rounded-r-lg |border-top-right-radius: .5rem; border-bottom-right-radius: .5rem;
.rounded-b-lg |border-bottom-right-radius: .5rem; border-bottom-left-radius: .5rem;
.rounded-l-lg |border-top-left-radius: .5rem; border-bottom-left-radius: .5rem;
.rounded-t-full |border-top-left-radius: 9999px; border-top-right-radius: 9999px;
.rounded-r-full |border-top-right-radius: 9999px; border-bottom-right-radius: 9999px;
.rounded-b-full |border-bottom-right-radius: 9999px; border-bottom-left-radius: 9999px;
.rounded-l-full |border-top-left-radius: 9999px; border-bottom-left-radius: 9999px;
.rounded-tl-none |border-top-left-radius: 0;
.rounded-tr-none |border-top-right-radius: 0;
.rounded-br-none |border-bottom-right-radius: 0;
.rounded-bl-none |border-bottom-left-radius: 0;
.rounded-tl-sm |border-top-left-radius: .125rem;
.rounded-tr-sm |border-top-right-radius: .125rem;
.rounded-br-sm |border-bottom-right-radius: .125rem;
.rounded-bl-sm |border-bottom-left-radius: .125rem;
.rounded-tl |border-top-left-radius: .25rem;
.rounded-tr |border-top-right-radius: .25rem;
.rounded-br |border-bottom-right-radius: .25rem;
.rounded-bl |border-bottom-left-radius: .25rem;
.rounded-tl-lg |border-top-left-radius: .5rem;
.rounded-tr-lg |border-top-right-radius: .5rem;
.rounded-br-lg |border-bottom-right-radius: .5rem;
.rounded-bl-lg |border-bottom-left-radius: .5rem;
.rounded-tl-full |border-top-left-radius: 9999px;
.rounded-tr-full |border-top-right-radius: 9999px;
.rounded-br-full |border-bottom-right-radius: 9999px;
.rounded-bl-full |border-bottom-left-radius: 9999px;

### Border Width

Utilities for controlling the width an element's borders.

class|Properties
---|---:
.border |border-width: 1px;
.border-0 |border-width: 0;
.border-2 |border-width: 2px;
.border-4 |border-width: 4px;
.border-8 |border-width: 8px;
.border-t |border-top-width: 1px;
.border-r |border-right-width: 1px;
.border-b |border-bottom-width: 1px;
.border-l |border-left-width: 1px;
.border-t-0 |border-top-width: 0;
.border-r-0 |border-right-width: 0;
.border-b-0 |border-bottom-width: 0;
.border-l-0 |border-left-width: 0;
.border-t-2 |border-top-width: 2px;
.border-r-2 |border-right-width: 2px;
.border-b-2 |border-bottom-width: 2px;
.border-l-2 |border-left-width: 2px;
.border-t-4 |border-top-width: 4px;
.border-r-4 |border-right-width: 4px;
.border-b-4 |border-bottom-width: 4px;
.border-l-4 |border-left-width: 4px;
.border-t-8 |border-top-width: 8px;
.border-r-8 |border-right-width: 8px;
.border-b-8 |border-bottom-width: 8px;
.border-l-8 |border-left-width: 8px;

### Border Style

Utilities for controlling the style of an element's borders.

class|Properties
---|---:
.border-solid |border-style: solid;
.border-dashed |border-style: dashed;
.border-dotted |border-style: dotted;
.border-none |border-style: none;

### Border Color

```js
.border-transparent
.border-black
.border-white
// 100 -> 900
.border-gray-
.border-red-
.border-orange-
.border-yellow-
.border-green-
.border-teal-
.border-blue-
.border-indigo-
.border-purple-
.border-pink-
```

# Flex Direction

Utilities for controlling the direction of flex items.

class|Properties
---|---:
.flex-row |flex-direction: row;
.flex-row-reverse |flex-direction: row-reverse;
.flex-col |flex-direction: column;
.flex-col-reverse |flex-direction: column-reverse;

# Flex Wrap

Utilities for controlling how flex items wrap.

class|Properties
---|---:
.flex-no-wrap |flex-wrap: nowrap;
.flex-wrap |flex-wrap: wrap;
.flex-wrap-reverse |flex-wrap: wrap-reverse;

### Align Items

Utilities for controlling how flex items are positioned along a container's cross axis.

class|Properties
---|---:
.items-stretch |align-items: stretch;
.items-start |align-items: flex-start;
.items-center |align-items: center;
.items-end |align-items: flex-end;
.items-baseline |align-items: baseline;

### Align Content

Utilities for controlling how lines are positioned in multi-line flex containers.

class|Properties
---|---:
.content-start |align-content: flex-start;
.content-center |align-content: center;
.content-end |align-content: flex-end;
.content-between |align-content: space-between;
.content-around |align-content: space-around;

### Align Self

Utilities for controlling how an individual flex item is positioned along its container's cross axis.

class|Properties
---|---:
.self-auto |align-self: auto;
.self-start |align-self: flex-start;
.self-center |align-self: center;
.self-end |align-self: flex-end;
.self-stretch |align-self: stretch;

### Justify Content

Utilities for controlling how flex items are positioned along a container's main axis.

class|Properties
---|---:
.justify-start |justify-content: flex-start;
.justify-center |justify-content: center;
.justify-end |justify-content: flex-end;
.justify-between |justify-content: space-between;
.justify-around |justify-content: space-around;

### Flex

Utilities for controlling how flex items both grow and shrink.

class|Properties
---|---:
.flex-initial |flex: 0 1 auto;
.flex-1 |flex: 1 1 0%;
.flex-auto |flex: 1 1 auto;
.flex-none |flex: none;

### Flex Grow

Utilities for controlling how flex items grow.

class|Properties
---|---:
.flex-grow |flex-grow: 1;
.flex-grow-0 |flex-grow: 0;

### Flex Shrink

Utilities for controlling how flex items shrink.

class|Properties
---|---:
.flex-shrink |flex-shrink: 1;
.flex-shrink-0 |flex-shrink: 0;

### Order

Utilities for controlling the order of flex items.

class|Properties
---|---:
.order-first |order: -1;
.order-last |order: 999;
.order-none |order: 0;
.order-1 |order: 1;
.order-2 |order: 2;
.order-3 |order: 3;
.order-4 |order: 4;
.order-5 |order: 5;
.order-6 |order: 6;
.order-7 |order: 7;
.order-8 |order: 8;
.order-9 |order: 9;
.order-10 |order: 10;
.order-11 |order: 11;
.order-12 |order: 12;

# Grid Template Columns

Utilities for specifying the columns in a grid layout.

class|Properties
---|---:
.grid-cols-1 |grid-template-columns: repeat(1, minmax(0, 1fr));
.grid-cols-2 |grid-template-columns: repeat(2, minmax(0, 1fr));
.grid-cols-3 |grid-template-columns: repeat(3, minmax(0, 1fr));
.grid-cols-4 |grid-template-columns: repeat(4, minmax(0, 1fr));
.grid-cols-5 |grid-template-columns: repeat(5, minmax(0, 1fr));
.grid-cols-6 |grid-template-columns: repeat(6, minmax(0, 1fr));
.grid-cols-7 |grid-template-columns: repeat(7, minmax(0, 1fr));
.grid-cols-8 |grid-template-columns: repeat(8, minmax(0, 1fr));
.grid-cols-9 |grid-template-columns: repeat(9, minmax(0, 1fr));
.grid-cols-10 |grid-template-columns: repeat(10, minmax(0, 1fr));
.grid-cols-11 |grid-template-columns: repeat(11, minmax(0, 1fr));
.grid-cols-12 |grid-template-columns: repeat(12, minmax(0, 1fr));
.grid-cols-none |grid-template-columns: none;

# Grid Column Start / End

Utilities for controlling how elements are sized and placed across grid columns.

class|Properties
---|---:
.col-auto |grid-column: auto;
.col-span-1 |grid-column: span 1 / span 1;
.col-span-2 |grid-column: span 2 / span 2;
.col-span-3 |grid-column: span 3 / span 3;
.col-span-4 |grid-column: span 4 / span 4;
.col-span-5 |grid-column: span 5 / span 5;
.col-span-6 |grid-column: span 6 / span 6;
.col-span-7 |grid-column: span 7 / span 7;
.col-span-8 |grid-column: span 8 / span 8;
.col-span-9 |grid-column: span 9 / span 9;
.col-span-10 |grid-column: span 10 / span 10;
.col-span-11 |grid-column: span 11 / span 11;
.col-span-12 |grid-column: span 12 / span 12;
.col-start-1 |grid-column-start: 1;
.col-start-2 |grid-column-start: 2;
.col-start-3 |grid-column-start: 3;
.col-start-4 |grid-column-start: 4;
.col-start-5 |grid-column-start: 5;
.col-start-6 |grid-column-start: 6;
.col-start-7 |grid-column-start: 7;
.col-start-8 |grid-column-start: 8;
.col-start-9 |grid-column-start: 9;
.col-start-10 |grid-column-start: 10;
.col-start-11 |grid-column-start: 11;

# Grid Template Rows

Utilities for specifying the rows in a grid layout.

class|Properties
---|---:
.grid-rows-1 |grid-template-rows: repeat(1, minmax(0, 1fr));
.grid-rows-2 |grid-template-rows: repeat(2, minmax(0, 1fr));
.grid-rows-3 |grid-template-rows: repeat(3, minmax(0, 1fr));
.grid-rows-4 |grid-template-rows: repeat(4, minmax(0, 1fr));
.grid-rows-5 |grid-template-rows: repeat(5, minmax(0, 1fr));
.grid-rows-6 |grid-template-rows: repeat(6, minmax(0, 1fr));
.grid-rows-none |grid-template-rows: none;

# Grid Row Start / End

Utilities for controlling how elements are sized and placed across grid rows.

class|Properties
---|---:
.row-auto |grid-row: auto;
.row-span-1 |grid-row: span 1 / span 1;
.row-span-2 |grid-row: span 2 / span 2;
.row-span-3 |grid-row: span 3 / span 3;
.row-span-4 |grid-row: span 4 / span 4;
.row-span-5 |grid-row: span 5 / span 5;
.row-span-6 |grid-row: span 6 / span 6;
.row-start-1 |grid-row-start: 1;
.row-start-2 |grid-row-start: 2;
.row-start-3 |grid-row-start: 3;
.row-start-4 |grid-row-start: 4;
.row-start-5 |grid-row-start: 5;
.row-start-6 |grid-row-start: 6;
.row-start-7 |grid-row-start: 7;
.row-start-auto |grid-row-start: auto;
.row-end-1 |grid-row-end: 1;
.row-end-2 |grid-row-end: 2;
.row-end-3 |grid-row-end: 3;
.row-end-4 |grid-row-end: 4;
.row-end-5 |grid-row-end: 5;
.row-end-6 |grid-row-end: 6;
.row-end-7 |grid-row-end: 7;
.row-end-auto |grid-row-end: auto;

# Gap

Utilities for controlling gutters between grid rows and columns.

class|Properties
---|---:
.gap-0 |gap: 0;
.gap-1 |gap: 0.25rem;
.gap-2 |gap: 0.5rem;
.gap-3 |gap: 0.75rem;
.gap-4 |gap: 1rem;
.gap-5 |gap: 1.25rem;
.gap-6 	gap: 1.5rem;
.gap-8 |gap: 2rem;
.gap-10 |gap: 2.5rem;
.gap-12 |gap: 3rem;
.gap-16 |gap: 4rem;
.gap-20 |gap: 5rem;
.gap-24 |gap: 6rem;
.gap-32 |gap: 8rem;
.gap-40 |gap: 10rem;
.gap-48 |gap: 12rem;
.gap-56 |gap: 14rem;
.gap-64 |gap: 16rem;
.gap-px |gap: 1px;
.row-gap-0 |row-gap: 0;
.row-gap-1 |row-gap: 0.25rem;
.row-gap-2 |row-gap: 0.5rem;
.row-gap-3 |row-gap: 0.75rem;
.row-gap-4 |row-gap: 1rem;
.row-gap-5 |row-gap: 1.25rem;
.row-gap-6 |row-gap: 1.5rem;
.row-gap-8 |row-gap: 2rem;
.row-gap-10 |row-gap: 2.5rem;
.row-gap-12 |row-gap: 3rem;
.row-gap-16 |row-gap: 4rem;
.row-gap-20 |row-gap: 5rem;
.row-gap-24 |row-gap: 6rem;
.row-gap-32 |row-gap: 8rem;
.row-gap-40 |row-gap: 10rem;
.row-gap-48 |row-gap: 12rem;
.row-gap-56 |row-gap: 14rem;
.row-gap-64 |row-gap: 16rem;
.row-gap-px |row-gap: 1px;
.col-gap-0 |column-gap: 0;
.col-gap-1 |column-gap: 0.25rem;
.col-gap-2 |column-gap: 0.5rem;
.col-gap-3 |column-gap: 0.75rem;
.col-gap-4 |column-gap: 1rem;
.col-gap-5 |column-gap: 1.25rem;
.col-gap-6 |column-gap: 1.5rem;
.col-gap-8 |column-gap: 2rem;
.col-gap-10 |column-gap: 2.5rem;
.col-gap-12 |column-gap: 3rem;
.col-gap-16 |column-gap: 4rem;
.col-gap-20 |column-gap: 5rem;
.col-gap-24 |column-gap: 6rem;
.col-gap-32 |column-gap: 8rem;
.col-gap-40 |column-gap: 10rem;
.col-gap-48 |column-gap: 12rem;
.col-gap-56 |column-gap: 14rem;
.col-gap-64 |column-gap: 16rem;
.col-gap-px |column-gap: 1px;

# Grid Auto Flow

Utilities for controlling how elements in a grid are auto-placed.

class|Properties
---|---:
.grid-flow-row |grid-auto-flow: row;
.grid-flow-col |grid-auto-flow: column;
.grid-flow-row-dense |grid-auto-flow: row dense;
.grid-flow-col-dense |grid-auto-flow: column dense;

## Padding

class|Properties
---|---:
.p-0 |padding: 0;
.p-1 |padding: 0.25rem;
.p-2 |padding: 0.5rem;
.p-3 |padding: 0.75rem;
.p-4 |padding: 1rem;
.p-5 |padding: 1.25rem;
.p-6 |padding: 1.5rem;
.p-8 |padding: 2rem;
.p-10 |padding: 2.5rem;
.p-12 |padding: 3rem;
.p-16 |padding: 4rem;
.p-20 |padding: 5rem;
.p-24 |padding: 6rem;
.p-32 |padding: 8rem;
.p-40 |padding: 10rem;
.p-48 |padding: 12rem;
.p-56 |padding: 14rem;
.p-64 |padding: 16rem;
.p-px |padding: 1px;
.py-0 |padding-top: 0; padding-bottom: 0;
.py-1 |padding-top: 0.25rem; padding-bottom: 0.25rem;
.py-2 |padding-top: 0.5rem; padding-bottom: 0.5rem;
.py-3 |padding-top: 0.75rem; padding-bottom: 0.75rem;
.py-4 |padding-top: 1rem; padding-bottom: 1rem;
.py-5 |padding-top: 1.25rem; padding-bottom: 1.25rem;
.py-6 |padding-top: 1.5rem; padding-bottom: 1.5rem;
.py-8 |padding-top: 2rem; padding-bottom: 2rem;
.py-10 |padding-top: 2.5rem; padding-bottom: 2.5rem;
.py-12 |padding-top: 3rem; padding-bottom: 3rem;
.py-16 |padding-top: 4rem; padding-bottom: 4rem;
.py-20 |padding-top: 5rem; padding-bottom: 5rem;
.py-24 |padding-top: 6rem; padding-bottom: 6rem;
.py-32 |padding-top: 8rem; padding-bottom: 8rem;
.py-40 |padding-top: 10rem; padding-bottom: 10rem;
.py-48 |padding-top: 12rem; padding-bottom: 12rem;
.py-56 |padding-top: 14rem; padding-bottom: 14rem;
.py-64 |padding-top: 16rem; padding-bottom: 16rem;
.py-px |padding-top: 1px; padding-bottom: 1px;
.px-0 |padding-right: 0; padding-left: 0;
.px-1 |padding-right: 0.25rem; padding-left: 0.25rem;
.px-2 |padding-right: 0.5rem; padding-left: 0.5rem;
.px-3 |padding-right: 0.75rem; padding-left: 0.75rem;
.px-4 |padding-right: 1rem; padding-left: 1rem;
.px-5 |padding-right: 1.25rem; padding-left: 1.25rem;
.px-6 |padding-right: 1.5rem; padding-left: 1.5rem;
.px-8 |padding-right: 2rem; padding-left: 2rem;
.px-10 |padding-right: 2.5rem; padding-left: 2.5rem;
.px-12 |padding-right: 3rem; padding-left: 3rem;
.px-16 |padding-right: 4rem; padding-left: 4rem;
.px-20 |padding-right: 5rem; padding-left: 5rem;
.px-24 |padding-right: 6rem; padding-left: 6rem;
.px-32 |padding-right: 8rem; padding-left: 8rem;
.px-40 |padding-right: 10rem; padding-left: 10rem;
.px-48 |padding-right: 12rem; padding-left: 12rem;
.px-56 |padding-right: 14rem; padding-left: 14rem;
.px-64 |padding-right: 16rem; padding-left: 16rem;
.px-px |padding-right: 1px; padding-left: 1px;
.pt-0 |padding-top: 0;
.pt-1 |padding-top: 0.25rem;
.pt-2 |padding-top: 0.5rem;
.pt-3 |padding-top: 0.75rem;
.pt-4 |padding-top: 1rem;
.pt-5 |padding-top: 1.25rem;
.pt-6 |padding-top: 1.5rem;
.pt-8 |padding-top: 2rem;
.pt-10 |padding-top: 2.5rem;
.pt-12 |padding-top: 3rem;
.pt-16 |padding-top: 4rem;
.pt-20 |padding-top: 5rem;
.pt-24 |padding-top: 6rem;
.pt-32 |padding-top: 8rem;
.pt-40 |padding-top: 10rem;
.pt-48 |padding-top: 12rem;
.pt-56 |padding-top: 14rem;
.pt-64 |padding-top: 16rem;
.pt-px |padding-top: 1px;
.pr-0 |padding-right: 0;
.pr-1 |padding-right: 0.25rem;
.pr-2 |padding-right: 0.5rem;
.pr-3 |padding-right: 0.75rem;
.pr-4 |padding-right: 1rem;
.pr-5 |padding-right: 1.25rem;
.pr-6 |padding-right: 1.5rem;
.pr-8 |padding-right: 2rem;
.pr-10 |padding-right: 2.5rem;
.pr-12 |padding-right: 3rem;
.pr-16 |padding-right: 4rem;
.pr-20 |padding-right: 5rem;
.pr-24 |padding-right: 6rem;
.pr-32 |padding-right: 8rem;
.pr-40 |padding-right: 10rem;
.pr-48 |padding-right: 12rem;
.pr-56 |padding-right: 14rem;
.pr-64 |padding-right: 16rem;
.pr-px |padding-right: 1px;
.pb-0 |padding-bottom: 0;
.pb-1 |padding-bottom: 0.25rem;
.pb-2 |padding-bottom: 0.5rem;
.pb-3 |padding-bottom: 0.75rem;
.pb-4 |padding-bottom: 1rem;
.pb-5 |padding-bottom: 1.25rem;
.pb-6 |padding-bottom: 1.5rem;
.pb-8 |padding-bottom: 2rem;
.pb-10 |padding-bottom: 2.5rem;
.pb-12 |padding-bottom: 3rem;
.pb-16 |padding-bottom: 4rem;
.pb-20 |padding-bottom: 5rem;
.pb-24 |padding-bottom: 6rem;
.pb-32 |padding-bottom: 8rem;
.pb-40 |padding-bottom: 10rem;
.pb-48 |padding-bottom: 12rem;
.pb-56 |padding-bottom: 14rem;
.pb-64 |padding-bottom: 16rem;
.pb-px |padding-bottom: 1px;
.pl-0 |padding-left: 0;
.pl-1 |padding-left: 0.25rem;
.pl-2 |padding-left: 0.5rem;
.pl-3 |padding-left: 0.75rem;
.pl-4 |padding-left: 1rem;
.pl-5 |padding-left: 1.25rem;
.pl-6 |padding-left: 1.5rem;
.pl-8 |padding-left: 2rem;
.pl-10 |padding-left: 2.5rem;
.pl-12 |padding-left: 3rem;
.pl-16 |padding-left: 4rem;
.pl-20 |padding-left: 5rem;
.pl-24 |padding-left: 6rem;
.pl-32 |padding-left: 8rem;
.pl-40 |padding-left: 10rem;
.pl-48 |padding-left: 12rem;
.pl-56 |padding-left: 14rem;
.pl-64 |padding-left: 16rem;
.pl-px |padding-left: 1px;

## Margin

class|Properties
---|---:
.m-0 |margin: 0;
.m-1 |margin: 0.25rem;
.m-2 |margin: 0.5rem;
.m-3 |margin: 0.75rem;
.m-4 |margin: 1rem;
.m-5 |margin: 1.25rem;
.m-6 |margin: 1.5rem;
.m-8 |margin: 2rem;
.m-10 |margin: 2.5rem;
.m-12 |margin: 3rem;
.m-16 |margin: 4rem;
.m-20 |margin: 5rem;
.m-24 |margin: 6rem;
.m-32 |margin: 8rem;
.m-40 |margin: 10rem;
.m-48 |margin: 12rem;
.m-56 |margin: 14rem;
.m-64 |margin: 16rem;
.m-auto |margin: auto;
.m-px |margin: 1px;
.-m-1 |margin: -0.25rem;
.-m-2 |margin: -0.5rem;
.-m-3 |margin: -0.75rem;
.-m-4 |margin: -1rem;
.-m-5 |margin: -1.25rem;
.-m-6 |margin: -1.5rem;
.-m-8 |margin: -2rem;
.-m-10 |margin: -2.5rem;
.-m-12 |margin: -3rem;
.-m-16 |margin: -4rem;
.-m-20 |margin: -5rem;
.-m-24 |margin: -6rem;
.-m-32 |margin: -8rem;
.-m-40 |margin: -10rem;
.-m-48 |margin: -12rem;
.-m-56 |margin: -14rem;
.-m-64 |margin: -16rem;
.-m-px |margin: -1px;
.my-0 |margin-top: 0; margin-bottom: 0;
.my-1 |margin-top: 0.25rem; margin-bottom: 0.25rem;
.my-2 |margin-top: 0.5rem; margin-bottom: 0.5rem;
.my-3 |margin-top: 0.75rem; margin-bottom: 0.75rem;
.my-4 |margin-top: 1rem; margin-bottom: 1rem;
.my-5 |margin-top: 1.25rem; margin-bottom: 1.25rem;
.my-6 |margin-top: 1.5rem; margin-bottom: 1.5rem;
.my-8 |margin-top: 2rem; margin-bottom: 2rem;
.my-10 |margin-top: 2.5rem; margin-bottom: 2.5rem;
.my-12 |margin-top: 3rem; margin-bottom: 3rem;
.my-16 |margin-top: 4rem; margin-bottom: 4rem;
.my-20 |margin-top: 5rem; margin-bottom: 5rem;
.my-24 |margin-top: 6rem; margin-bottom: 6rem;
.my-32 |margin-top: 8rem; margin-bottom: 8rem;
.my-40 |margin-top: 10rem; margin-bottom: 10rem;
.my-48 |margin-top: 12rem; margin-bottom: 12rem;
.my-56 |margin-top: 14rem; margin-bottom: 14rem;
.my-64 |margin-top: 16rem; margin-bottom: 16rem;
.my-auto |margin-top: auto; margin-bottom: auto;
.my-px |margin-top: 1px; margin-bottom: 1px;
.-my-1 |margin-top: -0.25rem; margin-bottom: -0.25rem;
.-my-2 |margin-top: -0.5rem; margin-bottom: -0.5rem;
.-my-3 |margin-top: -0.75rem; margin-bottom: -0.75rem;
.-my-4 |margin-top: -1rem; margin-bottom: -1rem;
.-my-5 |margin-top: -1.25rem; margin-bottom: -1.25rem;
.-my-6 |margin-top: -1.5rem; margin-bottom: -1.5rem;
.-my-8 |margin-top: -2rem; margin-bottom: -2rem;
.-my-10 |margin-top: -2.5rem; margin-bottom: -2.5rem;
.-my-12 |margin-top: -3rem; margin-bottom: -3rem;
.-my-16 |margin-top: -4rem; margin-bottom: -4rem;
.-my-20 |margin-top: -5rem; margin-bottom: -5rem;
.-my-24 |margin-top: -6rem; margin-bottom: -6rem;
.-my-32 |margin-top: -8rem; margin-bottom: -8rem;
.-my-40 |margin-top: -10rem; margin-bottom: -10rem;
.-my-48 |margin-top: -12rem; margin-bottom: -12rem;
.-my-56 |margin-top: -14rem; margin-bottom: -14rem;
.-my-64 |margin-top: -16rem; margin-bottom: -16rem;
.-my-px |margin-top: -1px; margin-bottom: -1px;
.mx-0 |margin-right: 0; margin-left: 0;
.mx-1 |margin-right: 0.25rem; margin-left: 0.25rem;
.mx-2 |margin-right: 0.5rem; margin-left: 0.5rem;
.mx-3 |margin-right: 0.75rem; margin-left: 0.75rem;
.mx-4 |margin-right: 1rem; margin-left: 1rem;
.mx-5 |margin-right: 1.25rem; margin-left: 1.25rem;
.mx-6 |margin-right: 1.5rem; margin-left: 1.5rem;
.mx-8 |margin-right: 2rem; margin-left: 2rem;
.mx-10 |margin-right: 2.5rem; margin-left: 2.5rem;
.mx-12 |margin-right: 3rem; margin-left: 3rem;
.mx-16 |margin-right: 4rem; margin-left: 4rem;
.mx-20 |margin-right: 5rem; margin-left: 5rem;
.mx-24 |margin-right: 6rem; margin-left: 6rem;
.mx-32 |margin-right: 8rem; margin-left: 8rem;
.mx-40 |margin-right: 10rem; margin-left: 10rem;
.mx-48 |margin-right: 12rem; margin-left: 12rem;
.mx-56 |margin-right: 14rem; margin-left: 14rem;
.mx-64 |margin-right: 16rem; margin-left: 16rem;
.mx-auto |margin-right: auto; margin-left: auto;
.mx-px |margin-right: 1px; margin-left: 1px;
.-mx-1 |margin-right: -0.25rem; margin-left: -0.25rem;
.-mx-2 |margin-right: -0.5rem; margin-left: -0.5rem;
.-mx-3 |margin-right: -0.75rem; margin-left: -0.75rem;
.-mx-4 |margin-right: -1rem; margin-left: -1rem;
.-mx-5 |margin-right: -1.25rem; margin-left: -1.25rem;
.-mx-6 |margin-right: -1.5rem; margin-left: -1.5rem;
.-mx-8 |margin-right: -2rem; margin-left: -2rem;
.-mx-10 |margin-right: -2.5rem; margin-left: -2.5rem;
.-mx-12 |margin-right: -3rem; margin-left: -3rem;
.-mx-16 |margin-right: -4rem; margin-left: -4rem;
.-mx-20 |margin-right: -5rem; margin-left: -5rem;
.-mx-24 |margin-right: -6rem; margin-left: -6rem;
.-mx-32 |margin-right: -8rem; margin-left: -8rem;
.-mx-40 |margin-right: -10rem; margin-left: -10rem;
.-mx-48 |margin-right: -12rem; margin-left: -12rem;
.-mx-56 |margin-right: -14rem; margin-left: -14rem;
.-mx-64 |margin-right: -16rem; margin-left: -16rem;
.-mx-px |margin-right: -1px; margin-left: -1px;
.mt-0 |margin-top: 0;
.mt-1 |margin-top: 0.25rem;
.mt-2 |margin-top: 0.5rem;
.mt-3 |margin-top: 0.75rem;
.mt-4 |margin-top: 1rem;
.mt-5 |margin-top: 1.25rem;
.mt-6 |margin-top: 1.5rem;
.mt-8 |margin-top: 2rem;
.mt-10 |margin-top: 2.5rem;
.mt-12 |margin-top: 3rem;
.mt-16 |margin-top: 4rem;
.mt-20 |margin-top: 5rem;
.mt-24 |margin-top: 6rem;
.mt-32 |margin-top: 8rem;
.mt-40 |margin-top: 10rem;
.mt-48 |margin-top: 12rem;
.mt-56 |margin-top: 14rem;
.mt-64 |margin-top: 16rem;
.mt-auto |margin-top: auto;
.mt-px |margin-top: 1px;
.-mt-1 |margin-top: -0.25rem;
.-mt-2 |margin-top: -0.5rem;
.-mt-3 |margin-top: -0.75rem;
.-mt-4 |margin-top: -1rem;
.-mt-5 |margin-top: -1.25rem;
.-mt-6 |margin-top: -1.5rem;
.-mt-8 |margin-top: -2rem;
.-mt-10 |margin-top: -2.5rem;
.-mt-12 |margin-top: -3rem;
.-mt-16 |margin-top: -4rem;
.-mt-20 |margin-top: -5rem;
.-mt-24 |margin-top: -6rem;
.-mt-32 |margin-top: -8rem;
.-mt-40 |margin-top: -10rem;
.-mt-48 |margin-top: -12rem;
.-mt-56 |margin-top: -14rem;
.-mt-64 |margin-top: -16rem;
.-mt-px |margin-top: -1px;
.mr-0 |margin-right: 0;
.mr-1 |margin-right: 0.25rem;
.mr-2 |margin-right: 0.5rem;
.mr-3 |margin-right: 0.75rem;
.mr-4 |margin-right: 1rem;
.mr-5 |margin-right: 1.25rem;
.mr-6 |margin-right: 1.5rem;
.mr-8 |margin-right: 2rem;
.mr-10 |margin-right: 2.5rem;
.mr-12 |margin-right: 3rem;
.mr-16 |margin-right: 4rem;
.mr-20 |margin-right: 5rem;
.mr-24 |margin-right: 6rem;
.mr-32 |margin-right: 8rem;
.mr-40 |margin-right: 10rem;
.mr-48 |margin-right: 12rem;
.mr-56 |margin-right: 14rem;
.mr-64 |margin-right: 16rem;
.mr-auto |margin-right: auto;
.mr-px |margin-right: 1px;
.-mr-1 |margin-right: -0.25rem;
.-mr-2 |margin-right: -0.5rem;
.-mr-3 |margin-right: -0.75rem;
.-mr-4 |margin-right: -1rem;
.-mr-5 |margin-right: -1.25rem;
.-mr-6 |margin-right: -1.5rem;
.-mr-8 |margin-right: -2rem;
.-mr-10 |margin-right: -2.5rem;
.-mr-12 |margin-right: -3rem;
.-mr-16 |margin-right: -4rem;
.-mr-20 |margin-right: -5rem;
.-mr-24 |margin-right: -6rem;
.-mr-32 |margin-right: -8rem;
.-mr-40 |margin-right: -10rem;
.-mr-48 |margin-right: -12rem;
.-mr-56 |margin-right: -14rem;
.-mr-64 |margin-right: -16rem;
.-mr-px |margin-right: -1px;
.mb-0 |margin-bottom: 0;
.mb-1 |margin-bottom: 0.25rem;
.mb-2 |margin-bottom: 0.5rem;
.mb-3 |margin-bottom: 0.75rem;
.mb-4 |margin-bottom: 1rem;
.mb-5 |margin-bottom: 1.25rem;
.mb-6 |margin-bottom: 1.5rem;
.mb-8 |margin-bottom: 2rem;
.mb-10 |margin-bottom: 2.5rem;
.mb-12 |margin-bottom: 3rem;
.mb-16 |margin-bottom: 4rem;
.mb-20 |margin-bottom: 5rem;
.mb-24 |margin-bottom: 6rem;
.mb-32 |margin-bottom: 8rem;
.mb-40 |margin-bottom: 10rem;
.mb-48 |margin-bottom: 12rem;
.mb-56 |margin-bottom: 14rem;
.mb-64 |margin-bottom: 16rem;
.mb-auto |margin-bottom: auto;
.mb-px |margin-bottom: 1px;
.-mb-1 |margin-bottom: -0.25rem;
.-mb-2 |margin-bottom: -0.5rem;
.-mb-3 |margin-bottom: -0.75rem;
.-mb-4 |margin-bottom: -1rem;
.-mb-5 |margin-bottom: -1.25rem;
.-mb-6 |margin-bottom: -1.5rem;
.-mb-8 |margin-bottom: -2rem;
.-mb-10 |margin-bottom: -2.5rem;
.-mb-12 |margin-bottom: -3rem;
.-mb-16 |margin-bottom: -4rem;
.-mb-20 |margin-bottom: -5rem;
.-mb-24 |margin-bottom: -6rem;
.-mb-32 |margin-bottom: -8rem;
.-mb-40 |margin-bottom: -10rem;
.-mb-48 |margin-bottom: -12rem;
.-mb-56 |margin-bottom: -14rem;
.-mb-64 |margin-bottom: -16rem;
.-mb-px |margin-bottom: -1px;
.ml-0 |margin-left: 0;
.ml-1 |margin-left: 0.25rem;
.ml-2 |margin-left: 0.5rem;
.ml-3 |margin-left: 0.75rem;
.ml-4 |margin-left: 1rem;
.ml-5 |margin-left: 1.25rem;
.ml-6 |margin-left: 1.5rem;
.ml-8 |margin-left: 2rem;
.ml-10 |margin-left: 2.5rem;
.ml-12 |margin-left: 3rem;
.ml-16 |margin-left: 4rem;
.ml-20 |margin-left: 5rem;
.ml-24 |margin-left: 6rem;
.ml-32 |margin-left: 8rem;
.ml-40 |margin-left: 10rem;
.ml-48 |margin-left: 12rem;
.ml-56 |margin-left: 14rem;
.ml-64 |margin-left: 16rem;
.ml-auto |margin-left: auto;
.ml-px |margin-left: 1px;
.-ml-1 |margin-left: -0.25rem;
.-ml-2 |margin-left: -0.5rem;
.-ml-3 |margin-left: -0.75rem;
.-ml-4 |margin-left: -1rem;
.-ml-5 |margin-left: -1.25rem;
.-ml-6 |margin-left: -1.5rem;
.-ml-8 |margin-left: -2rem;
.-ml-10 |margin-left: -2.5rem;
.-ml-12 |margin-left: -3rem;
.-ml-16 |margin-left: -4rem;
.-ml-20 |margin-left: -5rem;
.-ml-24 |margin-left: -6rem;
.-ml-32 |margin-left: -8rem;
.-ml-40 |margin-left: -10rem;
.-ml-48 |margin-left: -12rem;
.-ml-56 |margin-left: -14rem;
.-ml-64 |margin-left: -16rem;
.-ml-px |margin-left: -1px;

## Width

class|Properties
---|---:
.w-0 |width: 0;
.w-1 |width: 0.25rem;
.w-2 |width: 0.5rem;
.w-3 |width: 0.75rem;
.w-4 |width: 1rem;
.w-5 |width: 1.25rem;
.w-6 |width: 1.5rem;
.w-8 |width: 2rem;
.w-10 |width: 2.5rem;
.w-12 |width: 3rem;
.w-16 |width: 4rem;
.w-20 |width: 5rem;
.w-24 |width: 6rem;
.w-32 |width: 8rem;
.w-40 |width: 10rem;
.w-48 |width: 12rem;
.w-56 |width: 14rem;
.w-64 |width: 16rem;
.w-auto |width: auto;
.w-px |width: 1px;
.w-1/2 |width: 50%;
.w-1/3 |width: 33.33333%;
.w-2/3 |width: 66.66667%;
.w-1/4 |width: 25%;
.w-2/4 |width: 50%;
.w-3/4 |width: 75%;
.w-1/5 |width: 20%;
.w-2/5 |width: 40%;
.w-3/5 |width: 60%;
.w-4/5 |width: 80%;
.w-1/6 |width: 16.66667%;
.w-2/6 |width: 33.33333%;
.w-3/6 |width: 50%;
.w-4/6 |width: 66.66667%;
.w-5/6 |width: 83.33333%;
.w-1/12 |width: 8.33333%;
.w-2/12 |width: 16.66667%;
.w-3/12 |width: 25%;
.w-4/12 |width: 33.33333%;
.w-5/12 |width: 41.66667%;
.w-6/12 |width: 50%;
.w-7/12 |width: 58.33333%;
.w-8/12 |width: 66.66667%;
.w-9/12 |width: 75%;
.w-10/12 |width: 83.33333%;
.w-11/12 |width: 91.66667%;
.w-full |width: 100%;
.w-screen |width: 100vw;

## Min-Width

class|Properties
---|---:
.min-w-0 |min-width: 0;
.min-w-full |min-width: 100%;

## Max-Width

Utilities for setting the maximum width of an element

class|Properties
---|---:
.max-w-xs |max-width: 20rem;
.max-w-sm |max-width: 24rem;
.max-w-md |max-width: 28rem;
.max-w-lg |max-width: 32rem;
.max-w-xl |max-width: 36rem;
.max-w-2xl |max-width: 42rem;
.max-w-3xl |max-width: 48rem;
.max-w-4xl |max-width: 56rem;
.max-w-5xl |max-width: 64rem;
.max-w-6xl |max-width: 72rem;
.max-w-full |max-width: 100%;

## Height

```js
.h-0 : height: 0;
.h-1 |height: 0.25rem;
.h-2 |height: 0.5rem;
.h-3 |height: 0.75rem;
.h-4 |height: 1rem;
.h-5 |height: 1.25rem;
.h-6 |height: 1.5rem;
.h-8 |height: 2rem;
.h-10 |height: 2.5rem;
.h-12 |height: 3rem;
.h-16 |height: 4rem;
.h-20 |height: 5rem;
.h-24 |height: 6rem;
.h-32 |height: 8rem;
.h-40 |height: 10rem;
.h-48 |height: 12rem;
.h-56 |height: 14rem;
.h-64 |height: 16rem;
.h-auto |height: auto;
.h-px |height: 1px;
.h-full |height: 100%;
.h-screen |height: 100vh;
```

## Min-Height

Utilities for setting the minimum height of an element

class|Properties
---|---:
.min-h-0 |min-height: 0;
.min-h-full |min-height: 100%;
.min-h-screen |min-height: 100vh;

## Max-Height

Utilities for setting the maximum height of an element

class|Properties
---|---:
.max-h-full |max-height: 100%;
.max-h-screen |max-height: 100vh;

## Tables

### Border Collapse

Utilities for controlling whether table borders should collapse or be separated.

class|Properties
---|---:
.border-collapse |border-collapse: collapse;
.border-separate |border-collapse: separate;

### Table Layout

Utilities for controlling the table layout algorithm.

class|Properties
---|---:
.table-auto |table-layout: auto;
.table-fixed |table-layout: fixed;

## Effects

### Box Shadow

Utilities for controlling the box shadow of an element.

class|Properties
---|---:
.shadow |box-shadow: 0 1px 3px 0 rgba(0, 0, 0, 0.1), 0 1px 2px 0 rgba(0, 0, 0, 0.06);
.shadow-md |box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06);
.shadow-lg |box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -2px rgba(0, 0, 0, 0.05);
.shadow-xl |box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.1), 0 10px 10px -5px rgba(0, 0, 0, 0.04);
.shadow-2xl |box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.25);
.shadow-inner |box-shadow: inset 0 2px 4px 0 rgba(0, 0, 0, 0.06);
.shadow-outline |box-shadow: 0 0 0 3px rgba(66, 153, 225, 0.5);
.shadow-none |box-shadow: none;

### Opacity

Utilities for controlling the opacity of an element.

class|Properties
---|---:
.opacity-100 |opacity: 1;
.opacity-75 |opacity: .75;
.opacity-50 |opacity: .5;
.opacity-25 |opacity: .25;
.opacity-0 |opacity: 0;

# 安装 tailwind

```bash
npm install tailwindcss --save-dev
npx tailwind init tailwind.js

npm install postcss-cli autoprefixer --save-dev

touch postcss.config.js
```

postcss.config.js 文件中输入以下内容：

```js
var tailwindcss = require('tailwindcss');

module.exports = {
  plugins: [
    tailwindcss('./path/to/your/tailwind.js'),
    require('autoprefixer'),
  ]
}
```

对tailwind进行配置：

```bash
mkdir -p src/styles
cd src/styles
touch tailwind.css
```

tailwind.css 文件中输入以下内容：

```js
@tailwind base;
@tailwind components;
@tailwind utilities;
```

对项目进行进行配置：

```bash
npm install npm-run-all --save-dev
```

打开package.json，进行修改：

```js
"scripts": {
    "start": "npm-run-all --parallel watch:css start:react",
    "build": "npm-run-all build:css build:react",
    "build:css": "postcss src/styles/tailwind.css -o src/index.css",
    "watch:css": "postcss src/styles/tailwind.css -o src/index.css -w",
    "start:react": "react-scripts start",
    "build:react": "react-scripts build",
    "test": "react-scripts test",
    "eject": "react-scripts eject"
  },
```

运行项目：

```bash
npm start
```

使用tailwind

```js
import React from "react";
import ReactDOM from "react-dom";
import "./index.css";

const App = () => {
  return <div className="bg-blue">Hello World!</div>
};

ReactDOM.render(<App />, document.querySelector("#root"));
```
