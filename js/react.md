# Create React App

```js
npx create-react-app my-app
cd my-app
npm start

npm run build
```

# 阻止默认行为

```js
<a href="#" onclick="console.log('The link was clicked.'); return false">
  Click me
</a>

function ActionLink() {
  function handleClick(e) {
    e.preventDefault();
    console.log('The link was clicked.');
  }

  return (
    <a href="#" onClick={handleClick}>
      Click me
    </a>
  );
}
```

Here, e is a synthetic event.

# 传递参数

```js
<button onClick={(e) => this.deleteRow(id, e)}>Delete Row</button>
```

# 返回 null 不进行绘制

```js
function WarningBanner(props) {
  if (!props.warn) {
    return null;
  }

  return (
    <div className="warning">
      Warning!
    </div>
  );
}
```

# Lists and Keys

```js
const numbers = [1, 2, 3, 4, 5];
const listItems = numbers.map((number) =>
  <li key={number.toString()}>
    {number}
  </li>
);
```

# focus

```js
class CustomTextInput extends React.Component {
  constructor(props) {
    super(props);
    // Create a ref to store the textInput DOM element
    this.textInput = React.createRef();
  }
  render() {
  // Use the `ref` callback to store a reference to the text input DOM
  // element in an instance field (for example, this.textInput).
    return (
      <input
        type="text"
        ref={this.textInput}
      />
    );
  }
}

focus() {
  // Explicitly focus the text input using the raw DOM API
  // Note: we're accessing "current" to get the DOM node
  this.textInput.current.focus();
}
```

Sometimes a parent component needs to set focus to an element in a child component. 

```js
function CustomTextInput(props) {
  return (
    <div>
      <input ref={props.inputRef} />
    </div>
  );
}

class Parent extends React.Component {
  constructor(props) {
    super(props);
    this.inputElement = React.createRef();
  }
  render() {
    return (
      <CustomTextInput inputRef={this.inputElement} />
    );
  }
}

// Now you can set focus when required.
this.inputElement.current.focus();
```

# Mouse and pointer events

```js
class OuterClickExample extends React.Component {
  constructor(props) {
    super(props);

    this.state = { isOpen: false };
    this.toggleContainer = React.createRef();

    this.onClickHandler = this.onClickHandler.bind(this);
    this.onClickOutsideHandler = this.onClickOutsideHandler.bind(this);
  }

  componentDidMount() {
    window.addEventListener('click', this.onClickOutsideHandler);
  }

  componentWillUnmount() {
    window.removeEventListener('click', this.onClickOutsideHandler);
  }

  onClickHandler() {
    this.setState(currentState => ({
      isOpen: !currentState.isOpen
    }));
  }

  onClickOutsideHandler(event) {
    if (this.state.isOpen && !this.toggleContainer.current.contains(event.target)) {
      this.setState({ isOpen: false });
    }
  }

  render() {
    return (
      <div ref={this.toggleContainer}>
        <button onClick={this.onClickHandler}>Select an option</button>
        {this.state.isOpen && (
          <ul>
            <li>Option 1</li>
            <li>Option 2</li>
            <li>Option 3</li>
          </ul>
        )}
      </div>
    );
  }
}
```

A toggle button opening a popover list implemented with the click outside pattern and operated with the keyboard showing the popover not being closed on blur and it obscuring other screen elements.

```js
class BlurExample extends React.Component {
  constructor(props) {
    super(props);

    this.state = { isOpen: false };
    this.timeOutId = null;

    this.onClickHandler = this.onClickHandler.bind(this);
    this.onBlurHandler = this.onBlurHandler.bind(this);
    this.onFocusHandler = this.onFocusHandler.bind(this);
  }

  onClickHandler() {
    this.setState(currentState => ({
      isOpen: !currentState.isOpen
    }));
  }

  // We close the popover on the next tick by using setTimeout.
  // This is necessary because we need to first check if
  // another child of the element has received focus as
  // the blur event fires prior to the new focus event.
  onBlurHandler() {
    this.timeOutId = setTimeout(() => {
      this.setState({
        isOpen: false
      });
    });
  }

  // If a child receives focus, do not close the popover.
  onFocusHandler() {
    clearTimeout(this.timeOutId);
  }

  render() {
    // React assists us by bubbling the blur and
    // focus events to the parent.
    return (
      <div onBlur={this.onBlurHandler}
           onFocus={this.onFocusHandler}>
        <button onClick={this.onClickHandler}>
          Select an option
        </button>
        {this.state.isOpen && (
          <ul>
            <li>Option 1</li>
            <li>Option 2</li>
            <li>Option 3</li>
          </ul>
        )}
      </div>
    );
  }
}
```

This code exposes the functionality to both pointer device and keyboard users.
A popover list correctly closing for both mouse and keyboard users.

# When to Use Context

Context is designed to share data that can be considered “global” for a tree of React components, such as the current authenticated user, theme, or preferred language.

```js
// Context lets us pass a value deep into the component tree
// without explicitly threading it through every component.
// Create a context for the current theme (with "light" as the default).
const ThemeContext = React.createContext('light');

class App extends React.Component {
  render() {
    // Use a Provider to pass the current theme to the tree below.
    // Any component can read it, no matter how deep it is.
    // In this example, we're passing "dark" as the current value.
    return (
      <ThemeContext.Provider value="dark">
        <Toolbar />
      </ThemeContext.Provider>
    );
  }
}

// A component in the middle doesn't have to
// pass the theme down explicitly anymore.
function Toolbar(props) {
  return (
    <div>
      <ThemedButton />
    </div>
  );
}

class ThemedButton extends React.Component {
  // Assign a contextType to read the current theme context.
  // React will find the closest theme Provider above and use its value.
  // In this example, the current theme is "dark".
  static contextType = ThemeContext;
  render() {
    return <Button theme={this.context} />;
  }
}
```

One Provider can be connected to many consumers. Providers can be nested to override values deeper within the tree.

```js
class MyClass extends React.Component {
  static contextType = MyContext;

  componentDidMount() {
    let value = this.context;
    /* perform a side-effect at mount using the value of MyContext */
  }
  componentDidUpdate() {
    let value = this.context;
    /* ... */
  }
  componentWillUnmount() {
    let value = this.context;
    /* ... */
  }
  render() {
    let value = this.context;
    /* render something based on the value of MyContext */
  }
}
```

```js
<MyContext.Consumer>
  {value => /* render something based on the context value */}
</MyContext.Consumer>
```

Requires a function as a child. The function receives the current context value and returns a React node.

```js
theme-context.js

export const themes = {
  light: {
    foreground: '#000000',
    background: '#eeeeee',
  },
  dark: {
    foreground: '#ffffff',
    background: '#222222',
  },
};

export const ThemeContext = React.createContext(
  themes.dark // default value
);

themed-button.js

import {ThemeContext} from './theme-context';

class ThemedButton extends React.Component {
  render() {
    let props = this.props;
    let theme = this.context;
    return (
      <button
        {...props}
        style={{backgroundColor: theme.background}}
      />
    );
  }
}
ThemedButton.contextType = ThemeContext;

export default ThemedButton;

app.js

import {ThemeContext, themes} from './theme-context';
import ThemedButton from './themed-button';

// An intermediate component that uses the ThemedButton
function Toolbar(props) {
  return (
    <ThemedButton onClick={props.changeTheme}>
      Change Theme
    </ThemedButton>
  );
}

class App extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
      theme: themes.light,
    };

    this.toggleTheme = () => {
      this.setState(state => ({
        theme:
          state.theme === themes.dark
            ? themes.light
            : themes.dark,
      }));
    };
  }

  render() {
    // The ThemedButton button inside the ThemeProvider
    // uses the theme from state while the one outside uses
    // the default dark theme
    return (
      <Page>
        <ThemeContext.Provider value={this.state.theme}>
          <Toolbar changeTheme={this.toggleTheme} />
        </ThemeContext.Provider>
        <Section>
          <ThemedButton />
        </Section>
      </Page>
    );
  }
}

ReactDOM.render(<App />, document.root);
```

It is often necessary to update the context from a component that is nested somewhere deeply in the component tree. In this case you can pass a function down through the context to allow consumers to update the context:

```js
theme-context.js

// Make sure the shape of the default value passed to
// createContext matches the shape that the consumers expect!
export const ThemeContext = React.createContext({
  theme: themes.dark,
  toggleTheme: () => {},
});

theme-toggler-button.js

import {ThemeContext} from './theme-context';

function ThemeTogglerButton() {
  // The Theme Toggler Button receives not only the theme
  // but also a toggleTheme function from the context
  return (
    <ThemeContext.Consumer>
      {({theme, toggleTheme}) => (
        <button
          onClick={toggleTheme}
          style={{backgroundColor: theme.background}}>
          Toggle Theme
        </button>
      )}
    </ThemeContext.Consumer>
  );
}

export default ThemeTogglerButton;

app.js

import {ThemeContext, themes} from './theme-context';
import ThemeTogglerButton from './theme-toggler-button';

class App extends React.Component {
  constructor(props) {
    super(props);

    this.toggleTheme = () => {
      this.setState(state => ({
        theme:
          state.theme === themes.dark
            ? themes.light
            : themes.dark,
      }));
    };

    // State also contains the updater function so it will
    // be passed down into the context provider
    this.state = {
      theme: themes.light,
      toggleTheme: this.toggleTheme,
    };
  }

  render() {
    // The entire state is passed to the provider
    return (
      <ThemeContext.Provider value={this.state}>
        <Content />
      </ThemeContext.Provider>
    );
  }
}

function Content() {
  return (
    <div>
      <ThemeTogglerButton />
    </div>
  );
}

ReactDOM.render(<App />, document.root);
```

To keep context re-rendering fast, React needs to make each context consumer a separate node in the tree.

```js
// Theme context, default to light theme
const ThemeContext = React.createContext('light');

// Signed-in user context
const UserContext = React.createContext({
  name: 'Guest',
});

class App extends React.Component {
  render() {
    const {signedInUser, theme} = this.props;

    // App component that provides initial context values
    return (
      <ThemeContext.Provider value={theme}>
        <UserContext.Provider value={signedInUser}>
          <Layout />
        </UserContext.Provider>
      </ThemeContext.Provider>
    );
  }
}

function Layout() {
  return (
    <div>
      <Sidebar />
      <Content />
    </div>
  );
}

// A component may consume multiple contexts
function Content() {
  return (
    <ThemeContext.Consumer>
      {theme => (
        <UserContext.Consumer>
          {user => (
            <ProfilePage user={user} theme={theme} />
          )}
        </UserContext.Consumer>
      )}
    </ThemeContext.Consumer>
  );
}
```

 For example, the code below will re-render all consumers every time the Provider re-renders because a new object is always created for value:

```js
class App extends React.Component {
  render() {
    return (
      <Provider value={{something: 'something'}}>
        <Toolbar />
      </Provider>
    );
  }
}
```

To get around this, lift the value into the parent’s state:

```js
class App extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
      value: {something: 'something'},
    };
  }

  render() {
    return (
      <Provider value={this.state.value}>
        <Toolbar />
      </Provider>
    );
  }
}
```

Error Boundaries

```js
class ErrorBoundary extends React.Component {
  constructor(props) {
    super(props);
    this.state = { hasError: false };
  }

  static getDerivedStateFromError(error) {
    // Update state so the next render will show the fallback UI.
    return { hasError: true };
  }

  componentDidCatch(error, info) {
    // You can also log the error to an error reporting service
    logErrorToMyService(error, info);
  }

  render() {
    if (this.state.hasError) {
      // You can render any custom fallback UI
      return <h1>Something went wrong.</h1>;
    }

    return this.props.children;
  }
}

<ErrorBoundary>
  <MyWidget />
</ErrorBoundary>
```

Only class components can be error boundaries. In practice, most of the time you’ll want to declare an error boundary component once and use it throughout your application.

Error boundaries do not catch errors inside event handlers.

If you need to catch an error inside event handler, use the regular JavaScript try / catch statement:

```js
class MyComponent extends React.Component {
  constructor(props) {
    super(props);
    this.state = { error: null };
    this.handleClick = this.handleClick.bind(this);
  }

  handleClick() {
    try {
      // Do something that could throw
    } catch (error) {
      this.setState({ error });
    }
  }

  render() {
    if (this.state.error) {
      return <h1>Caught an error.</h1>
    }
    return <div onClick={this.handleClick}>Click Me</div>
  }
}
```

# Forwarding Refs

Ref forwarding is a technique for automatically passing a ref through a component to one of its children.

```js
function FancyButton(props) {
  return (
    <button className="FancyButton">
      {props.children}
    </button>
  );
}
```

In the example below, FancyButton uses React.forwardRef to obtain the ref passed to it, and then forward it to the DOM button that it renders:

```js
const FancyButton = React.forwardRef((props, ref) => (
  <button ref={ref} className="FancyButton">
    {props.children}
  </button>
));

// You can now get a ref directly to the DOM button:
const ref = React.createRef();
<FancyButton ref={ref}>Click me!</FancyButton>;
```

```js
function logProps(WrappedComponent) {
  class LogProps extends React.Component {
    componentDidUpdate(prevProps) {
      console.log('old props:', prevProps);
      console.log('new props:', this.props);
    }

    render() {
      return <WrappedComponent {...this.props} />;
    }
  }

  return LogProps;
}
```

the above example: refs will not get passed through. That’s because ref is not a prop. Like key, it’s handled differently by React. If you add a ref to a HOC, the ref will refer to the outermost container component, not the wrapped component.

```js
import FancyButton from './FancyButton';

const ref = React.createRef();

// The FancyButton component we imported is the LogProps HOC.
// Even though the rendered output will be the same,
// Our ref will point to LogProps instead of the inner FancyButton component!
// This means we can't call e.g. ref.current.focus()
<FancyButton
  label="Click Me"
  handleClick={handleClick}
  ref={ref}
/>;
```

```js
function logProps(Component) {
  class LogProps extends React.Component {
    componentDidUpdate(prevProps) {
      console.log('old props:', prevProps);
      console.log('new props:', this.props);
    }

    render() {
      const {forwardedRef, ...rest} = this.props;

      // Assign the custom prop "forwardedRef" as a ref
      return <Component ref={forwardedRef} {...rest} />;
    }
  }

  // Note the second param "ref" provided by React.forwardRef.
  // We can pass it along to LogProps as a regular prop, e.g. "forwardedRef"
  // And it can then be attached to the Component.
  return React.forwardRef((props, ref) => {
    return <LogProps {...props} forwardedRef={ref} />;
  });
}
```

# Higher-Order Components

Concretely, a higher-order component is a function that takes a component and returns a new component.

```js
const CommentListWithSubscription = withSubscription(
  CommentList,
  (DataSource) => DataSource.getComments()
);

const BlogPostWithSubscription = withSubscription(
  BlogPost,
  (DataSource, props) => DataSource.getBlogPost(props.id)
);
```

The first parameter is the wrapped component. The second parameter retrieves the data we’re interested in, given a DataSource and the current props.

When CommentListWithSubscription and BlogPostWithSubscription are rendered, CommentList and BlogPost will be passed a data prop with the most current data retrieved from DataSource:

```js
// This function takes a component...
function withSubscription(WrappedComponent, selectData) {
  // ...and returns another component...
  return class extends React.Component {
    constructor(props) {
      super(props);
      this.handleChange = this.handleChange.bind(this);
      this.state = {
        data: selectData(DataSource, props)
      };
    }

    componentDidMount() {
      // ... that takes care of the subscription...
      DataSource.addChangeListener(this.handleChange);
    }

    componentWillUnmount() {
      DataSource.removeChangeListener(this.handleChange);
    }

    handleChange() {
      this.setState({
        data: selectData(DataSource, this.props)
      });
    }

    render() {
      // ... and renders the wrapped component with the fresh data!
      // Notice that we pass through any additional props
      return <WrappedComponent data={this.state.data} {...this.props} />;
    }
  };
}
```

Note that a HOC doesn’t modify the input component, nor does it use inheritance to copy its behavior. A HOC is a pure function with zero side-effects.

```js
function withSubscription(WrappedComponent) {
  class WithSubscription extends React.Component {/* ... */}
  WithSubscription.displayName = `WithSubscription(${getDisplayName(WrappedComponent)})`;
  return WithSubscription;
}

function getDisplayName(WrappedComponent) {
  return WrappedComponent.displayName || WrappedComponent.name || 'Component';
}
```

Don’t Use HOCs Inside the render Method

Instead, apply HOCs outside the component definition so that the resulting component is created only once.

In those rare cases where you need to apply a HOC dynamically, you can also do it inside a component’s lifecycle methods or its constructor.

When you apply a HOC to a component, though, the original component is wrapped with a container component. That means the new component does not have any of the static methods of the original component.

To solve this, you could copy the methods onto the container before returning it:

```js
function enhance(WrappedComponent) {
  class Enhance extends React.Component {/*...*/}
  // Must know exactly which method(s) to copy :(
  Enhance.staticMethod = WrappedComponent.staticMethod;
  return Enhance;
}
```

You can use hoist-non-react-statics to automatically copy all non-React static methods:

```js
import hoistNonReactStatic from 'hoist-non-react-statics';

function enhance(WrappedComponent) {
  class Enhance extends React.Component {/*...*/}
  hoistNonReactStatic(Enhance, WrappedComponent);
  return Enhance;
}
```

Refs Aren’t Passed Through

ref is not really a prop — like key, it’s handled specially by React.

# Extracting Data from Backbone Models

```js
function connectToBackboneModel(WrappedComponent) {
  return class BackboneComponent extends React.Component {
    constructor(props) {
      super(props);
      this.state = Object.assign({}, props.model.attributes);
      this.handleChange = this.handleChange.bind(this);
    }

    componentDidMount() {
      this.props.model.on('change', this.handleChange);
    }

    componentWillReceiveProps(nextProps) {
      this.setState(Object.assign({}, nextProps.model.attributes));
      if (nextProps.model !== this.props.model) {
        this.props.model.off('change', this.handleChange);
        nextProps.model.on('change', this.handleChange);
      }
    }

    componentWillUnmount() {
      this.props.model.off('change', this.handleChange);
    }

    handleChange(model) {
      this.setState(model.changedAttributes());
    }

    render() {
      const propsExceptModel = Object.assign({}, this.props);
      delete propsExceptModel.model;
      return <WrappedComponent {...propsExceptModel} {...this.state} />;
    }
  }
}

function NameInput(props) {
  return (
    <p>
      <input value={props.firstName} onChange={props.handleChange} />
      <br />
      My name is {props.firstName}.
    </p>
  );
}

const BackboneNameInput = connectToBackboneModel(NameInput);

function Example(props) {
  function handleChange(e) {
    props.model.set('firstName', e.target.value);
  }

  return (
    <BackboneNameInput
      model={props.model}
      handleChange={handleChange}
    />
  );
}

const model = new Backbone.Model({ firstName: 'Frodo' });
ReactDOM.render(
  <Example model={model} />,
  document.getElementById('root')
);
```

# JSX In Depth

```js
const MyComponents = {
  DatePicker: function DatePicker(props) {
    return <div>Imagine a {props.color} datepicker here.</div>;
  }
}

function BlueDatePicker() {
  return <MyComponents.DatePicker color="blue" />;
}

<div>
  {showHeader && <Header />}
  <Content />
</div>

<div>
  {props.messages.length > 0 &&
    <MessageList messages={props.messages} />
  }
</div>
```

# the most efficient Brunch production build

```bash
npm install --save-dev uglify-js-brunch
npm install --save-dev envify uglify-js uglifyify
npm install --save-dev rollup-plugin-commonjs rollup-plugin-replace rollup-plugin-uglify

plugins: [
  // ...
  require('rollup-plugin-replace')({
    'process.env.NODE_ENV': JSON.stringify('production')
  }),
  require('rollup-plugin-commonjs')(),
  require('rollup-plugin-uglify')(),
  // ...
]

brunch build -p
```

# Virtualize Long Lists

If your application renders long lists of data (hundreds or thousands of rows), we recommended using a technique known as “windowing”. This technique only renders a small subset of your rows at any given time, and can dramatically reduce the time it takes to re-render the components as well as the number of DOM nodes created.

react-window and react-virtualized are popular windowing libraries. 

# shouldComponentUpdate In Action

```js
class CounterButton extends React.Component {
  constructor(props) {
    super(props);
    this.state = {count: 1};
  }

  shouldComponentUpdate(nextProps, nextState) {
    if (this.props.color !== nextProps.color) {
      return true;
    }
    if (this.state.count !== nextState.count) {
      return true;
    }
    return false;
  }

  render() {
    return (
      <button
        color={this.props.color}
        onClick={() => this.setState(state => ({count: state.count + 1}))}>
        Count: {this.state.count}
      </button>
    );
  }
}
```

```js
class ListOfWords extends React.PureComponent {
  render() {
    return <div>{this.props.words.join(',')}</div>;
  }
}

class WordAdder extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
      words: ['marklar']
    };
    this.handleClick = this.handleClick.bind(this);
  }

  handleClick() {
    // This section is bad style and causes a bug
    const words = this.state.words;
    words.push('marklar');
    this.setState({words: words});
  }

  render() {
    return (
      <div>
        <button onClick={this.handleClick} />
        <ListOfWords words={this.state.words} />
      </div>
    );
  }
}
```

The problem is that PureComponent will do a simple comparison between the old and new values of this.props.words. Since this code mutates the words array in the handleClick method of WordAdder, the old and new values of this.props.words will compare as equal, even though the actual words in the array have changed. The ListOfWords will thus not update even though it has new words that should be rendered.

The simplest way to avoid this problem is to avoid mutating values that you are using as props or state. For example, the handleClick method above could be rewritten using concat as:

```js
handleClick() {
  this.setState(state => ({
    words: [...state.words, 'marklar'],
  }));
};
```

# Refs and Function Components

You may not use the ref attribute on function components because they don’t have instances:

You should convert the component to a class if you need a ref to it, just like you do when you need lifecycle methods or state.

You can, however, use the ref attribute inside a function component as long as you refer to a DOM element or a class component:

```js
function CustomTextInput(props) {
  // textInput must be declared here so the ref can refer to it
  let textInput = React.createRef();

  function handleClick() {
    textInput.current.focus();
  }

  return (
    <div>
      <input
        type="text"
        ref={textInput} />
      <input
        type="button"
        value="Focus the text input"
        onClick={handleClick}
      />
    </div>
  );
}
```

If the ref callback is defined as an inline function, it will get called twice during updates, first with null and then again with the DOM element. This is because a new instance of the function is created with each render, so React needs to clear the old ref and set up the new one. You can avoid this by defining the ref callback as a bound method on the class, but note that it shouldn’t matter in most cases.

# Render Props

The term “render prop” refers to a technique for sharing code between React components using a prop whose value is a function.

A component with a render prop takes a function that returns a React element and calls it instead of implementing its own render logic.

<DataProvider render={data => (
  <h1>Hello {data.target}</h1>
)}/>

Libraries that use render props include React Router and Downshift.

In this document, we’ll discuss why render props are useful, and how to write your own.
Use Render Props for Cross-Cutting Concerns

Components are the primary unit of code reuse in React, but it’s not always obvious how to share the state or behavior that one component encapsulates to other components that need that same state.

For example, the following component tracks the mouse position in a web app:

class MouseTracker extends React.Component {
  constructor(props) {
    super(props);
    this.handleMouseMove = this.handleMouseMove.bind(this);
    this.state = { x: 0, y: 0 };
  }

  handleMouseMove(event) {
    this.setState({
      x: event.clientX,
      y: event.clientY
    });
  }

  render() {
    return (
      <div style={{ height: '100%' }} onMouseMove={this.handleMouseMove}>
        <h1>Move the mouse around!</h1>
        <p>The current mouse position is ({this.state.x}, {this.state.y})</p>
      </div>
    );
  }
}

As the cursor moves around the screen, the component displays its (x, y) coordinates in a <p>.

Now the question is: How can we reuse this behavior in another component? In other words, if another component needs to know about the cursor position, can we encapsulate that behavior so that we can easily share it with that component?

Since components are the basic unit of code reuse in React, let’s try refactoring the code a bit to use a <Mouse> component that encapsulates the behavior we need to reuse elsewhere.

// The <Mouse> component encapsulates the behavior we need...
class Mouse extends React.Component {
  constructor(props) {
    super(props);
    this.handleMouseMove = this.handleMouseMove.bind(this);
    this.state = { x: 0, y: 0 };
  }

  handleMouseMove(event) {
    this.setState({
      x: event.clientX,
      y: event.clientY
    });
  }

  render() {
    return (
      <div style={{ height: '100%' }} onMouseMove={this.handleMouseMove}>

        {/* ...but how do we render something other than a <p>? */}
        <p>The current mouse position is ({this.state.x}, {this.state.y})</p>
      </div>
    );
  }
}

class MouseTracker extends React.Component {
  render() {
    return (
      <div>
        <h1>Move the mouse around!</h1>
        <Mouse />
      </div>
    );
  }
}

Now the <Mouse> component encapsulates all behavior associated with listening for mousemove events and storing the (x, y) position of the cursor, but it’s not yet truly reusable.

For example, let’s say we have a <Cat> component that renders the image of a cat chasing the mouse around the screen. We might use a <Cat mouse={{ x, y }}> prop to tell the component the coordinates of the mouse so it knows where to position the image on the screen.

As a first pass, you might try rendering the <Cat> inside <Mouse>’s render method, like this:

class Cat extends React.Component {
  render() {
    const mouse = this.props.mouse;
    return (
      <img src="/cat.jpg" style={{ position: 'absolute', left: mouse.x, top: mouse.y }} />
    );
  }
}

class MouseWithCat extends React.Component {
  constructor(props) {
    super(props);
    this.handleMouseMove = this.handleMouseMove.bind(this);
    this.state = { x: 0, y: 0 };
  }

  handleMouseMove(event) {
    this.setState({
      x: event.clientX,
      y: event.clientY
    });
  }

  render() {
    return (
      <div style={{ height: '100%' }} onMouseMove={this.handleMouseMove}>

        {/*
          We could just swap out the <p> for a <Cat> here ... but then
          we would need to create a separate <MouseWithSomethingElse>
          component every time we need to use it, so <MouseWithCat>
          isn't really reusable yet.
        */}
        <Cat mouse={this.state} />
      </div>
    );
  }
}

class MouseTracker extends React.Component {
  render() {
    return (
      <div>
        <h1>Move the mouse around!</h1>
        <MouseWithCat />
      </div>
    );
  }
}

This approach will work for our specific use case, but we haven’t achieved the objective of truly encapsulating the behavior in a reusable way. Now, every time we want the mouse position for a different use case, we have to create a new component (i.e. essentially another <MouseWithCat>) that renders something specifically for that use case.

Here’s where the render prop comes in: Instead of hard-coding a <Cat> inside a <Mouse> component, and effectively changing its rendered output, we can provide <Mouse> with a function prop that it uses to dynamically determine what to render–a render prop.

class Cat extends React.Component {
  render() {
    const mouse = this.props.mouse;
    return (
      <img src="/cat.jpg" style={{ position: 'absolute', left: mouse.x, top: mouse.y }} />
    );
  }
}

class Mouse extends React.Component {
  constructor(props) {
    super(props);
    this.handleMouseMove = this.handleMouseMove.bind(this);
    this.state = { x: 0, y: 0 };
  }

  handleMouseMove(event) {
    this.setState({
      x: event.clientX,
      y: event.clientY
    });
  }

  render() {
    return (
      <div style={{ height: '100%' }} onMouseMove={this.handleMouseMove}>

        {/*
          Instead of providing a static representation of what <Mouse> renders,
          use the `render` prop to dynamically determine what to render.
        */}
        {this.props.render(this.state)}
      </div>
    );
  }
}

class MouseTracker extends React.Component {
  render() {
    return (
      <div>
        <h1>Move the mouse around!</h1>
        <Mouse render={mouse => (
          <Cat mouse={mouse} />
        )}/>
      </div>
    );
  }
}

Now, instead of effectively cloning the <Mouse> component and hard-coding something else in its render method to solve for a specific use case, we provide a render prop that <Mouse> can use to dynamically determine what it renders.

More concretely, a render prop is a function prop that a component uses to know what to render.

This technique makes the behavior that we need to share extremely portable. To get that behavior, render a <Mouse> with a render prop that tells it what to render with the current (x, y) of the cursor.

One interesting thing to note about render props is that you can implement most higher-order components (HOC) using a regular component with a render prop. For example, if you would prefer to have a withMouse HOC instead of a <Mouse> component, you could easily create one using a regular <Mouse> with a render prop:

// If you really want a HOC for some reason, you can easily
// create one using a regular component with a render prop!
function withMouse(Component) {
  return class extends React.Component {
    render() {
      return (
        <Mouse render={mouse => (
          <Component {...this.props} mouse={mouse} />
        )}/>
      );
    }
  }
}

So using a render prop makes it possible to use either pattern.
Using Props Other Than render

It’s important to remember that just because the pattern is called “render props” you don’t have to use a prop named render to use this pattern. In fact, any prop that is a function that a component uses to know what to render is technically a “render prop”.

Although the examples above use render, we could just as easily use the children prop!

<Mouse children={mouse => (
  <p>The mouse position is {mouse.x}, {mouse.y}</p>
)}/>

And remember, the children prop doesn’t actually need to be named in the list of “attributes” in your JSX element. Instead, you can put it directly inside the element!

<Mouse>
  {mouse => (
    <p>The mouse position is {mouse.x}, {mouse.y}</p>
  )}
</Mouse>

You’ll see this technique used in the react-motion API.

Since this technique is a little unusual, you’ll probably want to explicitly state that children should be a function in your propTypes when designing an API like this.

Mouse.propTypes = {
  children: PropTypes.func.isRequired
};

Caveats
Be careful when using Render Props with React.PureComponent

Using a render prop can negate the advantage that comes from using React.PureComponent if you create the function inside a render method. This is because the shallow prop comparison will always return false for new props, and each render in this case will generate a new value for the render prop.

For example, continuing with our <Mouse> component from above, if Mouse were to extend React.PureComponent instead of React.Component, our example would look like this:

class Mouse extends React.PureComponent {
  // Same implementation as above...
}

class MouseTracker extends React.Component {
  render() {
    return (
      <div>
        <h1>Move the mouse around!</h1>

        {/*
          This is bad! The value of the `render` prop will
          be different on each render.
        */}
        <Mouse render={mouse => (
          <Cat mouse={mouse} />
        )}/>
      </div>
    );
  }
}

In this example, each time <MouseTracker> renders, it generates a new function as the value of the <Mouse render> prop, thus negating the effect of <Mouse> extending React.PureComponent in the first place!

To get around this problem, you can sometimes define the prop as an instance method, like so:

class MouseTracker extends React.Component {
  // Defined as an instance method, `this.renderTheCat` always
  // refers to *same* function when we use it in render
  renderTheCat(mouse) {
    return <Cat mouse={mouse} />;
  }

  render() {
    return (
      <div>
        <h1>Move the mouse around!</h1>
        <Mouse render={this.renderTheCat} />
      </div>
    );
  }
}

In cases where you cannot define the prop statically (e.g. because you need to close over the component’s props and/or state) <Mouse> should extend React.Component instead.
Static Type Checking

Static type checkers like Flow and TypeScript identify certain types of problems before you even run your code. They can also improve developer workflow by adding features like auto-completion. For this reason, we recommend using Flow or TypeScript instead of PropTypes for larger code bases.

# Hooks

React.memo

const MyComponent = React.memo(function MyComponent(props) {
  /* render using props */
});

React.memo is a higher order component. It’s similar to React.PureComponent but for function components instead of classes.

If your function component renders the same result given the same props, you can wrap it in a call to React.memo for a performance boost in some cases by memoizing the result. This means that React will skip rendering the component, and reuse the last rendered result.

By default it will only shallowly compare complex objects in the props object. If you want control over the comparison, you can also provide a custom comparison function as the second argument.

function MyComponent(props) {
  /* render using props */
}
function areEqual(prevProps, nextProps) {
  /*
  return true if passing nextProps to render would return
  the same result as passing prevProps to render,
  otherwise return false
  */
}
export default React.memo(MyComponent, areEqual);

This method only exists as a performance optimization. Do not rely on it to “prevent” a render, as this can lead to bugs.

    Note

    Unlike the shouldComponentUpdate() method on class components, the areEqual function returns true if the props are equal and false if the props are not equal. This is the inverse from shouldComponentUpdate.

createElement()

React.createElement(
  type,
  [props],
  [...children]
)

Create and return a new React element of the given type. The type argument can be either a tag name string (such as 'div' or 'span'), a React component type (a class or a function), or a React fragment type.

Code written with JSX will be converted to use React.createElement(). You will not typically invoke React.createElement() directly if you are using JSX. See React Without JSX to learn more.
cloneElement()

React.cloneElement(
  element,
  [props],
  [...children]
)

Clone and return a new React element using element as the starting point. The resulting element will have the original element’s props with the new props merged in shallowly. New children will replace existing children. key and ref from the original element will be preserved.

React.cloneElement() is almost equivalent to:

<element.type {...element.props} {...props}>{children}</element.type>

However, it also preserves refs. This means that if you get a child with a ref on it, you won’t accidentally steal it from your ancestor. You will get the same ref attached to your new element.

This API was introduced as a replacement of the deprecated React.addons.cloneWithProps().
createFactory()

React.createFactory(type)

Return a function that produces React elements of a given type. Like React.createElement(), the type argument can be either a tag name string (such as 'div' or 'span'), a React component type (a class or a function), or a React fragment type.

This helper is considered legacy, and we encourage you to either use JSX or use React.createElement() directly instead.

You will not typically invoke React.createFactory() directly if you are using JSX. See React Without JSX to learn more.
isValidElement()

React.isValidElement(object)

Verifies the object is a React element. Returns true or false.
React.Children

React.Children provides utilities for dealing with the this.props.children opaque data structure.
React.Children.map

React.Children.map(children, function[(thisArg)])

Invokes a function on every immediate child contained within children with this set to thisArg. If children is an array it will be traversed and the function will be called for each child in the array. If children is null or undefined, this method will return null or undefined rather than an array.

    Note

    If children is a Fragment it will be treated as a single child and not traversed.

React.Children.forEach

React.Children.forEach(children, function[(thisArg)])

Like React.Children.map() but does not return an array.
React.Children.count

React.Children.count(children)

Returns the total number of components in children, equal to the number of times that a callback passed to map or forEach would be invoked.
React.Children.only

React.Children.only(children)

Verifies that children has only one child (a React element) and returns it. Otherwise this method throws an error.

    Note:

    React.Children.only() does not accept the return value of React.Children.map() because it is an array rather than a React element.

React.Children.toArray

React.Children.toArray(children)

Returns the children opaque data structure as a flat array with keys assigned to each child. Useful if you want to manipulate collections of children in your render methods, especially if you want to reorder or slice this.props.children before passing it down.

    Note:

    React.Children.toArray() changes keys to preserve the semantics of nested arrays when flattening lists of children. That is, toArray prefixes each key in the returned array so that each element’s key is scoped to the input array containing it.

React.Fragment

The React.Fragment component lets you return multiple elements in a render() method without creating an additional DOM element:

render() {
  return (
    <React.Fragment>
      Some text.
      <h2>A heading</h2>
    </React.Fragment>
  );
}

You can also use it with the shorthand <></> syntax. For more information, see React v16.2.0: Improved Support for Fragments.
React.createRef

React.createRef creates a ref that can be attached to React elements via the ref attribute.

class MyComponent extends React.Component {
  constructor(props) {
    super(props);

    this.inputRef = React.createRef();
  }

  render() {
    return <input type="text" ref={this.inputRef} />;
  }

  componentDidMount() {
    this.inputRef.current.focus();
  }
}

React.forwardRef

React.forwardRef creates a React component that forwards the ref attribute it receives to another component below in the tree. This technique is not very common but is particularly useful in two scenarios:

    Forwarding refs to DOM components
    Forwarding refs in higher-order-components

React.forwardRef accepts a rendering function as an argument. React will call this function with props and ref as two arguments. This function should return a React node.

const FancyButton = React.forwardRef((props, ref) => (
  <button ref={ref} className="FancyButton">
    {props.children}
  </button>
));

// You can now get a ref directly to the DOM button:
const ref = React.createRef();
<FancyButton ref={ref}>Click me!</FancyButton>;

In the above example, React passes a ref given to <FancyButton ref={ref}> element as a second argument to the rendering function inside the React.forwardRef call. This rendering function passes the ref to the <button ref={ref}> element.

As a result, after React attaches the ref, ref.current will point directly to the <button> DOM element instance.

For more information, see forwarding refs.
React.lazy

React.lazy() lets you define a component that is loaded dynamically. This helps reduce the bundle size to delay loading components that aren’t used during the initial render.

You can learn how to use it from our code splitting documentation. You might also want to check out this article explaining how to use it in more detail.

// This component is loaded dynamically
const SomeComponent = React.lazy(() => import('./SomeComponent'));

Note that rendering lazy components requires that there’s a <React.Suspense> component higher in the rendering tree. This is how you specify a loading indicator.

    Note

    Using React.lazywith dynamic import requires Promises to be available in the JS environment. This requires a polyfill on IE11 and below.

React.Suspense

React.Suspense let you specify the loading indicator in case some components in the tree below it are not yet ready to render. Today, lazy loading components is the only use case supported by <React.Suspense>:

// This component is loaded dynamically
const OtherComponent = React.lazy(() => import('./OtherComponent'));

function MyComponent() {
  return (
    // Displays <Spinner> until OtherComponent loads
    <React.Suspense fallback={<Spinner />}>
      <div>
        <OtherComponent />
      </div>
    </React.Suspense>
  );
}

It is documented in our code splitting guide. Note that lazy components can be deep inside the Suspense tree — it doesn’t have to wrap every one of them. The best practice is to place <Suspense> where you want to see a loading indicator, but to use lazy() wherever you want to do code splitting.

While this is not supported today, in the future we plan to let Suspense handle more scenarios such as data fetching. You can read about this in our roadmap.

    Note:

    React.lazy() and <React.Suspense> are not yet supported by ReactDOMServer. This is a known limitation that will be resolved in the future.

React.Component

This page contains a detailed API reference for the React component class definition. It assumes you’re familiar with fundamental React concepts, such as Components and Props, as well as State and Lifecycle. If you’re not, read them first.
Overview

React lets you define components as classes or functions. Components defined as classes currently provide more features which are described in detail on this page. To define a React component class, you need to extend React.Component:

class Welcome extends React.Component {
  render() {
    return <h1>Hello, {this.props.name}</h1>;
  }
}

The only method you must define in a React.Component subclass is called render(). All the other methods described on this page are optional.

We strongly recommend against creating your own base component classes. In React components, code reuse is primarily achieved through composition rather than inheritance.

    Note:

    React doesn’t force you to use the ES6 class syntax. If you prefer to avoid it, you may use the create-react-class module or a similar custom abstraction instead. Take a look at Using React without ES6 to learn more.

The Component Lifecycle

Each component has several “lifecycle methods” that you can override to run code at particular times in the process. You can use this lifecycle diagram as a cheat sheet. In the list below, commonly used lifecycle methods are marked as bold. The rest of them exist for relatively rare use cases.
Mounting

These methods are called in the following order when an instance of a component is being created and inserted into the DOM:

    constructor()
    static getDerivedStateFromProps()
    render()
    componentDidMount()

    Note:

    These methods are considered legacy and you should avoid them in new code:

        UNSAFE_componentWillMount()

Updating

An update can be caused by changes to props or state. These methods are called in the following order when a component is being re-rendered:

    static getDerivedStateFromProps()
    shouldComponentUpdate()
    render()
    getSnapshotBeforeUpdate()
    componentDidUpdate()

    Note:

    These methods are considered legacy and you should avoid them in new code:

        UNSAFE_componentWillUpdate()
        UNSAFE_componentWillReceiveProps()

Unmounting

This method is called when a component is being removed from the DOM:

    componentWillUnmount()

Error Handling

These methods are called when there is an error during rendering, in a lifecycle method, or in the constructor of any child component.

    static getDerivedStateFromError()
    componentDidCatch()

Other APIs

Each component also provides some other APIs:

    setState()
    forceUpdate()

Class Properties

    defaultProps
    displayName

Instance Properties

    props
    state

Reference
Commonly Used Lifecycle Methods

The methods in this section cover the vast majority of use cases you’ll encounter creating React components. For a visual reference, check out this lifecycle diagram.
render()

render()

The render() method is the only required method in a class component.

When called, it should examine this.props and this.state and return one of the following types:

    React elements. Typically created via JSX. For example, <div /> and <MyComponent /> are React elements that instruct React to render a DOM node, or another user-defined component, respectively.
    Arrays and fragments. Let you return multiple elements from render. See the documentation on fragments for more details.
    Portals. Let you render children into a different DOM subtree. See the documentation on portals for more details.
    String and numbers. These are rendered as text nodes in the DOM.
    Booleans or null. Render nothing. (Mostly exists to support return test && <Child /> pattern, where test is boolean.)

The render() function should be pure, meaning that it does not modify component state, it returns the same result each time it’s invoked, and it does not directly interact with the browser.

If you need to interact with the browser, perform your work in componentDidMount() or the other lifecycle methods instead. Keeping render() pure makes components easier to think about.

    Note

    render() will not be invoked if shouldComponentUpdate() returns false.

constructor()

constructor(props)

If you don’t initialize state and you don’t bind methods, you don’t need to implement a constructor for your React component.

The constructor for a React component is called before it is mounted. When implementing the constructor for a React.Component subclass, you should call super(props) before any other statement. Otherwise, this.props will be undefined in the constructor, which can lead to bugs.

Typically, in React constructors are only used for two purposes:

    Initializing local state by assigning an object to this.state.
    Binding event handler methods to an instance.

You should not call setState() in the constructor(). Instead, if your component needs to use local state, assign the initial state to this.state directly in the constructor:

constructor(props) {
  super(props);
  // Don't call this.setState() here!
  this.state = { counter: 0 };
  this.handleClick = this.handleClick.bind(this);
}

Constructor is the only place where you should assign this.state directly. In all other methods, you need to use this.setState() instead.

Avoid introducing any side-effects or subscriptions in the constructor. For those use cases, use componentDidMount() instead.

    Note

    Avoid copying props into state! This is a common mistake:

    constructor(props) {
     super(props);
     // Don't do this!
     this.state = { color: props.color };
    }

    The problem is that it’s both unnecessary (you can use this.props.color directly instead), and creates bugs (updates to the color prop won’t be reflected in the state).

    Only use this pattern if you intentionally want to ignore prop updates. In that case, it makes sense to rename the prop to be called initialColor or defaultColor. You can then force a component to “reset” its internal state by changing its key when necessary.

    Read our blog post on avoiding derived state to learn about what to do if you think you need some state to depend on the props.

componentDidMount()

componentDidMount()

componentDidMount() is invoked immediately after a component is mounted (inserted into the tree). Initialization that requires DOM nodes should go here. If you need to load data from a remote endpoint, this is a good place to instantiate the network request.

This method is a good place to set up any subscriptions. If you do that, don’t forget to unsubscribe in componentWillUnmount().

You may call setState() immediately in componentDidMount(). It will trigger an extra rendering, but it will happen before the browser updates the screen. This guarantees that even though the render() will be called twice in this case, the user won’t see the intermediate state. Use this pattern with caution because it often causes performance issues. In most cases, you should be able to assign the initial state in the constructor() instead. It can, however, be necessary for cases like modals and tooltips when you need to measure a DOM node before rendering something that depends on its size or position.
componentDidUpdate()

componentDidUpdate(prevProps, prevState, snapshot)

componentDidUpdate() is invoked immediately after updating occurs. This method is not called for the initial render.

Use this as an opportunity to operate on the DOM when the component has been updated. This is also a good place to do network requests as long as you compare the current props to previous props (e.g. a network request may not be necessary if the props have not changed).

componentDidUpdate(prevProps) {
  // Typical usage (don't forget to compare props):
  if (this.props.userID !== prevProps.userID) {
    this.fetchData(this.props.userID);
  }
}

You may call setState() immediately in componentDidUpdate() but note that it must be wrapped in a condition like in the example above, or you’ll cause an infinite loop. It would also cause an extra re-rendering which, while not visible to the user, can affect the component performance. If you’re trying to “mirror” some state to a prop coming from above, consider using the prop directly instead. Read more about why copying props into state causes bugs.

If your component implements the getSnapshotBeforeUpdate() lifecycle (which is rare), the value it returns will be passed as a third “snapshot” parameter to componentDidUpdate(). Otherwise this parameter will be undefined.

    Note

    componentDidUpdate() will not be invoked if shouldComponentUpdate() returns false.

componentWillUnmount()

componentWillUnmount()

componentWillUnmount() is invoked immediately before a component is unmounted and destroyed. Perform any necessary cleanup in this method, such as invalidating timers, canceling network requests, or cleaning up any subscriptions that were created in componentDidMount().

You should not call setState() in componentWillUnmount() because the component will never be re-rendered. Once a component instance is unmounted, it will never be mounted again.
Rarely Used Lifecycle Methods

The methods in this section correspond to uncommon use cases. They’re handy once in a while, but most of your components probably don’t need any of them. You can see most of the methods below on this lifecycle diagram if you click the “Show less common lifecycles” checkbox at the top of it.
shouldComponentUpdate()

shouldComponentUpdate(nextProps, nextState)

Use shouldComponentUpdate() to let React know if a component’s output is not affected by the current change in state or props. The default behavior is to re-render on every state change, and in the vast majority of cases you should rely on the default behavior.

shouldComponentUpdate() is invoked before rendering when new props or state are being received. Defaults to true. This method is not called for the initial render or when forceUpdate() is used.

This method only exists as a performance optimization. Do not rely on it to “prevent” a rendering, as this can lead to bugs. Consider using the built-in PureComponent instead of writing shouldComponentUpdate() by hand. PureComponent performs a shallow comparison of props and state, and reduces the chance that you’ll skip a necessary update.

If you are confident you want to write it by hand, you may compare this.props with nextProps and this.state with nextState and return false to tell React the update can be skipped. Note that returning false does not prevent child components from re-rendering when their state changes.

We do not recommend doing deep equality checks or using JSON.stringify() in shouldComponentUpdate(). It is very inefficient and will harm performance.

Currently, if shouldComponentUpdate() returns false, then UNSAFE_componentWillUpdate(), render(), and componentDidUpdate() will not be invoked. In the future React may treat shouldComponentUpdate() as a hint rather than a strict directive, and returning false may still result in a re-rendering of the component.
static getDerivedStateFromProps()

static getDerivedStateFromProps(props, state)

getDerivedStateFromProps is invoked right before calling the render method, both on the initial mount and on subsequent updates. It should return an object to update the state, or null to update nothing.

This method exists for rare use cases where the state depends on changes in props over time. For example, it might be handy for implementing a <Transition> component that compares its previous and next children to decide which of them to animate in and out.

Deriving state leads to verbose code and makes your components difficult to think about.
Make sure you’re familiar with simpler alternatives:

    If you need to perform a side effect (for example, data fetching or an animation) in response to a change in props, use componentDidUpdate lifecycle instead.

    If you want to re-compute some data only when a prop changes, use a memoization helper instead.

    If you want to “reset” some state when a prop changes, consider either making a component fully controlled or fully uncontrolled with a key instead.

This method doesn’t have access to the component instance. If you’d like, you can reuse some code between getDerivedStateFromProps() and the other class methods by extracting pure functions of the component props and state outside the class definition.

Note that this method is fired on every render, regardless of the cause. This is in contrast to UNSAFE_componentWillReceiveProps, which only fires when the parent causes a re-render and not as a result of a local setState.
getSnapshotBeforeUpdate()

getSnapshotBeforeUpdate(prevProps, prevState)

getSnapshotBeforeUpdate() is invoked right before the most recently rendered output is committed to e.g. the DOM. It enables your component to capture some information from the DOM (e.g. scroll position) before it is potentially changed. Any value returned by this lifecycle will be passed as a parameter to componentDidUpdate().

This use case is not common, but it may occur in UIs like a chat thread that need to handle scroll position in a special way.

A snapshot value (or null) should be returned.

For example:

class ScrollingList extends React.Component {
  constructor(props) {
    super(props);
    this.listRef = React.createRef();
  }

  getSnapshotBeforeUpdate(prevProps, prevState) {
    // Are we adding new items to the list?
    // Capture the scroll position so we can adjust scroll later.
    if (prevProps.list.length < this.props.list.length) {
      const list = this.listRef.current;
      return list.scrollHeight - list.scrollTop;
    }
    return null;
  }

  componentDidUpdate(prevProps, prevState, snapshot) {
    // If we have a snapshot value, we've just added new items.
    // Adjust scroll so these new items don't push the old ones out of view.
    // (snapshot here is the value returned from getSnapshotBeforeUpdate)
    if (snapshot !== null) {
      const list = this.listRef.current;
      list.scrollTop = list.scrollHeight - snapshot;
    }
  }

  render() {
    return (
      <div ref={this.listRef}>{/* ...contents... */}</div>
    );
  }
}

In the above examples, it is important to read the scrollHeight property in getSnapshotBeforeUpdate because there may be delays between “render” phase lifecycles (like render) and “commit” phase lifecycles (like getSnapshotBeforeUpdate and componentDidUpdate).
Error boundaries

Error boundaries are React components that catch JavaScript errors anywhere in their child component tree, log those errors, and display a fallback UI instead of the component tree that crashed. Error boundaries catch errors during rendering, in lifecycle methods, and in constructors of the whole tree below them.

A class component becomes an error boundary if it defines either (or both) of the lifecycle methods static getDerivedStateFromError() or componentDidCatch(). Updating state from these lifecycles lets you capture an unhandled JavaScript error in the below tree and display a fallback UI.

Only use error boundaries for recovering from unexpected exceptions; don’t try to use them for control flow.

For more details, see Error Handling in React 16.

    Note

    Error boundaries only catch errors in the components below them in the tree. An error boundary can’t catch an error within itself.

static getDerivedStateFromError()

static getDerivedStateFromError(error)

This lifecycle is invoked after an error has been thrown by a descendant component. It receives the error that was thrown as a parameter and should return a value to update state.

class ErrorBoundary extends React.Component {
  constructor(props) {
    super(props);
    this.state = { hasError: false };
  }

  static getDerivedStateFromError(error) {
    // Update state so the next render will show the fallback UI.
    return { hasError: true };
  }

  render() {
    if (this.state.hasError) {
      // You can render any custom fallback UI
      return <h1>Something went wrong.</h1>;
    }

    return this.props.children; 
  }
}

    Note

    getDerivedStateFromError() is called during the “render” phase, so side-effects are not permitted. For those use cases, use componentDidCatch() instead.

componentDidCatch()

componentDidCatch(error, info)

This lifecycle is invoked after an error has been thrown by a descendant component. It receives two parameters:

    error - The error that was thrown.
    info - An object with a componentStack key containing information about which component threw the error.

componentDidCatch() is called during the “commit” phase, so side-effects are permitted. It should be used for things like logging errors:

class ErrorBoundary extends React.Component {
  constructor(props) {
    super(props);
    this.state = { hasError: false };
  }

  static getDerivedStateFromError(error) {
    // Update state so the next render will show the fallback UI.
    return { hasError: true };
  }

  componentDidCatch(error, info) {
    // Example "componentStack":
    //   in ComponentThatThrows (created by App)
    //   in ErrorBoundary (created by App)
    //   in div (created by App)
    //   in App
    logComponentStackToMyService(info.componentStack);
  }

  render() {
    if (this.state.hasError) {
      // You can render any custom fallback UI
      return <h1>Something went wrong.</h1>;
    }

    return this.props.children; 
  }
}

    Note

    In the event of an error, you can render a fallback UI with componentDidCatch() by calling setState, but this will be deprecated in a future release. Use static getDerivedStateFromError() to handle fallback rendering instead.

Legacy Lifecycle Methods

The lifecycle methods below are marked as “legacy”. They still work, but we don’t recommend using them in the new code. You can learn more about migrating away from legacy lifecycle methods in this blog post.
UNSAFE_componentWillMount()

UNSAFE_componentWillMount()

    Note

    This lifecycle was previously named componentWillMount. That name will continue to work until version 17. Use the rename-unsafe-lifecycles codemod to automatically update your components.

UNSAFE_componentWillMount() is invoked just before mounting occurs. It is called before render(), therefore calling setState() synchronously in this method will not trigger an extra rendering. Generally, we recommend using the constructor() instead for initializing state.

Avoid introducing any side-effects or subscriptions in this method. For those use cases, use componentDidMount() instead.

This is the only lifecycle method called on server rendering.
UNSAFE_componentWillReceiveProps()

UNSAFE_componentWillReceiveProps(nextProps)

    Note

    This lifecycle was previously named componentWillReceiveProps. That name will continue to work until version 17. Use the rename-unsafe-lifecycles codemod to automatically update your components.

    Note:

    Using this lifecycle method often leads to bugs and inconsistencies

        If you need to perform a side effect (for example, data fetching or an animation) in response to a change in props, use componentDidUpdate lifecycle instead.
        If you used componentWillReceiveProps for re-computing some data only when a prop changes, use a memoization helper instead.
        If you used componentWillReceiveProps to “reset” some state when a prop changes, consider either making a component fully controlled or fully uncontrolled with a key instead.

    For other use cases, follow the recommendations in this blog post about derived state.

UNSAFE_componentWillReceiveProps() is invoked before a mounted component receives new props. If you need to update the state in response to prop changes (for example, to reset it), you may compare this.props and nextProps and perform state transitions using this.setState() in this method.

Note that if a parent component causes your component to re-render, this method will be called even if props have not changed. Make sure to compare the current and next values if you only want to handle changes.

React doesn’t call UNSAFE_componentWillReceiveProps() with initial props during mounting. It only calls this method if some of component’s props may update. Calling this.setState() generally doesn’t trigger UNSAFE_componentWillReceiveProps().
UNSAFE_componentWillUpdate()

UNSAFE_componentWillUpdate(nextProps, nextState)

    Note

    This lifecycle was previously named componentWillUpdate. That name will continue to work until version 17. Use the rename-unsafe-lifecycles codemod to automatically update your components.

UNSAFE_componentWillUpdate() is invoked just before rendering when new props or state are being received. Use this as an opportunity to perform preparation before an update occurs. This method is not called for the initial render.

Note that you cannot call this.setState() here; nor should you do anything else (e.g. dispatch a Redux action) that would trigger an update to a React component before UNSAFE_componentWillUpdate() returns.

Typically, this method can be replaced by componentDidUpdate(). If you were reading from the DOM in this method (e.g. to save a scroll position), you can move that logic to getSnapshotBeforeUpdate().

    Note

    UNSAFE_componentWillUpdate() will not be invoked if shouldComponentUpdate() returns false.

Other APIs

Unlike the lifecycle methods above (which React calls for you), the methods below are the methods you can call from your components.

There are just two of them: setState() and forceUpdate().
setState()

setState(updater[, callback])

setState() enqueues changes to the component state and tells React that this component and its children need to be re-rendered with the updated state. This is the primary method you use to update the user interface in response to event handlers and server responses.

Think of setState() as a request rather than an immediate command to update the component. For better perceived performance, React may delay it, and then update several components in a single pass. React does not guarantee that the state changes are applied immediately.

setState() does not always immediately update the component. It may batch or defer the update until later. This makes reading this.state right after calling setState() a potential pitfall. Instead, use componentDidUpdate or a setState callback (setState(updater, callback)), either of which are guaranteed to fire after the update has been applied. If you need to set the state based on the previous state, read about the updater argument below.

setState() will always lead to a re-render unless shouldComponentUpdate() returns false. If mutable objects are being used and conditional rendering logic cannot be implemented in shouldComponentUpdate(), calling setState() only when the new state differs from the previous state will avoid unnecessary re-renders.

The first argument is an updater function with the signature:

(state, props) => stateChange

state is a reference to the component state at the time the change is being applied. It should not be directly mutated. Instead, changes should be represented by building a new object based on the input from state and props. For instance, suppose we wanted to increment a value in state by props.step:

this.setState((state, props) => {
  return {counter: state.counter + props.step};
});

Both state and props received by the updater function are guaranteed to be up-to-date. The output of the updater is shallowly merged with state.

The second parameter to setState() is an optional callback function that will be executed once setState is completed and the component is re-rendered. Generally we recommend using componentDidUpdate() for such logic instead.

You may optionally pass an object as the first argument to setState() instead of a function:

setState(stateChange[, callback])

This performs a shallow merge of stateChange into the new state, e.g., to adjust a shopping cart item quantity:

this.setState({quantity: 2})

This form of setState() is also asynchronous, and multiple calls during the same cycle may be batched together. For example, if you attempt to increment an item quantity more than once in the same cycle, that will result in the equivalent of:

Object.assign(
  previousState,
  {quantity: state.quantity + 1},
  {quantity: state.quantity + 1},
  ...
)

Subsequent calls will override values from previous calls in the same cycle, so the quantity will only be incremented once. If the next state depends on the current state, we recommend using the updater function form, instead:

this.setState((state) => {
  return {quantity: state.quantity + 1};
});

For more detail, see:

    State and Lifecycle guide
    In depth: When and why are setState() calls batched?
    In depth: Why isn’t this.state updated immediately?

forceUpdate()

component.forceUpdate(callback)

By default, when your component’s state or props change, your component will re-render. If your render() method depends on some other data, you can tell React that the component needs re-rendering by calling forceUpdate().

Calling forceUpdate() will cause render() to be called on the component, skipping shouldComponentUpdate(). This will trigger the normal lifecycle methods for child components, including the shouldComponentUpdate() method of each child. React will still only update the DOM if the markup changes.

Normally you should try to avoid all uses of forceUpdate() and only read from this.props and this.state in render().
Class Properties
defaultProps

defaultProps can be defined as a property on the component class itself, to set the default props for the class. This is used for undefined props, but not for null props. For example:

class CustomButton extends React.Component {
  // ...
}

CustomButton.defaultProps = {
  color: 'blue'
};

If props.color is not provided, it will be set by default to 'blue':

  render() {
    return <CustomButton /> ; // props.color will be set to blue
  }

If props.color is set to null, it will remain null:

  render() {
    return <CustomButton color={null} /> ; // props.color will remain null
  }

displayName

The displayName string is used in debugging messages. Usually, you don’t need to set it explicitly because it’s inferred from the name of the function or class that defines the component. You might want to set it explicitly if you want to display a different name for debugging purposes or when you create a higher-order component, see Wrap the Display Name for Easy Debugging for details.
Instance Properties
props

this.props contains the props that were defined by the caller of this component. See Components and Props for an introduction to props.

In particular, this.props.children is a special prop, typically defined by the child tags in the JSX expression rather than in the tag itself.
state

The state contains data specific to this component that may change over time. The state is user-defined, and it should be a plain JavaScript object.

If some value isn’t used for rendering or data flow (for example, a timer ID), you don’t have to put it in the state. Such values can be defined as fields on the component instance.

See State and Lifecycle for more information about the state.

Never mutate this.state directly, as calling setState() afterwards may replace the mutation you made. Treat this.state as if it were immutable.
ReactDOM

If you load React from a <script> tag, these top-level APIs are available on the ReactDOM global. If you use ES6 with npm, you can write import ReactDOM from 'react-dom'. If you use ES5 with npm, you can write var ReactDOM = require('react-dom').
Overview

The react-dom package provides DOM-specific methods that can be used at the top level of your app and as an escape hatch to get outside of the React model if you need to. Most of your components should not need to use this module.

    render()
    hydrate()
    unmountComponentAtNode()
    findDOMNode()
    createPortal()

Browser Support

React supports all popular browsers, including Internet Explorer 9 and above, although some polyfills are required for older browsers such as IE 9 and IE 10.

    Note

    We don’t support older browsers that don’t support ES5 methods, but you may find that your apps do work in older browsers if polyfills such as es5-shim and es5-sham are included in the page. You’re on your own if you choose to take this path.

Reference
render()

ReactDOM.render(element, container[, callback])

Render a React element into the DOM in the supplied container and return a reference to the component (or returns null for stateless components).

If the React element was previously rendered into container, this will perform an update on it and only mutate the DOM as necessary to reflect the latest React element.

If the optional callback is provided, it will be executed after the component is rendered or updated.

    Note:

    ReactDOM.render() controls the contents of the container node you pass in. Any existing DOM elements inside are replaced when first called. Later calls use React’s DOM diffing algorithm for efficient updates.

    ReactDOM.render() does not modify the container node (only modifies the children of the container). It may be possible to insert a component to an existing DOM node without overwriting the existing children.

    ReactDOM.render() currently returns a reference to the root ReactComponent instance. However, using this return value is legacy and should be avoided because future versions of React may render components asynchronously in some cases. If you need a reference to the root ReactComponent instance, the preferred solution is to attach a callback ref to the root element.

    Using ReactDOM.render() to hydrate a server-rendered container is deprecated and will be removed in React 17. Use hydrate() instead.

hydrate()

ReactDOM.hydrate(element, container[, callback])

Same as render(), but is used to hydrate a container whose HTML contents were rendered by ReactDOMServer. React will attempt to attach event listeners to the existing markup.

React expects that the rendered content is identical between the server and the client. It can patch up differences in text content, but you should treat mismatches as bugs and fix them. In development mode, React warns about mismatches during hydration. There are no guarantees that attribute differences will be patched up in case of mismatches. This is important for performance reasons because in most apps, mismatches are rare, and so validating all markup would be prohibitively expensive.

If a single element’s attribute or text content is unavoidably different between the server and the client (for example, a timestamp), you may silence the warning by adding suppressHydrationWarning={true} to the element. It only works one level deep, and is intended to be an escape hatch. Don’t overuse it. Unless it’s text content, React still won’t attempt to patch it up, so it may remain inconsistent until future updates.

If you intentionally need to render something different on the server and the client, you can do a two-pass rendering. Components that render something different on the client can read a state variable like this.state.isClient, which you can set to true in componentDidMount(). This way the initial render pass will render the same content as the server, avoiding mismatches, but an additional pass will happen synchronously right after hydration. Note that this approach will make your components slower because they have to render twice, so use it with caution.

Remember to be mindful of user experience on slow connections. The JavaScript code may load significantly later than the initial HTML render, so if you render something different in the client-only pass, the transition can be jarring. However, if executed well, it may be beneficial to render a “shell” of the application on the server, and only show some of the extra widgets on the client. To learn how to do this without getting the markup mismatch issues, refer to the explanation in the previous paragraph.
unmountComponentAtNode()

ReactDOM.unmountComponentAtNode(container)

Remove a mounted React component from the DOM and clean up its event handlers and state. If no component was mounted in the container, calling this function does nothing. Returns true if a component was unmounted and false if there was no component to unmount.
findDOMNode()

    Note:

    findDOMNode is an escape hatch used to access the underlying DOM node. In most cases, use of this escape hatch is discouraged because it pierces the component abstraction. It has been deprecated in StrictMode.

ReactDOM.findDOMNode(component)

If this component has been mounted into the DOM, this returns the corresponding native browser DOM element. This method is useful for reading values out of the DOM, such as form field values and performing DOM measurements. In most cases, you can attach a ref to the DOM node and avoid using findDOMNode at all.

When a component renders to null or false, findDOMNode returns null. When a component renders to a string, findDOMNode returns a text DOM node containing that value. As of React 16, a component may return a fragment with multiple children, in which case findDOMNode will return the DOM node corresponding to the first non-empty child.

    Note:

    findDOMNode only works on mounted components (that is, components that have been placed in the DOM). If you try to call this on a component that has not been mounted yet (like calling findDOMNode() in render() on a component that has yet to be created) an exception will be thrown.

    findDOMNode cannot be used on function components.

createPortal()

ReactDOM.createPortal(child, container)

Creates a portal. Portals provide a way to render children into a DOM node that exists outside the hierarchy of the DOM component.
ReactDOMServer

The ReactDOMServer object enables you to render components to static markup. Typically, it’s used on a Node server:

// ES modules
import ReactDOMServer from 'react-dom/server';
// CommonJS
var ReactDOMServer = require('react-dom/server');

Overview

The following methods can be used in both the server and browser environments:

    renderToString()
    renderToStaticMarkup()

These additional methods depend on a package (stream) that is only available on the server, and won’t work in the browser.

    renderToNodeStream()
    renderToStaticNodeStream()

Reference
renderToString()

ReactDOMServer.renderToString(element)

Render a React element to its initial HTML. React will return an HTML string. You can use this method to generate HTML on the server and send the markup down on the initial request for faster page loads and to allow search engines to crawl your pages for SEO purposes.

If you call ReactDOM.hydrate() on a node that already has this server-rendered markup, React will preserve it and only attach event handlers, allowing you to have a very performant first-load experience.
renderToStaticMarkup()

ReactDOMServer.renderToStaticMarkup(element)

Similar to renderToString, except this doesn’t create extra DOM attributes that React uses internally, such as data-reactroot. This is useful if you want to use React as a simple static page generator, as stripping away the extra attributes can save some bytes.

If you plan to use React on the client to make the markup interactive, do not use this method. Instead, use renderToString on the server and ReactDOM.hydrate() on the client.
renderToNodeStream()

ReactDOMServer.renderToNodeStream(element)

Render a React element to its initial HTML. Returns a Readable stream that outputs an HTML string. The HTML output by this stream is exactly equal to what ReactDOMServer.renderToString would return. You can use this method to generate HTML on the server and send the markup down on the initial request for faster page loads and to allow search engines to crawl your pages for SEO purposes.

If you call ReactDOM.hydrate() on a node that already has this server-rendered markup, React will preserve it and only attach event handlers, allowing you to have a very performant first-load experience.

    Note:

    Server-only. This API is not available in the browser.

    The stream returned from this method will return a byte stream encoded in utf-8. If you need a stream in another encoding, take a look at a project like iconv-lite, which provides transform streams for transcoding text.

renderToStaticNodeStream()

ReactDOMServer.renderToStaticNodeStream(element)

Similar to renderToNodeStream, except this doesn’t create extra DOM attributes that React uses internally, such as data-reactroot. This is useful if you want to use React as a simple static page generator, as stripping away the extra attributes can save some bytes.

The HTML output by this stream is exactly equal to what ReactDOMServer.renderToStaticMarkup would return.

If you plan to use React on the client to make the markup interactive, do not use this method. Instead, use renderToNodeStream on the server and ReactDOM.hydrate() on the client.

    Note:

    Server-only. This API is not available in the browser.

    The stream returned from this method will return a byte stream encoded in utf-8. If you need a stream in another encoding, take a look at a project like iconv-lite, which provides transform streams for transcoding text.

DOM Elements

React implements a browser-independent DOM system for performance and cross-browser compatibility. We took the opportunity to clean up a few rough edges in browser DOM implementations.

In React, all DOM properties and attributes (including event handlers) should be camelCased. For example, the HTML attribute tabindex corresponds to the attribute tabIndex in React. The exception is aria-* and data-* attributes, which should be lowercased. For example, you can keep aria-label as aria-label.
Differences In Attributes

There are a number of attributes that work differently between React and HTML:
checked

The checked attribute is supported by <input> components of type checkbox or radio. You can use it to set whether the component is checked. This is useful for building controlled components. defaultChecked is the uncontrolled equivalent, which sets whether the component is checked when it is first mounted.
className

To specify a CSS class, use the className attribute. This applies to all regular DOM and SVG elements like <div>, <a>, and others.

If you use React with Web Components (which is uncommon), use the class attribute instead.
dangerouslySetInnerHTML

dangerouslySetInnerHTML is React’s replacement for using innerHTML in the browser DOM. In general, setting HTML from code is risky because it’s easy to inadvertently expose your users to a cross-site scripting (XSS) attack. So, you can set HTML directly from React, but you have to type out dangerouslySetInnerHTML and pass an object with a __html key, to remind yourself that it’s dangerous. For example:

function createMarkup() {
  return {__html: 'First &middot; Second'};
}

function MyComponent() {
  return <div dangerouslySetInnerHTML={createMarkup()} />;
}

htmlFor

Since for is a reserved word in JavaScript, React elements use htmlFor instead.
onChange

The onChange event behaves as you would expect it to: whenever a form field is changed, this event is fired. We intentionally do not use the existing browser behavior because onChange is a misnomer for its behavior and React relies on this event to handle user input in real time.
selected

The selected attribute is supported by <option> components. You can use it to set whether the component is selected. This is useful for building controlled components.
style

    Note

    Some examples in the documentation use style for convenience, but using the style attribute as the primary means of styling elements is generally not recommended. In most cases, className should be used to reference classes defined in an external CSS stylesheet. style is most often used in React applications to add dynamically-computed styles at render time. See also FAQ: Styling and CSS.

The style attribute accepts a JavaScript object with camelCased properties rather than a CSS string. This is consistent with the DOM style JavaScript property, is more efficient, and prevents XSS security holes. For example:

const divStyle = {
  color: 'blue',
  backgroundImage: 'url(' + imgUrl + ')',
};

function HelloWorldComponent() {
  return <div style={divStyle}>Hello World!</div>;
}

Note that styles are not autoprefixed. To support older browsers, you need to supply corresponding style properties:

const divStyle = {
  WebkitTransition: 'all', // note the capital 'W' here
  msTransition: 'all' // 'ms' is the only lowercase vendor prefix
};

function ComponentWithTransition() {
  return <div style={divStyle}>This should work cross-browser</div>;
}

Style keys are camelCased in order to be consistent with accessing the properties on DOM nodes from JS (e.g. node.style.backgroundImage). Vendor prefixes other than ms should begin with a capital letter. This is why WebkitTransition has an uppercase “W”.

React will automatically append a “px” suffix to certain numeric inline style properties. If you want to use units other than “px”, specify the value as a string with the desired unit. For example:

// Result style: '10px'
<div style={{ height: 10 }}>
  Hello World!
</div>

// Result style: '10%'
<div style={{ height: '10%' }}>
  Hello World!
</div>

Not all style properties are converted to pixel strings though. Certain ones remain unitless (eg zoom, order, flex). A complete list of unitless properties can be seen here.
suppressContentEditableWarning

Normally, there is a warning when an element with children is also marked as contentEditable, because it won’t work. This attribute suppresses that warning. Don’t use this unless you are building a library like Draft.js that manages contentEditable manually.
suppressHydrationWarning

If you use server-side React rendering, normally there is a warning when the server and the client render different content. However, in some rare cases, it is very hard or impossible to guarantee an exact match. For example, timestamps are expected to differ on the server and on the client.

If you set suppressHydrationWarning to true, React will not warn you about mismatches in the attributes and the content of that element. It only works one level deep, and is intended to be used as an escape hatch. Don’t overuse it. You can read more about hydration in the ReactDOM.hydrate() documentation.
value

The value attribute is supported by <input> and <textarea> components. You can use it to set the value of the component. This is useful for building controlled components. defaultValue is the uncontrolled equivalent, which sets the value of the component when it is first mounted.
All Supported HTML Attributes

As of React 16, any standard or custom DOM attributes are fully supported.

React has always provided a JavaScript-centric API to the DOM. Since React components often take both custom and DOM-related props, React uses the camelCase convention just like the DOM APIs:

<div tabIndex="-1" />      // Just like node.tabIndex DOM API
<div className="Button" /> // Just like node.className DOM API
<input readOnly={true} />  // Just like node.readOnly DOM API

These props work similarly to the corresponding HTML attributes, with the exception of the special cases documented above.

Some of the DOM attributes supported by React include:

accept acceptCharset accessKey action allowFullScreen alt async autoComplete
autoFocus autoPlay capture cellPadding cellSpacing challenge charSet checked
cite classID className colSpan cols content contentEditable contextMenu controls
controlsList coords crossOrigin data dateTime default defer dir disabled
download draggable encType form formAction formEncType formMethod formNoValidate
formTarget frameBorder headers height hidden high href hrefLang htmlFor
httpEquiv icon id inputMode integrity is keyParams keyType kind label lang list
loop low manifest marginHeight marginWidth max maxLength media mediaGroup method
min minLength multiple muted name noValidate nonce open optimum pattern
placeholder poster preload profile radioGroup readOnly rel required reversed
role rowSpan rows sandbox scope scoped scrolling seamless selected shape size
sizes span spellCheck src srcDoc srcLang srcSet start step style summary
tabIndex target title type useMap value width wmode wrap

Similarly, all SVG attributes are fully supported:

accentHeight accumulate additive alignmentBaseline allowReorder alphabetic
amplitude arabicForm ascent attributeName attributeType autoReverse azimuth
baseFrequency baseProfile baselineShift bbox begin bias by calcMode capHeight
clip clipPath clipPathUnits clipRule colorInterpolation
colorInterpolationFilters colorProfile colorRendering contentScriptType
contentStyleType cursor cx cy d decelerate descent diffuseConstant direction
display divisor dominantBaseline dur dx dy edgeMode elevation enableBackground
end exponent externalResourcesRequired fill fillOpacity fillRule filter
filterRes filterUnits floodColor floodOpacity focusable fontFamily fontSize
fontSizeAdjust fontStretch fontStyle fontVariant fontWeight format from fx fy
g1 g2 glyphName glyphOrientationHorizontal glyphOrientationVertical glyphRef
gradientTransform gradientUnits hanging horizAdvX horizOriginX ideographic
imageRendering in in2 intercept k k1 k2 k3 k4 kernelMatrix kernelUnitLength
kerning keyPoints keySplines keyTimes lengthAdjust letterSpacing lightingColor
limitingConeAngle local markerEnd markerHeight markerMid markerStart
markerUnits markerWidth mask maskContentUnits maskUnits mathematical mode
numOctaves offset opacity operator order orient orientation origin overflow
overlinePosition overlineThickness paintOrder panose1 pathLength
patternContentUnits patternTransform patternUnits pointerEvents points
pointsAtX pointsAtY pointsAtZ preserveAlpha preserveAspectRatio primitiveUnits
r radius refX refY renderingIntent repeatCount repeatDur requiredExtensions
requiredFeatures restart result rotate rx ry scale seed shapeRendering slope
spacing specularConstant specularExponent speed spreadMethod startOffset
stdDeviation stemh stemv stitchTiles stopColor stopOpacity
strikethroughPosition strikethroughThickness string stroke strokeDasharray
strokeDashoffset strokeLinecap strokeLinejoin strokeMiterlimit strokeOpacity
strokeWidth surfaceScale systemLanguage tableValues targetX targetY textAnchor
textDecoration textLength textRendering to transform u1 u2 underlinePosition
underlineThickness unicode unicodeBidi unicodeRange unitsPerEm vAlphabetic
vHanging vIdeographic vMathematical values vectorEffect version vertAdvY
vertOriginX vertOriginY viewBox viewTarget visibility widths wordSpacing
writingMode x x1 x2 xChannelSelector xHeight xlinkActuate xlinkArcrole
xlinkHref xlinkRole xlinkShow xlinkTitle xlinkType xmlns xmlnsXlink xmlBase
xmlLang xmlSpace y y1 y2 yChannelSelector z zoomAndPan

You may also use custom attributes as long as they’re fully lowercase.
SyntheticEvent

This reference guide documents the SyntheticEvent wrapper that forms part of React’s Event System. See the Handling Events guide to learn more.
Overview

Your event handlers will be passed instances of SyntheticEvent, a cross-browser wrapper around the browser’s native event. It has the same interface as the browser’s native event, including stopPropagation() and preventDefault(), except the events work identically across all browsers.

If you find that you need the underlying browser event for some reason, simply use the nativeEvent attribute to get it. Every SyntheticEvent object has the following attributes:

boolean bubbles
boolean cancelable
DOMEventTarget currentTarget
boolean defaultPrevented
number eventPhase
boolean isTrusted
DOMEvent nativeEvent
void preventDefault()
boolean isDefaultPrevented()
void stopPropagation()
boolean isPropagationStopped()
DOMEventTarget target
number timeStamp
string type

    Note:

    As of v0.14, returning false from an event handler will no longer stop event propagation. Instead, e.stopPropagation() or e.preventDefault() should be triggered manually, as appropriate.

Event Pooling

The SyntheticEvent is pooled. This means that the SyntheticEvent object will be reused and all properties will be nullified after the event callback has been invoked. This is for performance reasons. As such, you cannot access the event in an asynchronous way.

function onClick(event) {
  console.log(event); // => nullified object.
  console.log(event.type); // => "click"
  const eventType = event.type; // => "click"

  setTimeout(function() {
    console.log(event.type); // => null
    console.log(eventType); // => "click"
  }, 0);

  // Won't work. this.state.clickEvent will only contain null values.
  this.setState({clickEvent: event});

  // You can still export event properties.
  this.setState({eventType: event.type});
}

    Note:

    If you want to access the event properties in an asynchronous way, you should call event.persist() on the event, which will remove the synthetic event from the pool and allow references to the event to be retained by user code.

Supported Events

React normalizes events so that they have consistent properties across different browsers.

The event handlers below are triggered by an event in the bubbling phase. To register an event handler for the capture phase, append Capture to the event name; for example, instead of using onClick, you would use onClickCapture to handle the click event in the capture phase.

    Clipboard Events
    Composition Events
    Keyboard Events
    Focus Events
    Form Events
    Mouse Events
    Pointer Events
    Selection Events
    Touch Events
    UI Events
    Wheel Events
    Media Events
    Image Events
    Animation Events
    Transition Events
    Other Events

Reference
Clipboard Events

Event names:

onCopy onCut onPaste

Properties:

DOMDataTransfer clipboardData

Composition Events

Event names:

onCompositionEnd onCompositionStart onCompositionUpdate

Properties:

string data

Keyboard Events

Event names:

onKeyDown onKeyPress onKeyUp

Properties:

boolean altKey
number charCode
boolean ctrlKey
boolean getModifierState(key)
string key
number keyCode
string locale
number location
boolean metaKey
boolean repeat
boolean shiftKey
number which

The key property can take any of the values documented in the DOM Level 3 Events spec.
Focus Events

Event names:

onFocus onBlur

These focus events work on all elements in the React DOM, not just form elements.

Properties:

DOMEventTarget relatedTarget

Form Events

Event names:

onChange onInput onInvalid onSubmit

For more information about the onChange event, see Forms.
Mouse Events

Event names:

onClick onContextMenu onDoubleClick onDrag onDragEnd onDragEnter onDragExit
onDragLeave onDragOver onDragStart onDrop onMouseDown onMouseEnter onMouseLeave
onMouseMove onMouseOut onMouseOver onMouseUp

The onMouseEnter and onMouseLeave events propagate from the element being left to the one being entered instead of ordinary bubbling and do not have a capture phase.

Properties:

boolean altKey
number button
number buttons
number clientX
number clientY
boolean ctrlKey
boolean getModifierState(key)
boolean metaKey
number pageX
number pageY
DOMEventTarget relatedTarget
number screenX
number screenY
boolean shiftKey

Pointer Events

Event names:

onPointerDown onPointerMove onPointerUp onPointerCancel onGotPointerCapture
onLostPointerCapture onPointerEnter onPointerLeave onPointerOver onPointerOut

The onPointerEnter and onPointerLeave events propagate from the element being left to the one being entered instead of ordinary bubbling and do not have a capture phase.

Properties:

As defined in the W3 spec, pointer events extend Mouse Events with the following properties:

number pointerId
number width
number height
number pressure
number tangentialPressure
number tiltX
number tiltY
number twist
string pointerType
boolean isPrimary

A note on cross-browser support:

Pointer events are not yet supported in every browser (at the time of writing this article, supported browsers include: Chrome, Firefox, Edge, and Internet Explorer). React deliberately does not polyfill support for other browsers because a standard-conform polyfill would significantly increase the bundle size of react-dom.

If your application requires pointer events, we recommend adding a third party pointer event polyfill.
Selection Events

Event names:

onSelect

Touch Events

Event names:

onTouchCancel onTouchEnd onTouchMove onTouchStart

Properties:

boolean altKey
DOMTouchList changedTouches
boolean ctrlKey
boolean getModifierState(key)
boolean metaKey
boolean shiftKey
DOMTouchList targetTouches
DOMTouchList touches

UI Events

Event names:

onScroll

Properties:

number detail
DOMAbstractView view

Wheel Events

Event names:

onWheel

Properties:

number deltaMode
number deltaX
number deltaY
number deltaZ

Media Events

Event names:

onAbort onCanPlay onCanPlayThrough onDurationChange onEmptied onEncrypted
onEnded onError onLoadedData onLoadedMetadata onLoadStart onPause onPlay
onPlaying onProgress onRateChange onSeeked onSeeking onStalled onSuspend
onTimeUpdate onVolumeChange onWaiting

Image Events

Event names:

onLoad onError

Animation Events

Event names:

onAnimationStart onAnimationEnd onAnimationIteration

Properties:

string animationName
string pseudoElement
float elapsedTime

Transition Events

Event names:

onTransitionEnd

Properties:

string propertyName
string pseudoElement
float elapsedTime

Other Events

Event names:

onToggle

Importing

import ReactTestUtils from 'react-dom/test-utils'; // ES6
var ReactTestUtils = require('react-dom/test-utils'); // ES5 with npm

Overview

ReactTestUtils makes it easy to test React components in the testing framework of your choice. At Facebook we use Jest for painless JavaScript testing. Learn how to get started with Jest through the Jest website’s React Tutorial.

    Note:

    We recommend using React Testing Library which is designed to enable and encourage writing tests that use your components as the end users do.

    Alternatively, Airbnb has released a testing utility called Enzyme, which makes it easy to assert, manipulate, and traverse your React Components’ output.

    act()
    mockComponent()
    isElement()
    isElementOfType()
    isDOMComponent()
    isCompositeComponent()
    isCompositeComponentWithType()
    findAllInRenderedTree()
    scryRenderedDOMComponentsWithClass()
    findRenderedDOMComponentWithClass()
    scryRenderedDOMComponentsWithTag()
    findRenderedDOMComponentWithTag()
    scryRenderedComponentsWithType()
    findRenderedComponentWithType()
    renderIntoDocument()
    Simulate

Reference
act()

To prepare a component for assertions, wrap the code rendering it and performing updates inside an act() call. This makes your test run closer to how React works in the browser.

    Note

    If you use react-test-renderer, it also provides an act export that behaves the same way.

For example, let’s say we have this Counter component:

class Counter extends React.Component {
  constructor(props) {
    super(props);
    this.state = {count: 0};
    this.handleClick = this.handleClick.bind(this);
  }
  componentDidMount() {
    document.title = `You clicked ${this.state.count} times`;
  }
  componentDidUpdate() {
    document.title = `You clicked ${this.state.count} times`;
  }
  handleClick() {
    this.setState(state => ({
      count: state.count + 1,
    }));
  }
  render() {
    return (
      <div>
        <p>You clicked {this.state.count} times</p>
        <button onClick={this.handleClick}>
          Click me
        </button>
      </div>
    );
  }
}

Here is how we can test it:

import React from 'react';
import ReactDOM from 'react-dom';
import { act } from 'react-dom/test-utils';
import Counter from './Counter';

let container;

beforeEach(() => {
  container = document.createElement('div');
  document.body.appendChild(container);
});

afterEach(() => {
  document.body.removeChild(container);
  container = null;
});

it('can render and update a counter', () => {
  // Test first render and componentDidMount
  act(() => {
    ReactDOM.render(<Counter />, container);
  });
  const button = container.querySelector('button');
  const label = container.querySelector('p');
  expect(label.textContent).toBe('You clicked 0 times');
  expect(document.title).toBe('You clicked 0 times');

  // Test second render and componentDidUpdate
  act(() => {
    button.dispatchEvent(new MouseEvent('click', {bubbles: true}));
  });
  expect(label.textContent).toBe('You clicked 1 times');
  expect(document.title).toBe('You clicked 1 times');
});

Don’t forget that dispatching DOM events only works when the DOM container is added to the document. You can use a helper like react-testing-library to reduce the boilerplate code.
mockComponent()

mockComponent(
  componentClass,
  [mockTagName]
)

Pass a mocked component module to this method to augment it with useful methods that allow it to be used as a dummy React component. Instead of rendering as usual, the component will become a simple <div> (or other tag if mockTagName is provided) containing any provided children.

    Note:

    mockComponent() is a legacy API. We recommend using shallow rendering or jest.mock() instead.

isElement()

isElement(element)

Returns true if element is any React element.
isElementOfType()

isElementOfType(
  element,
  componentClass
)

Returns true if element is a React element whose type is of a React componentClass.
isDOMComponent()

isDOMComponent(instance)

Returns true if instance is a DOM component (such as a <div> or <span>).
isCompositeComponent()

isCompositeComponent(instance)

Returns true if instance is a user-defined component, such as a class or a function.
isCompositeComponentWithType()

isCompositeComponentWithType(
  instance,
  componentClass
)

Returns true if instance is a component whose type is of a React componentClass.
findAllInRenderedTree()

findAllInRenderedTree(
  tree,
  test
)

Traverse all components in tree and accumulate all components where test(component) is true. This is not that useful on its own, but it’s used as a primitive for other test utils.
scryRenderedDOMComponentsWithClass()

scryRenderedDOMComponentsWithClass(
  tree,
  className
)

Finds all DOM elements of components in the rendered tree that are DOM components with the class name matching className.
findRenderedDOMComponentWithClass()

findRenderedDOMComponentWithClass(
  tree,
  className
)

Like scryRenderedDOMComponentsWithClass() but expects there to be one result, and returns that one result, or throws exception if there is any other number of matches besides one.
scryRenderedDOMComponentsWithTag()

scryRenderedDOMComponentsWithTag(
  tree,
  tagName
)

Finds all DOM elements of components in the rendered tree that are DOM components with the tag name matching tagName.
findRenderedDOMComponentWithTag()

findRenderedDOMComponentWithTag(
  tree,
  tagName
)

Like scryRenderedDOMComponentsWithTag() but expects there to be one result, and returns that one result, or throws exception if there is any other number of matches besides one.
scryRenderedComponentsWithType()

scryRenderedComponentsWithType(
  tree,
  componentClass
)

Finds all instances of components with type equal to componentClass.
findRenderedComponentWithType()

findRenderedComponentWithType(
  tree,
  componentClass
)

Same as scryRenderedComponentsWithType() but expects there to be one result and returns that one result, or throws exception if there is any other number of matches besides one.
renderIntoDocument()

renderIntoDocument(element)

Render a React element into a detached DOM node in the document. This function requires a DOM. It is effectively equivalent to:

const domContainer = document.createElement('div');
ReactDOM.render(element, domContainer);

    Note:

    You will need to have window, window.document and window.document.createElement globally available before you import React. Otherwise React will think it can’t access the DOM and methods like setState won’t work.

Other Utilities
Simulate

Simulate.{eventName}(
  element,
  [eventData]
)

Simulate an event dispatch on a DOM node with optional eventData event data.

Simulate has a method for every event that React understands.

Clicking an element

// <button ref={(node) => this.button = node}>...</button>
const node = this.button;
ReactTestUtils.Simulate.click(node);

Changing the value of an input field and then pressing ENTER.

// <input ref={(node) => this.textInput = node} />
const node = this.textInput;
node.value = 'giraffe';
ReactTestUtils.Simulate.change(node);
ReactTestUtils.Simulate.keyDown(node, {key: "Enter", keyCode: 13, which: 13});

    Note

    You will have to provide any event property that you’re using in your component (e.g. keyCode, which, etc…) as React is not creating any of these for you.

Shallow Renderer

Importing

import ShallowRenderer from 'react-test-renderer/shallow'; // ES6
var ShallowRenderer = require('react-test-renderer/shallow'); // ES5 with npm

Overview

When writing unit tests for React, shallow rendering can be helpful. Shallow rendering lets you render a component “one level deep” and assert facts about what its render method returns, without worrying about the behavior of child components, which are not instantiated or rendered. This does not require a DOM.

For example, if you have the following component:

function MyComponent() {
  return (
    <div>
      <span className="heading">Title</span>
      <Subcomponent foo="bar" />
    </div>
  );
}

Then you can assert:

import ShallowRenderer from 'react-test-renderer/shallow';

// in your test:
const renderer = new ShallowRenderer();
renderer.render(<MyComponent />);
const result = renderer.getRenderOutput();

expect(result.type).toBe('div');
expect(result.props.children).toEqual([
  <span className="heading">Title</span>,
  <Subcomponent foo="bar" />
]);

Shallow testing currently has some limitations, namely not supporting refs.

    Note:

    We also recommend checking out Enzyme’s Shallow Rendering API. It provides a nicer higher-level API over the same functionality.

Reference
shallowRenderer.render()

You can think of the shallowRenderer as a “place” to render the component you’re testing, and from which you can extract the component’s output.

shallowRenderer.render() is similar to ReactDOM.render() but it doesn’t require DOM and only renders a single level deep. This means you can test components isolated from how their children are implemented.
shallowRenderer.getRenderOutput()

After shallowRenderer.render() has been called, you can use shallowRenderer.getRenderOutput() to get the shallowly rendered output.

You can then begin to assert facts about the output.
Importing

import TestRenderer from 'react-test-renderer'; // ES6
const TestRenderer = require('react-test-renderer'); // ES5 with npm

Overview

This package provides a React renderer that can be used to render React components to pure JavaScript objects, without depending on the DOM or a native mobile environment.

Essentially, this package makes it easy to grab a snapshot of the platform view hierarchy (similar to a DOM tree) rendered by a React DOM or React Native component without using a browser or jsdom.

Example:

import TestRenderer from 'react-test-renderer';

function Link(props) {
  return <a href={props.page}>{props.children}</a>;
}

const testRenderer = TestRenderer.create(
  <Link page="https://www.facebook.com/">Facebook</Link>
);

console.log(testRenderer.toJSON());
// { type: 'a',
//   props: { href: 'https://www.facebook.com/' },
//   children: [ 'Facebook' ] }

You can use Jest’s snapshot testing feature to automatically save a copy of the JSON tree to a file and check in your tests that it hasn’t changed: Learn more about it.

You can also traverse the output to find specific nodes and make assertions about them.

import TestRenderer from 'react-test-renderer';

function MyComponent() {
  return (
    <div>
      <SubComponent foo="bar" />
      <p className="my">Hello</p>
    </div>
  )
}

function SubComponent() {
  return (
    <p className="sub">Sub</p>
  );
}

const testRenderer = TestRenderer.create(<MyComponent />);
const testInstance = testRenderer.root;

expect(testInstance.findByType(SubComponent).props.foo).toBe('bar');
expect(testInstance.findByProps({className: "sub"}).children).toEqual(['Sub']);

TestRenderer

    TestRenderer.create()

TestRenderer instance

    testRenderer.toJSON()
    testRenderer.toTree()
    testRenderer.update()
    testRenderer.unmount()
    testRenderer.getInstance()
    testRenderer.root

TestInstance

    testInstance.find()
    testInstance.findByType()
    testInstance.findByProps()
    testInstance.findAll()
    testInstance.findAllByType()
    testInstance.findAllByProps()
    testInstance.instance
    testInstance.type
    testInstance.props
    testInstance.parent
    testInstance.children

Reference
TestRenderer.create()

TestRenderer.create(element, options);

Create a TestRenderer instance with the passed React element. It doesn’t use the real DOM, but it still fully renders the component tree into memory so you can make assertions about it. The returned instance has the following methods and properties.
testRenderer.toJSON()

testRenderer.toJSON()

Return an object representing the rendered tree. This tree only contains the platform-specific nodes like <div> or <View> and their props, but doesn’t contain any user-written components. This is handy for snapshot testing.
testRenderer.toTree()

testRenderer.toTree()

Return an object representing the rendered tree. Unlike toJSON(), the representation is more detailed than the one provided by toJSON(), and includes the user-written components. You probably don’t need this method unless you’re writing your own assertion library on top of the test renderer.
testRenderer.update()

testRenderer.update(element)

Re-render the in-memory tree with a new root element. This simulates a React update at the root. If the new element has the same type and key as the previous element, the tree will be updated; otherwise, it will re-mount a new tree.
testRenderer.unmount()

testRenderer.unmount()

Unmount the in-memory tree, triggering the appropriate lifecycle events.
testRenderer.getInstance()

testRenderer.getInstance()

Return the instance corresponding to the root element, if available. This will not work if the root element is a function component because they don’t have instances.
testRenderer.root

testRenderer.root

Returns the root “test instance” object that is useful for making assertions about specific nodes in the tree. You can use it to find other “test instances” deeper below.
testInstance.find()

testInstance.find(test)

Find a single descendant test instance for which test(testInstance) returns true. If test(testInstance) does not return true for exactly one test instance, it will throw an error.
testInstance.findByType()

testInstance.findByType(type)

Find a single descendant test instance with the provided type. If there is not exactly one test instance with the provided type, it will throw an error.
testInstance.findByProps()

testInstance.findByProps(props)

Find a single descendant test instance with the provided props. If there is not exactly one test instance with the provided props, it will throw an error.
testInstance.findAll()

testInstance.findAll(test)

Find all descendant test instances for which test(testInstance) returns true.
testInstance.findAllByType()

testInstance.findAllByType(type)

Find all descendant test instances with the provided type.
testInstance.findAllByProps()

testInstance.findAllByProps(props)

Find all descendant test instances with the provided props.
testInstance.instance

testInstance.instance

The component instance corresponding to this test instance. It is only available for class components, as function components don’t have instances. It matches the this value inside the given component.
testInstance.type

testInstance.type

The component type corresponding to this test instance. For example, a <Button /> component has a type of Button.
testInstance.props

testInstance.props

The props corresponding to this test instance. For example, a <Button size="small" /> component has {size: 'small'} as props.
testInstance.parent

testInstance.parent

The parent test instance of this test instance.
testInstance.children

testInstance.children

The children test instances of this test instance.
Ideas

You can pass createNodeMock function to TestRenderer.create as the option, which allows for custom mock refs. createNodeMock accepts the current element and should return a mock ref object. This is useful when you test a component that relies on refs.

import TestRenderer from 'react-test-renderer';

class MyComponent extends React.Component {
  constructor(props) {
    super(props);
    this.input = null;
  }
  componentDidMount() {
    this.input.focus();
  }
  render() {
    return <input type="text" ref={el => this.input = el} />
  }
}

let focused = false;
TestRenderer.create(
  <MyComponent />,
  {
    createNodeMock: (element) => {
      if (element.type === 'input') {
        // mock a focus function
        return {
          focus: () => {
            focused = true;
          }
        };
      }
      return null;
    }
  }
);
expect(focused).toBe(true);

JavaScript Environment Requirements

React 16 depends on the collection types Map and Set. If you support older browsers and devices which may not yet provide these natively (e.g. IE < 11) or which have non-compliant implementations (e.g. IE 11), consider including a global polyfill in your bundled application, such as core-js or babel-polyfill.

A polyfilled environment for React 16 using core-js to support older browsers might look like:

import 'core-js/es/map';
import 'core-js/es/set';

import React from 'react';
import ReactDOM from 'react-dom';

ReactDOM.render(
  <h1>Hello, world!</h1>,
  document.getElementById('root')
);

React also depends on requestAnimationFrame (even in test environments).
You can use the raf package to shim requestAnimationFrame:

import 'raf/polyfill';

Glossary of React Terms
Single-page Application

A single-page application is an application that loads a single HTML page and all the necessary assets (such as JavaScript and CSS) required for the application to run. Any interactions with the page or subsequent pages do not require a round trip to the server which means the page is not reloaded.

Though you may build a single-page application in React, it is not a requirement. React can also be used for enhancing small parts of existing websites with additional interactivity. Code written in React can coexist peacefully with markup rendered on the server by something like PHP, or with other client-side libraries. In fact, this is exactly how React is being used at Facebook.
ES6, ES2015, ES2016, etc

These acronyms all refer to the most recent versions of the ECMAScript Language Specification standard, which the JavaScript language is an implementation of. The ES6 version (also known as ES2015) includes many additions to the previous versions such as: arrow functions, classes, template literals, let and const statements. You can learn more about specific versions here.
Compilers

A JavaScript compiler takes JavaScript code, transforms it and returns JavaScript code in a different format. The most common use case is to take ES6 syntax and transform it into syntax that older browsers are capable of interpreting. Babel is the compiler most commonly used with React.
Bundlers

Bundlers take JavaScript and CSS code written as separate modules (often hundreds of them), and combine them together into a few files better optimized for the browsers. Some bundlers commonly used in React applications include Webpack and Browserify.
Package Managers

Package managers are tools that allow you to manage dependencies in your project. npm and Yarn are two package managers commonly used in React applications. Both of them are clients for the same npm package registry.
CDN

CDN stands for Content Delivery Network. CDNs deliver cached, static content from a network of servers across the globe.
JSX

JSX is a syntax extension to JavaScript. It is similar to a template language, but it has full power of JavaScript. JSX gets compiled to React.createElement() calls which return plain JavaScript objects called “React elements”. To get a basic introduction to JSX see the docs here and find a more in-depth tutorial on JSX here.

React DOM uses camelCase property naming convention instead of HTML attribute names. For example, tabindex becomes tabIndex in JSX. The attribute class is also written as className since class is a reserved word in JavaScript:

const name = 'Clementine';
ReactDOM.render(
  <h1 className="hello">My name is {name}!</h1>,
  document.getElementById('root')
);

Elements

React elements are the building blocks of React applications. One might confuse elements with a more widely known concept of “components”. An element describes what you want to see on the screen. React elements are immutable.

const element = <h1>Hello, world</h1>;

Typically, elements are not used directly, but get returned from components.
Components

React components are small, reusable pieces of code that return a React element to be rendered to the page. The simplest version of React component is a plain JavaScript function that returns a React element:

function Welcome(props) {
  return <h1>Hello, {props.name}</h1>;
}

Components can also be ES6 classes:

class Welcome extends React.Component {
  render() {
    return <h1>Hello, {this.props.name}</h1>;
  }
}

Components can be broken down into distinct pieces of functionality and used within other components. Components can return other components, arrays, strings and numbers. A good rule of thumb is that if a part of your UI is used several times (Button, Panel, Avatar), or is complex enough on its own (App, FeedStory, Comment), it is a good candidate to be a reusable component. Component names should also always start with a capital letter (<Wrapper/> not <wrapper/>). See this documentation for more information on rendering components.
props

props are inputs to a React component. They are data passed down from a parent component to a child component.

Remember that props are readonly. They should not be modified in any way:

// Wrong!
props.number = 42;

If you need to modify some value in response to user input or a network response, use state instead.
props.children

props.children is available on every component. It contains the content between the opening and closing tags of a component. For example:

<Welcome>Hello world!</Welcome>

The string Hello world! is available in props.children in the Welcome component:

function Welcome(props) {
  return <p>{props.children}</p>;
}

For components defined as classes, use this.props.children:

class Welcome extends React.Component {
  render() {
    return <p>{this.props.children}</p>;
  }
}

state

A component needs state when some data associated with it changes over time. For example, a Checkbox component might need isChecked in its state, and a NewsFeed component might want to keep track of fetchedPosts in its state.

The most important difference between state and props is that props are passed from a parent component, but state is managed by the component itself. A component cannot change its props, but it can change its state.

For each particular piece of changing data, there should be just one component that “owns” it in its state. Don’t try to synchronize states of two different components. Instead, lift it up to their closest shared ancestor, and pass it down as props to both of them.
Lifecycle Methods

Lifecycle methods are custom functionality that gets executed during the different phases of a component. There are methods available when the component gets created and inserted into the DOM (mounting), when the component updates, and when the component gets unmounted or removed from the DOM.
Controlled vs. Uncontrolled Components

React has two different approaches to dealing with form inputs.

An input form element whose value is controlled by React is called a controlled component. When a user enters data into a controlled component a change event handler is triggered and your code decides whether the input is valid (by re-rendering with the updated value). If you do not re-render then the form element will remain unchanged.

An uncontrolled component works like form elements do outside of React. When a user inputs data into a form field (an input box, dropdown, etc) the updated information is reflected without React needing to do anything. However, this also means that you can’t force the field to have a certain value.

In most cases you should use controlled components.
Keys

A “key” is a special string attribute you need to include when creating arrays of elements. Keys help React identify which items have changed, are added, or are removed. Keys should be given to the elements inside an array to give the elements a stable identity.

Keys only need to be unique among sibling elements in the same array. They don’t need to be unique across the whole application or even a single component.

Don’t pass something like Math.random() to keys. It is important that keys have a “stable identity” across re-renders so that React can determine when items are added, removed, or re-ordered. Ideally, keys should correspond to unique and stable identifiers coming from your data, such as post.id.
Refs

React supports a special attribute that you can attach to any component. The ref attribute can be an object created by React.createRef() function or a callback function, or a string (in legacy API). When the ref attribute is a callback function, the function receives the underlying DOM element or class instance (depending on the type of element) as its argument. This allows you to have direct access to the DOM element or component instance.

Use refs sparingly. If you find yourself often using refs to “make things happen” in your app, consider getting more familiar with top-down data flow.
Events

Handling events with React elements has some syntactic differences:

    React event handlers are named using camelCase, rather than lowercase.
    With JSX you pass a function as the event handler, rather than a string.

Reconciliation

When a component’s props or state change, React decides whether an actual DOM update is necessary by comparing the newly returned element with the previously rendered one. When they are not equal, React will update the DOM. This process is called “reconciliation”.

Glossary of React Terms
Single-page Application

A single-page application is an application that loads a single HTML page and all the necessary assets (such as JavaScript and CSS) required for the application to run. Any interactions with the page or subsequent pages do not require a round trip to the server which means the page is not reloaded.

Though you may build a single-page application in React, it is not a requirement. React can also be used for enhancing small parts of existing websites with additional interactivity. Code written in React can coexist peacefully with markup rendered on the server by something like PHP, or with other client-side libraries. In fact, this is exactly how React is being used at Facebook.
ES6, ES2015, ES2016, etc

These acronyms all refer to the most recent versions of the ECMAScript Language Specification standard, which the JavaScript language is an implementation of. The ES6 version (also known as ES2015) includes many additions to the previous versions such as: arrow functions, classes, template literals, let and const statements. You can learn more about specific versions here.
Compilers

A JavaScript compiler takes JavaScript code, transforms it and returns JavaScript code in a different format. The most common use case is to take ES6 syntax and transform it into syntax that older browsers are capable of interpreting. Babel is the compiler most commonly used with React.
Bundlers

Bundlers take JavaScript and CSS code written as separate modules (often hundreds of them), and combine them together into a few files better optimized for the browsers. Some bundlers commonly used in React applications include Webpack and Browserify.
Package Managers

Package managers are tools that allow you to manage dependencies in your project. npm and Yarn are two package managers commonly used in React applications. Both of them are clients for the same npm package registry.
CDN

CDN stands for Content Delivery Network. CDNs deliver cached, static content from a network of servers across the globe.
JSX

JSX is a syntax extension to JavaScript. It is similar to a template language, but it has full power of JavaScript. JSX gets compiled to React.createElement() calls which return plain JavaScript objects called “React elements”. To get a basic introduction to JSX see the docs here and find a more in-depth tutorial on JSX here.

React DOM uses camelCase property naming convention instead of HTML attribute names. For example, tabindex becomes tabIndex in JSX. The attribute class is also written as className since class is a reserved word in JavaScript:

const name = 'Clementine';
ReactDOM.render(
  <h1 className="hello">My name is {name}!</h1>,
  document.getElementById('root')
);

Elements

React elements are the building blocks of React applications. One might confuse elements with a more widely known concept of “components”. An element describes what you want to see on the screen. React elements are immutable.

const element = <h1>Hello, world</h1>;

Typically, elements are not used directly, but get returned from components.
Components

React components are small, reusable pieces of code that return a React element to be rendered to the page. The simplest version of React component is a plain JavaScript function that returns a React element:

function Welcome(props) {
  return <h1>Hello, {props.name}</h1>;
}

Components can also be ES6 classes:

class Welcome extends React.Component {
  render() {
    return <h1>Hello, {this.props.name}</h1>;
  }
}

Components can be broken down into distinct pieces of functionality and used within other components. Components can return other components, arrays, strings and numbers. A good rule of thumb is that if a part of your UI is used several times (Button, Panel, Avatar), or is complex enough on its own (App, FeedStory, Comment), it is a good candidate to be a reusable component. Component names should also always start with a capital letter (<Wrapper/> not <wrapper/>). See this documentation for more information on rendering components.
props

props are inputs to a React component. They are data passed down from a parent component to a child component.

Remember that props are readonly. They should not be modified in any way:

// Wrong!
props.number = 42;

If you need to modify some value in response to user input or a network response, use state instead.
props.children

props.children is available on every component. It contains the content between the opening and closing tags of a component. For example:

<Welcome>Hello world!</Welcome>

The string Hello world! is available in props.children in the Welcome component:

function Welcome(props) {
  return <p>{props.children}</p>;
}

For components defined as classes, use this.props.children:

class Welcome extends React.Component {
  render() {
    return <p>{this.props.children}</p>;
  }
}

state

A component needs state when some data associated with it changes over time. For example, a Checkbox component might need isChecked in its state, and a NewsFeed component might want to keep track of fetchedPosts in its state.

The most important difference between state and props is that props are passed from a parent component, but state is managed by the component itself. A component cannot change its props, but it can change its state.

For each particular piece of changing data, there should be just one component that “owns” it in its state. Don’t try to synchronize states of two different components. Instead, lift it up to their closest shared ancestor, and pass it down as props to both of them.
Lifecycle Methods

Lifecycle methods are custom functionality that gets executed during the different phases of a component. There are methods available when the component gets created and inserted into the DOM (mounting), when the component updates, and when the component gets unmounted or removed from the DOM.
Controlled vs. Uncontrolled Components

React has two different approaches to dealing with form inputs.

An input form element whose value is controlled by React is called a controlled component. When a user enters data into a controlled component a change event handler is triggered and your code decides whether the input is valid (by re-rendering with the updated value). If you do not re-render then the form element will remain unchanged.

# Introducing Hooks

## State Hook

import React, { useState } from 'react';

function Example() {
  // Declare a new state variable, which we'll call "count"
  const [count, setCount] = useState(0);

  return (
    <div>
      <p>You clicked {count} times</p>
      <button onClick={() => setCount(count + 1)}>
        Click me
      </button>
    </div>
  );
}

useState is a Hook. It’s similar to this.setState in a class, except it doesn’t merge the old and new state together.
The initial state argument is only used during the first render.

You can use the State Hook more than once in a single component:

function ExampleWithManyStates() {
  // Declare multiple state variables!
  const [age, setAge] = useState(42);
  const [fruit, setFruit] = useState('banana');
  const [todos, setTodos] = useState([{ text: 'Learn Hooks' }]);
  // ...
}

## Effect Hook

The Effect Hook, useEffect, adds the ability to perform side effects from a function component. It serves the same purpose as componentDidMount, componentDidUpdate, and componentWillUnmount in React classes, but unified into a single API. 

function FriendStatusWithCounter(props) {
  const [count, setCount] = useState(0);
  useEffect(() => {
    document.title = `You clicked ${count} times`;
  });

  const [isOnline, setIsOnline] = useState(null);
  useEffect(() => {
    ChatAPI.subscribeToFriendStatus(props.friend.id, handleStatusChange);
    return () => {
      ChatAPI.unsubscribeFromFriendStatus(props.friend.id, handleStatusChange);
    };
  });

  function handleStatusChange(status) {
    setIsOnline(status.isOnline);
  }
  // ...

## Rules of Hooks

    Only call Hooks at the top level. Don’t call Hooks inside loops, conditions, or nested functions.
    Only call Hooks from React function components. Don’t call Hooks from regular JavaScript functions. (There is just one other valid place to call Hooks — your own custom Hooks. We’ll learn about them in a moment.)

## Building Your Own Hooks

import React, { useState, useEffect } from 'react';

function useFriendStatus(friendID) {
  const [isOnline, setIsOnline] = useState(null);

  function handleStatusChange(status) {
    setIsOnline(status.isOnline);
  }

  useEffect(() => {
    ChatAPI.subscribeToFriendStatus(friendID, handleStatusChange);
    return () => {
      ChatAPI.unsubscribeFromFriendStatus(friendID, handleStatusChange);
    };
  });

  return isOnline;
}

function FriendStatus(props) {
  const isOnline = useFriendStatus(props.friend.id);

  if (isOnline === null) {
    return 'Loading...';
  }
  return isOnline ? 'Online' : 'Offline';
}

function FriendListItem(props) {
  const isOnline = useFriendStatus(props.friend.id);

  return (
    <li style={{ color: isOnline ? 'green' : 'black' }}>
      {props.friend.name}
    </li>
  );
}

Hooks are a way to reuse stateful logic, not state itself. In fact, each call to a Hook has a completely isolated state — so you can even use the same custom Hook twice in one component.

## Using the State Hook

import React, { useState } from 'react';

function Example() {
  // Declare a new state variable, which we'll call "count"
  const [count, setCount] = useState(0);

  return (
    <div>
      <p>You clicked {count} times</p>
      <button onClick={() => setCount(count + 1)}>
        Click me
      </button>
    </div>
  );
}

If you used classes in React before, this code should look familiar:

class Example extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
      count: 0
    };
  }

  render() {
    return (
      <div>
        <p>You clicked {this.state.count} times</p>
        <button onClick={() => this.setState({ count: this.state.count + 1 })}>
          Click me
        </button>
      </div>
    );
  }
}

const Example = (props) => {
  // You can use Hooks here!
  return <div />;
}

or this:

function Example(props) {
  // You can use Hooks here!
  return <div />;
}

## Using the Effect Hook

import React, { useState, useEffect } from 'react';

function Example() {
  const [count, setCount] = useState(0);

  // Similar to componentDidMount and componentDidUpdate:
  useEffect(() => {
    // Update the document title using the browser API
    document.title = `You clicked ${count} times`;
  });

  return (
    <div>
      <p>You clicked {count} times</p>
      <button onClick={() => setCount(count + 1)}>
        Click me
      </button>
    </div>
  );
}

class Example extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
      count: 0
    };
  }

  componentDidMount() {
    document.title = `You clicked ${this.state.count} times`;
  }

  componentDidUpdate() {
    document.title = `You clicked ${this.state.count} times`;
  }

  render() {
    return (
      <div>
        <p>You clicked {this.state.count} times</p>
        <button onClick={() => this.setState({ count: this.state.count + 1 })}>
          Click me
        </button>
      </div>
    );
  }
}

function passed to useEffect is going to be different on every render. This is intentional. In fact, this is what lets us read the count value from inside the effect without worrying about it getting stale. Every time we re-render, we schedule a different effect, replacing the previous one. In a way, this makes the effects behave more like a part of the render result — each effect “belongs” to a particular render.

  useEffect(() => {
    function handleStatusChange(status) {
      setIsOnline(status.isOnline);
    }

    ChatAPI.subscribeToFriendStatus(props.friend.id, handleStatusChange);
    return () => {
      ChatAPI.unsubscribeFromFriendStatus(props.friend.id, handleStatusChange);
    };
  });

Tip: Optimizing Performance by Skipping Effects

In some cases, cleaning up or applying the effect after every render might create a performance problem. In class components, we can solve this by writing an extra comparison with prevProps or prevState inside componentDidUpdate:

componentDidUpdate(prevProps, prevState) {
  if (prevState.count !== this.state.count) {
    document.title = `You clicked ${this.state.count} times`;
  }
}

You can tell React to skip applying an effect if certain values haven’t changed between re-renders. To do so, pass an array as an optional second argument to useEffect:

useEffect(() => {
  document.title = `You clicked ${count} times`;
}, [count]); // Only re-run the effect if count changes

    If you use this optimization, make sure the array includes all values from the component scope (such as props and state) that change over time and that are used by the effect. Otherwise, your code will reference stale values from previous renders.

    If you want to run an effect and clean it up only once (on mount and unmount), you can pass an empty array ([]) as a second argument. This tells React that your effect doesn’t depend on any values from props or state, so it never needs to re-run. This isn’t handled as a special case — it follows directly from how the dependencies array always works.

    If you pass an empty array ([]), the props and state inside the effect will always have their initial values. While passing [] as the second argument is closer to the familiar componentDidMount and componentWillUnmount mental model, there are usually better solutions to avoid re-running effects too often. Also, don’t forget that React defers running useEffect until after the browser has painted, so doing extra work is less of a problem.

Building Your Own Hooks

import React, { useState, useEffect } from 'react';

function useFriendStatus(friendID) {
  const [isOnline, setIsOnline] = useState(null);

  useEffect(() => {
    function handleStatusChange(status) {
      setIsOnline(status.isOnline);
    }

    ChatAPI.subscribeToFriendStatus(friendID, handleStatusChange);
    return () => {
      ChatAPI.unsubscribeFromFriendStatus(friendID, handleStatusChange);
    };
  });

  return isOnline;
}

Do two components using the same Hook share state? No. Custom Hooks are a mechanism to reuse stateful logic (such as setting up a subscription and remembering the current value), but every time you use a custom Hook, all state and effects inside of it are fully isolated.

function useReducer(reducer, initialState) {
  const [state, setState] = useState(initialState);

  function dispatch(action) {
    const nextState = reducer(state, action);
    setState(nextState);
  }

  return [state, dispatch];
}

function Todos() {
  const [todos, dispatch] = useReducer(todosReducer, []);

  function handleAddClick(text) {
    dispatch({ type: 'add', text });
  }

  // ...
}

The need to manage local state with a reducer in a complex component is common enough that we’ve built the useReducer Hook right into React. You’ll find it together with other built-in Hooks in the Hooks API reference.

Hooks API Reference

Functional updates

If the new state is computed using the previous state, you can pass a function to setState. The function will receive the previous value, and return an updated value.

function Counter({initialCount}) {
  const [count, setCount] = useState(initialCount);
  return (
    <>
      Count: {count}
      <button onClick={() => setCount(initialCount)}>Reset</button>
      <button onClick={() => setCount(prevCount => prevCount + 1)}>+</button>
      <button onClick={() => setCount(prevCount => prevCount - 1)}>-</button>
    </>
  );
}

    Unlike the setState method found in class components, useState does not automatically merge update objects. You can replicate this behavior by combining the function updater form with object spread syntax:

    setState(prevState => {
      // Object.assign would also work
      return {...prevState, ...updatedValues};
    });

Lazy initial state

The initialState argument is the state used during the initial render. In subsequent renders, it is disregarded. If the initial state is the result of an expensive computation, you may provide a function instead, which will be executed only on the initial render:

const [state, setState] = useState(() => {
  const initialState = someExpensiveComputation(props);
  return initialState;
});

useEffect

useEffect(didUpdate);

Accepts a function that contains imperative, possibly effectful code.

By default, effects run after every completed render, but you can choose to fire it only when certain values have changed.

useEffect(() => {
  const subscription = props.source.subscribe();
  return () => {
    // Clean up the subscription
    subscription.unsubscribe();
  };
});

useContext

const value = useContext(MyContext);

Accepts a context object (the value returned from React.createContext) and returns the current context value for that context. The current context value is determined by the value prop of the nearest <MyContext.Provider> above the calling component in the tree.

When the nearest <MyContext.Provider> above the component updates, this Hook will trigger a rerender with the latest context value passed to that MyContext provider.

Don’t forget that the argument to useContext must be the context object itself:

    Correct: useContext(MyContext)
    Incorrect: useContext(MyContext.Consumer)
    Incorrect: useContext(MyContext.Provider)

A component calling useContext will always re-render when the context value changes. If re-rendering the component is expensive, you can optimize it by using memoization.

    Tip

    If you’re familiar with the context API before Hooks, useContext(MyContext) is equivalent to static contextType = MyContext in a class, or to <MyContext.Consumer>.

    useContext(MyContext) only lets you read the context and subscribe to its changes. You still need a <MyContext.Provider> above in the tree to provide the value for this context.

Additional Hooks

useReducer

const [state, dispatch] = useReducer(reducer, initialArg, init);

An alternative to useState. Accepts a reducer of type (state, action) => newState, and returns the current state paired with a dispatch method. (If you’re familiar with Redux, you already know how this works.)

useReducer is usually preferable to useState when you have complex state logic that involves multiple sub-values or when the next state depends on the previous one. useReducer also lets you optimize performance for components that trigger deep updates because you can pass dispatch down instead of callbacks.

Here’s the counter example from the useState section, rewritten to use a reducer:

const initialState = {count: 0};

function reducer(state, action) {
  switch (action.type) {
    case 'increment':
      return {count: state.count + 1};
    case 'decrement':
      return {count: state.count - 1};
    default:
      throw new Error();
  }
}

function Counter() {
  const [state, dispatch] = useReducer(reducer, initialState);
  return (
    <>
      Count: {state.count}
      <button onClick={() => dispatch({type: 'increment'})}>+</button>
      <button onClick={() => dispatch({type: 'decrement'})}>-</button>
    </>
  );
}

    React guarantees that dispatch function identity is stable and won’t change on re-renders. This is why it’s safe to omit from the useEffect or useCallback dependency list.

Specifying the initial state

There’s two different ways to initialize useReducer state. You may choose either one depending on the use case. The simplest way to pass the initial state as a second argument:

  const [state, dispatch] = useReducer(
    reducer,
    {count: initialCount}
  );

    Note

    React doesn’t use the state = initialState argument convention popularized by Redux. The initial value sometimes needs to depend on props and so is specified from the Hook call instead. If you feel strongly about this, you can call useReducer(reducer, undefined, reducer) to emulate the Redux behavior, but it’s not encouraged.

Lazy initialization

You can also create the initial state lazily. To do this, you can pass an init function as the third argument. The initial state will be set to init(initialArg).

It lets you extract the logic for calculating the initial state outside the reducer. This is also handy for resetting the state later in response to an action:

function init(initialCount) {
  return {count: initialCount};
}

function reducer(state, action) {
  switch (action.type) {
    case 'increment':
      return {count: state.count + 1};
    case 'decrement':
      return {count: state.count - 1};
    case 'reset':
      return init(action.payload);
    default:
      throw new Error();
  }
}

function Counter({initialCount}) {
  const [state, dispatch] = useReducer(reducer, initialCount, init);
  return (
    <>
      Count: {state.count}
      <button
        onClick={() => dispatch({type: 'reset', payload: initialCount})}>
        Reset
      </button>
      <button onClick={() => dispatch({type: 'increment'})}>+</button>
      <button onClick={() => dispatch({type: 'decrement'})}>-</button>
    </>
  );
}

Bailing out of a dispatch

If you return the same value from a Reducer Hook as the current state, React will bail out without rendering the children or firing effects. (React uses the Object.is comparison algorithm.)

Note that React may still need to render that specific component again before bailing out. That shouldn’t be a concern because React won’t unnecessarily go “deeper” into the tree. If you’re doing expensive calculations while rendering, you can optimize them with useMemo.
useCallback

const memoizedCallback = useCallback(
  () => {
    doSomething(a, b);
  },
  [a, b],
);

Returns a memoized callback.

Pass an inline callback and an array of dependencies. useCallback will return a memoized version of the callback that only changes if one of the dependencies has changed. This is useful when passing callbacks to optimized child components that rely on reference equality to prevent unnecessary renders (e.g. shouldComponentUpdate).

useCallback(fn, deps) is equivalent to useMemo(() => fn, deps).

    Note

    The array of dependencies is not passed as arguments to the callback. Conceptually, though, that’s what they represent: every value referenced inside the callback should also appear in the dependencies array. In the future, a sufficiently advanced compiler could create this array automatically.

    We recommend using the exhaustive-deps rule as part of our eslint-plugin-react-hooks package. It warns when dependencies are specified incorrectly and suggests a fix.

useMemo

const memoizedValue = useMemo(() => computeExpensiveValue(a, b), [a, b]);

Returns a memoized value.

Pass a “create” function and an array of dependencies. useMemo will only recompute the memoized value when one of the dependencies has changed. This optimization helps to avoid expensive calculations on every render.

Remember that the function passed to useMemo runs during rendering. Don’t do anything there that you wouldn’t normally do while rendering. For example, side effects belong in useEffect, not useMemo.

If no array is provided, a new value will be computed on every render.

You may rely on useMemo as a performance optimization, not as a semantic guarantee. In the future, React may choose to “forget” some previously memoized values and recalculate them on next render, e.g. to free memory for offscreen components. Write your code so that it still works without useMemo — and then add it to optimize performance.

    Note

    The array of dependencies is not passed as arguments to the function. Conceptually, though, that’s what they represent: every value referenced inside the function should also appear in the dependencies array. In the future, a sufficiently advanced compiler could create this array automatically.

    We recommend using the exhaustive-deps rule as part of our eslint-plugin-react-hooks package. It warns when dependencies are specified incorrectly and suggests a fix.

useRef

const refContainer = useRef(initialValue);

useRef returns a mutable ref object whose .current property is initialized to the passed argument (initialValue). The returned object will persist for the full lifetime of the component.

A common use case is to access a child imperatively:

function TextInputWithFocusButton() {
  const inputEl = useRef(null);
  const onButtonClick = () => {
    // `current` points to the mounted text input element
    inputEl.current.focus();
  };
  return (
    <>
      <input ref={inputEl} type="text" />
      <button onClick={onButtonClick}>Focus the input</button>
    </>
  );
}

Essentially, useRef is like a “box” that can hold a mutable value in its .current property.

You might be familiar with refs primarily as a way to access the DOM. If you pass a ref object to React with <div ref={myRef} />, React will set its .current property to the corresponding DOM node whenever that node changes.

However, useRef() is useful for more than the ref attribute. It’s handy for keeping any mutable value around similar to how you’d use instance fields in classes.

This works because useRef() creates a plain JavaScript object. The only difference between useRef() and creating a {current: ...} object yourself is that useRef will give you the same ref object on every render.

Keep in mind that useRef doesn’t notify you when its content changes. Mutating the .current property doesn’t cause a re-render. If you want to run some code when React attaches or detaches a ref to a DOM node, you may want to use a callback ref instead.
useImperativeHandle

useImperativeHandle(ref, createHandle, [deps])

useImperativeHandle customizes the instance value that is exposed to parent components when using ref. As always, imperative code using refs should be avoided in most cases. useImperativeHandle should be used with forwardRef:

function FancyInput(props, ref) {
  const inputRef = useRef();
  useImperativeHandle(ref, () => ({
    focus: () => {
      inputRef.current.focus();
    }
  }));
  return <input ref={inputRef} ... />;
}
FancyInput = forwardRef(FancyInput);

In this example, a parent component that renders <FancyInput ref={fancyInputRef} /> would be able to call fancyInputRef.current.focus().
useLayoutEffect

The signature is identical to useEffect, but it fires synchronously after all DOM mutations. Use this to read layout from the DOM and synchronously re-render. Updates scheduled inside useLayoutEffect will be flushed synchronously, before the browser has a chance to paint.

Prefer the standard useEffect when possible to avoid blocking visual updates.

    Tip

    If you’re migrating code from a class component, note useLayoutEffect fires in the same phase as componentDidMount and componentDidUpdate. However, we recommend starting with useEffect first and only trying useLayoutEffect if that causes a problem.

    If you use server rendering, keep in mind that neither useLayoutEffect nor useEffect can run until the JavaScript is downloaded. This is why React warns when a server-rendered component contains useLayoutEffect. To fix this, either move that logic to useEffect (if it isn’t necessary for the first render), or delay showing that component until after the client renders (if the HTML looks broken until useLayoutEffect runs).

    To exclude a component that needs layout effects from the server-rendered HTML, render it conditionally with showChild && <Child /> and defer showing it with useEffect(() => { setShowChild(true); }, []). This way, the UI doesn’t appear broken before hydration.

useDebugValue

useDebugValue(value)

useDebugValue can be used to display a label for custom hooks in React DevTools.

For example, consider the useFriendStatus custom Hook described in “Building Your Own Hooks”:

function useFriendStatus(friendID) {
  const [isOnline, setIsOnline] = useState(null);

  // ...

  // Show a label in DevTools next to this Hook
  // e.g. "FriendStatus: Online"
  useDebugValue(isOnline ? 'Online' : 'Offline');

  return isOnline;
}

    We don’t recommend adding debug values to every custom Hook. It’s most valuable for custom Hooks that are part of shared libraries.

Defer formatting debug values

In some cases formatting a value for display might be an expensive operation. It’s also unnecessary unless a Hook is actually inspected.

For this reason useDebugValue accepts a formatting function as an optional second parameter. This function is only called if the Hooks are inspected. It receives the debug value as a parameter and should return a formatted display value.

For example a custom Hook that returned a Date value could avoid calling the toDateString function unnecessarily by passing the following formatter:

useDebugValue(date, date => date.toDateString());

From Classes to Hooks
    constructor: Function components don’t need a constructor. You can initialize the state in the useState call. If computing the initial state is expensive, you can pass a function to useState.

    getDerivedStateFromProps: Schedule an update while rendering instead.

    shouldComponentUpdate: See React.memo below.

    render: This is the function component body itself.

    componentDidMount, componentDidUpdate, componentWillUnmount: The useEffect Hook can express all combinations of these (including less common cases).

    componentDidCatch and getDerivedStateFromError: There are no Hook equivalents for these methods yet, but they will be added soon.

How can I do data fetching with Hooks?
function Timer() {
  const intervalRef = useRef();

  useEffect(() => {
    const id = setInterval(() => {
      // ...
    });
    intervalRef.current = id;
    return () => {
      clearInterval(intervalRef.current);
    };
  });

  // ...
}

If we just wanted to set an interval, we wouldn’t need the ref (id could be local to the effect), but it’s useful if we want to clear the interval from an event handler:

  // ...
  function handleCancelClick() {
    clearInterval(intervalRef.current);
  }
  // ...

Conceptually, you can think of refs as similar to instance variables in a class. Unless you’re doing lazy initialization, avoid setting refs during rendering — this can lead to surprising behavior. Instead, typically you want to modify refs in event handlers and effects.

  useEffect(() => {
    function handleWindowMouseMove(e) {
      // Spreading "...state" ensures we don't "lose" width and height
      setState(state => ({ ...state, left: e.pageX, top: e.pageY }));
    }
    // Note: this implementation is a bit simplified
    window.addEventListener('mousemove', handleWindowMouseMove);
    return () => window.removeEventListener('mousemove', handleWindowMouseMove);
  }, []);
  // ...

This is because when we update a state variable, we replace its value. This is different from this.setState in a class, which merges the updated fields into the object.

Can I run an effect only on updates?

This is a rare use case. If you need it, you can use a mutable ref to manually store a boolean value corresponding to whether you are on the first or a subsequent render, then check that flag in your effect. (If you find yourself doing this often, you could create a custom Hook for it.)
How to get the previous props or state?

function Counter() {
  const [count, setCount] = useState(0);

  const prevCountRef = useRef();
  useEffect(() => {
    prevCountRef.current = count;
  });
  const prevCount = prevCountRef.current;

  return <h1>Now: {count}, before: {prevCount}</h1>;
}

This might be a bit convoluted but you can extract it into a custom Hook:

function Counter() {
  const [count, setCount] = useState(0);
  const prevCount = usePrevious(count);
  return <h1>Now: {count}, before: {prevCount}</h1>;
}

function usePrevious(value) {
  const ref = useRef();
  useEffect(() => {
    ref.current = value;
  });
  return ref.current;
}

Note how this would work for props, state, or any other calculated value.

function Counter() {
  const [count, setCount] = useState(0);

  const calculation = count * 100;
  const prevCalculation = usePrevious(calculation);
  // ...

It’s possible that in the future React will provide a usePrevious Hook out of the box since it’s a relatively common use case.

See also the recommended pattern for derived state.
Why am I seeing stale props or state inside my function?

Any function inside a component, including event handlers and effects, “sees” the props and state from the render it was created in. For example, consider code like this:

function Example() {
  const [count, setCount] = useState(0);

  function handleAlertClick() {
    setTimeout(() => {
      alert('You clicked on: ' + count);
    }, 3000);
  }

  return (
    <div>
      <p>You clicked {count} times</p>
      <button onClick={() => setCount(count + 1)}>
        Click me
      </button>
      <button onClick={handleAlertClick}>
        Show alert
      </button>
    </div>
  );
}

If you first click “Show alert” and then increment the counter, the alert will show the count variable at the time you clicked the “Show alert” button. This prevents bugs caused by the code assuming props and state don’t change.

If you intentionally want to read the latest state from some asynchronous callback, you could keep it in a ref, mutate it, and read from it.

Finally, another possible reason you’re seeing stale props or state is if you use the “dependency array” optimization but didn’t correctly specify all the dependencies. For example, if an effect specifies [] as the second argument but reads someProp inside, it will keep “seeing” the initial value of someProp. The solution is to either remove the dependency array, or to fix it. Here’s how you can deal with functions, and here’s other common strategies to run effects less often without incorrectly skipping dependencies.

    Note

    We provide an exhaustive-deps ESLint rule as a part of the eslint-plugin-react-hooks package. It warns when dependencies are specified incorrectly and suggests a fix.

How do I implement getDerivedStateFromProps?

While you probably don’t need it, in rare cases that you do (such as implementing a <Transition> component), you can update the state right during rendering. React will re-run the component with updated state immediately after exiting the first render so it wouldn’t be expensive.

Here, we store the previous value of the row prop in a state variable so that we can compare:

function ScrollView({row}) {
  let [isScrollingDown, setIsScrollingDown] = useState(false);
  let [prevRow, setPrevRow] = useState(null);

  if (row !== prevRow) {
    // Row changed since last render. Update isScrollingDown.
    setIsScrollingDown(prevRow !== null && row > prevRow);
    setPrevRow(row);
  }

  return `Scrolling down: ${isScrollingDown}`;
}

This might look strange at first, but an update during rendering is exactly what getDerivedStateFromProps has always been like conceptually.
Is there something like forceUpdate?

Both useState and useReducer Hooks bail out of updates if the next value is the same as the previous one. Mutating state in place and calling setState will not cause a re-render.

Normally, you shouldn’t mutate local state in React. However, as an escape hatch, you can use an incrementing counter to force a re-render even if the state has not changed:

  const [ignored, forceUpdate] = useReducer(x => x + 1, 0);

  function handleClick() {
    forceUpdate();
  }

Try to avoid this pattern if possible.
Can I make a ref to a function component?

While you shouldn’t need this often, you may expose some imperative methods to a parent component with the useImperativeHandle Hook.
How can I measure a DOM node?

In order to measure the position or size of a DOM node, you can use a callback ref. React will call that callback whenever the ref gets attached to a different node. Here is a small demo:

function MeasureExample() {
  const [height, setHeight] = useState(0);

  const measuredRef = useCallback(node => {
    if (node !== null) {
      setHeight(node.getBoundingClientRect().height);
    }
  }, []);

  return (
    <>
      <h1 ref={measuredRef}>Hello, world</h1>
      <h2>The above header is {Math.round(height)}px tall</h2>
    </>
  );
}

We didn’t choose useRef in this example because an object ref doesn’t notify us about changes to the current ref value. Using a callback ref ensures that even if a child component displays the measured node later (e.g. in response to a click), we still get notified about it in the parent component and can update the measurements.

Note that we pass [] as a dependency array to useCallback. This ensures that our ref callback doesn’t change between the re-renders, and so React won’t call it unnecessarily.

If you want, you can extract this logic into a reusable Hook:

function MeasureExample() {
  const [rect, ref] = useClientRect();
  return (
    <>
      <h1 ref={ref}>Hello, world</h1>
      {rect !== null &&
        <h2>The above header is {Math.round(rect.height)}px tall</h2>
      }
    </>
  );
}

function useClientRect() {
  const [rect, setRect] = useState(null);
  const ref = useCallback(node => {
    if (node !== null) {
      setRect(node.getBoundingClientRect());
    }
  }, []);
  return [rect, ref];
}

What does const [thing, setThing] = useState() mean?

If you’re not familiar with this syntax, check out the explanation in the State Hook documentation.
Performance Optimizations
Can I skip an effect on updates?

Yes. See conditionally firing an effect. Note that forgetting to handle updates often introduces bugs, which is why this isn’t the default behavior.
Is it safe to omit functions from the list of dependencies?

Generally speaking, no.

function Example({ someProp }) {
  function doSomething() {
    console.log(someProp);
  }

  useEffect(() => {
    doSomething();
  }, []); // 🔴 This is not safe (it calls `doSomething` which uses `someProp`)
}

It’s difficult to remember which props or state are used by functions outside of the effect. This is why usually you’ll want to declare functions needed by an effect inside of it. Then it’s easy to see what values from the component scope that effect depends on:

function Example({ someProp }) {
  useEffect(() => {
    function doSomething() {
      console.log(someProp);
    }

    doSomething();
  }, [someProp]); // ✅ OK (our effect only uses `someProp`)
}

If after that we still don’t use any values from the component scope, it’s safe to specify []:

useEffect(() => {
  function doSomething() {
    console.log('hello');
  }

  doSomething();
}, []); // ✅ OK in this example because we don't use *any* values from component scope

Depending on your use case, there are a few more options described below.

    Note

    We provide the exhaustive-deps ESLint rule as a part of the eslint-plugin-react-hooks package. It help you find components that don’t handle updates consistently.

Let’s see why this matters.

If you specify a list of dependencies as the last argument to useEffect, useMemo, useCallback, or useImperativeHandle, it must include all values used inside that participate in the React data flow. That includes props, state, and anything derived from them.

It is only safe to omit a function from the dependency list if nothing in it (or the functions called by it) references props, state, or values derived from them. This example has a bug:

function ProductPage({ productId }) {
  const [product, setProduct] = useState(null);

  async function fetchProduct() {
    const response = await fetch('http://myapi/product' + productId); // Uses productId prop
    const json = await response.json();
    setProduct(json);
  }

  useEffect(() => {
    fetchProduct();
  }, []); // 🔴 Invalid because `fetchProduct` uses `productId`
  // ...
}

The recommended fix is to move that function inside of your effect. That makes it easy to see which props or state your effect uses, and to ensure they’re all declared:

function ProductPage({ productId }) {
  const [product, setProduct] = useState(null);

  useEffect(() => {
    // By moving this function inside the effect, we can clearly see the values it uses.
    async function fetchProduct() {
      const response = await fetch('http://myapi/product' + productId);
      const json = await response.json();
      setProduct(json);
    }

    fetchProduct();
  }, [productId]); // ✅ Valid because our effect only uses productId
  // ...
}

This also allows you to handle out-of-order responses with a local variable inside the effect:

  useEffect(() => {
    let ignore = false;
    async function fetchProduct() {
      const response = await fetch('http://myapi/product/' + productId);
      const json = await response.json();
      if (!ignore) setProduct(json);
    }
    return () => { ignore = true };
  }, [productId]);

We moved the function inside the effect so it doesn’t need to be in its dependency list.

    Tip

    Check out this small demo and this article to learn more about data fetching with Hooks.

If for some reason you can’t move a function inside an effect, there are a few more options:

    You can try moving that function outside of your component. In that case, the function is guaranteed to not reference any props or state, and also doesn’t need to be in the list of dependencies.
    If the function you’re calling is a pure computation and is safe to call while rendering, you may call it outside of the effect instead, and make the effect depend on the returned value.
    As a last resort, you can add a function to effect dependencies but wrap its definition into the useCallback Hook. This ensures it doesn’t change on every render unless its own dependencies also change:

function ProductPage({ productId }) {
  // ✅ Wrap with useCallback to avoid change on every render
  const fetchProduct = useCallback(() => {
    // ... Does something with productId ...
  }, [productId]); // ✅ All useCallback dependencies are specified

  return <ProductDetails fetchProduct={fetchProduct} />;
}

function ProductDetails({ fetchProduct })
  useEffect(() => {
    fetchProduct();
  }, [fetchProduct]); // ✅ All useEffect dependencies are specified
  // ...
}

Note that in the above example we need to keep the function in the dependencies list. This ensures that a change in the productId prop of ProductPage automatically triggers a refetch in the ProductDetails component.
What can I do if my effect dependencies change too often?

Sometimes, your effect may be using state that changes too often. You might be tempted to omit that state from a list of dependencies, but that usually leads to bugs:

function Counter() {
  const [count, setCount] = useState(0);

  useEffect(() => {
    const id = setInterval(() => {
      setCount(count + 1); // This effect depends on the `count` state
    }, 1000);
    return () => clearInterval(id);
  }, []); // 🔴 Bug: `count` is not specified as a dependency

  return <h1>{count}</h1>;
}

Specifying [count] as a list of dependencies would fix the bug, but would cause the interval to be reset on every change. That may not be desirable. To fix this, we can use the functional update form of setState. It lets us specify how the state needs to change without referencing the current state:

function Counter() {
  const [count, setCount] = useState(0);

  useEffect(() => {
    const id = setInterval(() => {
      setCount(c => c + 1); // ✅ This doesn't depend on `count` variable outside
    }, 1000);
    return () => clearInterval(id);
  }, []); // ✅ Our effect doesn't use any variables in the component scope

  return <h1>{count}</h1>;
}

(The identity of the setCount function is guaranteed to be stable so it’s safe to omit.)

In more complex cases (such as if one state depends on another state), try moving the state update logic outside the effect with the useReducer Hook. This article offers an example of how you can do this. The identity of the dispatch function from useReducer is always stable — even if the reducer function is declared inside the component and reads its props.

As a last resort, if you want something like this in a class, you can use a ref to hold a mutable variable. Then you can write and read to it. For example:

function Example(props) {
  // Keep latest props in a ref.
  let latestProps = useRef(props);
  useEffect(() => {
    latestProps.current = props;
  });

  useEffect(() => {
    function tick() {
      // Read latest props at any time
      console.log(latestProps.current);
    }

    const id = setInterval(tick, 1000);
    return () => clearInterval(id);
  }, []); // This effect never re-runs
}

Only do this if you couldn’t find a better alternative, as relying on mutation makes components less predictable. If there’s a specific pattern that doesn’t translate well, file an issue with a runnable example code and we can try to help.
How do I implement shouldComponentUpdate?

You can wrap a function component with React.memo to shallowly compare its props:

const Button = React.memo((props) => {
  // your component
});

It’s not a Hook because it doesn’t compose like Hooks do. React.memo is equivalent to PureComponent, but it only compares props. (You can also add a second argument to specify a custom comparison function that takes the old and new props. If it returns true, the update is skipped.)

React.memo doesn’t compare state because there is no single state object to compare. But you can make children pure too, or even optimize individual children with useMemo.
How to memoize calculations?

The useMemo Hook lets you cache calculations between multiple renders by “remembering” the previous computation:

const memoizedValue = useMemo(() => computeExpensiveValue(a, b), [a, b]);

This code calls computeExpensiveValue(a, b). But if the dependencies [a, b] haven’t changed since the last value, useMemo skips calling it a second time and simply reuses the last value it returned.

Remember that the function passed to useMemo runs during rendering. Don’t do anything there that you wouldn’t normally do while rendering. For example, side effects belong in useEffect, not useMemo.

You may rely on useMemo as a performance optimization, not as a semantic guarantee. In the future, React may choose to “forget” some previously memoized values and recalculate them on next render, e.g. to free memory for offscreen components. Write your code so that it still works without useMemo — and then add it to optimize performance. (For rare cases when a value must never be recomputed, you can lazily initialize a ref.)

Conveniently, useMemo also lets you skip an expensive re-render of a child:

function Parent({ a, b }) {
  // Only re-rendered if `a` changes:
  const child1 = useMemo(() => <Child1 a={a} />, [a]);
  // Only re-rendered if `b` changes:
  const child2 = useMemo(() => <Child2 b={b} />, [b]);
  return (
    <>
      {child1}
      {child2}
    </>
  )
}

Note that this approach won’t work in a loop because Hook calls can’t be placed inside loops. But you can extract a separate component for the list item, and call useMemo there.
How to create expensive objects lazily?

useMemo lets you memoize an expensive calculation if the dependencies are the same. However, it only serves as a hint, and doesn’t guarantee the computation won’t re-run. But sometimes you need to be sure an object is only created once.

The first common use case is when creating the initial state is expensive:

function Table(props) {
  // ⚠️ createRows() is called on every render
  const [rows, setRows] = useState(createRows(props.count));
  // ...
}

To avoid re-creating the ignored initial state, we can pass a function to useState:

function Table(props) {
  // ✅ createRows() is only called once
  const [rows, setRows] = useState(() => createRows(props.count));
  // ...
}

React will only call this function during the first render. See the useState API reference.

You might also occasionally want to avoid re-creating the useRef() initial value. For example, maybe you want to ensure some imperative class instance only gets created once:

function Image(props) {
  // ⚠️ IntersectionObserver is created on every render
  const ref = useRef(new IntersectionObserver(onIntersect));
  // ...
}

useRef does not accept a special function overload like useState. Instead, you can write your own function that creates and sets it lazily:

function Image(props) {
  const ref = useRef(null);

  // ✅ IntersectionObserver is created lazily once
  function getObserver() {
    if (ref.current === null) {
      ref.current = new IntersectionObserver(onIntersect);
    }
    return ref.current;
  }

  // When you need it, call getObserver()
  // ...
}

This avoids creating an expensive object until it’s truly needed for the first time. If you use Flow or TypeScript, you can also give getObserver() a non-nullable type for convenience.
Are Hooks slow because of creating functions in render?

No. In modern browsers, the raw performance of closures compared to classes doesn’t differ significantly except in extreme scenarios.

In addition, consider that the design of Hooks is more efficient in a couple ways:

    Hooks avoid a lot of the overhead that classes require, like the cost of creating class instances and binding event handlers in the constructor.

    Idiomatic code using Hooks doesn’t need the deep component tree nesting that is prevalent in codebases that use higher-order components, render props, and context. With smaller component trees, React has less work to do.

Traditionally, performance concerns around inline functions in React have been related to how passing new callbacks on each render breaks shouldComponentUpdate optimizations in child components. Hooks approach this problem from three sides.

    The useCallback Hook lets you keep the same callback reference between re-renders so that shouldComponentUpdate continues to work:

    // Will not change unless `a` or `b` changes
    const memoizedCallback = useCallback(() => {
      doSomething(a, b);
    }, [a, b]);

    The useMemo Hook makes it easier to control when individual children update, reducing the need for pure components.

    Finally, the useReducer Hook reduces the need to pass callbacks deeply, as explained below.

How to avoid passing callbacks down?

We’ve found that most people don’t enjoy manually passing callbacks through every level of a component tree. Even though it is more explicit, it can feel like a lot of “plumbing”.

In large component trees, an alternative we recommend is to pass down a dispatch function from useReducer via context:

const TodosDispatch = React.createContext(null);

function TodosApp() {
  // Note: `dispatch` won't change between re-renders
  const [todos, dispatch] = useReducer(todosReducer);

  return (
    <TodosDispatch.Provider value={dispatch}>
      <DeepTree todos={todos} />
    </TodosDispatch.Provider>
  );
}

Any child in the tree inside TodosApp can use the dispatch function to pass actions up to TodosApp:

function DeepChild(props) {
  // If we want to perform an action, we can get dispatch from context.
  const dispatch = useContext(TodosDispatch);

  function handleClick() {
    dispatch({ type: 'add', text: 'hello' });
  }

  return (
    <button onClick={handleClick}>Add todo</button>
  );
}

This is both more convenient from the maintenance perspective (no need to keep forwarding callbacks), and avoids the callback problem altogether. Passing dispatch down like this is the recommended pattern for deep updates.

Note that you can still choose whether to pass the application state down as props (more explicit) or as context (more convenient for very deep updates). If you use context to pass down the state too, use two different context types — the dispatch context never changes, so components that read it don’t need to rerender unless they also need the application state.
How to read an often-changing value from useCallback?

    Note

    We recommend to pass dispatch down in context rather than individual callbacks in props. The approach below is only mentioned here for completeness and as an escape hatch.

    Also note that this pattern might cause problems in the concurrent mode. We plan to provide more ergonomic alternatives in the future, but the safest solution right now is to always invalidate the callback if some value it depends on changes.

In some rare cases you might need to memoize a callback with useCallback but the memoization doesn’t work very well because the inner function has to be re-created too often. If the function you’re memoizing is an event handler and isn’t used during rendering, you can use ref as an instance variable, and save the last committed value into it manually:

function Form() {
  const [text, updateText] = useState('');
  const textRef = useRef();

  useEffect(() => {
    textRef.current = text; // Write it to the ref
  });

  const handleSubmit = useCallback(() => {
    const currentText = textRef.current; // Read it from the ref
    alert(currentText);
  }, [textRef]); // Don't recreate handleSubmit like [text] would do

  return (
    <>
      <input value={text} onChange={e => updateText(e.target.value)} />
      <ExpensiveTree onSubmit={handleSubmit} />
    </>
  );
}

This is a rather convoluted pattern but it shows that you can do this escape hatch optimization if you need it. It’s more bearable if you extract it to a custom Hook:

function Form() {
  const [text, updateText] = useState('');
  // Will be memoized even if `text` changes:
  const handleSubmit = useEventCallback(() => {
    alert(text);
  }, [text]);

  return (
    <>
      <input value={text} onChange={e => updateText(e.target.value)} />
      <ExpensiveTree onSubmit={handleSubmit} />
    </>
  );
}

function useEventCallback(fn, dependencies) {
  const ref = useRef(() => {
    throw new Error('Cannot call an event handler while rendering.');
  });

  useEffect(() => {
    ref.current = fn;
  }, [fn, ...dependencies]);

  return useCallback(() => {
    const fn = ref.current;
    return fn();
  }, [ref]);
}

In either case, we don’t recommend this pattern and only show it here for completeness. Instead, it is preferable to avoid passing callbacks deep down.
Under the Hood
How does React associate Hook calls with components?

React keeps track of the currently rendering component. Thanks to the Rules of Hooks, we know that Hooks are only called from React components (or custom Hooks — which are also only called from React components).

There is an internal list of “memory cells” associated with each component. They’re just JavaScript objects where we can put some data. When you call a Hook like useState(), it reads the current cell (or initializes it during the first render), and then moves the pointer to the next one. This is how multiple useState() calls each get independent local state.
What is the prior art for Hooks?

Hooks synthesize ideas from several different sources:

    Our old experiments with functional APIs in the react-future repository.
    React community’s experiments with render prop APIs, including Ryan Florence’s Reactions Component.
    Dominic Gannaway’s adopt keyword proposal as a sugar syntax for render props.
    State variables and state cells in DisplayScript.
    Reducer components in ReasonReact.
    Subscriptions in Rx.
    Algebraic effects in Multicore OCaml.

Sebastian Markbåge came up with the original design for Hooks, later refined by Andrew Clark, Sophie Alpert, Dominic Gannaway, and other members of the React team.

How to Contribute

React is one of Facebook’s first open source projects that is both under very active development and is also being used to ship code to everybody on facebook.com. We’re still working out the kinks to make contributing to this project as easy and transparent as possible, but we’re not quite there yet. Hopefully this document makes the process for contributing clear and answers some questions that you may have.
Code of Conduct

Facebook has adopted a Code of Conduct that we expect project participants to adhere to. Please read the full text so that you can understand what actions will and will not be tolerated.
Open Development

All work on React happens directly on GitHub. Both core team members and external contributors send pull requests which go through the same review process.
Branch Organization

We will do our best to keep the master branch in good shape, with tests passing at all times. But in order to move fast, we will make API changes that your application might not be compatible with. We recommend that you use the latest stable version of React.

If you send a pull request, please do it against the master branch. We maintain stable branches for major versions separately but we don’t accept pull requests to them directly. Instead, we cherry-pick non-breaking changes from master to the latest stable major version.
Semantic Versioning

React follows semantic versioning. We release patch versions for bugfixes, minor versions for new features, and major versions for any breaking changes. When we make breaking changes, we also introduce deprecation warnings in a minor version so that our users learn about the upcoming changes and migrate their code in advance.

We tag every pull request with a label marking whether the change should go in the next patch, minor, or a major version. We release new patch versions every few weeks, minor versions every few months, and major versions one or two times a year.

Every significant change is documented in the changelog file.
Bugs
Where to Find Known Issues

We are using GitHub Issues for our public bugs. We keep a close eye on this and try to make it clear when we have an internal fix in progress. Before filing a new task, try to make sure your problem doesn’t already exist.
Reporting New Issues

The best way to get your bug fixed is to provide a reduced test case. This JSFiddle template is a great starting point.
Security Bugs

Facebook has a bounty program for the safe disclosure of security bugs. With that in mind, please do not file public issues; go through the process outlined on that page.
How to Get in Touch

    IRC: #reactjs on freenode
    Discussion forum: discuss.reactjs.org

There is also an active community of React users on the Discord chat platform in case you need help with React.
Proposing a Change

If you intend to change the public API, or make any non-trivial changes to the implementation, we recommend filing an issue. This lets us reach an agreement on your proposal before you put significant effort into it.

If you’re only fixing a bug, it’s fine to submit a pull request right away but we still recommend to file an issue detailing what you’re fixing. This is helpful in case we don’t accept that specific fix but want to keep track of the issue.
Your First Pull Request

Working on your first Pull Request? You can learn how from this free video series:

How to Contribute to an Open Source Project on GitHub

To help you get your feet wet and get you familiar with our contribution process, we have a list of good first issues that contain bugs that have a relatively limited scope. This is a great place to get started.

If you decide to fix an issue, please be sure to check the comment thread in case somebody is already working on a fix. If nobody is working on it at the moment, please leave a comment stating that you intend to work on it so other people don’t accidentally duplicate your effort.

If somebody claims an issue but doesn’t follow up for more than two weeks, it’s fine to take it over but you should still leave a comment.
Sending a Pull Request

The core team is monitoring for pull requests. We will review your pull request and either merge it, request changes to it, or close it with an explanation. For API changes we may need to fix our internal uses at Facebook.com, which could cause some delay. We’ll do our best to provide updates and feedback throughout the process.

Before submitting a pull request, please make sure the following is done:

    Fork the repository and create your branch from master.
    Run yarn in the repository root.
    If you’ve fixed a bug or added code that should be tested, add tests!
    Ensure the test suite passes (yarn test). Tip: yarn test --watch TestName is helpful in development.
    Run yarn test-prod to test in the production environment. It supports the same options as yarn test.
    If you need a debugger, run yarn debug-test --watch TestName, open chrome://inspect, and press “Inspect”.
    Format your code with prettier (yarn prettier).
    Make sure your code lints (yarn lint). Tip: yarn linc to only check changed files.
    Run the Flow typechecks (yarn flow).
    If you haven’t already, complete the CLA.

Contributor License Agreement (CLA)

In order to accept your pull request, we need you to submit a CLA. You only need to do this once, so if you’ve done this for another Facebook open source project, you’re good to go. If you are submitting a pull request for the first time, just let us know that you have completed the CLA and we can cross-check with your GitHub username.

Complete your CLA here.
Contribution Prerequisites

    You have Node installed at v8.0.0+ and Yarn at v1.2.0+.
    You have gcc installed or are comfortable installing a compiler if needed. Some of our dependencies may require a compilation step. On OS X, the Xcode Command Line Tools will cover this. On Ubuntu, apt-get install build-essential will install the required packages. Similar commands should work on other Linux distros. Windows will require some additional steps, see the node-gyp installation instructions for details.
    You are familiar with Git.

Development Workflow

After cloning React, run yarn to fetch its dependencies. Then, you can run several commands:

    yarn lint checks the code style.
    yarn linc is like yarn lint but faster because it only checks files that differ in your branch.
    yarn test runs the complete test suite.
    yarn test --watch runs an interactive test watcher.
    yarn test <pattern> runs tests with matching filenames.
    yarn test-prod runs tests in the production environment. It supports all the same options as yarn test.
    yarn debug-test is just like yarn test but with a debugger. Open chrome://inspect and press “Inspect”.
    yarn flow runs the Flow typechecks.
    yarn build creates a build folder with all the packages.
    yarn build react/index,react-dom/index --type=UMD creates UMD builds of just React and ReactDOM.

We recommend running yarn test (or its variations above) to make sure you don’t introduce any regressions as you work on your change. However it can be handy to try your build of React in a real project.

First, run yarn build. This will produce pre-built bundles in build folder, as well as prepare npm packages inside build/packages.

The easiest way to try your changes is to run yarn build react/index,react-dom/index --type=UMD and then open fixtures/packaging/babel-standalone/dev.html. This file already uses react.development.js from the build folder so it will pick up your changes.

If you want to try your changes in your existing React project, you may copy build/dist/react.development.js, build/dist/react-dom.development.js, or any other build products into your app and use them instead of the stable version. If your project uses React from npm, you may delete react and react-dom in its dependencies and use yarn link to point them to your local build folder:

cd ~/path_to_your_react_clone/build/node_modules/react
yarn link
cd ~/path_to_your_react_clone/build/node_modules/react-dom
yarn link
cd /path/to/your/project
yarn link react react-dom

Every time you run yarn build in the React folder, the updated versions will appear in your project’s node_modules. You can then rebuild your project to try your changes.

We still require that your pull request contains unit tests for any new functionality. This way we can ensure that we don’t break your code in the future.
Style Guide

We use an automatic code formatter called Prettier. Run yarn prettier after making any changes to the code.

Then, our linter will catch most issues that may exist in your code. You can check the status of your code styling by simply running yarn linc.

However, there are still some styles that the linter cannot pick up. If you are unsure about something, looking at Airbnb’s Style Guide will guide you in the right direction.
Introductory Video

You may be interested in watching this short video (26 mins) which gives an introduction on how to contribute to React.
Video highlights:

    4:12 - Building and testing React locally
    6:07 - Creating and sending pull requests
    8:25 - Organizing code
    14:43 - React npm registry
    19:15 - Adding new React features

For a realistic overview of what it feels like to contribute to React for the first time, check out this entertaining ReactNYC talk.
Request for Comments (RFC)

Many changes, including bug fixes and documentation improvements can be implemented and reviewed via the normal GitHub pull request workflow.

Some changes though are “substantial”, and we ask that these be put through a bit of a design process and produce a consensus among the React core team.

The “RFC” (request for comments) process is intended to provide a consistent and controlled path for new features to enter the project. You can contribute by visiting the rfcs repository.
License

By contributing to React, you agree that your contributions will be licensed under its MIT license.
What Next?

Read the next section to learn how the codebase is organized.

Codebase Overview

This section will give you an overview of the React codebase organization, its conventions, and the implementation.

If you want to contribute to React we hope that this guide will help you feel more comfortable making changes.

We don’t necessarily recommend any of these conventions in React apps. Many of them exist for historical reasons and might change with time.
External Dependencies

React has almost no external dependencies. Usually, a require() points to a file in React’s own codebase. However, there are a few relatively rare exceptions.

The fbjs repository exists because React shares some small utilities with libraries like Relay, and we keep them in sync. We don’t depend on equivalent small modules in the Node ecosystem because we want Facebook engineers to be able to make changes to them whenever necessary. None of the utilities inside fbjs are considered to be public API, and they are only intended for use by Facebook projects such as React.
Top-Level Folders

After cloning the React repository, you will see a few top-level folders in it:

    packages contains metadata (such as package.json) and the source code (src subdirectory) for all packages in the React repository. If your change is related to the code, the src subdirectory of each package is where you’ll spend most of your time.
    fixtures contains a few small React test applications for contributors.
    build is the build output of React. It is not in the repository but it will appear in your React clone after you build it for the first time.

The documentation is hosted in a separate repository from React.

There are a few other top-level folders but they are mostly used for the tooling and you likely won’t ever encounter them when contributing.
Colocated Tests

We don’t have a top-level directory for unit tests. Instead, we put them into a directory called __tests__ relative to the files that they test.

For example, a test for setInnerHTML.js is located in __tests__/setInnerHTML-test.js right next to it.
Warnings and Invariants

The React codebase uses the warning module to display warnings:

var warning = require('warning');

warning(
  2 + 2 === 4,
  'Math is not working today.'
);

The warning is shown when the warning condition is false.

One way to think about it is that the condition should reflect the normal situation rather than the exceptional one.

It is a good idea to avoid spamming the console with duplicate warnings:

var warning = require('warning');

var didWarnAboutMath = false;
if (!didWarnAboutMath) {
  warning(
    2 + 2 === 4,
    'Math is not working today.'
  );
  didWarnAboutMath = true;
}

Warnings are only enabled in development. In production, they are completely stripped out. If you need to forbid some code path from executing, use invariant module instead:

var invariant = require('invariant');

invariant(
  2 + 2 === 4,
  'You shall not pass!'
);

The invariant is thrown when the invariant condition is false.

“Invariant” is just a way of saying “this condition always holds true”. You can think about it as making an assertion.

It is important to keep development and production behavior similar, so invariant throws both in development and in production. The error messages are automatically replaced with error codes in production to avoid negatively affecting the byte size.
Development and Production

You can use __DEV__ pseudo-global variable in the codebase to guard development-only blocks of code.

It is inlined during the compile step, and turns into process.env.NODE_ENV !== 'production' checks in the CommonJS builds.

For standalone builds, it becomes true in the unminified build, and gets completely stripped out with the if blocks it guards in the minified build.

if (__DEV__) {
  // This code will only run in development.
}

Flow

We recently started introducing Flow checks to the codebase. Files marked with the @flow annotation in the license header comment are being typechecked.

We accept pull requests adding Flow annotations to existing code. Flow annotations look like this:

ReactRef.detachRefs = function(
  instance: ReactInstance,
  element: ReactElement | string | number | null | false,
): void {
  // ...
}

When possible, new code should use Flow annotations. You can run yarn flow locally to check your code with Flow.
Dynamic Injection

React uses dynamic injection in some modules. While it is always explicit, it is still unfortunate because it hinders understanding of the code. The main reason it exists is because React originally only supported DOM as a target. React Native started as a React fork. We had to add dynamic injection to let React Native override some behaviors.

You may see modules declaring their dynamic dependencies like this:

// Dynamically injected
var textComponentClass = null;

// Relies on dynamically injected value
function createInstanceForText(text) {
  return new textComponentClass(text);
}

var ReactHostComponent = {
  createInstanceForText,

  // Provides an opportunity for dynamic injection
  injection: {
    injectTextComponentClass: function(componentClass) {
      textComponentClass = componentClass;
    },
  },
};

module.exports = ReactHostComponent;

The injection field is not handled specially in any way. But by convention, it means that this module wants to have some (presumably platform-specific) dependencies injected into it at runtime.

There are multiple injection points in the codebase. In the future, we intend to get rid of the dynamic injection mechanism and wire up all the pieces statically during the build.
Multiple Packages

React is a monorepo. Its repository contains multiple separate packages so that their changes can be coordinated together, and issues live in one place.
React Core

The “core” of React includes all the top-level React APIs, for example:

    React.createElement()
    React.Component
    React.Children

React core only includes the APIs necessary to define components. It does not include the reconciliation algorithm or any platform-specific code. It is used both by React DOM and React Native components.

The code for React core is located in packages/react in the source tree. It is available on npm as the react package. The corresponding standalone browser build is called react.js, and it exports a global called React.
Renderers

React was originally created for the DOM but it was later adapted to also support native platforms with React Native. This introduced the concept of “renderers” to React internals.

Renderers manage how a React tree turns into the underlying platform calls.

Renderers are also located in packages/:

    React DOM Renderer renders React components to the DOM. It implements top-level ReactDOM APIs and is available as react-dom npm package. It can also be used as standalone browser bundle called react-dom.js that exports a ReactDOM global.
    React Native Renderer renders React components to native views. It is used internally by React Native.
    React Test Renderer renders React components to JSON trees. It is used by the Snapshot Testing feature of Jest and is available as react-test-renderer npm package.

The only other officially supported renderer is react-art. It used to be in a separate GitHub repository but we moved it into the main source tree for now.

    Note:

    Technically the react-native-renderer is a very thin layer that teaches React to interact with React Native implementation. The real platform-specific code managing the native views lives in the React Native repository together with its components.

Reconcilers

Even vastly different renderers like React DOM and React Native need to share a lot of logic. In particular, the reconciliation algorithm should be as similar as possible so that declarative rendering, custom components, state, lifecycle methods, and refs work consistently across platforms.

To solve this, different renderers share some code between them. We call this part of React a “reconciler”. When an update such as setState() is scheduled, the reconciler calls render() on components in the tree and mounts, updates, or unmounts them.

Reconcilers are not packaged separately because they currently have no public API. Instead, they are exclusively used by renderers such as React DOM and React Native.
Stack Reconciler

The “stack” reconciler is the implementation powering React 15 and earlier. We have since stopped using it, but it is documented in detail in the next section.
Fiber Reconciler

The “fiber” reconciler is a new effort aiming to resolve the problems inherent in the stack reconciler and fix a few long-standing issues. It has been the default reconciler since React 16.

Its main goals are:

    Ability to split interruptible work in chunks.
    Ability to prioritize, rebase and reuse work in progress.
    Ability to yield back and forth between parents and children to support layout in React.
    Ability to return multiple elements from render().
    Better support for error boundaries.

You can read more about React Fiber Architecture here and here. While it has shipped with React 16, the async features are not enabled by default yet.

Its source code is located in packages/react-reconciler.
Event System

React implements a synthetic event system which is agnostic of the renderers and works both with React DOM and React Native. Its source code is located in packages/events.

There is a video with a deep code dive into it (66 mins).
What Next?

Read the next section to learn about the pre-React 16 implementation of reconciler in more detail. We haven’t documented the internals of the new reconciler yet.

Implementation Notes

This section is a collection of implementation notes for the stack reconciler.

It is very technical and assumes a strong understanding of React public API as well as how it’s divided into core, renderers, and the reconciler. If you’re not very familiar with the React codebase, read the codebase overview first.

It also assumes an understanding of the differences between React components, their instances, and elements.

The stack reconciler was used in React 15 and earlier. It is located at src/renderers/shared/stack/reconciler.
Video: Building React from Scratch

Paul O’Shannessy gave a talk about building React from scratch that largely inspired this document.

Both this document and his talk are simplifications of the real codebase so you might get a better understanding by getting familiar with both of them.
Overview

The reconciler itself doesn’t have a public API. Renderers like React DOM and React Native use it to efficiently update the user interface according to the React components written by the user.
Mounting as a Recursive Process

Let’s consider the first time you mount a component:

ReactDOM.render(<App />, rootEl);

React DOM will pass <App /> along to the reconciler. Remember that <App /> is a React element, that is, a description of what to render. You can think about it as a plain object:

console.log(<App />);
// { type: App, props: {} }

The reconciler will check if App is a class or a function.

If App is a function, the reconciler will call App(props) to get the rendered element.

If App is a class, the reconciler will instantiate an App with new App(props), call the componentWillMount() lifecycle method, and then will call the render() method to get the rendered element.

Either way, the reconciler will learn the element App “rendered to”.

This process is recursive. App may render to a <Greeting />, Greeting may render to a <Button />, and so on. The reconciler will “drill down” through user-defined components recursively as it learns what each component renders to.

You can imagine this process as a pseudocode:

function isClass(type) {
  // React.Component subclasses have this flag
  return (
    Boolean(type.prototype) &&
    Boolean(type.prototype.isReactComponent)
  );
}

// This function takes a React element (e.g. <App />)
// and returns a DOM or Native node representing the mounted tree.
function mount(element) {
  var type = element.type;
  var props = element.props;

  // We will determine the rendered element
  // by either running the type as function
  // or creating an instance and calling render().
  var renderedElement;
  if (isClass(type)) {
    // Component class
    var publicInstance = new type(props);
    // Set the props
    publicInstance.props = props;
    // Call the lifecycle if necessary
    if (publicInstance.componentWillMount) {
      publicInstance.componentWillMount();
    }
    // Get the rendered element by calling render()
    renderedElement = publicInstance.render();
  } else {
    // Component function
    renderedElement = type(props);
  }

  // This process is recursive because a component may
  // return an element with a type of another component.
  return mount(renderedElement);

  // Note: this implementation is incomplete and recurses infinitely!
  // It only handles elements like <App /> or <Button />.
  // It doesn't handle elements like <div /> or <p /> yet.
}

var rootEl = document.getElementById('root');
var node = mount(<App />);
rootEl.appendChild(node);

    Note:

    This really is a pseudo-code. It isn’t similar to the real implementation. It will also cause a stack overflow because we haven’t discussed when to stop the recursion.

Let’s recap a few key ideas in the example above:

    React elements are plain objects representing the component type (e.g. App) and the props.
    User-defined components (e.g. App) can be classes or functions but they all “render to” elements.
    “Mounting” is a recursive process that creates a DOM or Native tree given the top-level React element (e.g. <App />).

Mounting Host Elements

This process would be useless if we didn’t render something to the screen as a result.

In addition to user-defined (“composite”) components, React elements may also represent platform-specific (“host”) components. For example, Button might return a <div /> from its render method.

If element’s type property is a string, we are dealing with a host element:

console.log(<div />);
// { type: 'div', props: {} }

There is no user-defined code associated with host elements.

When the reconciler encounters a host element, it lets the renderer take care of mounting it. For example, React DOM would create a DOM node.

If the host element has children, the reconciler recursively mounts them following the same algorithm as above. It doesn’t matter whether children are host (like <div><hr /></div>), composite (like <div><Button /></div>), or both.

The DOM nodes produced by the child components will be appended to the parent DOM node, and recursively, the complete DOM structure will be assembled.

    Note:

    The reconciler itself is not tied to the DOM. The exact result of mounting (sometimes called “mount image” in the source code) depends on the renderer, and can be a DOM node (React DOM), a string (React DOM Server), or a number representing a native view (React Native).

If we were to extend the code to handle host elements, it would look like this:

function isClass(type) {
  // React.Component subclasses have this flag
  return (
    Boolean(type.prototype) &&
    Boolean(type.prototype.isReactComponent)
  );
}

// This function only handles elements with a composite type.
// For example, it handles <App /> and <Button />, but not a <div />.
function mountComposite(element) {
  var type = element.type;
  var props = element.props;

  var renderedElement;
  if (isClass(type)) {
    // Component class
    var publicInstance = new type(props);
    // Set the props
    publicInstance.props = props;
    // Call the lifecycle if necessary
    if (publicInstance.componentWillMount) {
      publicInstance.componentWillMount();
    }
    renderedElement = publicInstance.render();
  } else if (typeof type === 'function') {
    // Component function
    renderedElement = type(props);
  }

  // This is recursive but we'll eventually reach the bottom of recursion when
  // the element is host (e.g. <div />) rather than composite (e.g. <App />):
  return mount(renderedElement);
}

// This function only handles elements with a host type.
// For example, it handles <div /> and <p /> but not an <App />.
function mountHost(element) {
  var type = element.type;
  var props = element.props;
  var children = props.children || [];
  if (!Array.isArray(children)) {
    children = [children];
  }
  children = children.filter(Boolean);

  // This block of code shouldn't be in the reconciler.
  // Different renderers might initialize nodes differently.
  // For example, React Native would create iOS or Android views.
  var node = document.createElement(type);
  Object.keys(props).forEach(propName => {
    if (propName !== 'children') {
      node.setAttribute(propName, props[propName]);
    }
  });

  // Mount the children
  children.forEach(childElement => {
    // Children may be host (e.g. <div />) or composite (e.g. <Button />).
    // We will also mount them recursively:
    var childNode = mount(childElement);

    // This line of code is also renderer-specific.
    // It would be different depending on the renderer:
    node.appendChild(childNode);
  });

  // Return the DOM node as mount result.
  // This is where the recursion ends.
  return node;
}

function mount(element) {
  var type = element.type;
  if (typeof type === 'function') {
    // User-defined components
    return mountComposite(element);
  } else if (typeof type === 'string') {
    // Platform-specific components
    return mountHost(element);
  }
}

var rootEl = document.getElementById('root');
var node = mount(<App />);
rootEl.appendChild(node);

This is working but still far from how the reconciler is really implemented. The key missing ingredient is support for updates.
Introducing Internal Instances

The key feature of React is that you can re-render everything, and it won’t recreate the DOM or reset the state:

ReactDOM.render(<App />, rootEl);
// Should reuse the existing DOM:
ReactDOM.render(<App />, rootEl);

However, our implementation above only knows how to mount the initial tree. It can’t perform updates on it because it doesn’t store all the necessary information, such as all the publicInstances, or which DOM nodes correspond to which components.

The stack reconciler codebase solves this by making the mount() function a method and putting it on a class. There are drawbacks to this approach, and we are going in the opposite direction in the ongoing rewrite of the reconciler. Nevertheless this is how it works now.

Instead of separate mountHost and mountComposite functions, we will create two classes: DOMComponent and CompositeComponent.

Both classes have a constructor accepting the element, as well as a mount() method returning the mounted node. We will replace a top-level mount() function with a factory that instantiates the correct class:

function instantiateComponent(element) {
  var type = element.type;
  if (typeof type === 'function') {
    // User-defined components
    return new CompositeComponent(element);
  } else if (typeof type === 'string') {
    // Platform-specific components
    return new DOMComponent(element);
  }  
}

First, let’s consider the implementation of CompositeComponent:

class CompositeComponent {
  constructor(element) {
    this.currentElement = element;
    this.renderedComponent = null;
    this.publicInstance = null;
  }

  getPublicInstance() {
    // For composite components, expose the class instance.
    return this.publicInstance;
  }

  mount() {
    var element = this.currentElement;
    var type = element.type;
    var props = element.props;

    var publicInstance;
    var renderedElement;
    if (isClass(type)) {
      // Component class
      publicInstance = new type(props);
      // Set the props
      publicInstance.props = props;
      // Call the lifecycle if necessary
      if (publicInstance.componentWillMount) {
        publicInstance.componentWillMount();
      }
      renderedElement = publicInstance.render();
    } else if (typeof type === 'function') {
      // Component function
      publicInstance = null;
      renderedElement = type(props);
    }

    // Save the public instance
    this.publicInstance = publicInstance;

    // Instantiate the child internal instance according to the element.
    // It would be a DOMComponent for <div /> or <p />,
    // and a CompositeComponent for <App /> or <Button />:
    var renderedComponent = instantiateComponent(renderedElement);
    this.renderedComponent = renderedComponent;

    // Mount the rendered output
    return renderedComponent.mount();
  }
}

This is not much different from our previous mountComposite() implementation, but now we can save some information, such as this.currentElement, this.renderedComponent, and this.publicInstance, for use during updates.

Note that an instance of CompositeComponent is not the same thing as an instance of the user-supplied element.type. CompositeComponent is an implementation detail of our reconciler, and is never exposed to the user. The user-defined class is the one we read from element.type, and CompositeComponent creates an instance of it.

To avoid the confusion, we will call instances of CompositeComponent and DOMComponent “internal instances”. They exist so we can associate some long-lived data with them. Only the renderer and the reconciler are aware that they exist.

In contrast, we call an instance of the user-defined class a “public instance”. The public instance is what you see as this in the render() and other methods of your custom components.

The mountHost() function, refactored to be a mount() method on DOMComponent class, also looks familiar:

class DOMComponent {
  constructor(element) {
    this.currentElement = element;
    this.renderedChildren = [];
    this.node = null;
  }

  getPublicInstance() {
    // For DOM components, only expose the DOM node.
    return this.node;
  }

  mount() {
    var element = this.currentElement;
    var type = element.type;
    var props = element.props;
    var children = props.children || [];
    if (!Array.isArray(children)) {
      children = [children];
    }

    // Create and save the node
    var node = document.createElement(type);
    this.node = node;

    // Set the attributes
    Object.keys(props).forEach(propName => {
      if (propName !== 'children') {
        node.setAttribute(propName, props[propName]);
      }
    });

    // Create and save the contained children.
    // Each of them can be a DOMComponent or a CompositeComponent,
    // depending on whether the element type is a string or a function.
    var renderedChildren = children.map(instantiateComponent);
    this.renderedChildren = renderedChildren;

    // Collect DOM nodes they return on mount
    var childNodes = renderedChildren.map(child => child.mount());
    childNodes.forEach(childNode => node.appendChild(childNode));

    // Return the DOM node as mount result
    return node;
  }
}

The main difference after refactoring from mountHost() is that we now keep this.node and this.renderedChildren associated with the internal DOM component instance. We will also use them for applying non-destructive updates in the future.

As a result, each internal instance, composite or host, now points to its child internal instances. To help visualize this, if a function <App> component renders a <Button> class component, and Button class renders a <div>, the internal instance tree would look like this:

[object CompositeComponent] {
  currentElement: <App />,
  publicInstance: null,
  renderedComponent: [object CompositeComponent] {
    currentElement: <Button />,
    publicInstance: [object Button],
    renderedComponent: [object DOMComponent] {
      currentElement: <div />,
      node: [object HTMLDivElement],
      renderedChildren: []
    }
  }
}

In the DOM you would only see the <div>. However the internal instance tree contains both composite and host internal instances.

The composite internal instances need to store:

    The current element.
    The public instance if element type is a class.
    The single rendered internal instance. It can be either a DOMComponent or a CompositeComponent.

The host internal instances need to store:

    The current element.
    The DOM node.
    All the child internal instances. Each of them can be either a DOMComponent or a CompositeComponent.

If you’re struggling to imagine how an internal instance tree is structured in more complex applications, React DevTools can give you a close approximation, as it highlights host instances with grey, and composite instances with purple:
React DevTools tree

To complete this refactoring, we will introduce a function that mounts a complete tree into a container node, just like ReactDOM.render(). It returns a public instance, also like ReactDOM.render():

function mountTree(element, containerNode) {
  // Create the top-level internal instance
  var rootComponent = instantiateComponent(element);

  // Mount the top-level component into the container
  var node = rootComponent.mount();
  containerNode.appendChild(node);

  // Return the public instance it provides
  var publicInstance = rootComponent.getPublicInstance();
  return publicInstance;
}

var rootEl = document.getElementById('root');
mountTree(<App />, rootEl);

Unmounting

Now that we have internal instances that hold onto their children and the DOM nodes, we can implement unmounting. For a composite component, unmounting calls a lifecycle method and recurses.

class CompositeComponent {

  // ...

  unmount() {
    // Call the lifecycle method if necessary
    var publicInstance = this.publicInstance;
    if (publicInstance) {
      if (publicInstance.componentWillUnmount) {
        publicInstance.componentWillUnmount();
      }
    }

    // Unmount the single rendered component
    var renderedComponent = this.renderedComponent;
    renderedComponent.unmount();
  }
}

For DOMComponent, unmounting tells each child to unmount:

class DOMComponent {

  // ...

  unmount() {
    // Unmount all the children
    var renderedChildren = this.renderedChildren;
    renderedChildren.forEach(child => child.unmount());
  }
}

In practice, unmounting DOM components also removes the event listeners and clears some caches, but we will skip those details.

We can now add a new top-level function called unmountTree(containerNode) that is similar to ReactDOM.unmountComponentAtNode():

function unmountTree(containerNode) {
  // Read the internal instance from a DOM node:
  // (This doesn't work yet, we will need to change mountTree() to store it.)
  var node = containerNode.firstChild;
  var rootComponent = node._internalInstance;

  // Unmount the tree and clear the container
  rootComponent.unmount();
  containerNode.innerHTML = '';
}

In order for this to work, we need to read an internal root instance from a DOM node. We will modify mountTree() to add the _internalInstance property to the root DOM node. We will also teach mountTree() to destroy any existing tree so it can be called multiple times:

function mountTree(element, containerNode) {
  // Destroy any existing tree
  if (containerNode.firstChild) {
    unmountTree(containerNode);
  }

  // Create the top-level internal instance
  var rootComponent = instantiateComponent(element);

  // Mount the top-level component into the container
  var node = rootComponent.mount();
  containerNode.appendChild(node);

  // Save a reference to the internal instance
  node._internalInstance = rootComponent;

  // Return the public instance it provides
  var publicInstance = rootComponent.getPublicInstance();
  return publicInstance;
}

Now, running unmountTree(), or running mountTree() repeatedly, removes the old tree and runs the componentWillUnmount() lifecycle method on components.
Updating

In the previous section, we implemented unmounting. However React wouldn’t be very useful if each prop change unmounted and mounted the whole tree. The goal of the reconciler is to reuse existing instances where possible to preserve the DOM and the state:

var rootEl = document.getElementById('root');

mountTree(<App />, rootEl);
// Should reuse the existing DOM:
mountTree(<App />, rootEl);

We will extend our internal instance contract with one more method. In addition to mount() and unmount(), both DOMComponent and CompositeComponent will implement a new method called receive(nextElement):

class CompositeComponent {
  // ...

  receive(nextElement) {
    // ...
  }
}

class DOMComponent {
  // ...

  receive(nextElement) {
    // ...
  }
}

Its job is to do whatever is necessary to bring the component (and any of its children) up to date with the description provided by the nextElement.

This is the part that is often described as “virtual DOM diffing” although what really happens is that we walk the internal tree recursively and let each internal instance receive an update.
Updating Composite Components

When a composite component receives a new element, we run the componentWillUpdate() lifecycle method.

Then we re-render the component with the new props, and get the next rendered element:

class CompositeComponent {

  // ...

  receive(nextElement) {
    var prevProps = this.currentElement.props;
    var publicInstance = this.publicInstance;
    var prevRenderedComponent = this.renderedComponent;
    var prevRenderedElement = prevRenderedComponent.currentElement;

    // Update *own* element
    this.currentElement = nextElement;
    var type = nextElement.type;
    var nextProps = nextElement.props;

    // Figure out what the next render() output is
    var nextRenderedElement;
    if (isClass(type)) {
      // Component class
      // Call the lifecycle if necessary
      if (publicInstance.componentWillUpdate) {
        publicInstance.componentWillUpdate(nextProps);
      }
      // Update the props
      publicInstance.props = nextProps;
      // Re-render
      nextRenderedElement = publicInstance.render();
    } else if (typeof type === 'function') {
      // Component function
      nextRenderedElement = type(nextProps);
    }

    // ...

Next, we can look at the rendered element’s type. If the type has not changed since the last render, the component below can also be updated in place.

For example, if it returned <Button color="red" /> the first time, and <Button color="blue" /> the second time, we can just tell the corresponding internal instance to receive() the next element:

    // ...

    // If the rendered element type has not changed,
    // reuse the existing component instance and exit.
    if (prevRenderedElement.type === nextRenderedElement.type) {
      prevRenderedComponent.receive(nextRenderedElement);
      return;
    }

    // ...

However, if the next rendered element has a different type than the previously rendered element, we can’t update the internal instance. A <button> can’t “become” an <input>.

Instead, we have to unmount the existing internal instance and mount the new one corresponding to the rendered element type. For example, this is what happens when a component that previously rendered a <button /> renders an <input />:

    // ...

    // If we reached this point, we need to unmount the previously
    // mounted component, mount the new one, and swap their nodes.

    // Find the old node because it will need to be replaced
    var prevNode = prevRenderedComponent.getHostNode();

    // Unmount the old child and mount a new child
    prevRenderedComponent.unmount();
    var nextRenderedComponent = instantiateComponent(nextRenderedElement);
    var nextNode = nextRenderedComponent.mount();

    // Replace the reference to the child
    this.renderedComponent = nextRenderedComponent;

    // Replace the old node with the new one
    // Note: this is renderer-specific code and
    // ideally should live outside of CompositeComponent:
    prevNode.parentNode.replaceChild(nextNode, prevNode);
  }
}

To sum this up, when a composite component receives a new element, it may either delegate the update to its rendered internal instance, or unmount it and mount a new one in its place.

There is another condition under which a component will re-mount rather than receive an element, and that is when the element’s key has changed. We don’t discuss key handling in this document because it adds more complexity to an already complex tutorial.

Note that we needed to add a method called getHostNode() to the internal instance contract so that it’s possible to locate the platform-specific node and replace it during the update. Its implementation is straightforward for both classes:

class CompositeComponent {
  // ...

  getHostNode() {
    // Ask the rendered component to provide it.
    // This will recursively drill down any composites.
    return this.renderedComponent.getHostNode();
  }
}

class DOMComponent {
  // ...

  getHostNode() {
    return this.node;
  }  
}

Updating Host Components

Host component implementations, such as DOMComponent, update differently. When they receive an element, they need to update the underlying platform-specific view. In case of React DOM, this means updating the DOM attributes:

class DOMComponent {
  // ...

  receive(nextElement) {
    var node = this.node;
    var prevElement = this.currentElement;
    var prevProps = prevElement.props;
    var nextProps = nextElement.props;    
    this.currentElement = nextElement;

    // Remove old attributes.
    Object.keys(prevProps).forEach(propName => {
      if (propName !== 'children' && !nextProps.hasOwnProperty(propName)) {
        node.removeAttribute(propName);
      }
    });
    // Set next attributes.
    Object.keys(nextProps).forEach(propName => {
      if (propName !== 'children') {
        node.setAttribute(propName, nextProps[propName]);
      }
    });

    // ...

Then, host components need to update their children. Unlike composite components, they might contain more than a single child.

In this simplified example, we use an array of internal instances and iterate over it, either updating or replacing the internal instances depending on whether the received type matches their previous type. The real reconciler also takes element’s key in the account and track moves in addition to insertions and deletions, but we will omit this logic.

We collect DOM operations on children in a list so we can execute them in batch:

    // ...

    // These are arrays of React elements:
    var prevChildren = prevProps.children || [];
    if (!Array.isArray(prevChildren)) {
      prevChildren = [prevChildren];
    }
    var nextChildren = nextProps.children || [];
    if (!Array.isArray(nextChildren)) {
      nextChildren = [nextChildren];
    }
    // These are arrays of internal instances:
    var prevRenderedChildren = this.renderedChildren;
    var nextRenderedChildren = [];

    // As we iterate over children, we will add operations to the array.
    var operationQueue = [];

    // Note: the section below is extremely simplified!
    // It doesn't handle reorders, children with holes, or keys.
    // It only exists to illustrate the overall flow, not the specifics.

    for (var i = 0; i < nextChildren.length; i++) {
      // Try to get an existing internal instance for this child
      var prevChild = prevRenderedChildren[i];

      // If there is no internal instance under this index,
      // a child has been appended to the end. Create a new
      // internal instance, mount it, and use its node.
      if (!prevChild) {
        var nextChild = instantiateComponent(nextChildren[i]);
        var node = nextChild.mount();

        // Record that we need to append a node
        operationQueue.push({type: 'ADD', node});
        nextRenderedChildren.push(nextChild);
        continue;
      }

      // We can only update the instance if its element's type matches.
      // For example, <Button size="small" /> can be updated to
      // <Button size="large" /> but not to an <App />.
      var canUpdate = prevChildren[i].type === nextChildren[i].type;

      // If we can't update an existing instance, we have to unmount it
      // and mount a new one instead of it.
      if (!canUpdate) {
        var prevNode = prevChild.getHostNode();
        prevChild.unmount();

        var nextChild = instantiateComponent(nextChildren[i]);
        var nextNode = nextChild.mount();

        // Record that we need to swap the nodes
        operationQueue.push({type: 'REPLACE', prevNode, nextNode});
        nextRenderedChildren.push(nextChild);
        continue;
      }

      // If we can update an existing internal instance,
      // just let it receive the next element and handle its own update.
      prevChild.receive(nextChildren[i]);
      nextRenderedChildren.push(prevChild);
    }

    // Finally, unmount any children that don't exist:
    for (var j = nextChildren.length; j < prevChildren.length; j++) {
      var prevChild = prevRenderedChildren[j];
      var node = prevChild.getHostNode();
      prevChild.unmount();

      // Record that we need to remove the node
      operationQueue.push({type: 'REMOVE', node});
    }

    // Point the list of rendered children to the updated version.
    this.renderedChildren = nextRenderedChildren;

    // ...

As the last step, we execute the DOM operations. Again, the real reconciler code is more complex because it also handles moves:

    // ...

    // Process the operation queue.
    while (operationQueue.length > 0) {
      var operation = operationQueue.shift();
      switch (operation.type) {
      case 'ADD':
        this.node.appendChild(operation.node);
        break;
      case 'REPLACE':
        this.node.replaceChild(operation.nextNode, operation.prevNode);
        break;
      case 'REMOVE':
        this.node.removeChild(operation.node);
        break;
      }
    }
  }
}

And that is it for updating host components.
Top-Level Updates

Now that both CompositeComponent and DOMComponent implement the receive(nextElement) method, we can change the top-level mountTree() function to use it when the element type is the same as it was the last time:

function mountTree(element, containerNode) {
  // Check for an existing tree
  if (containerNode.firstChild) {
    var prevNode = containerNode.firstChild;
    var prevRootComponent = prevNode._internalInstance;
    var prevElement = prevRootComponent.currentElement;

    // If we can, reuse the existing root component
    if (prevElement.type === element.type) {
      prevRootComponent.receive(element);
      return;
    }

    // Otherwise, unmount the existing tree
    unmountTree(containerNode);
  }

  // ...

}

Now calling mountTree() two times with the same type isn’t destructive:

var rootEl = document.getElementById('root');

mountTree(<App />, rootEl);
// Reuses the existing DOM:
mountTree(<App />, rootEl);

These are the basics of how React works internally.
What We Left Out

This document is simplified compared to the real codebase. There are a few important aspects we didn’t address:

    Components can render null, and the reconciler can handle “empty slots” in arrays and rendered output.

    The reconciler also reads key from the elements, and uses it to establish which internal instance corresponds to which element in an array. A bulk of complexity in the actual React implementation is related to that.

    In addition to composite and host internal instance classes, there are also classes for “text” and “empty” components. They represent text nodes and the “empty slots” you get by rendering null.

    Renderers use injection to pass the host internal class to the reconciler. For example, React DOM tells the reconciler to use ReactDOMComponent as the host internal instance implementation.

    The logic for updating the list of children is extracted into a mixin called ReactMultiChild which is used by the host internal instance class implementations both in React DOM and React Native.

    The reconciler also implements support for setState() in composite components. Multiple updates inside event handlers get batched into a single update.

    The reconciler also takes care of attaching and detaching refs to composite components and host nodes.

    Lifecycle methods that are called after the DOM is ready, such as componentDidMount() and componentDidUpdate(), get collected into “callback queues” and are executed in a single batch.

    React puts information about the current update into an internal object called “transaction”. Transactions are useful for keeping track of the queue of pending lifecycle methods, the current DOM nesting for the warnings, and anything else that is “global” to a specific update. Transactions also ensure React “cleans everything up” after updates. For example, the transaction class provided by React DOM restores the input selection after any update.

Jumping into the Code

    ReactMount is where the code like mountTree() and unmountTree() from this tutorial lives. It takes care of mounting and unmounting top-level components. ReactNativeMount is its React Native analog.

    ReactDOMComponent is the equivalent of DOMComponent in this tutorial. It implements the host component class for React DOM renderer. ReactNativeBaseComponent is its React Native analog.

    ReactCompositeComponent is the equivalent of CompositeComponent in this tutorial. It handles calling user-defined components and maintaining their state.

    instantiateReactComponent contains the switch that picks the right internal instance class to construct for an element. It is equivalent to instantiateComponent() in this tutorial.

    ReactReconciler is a wrapper with mountComponent(), receiveComponent(), and unmountComponent() methods. It calls the underlying implementations on the internal instances, but also includes some code around them that is shared by all internal instance implementations.

    ReactChildReconciler implements the logic for mounting, updating, and unmounting children according to the key of their elements.

    ReactMultiChild implements processing the operation queue for child insertions, deletions, and moves independently of the renderer.

    mount(), receive(), and unmount() are really called mountComponent(), receiveComponent(), and unmountComponent() in React codebase for legacy reasons, but they receive elements.

    Properties on the internal instances start with an underscore, e.g. _currentElement. They are considered to be read-only public fields throughout the codebase.

Future Directions

Stack reconciler has inherent limitations such as being synchronous and unable to interrupt the work or split it in chunks. There is a work in progress on the new Fiber reconciler with a completely different architecture. In the future, we intend to replace stack reconciler with it, but at the moment it is far from feature parity.
Next Steps

Read the next section to learn about the guiding principles we use for React development.

Design Principles

We wrote this document so that you have a better idea of how we decide what React does and what React doesn’t do, and what our development philosophy is like. While we are excited to see community contributions, we are not likely to choose a path that violates one or more of these principles.

    Note:

    This document assumes a strong understanding of React. It describes the design principles of React itself, not React components or applications.

    For an introduction to React, check out Thinking in React instead.

Composition

The key feature of React is composition of components. Components written by different people should work well together. It is important to us that you can add functionality to a component without causing rippling changes throughout the codebase.

For example, it should be possible to introduce some local state into a component without changing any of the components using it. Similarly, it should be possible to add some initialization and teardown code to any component when necessary.

There is nothing “bad” about using state or lifecycle methods in components. Like any powerful feature, they should be used in moderation, but we have no intention to remove them. On the contrary, we think they are integral parts of what makes React useful. We might enable more functional patterns in the future, but both local state and lifecycle methods will be a part of that model.

Components are often described as “just functions” but in our view they need to be more than that to be useful. In React, components describe any composable behavior, and this includes rendering, lifecycle, and state. Some external libraries like Relay augment components with other responsibilities such as describing data dependencies. It is possible that those ideas might make it back into React too in some form.
Common Abstraction

In general we resist adding features that can be implemented in userland. We don’t want to bloat your apps with useless library code. However, there are exceptions to this.

For example, if React didn’t provide support for local state or lifecycle methods, people would create custom abstractions for them. When there are multiple abstractions competing, React can’t enforce or take advantage of the properties of either of them. It has to work with the lowest common denominator.

This is why sometimes we add features to React itself. If we notice that many components implement a certain feature in incompatible or inefficient ways, we might prefer to bake it into React. We don’t do it lightly. When we do it, it’s because we are confident that raising the abstraction level benefits the whole ecosystem. State, lifecycle methods, cross-browser event normalization are good examples of this.

We always discuss such improvement proposals with the community. You can find some of those discussions by the “big picture” label on the React issue tracker.
Escape Hatches

React is pragmatic. It is driven by the needs of the products written at Facebook. While it is influenced by some paradigms that are not yet fully mainstream such as functional programming, staying accessible to a wide range of developers with different skills and experience levels is an explicit goal of the project.

If we want to deprecate a pattern that we don’t like, it is our responsibility to consider all existing use cases for it and educate the community about the alternatives before we deprecate it. If some pattern that is useful for building apps is hard to express in a declarative way, we will provide an imperative API for it. If we can’t figure out a perfect API for something that we found necessary in many apps, we will provide a temporary subpar working API as long as it is possible to get rid of it later and it leaves the door open for future improvements.
Stability

We value API stability. At Facebook, we have more than 50 thousand components using React. Many other companies, including Twitter and Airbnb, are also heavy users of React. This is why we are usually reluctant to change public APIs or behavior.

However we think stability in the sense of “nothing changes” is overrated. It quickly turns into stagnation. Instead, we prefer the stability in the sense of “It is heavily used in production, and when something changes, there is a clear (and preferably automated) migration path.”

When we deprecate a pattern, we study its internal usage at Facebook and add deprecation warnings. They let us assess the impact of the change. Sometimes we back out if we see that it is too early, and we need to think more strategically about getting the codebases to the point where they are ready for this change.

If we are confident that the change is not too disruptive and the migration strategy is viable for all use cases, we release the deprecation warning to the open source community. We are closely in touch with many users of React outside of Facebook, and we monitor popular open source projects and guide them in fixing those deprecations.

Given the sheer size of the Facebook React codebase, successful internal migration is often a good indicator that other companies won’t have problems either. Nevertheless sometimes people point out additional use cases we haven’t thought of, and we add escape hatches for them or rethink our approach.

We don’t deprecate anything without a good reason. We recognize that sometimes deprecations warnings cause frustration but we add them because deprecations clean up the road for the improvements and new features that we and many people in the community consider valuable.

For example, we added a warning about unknown DOM props in React 15.2.0. Many projects were affected by this. However fixing this warning is important so that we can introduce the support for custom attributes to React. There is a reason like this behind every deprecation that we add.

When we add a deprecation warning, we keep it for the rest of the current major version, and change the behavior in the next major version. If there is a lot of repetitive manual work involved, we release a codemod script that automates most of the change. Codemods enable us to move forward without stagnation in a massive codebase, and we encourage you to use them as well.

You can find the codemods that we released in the react-codemod repository.
Interoperability

We place high value in interoperability with existing systems and gradual adoption. Facebook has a massive non-React codebase. Its website uses a mix of a server-side component system called XHP, internal UI libraries that came before React, and React itself. It is important to us that any product team can start using React for a small feature rather than rewrite their code to bet on it.

This is why React provides escape hatches to work with mutable models, and tries to work well together with other UI libraries. You can wrap an existing imperative UI into a declarative component, and vice versa. This is crucial for gradual adoption.
Scheduling

Even when your components are described as functions, when you use React you don’t call them directly. Every component returns a description of what needs to be rendered, and that description may include both user-written components like <LikeButton> and platform-specific components like <div>. It is up to React to “unroll” <LikeButton> at some point in the future and actually apply changes to the UI tree according to the render results of the components recursively.

This is a subtle distinction but a powerful one. Since you don’t call that component function but let React call it, it means React has the power to delay calling it if necessary. In its current implementation React walks the tree recursively and calls render functions of the whole updated tree during a single tick. However in the future it might start delaying some updates to avoid dropping frames.

This is a common theme in React design. Some popular libraries implement the “push” approach where computations are performed when the new data is available. React, however, sticks to the “pull” approach where computations can be delayed until necessary.

React is not a generic data processing library. It is a library for building user interfaces. We think that it is uniquely positioned in an app to know which computations are relevant right now and which are not.

If something is offscreen, we can delay any logic related to it. If data is arriving faster than the frame rate, we can coalesce and batch updates. We can prioritize work coming from user interactions (such as an animation caused by a button click) over less important background work (such as rendering new content just loaded from the network) to avoid dropping frames.

To be clear, we are not taking advantage of this right now. However the freedom to do something like this is why we prefer to have control over scheduling, and why setState() is asynchronous. Conceptually, we think of it as “scheduling an update”.

The control over scheduling would be harder for us to gain if we let the user directly compose views with a “push” based paradigm common in some variations of Functional Reactive Programming. We want to own the “glue” code.

It is a key goal for React that the amount of the user code that executes before yielding back into React is minimal. This ensures that React retains the capability to schedule and split work in chunks according to what it knows about the UI.

There is an internal joke in the team that React should have been called “Schedule” because React does not want to be fully “reactive”.
Developer Experience

Providing a good developer experience is important to us.

For example, we maintain React DevTools which let you inspect the React component tree in Chrome and Firefox. We have heard that it brings a big productivity boost both to the Facebook engineers and to the community.

We also try to go an extra mile to provide helpful developer warnings. For example, React warns you in development if you nest tags in a way that the browser doesn’t understand, or if you make a common typo in the API. Developer warnings and the related checks are the main reason why the development version of React is slower than the production version.

The usage patterns that we see internally at Facebook help us understand what the common mistakes are, and how to prevent them early. When we add new features, we try to anticipate the common mistakes and warn about them.

We are always looking out for ways to improve the developer experience. We love to hear your suggestions and accept your contributions to make it even better.
Debugging

When something goes wrong, it is important that you have breadcrumbs to trace the mistake to its source in the codebase. In React, props and state are those breadcrumbs.

If you see something wrong on the screen, you can open React DevTools, find the component responsible for rendering, and then see if the props and state are correct. If they are, you know that the problem is in the component’s render() function, or some function that is called by render(). The problem is isolated.

If the state is wrong, you know that the problem is caused by one of the setState() calls in this file. This, too, is relatively simple to locate and fix because usually there are only a few setState() calls in a single file.

If the props are wrong, you can traverse the tree up in the inspector, looking for the component that first “poisoned the well” by passing bad props down.

This ability to trace any UI to the data that produced it in the form of current props and state is very important to React. It is an explicit design goal that state is not “trapped” in closures and combinators, and is available to React directly.

While the UI is dynamic, we believe that synchronous render() functions of props and state turn debugging from guesswork into a boring but finite procedure. We would like to preserve this constraint in React even though it makes some use cases, like complex animations, harder.
Configuration

We find global runtime configuration options to be problematic.

For example, it is occasionally requested that we implement a function like React.configure(options) or React.register(component). However this poses multiple problems, and we are not aware of good solutions to them.

What if somebody calls such a function from a third-party component library? What if one React app embeds another React app, and their desired configurations are incompatible? How can a third-party component specify that it requires a particular configuration? We think that global configuration doesn’t work well with composition. Since composition is central to React, we don’t provide global configuration in code.

We do, however, provide some global configuration on the build level. For example, we provide separate development and production builds. We may also add a profiling build in the future, and we are open to considering other build flags.
Beyond the DOM

We see the value of React in the way it allows us to write components that have fewer bugs and compose together well. DOM is the original rendering target for React but React Native is just as important both to Facebook and the community.

Being renderer-agnostic is an important design constraint of React. It adds some overhead in the internal representations. On the other hand, any improvements to the core translate across platforms.

Having a single programming model lets us form engineering teams around products instead of platforms. So far the tradeoff has been worth it for us.
Implementation

We try to provide elegant APIs where possible. We are much less concerned with the implementation being elegant. The real world is far from perfect, and to a reasonable extent we prefer to put the ugly code into the library if it means the user does not have to write it. When we evaluate new code, we are looking for an implementation that is correct, performant and affords a good developer experience. Elegance is secondary.

We prefer boring code to clever code. Code is disposable and often changes. So it is important that it doesn’t introduce new internal abstractions unless absolutely necessary. Verbose code that is easy to move around, change and remove is preferred to elegant code that is prematurely abstracted and hard to change.
Optimized for Tooling

Some commonly used APIs have verbose names. For example, we use componentDidMount() instead of didMount() or onMount(). This is intentional. The goal is to make the points of interaction with the library highly visible.

In a massive codebase like Facebook, being able to search for uses of specific APIs is very important. We value distinct verbose names, and especially for the features that should be used sparingly. For example, dangerouslySetInnerHTML is hard to miss in a code review.

Optimizing for search is also important because of our reliance on codemods to make breaking changes. We want it to be easy and safe to apply vast automated changes across the codebase, and unique verbose names help us achieve this. Similarly, distinctive names make it easy to write custom lint rules about using React without worrying about potential false positives.

JSX plays a similar role. While it is not required with React, we use it extensively at Facebook both for aesthetic and pragmatic reasons.

In our codebase, JSX provides an unambiguous hint to the tools that they are dealing with a React element tree. This makes it possible to add build-time optimizations such as hoisting constant elements, safely lint and codemod internal component usage, and include JSX source location into the warnings.
Dogfooding

We try our best to address the problems raised by the community. However we are likely to prioritize the issues that people are also experiencing internally at Facebook. Perhaps counter-intuitively, we think this is the main reason why the community can bet on React.

Heavy internal usage gives us the confidence that React won’t disappear tomorrow. React was created at Facebook to solve its problems. It brings tangible business value to the company and is used in many of its products. Dogfooding it means that our vision stays sharp and we have a focused direction going forward.

This doesn’t mean that we ignore the issues raised by the community. For example, we added support for web components and SVG to React even though we don’t rely on either of them internally. We are actively listening to your pain points and address them to the best of our ability. The community is what makes React special to us, and we are honored to contribute back.

After releasing many open source projects at Facebook, we have learned that trying to make everyone happy at the same time produced projects with poor focus that didn’t grow well. Instead, we found that picking a small audience and focusing on making them happy brings a positive net effect. That’s exactly what we did with React, and so far solving the problems encountered by Facebook product teams has translated well to the open source community.

The downside of this approach is that sometimes we fail to give enough focus to the things that Facebook teams don’t have to deal with, such as the “getting started” experience. We are acutely aware of this, and we are thinking of how to improve in a way that would benefit everyone in the community without making the same mistakes we did with open source projects before.

AJAX and APIs
How can I make an AJAX call?

You can use any AJAX library you like with React. Some popular ones are Axios, jQuery AJAX, and the browser built-in window.fetch.
Where in the component lifecycle should I make an AJAX call?

You should populate data with AJAX calls in the componentDidMount lifecycle method. This is so you can use setState to update your component when the data is retrieved.
Example: Using AJAX results to set local state

The component below demonstrates how to make an AJAX call in componentDidMount to populate local component state.

The example API returns a JSON object like this:

{
  "items": [
    { "id": 1, "name": "Apples",  "price": "$2" },
    { "id": 2, "name": "Peaches", "price": "$5" }
  ] 
}

class MyComponent extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
      error: null,
      isLoaded: false,
      items: []
    };
  }

  componentDidMount() {
    fetch("https://api.example.com/items")
      .then(res => res.json())
      .then(
        (result) => {
          this.setState({
            isLoaded: true,
            items: result.items
          });
        },
        // Note: it's important to handle errors here
        // instead of a catch() block so that we don't swallow
        // exceptions from actual bugs in components.
        (error) => {
          this.setState({
            isLoaded: true,
            error
          });
        }
      )
  }

  render() {
    const { error, isLoaded, items } = this.state;
    if (error) {
      return <div>Error: {error.message}</div>;
    } else if (!isLoaded) {
      return <div>Loading...</div>;
    } else {
      return (
        <ul>
          {items.map(item => (
            <li key={item.name}>
              {item.name} {item.price}
            </li>
          ))}
        </ul>
      );
    }
  }
}

Babel, JSX, and Build Steps
Do I need to use JSX with React?

No! Check out “React Without JSX” to learn more.
Do I need to use ES6 (+) with React?

No! Check out “React Without ES6” to learn more.
How can I write comments in JSX?

<div>
  {/* Comment goes here */}
  Hello, {name}!
</div>

<div>
  {/* It also works 
  for multi-line comments. */}
  Hello, {name}! 
</div>

Passing Functions to Components

<button onClick={this.handleClick}>

class Foo extends Component {
  constructor(props) {
    super(props);
    this.handleClick = this.handleClick.bind(this);
  }
  handleClick() {
    console.log('Click happened');
  }
  render() {
    return <button onClick={this.handleClick}>Click Me</button>;
  }
}

Bind in Render

class Foo extends Component {
  handleClick() {
    console.log('Click happened');
  }
  render() {
    return <button onClick={this.handleClick.bind(this)}>Click Me</button>;
  }
}

    Using Function.prototype.bind in render creates a new function each time the component renders, which may have performance implications (see below).

Arrow Function in Render

class Foo extends Component {
  handleClick() {
    console.log('Click happened');
  }
  render() {
    return <button onClick={() => this.handleClick()}>Click Me</button>;
  }
}

    Using an arrow function in render creates a new function each time the component renders, which may break optimizations based on strict identity comparison.

Why is binding necessary at all?

In JavaScript, these two code snippets are not equivalent:

obj.method();

var method = obj.method;
method();

Binding methods helps ensure that the second snippet works the same way as the first one.

With React, typically you only need to bind the methods you pass to other components. For example, <button onClick={this.handleClick}> passes this.handleClick so you want to bind it. However, it is unnecessary to bind the render method or the lifecycle methods: we don’t pass them to other components.

Make sure you aren’t calling the function when you pass it to the component:

render() {
  // Wrong: handleClick is called instead of passed as a reference!
  return <button onClick={this.handleClick()}>Click Me</button>
}

Instead, pass the function itself (without parens):

render() {
  // Correct: handleClick is passed as a reference!
  return <button onClick={this.handleClick}>Click Me</button>
}

How do I pass a parameter to an event handler or callback?

<button onClick={() => this.handleClick(id)} />
<button onClick={this.handleClick.bind(this, id)} />

const A = 65 // ASCII character code

class Alphabet extends React.Component {
  constructor(props) {
    super(props);
    this.handleClick = this.handleClick.bind(this);
    this.state = {
      justClicked: null,
      letters: Array.from({length: 26}, (_, i) => String.fromCharCode(A + i))
    };
  }
  handleClick(letter) {
    this.setState({ justClicked: letter });
  }
  render() {
    return (
      <div>
        Just clicked: {this.state.justClicked}
        <ul>
          {this.state.letters.map(letter =>
            <li key={letter} onClick={() => this.handleClick(letter)}>
              {letter}
            </li>
          )}
        </ul>
      </div>
    )
  }
}

Alternately, you can use DOM APIs to store data needed for event handlers. Consider this approach if you need to optimize a large number of elements or have a render tree that relies on React.PureComponent equality checks.

const A = 65 // ASCII character code

class Alphabet extends React.Component {
  constructor(props) {
    super(props);
    this.handleClick = this.handleClick.bind(this);
    this.state = {
      justClicked: null,
      letters: Array.from({length: 26}, (_, i) => String.fromCharCode(A + i))
    };
  }

  handleClick(e) {
    this.setState({
      justClicked: e.target.dataset.letter
    });
  }

  render() {
    return (
      <div>
        Just clicked: {this.state.justClicked}
        <ul>
          {this.state.letters.map(letter =>
            <li key={letter} data-letter={letter} onClick={this.handleClick}>
              {letter}
            </li>
          )}
        </ul>
      </div>
    )
  }
}

How can I prevent a function from being called too quickly or too many times in a row?

If you have an event handler such as onClick or onScroll and want to prevent the callback from being fired too quickly, then you can limit the rate at which callback is executed. This can be done by using:

    throttling: sample changes based on a time based frequency (eg _.throttle)
    debouncing: publish changes after a period of inactivity (eg _.debounce)
    requestAnimationFrame throttling: sample changes based on requestAnimationFrame (eg raf-schd)

    _.debounce, _.throttle and raf-schd provide a cancel method to cancel delayed callbacks. You should either call this method from componentWillUnmount or check to ensure that the component is still mounted within the delayed function.

Throttle

Throttling prevents a function from being called more than once in a given window of time. The example below throttles a “click” handler to prevent calling it more than once per second.

import throttle from 'lodash.throttle';

class LoadMoreButton extends React.Component {
  constructor(props) {
    super(props);
    this.handleClick = this.handleClick.bind(this);
    this.handleClickThrottled = throttle(this.handleClick, 1000);
  }

  componentWillUnmount() {
    this.handleClickThrottled.cancel();
  }

  render() {
    return <button onClick={this.handleClickThrottled}>Load More</button>;
  }

  handleClick() {
    this.props.loadMore();
  }
}

Debounce

Debouncing ensures that a function will not be executed until after a certain amount of time has passed since it was last called. This can be useful when you have to perform some expensive calculation in response to an event that might dispatch rapidly (eg scroll or keyboard events). The example below debounces text input with a 250ms delay.

import debounce from 'lodash.debounce';

class Searchbox extends React.Component {
  constructor(props) {
    super(props);
    this.handleChange = this.handleChange.bind(this);
    this.emitChangeDebounced = debounce(this.emitChange, 250);
  }

  componentWillUnmount() {
    this.emitChangeDebounced.cancel();
  }

  render() {
    return (
      <input
        type="text"
        onChange={this.handleChange}
        placeholder="Search..."
        defaultValue={this.props.value}
      />
    );
  }

  handleChange(e) {
    // React pools events, so we read the value before debounce.
    // Alternately we could call `event.persist()` and pass the entire event.
    // For more info see reactjs.org/docs/events.html#event-pooling
    this.emitChangeDebounced(e.target.value);
  }

  emitChange(value) {
    this.props.onChange(value);
  }
}

requestAnimationFrame throttling

requestAnimationFrame is a way of queuing a function to be executed in the browser at the optimal time for rendering performance. A function that is queued with requestAnimationFrame will fire in the next frame. The browser will work hard to ensure that there are 60 frames per second (60 fps). However, if the browser is unable to it will naturally limit the amount of frames in a second. For example, a device might only be able to handle 30 fps and so you will only get 30 frames in that second. Using requestAnimationFrame for throttling is a useful technique in that it prevents you from doing more than 60 updates in a second. If you are doing 100 updates in a second this creates additional work for the browser that the user will not see anyway.

    Note:

    Using this technique will only capture the last published value in a frame. You can see an example of how this optimization works on MDN

import rafSchedule from 'raf-schd';

class ScrollListener extends React.Component {
  constructor(props) {
    super(props);

    this.handleScroll = this.handleScroll.bind(this);

    // Create a new function to schedule updates.
    this.scheduleUpdate = rafSchedule(
      point => this.props.onScroll(point)
    );
  }

  handleScroll(e) {
    // When we receive a scroll event, schedule an update.
    // If we receive many updates within a frame, we'll only publish the latest value.
    this.scheduleUpdate({ x: e.clientX, y: e.clientY });
  }

  componentWillUnmount() {
    // Cancel any pending updates since we're unmounting.
    this.scheduleUpdate.cancel();
  }

  render() {
    return (
      <div
        style={{ overflow: 'scroll' }}
        onScroll={this.handleScroll}
      >
        <img src="/my-huge-image.jpg" />
      </div>
    );
  }
}

Testing your rate limiting

When testing your rate limiting code works correctly it is helpful to have the ability to fast forward time. If you are using jest then you can use mock timers to fast forward time. If you are using requestAnimationFrame throttling then you may find raf-stub to be a useful tool to control the ticking of animation frames.

## What is the difference between state and props?

props (short for “properties”) and state are both plain JavaScript objects. While both hold information that influences the output of render, they are different in one important way: props get passed to the component (similar to function parameters) whereas state is managed within the component (similar to variables declared within a function).

Calls to setState are asynchronous - don’t rely on this.state to reflect the new value immediately after calling setState. Pass an updater function instead of an object if you need to compute values based on the current state (see below for details).

```js
incrementCount() {
  // Note: this will *not* work as intended.
  this.setState({count: this.state.count + 1});
}

handleSomething() {
  // Let's say `this.state.count` starts at 0.
  this.incrementCount();
  this.incrementCount();
  this.incrementCount();
  // When React re-renders the component, `this.state.count` will be 1, but you expected 3.

  // This is because `incrementCount()` function above reads from `this.state.count`,
  // but React doesn't update `this.state.count` until the component is re-rendered.
  // So `incrementCount()` ends up reading `this.state.count` as 0 every time, and sets it to 1.

  // The fix is described below!
}
```

Pass a function instead of an object to setState to ensure the call always uses the most updated version of state (see below).

Passing an update function allows you to access the current state value inside the updater. Since setState calls are batched, this lets you chain updates and ensure they build on top of each other instead of conflicting:

incrementCount() {
  this.setState((state) => {
    // Important: read `state` instead of `this.state` when updating.
    return {count: state.count + 1}
  });
}

handleSomething() {
  // Let's say `this.state.count` starts at 0.
  this.incrementCount();
  this.incrementCount();
  this.incrementCount();

  // If you read `this.state.count` now, it would still be 0.
  // But when React re-renders the component, it will be 3.
}

As explained in the previous section, React intentionally “waits” until all components call setState() in their event handlers before starting to re-render. This boosts performance by avoiding unnecessary re-renders.
