~/build/firefox/firefox/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/browser/content/browser/browser.xul

https://blog.csdn.net/wangmx1993328/article/details/81164440


https://legacy.gitbook.com/

https://developer.mozilla.org/en-US/docs/Mozilla/Tech/XUL/Tutorial

https://developer.mozilla.org/en-US/docs/Mozilla/Tech/XUL

sudo apt-get install mercurial
wget -O bootstrap.py https://hg.mozilla.org/mozilla-central/raw-file/default/python/mozboot/bin/bootstrap.py && python bootstrap.py

They aren’t complicated, but there are a few prerequisites to building Firefox on Linux. You need:

    A 64-bit installation of Linux. You can check by opening a terminal window; if uname -m returns x86_64 you can proceed.
    Next, you’ll need Python installed. You can check with python --version to see if you have it already. If not, you can install it with your distribution’s package manager. Make sure your system is up to date!
    Finally, a reasonably fast internet connection and 30GB of free disk space.

Getting Started.

Getting set up on Linux is fast and easy.

If you don’t have one yet, create a "src" directory for yourself under your home directory:

mkdir src && cd src

Next download the bootstrap.py script and save it in your src/ directory.

Building Firefox in Linux on top of a non-native file system - for example, on a mounted NTFS partition - is explicitly not supported. While a build environment like this may succeed it may also fail while claiming to have succeeded, which can be quite difficult to diagnose and fix.

And finally, in your terminal change directory to src and start the bootstrapper like this:

cd ~/src 
python bootstrap.py

... follow the prompts.
Getting Access

Bugzilla is Mozilla’s issue tracker; you'll need to be able to log into it to comment on a bug or submit a patch.  You can either use your GitHub account, or you can sign up for a Bugzilla account here.

As well as Bugzilla, much of Mozilla’s internal communication happens over IRC. You can download Limechat here for mac, Chatzilla here for Linux and learn how to connect to Mozilla with IRC here. If you’re just getting started or have questions about getting set up you can join us in the #introduction channel, where some of our community members hang out to try and help new contributors get rolling.
Get The Code

If you haven't let bootstrap.py download mozilla-unified, you can clone mozilla-central repository with the following command, in your terminal:

hg clone https://hg.mozilla.org/mozilla-central

this could take some time depending on your network connection and environment.

If you have any network connection issues and cannot clone with the above command, try Mercurial Bundle.
Let’s Build Firefox

You’re ready; now we can tie it all together. In your terminal:

cd mozilla-central

and then

./mach build

If you encounter any error related to LLVM/Clang on Ubuntu or Debian, download the latest version of LLVM and Clang and then re-run ./mach build.

And you’re on your way, building your own copy of Firefox from source. Don’t be discouraged if this takes a while; this takes some time on even the fastest modern machines, and as much as two hours or more on older hardware.
Join Mozillians.org!

There’s one more thing you can do for yourself while you’re waiting: create an account for yourself on Mozillians. Mozillians is the Mozilla community directory, where you can connect with people who share your interests, projects or countries. This step is optional, but we think it will be worth your while.
Now the fun starts.

You have the code, you’ve compiled Firefox. Fire it up with ./mach run and you’re ready to start hacking. The next steps are up to you: join us on IRC in the #introduction channel, follow StartMozilla on Twitter and find a bug to start working on.

 
Detailed Requirements
General considerations

    2GB RAM with an additional 2GB of available swap space is the bare minimum, and more RAM is always better - having 8GB or more will dramatically improve build time.
    30 GB free disk space.
    A 64-bit x86 CPU and a 64-bit OS. As of early 2015 it is no longer possible to do a full build of Firefox from source on most 32-bit systems; a 64-bit OS is required. "Artifact builds" may be possible, but are not a supported configuration. On Linux you can determine this by typing "uname -a" in a terminal.
    GCC 4.9 is required as of Firefox 50. The Firefox codebase relies on some C++ features that are not supported in earlier versions of GCC, and as of December 2016 causes some errors in GCC 5.x or later. You can learn more about the features we use and their compiler support here
    autoconf 2.13. Most Linux distros now install a later version of autoconf, which the build system cannot use, reporting the error "*** Couldn't find autoconf 2.13.  Stop." However a separate autoconf2.13 package is usually available. To install autoconf 2.13 in Debian based distros copy this line and paste it into a terminal window:

sudo apt-get install autoconf2.13

    If you are on a Fedora machine then simply install the following prerequisites from the terminal window:

sudo dnf install @development-tools @c-development autoconf213 gtk2-devel gtk3-devel libXt-devel GConf2-devel dbus-glib-devel yasm-devel alsa-lib-devel pulseaudio-libs-devel 

Requirements for Debian / Ubuntu users

You need a number of different packages:

# the rust compiler
aptitude install rustc

# the rust package manager
aptitude install cargo

# the required (old) version of autoconf
aptitude install autoconf2.13

# the headers of important libs
aptitude install libgtk-3-dev
aptitude install libgconf2-dev
aptitude install libdbus-glib-1-dev
aptitude install libpulse-dev

# an assembler for compiling webm
aptitude install yasm

 
One-Line Bootstrapping

Our system bootstrapping script can automatically install the required dependencies. You can download and run it by copying this line and pasting it into a terminal window:

wget -q https://hg.mozilla.org/mozilla-central/raw-file/default/python/mozboot/bin/bootstrap.py -O bootstrap.py && python bootstrap.py

Note: piping bootstrap.py to stdin of a python process will cause interactive prompts in the bootstrap script to fail, causing the bootstrap process to fail. You must run Python against a local file.

If the above command fails, the reason is often because some Linux distributions ship with an outdated list of root certificates. In this case, you should upgrade your Linux distribution or use your browser to download the file. That ensures that you will get it from the right source.

If you get an error from this process, consider filing a bug saying that the bootstrapper didn't work and contact Mike Hoye directly for help. Please include the error message and some details about your operating system.

If you have already checked out the source code via Mercurial or Git you can also use mach with the bootstrap command:

./mach bootstrap

Common Bootstrapper Failures

wget: command not found

You may not have wget (or curl) installed. In that case, you can either install it via your package manager: 

On Debian-based distros like Ubuntu:

sudo apt install wget 

On Fedora-based distros:

sudo dnf install wget

or you can just download bootstrap.py using your browser and then run it with this command:

python bootstrap.py 

In some cases people who've customized their command prompt to include emoji or other non-text symbols have found that bootstrap.py fails with a UnicodeDecodeError. We have a bug filed for that but in the meantime if you run into this problem you'll need to change your prompt back to something boring.
More Info

The above bootstrap script supports popular Linux distributions. If it doesn't work for you, see Linux build prerequisites for more. Again, if you run into other problems or our documentation isn't clear, please contact Mike Hoye directly.

                  
Gecko:Overview
                                      This document attempts to give an overview of the different parts of Gecko, what they do, and why they do it, say where the code for them is within the repository, and link to more specific documentation (when available) covering the details of that code.   It is not yet complete.  Maintainers of these areas of code should correct errors, add information, and add links to more detailed documentation (since this document is intended to remain an overview, not complete documentation). 

Contents
• 1 Browsers, Frames, and Document Navigation• 1.1 Docshell
• 1.2 Session History
• 1.3 Embedding
• 1.4 Multi-process and IPC

• 2 Networking• 2.1 Protocol handlers
• 2.2 URI objects
• 2.3 Channels

• 3 Document rendering pipeline• 3.1 Parser
• 3.2 DOM / Content
• 3.3 Style System• 3.3.1 Quantum CSS (Stylo)
• 3.3.2 Gecko

• 3.4 Layout• 3.4.1 Frame Construction
• 3.4.2 Reflow
• 3.4.3 Painting
• 3.4.4 Pagination

• 3.5 Dynamic change handling along the rendering pipeline
• 3.6 Refresh driver

• 4 Graphics• 4.1 Painting/Rasterizing
• 4.2 Compositing
• 4.3 Async Panning and Zooming

• 5 Scripting• 5.1 JavaScript Engine
• 5.2 XPConnect
• 5.3 WebIDL Bindings
• 5.4 Security

• 6 Images
• 7 Plugins
• 8 Platform-specific layers
• 9 Editor
• 10 Base layers• 10.1 NSPR
• 10.2 XPCOM
• 10.3 String


Browsers, Frames, and Document Navigation

Docshell
The user of a Web browser can change the page shown in that browser in many ways:  by clicking a link, loading a new URL, using the forward and back buttons, or other ways.  This can happen inside a space we'll call a browsing context; this space can be a browser window, a tab, or a frame or iframe within a document.  The toplevel data structures within Gecko represent this browsing context; they contain other data structures representing the individual pages displayed inside of it (most importantly, the current one).  In terms of implementation, these two types of navigation, the top level of a browser and the frames within it, largely use the same data structures. 
In Gecko, the docshell is the toplevel object responsible for managing a single browsing context.  It, and the associated session history code,  manage the navigation between pages inside of a docshell.  (Note the difference between session history, which is a sequence of pages in a single browser session, used for recording information for back and forward navigation, and global history, which is the history of pages visited and associated times, regardless of browser session, used for things like link coloring and address autocompletion.) 
There are relatively few objects in Gecko that are associated  with a docshell rather than being associated with a particular one of the pages inside of it. Most such objects are attached to the docshell. An  important object associated with the docshell is the nsGlobalWindowOuter  which is what the HTML5 spec refers to as a WindowProxy (into which  Window objects, as implemented by nsGlobalWindowInner, are loaded).  See  the DOM section below for more information on this. 
The most toplevel object for managing the contents of a particular page being displayed within a docshell is a document viewer (see layout). Other important objects associated with this presentation are the document (see DOM) and the pres(entation) shell and pres(entation) context (see layout). 
  <xul:browser>, frameloader and docshell in single process configurationDocshells are organized into a tree. If a docshell has a non-null  parent, then it corresponds to a subframe in whatever page is currently  loaded in the parent docshell, and the corresponding subframe element  (for example, an iframe) is called a browsing context container. In Gecko, a browsing context container would implement nsIFrameLoaderOwner to hold a frameloader, which holds and manages the docshell.  
One of the most interfaces docshell implemented is nsIWebNavigation. It defines major functions of a browsing context, such as loadURI / goBack / goForward and reload. In single process configuration of desktop Firefox, <xul:browser> (which represents a tab) operates on docshell through nsIWebNavigation, as shown in the right figure. 
•  code: mozilla/docshell/
•  bugzilla:  Core::Document Navigation
•  documentation: DocShell:Home Page

Session History
In order to keep the session history of subframes after the root  document is unloaded and docshells of subframes are destroyed, only the  root docshell of a docshell tree manages the session history (this does  not match the conceptual model in the HTML5 spec and may be subject to  change). 
As an example to explain the structure of session history  implementation in Gecko, consider there's a tab A, which loads document  1; in document 1, there are iframe B & C, which loads document 2  & 3, respectively. Later, an user navigates iframe B to document 4.  The following figures show the conceptual model of the example, and the  corresponding session history structure. 






~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
|| click me || click me ||
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
 Conceptual model representation; color denotes current visible active documents ||     ||
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~


~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
Session history structure; color denotes current transaction / entries
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

In Gecko, a session history object holds a list of transactions (denoted as T1 & T2 in the figure); each transaction points to a tree of entries;  each entry records the docshell it associated to, and a URL. Now that  if we navigate the tab to a different root document 5, which includes an  iframe D with document 6, then it becomes: 






~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
|| click me || click me ||
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
 Conceptual model representation ||     ||
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~


~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
Session history structure
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~


Embedding
To be written (and maybe rewritten if we get an IPC embedding API). 
  <xul:remote-browser>, frameloader and docshell in multiprocess configuration. The horizontal dash-lines represent inter-process communication
Multi-process and IPC
In a multi-process desktop Firefox, a tab is managed by <xul:remote-browser>.  It still operates on nsIWebNavigation, but in this case the docshell is  in a remote process and can not be accessed directly. The encapsulation  of remote nsIWebNavigation is done by the javascript-implemented RemoteWebNavigation. 
In this configuration, the frameloader of a root docshell lives  in parent process, so it can not access docshell directly either.  Instead, it holds a TabParent instance to interact with TabChild in child-process. At C/C++ level, the communication across processes for a single tab is through the PBrowser IPC protocol implemented by TabParent and TabChild, while at the javascript level it's done by message manager (which is ontop of PBrowser). RemoteWebNavigation, for example, sends messages to browser-child.js in content process through message manager. 

Networking
The network library Gecko uses is called Necko.  Necko APIs are largely organized around three concepts: URI objects, protocol handlers, and channels. 

Protocol handlers
A protocol handler is an XPCOM service associated with a  particular URI scheme or network protocol.  Necko includes protocol  handlers for HTTP, FTP, the data: URI scheme, and various others.   Extensions can implement protocol handlers of their own. 
A protocol handler implements the nsIProtocolHandler API, which serves three primary purposes: 
1.  Providing metadata about the protocol (its security characteristics, whether it requires actual network access, what the corresponding URI scheme is, what TCP port the protocol uses by default).
2.  Creating URI objects for the protocol's scheme.
3.  Creating channel objects for the protocol's URI objects
Typically, the built-in I/O service (nsIIOService)  is responsible for finding the right protocol handler for URI object  creation and channel creation, while a variety of consumers queries  protocol metadata.  Querying protocol metadata is the recommended way to  handle any sort of code that needs to have different behavior for  different URI schemes.  In particular, unlike whitelists or blacklists,  it correctly handles the addition of new protocols. 
A service can register itself as a protocol handler by registering for the contract ID "@mozilla.org/network/protocol;1?name=SSSSS" where SSSS is the URI scheme for the protocol (e.g. "http", "ftp", and so forth). 

URI objects
URI objects, which implement the nsIURI  API, are a way of representing URIs and IRIs.  Their main advantage  over strings is that they do basic syntax checking and canonicalization  on the URI string that they're provided with.  They also provide various  accessors to extract particular parts of the URI and provide URI  equality comparisons.  URIs that correspond to hierarchical schemes  implement the additional nsIURL interface, which exposes even more accessors for breaking out parts of the URI. 
URI objects are typically created by calling the newURI method on the I/O service, or in C++ by calling the NS_NewURI  utility function.  This makes sure to create the URI object using the  right protocol handler, which ensures that the right kind of object is  created.  Direct creation of URIs via createInstance is reserved for protocol handler implementations. 

Channels
Channels are the Necko representation of a single request/response interaction with a server.  A channel is created by calling the newChannel method on the I/O service, or in C++ by calling the NS_NewChannel utility function.  The channel can then be configured as needed, and finally its asyncOpen method can be called.  This method takes an nsIStreamListener as an argument. 
If asyncOpen has returned successfully, the channel guarantees that it will asynchronously call the onStartRequest and onStopRequest  methods on its stream listener.  This will happen even if there are  network errors that prevent Necko from actually performing the requests.   Such errors will be reported in the channel's status and in the status argument to onStopRequest. 
If the channel ends up being able to provide data, it will make one or more onDataAvailable on its listener after calling onStartRequest and before calling onStopRequest.   For each call, the listener is responsible for either returning an  error or reading the entire data stream passed in to the call. 
If an error is returned from either onStartRequest or onDataAvailable, the channel must act as if it has been canceled with the corresponding error code. 
A channel has two URI objects associated with it.  The originalURI of the channel is the URI that was originally passed to newChannel to create the channel that then had asyncOpen called on it.  The URI  is the URI from which the channel is reading data.  These can be  different in various cases involving protocol handlers that forward  network access to other protocol handlers, as well as in situations in  which a redirect occurs (e.g. following an HTTP 3xx response).  In  redirect situations, a new channel object will be created, but the  originalURI will be propagated from the old channel to the new channel. 
Note that the nsIRequest that's passed to  onStartRequest must match the one passed to onDataAvailable and  onStopRequest, but need not be the original channel that asyncOpen was  called on.  In particular, when an HTTP redirect happens the request  argument to the callbacks will be the post-redirect channel. 
TODO: crypto? 

Document rendering pipeline
Some of the major components of Gecko can be described as steps on the path from an HTML document coming in from the network to the graphics commands needed to render that document.  An HTML document is a serialization of a tree structure.  (FIXME: add diagram)  The HTML parser and content sink create an in-memory representation of this tree, which we call the DOM tree or content tree. Many JavaScript APIs operate on the content tree.  Then, in layout, we create a second tree, the frame tree (or rendering tree) that is a similar shape to the content tree, but where each node in the tree represents a rectangle (except in SVG where they represent other shapes).  We then compute the positions of the nodes in the frame tree (called frames) and paint them using our cross-platform graphics APIs (which, underneath, map to platform-specific graphics APIs). 
Further documentation: 
•  Talk: Fast CSS: How Browsers Lay Out Web Pages (David Baron, 2012-03-11): slideshow, all slides, audio (MP3), session page
•  Talk: Efficient CSS Animations (an updated version of the previous talk with a slightly different focus) (David Baron, 2014-06-04): slideshow, all slides, video
•  Talk: Overview of the Gecko Rendering Pipeline (Benoit Girard, 2014-10-14)

Parser
The parser's job is to transform a character stream into a tree structure, with the help of the content sink classes. 
HTML is parsed using a parser implementing the parsing algorithm  in the HTML specification (starting with HTML5).  Much of this parser is translated from Java, and changes are made to the Java version.  This parser in parser/html/.  The parser is driven by the output of the  networking layer (see nsHtml5StreamParser::OnDataAvailable).  The HTML5  parser is capable of parsing off the main thread which is the normal  case.  It also parses on the main thread to be able to synchronously  handle things such as innerHTML modifications. 
The codebase still has the previous generation HTML parser, which is still used for a small number of things, though we hope to be able to remove it entirely soon.  This parser is in parser/htmlparser/. 
XML is parsed using the expat library (parser/expat/) and code that wraps it (parser/xml/).  This is a non-validating parser; however, it loads certain DTDs to support XUL localization. 

DOM / Content
The content tree or DOM tree is the central data structure for Web pages.  It is a tree structure, initially created from the tree structure expressed in the HTML or XML markup.  The nodes in the tree implement major parts of the DOM (Document Object Model) specifications. The nodes themselves are part of a class hierarchy rooted at nsINode; different derived classes are used for things such as text nodes, the document itself, HTML elements, SVG elements, etc., with further subclasses of many of these types (e.g., for specific HTML elements).  Many of the APIs available to script running in Web pages are associated with these nodes.  The tree structure persists while the Web pages is displayed, since it stores much of state associated with the Web page.  The code for these nodes lives in the content/ directory. 
The DOM APIs are not threadsafe.  DOM nodes can be accessed only from the main thread (also known as the UI thread (user interface thread)) of the application. 
There are also many other APIs available to Web pages that are not APIs on the nodes in the DOM tree.  Many of these other APIs also live in the same directories, though some live in content/ and some in dom/.  These include APIs such as the DOM event model. 
The dom/ directory also includes some of the code needed to expose Web APIs to JavaScript (in other words, the glue code between JavaScript and these APIs). For an overview, see DOM API Implementation. See Scripting below for details of the JS engine. 
TODO: Internal APIs vs. DOM APIs. 
TODO: Mutation observers / document observers. 
TODO: Reference counting and cycle collection. 
TODO: specification links 

Style System

Quantum CSS (Stylo)
Starting with Firefox 57 and later, Gecko makes use of the parallel  style system written in Rust that comes from Servo.  There's an overview of this with graphics to help explain what's going on.  The Servo wiki has some more details. 

Gecko
The rest of the style section section describes the Gecko style  system used in Firefox 56 and earlier.  Some bits may still apply, but  it likely needs revising. 
In order to display the content, Gecko needs to compute the styles relevant to each DOM node.  It does this based on the model described in the CSS specifications:  this model applies to style specified in CSS (e.g. by a 'style' element, an 'xml-stylesheet' processing instruction or a 'style' attribute), style specified by presentation attributes, and the default style specified by our own user agent style sheets.  There are two major sets of data structures within the style system: 
•  first, data structures that represent sources of style data, such as CSS style sheets or data from stylistic HTML attributes
•  second, data structures that represent computed style for a given DOM node.
These sets of data structures are mostly distinct (for example, they store values in different ways). 
The loading of CSS style sheets from the network is managed by the  CSS loader;  they are then tokenized by the  CSS scanner  and parsed by the  CSS parser.   Those that are attached to the document also expose APIs to script that are known as the CSS Object Model, or CSSOM. 
The style sheets that apply to a document are managed by a class called the style set. The style set interacts with the different types of style sheets (representing CSS style sheets, presentational attributes, and 'style' attributes) through two interfaces: nsIStyleSheet for basic management of style sheets and nsIStyleRuleProcessor for getting the style data out of them.  Usually the same object implements both interfaces, except in the most important case, CSS style sheets, where there is a single rule processor for all of the CSS style sheets in each origin (user/UA/author) of the CSS cascade. 
The computed style data for an element/frame are exposed to the rest of Gecko through the class mozilla::ComputedStyle (previously called nsStyleContext).  Rather than having a member variable for each CSS property, it breaks up the properties into groups of related properties called style structs.  These style structs obey the rule that all of the properties in a single struct either inherit by default (what the CSS specifications call "Inherited: yes" in the definition of properties; we call these inherited structs) or all are not inherited by default (we call these reset structs).  Separating the properties in this way improves the ability to share the structs between similar ComputedStyle objects and reduce the amount of memory needed to store the style data. The ComputedStyle API exposes a method for getting each struct, so you'll see code like sc->GetStyleText()->mTextAlign for getting the value of the text-align CSS property.  (Frames (see the Layout section below) also have the same GetStyle* methods, which just forward the call to the frame's ComputedStyle.) 
The ComputedStyles form a tree structure, in a shape somewhat like the content tree (except that we coalesce identical sibling ComputedStyles rather than keeping two of them around; if the parents have been coalesced then this can apply recursively and coalasce cousins, etc.; we do not coalesce parent/child ComputedStyles). The parent of a ComputedStyle has the style data that the ComputedStyle inherits from when CSS inheritance occurs.  This means that the parent of the ComputedStyle for a DOM element is generally the ComputedStyle for that DOM element's parent, since that's how CSS says inheritance works. 
The process of turning the style sheets into computed style data goes through three main steps, the first two of which closely relate to the nsIStyleRule interface, which represents an immutable source of style data, conceptually representing (and for CSS style rules, directly storing) a set of property:value pairs.  (It is similar to the idea of a CSS style rule, except that it is immutable; this immutability allows for significant optimization.  When a CSS style rule is changed through script, we create a new style rule.) 
The first step of going from style sheets to computed style data is finding the ordered sequence of style rules that apply to an element. The order represents which rules override which other rules:  if two rules have a value for the same property, the higher ranking one wins. (Note that there's another difference from CSS style rules: declarations with !important are represented using a separate style rule.)  This is done by calling one of the nsIStyleRuleProcessor::RulesMatching methods. The ordered sequence is stored in a trie called the rule tree:  the path from the root of the rule tree to any (leaf or non-leaf) node in the rule tree represents a sequence of rules, with the highest ranking farthest from the root.  Each rule node (except for the root) has a pointer to a rule, but since a rule may appear in many sequences, there are sometimes many rule nodes pointing to the same rule.  Once we have this list we create a ComputedStyle (or find an appropriate existing sibling) with the correct parent pointer (for inheritance) and rule node pointer (for the list of rules), and a few other pieces of information (like the pseudo-element). 
The second step of going from style sheets to computed style data is getting the winning property:value pairs from the rules.  (This only provides property:value pairs for some of the properties; the remaining properties will fall back to inheritance or to their initial values depending on whether the property is inherited by default.)  We do this step (and the third) for each style struct, the first time it is needed. This is done in nsRuleNode::WalkRuleTree, where we ask each style rule to fill in its property:value pairs by calling its MapRuleInfoInto function.  When called, the rule fills in only those pairs that haven't been filled in already, since we're calling from the highest priority rule to the lowest (since in many cases this allows us to stop before going through the whole list, or to do partial computation that just adds to data cached higher in the rule tree). 
The third step of going from style sheets to computed style data (which various caching optimizations allow us to skip in many cases) is actually doing the computation; this generally means we transform the style data into the data type described in the "Computed Value" line in the property's definition in the CSS specifications.  This transformation happens in functions called nsRuleNode::Compute*Data, where the * in the middle represents the name of the style struct.  This is where the transformation from the style sheet value storage format to the computed value storage format happens. 
Once we have the computed style data, we then store it:  if a style struct in the computed style data doesn't depend on inherited values or on data from other style structs, then we can cache it in the rule tree (and then reuse it, without recomputing it, for any ComputedStyles pointing to that rule node).  Otherwise, we store it on the ComputedStyle (in which case it may be shared with the ComputedStyle's descendant ComputedStyles). This is where keeping inherited and non-inherited properties separate is useful:  in the common case of relatively few properties being specified, we can generally cache the non-inherited structs in the rule tree, and we can generally share the inherited structs up and down the ComputedStyle tree. 
The ownership models in style sheet structures are a mix of reference counted structures (for things accessible from script) and directly owned structures.  ComputedStyles are reference counted, and own their parents (from which they inherit), and rule nodes are garbage collected with a simple mark and sweep collector (which often never needs to run). 
•  code: layout/style/, where most files have useful one line descriptions at the top that show up in DXR
•  Bugzilla: Style System (CSS)
•  specifications•  CSS 2.1
•  CSS 2010, listing stable css3 modules
•  CSS WG editors drafts (often more current, but sometimes more unstable than the drafts on the technical reports page)
•  Preventing attacks on a user's history through CSS :visited selectors

•  documentation•  style system documentation (somewhat out of date)


Layout
Much of the layout code deals with operations on the frame tree (or rendering tree).  In the frame tree, each node represents a rectangle (or, for SVG, other shapes).  The frame tree has a shape similar to the content tree, since many content nodes have one corresponding frame, though it differs in a few ways, since some content nodes have more than one frame or don't have any frames at all.  When elements are display:none in CSS or undisplayed for certain other reasons, they won't have any frames.  When elements are broken across lines or pages, they have multiple frames; elements may also have multiple frames when multiple frames nested inside each other are needed to display a single element (for example, a table, a table cell, or many types of form controls). 
Each node in the frame tree is an instance of a class derived from nsIFrame.  As with the content tree, there is a substantial type hierarchy, but the type hierarchy is very different:  it includes types like text frames, blocks and inlines, the various parts of tables, and the various types of HTML form controls. 
Frames are allocated within an arena owned by the pres shell.  Each frame is owned by its parent; frames are not reference counted, and code must not hold on to pointers to frames.  To mitigate potential security bugs when pointers to destroyed frames, we use frame poisoning, which takes two parts.  When a frame is destroyed other than at the end of life of the presentation, we fill its memory with a pattern consisting of a repeated pointer to inaccessible memory, and then put the memory on a per-frame-class freelist.  This means that if code accesses the memory through a dangling pointer, it will either crash quickly by dereferencing the poison pattern or it will find a valid frame. 
Like the content tree, frames must be accessed only from the UI thread. 
The frame tree should not store any important data.  While it does usually persist while a page is being displayed, frames are often destroyed and recreated in response to certain style changes, and in the future we may do the same to reduce memory use for pages that are currently inactive.  There were a number of cases where this rule was violated in the past and we stored important data in the frame tree; however, most (though not quite all) such cases are now fixed. 
The rectangle represented by the frame is what CSS calls the element's border box.  This is the outside edge of the border (or the inside edge of the margin).  The margin lives outside the border; and the padding lives inside the border.  In addition to nsIFrame::GetRect, we also have the APIs nsIFrame::GetPaddingRect to get the padding box (the outside edge of the padding, or inside edge of the border) and nsIFrame::GetContentRect to get the content box (the outside edge of the content, or inside edge of the padding).  These APIs may produce out of date results when reflow is needed (or has not yet occurred). 
In addition to tracking a rectangle, frames also track two overflow areas: visual overflow and scrollable overflow.  These overflow areas represent the union of the area needed by the frame and by all its descendants.  The visual overflow is used for painting-related optimizations:  it is a rectangle covering all of the area that might be painted when the frame and all of its descendants paint.  The scrollable overflow represents the area that the user should be able to scroll to to see the frame and all of its descendants.  In some cases differences between the frame's rect and its overflow happen because of descendants that stick out of the frame; in other cases they occur because of some characteristic of the frame itself.  The two overflow areas are similar, but there are differences:  for example, margins are part of scrollable overflow but not visual overflow, whereas text-shadows are part of visual overflow but not scrollable overflow. 
When frames are broken across lines, columns, or pages, we create multiple frames representing the multiple rectangles of the element. The first one is the primary frame, and the rest are its continuations (which are more likely to be destroyed and recreated during reflow). These frames are linked together as continuations: they have a doubly-linked list that can be used to traverse the continuations using nsIFrame::GetPrevContinuation and nsIFrame::GetNextContinuation. (Currently continuations always have the same style data, though we may at some point want to break that invariant.) 
Continuations are sometimes siblings of each other, and sometimes not. For example, if a paragraph contains a span which contains a link, and the link is split across lines, then the continuations of the span are siblings (since they are both children of the paragraph), but the continuations of the link are not siblings (since each continuation of the link is descended from a different continuation of the span). Traversing the entire frame tree does not require considering continuations, since all of the continuations are descendants of the element containing the break. 
We also use continuations for cases (most importantly, bidi reordering, where left-to-right text and right-to-left text need to be separated into different continuations since they may not form a contiguous rectangle) where the continuations should not be rewrapped during reflow:  we call these continuations fixed rather than fluid. nsIFrame::GetNextInFlow and nsIFrame::GetPrevInFlow traverse only the fluid continuations and do not cross fixed continuation boundaries. 
If an inline frame has non-inline children, then we split the original  inline frame into parts. The original inline's children are  distributed into these parts like so- The children of the original  inline are grouped into runs of inline and non-inline and runs of  inline get an inline parent while runs of non-inline get an anonymous  block parent. This is 'ib-splitting' or block-inside-inline-splitting.  This splitting proceeds recursively up the frame tree until all  non-inlines inside inlines are ancestors of a block frame with anonymous  block wrappers in between. This splitting maintains the relative order between these child frames and the relationship between the parts of a  split inline is maintained using an ib-sibling chain. It is important  to note that any wrappers created during frame construction (such as  for tables) may not be included in the ib-sibling chain depending on  when this wrapper creation takes place.  
TODO: nsBox craziness from https://bugzilla.mozilla.org/show_bug.cgi?id=524925#c64 
TODO: link to documentation of block and inline layout 
TODO: link to documentation of scrollframes 
TODO: link to documentation of XUL frame classes 
Code (note that most files in base and generic have useful one line descriptions at the top that show up in DXR): 
•  layout/base/ contains objects that coordinate everything and a bunch of other miscellaneous things
•  layout/generic/ contains the basic frame classes as well as support code for their reflow methods (ReflowInput, ReflowOutput)
•  layout/forms/ contains frame classes for HTML form controls
•  layout/tables/ contains frame classes for CSS/HTML tables
•  layout/mathml/ contains frame classes for MathML
•  layout/svg/ contains frame classes for SVG
•  layout/xul/ contains frame classes for the XUL box model and for various XUL widgets
Bugzilla: 
•  All of the components whose names begin with "Layout" in the "Core" product
Further documentation: 
•  Talk: Introduction to graphics/layout architecture (Robert O'Callahan, 2014-04-18)
•  Talk: Layout and Styles (Boris Zbarsky, 2014-10-14)

 

Frame Construction
Frame construction is the process of creating frames.  This is done  when styles change in ways that require frames to be created or  recreated or when nodes are inserted into the document.  The content  tree and the frame tree don't have quite the same shape, and the frame  construction process does some of the work of creating the right shape  for the frame tree.  It handles the aspects of creating the right shape  that don't depend on layout information.  So for example, frame  construction handles the work needed to implement table anonymous objects but does not handle frames that need to be created when an element is broken across lines or pages. 
The basic unit of frame construction is a run of contiguous  children of a single parent element.  When asked to construct frames for  such a run of children, the frame constructor first determines, based  on the siblings and parent of the nodes involved, where in the frame  tree the new frames should be inserted.  Then the frame constructor  walks through the list of content nodes involved and for each one  creates a temporary data structure called a frame construction item.   The frame construction item encapsulates various information needed to  create the frames for the content node: its style data, some metadata  about how one would create a frame for this node based on its namespace,  tag name, and styles, and some data about what sort of frame will be  created.  This list of frame construction items is then analyzed to see  whether constructing frames based on it and inserting them at the chosen  insertion point will produce a valid frame tree.  If it will not, the  frame constructor either fixes up the list of frame construction items  so that the resulting frame tree would be valid or throws away the list  of frame construction items and requests the destruction and re-creation  of the frame for the parent element so that it has a chance to create a  list of frame construction items that it can fix up. 
Once the frame constructor has a list of frame construction items  and an insertion point that would lead to a valid frame tree, it goes  ahead and creates frames based on those items.  Creation of a non-leaf  frame recursively attempts to create frames for the children of that  frame's element, so in effect frames are created in a depth-first  traversal of the content tree. 
The vast majority of the code in the frame constructor, therefore, falls into one of these categories: 
•  Code to determine the correct insertion point in the frame tree for new frames.
•  Code to create, for a given content node, frame construction items.  This involves some searches through static data tables for metadata about the frame to be created.
•  Code to analyze the list of frame construction items.
•  Code to fix up the list of frame construction items.
•  Code to create frames from frame construction items.
Code: layout/base/nsCSSFrameConstructor.h and layout/base/nsCSSFrameConstructor.cpp 

Reflow
Reflow is the process of computing the positions and sizes of frames.  (After all, frames represent rectangles, and at some point we need to figure out exactly *what* rectangle.)  Reflow is done recursively, with each frame's Reflow method calling the Reflow methods on that frame's descendants. 
In many cases, the correct results are defined by CSS specifications (particularly CSS 2.1).  In some cases, the details are not defined by CSS, though in some (but not all) of those cases we are constrained by Web compatibility.  When the details are defined by CSS, however, the code to compute the layout is generally structured somewhat differently from the way it is described in the CSS specifications, since the CSS specifications are generally written in terms of constraints, whereas our layout code consists of algorithms optimized for incremental recomputation. 
The reflow generally starts from the root of the frame tree, though some other types of frame can act as reflow roots and start a reflow from them. Reflow roots must obey the invariant that a change inside one of their descendants never changes their rect or overflow areas (though currently scrollbars are reflow roots but don't quite obey this invariant). 
In many cases, we want to reflow a part of the frame tree, and we want this reflow to be efficient.  For example, when content is added or removed from the document tree or when styles change, we want the amount of work we need to redo to be proportional to the amount of content.  We also want to efficiently handle a series of changes to the same content. 
To do this, we maintain two bits on frames: NS_FRAME_IS_DIRTY indicates that a frame and all of its descendants require reflow. NS_FRAME_HAS_DIRTY_CHILDREN indicates that a frame has a descendant that is dirty or has had a descendant removed (i.e., that it has a child that has NS_FRAME_IS_DIRTY or NS_FRAME_HAS_DIRTY_CHILDREN or it had a child removed).  These bits allow coalescing of multiple updates; this coalescing is done in nsPresShell, which tracks the set of reflow roots that require reflow.  The bits are set during calls to nsPresShell::FrameNeedsReflow and are cleared during reflow. 
The layout algorithms used by many of the frame classes are those specified in CSS, which are based on the traditional document formatting model, where widths are input and heights are output. 
In some cases, however, widths need to be determined based on the content.  This depends on two intrinsic widths: the minimum intrinsic width (see nsIFrame::GetMinWidth) and the preferred intrinsic width (see nsIFrame::GetPrefWidth).  The concept of what these widths represent is best explained by describing what they are on a paragraph containing only text:  in such a paragraph the minimum intrinsic width is the width of the longest word, and the preferred intrinsic width is the width of the entire paragraph laid out on one line. 
Intrinsic widths are invalidated separately from the dirty bits described above.  When a caller informs the pres shell that a frame needs reflow (nsIPresShell::FrameNeedsReflow), it passes one of three options: 
•  eResize indicates that no intrinsic widths are dirty
•  eTreeChange indicates that intrinsic widths on it and its ancestors are dirty (which happens, for example, if new children are added to it)
•  eStyleChange indicates that intrinsic widths on it, its ancestors, and its descendants are dirty (for example, if the font-size changes)
Reflow is the area where the XUL frame classes (those that inherit from nsBoxFrame or nsLeafBoxFrame) are most different from the rest.  Instead of using nsIFrame::Reflow, they do their layout computations using intrinsic size methods called GetMinSize, GetPrefSize, and GetMaxSize (which report intrinsic sizes in two dimensions) and a final layout method called Layout.  In many cases these methods defer some of the computation to a separate object called a layout manager. 
When an individual frame's Reflow method is called, most of the input is provided on an object called ReflowInput and the output is filled in to an object called ReflowOutput.  After reflow, the caller (usually the parent) is responsible for setting the frame's size based on the metrics reported.  (This can make some computations during reflow difficult, since the new size is found in either the reflow state or the metrics, but the frame's size is still the old size.  However, it's useful for invalidating the correct areas that need to be repainted.) 
One major difference worth noting is that in XUL layout, the size of the child is set prior to its parent calling its Layout method.  (Once invalidation uses display lists and is no longer tangled up in Reflow, it may be worth switching non-XUL layout to work this way as well.) 

Painting
TODO: display lists (and event handling) 
TODO: layers 

Pagination
The concepts behind pagination (also known as fragmentation) are a  bit complicated, so for now we've split them off into a separate  document: Gecko:Continuation_Model.  This code is used for printing, print-preview, and multicolumn frames. 

Dynamic change handling along the rendering pipeline
The ability to make changes to the DOM from script is a major feature  of the Web platform.  Web authors rely on the concept (though there are  a few exceptions, such as animations) that changing the DOM from script  leads to the same rendering that would have resulted from starting from  that DOM tree.  They also rely on the performance characteristics of  these changes:  small changes to the DOM that have small effects should  have proportionally small processing time.  This means that Gecko needs  to efficiently propagate changes from the content tree to style, the  frame tree, the geometry of the frame tree, and the screen. 
For many types of changes, however, there is substantial overhead  to processing a change, no matter how small.  For example, reflow must  propagate from the top of the frame tree down to the frames that are  dirty, no matter how small the change.  One very common way around this  is to batch up changes.  We batch up changes in lots of ways, for  example: 
•  The content sink adds multiple nodes to the DOM tree before notifying listeners that they've been added.  This allows notifying once about an ancestor rather than for each of its descendants, or notifying about a group of descendants all at once, which speeds up the processing of those notifications.
•  We batch up nodes that require style reresolution (recomputation of selector matching and processing the resulting style changes).  This batching is tree based, so it not only merges multiple notifications on the same element, but also merges a notification on an ancestor with a notification on its descendant (since some of these notifications imply that style reresolution is required on all descendants).
•  We wait to reconstruct frames that require reconstruction (after destroying frames eagerly).  This, like the tree-based style reresolution batching, avoids duplication both for same-element notifications and ancestor-descendant notifications, even though it doesn't actually do any tree-based caching.
•  We postpone doing reflows until needed.  As for style reresolution, this maintains tree-based dirty bits (see the description of NS_FRAME_IS_DIRTY and NS_FRAME_HAS_DIRTY_CHILDREN under Reflow).
•  We allow the OS to queue up multiple invalidates before repainting (though we will likely switch to controlling that ourselves).  This leads to a single repaint of some set of pixels where there might otherwise have been multiple (though it may also lead to more pixels being repainted if multiple rectangles are merged to a single one).
Having changes buffered up means, however, that various pieces of  information (layout, style, etc.) may not be up-to-date.  Some things  require up-to-date information:  for example, we don't want to expose  the details of our buffering to Web page script since the programming  model of Web page script assumes that DOM changes take effect  "immediately", i.e., that the script shouldn't be able to detect any  buffering.  Many Web pages depend on this. 
We therefore have ways to flush these different sorts of buffers.   There are methods called FlushPendingNotifications on nsIDocument and  nsIPresShell, that take an argument of what things to flush: 
•  Flush_Content: create all the content nodes from data buffered in the parser
•  Flush_ContentAndNotify: the above, plus notify document observers about the creation of all nodes created so far
•  Flush_Style: the above, plus make sure style data are up-to-date
•  Flush_Frames: the above, plus make sure all frame construction has happened (currently the same as Flush_Style)
•  Flush_InterruptibleLayout: the above, plus perform layout (Reflow), but allow interrupting layout if it takes too long
•  Flush_Layout: the above, plus ensure layout (Reflow) runs to completion
•  Flush_Display (should never be used): the above, plus ensure repainting happens
The major way that notifications of changes propagate from the  content code to layout and other areas of code is through the  nsIDocumentObserver and nsIMutationObserver interfaces.  Classes can  implement this interface to listen to notifications of changes for an  entire document or for a subtree of the content tree. 
WRITE ME: ... layout document observer implementations 
TODO: how style system optimizes away rerunning selector matching 
TODO: style changes and nsChangeHint 

Refresh driver

Graphics
Wiki page | contribute 
Further documentation: 
•  Talk: Introduction to graphics/layout architecture (Robert O'Callahan, 2014-04-18)
•  Talk: An overview of Gecko's graphics stack (Benoit Jacob, 2014-08-12)
•  Talk: 2D Graphics (Jeff Muizelaar, 2014-10-14)

Painting/Rasterizing
Painting/rendering/rasterizing is the step where graphical primitives (such as a command to fill an SVG circle, or a canvas  command to stroke a path between x, y, z, or the internally produced  command to draw the edge of an HTML div's border) are used to color in  the pixels of a surface so that the surface "displays" those rasterized  primitives. 
The platform independent library that gecko uses to render is Moz2D.  Gecko code paints into Moz2D DrawTarget objects (the DrawTarget  baseclass having multiple platform dependent subclasses). (At least this  is where gecko is going - for now there is still significant amounts of  code that uses Thebes gfxContext  objects or the even older nsRenderingContext objects. Objects of these  types now always wrap a DrawTarget so all painting does now go through  Moz2D, but conversion to use the wrapped DrawTargets directly is taking  time since consumer code can sometimes need to be radically rewritten to  fit the Moz2D API and immediate mode paradigm.) 

Compositing
The compositing stage of the rendering pipeline is operated by the gfx/layers module. 
Different parts of a web page can sometimes be painted into  intermediate surfaces retained by layers. It can be convenient to think  of layers as the layers in image manipulation programs like The Gimp or  Photoshop. Layers are organized as a tree. Layers are primarily used to  minimize invalidating and repainting when elements are being animated  independently of one another in a way that can be optimized using layers  (e.g. opacity or transform animations), and to enable some effects  (such as transparency and 3D transforms). 
Compositing is the action of flattening the layers into the final image that is shown on the screen. 
On some platform, compositing happens on the Content thread.  However, on other platforms we composite on a separate thread  (Off-main-thread compositing, or OMTC). OMTC is at the moment the  default behavior on mobile platforms, but the goal is to do it on all  platforms in the long run. 
The Layers architecture is built on the following notions: 
•  Compositor: an object that can draw quads on the screen (or on an off-screen render target).
•  Texture: an object that contains image data.
•  Compositable: an object that can manipulate one or several textures, and knows how to present them to the compositor
•  Layer: an element of the layer tree. A layer usually doesn't know much about compositing: it uses a compositable to do the work. Layers are mostly nodes of the layer tree.
  New layers architecture, with OGL backendThe above abstractions are sufficient to perform compositing on the  main thread, but on the OMTC case, we need a mechanism to synchronize a  client layer tree that is constructed on the content thread, and a host  layer tree that is used for compositing on the compositor thread. This synchronization process must ensure that the host layer tree  reflects the state of the current layer tree while remaining in a  consistent state, and must transfer texture data from a thread to the  other. Moreover, on some platforms the content and compositor threads  may live in separate processes. 
To perform this synchronization, we use the IPDL IPC framework.  IPDL lets us describe communications protocols and actors in a domain  specific language. for more information about IPDL, read https://wiki.mozilla.org/IPDL 
When the client layer tree is modified, we record modifications  into messages that are sent together to the host side within a  transaction. A transaction is basically a list of modifications to the  layer tree that have to be applied all at once to preserve consistency  in the state of the host layer tree. 
Texture transfer is done by synchronizing texture objects across  processes/threads using a couple TextureClient/TextureHost that wrap  shared data and provide access to it on both sides. TextureHosts provide  access to one or several TextureSource, which has the necessary API to  actually composite the texture (TextureClient/Host being more about IPC  synchronization than actual compositing. The logic behind texture transfer (as in single/double/triple buffering,  texture tiling, etc) is operated by the CompositableClient and  CompositableHost. 
It is important to understand the separation between layers and  compositables. Compositables handle all the logic around texture  transfer, while layers define the shape of the layer tree. a  Compositable is created independently from a layer and attached to it on  both sides. While layers transactions can only originate from the  content thread, this separation makes it possible for us to have  separate compositable transactions between any thread and the compositor  thread. We use the ImageBridge IPDL protocol to that end. The Idea of  ImageBridge is to create a Compositable that is manipulated on the  ImageBridgeThread, and that can transfer/synchronize textures without  using the content thread at all. This is very useful for smooth Video  compositing: video frames are decoded and passed into the ImageBridge  without ever touching the content thread, which could be busy processing  reflows or heavy javascript workloads. 
It is worth reading the inline code documentation in the following files: 
•  gfx/layers/Compositor.h
•  gfx/layers/ShadowLayers.h
•  gfx/layers/Layers.h
•  gfx/layers/host/TextureHost.h
•  gfx/layers/client/CompositableClient.h
To visualize how a web page is layered, turn on the pref  "layers.draw-borders" in about:config. When this pref is on, the borders  of layers and tiles are displayed on top of the content. This only  works when using the new Compositor API, that is when off-main-thread  compositing is activated. OMTC is activated by default on all mobile  platforms, and will progressively get activated on desktop platforms  (not yet, though). On platforms where OMTC is not used, this pref has no  effect.  
Blog posts with information on Layers that should be integrated here: 
•  Layers: Cross-Platform Acceleration
•  Layers
•  Retained Layers
•  Shadow Layers, and learning by failing
•  Accelerated layer-rendering, and learning by (some) success
•  Mask Layers
•  Building a mask layer
•  Mask Layers on the Direct3D backends
•  Graphics API Design

Async Panning and Zooming
On some of our mobile platforms we have some code that allows the  user to do asynchronous (i.e. off-main-thread) panning and zooming of  content. This code is the Async Pan/Zoom module (APZ) code and is  further documented at Platform/GFX/APZ. 

Scripting

JavaScript Engine
Gecko embeds the SpiderMonkey JavaScript engine (located in js/src).   The JS engine includes a number of Just-In-Time compilers (or JITs).   SpiderMonkey provides a powerful and extensive embedding API (called the  JSAPI).  Because of its complexity, using the JSAPI directly is frowned  upon, and a number of abstractions have been built in Gecko to allow  interacting with the JS engine without using JSAPI directly.  Some JSAPI  concepts are important for Gecko developers to understand, and they are  listed below: 
•  Global object: The global object is the object on which all global variables/properties/methods live.  In a web page this is the 'window' object.  In other things such as XPCOM components or sandboxes XPConnect creates a global object with a small number of builtin APIs.
•  Compartments: A compartment is a subdivision of the JS heap.  The mapping from global objects to compartments is one-to-one; that is, every global object has a compartment associated with it, and no compartment is associated with more than one global object. (NB: There are compartments associated with no global object, but they aren't very interesting).  When JS code is running SpiderMonkey has a concept of a "current" compartment.  New objects that are created will be created in the "current" compartment and associated with the global object of that compartment.
•  When objects in one compartment can see objects in another compartment (via e.g. iframe.contentWindow.foo) cross-compartment wrappers or CCWs mediate the interaction between the two compartments.  By default CCWs ensure that the current compartment is kept up-to-date when execution crosses a compartment boundary.  SpiderMonkey also allows embeddings to extend wrappers with custom behavior to implement security policies, which Gecko uses extensively (see the Security section for more information).
Further documentation: 
•  Talk: Baseline JIT (Kannan Vijayan, 2014-10-14)

XPConnect
XPConnect is a bridge between C++ and JS used by XPCOM  components exposed to or implemented in JavaScript.  XPConnect allows  two XPCOM components to talk to each other without knowing or caring  which language they are implemented in.  XPConnect allows JS code to  call XPCOM components by exposing XPCOM interfaces as JS objects, and  mapping function calls and attributes from JS into virtual method calls  on the underlying C++ object.  It also allows JS code to implement an  XPCOM component that can be called from C++ by faking the vtable of an  interface and translating calls on the interface into JS calls.   XPConnect accomplishes this using the vtable data stored in XPCOM  TypeLibs (or XPT files) by the XPIDL compiler and a small layer of  platform specific code called xptcall that knows how to interact with and impersonate vtables of XPCOM interfaces. 
XPConnect is also used for a small (and decreasing) number of  legacy DOM objects.  At one time all of the DOM used XPConnect to talk  to JS, but we have been replacing XPConnect with the WebIDL bindings  (see the next section for more information).  Arbitrary XPCOM components  are not exposed to web content.  XPCOM components that want to be  accessible to web content must provide class info (that is, they must QI  to nsIClassInfo) and must be marked with the nsIClassInfo::DOM_OBJECT  flag.  Most of these objects also are listed in  dom/base/nsDOMClassInfo.cpp, where the interfaces that should be visible  to the web are enumerated.  This file also houses some "scriptable  helpers" (classes ending in "SH" and implementing nsIXPCScriptable),  which are a set of optional hooks that can be used by XPConnect objects  to implement more exotic behaviors (such as indexed getters or setters,  lazily resolved properties, etc).  This is all legacy code, and any new  DOM code should use the WebIDL bindings. 

WebIDL Bindings
The WebIDL bindings are a separate bridge between C++ and JS  that is specifically designed for use by DOM code.  We intend to replace  all uses of XPConnect to interface with content JavaScript with the  WebIDL bindings.  Extensive documentation  on the WebIDL bindings is available, but the basic idea is that a  binding generator takes WebIDL files from web standards and some  configuration data provided by us and generates at build time C++ glue  code that sits between the JS engine and the DOM object.  This setup  allows us to produce faster, more specialized code for better  performance at the cost of increased codesize. 
The WebIDL bindings also implement all of the behavior specified  in WebIDL, making it possible for new additions to the DOM to behave  correctly "out of the box".  The binding layer also provides C++  abstractions for types that would otherwise require direct JSAPI calls  to handle, such as typed arrays.   

Security
Gecko's JS security model is based on the concept of compartments.  Every compartment has a principal associated with it that contains security information such as the origin of the compartment, whether or not it has system privileges, etc.  For the following discussion, wrappee refers to the underlying object being wrapped, scope refers to the compartment the object is being wrapped for use in, and wrapper refers to the object created in scope that allows it to interact with wrappee.   "Chrome" refers to JS that is built in to the browser or part of an  extension, which runs with full privileges, and is contrasted with  "content", which is web provided script that is subject to security  restrictions and the same origin policy. 
•  Xray wrappers: Xray wrappers are used when the scope has chrome privileges and the wrappee is the JS reflection of an underlying DOM object.  Beyond the usual CCW duties of making sure that content code does not run with chrome privileges, Xray wrappers also ensure that calling methods or accessing attributes on the wrappee has the "expected" effect.  For example, a web page could replace the close method on its window with a JS function that does something completely different. (e.g. window.close = function() { alert("This isn't what you wanted!") };).  Overwriting methods in this way (or creating entirely new ones) is referred to as expando properties.  Xrays see through expando properties, invoking the original method/getter/setter if the expando overwrites a builtin or seeing undefined in the expando creates a new property.  This allows chrome code to call methods on content DOM objects without worrying about how the page has changed the object.
•  Waived xray wrappers: The xray behavior is not always desirable.  It is possible for chrome to "waive" the xray behavior and see the actual JS object.  The wrapper still guarantees that code runs with the correct privileges, but methods/getters/setters may not behave as expected.  This is equivalent to the behavior chrome sees when it looks at non-DOM content JS objects.
•  For more information, see the MDN page
•  bholley's Enter the Compartment talk also provides an overview of our compartment architecture.

Images

Plugins

Platform-specific layers
•  widget
•  native theme
•  files, networking, other low-level things
•  Accessibility APIs
•  Input. Touch-input stuff is somewhat described at Platform/Input/Touch.

Editor

Base layers

NSPR
NSPR is a library for providing cross-platform APIs for various platform-specific functions.  We tend to be trying to use it as little as possible, although there are a number of areas (particularly some network-related APIs and threading/locking primitives) where we use it quite a bit. 

XPCOM
XPCOM is a cross-platform modularity library, modeled on Microsoft COM.  It is an object system in which all objects inherit from the nsISupports interface. 
components and services, contract IDs and CIDs 
prior overuse of XPCOM; littering with XPCOM does not produce modularity 
Base headers (part of xpcom/base/) and data structures.  See also mfbt. 
Threading 
xptcall, proxies 
reference counting, cycle collection 
Further documentation: 
•  XPCOM and Internal Strings in Gecko (talk by David Baron, 2014-03-20): video, speaking notes

String
XPCOM has string classes for representing sequences of characters.   Typical C++ string classes have the goals of encapsulating the memory  management of buffers and the issues needed to avoid buffer overruns  common in traditional C string handling.  Mozilla's string classes have  these goals, and also the goal of reducing copying of strings. 
(There is a second set of string classes in xpcom/glue for  callers outside of libxul; these classes are partially source-compatible  but have different (worse) performance characteristics.  This  discussion does not cover those classes.) 
We have two parallel sets of classes, one for strings with 1-byte units (char, which may be signed or unsigned), and one for strings with 2-byte units (char16_t, always unsigned).  The classes are named such that the class for 2-byte characters ends with String and the corresponding class for 1-byte characters ends with CString.  2-byte strings are almost always used to encode UTF-16.  1-byte strings are usually used to encode either ASCII or UTF-8, but are sometimes also used to hold data in some other encoding or just byte sequences. 
The string classes distinguish, as part of the type hierarchy,  between strings that must have a null-terminator at the end of their  buffer (ns[C]String) and strings that are not required to have a null-terminator (ns[C]Substring).  ns[C]Substring is the base of the string classes (since it imposes fewer requirements) and ns[C]String is a class derived from it.  Functions taking strings as parameters should generally take one of these four types. 
In order to avoid unnecessary copying of string data (which can  have significant performance cost), the string classes support different  ownership models.  All string classes support the following three  ownership models dynamically: 
•  reference counted, copy-on-write, buffers (the default)
•  adopted buffers (a buffer that the string class owns, but is not reference counted, because it came from somewhere else)
•  dependent buffers, that is, an underlying buffer that the string class does not own, but that the caller that constructed the string guarantees will outlive the string instance
In addition, there is a special string class, nsAuto[C]String, that additionally contains an internal 64-unit buffer (intended primarily for use on the stack), leading to a fourth ownership model: 
•  storage within an auto string's stack buffer
Auto strings will prefer reference counting an existing  reference-counted buffer over their stack buffer, but will otherwise use  their stack buffer for anything that will fit in it. 
There are a number of additional string classes, particularly nsDependent[C]String, nsDependent[C]Substring, and the NS_LITERAL_[C]STRING macros which construct an nsLiteral[C]String  which exist primarily as constructors for the other types.  These types  are really just convenient notation for constructing an ns[C]S[ubs]tring with a non-default ownership mode; they should not be thought of as different types.  (The Substring, StringHead, and StringTail functions are also constructors for dependent [sub]strings.)  Non-default ownership modes can also be set up using the Rebind and Adopt methods, although the Rebind  methods actually live on the derived types, which is probably a mistake  (although moving them up would require some care to avoid making an API  that easily allows assigning a non-null-terminated buffer to a string  whose static type indicates that it is null-terminated). 
Note that the presence of all of these classes imposes some  awkwardness in terms of distinctions being available as both static type  distinctions and dynamic type distinctions.  In general, the only  distinctions that should be made statically are 1-byte vs. 2-byte  sequences (CString vs String) and whether the buffer is null-terminated or not (Substring vs String).  (Does the code actually do a good job of dynamically enforcing the Substring vs. String restriction?) 
TODO: buffer growth, concatenation optimizations 
TODO: encoding conversion, what's validated and what isn't 
TODO: "string API", nsAString (historical) 
•  Code: xpcom/string/
•  Bugzilla: Core::String
Further documentation: 
•  XPCOM and Internal Strings in Gecko (talk by David Baron, 2014-03-20): video, speaking notes
                           
Navigation menu
                               • Log in
• Request account
                                                         • Gecko
               • Discussion
                                                                              • Read
               • View source
               • View history
                                                                                                  • Main page
• Product releases
• New pages
• Recent changes
• Recent uploads
• Random page
• Help
                    
How to Contribute
                  • All-hands meeting
• Other meetings
• Contribute to Mozilla
• Mozilla Reps
• Student Ambassadors
                    
MozillaWiki
                  • News
• About
• Team
• Policies
• Releases
• @MozillaWiki
• Report a wiki bug
                    
Around Mozilla
                  • Mozilla Support
• Mozilla Developer Network
• Planet Mozilla
• Mozilla Blog
• Webmaker
• Research
                    
Tools
                  • What links here
• Related changes
• Special pages
• Printable version
• Permanent link
• Page information
• Import an Etherpad
                                        •  This page was last modified on 2 May 2018, at 21:34.
                           • Privacy policy
           • About MozillaWiki
           • Mobile view
                              •              
                    

                     Use "mach help run" to get more details.  If inside the source directory, you would use "./mach".  If you have previously added mach to your path, then just use "mach". Please note that mach is aware of mozconfigs.

$ ./mach run --debug [arguments to pass to firefox]

If you need to direct arguments to gdb, you can use '+gdbparams' options via the command line parser, taking care to adhere to shell splitting rules. For example, if you wanted to run the command 'show args' when gdb starts, you would use:

$ ./mach run --debug --debugparams "-ex 'show args'"

Alternatively, you can run gdb directly against Firefox. However, tou won't get some of the more useful capabilities this way. For example, mach sets an environment variable (see below) to stop the JS engine from generating synthetic segfaults to support the slower script dialoging mechanism.

$ gdb OBJDIR/dist/bin/firefox

See this old version for specialized instructions on older builds of Firefox.
How do I pass arguments in prun?

Set the arguments in GDB before calling prun. Here's an example on how to do that:

(gdb) set args https://www.mozilla.org
(gdb) prun

How do I set a breakpoint in a library that hasn't been loaded?

In older versions, there isn't a way to set breakpoints in a library that has not yet been loaded. See more on setting a breakpoint when a component is loaded. If you have to set a breakpoint you can set a breakpoint in _dl_open. This function is called when a new library is loaded, when you can finally set your breakpoint.
How do I set a breakpoint when a component is loaded? 

In Firefox Version 57 (and possibly earlier) XPCOM_BREAK_ON_LOAD does not seem to exist.

There's a facility in XPCOM which allows you to set an environment variable to drop into the debugger when loading a certain component. You have to set XPCOM_BREAK_ON_LOAD variable before you run Firefox, setting it to a string containing the names of libraries you want to load. For example, if you wish to stop when a library named raptor or necko is loaded, you set the variable to raptor:necko. Here's an example:

(gdb) set env XPCOM_BREAK_ON_LOAD raptor:necko
(gdb) prun

Why can't I set a breakpoint?

You probably can't set a breakpoint because its library hasn't been loaded. Most Firefox functionality is in libraries loaded mid-way through the main()function. If you break on main(),and step through until the libraries are loaded, with a call to InitCOMGlue(), you should be able to set breakpoints on many more symbols, source files, and continue running.

(gdb) break main
(gdb) run
Breakpoint 1, main(argc=4, argv=0x7fffffffde98, envp=0x7ffffffffdec0) .....
256    {
(gdb) next
...
293      nsresult rv = InitXPCOMGlue()
(gdb) next

If you still can't set the breakpoints, you need confirm the library has loaded. You can't proceed until the library loads. See more on loading shared libraries. If you wish to break as soon as the library is loaded, see the section on breaking when a component is loaded and breaking on a library load.

 
How do I display PRUnichar's?

One suggestion is this:

(gdb) print ((PRUnichar*)uri.mBuffer)[0]@16
$47 = {114, 100, 102, 58, 110, 117, 108, 108, 0, 0, 8, 0, 0, 0, 37432,
16514}

     

(gdb) print aURI
$1 = (const PRUnichar *) 0x855e6e0
(gdb) x/32ch aURI
0x855e6e0:      104 'h' 116 't' 116 't' 112 'p' 58 ':'  47 '/'  47 '/'  119 'w'
0x855e6f0:      119 'w' 119 'w' 46 '.'  109 'm' 111 'o' 122 'z' 105 'i' 108 'l'
0x855e700:      108 'l' 97 'a'  46 '.'  111 'o' 114 'r' 103 'g' 47 '/'  115 's'
0x855e710:      116 't' 97 'a'  114 'r' 116 't' 47 '/'  0 '\0'  25 '\031'       0 '\0'
(gdb)

        Define helper functions in your .gdbinit

# Define a "pu" command to display PRUnichar * strings (100 chars max)
# Also allows an optional argument for how many chars to print as long as
# it's less than 100.
def pu
  set $uni = $arg0
  if $argc == 2
    set $limit = $arg1
    if $limit > 100
      set $limit = 100
    end
  else
    set $limit = 100
  end
  # scratch array with space for 100 chars plus null terminator.  Make
  # sure to not use ' ' as the char so this copy/pastes well.
  set $scratch = "____________________________________________________________________________________________________"
  set $i = 0
  set $scratch_idx = 0
  while (*$uni && $i++ < $limit)
    if (*$uni < 0x80)
      set $scratch[$scratch_idx++] = *(char*)$uni++
    else
      if ($scratch_idx > 0)
	set $scratch[$scratch_idx] = '\0'
	print $scratch
	set $scratch_idx = 0
      end
      print /x *(short*)$uni++
    end
  end
  if ($scratch_idx > 0)
    set $scratch[$scratch_idx] = '\0'
    print $scratch
  end
end

# Define a "ps" command to display subclasses of nsAC?String.  Note that
# this assumes strings as of Gecko 1.9 (well, and probably a few
# releases before that as well); going back far enough will get you
# to string classes that this function doesn't work for.
def ps
  set $str = $arg0
  if (sizeof(*$str.mData) == 1 && ($str.mFlags & 1) != 0)
    print $str.mData
  else
    pu $str.mData $str.mLength
  end
end

This is hard. Give me a .gdbinit that already has the functions.

        Define a small helper function "punichar" in #ifdef NS_DEBUG code somewhere.

How do I display an nsString?

You can call the ToNewCString() method on the nsString. It leaks a little memory but it shouldn't hurt anything if you only do it a few times in one gdb session. (via akkana@netscape.com)

(gdb) p string.ToNewCString()

Another method (via bent) is the following (replace n with: the returned length of your string):

(gdb) p string.Length()
$1 = n
(gdb) x/ns string.BeginReading()

You can of course use any of the above unichar-printing routines instead of x/s.
This is hard. Give me a .gdbinit that works.

See Boris Zbarsky's .gdbinit. It contained several function definitions including:

        "prun" to start the browser and disable library loading.
        "pu" which will display a (PRUnichar *) string.
        "ps" which will display a nsString.

How do I determine the concrete type of an object pointed to by an interface pointer?

You can determine the concrete type of any object pointed to, by an XPCOM interface pointer, by looking at the mangled name of the symbol for the object's vtable:

(gdb) p aKidFrame
$1 = (nsIFrame *) 0x85058d4
(gdb) x/wa *(void**)aKidFrame
0x4210d380 <__vt_14nsRootBoxFrame>: 0x0
(gdb) p *(nsRootBoxFrame*)aKidFrame
 [ all the member variables of aKidFrame ]

If you're using gcc 3.x, the output is slightly different from the gcc 2.9x output above. Pay particular attention to the vtable symbol, in this case __vt_14nsRootBoxFrame. You won't get anything useful if the shared library containing the object is not loaded. See How do I load shared libraries? and How do I see what libraries I already have loaded?

Or use the gdb command set print object on.
How can I debug JavaScript from gdb?

If you have JavaScript Engine code on the stack, you'll probably want a JS stack in addition to the C++ stack.

(gdb) call DumpJSStack() 

See https://www.mozilla.org/scriptable/javascript-stack-dumper.html for more JS debugging tricks.
How can I debug race conditions and/or how can I make something different happen at NS_ASSERTION time?

[submitted by Dan Mosedale]
As Linux is unable to generate useful core files for multi-threaded applications, tracking down race-conditions which don't show up under the debugger can be a bit tricky. Unless you've given the --enable-crash-on-assert switch to configure, you can now change the behavior of NS_ASSERTION (nsDebug::Break) using the XPCOM_DEBUG_BREAK environment variable.
How do I run the debugger in emacs/xemacs?

Emacs and XEmacs contain modes for doing visual debugging. However, you might want to set up environment variables, specifiying the loading of symbols and components. The easiest way to set up these is to use the run-mozilla.sh script, located in the dist/bin directory of your build. This script sets up the environment to run the editor, shell, debugger, or defining a preferred setup and running any commands you wish. For example:

$ ./run-mozilla.sh /bin/bash
MOZILLA_FIVE_HOME=/home/USER/src/mozilla/build/dist/bin
  LD_LIBRARY_PATH=/home/USER/src/mozilla/build/dist/bin
     LIBRARY_PATH=/home/USER/src/mozilla/build/dist/bin
       SHLIB_PATH=/home/USER/src/mozilla/build/dist/bin
          LIBPATH=/home/USER/src/mozilla/build/dist/bin
       ADDON_PATH=/home/USER/src/mozilla/build/dist/bin
      MOZ_PROGRAM=/bin/bash
      MOZ_TOOLKIT=
        moz_debug=0
     moz_debugger=

GDB 5 used to work for me, but now Firefox won't start. What can I do?

A recent threading change (see bug 57051 for details) caused a problem on some systems. Firefox would get part-way through its initialization, then stop before showing a window. A recent change to gdb has fixed this. Download and build  the latest version of Insight, or if you don't want a GUI, the latest version of gdb.
"run" or "prun" in GDB fails with "error in loading shared libraries."

Running mozilla-bin inside GDB fails with an error message like:

Starting program:
/u/dmose/s/mozilla/mozilla-all/mozilla/dist/bin/./mozilla-bin
/u/dmose/s/mozilla/mozilla-all/mozilla/dist/bin/./mozilla-bin: error
in loading shared libraries: libraptorgfx.so: cannot open shared
object file: No such file or directory

Your LD_LIBRARY_PATH is probably being reset by your .cshrc or .profile. From the GDB manual:

*Warning:* GDB runs your program using the shell indicated by your 'SHELL' environment variable if it exists (or '/bin/sh' if not). If your 'SHELL' variable names a shell that runs an initialization file -- such as '.cshrc' for C-shell, or '.bashrc' for BASH--any variables you set in that file affect your program. You may wish to move the setting of environment variables to files that are only run when you sign on, such as '.login' or '.profile'.
Debian's GDB doesn't work. What do I do?

Debian's unstable distribution currently uses glibc 2.1 and GDB 4.18. However, there is no package of GDB for Debian with the appropriate threads patches that will work with glibc 2.1. I was able to get this to work by getting the GDB 4.18 RPM from Red Hat's rawhide server and installing that. It has all of the patches necessary for debugging threaded software. These fixes are expected to be merged into GDB, which will fix the problem for Debian Linux. (via Bruce Mitchener)
Firefox is aborting. Where do I set a breakpoint to find out where it is exiting?

On Linux there are two possible symbols that are causing this: PR_ASSERT() and NS_ASSERTION(). To see where it's asserting you can stop at two places:

(gdb) b abort
(gdb) b exit

I keep getting a SIGSEGV in JS/JIT code under gdb even though there is no crash when gdb is not attached.  How do I fix it?

Set the JS_DISABLE_SLOW_SCRIPT_SIGNALS environment variable (in FF33, the shorter and easier-to-remember JS_NO_SIGNALS).  For an explanation, read Jan's blog post.
I keep getting a SIG32 in the debugger. How do I fix it?

If you are getting a SIG32 while trying to debug Firefox you might have turned off shared library loading before the pthreads library was loaded. For example, set auto-solib-add 0 in your .gdbinit file. In this case, you can either:

        Remove it and use the method explained in the section about GDB's memory usage
        Use handle SIG32 noprint either in gdb or in your .gdbinit file

Alternatively, the problem might lie in your pthread library. If this library has its symbols stripped, then GDB can't hook into thread events, and you end up with SIG32 signals. You can check if your libpthread is stripped in file /lib/libpthread* and looking for 'stripped'.To fix this problem on Gentoo Linux, you can re-emerge glibc after adding "nostrip" to your FEATURES in /etc/make.conf.
How do I get useful stack traces inside system libraries?

Many Linux distributions provide separate packages with debugging information for system libraries, such as gdb, Valgrind, profiling tools, etc., to give useful stack traces via system libraries.
Fedora

On Fedora, you need to enable the debuginfo repositories, as the packages are in separate repositories. Enable them permanently, so when you get updates you also get security updates for these packages. A way to do this is edit /etc/yum.repos.d/fedora.repo and fedora-updates.repo to change the enabled=0 line in the debuginfo section to enabled=1. This may then flag a conflict when upgrading to a new distribution version. You would the need to perform this edit again.

You can finally install debuginfo packages with yum or other package management tools. The best way is install the yum-utils package, and then use the debuginfo-install command to install all the debuginfo:

# yum install yum-utils
# debuginfo-install firefox

This can be done manually using:

# yum install GConf2-debuginfo ORBit2-debuginfo atk-debuginfo \
 cairo-debuginfo dbus-debuginfo dbus-glib-debuginfo expat-debuginfo \
 fontconfig-debuginfo freetype-debuginfo gcc-debuginfo glib2-debuginfo \
 glibc-debuginfo gnome-vfs2-debuginfo gtk2-debuginfo gtk2-engines-debuginfo \
 hal-debuginfo libX11-debuginfo libXcursor-debuginfo libXext-debuginfo \
 libXfixes-debuginfo libXft-debuginfo libXi-debuginfo libXinerama-debuginfo \
 libXrender-debuginfo libbonobo-debuginfo libgnome-debuginfo \
 libselinux-debuginfo pango-debuginfo popt-debuginfo scim-bridge-debuginfo

Ubuntu 8.04

Ubuntu provides similar debug symbol packages for many of its libraries, though not all of them. To install them, run:

$ sudo apt-get install libatk1.0-dbg libc6-dbg libcairo2-dbg \
 libfontconfig1-dbg libgcc1-dbg libglib2.0-0-dbg libgnomeui-0-dbg \
 libgnomevfs2-0-dbg libgnutls13-dbg libgtk2.0-0-dbg libice6-dbg \
 libjpeg62-dbg libpango1.0-0-dbg libpixman-1-0-dbg libstdc++6-4.2-dbg \
 libx11-6-dbg libx11-xcb1-dbg libxcb1-dbg libxft2-dbg zlib1g-dbg

Debugging electrolysis (e10s)

You can find some (outdated) information on https://wiki.mozilla.org/Electrolysis/Debugging. You may also like to read https://mikeconley.ca/blog/2014/04/25/electrolysis-debugging-child-processes-of-content-for-make-benefit-glorious-browser-of-firefox for a more up-to-date blog post.

To get the child process id use:

MOZ_DEBUG_CHILD_PROCESS=1 mach run


browser/app/nsBrowserApp.cpp: main

/build/build/firefox/firefox/widget/xremoteclient/XRemoteClient.cpp:164


/build/build/firefox/firefox/toolkit/components/startup/nsAppStartup.cpp:246
nsAppStartup::CreateHiddenWindow()

/build/build/firefox/firefox/xpfe/appshell/nsAppShellService.cpp:641
nsAppShellService::JustCreateTopWindow

/build/build/firefox/firefox/widget/gtk/nsWindow.cpp:3653
nsWindow::Create
MakeFullScreen 1280
5769 key_press_event_cb

/build/build/firefox/firefox/obj-x86_64-pc-linux-gnu/dom/bindings/WindowBinding.cpp:4760
set_fullScreen

/build/build/firefox/firefox/dom/base/nsGlobalWindowInner.cpp:3604
nsGlobalWindowInner::SetFullScreen

/build/build/firefox/firefox/toolkit/components/remote/nsGTKRemoteService.cpp:33
nsGTKRemoteService::Startup

/build/build/firefox/firefox/widget/gtk/nsAppShell.cpp:53
PollWrapper

/build/build/firefox/firefox/xpfe/appshell/nsWebShellWindow.cpp:660

Also note, when surfing web pages which use JavaScript, that JavaScript code is executed within a sand box, and does not have access to Mozilla’s internal objects. Only those objects that are exposed by DOM (Document Object Model) are accessible.

NSPR – Netscape portable runtime

The intention of NSPR is to provide a layer between the OS and the Mozilla source code, to allow for simpler coding in other areas of the Mozilla source code. 

NSPR provides an operating system independent facility to program with multiple threads. For example, all network data transfer happens on a separate thread, to make sure the user interface stays responsive while data is being transferred.

When components talk to each other, they only do so using well-defined interfaces using the component object model (COM).

One step of building Mozilla is automatically translating the interface definition files into C/C++ header files. That’s the job of Mozilla’s own IDL compiler, xpidl.

Besides of the methods and data members, interfaces have additional attributes. They have a UUID, a number to uniquely identify an interface. Interfaces can have the scriptable attribute, which means they will be accessible from the JavaScript code. A scriptable interface is restricted to only use data types for parameters that are valid within the JavaScript runtime.
XPCOM / nsISupports / nsCOMPtr

XPCOM is Mozilla’s own implementation of COM – the component object model. The XP in front means it is cross-platform. The fact that it is cross-platform makes XPCOM more versatile than any other form of COM.

Typically, an interface describes an object that can be used to get a job done. If you have a job to do, you need to request an implementation that provides the interface. That implementation can reside within another component. To decide which particular implementation you want, you are using a contract ID, which is a text based identifier. The contract ID is a contract on the behaviour of the implementation, accessible using the well defined interface. The XPCOM runtime system knows which class implements which contract ID, and which component provides it.

Even if your code stays completely within one component, and therefore using COM is not a strict requirement, it is very often used anyway. One reason is flexibility. Another is to allow sharing functionality with those portions of the Mozilla logic that are implemented using JavaScript. Mozilla provides a technology called XPConnect that enables communication between interpreted JavaScript and compiled C++ at runtime.

Whenever you request an instance of a COM object at runtime, a class object will be created and you will be given an interface pointer. For several reasons it was decided that these instances are reference counted. One reason is efficiency, since making unnecessary copies of objects should be avoided. Another requirement is that when data objects must be passed between threads, each thread needs to keep a pointer to the same data object in memory. Finally, the same data object might be referenced by multiple components, or stored in multiple lists.

As the lifetime of each reference is different, it is easiest to have each object maintain a reference count, to remember how often it is currently referenced by something else. When you are given a reference to an object (be it from the XPCOM engine directly or from a function call), you have to make sure that you care for reference counting. You must tell the object whether you want to keep a reference to it, or whether you are finished with it, and remove the reference. That way, the object can decide on its own whether it is still needed. When its not needed anymore, it deletes itself from memory.

In order to implement this generic functionality, all classes in Mozilla that implement any interface share the common base class nsISupports, which implements the reference counter and automatic destruction functionality. A similar base class exists in every implementation of COM.

There is a general rule that you must clean up what you allocate. For instance, if you add a reference, you are strongly encouraged to release the reference as soon as it is no longer needed. If you don’t, you might cause problems such as memory leaks.

In C++, this can be done by explicit method calls to methods of the nsISupports base class. But calling these methods is not only easy to forget, but it also makes your code less readable – especially since many functions/methods have multiple exit points (i.e. return statements).

You have to make sure that you release all your referenced objects in each of your exit points. To make this easier, and to not have to repeat many release calls, a general helper class has been provided for dealing with pointers to COM objects, whose name is nsCOMPtr. This is something special to XPCOM and makes coding COM much easier. It simulates a pointer by overriding certain operators. Although there might be some edge cases, the following general rule should be followed for nearly all code: Whenever you’d expect to use a pointer variable “interface*” to an object that implements an interface, use a local “nsCOMPtr<interface>” variable instead. As soon as this pointer goes “out of scope”, its destructor will automatically decrease the reference count, triggering destruction if possible.

In interpreted JavaScript this is easier to code, because of garbage collection. There is some magic that automatically releases the references when possible. However, this magic requires that you don’t introduce cycles. For example, if you have two objects, and each contain a reference to the other, but nobody else keeps a reference to them, this can not be detected. Those objects will live in memory for the rest of program execution.
Exceptions / nsresult

Code execution can fail at runtime. One programming mechanism to deal with failure is to use exceptions. While Mozilla uses Exceptions in its JavaScript code portions, it does not in C++. One of several reasons for this is exceptions haven’t always been portable, so what was done in the past has stuck. Mozilla C++ code uses return codes to simulate exceptions. That means that while you can use try-catch blocks in JavaScript, in C++ you should check the return code whenever you call an interface method. That return code is of type nsresult. For this reason, the logical return type, as defined in the IDL file, is mapped to an additional method parameter in the C++ code.

The nsresult type is intended to transport additional failure reasons. Instead of simply reporting failure or success, an integer type is used instead, to allow for the definition of a lot of different error codes.

There are some general result codes, like NS_OK, which is used to indicate that everything went well and program flow can continue, or NS_ERROR_FAILURE, which is used if something went wrong, but no more details need to be provided as of yet.

In addition to that, each component can request its own range of integers to define error codes that will not overlap with those failure codes used in other areas of an application. Look at mozilla/xpcom/base/nsError.h for more information.
Strings in C++

While many application frameworks or class libraries decided to use just a single string class, the Mozilla developers have decided they need something more powerful. They have implemented a hierarchy of several string classes, which allows the dynamic runtime behaviour to be optimized for different situations. Sometimes you just need a fixed size string; sometimes you need a large string that grows over time. Therefore, for example, not only flat strings, but also segmented string types are available.

An additional requirement is that Mozilla has to be fully multi-language. All strings that deal with information shown to a user are therefore using multi-byte Unicode character strings.

The string types are template-based, with the character type as the variable type, to allow the same logic to be used with regular and Unicode strings.

While that approach of having many string classes means a lot of flexibility, the drawback is that learning Mozilla’s string classes is not trivial.
Graphical User Interface / XUL

Most operating systems define their own way to develop graphical user interfaces, and they are mostly different.

For a cross-platform application like Mozilla it is crucial to have a set of technologies that hide the operating system dependent logic from the application logic.

In the past a lot of C/C++ libraries have been coded that were cross-platform. To my knowledge, none of them are used in Mozilla, yet we have created our own graphics system.

When defining the layout of a GUI (graphical user interface), you can choose to go with either of two possibilities. You could define the absolute positions of each UI (user interface) element that you want to appear. This approach actually has been chosen by a lot of GUI libraries. But it has some drawbacks – you are not very flexible when the layout changes when adding more elements, because you have to rearrange all elements to new positions. You also have to do that graphically, to get immediate feedback which elements overlap, etc. But still, the UI might not look as intended when a different font with different metrics has to be used – this can make a UI unusable.

Mozilla developers wanted to have something very flexible. As Mozilla is cross-platform, it has to be very flexible with regards to fonts.

Mozilla developers have chosen to use an approach where the contents of the UI are designed in a logical manner. We don’t currently use a UI editor. We write a file with instructions on how the UI should look. At runtime, the layout engine decides which fonts are available, and considers all the requests that have been defined in the UI description, and creates the actual UI dynamically. This is similar to what the web browser does when displaying web pages.

The web has gone from a mostly text-based system to a very graphically rich environment that has user interfaces akin to many programs. Therefore, it was only natural for a web browser to use web languages in order to define its user interface. It was decided to use an XML based syntax to write the UI description, which has been called XUL (extensible user-interface language, pronounced ‘zool’). (A good reference for XUL is available at XULPlanet).

A XUL file describes what elements the UI consists of, and where elements appear. The XUL language defines attributes that allow the programmer to define the user actions that controls will react to. To define the dynamic behaviour of the application, one can define JavaScript functions that will be called when certain user interface events happen. Within those JavaScript functions, you can either do the required application behaviour directly, or call any other application logic available through COM objects, including logic defined in C++.

In addition to the logical representation of the UI, people also prefer to have a pretty looking UI. To define the detailed characteristics of the UI, one uses CSS files, which define, for example, which images will be used to display certain UI elements. This makes it flexible enough to define additional “looks” for the application, which are referred to as “themes” or “skins”. Mozilla defines currently two themes, classic and modern, which are actively maintained by the Mozilla developers. While there are additional themes, they often exist only for certain versions of Mozilla. It is a lot of work for a theme designer to stay in sync with all the changes that happen to the UI each day.
Build System and Tree

Nowadays Mozilla is mostly used as a group of many shared libraries, loaded dynamically at runtime as needed. The COM system allows for a development environment, where you often need to re-compile only a subset of the application when you make changes to areas of the source code base. This is a very convenient development environment, but it means a slowdown at runtime. On the other hand, it is possible to create an executable where Mozilla’s internal components are mostly linked statically. Due to the size of the application, this link step takes a lot of time, and makes most sense when preparing a package for distribution.

Each component contains its own directory in the root directory of Mozilla. It might also contain subcomponents within it called modules. There are make files throughout the tree which tell Mozilla how to build.

Most of the platform-specific code is contained in only a few places of the tree. The layer that sits between the operating system and the rest of the Mozilla code consists of the interfaces that are implemented by this code. Only the platform-specific code pertaining to the platform in which the build is occurring is built.

Operating system messages are gathered by the platform-specific code and then sent to the platform-independent code in the same way.

Widgets, with respect to the Mozilla project, are OS-independent widgets drawn using platform-specific rendering techniques.

Within the tree, directories named ‘public’ usually contain interface headers, and directories named ‘src’ usually contain interface implementations and non-interface headers.

Building this program can be daunting for someone not used to a project this large. It can take from 20 minutes on a powerful workstation to a couple hours on an older PC. First, you have to get the source, then build it using the rules contained in http://www.mozilla.org/build/. While it’s building, it will move all the header files to the dist/include directory so that you don’t have to specify the directory of each header. It will also copy all the XUL, graphics, and JavaScript files (collectively called the chrome) to the chrome directory (a child of the directory containing the Mozilla binary). They are mapped to chrome:// URLs as defined in a file called jar.mn. In release versions of Mozilla, the chrome directory will only contain .jar files.

Building Mozilla is only part of the process. If you want to develop, you will have to maintain the tree using a program called Mercurial. When the build fails, you have to solve conflicts that occur when a merge between a file in your tree and one in the repository fails. Also, when you hack the tree, you have to build parts of the trees you modified. Occasionally, you will have to rebuild the entire tree using a process called depending, so that dependencies between source files can be determined. Also, you will occasionally have to rebuild the tree. Most likely, while you are doing this, you will be maintaining your own changes to the tree and trying to keep it in sync with others’ changes.
Application Startup

Mozilla’s COM system relies on type libraries and an internal registry of available components and interfaces. During application startup, a check is being made whether the registry is still up to date. If changed libraries are detected, the registry is updated. Each component is allowed to do initialization during that registration phase. If changed libraries are detected, they are loaded and their initialization logic is executed. Besides changed libraries, other application components are loaded only as they become required.
Internal Notification System

This section describes an engine that is available inside Mozilla. Most of the time you don’t need it, but if you are required to react to certain events, knowing this system might be helpful. The idea of OOP is, you only do your own job. However, it often happens that component A needs to react when another component triggers an action in B. But component B should not know the details of what needed to happen, because its better to keep parts of the code separate. What is required here is: If other components need to react to B’s actions, B should be extended to send out notifications when its actions are triggered. In addition, B keeps a dynamic list at runtime, where it remembers who wants to be notified. At runtime, when A becomes initialized, A informs B that it wants to become a member of the notification list.

To make this more generic, it has been decided to use a central observer service. When component B triggers an action, it just notifies the observer service, specifying a descriptive name of the event. Components like A subscribe to the observer service giving the names of events they would like to observe. Using that principle, only the observer service has to deal with lists of components watching for events. When the observer receives notification for an event, it passes that notification on to all listening components for that event. Look at nsIObserver for more information.
Localization

Mozilla was designed to separate code from human language. Whenever you need to use a text string that needs to be shown to a user, you are not allowed to store that string directly in your JavaScript or C++ file. Instead you define a descriptive identifier for your text, which is used in your C++ or JavaScript file. You use that identifier as a key and call members of the string bundle interface to retrieve the actual text. The text itself is stored in separate files, that only contain text. As Mozilla is modular, there are a lot of these files, each owned by a separate module. With that separation, it is easy to have translators create different language versions of these text files.

When defining the UI, there are two kinds of strings. Some strings are known at the time the application is compiled and packaged, like labels for input fields, or the text that appears within the help system. Other text is assembled dynamically at runtime.

Whenever you define text that does not need to be accessed at runtime, you define it in DTD files. You can refer to that text directly in XUL files.

If you need to work with text at runtime, for example if your text contains a placeholder for a user name that needs to be filled at runtime, you define your text in properties files.
Coding and Review Rules

You are free to download Mozilla, change the code, and work with a version that contains your own changes.

However, one idea behind open source as used in Mozilla is the following: You received the source for free, and if you make changes, you should consider giving something back to the community. By doing so, you become a contributor.

But the Mozilla community has decided that it can’t accept just any change to be integrated into the public central Mozilla codebase. If you want your code to become a part of it, you need to follow rules. These rules are not like law, but basically you must convince people that your change is good.

The idea is, a lot of effort has been made to get the Mozilla code to its current state. While the Mozilla code is, like every piece of software, far from being perfect, people try to avoid anything that decreases maintainability, and to only allow code that seems to be correct.

In order to achieve this, the Mozilla community has decided that all code has to be reviewed by other already known Mozilla hackers. There are two levels of review. You should get a first review (r=) from the owner of the code area you are changing. You are expected to make the corrections that are requested, or your code will not go in. If you have that first review, you need to get what is called super-review (sr=) in most cases. There are only a limited number of “Mozilla gurus” that have an accepted reputation for making good decisions on what code is good and what needs to be changed. Once you have both reviews, your code may be checked in most of the time. There are other levels of review in certain instances, and that is described elsewhere.

Many people make changes to Mozilla. While everybody tries to make Mozilla better, every change has the risk of unforeseen side effects. These range from application features that, as a result of the change, no longer work, to the problem that the Mozilla source code simply does not compile anymore. The latter is called “the tree is broken, burning, or red”, where tree refers to the Mercurial source repository.

If you are developing only on one combination of operating system and compiler, and you are not paying attention to the portability rules (read the documents on mozilla.org), it is very easy to break the tree. You should try your best to not break it, and getting review hopefully will find potential problems early, before the changes are checked in.

A broken tree sucks. The Mozilla community has decided on the rule, that no other check-ins are allowed while the tree is broken. This is to help people who try to fix the problem. Allowing additional changes would make it even more difficult to find the real cause of a problem, because any new check-in could contain new problems.

xul


This tutorial is a guide to learning XUL (XML User Interface Language) which is a cross-platform language for describing applications' user interfaces.

This tutorial will demonstrate creating a simple find file user interface, much like that provided by the Macintosh's Sherlock or the find file dialog in Windows. Note that only the user interface will be created, with only limited functionality. The actual finding of files will not be implemented. A blue line will appear to the left of a paragraph where the find file dialog is being modified. You can follow along by looking for these sections.
What is XUL and why was it created?

XUL (pronounced "zool" and rhyming with "cool") was created to make development of the Mozilla browser easier and faster. It is an XML language so all features available to XML are also available to XUL.

Most applications need to be developed using features of a specific platform making building cross-platform software time-consuming and costly. A number of cross-platform solutions have been developed in the past. Java, for example, has portability as a main selling point. XUL is one such language designed specifically for building portable user interfaces. It takes a long time to build an application even for only one platform. The time required to compile and debug can be lengthy. With XUL, an interface can be implemented and modified quickly and easily.

XUL has all the advantages of other XML languages. For example XHTML or other XML languages such as MathML or SVG can be inserted within it. Also, text displayed with XUL is easily localizable, which means that it can be translated into other languages with little effort.
What kinds of user interfaces can be made with XUL?

XUL provides the ability to create most elements found in modern graphical interfaces. Some elements that can be created are:

    Input controls such as textboxes and checkboxes
    Toolbars with buttons or other content
    Menus on a menu bar or pop up menus
    Tabbed dialogs
    Trees for hierarchical or tabular information
    Keyboard shortcuts

The displayed content can be created from the contents of a XUL file or with data from a datasource. In Mozilla, such datasources include a user's mailbox, their bookmarks and search results. The contents of menus, trees and other elements can be populated with this data, or with your own data supplied in an RDF file.

There are several ways you can use XUL:

Firefox extension
    An extension adds functionality to the browser itself, often in the form of extra toolbars, context menus, or customizations to the browser's user interface. This is done using a feature of XUL called an overlay, which allows the UI provided from one source, in this case, the Firefox browser, to be merged together with the UI from the extension. Extensions may also be applied to other Mozilla based products such as Thunderbird.
Standalone XULRunner application
    XULRunner is a packaged version of the Mozilla platform which allows you to create standalone XUL applications. A browser isn't required to run these applications, as they have their own executable file.
XUL package
    In between the other two are applications which are created in the same way as an extension, but they act like a separate application in a separate window. This is used when you don't want to have the larger size of a complete XULRunner application, but don't mind requiring a Mozilla browser to be installed to be able to run the application.
Remote XUL application
    You could also just place XUL code on a web server and open it in Firefox, as you would any other web page, however this is discouraged and was disabled in Firefox 8.  It is still possible to enable this for selected sites to let legacy apps to continue working, but for new remote applications you should use HTML to create your user interface instead; most of the features you used to have to use XUL for are available in HTML5.

The first three types all require an installation to be performed on the user's machine. However, these types of applications do not have security restrictions placed on them, so they may access local files and read and write preferences, for example. For extensions, the XUL files and associated scripts and images used by an application would be packaged into a single file and downloaded and installed by the user. Mozilla applications such as Firefox provide an extension manager which allows packages to be installed without having to write a lot of complex code.
What do I need to know to use this tutorial?

You should have an understanding of HTML and at least a basic understanding of XML and CSS. Here are some guidelines to keep in mind:

    XUL elements and attributes should all be entered in lowercase as XML is case-sensitive (unlike HTML).
    Attribute values in XUL must be placed inside quotes, even if they are numbers.
    XUL files are usually split into four files, one each for the layout and elements, for style declarations, for entity declarations (used for localization) and for scripts. In addition, you may have extra files for images or for platform specific data.

XUL is supported in Mozilla and browsers that are also based upon on the Gecko engine, such as Netscape 6 or later and Mozilla Firefox. Due to various changes in XUL syntax over time, you will want to get the latest version for the examples to work properly. XUL is fairly similar in Firefox and to other browsers, although it has some specific differences such as support for customizable toolbars.

This tutorial attempts to cover much of XUL's functionality, however, not all features are discussed. Once you are familiar with XUL, you can use the XUL Reference to find out about other features supported by specific elements.

Next »
Document Tags and Contributors
Tags: 

    Tutorials XUL XUL Tutorial 

Contributors to this page: SphinxKnight, xfq, Sheppy, bestlong, teoli, bovinespirit, fscholz, Dyhan81, Xprommer, Chbok, Mgjbot, Albertoknox, Solon, Taken, Waldo, Ptak82, Dria, Yama, Enn
Last updated by: SphinxKnight, Feb 5, 2018, 12:40:07 PM
Learn the best of web development

We'll begin by looking at how the XUL is handled in Mozilla.
How XUL is Handled

In Mozilla, XUL is handled in much the same way as HTML or other types of content are handled. When you type the URL of an HTML page into the browser's address field, the browser locates the web site and downloads the content. The Mozilla rendering engine takes the content in the form of HTML source and transforms it into a document tree. The tree is then converted into a set of objects that can be displayed on the screen. Style sheets (CSS), images, and other technologies are used to control the presentation. XUL functions in much the same way.

In fact, in Mozilla, all document types, whether they are HTML or XUL, or even SVG, are all handled by the same underlying code. This means that the same CSS properties may be used to style both HTML and XUL, and many of the features can be shared between both. However, there are some features that are specific to HTML, such as forms, and others which are specific to XUL, such as overlays. Since XUL and HTML are handled in the same way, you can load both from either your local file system, from a web page, or from an extension or standalone XULRunner application.

Content from remote sources (e.g. http://localhost/~username/ ), regardless of whether they are HTML or XUL or another document type, are limited in the type of operations they can perform, for security reasons. For this reason, Mozilla provides a method of installing content locally and registering the installed files as part of its chrome system. This allows a special URL form, called a chrome:// URL, to be used. By accessing a file using a chrome URL, the files receive elevated privileges to access local files, access preferences and bookmarks and perform other privileged operations. Obviously, web pages do not get these privileges, unless they are signed with a digital certificate and the user has granted permission to perform these operations.

This chrome package registration is the way Firefox extensions are able to add features to the browser. The extensions are small packages of XUL files, JavaScript, style sheets and images packed together into a single file. These packaged files can be created by using a ZIP utility. When the user downloads the zipped file, the packaged files will be installed onto the user's machine. The extension will hook into the browser using a XUL specific feature called an overlay which allows the XUL from the extension to mesh with the XUL in the browser. To the user, it may seem like the extension has modified the browser, but in reality, the code is all separate, and the extension may be easily uninstalled. Registered packages are not required to use overlays, of course. If they don't, you won't be able to access them via the main browser interface, but you can still access them via the chrome URL, if you know what it is.

Standalone XUL applications may include XUL code in a similar way, but, of course, the XUL for the application will be included as part of the installation, instead of having to be installed separately as an extension. However, this XUL code will be registered in the chrome system such that the application can display the UI.

It is worth noting that the Mozilla browser itself is actually just a set of packages containing XUL files, JavaScript and style sheets. These files are accessed via a chrome URL and have enhanced privileges and work just like any other package. Of course, the browser is much larger and more sophisticated than most extensions. Firefox and Thunderbird, as well as number of other components are all written in XUL and are all accessible via chrome URLs. You can examine these packages by looking in the chrome directory where Firefox or another XUL application is built.

The chrome URL always begins with 'chrome://'. In much the same way as an 'http://' URL always refers to remote web sites accessed using HTTP, and the 'file://' URL always refers to local files, the 'chrome://' URL always refers to installed packages and extensions. We'll look more at the syntax of a chrome URL in the next section. It is important to note that when accessing content through a chrome URL, it gains the enhanced privileges described above that other kinds of URLs do not. For instance, an HTTP URL does not have any special privileges, and an error will occur if a web page tries, for example, to read a local file. However, a file loaded via a chrome URL will be able to read files with the same access privileges as the installed extension.

This distinction is important. This means that there are certain things that content of web pages cannot do, such as read the user's bookmarks. This distinction is not based on the kind of content being displayed; only on the type of URL used. Both HTML and XUL placed on a web site have no extra permissions; however both HTML and XUL loaded through a chrome URL have enhanced permissions.

Remote XUL was disabled in Firefox 8, so the following is of historic interest only.

If you are going to use XUL on a web site, you can just put the XUL file on the web site as you would an HTML file, and then load its URL in a browser http://localhost/xul.php. Ensure that your web server is configured to send XUL files with the content type of application/vnd.mozilla.xul+xml (eg with PHP header('Content-type: application/vnd.mozilla.xul+xml');). This content type is how Mozilla knows the difference between HTML and XUL. Mozilla does not use the file extension, unless reading files from the file system, but you should use the .xul extension for all XUL files. You can load XUL files from your own machine by opening them in the browser, or by double-clicking the file in your file manager.
Remember that remote XUL will have significant restrictions on what it can do, and DOES NOT WORK WITH CURRENT FIREFOX VERSIONS!
Document types: HTML XML XUL CSS

Mozilla uses a distinctly different kind of document object (DOM) for HTML and XUL, although they share much of the same functionality. There are three main types of document in Mozilla: HTML, XML, and XUL. Naturally, the HTML document is used for HTML documents, the XUL document is used for XUL documents, and the XML document is used for other types of XML documents. Since XUL is also XML, the XUL document is a subtype of the more generic XML document. There are subtle differences in functionality. For example, while the form controls on an HTML page are accessible via the document.forms property, this property isn't available for XUL documents, since XUL doesn't have forms in the HTML sense. Likewise, XUL specific features such as overlays and templates are only available in XUL documents.

This distinction between documents is important. It is possible to use many XUL features in HTML or XML documents since they aren't document type specific; however other features require the right kind of document. For instance, you can use the XUL layout types in other documents since they don't rely on the XUL document type to function.

To summarize the points made above:

    Mozilla renders both HTML and XUL using the same underlying engine and uses CSS to specify their presentation.
    XUL may be loaded from a remote site, from the local file system, or installed as a package and accessed using a chrome URL. This last option is what browser extensions do.
    Chrome URLs may be used to access installed packages and open them with enhanced privileges.
    HTML, XML, and XUL each have a different document type. Some features may be used in any document type whereas some features are specific to one kind of document.

The next few sections describe the basic structure of a chrome package which can be installed into Mozilla. However, if you just want to get started building a simple application, you may skip ahead to Creating a Window and save this section for later.
Package Organization

Mozilla is organized such that you can have as many components as you want pre-installed. Each extension is something of a standalone component with a separate chrome URL. Mozilla also has one component for each installed theme and locale. Each of these components, or packages, is made up of a set of files that describe the user interface for it. For example, the messenger component has descriptions of the mail messages list window, the composition window and the address book dialogs.

The packages that are provided with Mozilla are located within the chrome directory, which is in the directory where you built Mozilla. The chrome directory is where you find all the files that describe the user interface used by the Mozilla browser, mail client, and other applications. Typically, you put all the XUL files for an application in this directory, although extensions are installed in the extensions directory for a particular user. Just copying a XUL file into the chrome directory doesn't give the file any extra permissions, nor can it be accessed via a chrome URL. To gain the extra privileges, you will need to create a manifest file and put that in the chrome directory. This file is easy to create, as it is typically only a couple of lines long. It is used to map a chrome URL to a file or directory path on the disk where the XUL files are located. Details of how to create this file will be discussed in a later section.

The only way to create content that can be accessed through a chrome URL is by creating a package as described in the next few sections. This directory is called 'chrome' probably because it seemed like a convenient name to use in keeping with the chrome packages that are included with Mozilla.

To further the confusion, there are two other places where the word "chrome" might appear. These are the -chrome command line argument and the chrome modifier to the window.open() function. Neither of these features grant extra privileges; instead they are used to open a new top-level window without the browser UI such as the menu and toolbar. You will commonly use this feature in more complex XUL applications since you wouldn't want the browser UI to exist around your dialog boxes.

The files for a package are usually combined into a single JAR file. A JAR file may be created and examined using a ZIP utility. For instance, you can open the JAR files in Mozilla's chrome directory to see the basic structure of a package. Although it's normal to combine the files into a JAR file, packages may also be accessed in expanded form into a directory. Although you don't normally distribute a package this way, it is handy during development since you can edit the file directly and then reload the XUL file without having to repackage or reinstall the files.

By default, Mozilla applications parse XUL files and scripts, and store a pre-compiled version in memory for the remainder of the application session. This improves performance. However, because of this, the XUL will be not be reloaded even when the source files are changed. To disable this mechanism, it is necessary to change the preference nglayout.debug.disable_xul_cache. In Firefox, this preference may be added to the user preferences by typing "about:config" in the address field, and setting this value to true. Or, just manually edit your user.js preferences file and add the following line:

pref("nglayout.debug.disable_xul_cache", true);

There are usually three different parts to a chrome package, although they are all optional. Each part is stored in a different directory. These three sets are the content, the skin, and the locale, which are all described below. A particular package might provide one or more skins and locales, but a user can replace them with their own. In addition, the package might include several different applications, each accessible via different chrome URLs. The packaging system is flexible enough so that you can include whatever parts you need and allow other parts, such as the text for different languages, to be downloaded separately.

The three types of chrome packages are:

    Content - Windows and scripts
    The declarations of the windows and the user interface elements contained within them. These are stored in XUL files, which have a .xul extension. A content package can have multiple XUL files, but the main window should have a filename that is the same as the package name. For example, the editor package will have a file within it called editor.xul. Scripts are placed in separate files alongside the XUL files.
    Skin - Style sheets, images and other theme specific files
    Style sheets describe details of the appearance of a window. They are stored separately from XUL files to facilitate modifying the skin (theme) of an application. Any images used are stored here also.
    Locale - Locale specific files
    All the text that is displayed within a window is stored separately. This way, a user can have a set for their own language.

Content Packages

The name of the JAR file might describe what it contains, but you can't be sure unless you view its contents. Let's use the browser package included with Firefox as an example. If you extract the files in browser.jar, you will find that it contains a directory structure much like the following:

content
   browser
      browser.xul
      browser.js
      -- other browser XUL and JS files goes here --
      bookmarks
         -- bookmarks files go here --
      preferences
         -- preferences files go here --
.
.
.

This is easily recognizable as a content package, as the top-level directory is called content. For skins, this directory will usually be called skin and for locales, it will usually be called locale. This naming scheme isn't necessary, but this is a common convention to make the parts of a package clearer. Some packages may include a content section, a skin, and a locale. In this case, you will find a subdirectory for each type. For example, Chatzilla is distributed in this way.

The content/browser directory contains a number of files with .xul and .js extensions. The XUL files are the ones with the .xul extension. The files with .js extensions are JavaScript files containing scripts that handle the functionality of a window. Many XUL files have a script file associated with them, and some may have more than one.

In the listing above, two files have been shown. There are of course others, but for simplicity they aren't shown. The file browser.xul is the XUL file that describes the main browser window. The main window for a content package should have the same name as the package with a .xul extension. In this case, the package name is "browser" so we expect to find browser.xul. Some of the other XUL files describe separate windows. For example, the file pageInfo.xul describes the page info dialog.

Many packages will include a contents.rdf file, which describes the package, its author, and the overlays it uses. However, this file is obsolete and has been replaced with a simpler mechanism. This newer method is the manifest file mentioned earlier, and you will find these as files with the .manifest extension in the chrome directory. For instance, browser.manifest describes the browser package.

Several subdirectories, such as bookmarks and preferences, describe additional sections of the browser component. They are placed in different directories only to keep the files more organized.
Skins or Themes

Although the underlying code for Mozilla calls them skins and the user interface calls them themes, they're both referring to the same thing. The classic.jar file describes the default theme provided with Firefox. The structure is similar to the content packages. For example, examining classic.jar:

skin
   classic
      browser
         browser.css
         -- other browser skin files go here --
      global
         -- global skin files go here --
.
.
.

Again, this directory structure isn't necessary and is used for convenience. You can actually put all the files in one directory at the top level and not use subdirectories. However, for larger applications, subdirectories are used to separate the different components. In the example above, a directory exists for theme related files for the browser and another for global theme related files. The global directory contains skin files that are general to all packages. These files will apply to all components and will be included with your own standalone applications. The global part defines the appearance of all of the common XUL widgets, whereas the other directories have files that are specific to those applications. Firefox includes both the global and browser theme files in one archive, but they can be included separately.

A skin is made up of CSS files and a number of images used to define the look of an interface. The file browser.css is used by browser.xul and contains styles that define the appearance of various parts of the browser interface. Again, note how the file browser.css has the same name as the package. By changing the CSS files, you can adjust the appearance of a window without changing its function. This is how you can create a new theme. The XUL part remains the same but the skin part changes independently.
Locales

The file en-US.jar describes the language information for each component, in this case for US English. Like the skins, each language file contains files that specify text used by the package for a specific language. The locale structure is similar to the others, so it won't be listed here.

The localized text is stored in two types of files: DTD files and properties files. The DTD files have a .dtd extension and contain entity declarations, one for each text string that is used in a window. For example, the file browser.dtd contains entity declarations for each menu command. In addition, keyboard shortcuts for each command are also defined, because they may be different for each language. DTD files are used by XUL files so, in general, you will have one per XUL file. The locale part also contains properties files, which are similar, but are used by script files. The file browser.properties contains a few such localized strings.

This structure allows you to translate Mozilla or a component into a different language by just adding a new locale for that language. You don't have to change the XUL code at all. In addition, another person could supply a separate package that applies a skin or locale to your content part, thus providing support for a new theme or language without having to change the original package.
Other Packages

There is a special package called toolkit (or global). We saw the global directory earlier for skins. The file toolkit.jar contains the corresponding content part for it. It contains some global dialogs and definitions. It also defines the default appearance and functionality of the various common XUL widgets such as textboxes and buttons. The files located in the global part of a skin package contain the default look for all of the XUL interface elements. The toolkit package is used by all XUL applications.
Adding a Package

Mozilla places the packages that are included with the installation in the chrome directory. However, they do not need to be placed there. When installing another package, you can place it anywhere on the disk, as long as a manifest file points to it. It is common to place packages into the chrome directory simply because it is convenient; however, they will work just as well from another directory or somewhere on your local network. You cannot store them on a remote site, unless the remote site is mounted through the local file system.

There are two chrome directories used for XUL applications: one is in the same place where the application is built, while the other is part of user's profile. The former allows packages that are shared by all users while the latter allows packages to be created only for a specific user or users. Extensions, while installed in a separate extensions directory, are also usually user specific. Any manifest files located in either chrome directory will be examined to see which packages are installed.

In the next section, we'll look at how to refer to chrome packages using the chrome URL.

« Previous
Next »
Document Tags and Contributors
Tags: 

    Tutorials XUL XUL_Tutorial 

Contributors to this page: SphinxKnight, arshdkhn1, stephaniehobson, xfq, Sheppy, teoli, KyleHamilton, kaimnryu, fscholz, purdeaandrei, tct, iohann, Kennykaiyinyu, Chbok, Freeman77, Enn, Gluegadget, Jochen Kiene, Emery, Mgjbot, Pmash, Morishoji, SylvainPasche, Napolj2, Kozawa, Iron, Ptak82, Dria
Last updated by: SphinxKnight, Feb 5, 2018, 12:08:07 PM
Learn the best of web development


The following section will describe how to refer to XUL documents and other chrome files.
The Chrome URL

XUL files can be referenced with a regular HTTP URL (or any type of URL) just like HTML files. However, packages that are installed into Mozilla's chrome system can be referenced with special chrome URLs. The packages included with Mozilla will already be installed but you can register your own.

Installed packages have the advantage that they don't have security restrictions placed on them, which is necessary for many applications. Another advantage over other URL types is that they automatically handle multiple themes and locales. For example, a chrome URL lets you refer to a file in the theme such as an image without needing to know which theme the user is using. As long as the filenames are the same in each theme, you can refer to the file using a chrome URL. Mozilla will take care of determining where the file is located and return the right data. This also means that it doesn't matter where the package is installed to be able to access it. The chrome URLs are independent of where the files might physically be located. This makes it much easier to write applications that have lots of files since you don't have to worry about the details of locating files.

The basic syntax of a chrome URL is as follows:

chrome://<package name>/<part>/<file.xul>

The text <package name> is the package name, such as messenger or editor. All package names are case insensitive, but lowercase names are preferred. The <part> is either 'content', 'skin' or 'locale' depending on which part you want. <file.xul> is simply the filename.

Example: chrome://messenger/content/messenger.xul

The example here refers to the messenger window. You could point to a file that is part of a skin by replacing 'content' with 'skin' and changing the filename. Similarly, you can point to a file that is part of a locale by using 'locale' instead of 'content'.

When you open a chrome URL, Mozilla looks through its list of installed packages and tries to locate the JAR file or directory that matches the package name and part. The mapping between chrome URLs and JAR files are specified in the manifest files stored in the chrome directory. If you were to move the file messenger.jar somewhere else and update the manifest file accordingly, Thunderbird will still work since it doesn't rely on its specific installed location. By using chrome URLs we can leave details like this to Mozilla. Similarly, if the user changes their theme, the 'skin' part of the chrome URL translates to a different set of files, yet the XUL and scripts don't need to change.

Here are some more examples. Note how none of the URLs specify which theme or locale is used and none specify a specific directory.

chrome://messenger/content/messenger.xul
chrome://messenger/content/attach.js
chrome://messenger/skin/icons/folder-inbox.gif
chrome://messenger/locale/messenger.dtd

To refer to subdirectories, you can just add them to the end of the chrome URL. The following URL will refer to the bookmarks window, listed for Firefox:

chrome://browser/content/bookmarks/bookmarksManager.xul

You can enter chrome URLs anywhere normal URLs can be used. You can even enter them directly into the URL bar in a Firefox browser window. If you enter the URL mentioned above into the browser's address bar, you should see that window appear like a web page might do and for the most part will function as if it was a separate window. Some dialog boxes may not work right, however, as they may be expecting arguments to be supplied from the window that opened them.

You will also see chrome URLs without specified filenames, such as:

chrome://browser/content/

In this case, only the package name and part is specified. This type of reference will automatically select an appropriate file from that right directory. For content, a file with the same name of the package and a xul extension is selected. In the above example, the file browser.xul is selected. For messenger, messenger.xul would be selected. When creating your own applications, you will want to create a file for your main window with the same name as the package, so it can be referred to using this shorter form. This is convenient since all a user needs to know is the package name to be able to open the application. Of course, for extensions that modify the browser interface, the user will not need to know the URL, as the extension will present itself in the UI.

For a skin, the file <package name>.css is selected; for a locale, the file <package name>.dtd is selected.

Remember, the chrome URL is not related to where it is located on disk. The first two pieces of the chrome URL are the package name and the part (either content, skin, or locale). While it is common to put the content files in a directory called 'content', this is purely out of convention, and these files may be placed in an entirely different structure.

In the next section, we will look at how to create .manifest files and packages.

« Previous
Next »
Document Tags and Contributors
Tags: 

    Tutorials XUL XUL_Tutorial 

Contributors to this page: xfq, Sheppy, rsilveira, teoli, trevorh, Borden, fscholz, Enn, Johnjbarton, Chbok, Morishoji, Nickolay, Pmash, Mgjbot, Napolj2, Kozawa, Zuroc, Ptak82, Dria
Last updated by: xfq, Oct 11, 2014, 6:39:45 PM
Learn the best of web development



In this section, we'll see how to put chrome and XUL files into a package and create the manifest files for them.
Packages

A package is a set of XUL files and scripts that define the functionality of a user interface. Packages may be installed into Mozilla and referenced with chrome URLs. A package can contain any kinds of files and may be split into subdirectories for different parts of the package. A package can be stored either as a directory or as a JAR archive.
Manifest files

A manifest file describes a package and maps its location on disk to a chrome URL. The manifest files in the chrome directory will be examined when a Mozilla application starts up to see what packages are installed. That means that all you need to do to install a new package is add a new manifest file either into the application chrome directory or the user specific chrome directory. This latter chrome directory is normally the one used since the application directory might not have sufficient permissions to write into it.

Note: Starting in Gecko 2.0, only the file named chrome.manifest is read automatically; if you need to read multiple manifest files, use the manifest command in that file to import additional manifests. Use manifest flags to configure the scenarios in which the secondary manifest is imported.

If you just want to try testing privileged XUL code in the Firefox browser, you can do this easily by just using a manifest with only one line in it:

    Create a new directory somewhere. For example, on a Windows machine, you might use C:\testfiles
    Create a new ASCII1 file called test.manifest in the chrome directory. It doesn't actually matter what the file is called as long as it has the .manifest extension. ( 1. doesn't work with UTF-8 with BOM.)
    Add the following line to it:

content tests file:///C:/testfiles/

The file path in that line should point to the directory created above. If you aren't sure what the file path is, open that directory in a browser and copy the URL from the address field.

That's it! Now, all you need to do is add some XUL files into that new directory, and you will be able to load them by typing in a chrome URL of the form chrome://tests/content/<filename>. Of course, you will need to restart the browser for the changes to take effect. If the file doesn't load, make sure that the file path is correct.

The basic syntax of the lines in the manifest file for content packages is:

'content <packagename> <filepath>'

The first field 'content' indicates a content package. For themes, 'skin' is used while 'locale' is used for locales. The packagename in the example above is 'tests', which means that the first field in the chrome URL is 'tests' as in chrome://tests/content/sample.xul. If the package name was 'browser', the chrome URL would be chrome://browser/content/. The final field is the path where the files are located. This can be either a local file path using a file URL or a JAR archive using a jar URL, which will be described in a moment. You can specify multiple packages by including another line in the manifest file.

The browser.manifest file used by Firefox looks like this:

content branding jar:browser.jar!/content/branding/ xpcnativewrappers=yes
content browser jar:browser.jar!/content/browser/ xpcnativewrappers=yes
overlay chrome://global/content/viewSource.xul chrome://browser/content/viewSourceOverlay.xul
overlay chrome://global/content/viewPartialSource.xul chrome://browser/content/viewSourceOverlay.xul
overlay chrome://browser/content/pageInfo.xul chrome://pippki/content/PageInfoOverlay.xul

Two packages are listed here, 'branding' and 'browser'. Three overlays are also specified, which allow content from different packages to combine together. Extensions will make the most use of overlays, since they merge their UI with the browser UI.

The file paths for the branding and browser packages use jar URLs as the content is packaged up into an archive. A JAR archive can be created with a ZIP utility. For a JAR file located in the chrome directory, the syntax is fairly simple:

jar:<filename.jar>!/<path_in_archive>

For the browser package, the archive is browser.jar, located alongside the manifest file in the chrome directory. The path 'content/browser' specifies the path inside the archive where the XUL files are located. You won't need to specify a path if you don't have any directories in the archive. In this case, there is, since the files for the branding package are stored in a different path in the same archive.

For the 'tests' package created above, the files are not packaged into an archive, so a direct file path is used instead. This is good for development since you don't have to package up all the files every time you change them. However, when distributing an application or extension, you will want to package them into an archive to avoid having to install lots of smaller files.

The xpcnativewrappers=yes part at the end of the manifest line is a flag that may optionally be used. In JavaScript, it is possible for a web page to override built-in functions with their own code. If the xpcnativewrappers flag is specified, it indicates that scripts running in a privileged context don't call these overridden versions, but the original built-in versions instead. Otherwise, if an extension attempted to call the modified versions, it would likely not work properly, or worse, create a security hole. This flag was added to prevent this problem and should always be used for newer extensions, but is left out for older extensions that might not be compatible with the change. For more information about this feature, see XPCNativeWrapper.
Themes and Locales

The themes and locales, the syntax is similar as for content packages, but you also need to specify the content package you are providing a theme or locale for. For example:

skin browser classic/1.0 jar:classic.jar!/skin/classic/browser/
locale browser en-US jar:en-US.jar!/locale/browser/

For these, the extra field has been added to indicate that the skin and locale applies to the browser. The skin name is 'classic/1.0'. In this case, a version number is being used as part of the theme name, but that is optional if you are making your own theme. Mozilla doesn't handle the version number in a special way; the version number is just part of the theme name. The locale is 'en-US'. The chrome URLs that these would map to would be chrome://browser/skin and chrome://browser/locale. If you were creating your own theme or locale for the browser, all you need to do is create a manifest file with one of these two lines in it, modified to suit your theme or locale.

For more information about Themes, see Themes. For more information about Locales, see Localization.
Our example find files dialog

Let's create a manifest file for the find files dialog we'll be creating. You can combine all of the three types into a single file if you wish. This may be done when creating an extension such that all of the parts are in one file. We will do this for the find files dialog. Create a file findfile.manifest in the chrome directory. Add the following to the file:

content findfile file:///findfile/content/
skin findfile classic/1.0 file:///findfile/skin/
locale findfile en-US file:///findfile/locale/

Create the new directories listed above. It doesn't matter where the directories are created, but the file paths in the manifest file should point to the directories. Naturally, you will want to use directory paths suitable for your system. If we were distributing the package, we would want to package them up into a JAR file, and modify the paths. In this case, we are just creating to demonstrate a manifest file and to prepare directories for examples which will see in the later sections.

Note how the second field of the skin and locale lines specifies 'findfile'. This means that the skin and locale modify the findfile package, which was specified on the first line.The three paths above specify subdirectories for each part. You will want to create these subdirectories to keep each part's files separate.
Installing a Package

For an application to be installed, you will need to create an installer for it, or include it as part of another application. The method used depends on what kind of application you are creating. For extensions, you will need to create an install file install.rdf which describes what will be installed, the author of the extension and which versions of the browser or other applications it is compatible with. A specific directory structure is needed as well since extensions are limited in where the files may be installed to. An extension is packaged up into an XPI file. XPI is short for XPInstall and is used by Mozilla to install components. Like a JAR file, an XPI file is just a ZIP file with a different extension, so you can create and view XPI files with a ZIP utility.

Firefox's extension manager handles installing extensions packaged into XPI files automatically. It is recommended to upload extensions to the Mozilla Add-ons site, where users can locate them for installation. While they may be installed from any site, other sites are not configured to allow installations by default.

It is also possible to use a install script written in JavaScript to install files. This allows you to copy files to any location and perform other file management tasks. However, applications installed with a script will not be listed in the extension manager and there is no automated method to uninstall them. For this reason, the install scripts are not used often.

For standalone applications, they can be packaged up using XULRunner. This allows a separate executable file, and the application may be distributed independently of a browser.

For more information about creating extensions, see Extensions. For more information about XULRunner, see XULRunner.
Older Applications

If you are creating applications for older versions of Mozilla software, that is, before Firefox 1.5 or Mozilla 1.8, the process is a bit more involved. The following describes how to set up a package for earlier versions. This section may be skipped if you are writing new extensions or XUL applications.
Note: This older process does also apply to the new SeaMonkey 1.0 though. The codebase there has not yet adopted the "Manifest" format.

<?xml version="1.0"?>

<RDF:RDF xmlns:RDF="http://www.w3.org/1999/02/22-rdf-syntax-ns#"
         xmlns:chrome="http://www.mozilla.org/rdf/chrome#">

  <RDF:Seq about="urn:mozilla:package:root">
    <RDF:li resource="urn:mozilla:package:myapplication"/>
  </RDF:Seq>

  <RDF:Description about="urn:mozilla:package:myapplication"
          chrome:displayName="Application Title"
          chrome:author="Author Name"
          chrome:name="myapplication"
          chrome:extension="true"/>

</RDF:RDF>

content,install,url,file:///main/app/

    Create a directory somewhere on your disk. Many people put this as a subdirectory inside Mozilla's chrome directory, but this isn't necessary. The directory could be anywhere and on any disk. Put your XUL files in this directory.
    Create a file called contents.rdf and place it in this directory. Copy the text in the box below into the new contents.rdf file. This file is used to identify the application id, its name, author, version and so on.
    Change the highlighted parts of the file above to your own information. The red text 'myapplication' should be the ID of your application. You make this up, but typically, the ID is similar to your application's name. Replace the blue highlighted text above with your application's title and author.
    If the 'chrome:extension' field is true, the application is a Mozilla Firefox Extension and it will show up in the Extensions window of the browser. If false, it will not appear.
    Save the contents.rdf and make sure it is in the directory you created in step 1.
    Open the file <mozilla-directory>/chrome/installed-chrome.txt, where <mozilla-directory> is the directory where Mozilla is installed. Exit Mozilla before you do this.
    Next, you are going to register the new application with Mozilla so it will know where to find it. Add a line at the end of installed-chrome.txt pointing to the new directory you created in step 1. Change the highlighted text to the file URL below of the directory. Make sure that it URL ends with a slash and that you press enter at the end of the line. If you aren't sure what the URL is, open the directory created in step 1 into a Mozilla browser and copy the URL from the location field. Note that the reference should always be a directory, not a file.
    Delete the file <mozilla-directory>/chrome/chrome.rdf.
    Start Mozilla. You should be able to view any XUL files you put into the directory using a URL of the form: chrome://applicationid/content/file.xul where file.xul is the filename. Your main XUL file should be applicationid.xul which you can load using the shortcut URL chrome://applicationid/content/.

If you are creating skin and/or locale portions, repeat the steps above, except that the format of the contents.rdf file is slightly different. Look at the contents.rdf files in other applications for details.
Troubleshooting

Creating a chrome package can often be tricky and it is difficult to diagnose problems. Here are a few tips in case you get stuck.

    Open the file <mozilla-directory>/chrome/chrome.rdf. You should find references to your application ID in there. If not, something is wrong with registration. If it is there, you are probably using the wrong chrome URL when you load the file.
    Try deleting the <mozilla-directory>/chrome/chrome.rdf file. It will get regenerated. Also delete the entire <mozilla-directory>/chrome/overlayinfo/ directory if you are using overlays.
    Make sure that the URL in the line you added to installed-chrome.txt ends with a slash and the file itself ends with a blank line.
    On Windows, file URLs are of the form file:///C:/files/app/, where C is the drive letter.
    Make sure the contents.rdf file is in the right directory and is well-formed. Open the contents.rdf file in Mozilla to see if it parses as well-formed XML. If not, you will see an error on a yellow background.
    If you are using a debug build of Mozilla, some info will be printed to the terminal when starting up indicating what chrome applications are being checked. Check if your application is listed.
    The error message "XML Parsing Error: undefined entity" in your XUL file can be caused by an error in the manifest or in the jar file referenced by the manifest. For example, in <!DOCTYPE window SYSTEM "chrome://fireclipse/locale/fireclipse.dtd"> the dtd file must exist and its directory must be correctly addressed in the "locale" manifest or XML parsing will fail.

For more information about manifest files, see Chrome Registration.

In the next section, we will start looking into the XUL language.

We're going to be creating a simple find files utility throughout this tutorial.

First, however, we should look at the basic syntax of a XUL file.
Creating a XUL File

An XUL file can be given any name but it really should have a .xul extension. The simplest XUL file has the following structure:

<?xml version="1.0"?>
<?xml-stylesheet href="chrome://global/skin/" type="text/css"?>

<window
    id="findfile-window"
    title="Find Files"
    orient="horizontal"
    xmlns="http://www.mozilla.org/keymaster/gatekeeper/there.is.only.xul">
<!-- Other elements go here --> 
</window>

This window will not do anything since it doesn't contain any UI elements. Those will be added in the next section. In fact, there will not be shown any window at all. If you want to force the window to become visible you can add the width and height attribute to the window tag. Here is a line by line breakdown of the code above:

    <?xml version="1.0"?>
    This line simply declares that this is an XML file. You would normally add this line as is at the top of each xul file.
    <?xml-stylesheet href="chrome://global/skin/" type="text/css"?>
    This line is used to specify the style sheets to use for the file. This is the syntax that XML files use to import style sheets. In this case, we import the styles found in the global part of a skin package. We didn't specify a specific file so Mozilla will determine which files in the directory to use. In this case, the all-important global.css file is selected. This file contains all the default declarations for all of the XUL elements. Because XML does not have any knowledge of how elements should be displayed, the file indicates how. Generally, you will put this line at the top of every XUL file. You can also import other style sheets using a similar syntax. Note that you would normally import the global style sheet from within your own style sheet file.
    <window
    This line declares that you are describing a window. Each user interface window is described in a separate file. This tag is much like the HTML tag which surrounds an entire HTML document, except that a user interface window is described instead of a document. Several attributes can be placed in the window tag -- in this case there are four. In the example, each attribute is placed on a separate line but they do not have to be.
    id="findfile-window"
    The id attribute is used as an identifier so that the window can be referred to by scripts. You will usually put an id attribute on all elements. The name can be anything you want although it should be something relevant.
    title="Find Files"
    The title attribute describes the text that would appear on the title bar of the window when it is displayed. In this case the text 'Find Files' will appear.
    orient="horizontal"
    The orient attribute specifies the arrangement of the items in the window. The value horizontal indicates that items should be placed horizontally across the window. You may also use the value vertical, which means that the items are placed in a column. This is the default value, so you may leave the attribute off entirely if you wish to have vertical orientation.
    xmlns="http://www.mozilla.org/keymaster/gatekeeper/there.is.only.xul">
    This line declares the namespace for XUL, which you should put on the window element to indicate that all of its children are XUL. Note that this URL is never actually downloaded. Mozilla will recognize this URL internally.
    <!-- Other elements go here -->
    Replace this comment block with other elements (the buttons, menus and other user interface components) to appear in the window. We'll add some of these in the next set of sections.
    </window>
    And finally, we need to close the window tag at the end of the file.

Opening a Window

In order to open a XUL window, there are several methods that can be used. If you are only in the development stage, you can just type the URL (whether a chrome:, file: or other URL type) into the location bar in a Mozilla browser window. You should also be able to double-click the file in your file manager, assuming that XUL files are associated with Mozilla. The XUL window will appear in the browser window as opposed to a new window, but this is often sufficient during the early stages of development.

Note: The local XUL document can be opened in the browser as mentioned above only if |dom.allow_XUL_XBL_for_file| preference in "about:config" has been set to |true| . If it does not exist, then it needs to be created and set to |true|.

The correct way, of course, is to open the window using JavaScript. No new syntax is necessary as you can use the window.open() function as one can do for HTML documents. However, one additional flag, called 'chrome' is necessary to indicate to the browser that this is a chrome document to open. This will open the window without the toolbars and menus and so forth that a normal browser window has. The syntax is described below:

window.open(url,windowname,flags);

where the flags contains the flag "chrome" as in this example

window.open("chrome://navigator/content/navigator.xul", "bmarks", "chrome,width=600,height=300");

If you are using Firefox, try below:

window.open("chrome://browser/content/places/places.xul", "bmarks", "chrome,width=600,height=300");

You can test lines of JavaScript like these in the Error Console. Choose Tools – Error Console, type a line of JavaScript, and press the Evaluate button, or the Return or Enter key.
The findfile.xul example

Let's begin by creating the basic file for the find file dialog. Create a file called findfile.xul and put it in the content directory specified in the findfile.manifest file (we've created in the previous section). Add the XUL template shown at the top of this page to the file and save it.

You can use the command-line parameter '-chrome' to specify the XUL file to open when Mozilla starts. If this is not specified, the default window will open. (Usually the browser window.) For example, we could open the find files dialog with either of the following:

mozilla -chrome chrome://findfile/content/findfile.xul 
 
mozilla -chrome resource:/chrome/findfile/content/findfile.xul

If you run this command from a command-line (assuming you have one on your platform), the find files dialog will open by default instead of the Mozilla browser window. Of course, because we haven't put any UI elements in the window, you won't see a window appear. We'll add some elements in the next section.

To see the effect though, the following will open the bookmarks window:

mozilla -chrome chrome://communicator/content/bookmarks/bookmarksManager.xul

If you are using Firefox, try below.

firefox -chrome chrome://browser/content/places/places.xul

The '-chrome' argument doesn't give the file any additional privileges. Instead, it causes the specified file to open as a top-level window without any browser chrome, such as the address field or menu. Only chrome URLs have additional privileges.
The Extension Developer's Extension contains an XUL editor that allows you to type in XUL code and see the results in real-time from within Mozilla!
Troubleshooting

    If the XUL window fails to show up on the desktop but has an icon on the desktop tool bar, check the xml-stylesheet declaration. Make sure that you have included the stylesheet correctly:

<?xml-stylesheet href="chrome://global/skin/" type="text/css"?>

In the next section, we will add some buttons to the window.

« Previous
Next »
Document Tags and Contributors
Tags: 

    Tutorials XUL XUL_Tutorial 

Contributors to this page: stephaniehobson, 74h7k3fg, Sheppy, abhishekp, teoli, trevorh, fgn_develop, ianstone, Aangelo, fscholz, Rod Whiteley, jhammel, Natch, Kennykaiyinyu, Chbok, Nathymig, Enn, Stalepretzel, Chris Chittleborough, JohnJBarton1, Mgjbot, Morishoji, Pmash, Laolu, Takenbot, Ptak82, Nickolay, Callek, Napolj2, Dria
Last updated by: stephaniehobson, Oct 21, 2015, 10:32:42 AM
Learn the best of web development

Get the latest and greatest from MDN delivered straight to your inbox.
E-mail
MDN Web Docs

    Web Technologies
    Learn Web Development
    About MDN
    Feedback

Mozilla

    About
    Contact Us
    Firefox

Other languages:

    Terms Privacy Cookies 

© 2005-2018 Mozilla and individual contributors.

Content is available under these licenses.


    Skip to main content
    Select language
    Skip to search

MDN Web Docs

    Technologies References & Guides Feedback 

    Sign in

Search

Edit

Adding Buttons

« Previous
Next »

In this section, we will look at how to add some simple buttons to a window.
Adding Buttons to a Window

The window we've created so far has had nothing in it, so it isn't very interesting yet. In this section, we will add two buttons, a Find button and a Cancel button. We will also learn a simple way to position them on the window.

Like HTML, XUL has a number of tags that can be used to create user interface elements. The most basic of these is the button tag. This element is used to create simple buttons.

The button element has two main properties associated with it, a label and an image. You need one or the other or both. Thus, a button can have a label only, an image only or both a label and an image. Buttons are commonly used for the OK and Cancel buttons in a dialog, for example.
Syntax of buttons

The button tag has the following syntax:

<button
    id="identifier"
    class="dialog"
    label="OK"
    image="images/image.jpg"
    disabled="true"
    accesskey="t"/>

The attributes are as follows, all of which are optional:

id 
    A unique identifier so that you can identify the button with. You'll see this attribute on all elements. You'll want to use this if you want to refer to the button in a style sheet or script. However, you should add this attribute to almost all elements. It isn't always placed on elements in this tutorial for simplicity.
class 
    The style class of the button. This works the same as in HTML. It is used to indicate the style that the button appears in. In this case the value dialog is used. In most cases, you will not use a class for a button.
label 
    The label that will appear on the button. For example, OK or Cancel. If this is left out, no text appears.
image 
    The URL of the image to appear on the button. If this is attribute is left out, no image appears. You can also specify the image in a stylesheet using the list-style-image property.
disabled 
    If this attribute is set to true, the button is disabled. This is usually drawn with the text in grey. If the button is disabled, the function of the button cannot be performed. If this attribute is left out entirely, the button is enabled. You can switch the disabled state of the button using JavaScript.
accesskey 
    This should be set to a letter that is used as a shortcut key. This letter should appear in the label text and will typically be drawn underlined. When the user presses ALT (or a similar key that varies on each platform) and the access key, the button will be focused from anywhere in the window.

Note: Buttons support more attributes than those listed above. Others will be discussed later.
Some examples of buttons

Example 1 : Source View
Image:buttons1.png

<button label="Normal"/>
<button label="Disabled" disabled="true"/>

The examples above will generate the buttons in the image. The first button is a normal button. The second button is disabled so it appears greyed out.

We'll start by creating a simple Find button for the find files utility. The example code below shows how to do this.

<button id="find-button" label="Find"/>

Note: Firefox does not allow you to open chrome windows from web pages, so the View links in the tutorial will open in normal browser windows. Due to this, the buttons will appear to stretch across the window. You can add align="start" to the window tag to prevent the stretching.
The findfile.xul example

Let's add this code to the file findfile.xul that we created in the previous section. The code needs to be inserted in-between the window tags. The code to add is shown in red below:

<?xml version="1.0"?>
<?xml-stylesheet href="chrome://global/skin/" type="text/css"?>
<window id="findfile-window"
        title="Find Files"
        orient="horizontal"
        xmlns="http://www.mozilla.org/keymaster/gatekeeper/there.is.only.xul">

  <button id="find-button" label="Find"/>
  <button id="cancel-button" label="Cancel"/>

</window>

Image:buttons2.png

You'll notice that the Cancel button was added also. The window has been given a horizontal orientation so that the two buttons appear beside each other. If you open the file in Mozilla, you should get something like the image shown here.
Note: You should not put text labels directly in the XUL file. You should use entities instead so that text can be easily translated.

In the next section, we will find out how to add labels and images to a XUL window.
See also

    More button features

« Previous
Next »
Document Tags and Contributors
Tags: 

    Tutorials XUL XUL_Tutorial 

Contributors to this page: SphinxKnight, Sheppy, achronop, teoli, trevorh, zaardvark, Mikes, Chbok, James0, Mgjbot, Morishoji, JPEG, Pmash, Ptak82, Takenbot, Enn, Dria
Last updated by: SphinxKnight, Feb 9, 2018, 11:27:28 AM
Learn the best of web development

Get the latest and greatest from MDN delivered straight to your inbox.
E-mail
MDN Web Docs

    Web Technologies
    Learn Web Development
    About MDN
    Feedback

Mozilla

    About
    Contact Us
    Firefox

Other languages:

    Terms Privacy Cookies 

© 2005-2018 Mozilla and individual contributors.

Content is available under these licenses.


    Skip to main content
    Select language
    Skip to search

MDN Web Docs

    Technologies References & Guides Feedback 

    Sign in

Search

Edit

Adding Labels and Images

« Previous
Next »

This section describes a way to add labels and images to a window. In addition, we look at how to include elements into groups.
Text Elements

You cannot embed text directly into a XUL file without tags around it and expect it to be displayed. You can use two XUL elements for this purpose.
Label Element

The most basic way to include text in a window is to use the label element. It should be used when you want to place a descriptive label beside a control, such as a button. An example is shown below:

Example 1 : Source View

<label value="This is some text"/>

The value attribute can be used to specify the text that you wish to have displayed. The text will not wrap, so the text will all be displayed on a single line. This syntax is the most common of labels.

If the text needs to wrap, you can place the text content inside opening and closing tags as in the following example:

Example 2 :

<label>This is some longer text that will wrap onto several lines.</label>

As with HTML, line breaks and extra whitespace are collapsed into a single space. Later, we'll find out how to constrain the width of elements so that we can see the wrapping more easily.
Control Attribute

You can use the control attribute to set which control the label is associated with. When the user clicks on an associated label, that control will be focused. This association is also important for accessibility, so that screen readers read aloud the label for the control as the user tabs to it. Set the value of the control attribute to the id of the element to be focused.

Example 3 : Source View

<label value="Click here:" control="open-button"/>
<button id="open-button" label="Open"/>

In the example above, clicking the label will cause the button to be focused.
Description Element

For descriptive text not associated with any particular control, you can use the description tag. This element is useful for informative text at the top of a dialog or a group of controls for example. As with the label element, you can either use the value attribute for a single line of text or place text or XHTML content inside opening and closing description tags for longer blocks of text. It is more common to use the attribute syntax for labels, and the text content syntax for descriptions.

Example 4 : Source View

<description>
  This longer section of text is displayed.
</description>

You can set the text via script using the textContent property, as in the following example:

<description id="text" width="200"/>

document.getElementById('text').textContent = "Some lengthy word wrapped text goes here.";

Internally, both the label element and the description elements are the same. The label element is intended for labels of controls, such as text fields. The control attribute is only supported for labels. The description element is intended for other descriptive text such as informative text at the top of a dialog box.
Images

XUL has an element to display images within a window. This element is appropriately named image. Note that the tag name is different than HTML (image instead of img). You can use the src attribute to specify the URL of the image file. The example below shows this:

<image src="images/banner.jpg"/>

Although you can use this syntax, it would be better in order to support different themes to use a style sheet to set the image URL. A later section will describe how to use style sheets, but an example will be shown here for completeness. You can use the list-style-image CSS property to set the URL for the image. Here are some examples:

XUL:
 <image id="image1"/>
 <image id="search"/>

Style Sheet:
 #image1 {
   list-style-image: url("chrome://findfile/skin/banner.jpg");
 }

 #search {
   list-style-image: url("http://example.com/images/search.png");
 }

These images come from within the chrome directory, in the skin for the findfile package. Because images vary by theme, you would usually place images in the skin directory.

In the next section, we will learn how to add some input controls to a window.

« Previous
Next »
Document Tags and Contributors
Tags: 

    Tutorials XUL XUL_Tutorial 

Contributors to this page: Sheppy, teoli, trevorh, jake33, Neil, Enn, LeslieM, Tsahi, Chbok, Morishoji, Aaronlev, Mgjbot, Pmash, Takenbot, Kakurady, Ptak82, Dria
Last updated by: Sheppy, Apr 14, 2014, 10:34:55 AM
Learn the best of web development

Get the latest and greatest from MDN delivered straight to your inbox.
E-mail
MDN Web Docs

    Web Technologies
    Learn Web Development
    About MDN
    Feedback

Mozilla

    About
    Contact Us
    Firefox

Other languages:

    Terms Privacy Cookies 

© 2005-2018 Mozilla and individual contributors.

Content is available under these licenses.


    Skip to main content
    Select language
    Skip to search

MDN Web Docs

    Technologies References & Guides Feedback 

    Sign in

Search

Edit

Input Controls

« Previous
Next »

XUL has elements that are similar to the HTML form controls.

Looking for a guide to using input controls and forms on the Web? See Forms in HTML.
Text Entry Fields

HTML has an input element which can be used for text entry controls. XUL has a similar element, textbox, used for text entry fields. Without any attributes, the textbox element creates a box in which the user can enter text. Textboxes accept many of the same attributes as HTML input controls. The following are some of them:

id 
    A unique identifier so that you can identify the textbox.
class 
    The style class of the textbox.
value 
    If you want the textbox to have default text, supply it with the value attribute.
disabled 
    Set to true to have text entry disabled.
textbox.type 
    You can set this attribute to the special value password to create a textbox that hides what it types. This is usually used for password entry fields.
maxlength 
    The maximum number of characters that the textbox allows.

Note that while in HTML, several different kinds of fields can be created with the input element, in XUL there are separate elements for each type.

The following example shows some textboxes:

Example 1 : Source View

<label control="some-text" value="Enter some text"/>
<textbox id="some-text"/>
<label control="some-password" value="Enter a password"/>
<textbox id="some-password" type="password" maxlength="8"/>

Multiline textbox

The textbox examples above will create text inputs that can only be used for entering one line of text. HTML also has a textarea element for creating a larger text entry area. In XUL, you can use the textbox element for this purpose as well -- two separate elements are not necessary. If you set the multiline attribute to true, the text entry field will display multiple rows.

Example 2 : Source View

<textbox multiline="true"
         value="This is some text that could wrap onto multiple lines."/>

Like the HTML textarea, you can use the rows and cols attributes to set the size. This should be set to the number of rows and columns of characters to display.
Our find files example

Let's add a search entry field to the find file dialog. We'll use the textbox element.

<label value="Search for:" control="find-text"/>
<textbox id="find-text"/>

<button id="find-button" label="Find"/>

Image:inputs1.png

Add these lines before the buttons we created in the last section. If you open this window, you will see something much like that shown in the image.

Notice that the label and the text input field have now appeared in the window. The textbox is fully functional and you can type into it and select text. Note that the control attribute has been used so that the textbox is selected when the label is clicked.
Checkboxes and Radio Buttons

Two additional elements are used for creating check boxes and radio buttons. They are variations of buttons. The checkbox element is used for options that can be enabled or disabled. Radio buttons can be used for a similar purpose when there are a set of them and only one can be selected at once.

You can use most of the same attributes on checkboxes and radio buttons as with buttons. The example below shows some simple checkboxes and radio buttons.

<checkbox id="case-sensitive" checked="true" label="Case sensitive"/>
<radio id="orange" label="Orange"/>
<radio id="violet" selected="true" label="Violet"/>
<radio id="yellow" label="Yellow"/>

The first line creates a simple checkbox. When the user clicks the checkbox, it switches between checked and unchecked. The checked attribute can be used to indicate the default state. You should set this to either true or false. The label attribute can be used to assign a label that will appear beside the check box. For radio buttons, you should use the selected attribute instead of the checked attribute. Set it to true to have a radio button selected by default, or leave it out for other radio buttons.
Radiogroup element

In order to group radio buttons together, you need to use the radiogroup element. Only one of the radio buttons in a radio group can be selected at once. Clicking one will turn off all of the others in the same group. The following example demonstrates this.

Example 3 : Source View

<radiogroup>
  <radio id="orange" label="Orange"/>
  <radio id="violet" selected="true" label="Violet"/>
  <radio id="yellow" label="Yellow"/>
</radiogroup>

Attributes

Like buttons, check boxes and radio buttons are made up of a label and an image, where the image switches between checked and unchecked when it is pressed. Check boxes have many of the same attributes as buttons:

label 
    The label on the check box or radio button.
disabled 
    Set this to either true or false to disable or enable the check box or radio button.
accesskey 
    The shortcut key that can be used to select the element. The letter specified is usually drawn underlined in the label.
to view XUL, you need e.g. seamonkey or waterfox and remote-xul-manager from https://github.com/jvillalobos/Remote-XUL-Manager

Find files example so far : Source View

In the next section, we will look at some elements for entering and selecting numbers.

« Previous
Next »

Looking for a guide to using input controls and forms on the Web? See Forms in HTML.
Document Tags and Contributors
Tags: 

    Tutorials XUL XUL_Tutorial 

Contributors to this page: KM-200, Sheppy, teoli, trevorh, selusi, tessarakt2, hobophobe, jonmmorgan, Chbok, James0, Mgjbot, Enn, Morishoji, JPEG, Pmash, Ptak82, Takenbot, Dria
Last updated by: KM-200, May 8, 2018, 2:41:57 PM
Learn the best of web development

Get the latest and greatest from MDN delivered straight to your inbox.
E-mail
MDN Web Docs

    Web Technologies
    Learn Web Development
    About MDN
    Feedback

Mozilla

    About
    Contact Us
    Firefox

Other languages:

    Terms Privacy Cookies 

© 2005-2018 Mozilla and individual contributors.

Content is available under these licenses.


    Skip to main content
    Select language
    Skip to search

MDN Web Docs

    Technologies References & Guides Feedback 

    Sign in

Search

Edit

Numeric Controls

« Previous
Next »

XUL has two elements used for the entry of numeric values or ranges, and well as two elements for entering dates and times. These elements are only available in Firefox 3 and later.
Number Fields

A textbox may be used for entering numbers by setting the value of the type attribute to the value number. This type of textbox may only be used to enter numbers. Other characters are not allowed and are just ignored if typed. In addition, arrow buttons appear beside the textbox to allow the user to cycle through the values.
Image:Controlguide-textbox-number.gif

As with other textboxes, the default value can be specified with the value attribute. Naturally, this value should be a number. However, the minimum and maximum values can also be specified using the min and max attributes. If these are set, you can control the range of values that the textbox may be set to. If the user enters a value less or greater than this value, it will be reset to the minimum or maximum value as necessary. For instance, the following numeric textbox has a range between 1 and 20.

<textbox type="number" min="1" max="20"/>

As the default value isn't specified, it will default to 1, the minimum value. The min attribute is set to 1 to indicate a minimum possible value of 1 and the max attribute is set to 20 to indiciate a maximum possible value of 20. If the minimum value is not specified, it has a default value of 0. The maximum value defaults to the special value Infinity which means that there is no limit.
Other numeric textbox attributes

The increment attribute may be used to specify by how much the value changes when the arrows are pressed. The default value is 1, but specifying a different value allows the number to change by a larger amount. For instance, the following example steps in multiples of 10.

<textbox type="number" increment="10" max="100"/>

This textbox steps in multiples of 10 from 0 to 100. Since the min attribute was not specified, it defaults to 0. Note that the user can still enter other values if they are typed in. The increment attribute only affects the arrow buttons. The user may also increment or decrement the value using this increment by using the up and down cursor keys while the textbox is focused.

The decimalplaces attribute indicates how many decimal places to show. The default value is 0, which means show integers only. However a different value may be used to show decimal values.

<textbox type="number" decimalplaces="2"/>

In this example, two digits right of the decimal point are shown. Values with additional fractional digits are rounded to two digits.
Scales

A scale element may also be used to select from a range of values. Instead of a textbox however, a sliding scale is used. The user may drag the thumb of the scale to adjust the value.
Image:Controlguide-scale.gif

Many of the same attributes as a numeric textbox may be used with a scale: value, min, max and increment may all be used in a similar fashion. The scale does not actually show the value as a number, but it may be used in a script. A scale will fire a change event whenever the scales's value is modified.

<scale value="40" min="1" max="50"/>

This scale defaults to a value of 40 and has a range between 1 and 50.

A numeric textbox would normally be used when the value was important to the user, for instance, a field to enter a number of days, or the maximum size of a file. A scale would be used when the actual value isn't important, just that sliding the scale decreases or increases a state. For instance, a volume slider or a zoom level.

The default arrangement of a scale is horizontal with lower values to the left and higher values to the right. However, you can change this orientation with the orient and dir attributes.

<scale orient="vertical" dir="reverse"/>

This scale will be shown vertical with lower values at the bottom and higher values at the top.
Date and Time Entry Fields

The datepicker and timepicker elements may be used to allow the user to enter dates and times. When used, they display a set of numeric textboxes to enter each of the components of the date or time.

<datepicker value="2004-03-24"/>
<timepicker value="15:30:00"/>

Image:Controlguide-timepicker.gif

The value attribute is used to set the default value; if this attribute is omitted, the field will be initially set to the current date or time. The format of the attribute is exactly as above, that is dates are of the form YYYY/MM/DD and times are of the form HH:MM:SS (although the seconds and the accompanying colon may be omitted).

These two elements ensure that the user enters a value date or time. This way, you do not have to check for valid dates, ensure that the day isn't greater than the number of days in the month, handle leap years, and so forth.

While the timepicker only comes is one style, the datepicker has three different variations. The default style shows three fields for entering the year, month and date. The type attribute may be used to select the other two. Using a value of grid uses a calendar grid, as shown in the image below.

Image:Controlsguide-datepicker-grid.png

You can also use the value popup which creates a combination of the two types. This type has three fields for entering the year, month and date, as well as a dropdown button for displaying a popup calendar grid for selecting a day.

<datepicker type="popup"/>

« Previous
Next »
Document Tags and Contributors
Tags: 

    Firefox 3 Tutorials XUL XUL_Tutorial 

Contributors to this page: Sheppy, teoli, trevorh, 宋晓光, Zguneri, Zachary8222, Mgjbot, Chbok, MarkFinkle, Enn
Last updated by: Sheppy, Apr 14, 2014, 10:34:54 AM
Learn the best of web development

Get the latest and greatest from MDN delivered straight to your inbox.
E-mail
MDN Web Docs

    Web Technologies
    Learn Web Development
    About MDN
    Feedback

Mozilla

    About
    Contact Us
    Firefox

Other languages:

    Terms Privacy Cookies 

© 2005-2018 Mozilla and individual contributors.

Content is available under these licenses.


    Skip to main content
    Select language
    Skip to search

MDN Web Docs

    Technologies References & Guides Feedback 

    Sign in

Search

Edit

List Controls
Jump to:

    List BoxesMulti-Column List BoxesHeader RowsDrop-down Lists 

« Previous
Next »

XUL has a number of types of elements for creating list boxes.
List Boxes

A list box is used to display a number of items in a list. The user may select an item from the list.

XUL provides two types of elements to create lists, a listbox element to create multi-row list boxes, and a menulist element to create drop-down list boxes. They work similar to the HTML select element, which performs both functions, but the XUL elements have additional features.

The simplest list box uses the listbox element for the box itself, and the listitem element for each item. For example, this list box will have four rows, one for each item.

Example 1 : Source View

<listbox>
  <listitem label="Butter Pecan" />
  <listitem label="Chocolate Chip" />
  <listitem label="Raspberry Ripple" />
  <listitem label="Squash Swirl" />
</listbox>

Like with the HTML option element, you can assign a value for each item using the value attribute. You can then use the value in a script. The list box will default to a suitable size, but you can control the size with the rows attribute. Set it to the number of rows to display in the list box. A scroll bar will appear that the user can use to display the additional rows.

The following example demonstrates these additional features:

Example 2 : Source View

<listbox rows="3">
  <listitem label="Butter Pecan" value="bpecan" />
  <listitem label="Chocolate Chip" value="chocchip" />
  <listitem label="Raspberry Ripple" value="raspripple" />
  <listitem label="Squash Swirl" value="squash" />
</listbox>

The example has been changed to display only 3 rows at a time. Values have also been added to each item in the list. List boxes have some additional features, which will be described later.
Multi-Column List Boxes

The listbox also supports multiple columns. Each cell may have arbitrary content within it, although usually only text is used. When the user selects an item in the list, the entire row is selected. You cannot have a single cell selected.

Two tags are used to specify the columns in the listbox. The listcols element is used to hold the column information, each of which is specified using a listcol element. You will need one listcol element for each column in the listbox.

The listcell element may be used for each cell in a row. If you want to have three columns, you will need to add three listcell elements inside each listitem. To specify the text content of a cell, place a label attribute on a listcell. For the simple case where there is only one column, you may also place the label attributes directly on the listitem elements and leave the listcell elements out entirely, as was seen in the earlier listbox examples.

The following example is of a listbox with two columns and three rows:

Example 3 : Source View

<listbox>
  <listcols>
    <listcol/>
    <listcol/>
  </listcols>
  <listitem>
    <listcell label="George" />
    <listcell label="House Painter" />
  </listitem>
  <listitem>
    <listcell label="Mary Ellen" />
    <listcell label="Candle Maker" />
  </listitem>
  <listitem>
    <listcell label="Roger" />
    <listcell label="Swashbuckler" />
  </listitem>
</listbox>

Header Rows

List boxes also allow a special header row to be used. This is much like a regular row except that it is displayed differently. You would use this to create column headers. Two new elements are used.

The listhead element is used for the header rows, just as the listitem element is used for regular rows. The header row is not a normal row however, so using a script to get the first row in the list box will skip the header row.

The listheader element is used for each cell in the header row. Use the label attribute to set the label for the header cell.

Here is the earlier example with a header row:

Example 4 : Source View

<listbox>

  <listhead>
    <listheader label="Name" />
    <listheader label="Occupation" />
  </listhead>

  <listcols>
    <listcol/>
    <listcol flex="1" />
  </listcols>

  <listitem>
    <listcell label="George" />
    <listcell label="House Painter" />
  </listitem>
  <listitem>
    <listcell label="Mary Ellen" />
    <listcell label="Candle Maker" />
  </listitem>
  <listitem>
    <listcell label="Roger" />
    <listcell label="Swashbuckler" />
  </listitem>

</listbox>

In this example, the flex attribute is used to make the column flexible. This attribute will be described in a later section, but here it allows the column to fill all of the remaining space horizontally. You can resize the window to see that the column stretches as the window does. If you shrink the size horizontally, the labels on the cells will crop themselves automatically using an ellipsis. You can use the crop attribute on the cells or items set to the value none to disable the ellipsis.
Drop-down Lists

Drop-down lists can be created in HTML using the select element. The user can see a single choice in a textbox and may click the arrow or some similar such button next to the textbox to make a different selection. The other choices will appear in a pop-up window. XUL has a menulist element which can be used for this purpose. It is made from a textbox with a button beside it. Its name was chosen because it pops up a menu with the choices in it.

Three elements are needed to describe a drop-down box. The first is the menulist element, which creates the textbox with the button beside it. The second, menupopup, creates the popup window which appears when the button is clicked. The third, menuitem, creates the individual choices.

Its syntax is best described with the example below:

Example 5 : Source View

<menulist label="Bus">
  <menupopup>
    <menuitem label="Car" />
    <menuitem label="Taxi" />
    <menuitem label="Bus" selected="true" />
    <menuitem label="Train" />
  </menupopup>
</menulist>

This menulist will contain four choices, one for each menuitem element. To show the choices, click the arrow button on the menulist. When one is selected, it appears as the choice in the menulist. The selected attribute is used to indicate the value that is selected by default.
Editable menulist

By default, you can only select choices from the list. You cannot enter your own text by typing it in. A variant of the menulist allows editing the text in the field. For example, the URL field in the browser has a drop-down for selecting previously typed URLs, but you can also type them in yourself.

To create an editable menulist, add the editable attribute as follows:

Example 6 : Source View

<menulist editable="true">
  <menupopup>
    <menuitem label="www.mozilla.org" />
    <menuitem label="www.xulplanet.com" />
    <menuitem label="www.dmoz.org" />
  </menupopup>
</menulist>

The URL field created here has three pre-populated choices that the user can select or they can enter one of their own by typing it into the field. The text the user enters is not added as a new choice. Because the label attribute was not used in this example, the default value will be blank.

In the next section, we'll learn about creating progress meters.

« Previous
Next »
Document Tags and Contributors
Tags: 

    Tutorials XUL XUL_Tutorial 

Contributors to this page: Sheppy, ethertank, teoli, trevorh, 宋晓光, Enn, Chbok, Mgjbot, Morishoji, JPEG, Pmash, Takenbot, Ptak82, Dria
Last updated by: Sheppy, Apr 14, 2014, 10:34:54 AM
Learn the best of web development

Get the latest and greatest from MDN delivered straight to your inbox.
E-mail
MDN Web Docs

    Web Technologies
    Learn Web Development
    About MDN
    Feedback

Mozilla

    About
    Contact Us
    Firefox

Other languages:

    Terms Privacy Cookies 

© 2005-2018 Mozilla and individual contributors.

Content is available under these licenses.


    Skip to main content
    Select language
    Skip to search

MDN Web Docs

    Technologies References & Guides Feedback 

    Sign in

Search

Edit

Progress Meters

« Previous
Next »

In this section, we'll look at creating progress meters.
Adding a Progress Meter

A progress meter is a bar that indicates how much of a task has been completed. You typically see it when downloading files or when performing a lengthy operation. XUL has a progressmeter element which can be used to create these. There are two types of progress meters: determinate and indeterminate.

Determinate progress meters are used when you know the length of time that an operation will take. The progress meter will fill up and, once full, the operation should be finished. This can be used for the download file dialog as the size of the file is known.

Indeterminate progress meters are used when you do not know the length of time of an operation. The progress meter will have an animation such as a spinning barber pole or a sliding box, depending on the platform and theme used.

Determinate progress meter: Image:prog-det.png

Indeterminate progress meter: Image:prog-udet.png

The progress meter has the following syntax:

<progressmeter
    id="identifier"
    mode="determined"
    value="50"/>

The attributes are as follows:

id
    The unique identifer of the progress meter
mode
    The type of the progress meter. If this is set to determined, the progress meter is a determinate progress meter where it fills up as the task completes. If this is set to undetermined, the progress meter is indeterminate where you do not know the length of time. The value determined is the default if you do not specify this attribute.
value
    The current value of the progress meter. You should only use this for a progress meter that is determinate. The value should be set to an integer percentage from 0 to 100. The value would be changed by a script as the task completes.

The find files example

Let's add a progress meter to our find file dialog. We would normally put an indeterminate progress meter as we don't know how many files we'll be searching or how long the search will take. However, we'll add a normal one for now as an animating one can be distracting during development. The progress meter would normally only appear while the search is taking place. We'll add a script later to turn it on and off.

<textbox id="find-text"/>

<progressmeter value="50" style="margin: 4px;"/>

<button id="find-button" label="Find" default="true"/>

The value has been set to 50% so that we can see the meter on the window. A margin has been set to 4 pixels so that it is separated from the edge of the window. As was stated earlier, we only want the progress bar to be displayed while the search was occurring. A script will show and hide it as necessary.


The example so far. Source View

Image:progress1.png

 

In the next section, we will learn how to add additional elements to the window using HTML.

« Previous
Next »
Document Tags and Contributors
Tags: 

    Tutorials XUL XUL_Tutorial 

Contributors to this page: SphinxKnight, Sheppy, teoli, trevorh, Brdude, Chbok, MarkFinkle, Morishoji, Ptak82, Mgjbot, Pmash, Takenbot, Zuroc, Dria, Trio
Last updated by: SphinxKnight, Feb 9, 2018, 10:53:52 AM
Learn the best of web development

Get the latest and greatest from MDN delivered straight to your inbox.
E-mail
MDN Web Docs

    Web Technologies
    Learn Web Development
    About MDN
    Feedback

Mozilla

    About
    Contact Us
    Firefox

Other languages:

    Terms Privacy Cookies 

© 2005-2018 Mozilla and individual contributors.

Content is available under these licenses.


    Skip to main content
    Select language
    Skip to search

MDN Web Docs

    Technologies References & Guides Feedback 

    Sign in

Search

Edit

Adding HTML Elements

« Previous
Next »

Now that we've added some buttons, let's add some other elements.
Adding HTML Elements to a Window

In addition to all of the XUL elements that are available, you can also add HTML elements directly within a XUL file. You can actually use any HTML element in a XUL file, meaning that Java applets and tables can be placed in a window. You should avoid using HTML elements in XUL files if you can. (There are some reasons, and the main one concerns the control of the layout described later). However, this section will describe how to use them anyway. Remember that XML is case-sensitive though, so you'll have to enter the tags and attributes in lowercase.
XHTML namespace

In order to use HTML elements in a XUL file, you must declare that you are doing so using the XHTML namespace. This way, Mozilla can distinguish the HTML tags from the XUL ones. The attribute below should be added to the window tag of the XUL file, or to the outermost HTML element.

xmlns:html="http://www.w3.org/1999/xhtml"

This is a declaration of HTML much like the one we used to declare XUL. This must be entered exactly as shown or it won't work correctly. Note that Mozilla does not actually download this URL, but it does recognize it as being HTML.

Here is an example as it might be added to the find file window:

<?xml version="1.0"?>
 <?xml-stylesheet href="chrome://global/skin/" type="text/css"?>
 <window
   id="findfile-window"
   title="Find Files"
   orient="horizontal"
   xmlns:html="http://www.w3.org/1999/xhtml"
   xmlns="http://www.mozilla.org/keymaster/gatekeeper/there.is.only.xul">

Then, you can use HTML tags as you would normally, keeping in mind the following:

    You must add a html: prefix to the beginning of each tag, assuming you declared the HTML namespace as above.
    The tags must be entered in lowercase.
    "Quotes" must be placed around all attribute values.
    XML requires a trailing slash at the end of tags that have no content. This may be clearer from the examples below.

Using HTML elements

You can use any HTML tag although some such as head and body are not really useful. Some examples of using HTML elements are shown below.

<html:img src="banner.jpg"/>

<html:input type="checkbox" value="true"/>

<html:table>
  <html:tr>
    <html:td>
      A simple table
    </html:td>
  </html:tr>
</html:table>

These examples will create an image from the file banner.jpg, a checkbox and a single-cell table. You should always use XUL features if they are available and you probably should not use tables for layout in XUL. (There are XUL elements for doing layout). Notice that the prefix html: was added to the front of each tag. This is so that Mozilla knows that this is an HTML tag and not a XUL one. If you left out the html: part, the browser would think that the elements were XUL elements and they would not display because img, input, table, and so on are not valid XUL tags.

In XUL, you can add labels with the description or label element. You should use these elements when you can. You can also add labels to controls either by using the HTML label element, or you can simply put the text inside another HTML block element (such as p or div) as in the example below.

Example 1 : Source View

<html:p>
  Search for:
  <html:input id="find-text"/>
  <button id="okbutton" label="OK"/>
</html:p>

This code will cause the text 'Search for:' to be displayed, followed by an input element and an OK button. Notice that the XUL button can appear inside the HTML elements, as it does here. Plain text will only be displayed when placed inside an HTML element that would normally allow you to display text (such as a p tag). Text outside of one will not be displayed, unless the XUL element the text is inside allows this (the description element, for example). The examples below may help.
Examples of HTML elements

What follows are some examples of adding HTML elements to windows. In each case, the window and other common information has been left out for simplicity.
A dialog with a check box

Example 2 : Source View
Image:htmlelem-ex1.png

<html:p>
  Click the box below to remember this decision. 
  <html:p>
    <html:input id="rtd" type="checkbox"/>
    <html:label for="rtd">Remember This Decision</html:label>
  </html:p>
</html:p>

In this case, one p tag was used to place the text in and another was used to break apart the text into multiple lines.

 
Text outside of HTML blocks

Example 3 : Source View
Image:htmlelem-ex2.png

<html:div>
    Would you like to save the following documents?
    <html:hr/>
</html:div>   
Expense Report 1
What I Did Last Summer
<button id="yes" label="Yes"/>
<button id="no" label="No"/>

As can be seen in the image, the text inside the div tag was displayed but the other text (Expense Report 1 and What I Did Last Summer) was not. This is because there is no HTML or XUL element capable of displaying text enclosing it. To have this text appear, you would need to put it inside the div tag, or enclose the text in a description tag.
Note: Due to bug 554290, this text actually does render in Gecko 1.9.2 and later.
Invalid HTML elements

<html:po>Case 1</html:po>
<div>Case 2</div>
<html:description value="Case 3"/>

All three of the cases above will not display, each for a different reason.

Case 1
    po is not a valid HTML tag and Mozilla has no idea what to do with it.
Case 2
    div is valid but only in HTML. To get it to work, you will need to add the html: qualifier.
Case 3
    A description element is only valid in XUL and not in HTML. It should not have the html: qualifier.

Next, we will learn how to adding spacing between elements.

« Previous
Next »
Document Tags and Contributors
Tags: 

    Tutorials XUL XUL_Tutorial 

Contributors to this page: Sheppy, system_push, teoli, trevorh, alice0775, Nalhilal, Ahirreddy, Chbok, Morishoji, Ptak82, Mgjbot, Pmash, Takenbot, Dria
Last updated by: Sheppy, Apr 14, 2014, 10:34:54 AM
Learn the best of web development

Get the latest and greatest from MDN delivered straight to your inbox.
E-mail
MDN Web Docs

    Web Technologies
    Learn Web Development
    About MDN
    Feedback

Mozilla

    About
    Contact Us
    Firefox

Other languages:

    Terms Privacy Cookies 

© 2005-2018 Mozilla and individual contributors.

Content is available under these licenses.


    Skip to main content
    Select language
    Skip to search

MDN Web Docs

    Technologies References & Guides Feedback 

    Sign in

Search

Edit

Using Spacers

« Previous
Next »

In this section, we will find out how to add some spacing in between the elements we have created.
Adding Spacers

One of the problems with developing user interfaces is that each user has a different display. Some users may have larger displays with higher resolutions and others may have lower resolutions. In addition, different platforms may have special requirements on the user interface. If adding support for multiple languages, the text for one language may require more room than another.

Applications that need to support multiple platforms and languages usually have their windows laid out with lots of space to allow for this. Some platforms and user interface toolkits provide components that are smart enough to resize and re-position themselves to fit the needs of the user. (Java uses layout managers for example.)

XUL provides the capability for elements to position and resize automatically. As we've seen, the find files window has appeared in a size that will fit the elements inside it. Each time we add something, the window gets bigger.

XUL uses a layout system called the 'Box Model'. We'll talk more about this in the next section but it essentially allows you to divide a window into a series of boxes that hold elements. The boxes will be positioned and resized based on specifications that you can define. For now, just know that the window element is a type of box.

Before we get into detail about boxes, we'll introduce another XUL element that is useful for layout, the spacer. A spacer is very simple and only requires one attribute, which will be explained in a moment. The simplest spacer looks like the following:

<spacer flex="1"/>

A spacer is used to place blank space into a window. Its most useful ability is that it can grow or shrink as the user resizes the window. This would be how one would place buttons on the right or bottom of a window and have them stick to the right or bottom edge no matter what size the window is. As we'll see, you can use a series of spacers to create a number of layout effects.

In this syntax above, the spacer has one attribute, called flex. This is used to define the flexibility of the spacer. In the case above, the spacer has a flex of 1. This makes the spacer element stretchy. If you place a spacer directly inside a window, the spacer will grow in size when the size of the window is changed.

We'll add a spacer to our find file dialog soon. First, let's take a look at what happens when the current dialog is resized.

Image:springs1.jpg

If you change the size of the find files window, you can see that the elements have remained where they started. None of them have been moved or resized even though the window has more room in it. Let's see what happens when a spacer is added between the text box and the Find button.

Image:springs2.jpg

By adding a spacer and resizing the window, you can see that the spacer has expanded to fill the space. The buttons have been pushed over.
Our find file example

The code to add a spacer is shown below. Insert it just before the Find button.

<spacer flex="1"/>

<button id="find-button" label="Find"/>

More About Flexibility

XUL lays out elements on a window by calculating suitable widths and heights for the elements and then adding space where they are flexible. Unless you specify information about the width and height of an element, the default size of an element is determined by its contents. You'll notice that the Cancel button in the dialogs has always set its width so that it fits the text inside it. If you create a button with a very long label, the button's default size will be large enough to hold the entire label. Other elements, such as the text box have chosen a suitable default size.

The flex attribute is used to specify if an element can change size to fit the box (in this case, the window) it is in. We've already seen the flex attribute applied to spacers, but it can be applied to any element. For example, you might want to have the Find button resize instead.

Image:springs3.jpg

As you can see in the image, by placing a flex attribute on the Find button, it resizes when the window does. A spacer is really nothing special. It could really be considered a hidden button. It works much the same way as a button except it does not draw on screen.

You may have noticed something about the image above. Not only did the Find button grow in size but some space has appeared between the main label and the button. Of course, this is the spacer that we put in earlier. It has resized itself also. If you look closely enough, you should notice that the change in size has been divided up equally between the spacer and the button. The spacer has received half of the free space and the button has received the other half.

The reason we're seeing this effect is that both the spacer and the Find button have a flex attribute. Because both are flexible, both the button and the spacer resize equally.

What if you want to set one element to grow twice as large an another? You can use a higher number as the value of the flex attribute. The values of the flex element are a ratio. If one element has a flex of 1 and the next one has a flex of 2, the second one will grow at twice the rate of the first one. In effect, a flex of 2 says that this element has a flex that is two times the elements that have a flex of one.

The flex attribute isn't used to specify an actual size. Instead, it specifies how empty space it divided among the children of a container box. We'll look at boxes in the next section. Once the default sizes of the children of a box are determined, the flexibility values are used to divide up the remaining empty space in the box. For example, if a box is 200 pixels wide and contains two flexible buttons, the first 50 pixels and the other 90 pixels, there will be 60 pixels of space left over. If both buttons have a flex value of 1, the space will be divided evenly with 30 extra pixels of width going to each button. If the second button's flexibility was increased to 2, the first button would receive 20 pixels of the extra space and the second button would receive 40 pixels of extra space instead.

The flex attribute can be placed on any element, however it only has any meaning when placed on an element directly inside a XUL box. This means that even though you can place a flex on an HTML element, it will have no effect if that element is inside a non-box element.
Flex examples

Example 1:
  <button label="Find" flex="1"/>
  <button label="Cancel" flex="1"/>

Example 2:
  <button label="Find" flex="10"/>
  <button label="Cancel" flex="1"/>

Example 3:
  <button label="Find" flex="2"/>
  <button label="Replace"/>
  <button label="Cancel" flex="4"/>

Example 4:
  <button label="Find" flex="2"/>
  <button label="Replace" flex="2"/>
  <button label="Cancel" flex="3"/>

Example 5:
  <html:div>
    <button label="Find" flex="2"/>
    <button label="Replace" flex="2"/>
  </html:div>

Example 6:
  <button label="Find" flex="145"/>
  <button label="Replace" flex="145"/>

Example 1
    in this case the flexibility is divided up evenly between both buttons. Both buttons will change size evenly.
Example 2
    here, both buttons will grow, but the Find button will grow ten times as much as the Cancel button, because it has a flex value that is 10 times the flex value of the Find button. Available space will be divided into 10 parts for the Find button and one part for the Cancel button.
Example 3
    only two of the buttons are marked as flexible here. The Replace button will never change size but the other two will. The Cancel button will always resize twice as large as the Find button because its flex value is twice as large.
Example 4
    in this case, all three buttons are flexible. Both the Find and Replace buttons will be the same size but the Cancel button will be somewhat larger (50% larger to be exact).
Example 5
    here, the two buttons are placed inside a div element. Flexibility is meaningless here as the buttons are not directly in a box. The effect would be the same if the flex attributes were left out.
Example 6
    because the flex values are the same on both buttons, they will flex equally. This would work just as well with flex values of one instead of 145. There's no difference in this case. It is recommended that you use lower numbers for readability.

Note that other factors, such as the button labels and button minimum sizes, will affect the actual sizes of the buttons. For instance, a button won't shrink less than the space needed to fit its label.

Specifying a flex value of 0 has the same effect as leaving the flex attribute out entirely. It means that the element is not flexible at all. You may also sometimes see a flex value specified as a percentage. This has no special meaning and is treated as if the percent sign was not there.

You may have noticed that when you resize the find file dialog vertically, the buttons resize themselves to fit the height of the window. This is because all of the buttons have an implied vertical flex given to them by the window. In the next section we'll learn how to change this.
Find files example so far

Source View

Next, we will learn some additional features of buttons.

« Previous
Next »
Document Tags and Contributors
Tags: 

    Tutorials XUL XUL_Tutorial 

Contributors to this page: Sheppy, teoli, trevorh, leuqarte, Chbok, J05ef, Enn, Mgjbot, Pmash, SylvainPasche, Takenbot, Ptak82, Dria
Last updated by: Sheppy, Apr 14, 2014, 10:34:54 AM
Learn the best of web development

Get the latest and greatest from MDN delivered straight to your inbox.
E-mail
MDN Web Docs

    Web Technologies
    Learn Web Development
    About MDN
    Feedback

Mozilla

    About
    Contact Us
    Firefox

Other languages:

    Terms Privacy Cookies 

© 2005-2018 Mozilla and individual contributors.

Content is available under these licenses.


    Skip to main content
    Select language
    Skip to search

MDN Web Docs

    Technologies References & Guides Feedback 

    Sign in

Search

Edit

More Button Features

« Previous
Next »

In this section, we will look at some additional features of buttons.
Adding an Image

You can add an image to a button by specifying a URL in the image attribute. The image is loaded from the URL, which can be a relative or absolute URL, and then the image is displayed on the button.

The button below will have both a label and the image 'happy.png'. The image will appear to the left of the label. You can change this position by using two other attributes. This will be explained in a moment.

Example 1 : Source View

<button label="Help" image="happy.png"/>

Button with CSS image

Another way to specify the image is by using the CSS list-style-image style property on the button. This is designed to allow the 'skin' (in this case, the appearance of the image) to be changed without changing the XUL file. An example is shown below.

Example 2 : Source View

<button id="find-button"
  label="Find" style="list-style-image: url('happy.png')"/>

In this case, the image 'happy.png' is displayed on the button. The style attribute functions similar to its HTML counterpart. In general, it can be used on all XUL elements. Note that you really should put the style declarations in a separate style sheet.
Positioning the Images

By default, the image on a button will appear to the left of the text label. There are two attributes that can be used to control this position.

The dir attribute controls the direction of the image and text. By setting this attribute to the value reverse, the image will be placed on the right side of the text. By using the value normal, or leaving the attribute out entirely, the image will be placed on the left side of the text.

The orient attribute can be used to place the image above or below the text. The default value is horizontal which is used to place the image on the left or right. You can also use the value vertical to place the image above or below. In this case, the dir attribute controls the placement above or below. The same values are used, where normal means place the image above the text, and reverse means place the image below the text.

Example 3 : Source View
Image:advbtns1.png

<button label="Left" image="happy.png"/>
<button label="Right" image="happy.png" dir="reverse"/>
<button label="Above" image="happy.png" orient="vertical"/>
<button label="Below" image="happy.png" orient="vertical" dir="reverse"/>

 

The example here shows all four types of alignment of buttons. Note that the two attributes are not specified when the default value can be used.
Buttons with Extra Content

Buttons may have arbitrary markup contained inside them, and it will be rendered inside the button. You probably wouldn't use this very often, but you might use it when creating custom elements.

For example, the following will create a button where two of the words are red:

Example 4 : Source View

<button>
  <description value="This is a"/>
  <description value="rather strange" style="color: red;"/>
  <description value="button"/>
</button>

Any XUL element may be placed inside the button. HTML elements will be ignored, so you need to wrap them inside a description element. If you specify the label attribute on the button, it will override any content placed inside the button.
Button with menupopup

You can place a menupopup inside the button to cause a menu to drop down when the button is pressed, much like the menulist. However, in this case you must set the type attribute to the value menu.

Example 5 : Source View
Image:advbtns2.png

<button type="menu" label="Device">
  <menupopup>
    <menuitem label="Printer"/>
    <menuitem label="Mouse"/>
    <menuitem label="Keyboard"/>
  </menupopup>
</button>

In this example, the user may click the button to pop up a menu containing three items. Note that selecting one of these menu items does not change the label on the button, unlike a menulist. This type of button is intended to be used like a menu, with scripts attached to each item to perform a task. We'll see more on menus later.

You can also set the type attribute to the value menu-button. This also creates a button with a menu, but the appearance will be different. The image to the right shows the difference. The left one is a 'menu' and the second one is a 'menu-button'. It has an arrow indicating the presence of a menu. For the 'menu', the user may click anywhere on the button to show the menu. For the 'menu-button', the user must click the arrow to show the menu.

Next, we will learn more details about how XUL elements are positioned in a window.

« Previous
Next »
Document Tags and Contributors
Tags: 

    Tutorials XUL XUL_Tutorial 

Contributors to this page: Sheppy, teoli, trevorh, Chbok, Mgjbot, Enn, Morishoji, Pmash, JPEG, Takenbot, Ptak82, Dria
Last updated by: Sheppy, Apr 14, 2014, 10:34:54 AM
Learn the best of web development

Get the latest and greatest from MDN delivered straight to your inbox.
E-mail
MDN Web Docs

    Web Technologies
    Learn Web Development
    About MDN
    Feedback

Mozilla

    About
    Contact Us
    Firefox

Other languages:

    Terms Privacy Cookies 

© 2005-2018 Mozilla and individual contributors.

Content is available under these licenses.


    Skip to main content
    Select language
    Skip to search

MDN Web Docs

    Technologies References & Guides Feedback 

    Sign in

Search

Edit

The Box Model

« Previous
Next »

In this section, we'll look at how XUL handles layout.
Introduction to Boxes

The main form of layout in XUL is called the 'Box Model'. This model allows you to divide a window into a series of boxes. Elements inside of a box will orient themselves horizontally or vertically. By combining a series of boxes, spacers and elements with flex and pack attributes, you can control the layout of a window.

Although a box is the fundamental part of XUL element layout, it follows a few very simple rules. A box can lay out its children in one of two orientations, either horizontally or vertically. A horizontal box lines up its elements horizontally and a vertical box orients its elements vertically. You can think of a box as one row or one column from an HTML table. Various attributes placed on the child elements in addition to some CSS style properties control the exact position and size of the children.
Box elements

The basic syntax of a box is as follows:

<hbox>
  <!-- horizontal elements -->
</hbox>

<vbox>
  <!-- vertical elements -->
</vbox>

The hbox element is used to create a horizontally oriented box. Each element placed in the hbox will be placed horizontally in a row. The vbox element is used to create a vertically oriented box. Added elements will be placed underneath each other in a column.

There is also a generic box element which defaults to horizontal orientation, meaning that it is equivalent to the hbox. However, you can use the orient attribute to control the orientation of the box. You can set this attribute to the value horizontal to create a horizontal box and vertical to create a vertical box.

Thus, the two lines below are equivalent:

<vbox></vbox>

<box orient="vertical"></box>

The following example shows how to place three buttons vertically.

Example 1 : Source View
Image:boxes-ex1.png

<vbox>
  <button id="yes" label="Yes"/>
  <button id="no" label="No"/>
  <button id="maybe" label="Maybe"/>
</vbox>

The three buttons here are oriented vertically as was indicated by the box. To change the buttons so that they are oriented horizontally, all you need to do is change the vbox element to a hbox element.
Login prompt example

You can add as many elements as you want inside a box, including other boxes. In the case of a horizontal box, each additional element will be placed to the right of the previous one. The elements will not wrap at all so the more elements you add, the wider the window will be. Similarly, each element added to a vertical box will be placed underneath the previous one. The example below shows a simple login prompt:

Example 2 : Source View
Image:boxes-ex2.png

<vbox>
  <hbox>
    <label control="login" value="Login:"/>
    <textbox id="login"/>
  </hbox>
  <hbox>
    <label control="pass" value="Password:"/>
    <textbox id="pass"/>
  </hbox>
  <button id="ok" label="OK"/>
  <button id="cancel" label="Cancel"/>
</vbox>

Here four elements have been oriented vertically, two inner hbox tags and two button elements. Notice that only the elements that are direct descendants of the box are oriented vertically. The labels and textboxes are inside the inner hbox elements, so they are oriented according to those boxes, which are horizontal. You can see in the image that each label and textbox is oriented horizontally.
Aligning textboxes

If you look closely at the image of the login dialog, you can see that the two textboxes are not aligned with each other horizontally. It would probably be better if they were. In order to do this we need to add some additional boxes.

Example 3 : Source View
Image:boxes-ex3.png

<vbox>
  <hbox>
    <vbox>
      <label control="login" value="Login:"/>
      <label control="pass" value="Password:"/>
    </vbox>
    <vbox>
      <textbox id="login"/>
      <textbox id="pass"/>
    </vbox>
  </hbox>
  <button id="ok" label="OK"/>
  <button id="cancel" label="Cancel"/>
</vbox>

Notice how the text boxes are now aligned with each other. To do this, we needed to add boxes inside of the main box. The two labels and textboxes are all placed inside a horizontal box. Then, the labels are placed inside another box, this time a vertical one, as are the textboxes. This inner box is what makes the elements orient vertically. The horizontal box is needed as we want the labels vbox and the inputs vbox to be placed horizontally with each other. If this box was removed, both textboxes would appear below both of the labels.

The issue now is that the 'Password' label is too high. We should really use the grid element here to fix this which we'll learn about in a later section.
Our Find Files Dialog example

Let's add some boxes to the find files dialog. A vertical box will be added around all of the elements, and a horizontal box with be added around the textbox and the buttons. The result will be that the buttons will appear below the textbox.

<vbox flex="1">

  <description>
    Enter your search criteria below and select the Find button to begin
    the search.
  </description>
  
  <hbox>
    <label value="Search for:" control="find-text"/>
    <textbox id="find-text"/>
  </hbox>

  <hbox>
    <spacer flex="1"/>

    <button id="find-button" label="Find"/>
    <button id="cancel-button" label="Cancel"/>
  </hbox>
</vbox>

Image:boxes1.png

The vertical box causes the main text, the box with the textbox and the box with the buttons to orient vertically. The inner boxes orient their elements horizontally. As you see in the image below, the label and text input are placed side by side. The spacer and two buttons are also placed horizontally in their box. Notice how the spacer causes the buttons to appear on the right side, because it is flexible.

Example so far: Source View

In the next section, we will look at specifying the sizes of individual elements and how to constrain their sizes.

« Previous
Next »
Document Tags and Contributors
Tags: 

    Tutorials XUL XUL_Tutorial 

Contributors to this page: Sheppy, teoli, trevorh, madarche, Chbok, Morishoji, Mgjbot, Pmash, JPEG, Takenbot, Ptak82, Dria
Last updated by: Sheppy, Apr 14, 2014, 10:34:49 AM
Learn the best of web development

Get the latest and greatest from MDN delivered straight to your inbox.
E-mail
MDN Web Docs

    Web Technologies
    Learn Web Development
    About MDN
    Feedback

Mozilla

    About
    Contact Us
    Firefox

Other languages:

    Terms Privacy Cookies 

© 2005-2018 Mozilla and individual contributors.

Content is available under these licenses.



    Skip to main content
    Select language
    Skip to search

MDN Web Docs

    Technologies References & Guides Feedback 

    Sign in

Search

Edit

Element Positioning
Jump to:

    Box Element PositioningSetting Minimum and Maximum SizesBox PackingBox AlignmentCropping Text and Buttons 

« Previous
Next »

Here we'll look at controlling the position and size of an element.
Box Element Positioning

So far, we know how to position elements either horizontally or vertically inside a box. We will often need more control over the position and size of elements within the box. For this, we first need to look at how a box works.

The position of an element is determined by the layout style of its container. For example, the position of a button in a horizontal box is to the right of the previous button, if any. The size of an element is determined by two factors, the size that the element wants to be and the size you specify. The size that an element wants to be is determined by what is in the element. For example, a button's width is determined by the amount of text inside the button.

An element will generally be as large as it needs to be to hold its contents, and no larger. Some elements, such as textboxes have a default size, which will be used. A box will be large enough to hold the elements inside the box. A horizontal box with three buttons in it will be as wide as the three buttons, plus a small amount of padding.
Image:boxstyle1n.png

In the image, the first two buttons have been given a suitable size to hold their text. The third button is larger because it contains more content. The width of the box containing the buttons is the total width of the buttons plus the padding between them. The height of the buttons is a suitable size to hold the text.
Width and height attributes

You may need to have more control over the size of an element in a window. There are a number of features that allow you to control the size of an element. The quick way is to simply add the width and height attributes on an element, much like you might do on an HTML img tag. An example is shown below:

Example 1: Source View

<button label="OK" width="100" height="40"/>

However, it is not recommended that you do this. It is not very portable and may not fit in with some themes. A better way is to use style properties, which work similarly to style sheets in HTML. The following CSS properties can be used.

width
    This specifies the width of the element. Note that the CSS global skin of the XUL application may also specify a min-width for the buttons and other elements, thus if simply setting the width property does not set the button width as you expect, try also to modify the button min-width property.
height
    This specifies the height of the element.

By setting either of the two properties, the element will be created with that width and height. If you specify only one size property, the other is calculated as needed. The size of these style properties should be specified as a number followed by a unit.
Flexible elements

The sizes are fairly easy to calculate for non-flexible elements. They simply obey their specified widths and heights, and if the size wasn't specified, the element's default size is just large enough to fit the contents. For flexible elements, the calculation is slightly trickier.

Flexible elements are those that have a flex attribute set to a value greater than 0. Recall that flexible elements grow and shrink to fit the available space. Their default size is still calculated the same as for inflexible elements. The following example demonstrates this:

Example 2 : Source View

<window orient="horizontal"
        xmlns="http://www.mozilla.org/keymaster/gatekeeper/there.is.only.xul">

<hbox>
  <button label="Yes" flex="1"/>
  <button label="No"/>
  <button label="I really don't know one way or the other"/>
</hbox>

</window>

The window will initially appear like in the image earlier. The first two buttons will be sized at a suitable default width and the third button will be larger because it has a longer label. The first button is made flexible and all three elements have been placed inside a box. The width of the box will be set to the initial total width of all three buttons (around 430 pixels in the image).

If you increase the width of the window, elements are checked to see whether they are flexible to fill the blank space that would appear. The first button is the only flexible element, but it will not grow wider. This is because the box that the button is inside is not flexible. An inflexible element never changes size even when space is available, so the button can't grow either. Thus, the button won't get wider.

The solution is to make the box flexible also. Then, when you make the window wider, extra space will be available, so the box will grow to fill the extra space. Because the box is larger, more extra space will be created inside it, and the flexible button inside it will grow to fit the available space. This process repeats for as many nested boxes as necessary.
Setting Minimum and Maximum Sizes

You may want to allow an element to be flexible but constrain the size so that it cannot be larger than a certain size. Or, you may want to set a minimum size. You can set this by using four attributes:

minwidth;
    This specifies the minimum width that the element can be.
minheight
    This specifies the minimum height that the element can be.
maxwidth
    This specifies the maximum width that the element can be.
maxheight
    This specifies the maximum height that the element can be.

The values are always measured in pixels. You can also use the corresponding CSS properties, min-width, min-height, max-width and max-height.

These properties are only useful for flexible elements. By setting a maximum height, for example, a stretchy button will only grow to a certain maximum height. You will still be able to resize the window beyond that point but the button will stop growing in size. The box the button is inside will also continue to grow, unless you set a maximum height on the box also.

If two buttons are equally flexible, normally both will share the amount of extra space. If one button has a maximum width, the second will still continue to grow and take all of the remaining space.

If a box has a maximum width or height, the children cannot grow larger than that maximum size. If a box has a minimum width or height, the children cannot shrink smaller than that minimum size.
Examples of setting widths and heights

<button label="1" style="width: 100px;"/>
<button label="2" style="width: 100em; height: 10px;"/>
<button label="3" flex="1" style="min-width: 50px;"/>
<button label="4" flex="1" style="min-height: 2ex; max-width: 100px"/>
<textbox flex="1" style="max-width: 10em;"/>
<description style="max-width: 50px">This is some boring but simple 
wrapping text.</description>

Example 1 
    The first button will be displayed with a width of 100 pixels (px means pixels). You need to add the unit or the width will be ignored.
Example 2 
    The second button will be displayed with a height of ten pixels and a width of 100 ems (an em is the size of a character in the current font).
Example 3 
    The third button is flexible so it will grow based on the size of the box the button is in. However, the button will never shrink to be less than 50 pixels. Other flexible components such as spacers will absorb the remaining space, breaking the flex ratio.
Example 4 
    The fourth button is flexible and will never have a height that is smaller than 2 ex (an ex is usually the height of the letter x in the current font) or wider than 100 pixels.
Example 5 
    The text input is flexible but will never grow to be larger than 10 ems. You will often want to use ems when specifying sizes with text in them. This unit is useful for textboxes so that the font can change and the textboxes would always be a suitable size, even if the font is very large.
Example 6 
    The description element is constrained to have a maximum width of 50 pixels. The text inside will wrap to the next line, after fifty pixels.

Our find files dialog

Let's add some of these styles to the find files dialog. We'll make it so that the textbox will resize to fit the entire window.

<textbox id="find-text" flex="1" style="min-width: 15em;"/>

Image:boxstyle1.png

Here, the text input has been made flexible. This way, it will grow if the user changes the size of the dialog. This is useful if the user wants to enter a long string of text. Also, a minimum width of 15 ems has been set so that the text box will always show at least 15 characters. If the user resizes the dialog to be very small, the text input will not shrink past 15 ems. It will be drawn as if it extends past the edge of the window. Notice in the image that the text input has grown to extend to the full size of the window.
Box Packing

Let's say you have a box with two child elements, both of which are not flexible, but the box is flexible. For example:

Example 3: Source View

<box flex="1">
  <button label="Happy"/>
  <button label="Sad"/>
</box>

If you resize the window, the box will stretch to fit the window size. The buttons are not flexible, so they will not change their widths. The result is extra space that will appear on the right side of the window, inside the box. You may wish, however, for the extra space to appear on the left side instead, so that the buttons stay right aligned in the window.

You could accomplish this by placing a spacer inside the box, but that gets messy when you have to do it numerous times. A better way is to use an additional attribute pack on the box. This attribute indicates how to pack the child elements inside the box. For horizontally oriented boxes, it controls the horizonal positioning of the children. For vertically oriented boxes, it controls the vertical positioning of the children. You can use the following values:

start
    This positions elements at the left edge for horizontal boxes and at the top edge for vertical boxes. This is the default value.
center
    This centers the child elements in the box.
end
    This positions elements at the right edge for horizontal boxes and at the bottom edge for vertical boxes.

The pack attribute is applied to the box containing the elements to be packed, not to the elements themselves.

We can change the earlier example to center the elements as follows:

Example 4: Source View

<box flex="1" pack="center">
  <button label="Happy"/>
  <button label="Sad"/>
</box>

Now, when the window is resized, the buttons center themselves horizontally. Compare this behavior to that of the previous example.
Box Alignment

If you resize the window in the Happy-Sad example above horizontally, the box will grow in width. If you resize the window vertically however, you will note that the buttons grow in height. This is because the flexibility is assumed by default in the other direction.

You can control this behavior with the align attribute. For horizontal boxes, it controls the position of the children vertically. For vertical boxes, it controls the position of the children horizontally. The possible values are similar to those for pack.

start
    This aligns elements along the top edge for horizontal boxes and along the left edge for vertical boxes.
center
    This centers the child elements in the box.
end
    This aligns elements along the bottom edge for horizontal boxes and along the right edge for vertical boxes.
baseline
    This aligns the elements so that the text lines up. This is only useful for horizontal boxes.
stretch
    This value, the default, causes the elements to grow to fit the size of the box, much like a flexible element, but in the opposite direction.

As with the pack attribute, the align attribute applies to the box containing the elements to be aligned, not to the elements themselves.

For example, the first box below will have its children stretch, because that is the default. The second box has an align attribute, so its children will be placed centered.

Example 5: Source View

<?xml version="1.0"?>
<?xml-stylesheet href="chrome://global/skin/" type="text/css"?>

<window id="yesno" title="Question" orient="horizontal"
        xmlns="http://www.mozilla.org/keymaster/gatekeeper/there.is.only.xul">

  <hbox>
    <button label="Yes"/>
    <button label="No"/>
  </hbox>
  <hbox align="center">
    <button label="Maybe"/>
    <button label="Perhaps"/>
  </hbox>

</window>

Image:boxstyle2-b.png

You can also use the style properties -moz-box-pack and -moz-box-align instead of specifying attributes.
You may find the Box Alignment Example useful for trying out the various box properties.
Cropping Text and Buttons

You could potentially create a button element that contains a label that is larger than the maximum width of the button. Of course, a solution would be to increase the size of the button. However, buttons (and other elements with a label) have a special attribute called crop that allows you to specify how the text may be cropped if it is too big.

If the text is cropped, an ellipsis (...) will appear on the button where the text was taken out. Four possible values are valid:

left
    The text is cropped on its left side
right
    The text is cropped on its right side
center
    The text is cropped in the middle.
none
    The text is not cropped. This is the default value.

This attribute is really only useful when a dialog has been designed to be useful at any size. The crop attribute can also be used with other elements that use the label attribute for labels. The following shows this attribute in use:

Example 6: Source View
Image:boxstyle2.png

<button label="Push Me Please!" crop="right" flex="1"/>

Notice how the text on the button has had the right side of it cropped after the window is made smaller.

Find files example so far: Source View

Next, a summary and some additional details of the box model are described.

« Previous
Next »
Document Tags and Contributors
Tags: 

    Tutorials XUL XUL_Tutorial 

Contributors to this page: fscholz, Sheppy, MattBrubeck, teoli, madarche, kiteroa, The moose, Chbok, Morishoji, Ptak82, Mgjbot, Pmash, SylvainPasche, Takenbot, Napolj2, Dria, MartinS1
Last updated by: fscholz, Mar 17, 2015, 2:35:00 PM
Learn the best of web development

Get the latest and greatest from MDN delivered straight to your inbox.
E-mail
MDN Web Docs

    Web Technologies
    Learn Web Development
    About MDN
    Feedback

Mozilla

    About
    Contact Us
    Firefox

Other languages:

    Terms Privacy Cookies 

© 2005-2018 Mozilla and individual contributors.

Content is available under these licenses.


    Skip to main content
    Select language
    Skip to search

MDN Web Docs

    Technologies References & Guides Feedback 

    Sign in

Search

Edit

Box Model Details

« Previous
Next »

We've seen a lot of features of the box model. Here, we'll find out some more details with some examples.
More Layout Details

The style properties such as min-width and max-height can be applied to any element. We've added them to buttons and textboxes, but we can also add them to spacers or boxes. In addition, the flex attribute can be applied to any element.

Example 1 : Source View

<hbox flex="1">
  <button label="Left" style="min-width: 100px;" flex="1"/>
  <spacer flex="1"/>
  <button label="Right" style="min-width: 100px;" flex="1"/>
</hbox>

In the example above, all three elements will resize themselves as they are all flexible. The two buttons indicate a minimum width of 100 pixels. The buttons will never be smaller than this size but they may grow larger. Here the window should appear just over 200 pixels wide. That's enough for the two buttons. Because all three elements are flexible, but they don't need any more room, the flexibility adds no extra space.
Image:boxdet1.png

As shown in the image, there are two buttons which expand vertically to fit their container, which in this case is the hbox. The align attribute controls this behavior on a horizontal box. You can also prevent this stretching by placing a maximum height on the elements or, better, on the box itself. If a box has a maximum height, the elements inside it are constrained by this. However, the problem with this is that you need to know how big the element will be beforehand.

Example 2 : Source View

<hbox flex="1" align="top">
  <button label="Left" style="min-width: 100px;" flex="1"/>
  <spacer flex="1"/>
  <button label="Right" style="min-width: 100px;" flex="1"/>
</hbox>

Summary of the box model

To achieve complicated layouts, you will generally need to add nested boxes, specify minimum and maximum sizes on some elements, and make certain elements flexible. The best interface is one that can be displayed at various sizes without problems. The box model may be difficult to understand without trying various various things out for yourself.

The following is an outline of both types of boxes:

Horizontal boxes

    Lay out their elements next to each other horizontally.
    Flexible elements are flexed horizontally.
    Packing controls their horizontal placement of child elements.
    Alignment controls how the row of elements are aligned vertically.

Vertical boxes

    Lay out their elements vertically in a column.
    Flexible elements are flexed vertically.
    Packing controls the vertical placement of child elements.
    Alignment controls how the column of child elements are aligned horizontally.

You can put boxes anywhere in a XUL file, including inside an HTML element such as a table. However, the layout will be partly controlled by the HTML element. That means that the flex might not work exactly as you want it. Remember that flexibility only has meaning for elements that are directly inside a box or an element that is a type of box.
Layout Examples
Using Spacers

Example 3 : Source View

<hbox>
  <button label="One"/>
  <spacer style="width: 5px"/>
  <button label="Two"/>
</hbox>

Here, a spacer is used as a separator between the two buttons, by setting an explicit width of 5 pixels. You could also set margins using the CSS margin property.
Centering Buttons

Example 4 : Source View

<hbox pack="center" align="center" flex="1">
  <button label="Look at Me!"/>
  <button label="Push Me!"/>
</hbox>

This example contains a horizontal box with two buttons in it, contained inside a flexible box. The box has the pack attribute which is used to center the buttons horizontally. The align attribute aligns the buttons vertically. The result is that the buttons will be centered in the box in both directions. This example will work with a vertical box as well, although the second button will be underneath the first one, instead of beside it.
A Find Text Dialog

Example 5 : Source View

<?xml version="1.0"?>
<?xml-stylesheet href="chrome://global/skin/" type="text/css"?>

<window id="findtext" title="Find Text" orient="horizontal"
        xmlns="http://www.mozilla.org/keymaster/gatekeeper/there.is.only.xul">

  <vbox flex="3">
    <label control="t1" value="Search Text:"/>
    <textbox id="t1" style="min-width: 100px;" flex="1"/>
  </vbox>

  <vbox style="min-width: 150px;" flex="1" align="start">
    <checkbox id="c1" label="Ignore Case"/>
    <spacer flex="1" style="max-height: 30px;"/>
    <button label="Find"/>
  </vbox>

</window>

Image:boxdet-ex3.png

Here, two vertical boxes are created, one for the textbox and the other for the check box and button. The left box has a flexiblity that is 3 times greater than the right one so it will always receive 3 times as much of the extra space when the window size is increased. The right box enforces a minimum width of 150 pixels.

The textbox is flexible so it will resize as the window resizes. The textbox also enforces a minimum width of 100 pixels. The check box appears in the right box along with its label. Just below the check box is a spacer. The spacer will grow and shrink but not exceed 30 pixels. The result is that the check box and the Find button will be spaced apart from each other by some space of no more than 30 pixels.

The second box has an alignment of start. This causes the child elements to be aligned on the left edge. If this was not specified, the default would be stretch, which would make the child elements stretch horizontally. Because we don't want the Find button to change size, we need to set an alignment.

Next, we will learn about a more specialized type of box, the groupbox.

« Previous
Next »
Document Tags and Contributors
Tags: 

    Tutorials XUL XUL_Tutorial 

Contributors to this page: Sheppy, teoli, trevorh, Chbok, Ptak82, Morishoji, Mgjbot, Pmash, Takenbot, Napolj2, Dria
Last updated by: Sheppy, Apr 14, 2014, 10:34:53 AM
Learn the best of web development

Get the latest and greatest from MDN delivered straight to your inbox.
E-mail
MDN Web Docs

    Web Technologies
    Learn Web Development
    About MDN
    Feedback

Mozilla

    About
    Contact Us
    Firefox

Other languages:

    Terms Privacy Cookies 

© 2005-2018 Mozilla and individual contributors.

Content is available under these licenses.


    Skip to main content
    Select language
    Skip to search

MDN Web Docs

    Technologies References & Guides Feedback 

    Sign in

Search

Edit

Groupboxes
Jump to:

    GroupboxesRadio Groups 

« Previous
Next »

This section describes a way to include elements into groups
Groupboxes

The groupbox element is used to group related XUL elements together, much like the HTML fieldset element is used to group HTML elements. The groupbox is a type of box, and the elements it contains are aligned according to the XUL box rules. However, there are several differences between groupboxes and regular boxes:

    The groupbox can be labeled using the caption element.
    The groupbox is displayed in a special way—usually with a beveled border and a caption, although you can change the appearance using CSS.

You can create a caption for your groupbox by inserting a caption element as the first child. Groupboxes are box elements, so you can use the box attributes, such as orient and flex.
A simple groupbox example

The example below shows a simple groupbox:

Example 1 : Source View
Image:titledbox1.png

<groupbox>
  <caption label="Answer"/>
  <description value="Banana"/>
  <description value="Tangerine"/>
  <description value="Phone Booth"/>
  <description value="Kiwi"/>
</groupbox>

This will cause four pieces of text to be displayed surrounded by a box with the label Answer. Note that the groupbox has a vertical orientation by default which is necessary to have the text elements stack in a single column.
More complex captions

You can also add child elements inside the caption element to create a more complex caption. For example, Mozilla's Font preferences panel uses a drop-down menu as a caption. Although any content can be used, usually you would use a checkbox or dropdown menu.

Example 2 : Source View
Image:groupbox2.png

<groupbox flex="1">
  <caption>
    <checkbox label="Enable Backups"/>
  </caption>
  <hbox>
    <label control="dir" value="Directory:"/>
    <textbox id="dir" flex="1"/>
  </hbox>
  <checkbox label="Compress archived files"/>
</groupbox>


In this example, a checkbox has been used as a caption. We might use a script to enable and disable the contents of the groupbox when the checkbox is checked and unchecked. The groupbox contains a horizontal box with a label and textbox. Both the textbox and groupbox have been made flexible so the textbox expands when the window is expanded. The additional checkbox appears below the textbox because of the vertical orientation of the groupbox. We'll add a groupbox to the find files dialog in the next section.
Radio Groups

You can use the radiogroup element to group radio elements together. The radiogroup is a type of box. You can put any element you want inside it, and apart from its special handling of radio buttons, it works like any other box.

Any radio buttons placed inside the radio group will be grouped together, even if they are inside nested boxes. This could be used to add extra elements within the structure, such as in the following example:

Example 3 : Source View

<radiogroup>
  <radio id="no" value="no" label="No Number"/>
  <radio id="random" value="random" label="Random Number"/>
  <hbox>
    <radio id="specify" value="specify" label="Specify Number:"/>
    <textbox id="specificnumber"/>
  </hbox>
</radiogroup>

Note that the radiogroup element does not draw a border around it. You should place a groupbox element around it if a border and caption are desired.

Next, we'll use what we've learned so far and add some additional elements to the find files dialog.

« Previous
Next »
Document Tags and Contributors
Tags: 

    Tutorials XUL XUL_Tutorial 

Contributors to this page: Sheppy, teoli, ethertank, trevorh, Jacobian, Chbok, JPEG, Morishoji, Ptak82, Mgjbot, Pmash, Takenbot, Dria
Last updated by: Sheppy, Apr 14, 2014, 10:34:53 AM
Learn the best of web development

Get the latest and greatest from MDN delivered straight to your inbox.
E-mail
MDN Web Docs

    Web Technologies
    Learn Web Development
    About MDN
    Feedback

Mozilla

    About
    Contact Us
    Firefox

Other languages:

    Terms Privacy Cookies 

© 2005-2018 Mozilla and individual contributors.

Content is available under these licenses.


    Skip to main content
    Select language
    Skip to search

MDN Web Docs

    Technologies References & Guides Feedback 

    Sign in

Search

Edit

Adding more elements

« Previous
Next »

We will conclude the discussion of boxes by adding some boxes to the find files dialog.
Adding Elements to our find files example

We will add some more elements to the find files dialog now. First, we will add the capability to search for other information such as the file size and date.

<hbox>
  <menulist id="searchtype">
    <menupopup>
      <menuitem label="Name"/>
      <menuitem label="Size"/>
      <menuitem label="Date Modified"/>
    </menupopup>
  </menulist>
  <spacer style="width: 10px;"/>
  <menulist id="searchmode">
    <menupopup>
      <menuitem label="Is"/>
      <menuitem label="Is Not"/>
    </menupopup>
  </menulist>
  <spacer style="width: 10px;"/>
  <textbox id="find-text" flex="1" style="min-width: 15em;"/>
</hbox>

 
 
 
 
 
 
 
 
 
 
 
 
 
 
 

Image:boxfinal1.png

Two drop down boxes have been added to the dialog. A spacer has been added in between each element to separate them. These spacers have been given an explicit width of 10 pixels each. You will notice that if the window is resized, the textbox grows but the other components do not. You will also notice that the label was removed.

If you resize the window vertically, the elements do not change size. This is because they are inside horizontal boxes. In might be more appropriate if the Find and Cancel buttons always stayed along the bottom of the window. This is easy to do by adding a spacer in between the two horizontal boxes.

<spacer style="height: 10px"/>
<hbox>
  <menulist id="searchtype">
    <menupopup>
      <menuitem label="Name"/>
      <menuitem label="Size"/>
      <menuitem label="Date Modified"/>
    </menupopup>
  </menulist>
  <spacer style="width: 10px;"/>
  <menulist id="searchmode">
    <menupopup>
      <menuitem label="Is"/>
      <menuitem label="Is Not"/>
    </menupopup>
  </menulist>
  <spacer style="width: 10px;"/>
  <textbox id="find-text" flex="1" style="min-width: 15em;"/>
</hbox>

<spacer style="height: 10px" flex="1"/>

<hbox>

 
 

Now when the dialog is resized, the two buttons will move so that they are always along the bottom of the dialog. The first spacer adds extra spacing in between the title label and the search criteria elements.

It might look nicer if there was a border around the search criteria. There are two ways to do this. We could use the CSS border property or we could use the groupbox element. This first method would require that we set the style on the box itself. We will use the latter method, however. A groupbox has the advantage that it draws a box with a nice beveled look, suitable for the current theme.

Let us change the box into a groupbox:

<groupbox orient="horizontal">
  <caption label="Search Criteria"/>
  <menulist id="searchtype">
  .
  .
  .
  <spacer style="width: 10px;"/>
  <textbox id="find-text" flex="1" style="min-width: 15em;"/>
</groupbox>

 
 
 

Image:boxfinal2.png

There are other cosmetic problems as well. We could also have the groupbox grow so that it extends vertically to the bottom of the box. Also, we could modify some of the margins so that the elements are positioned better.

We will see more examples of the box model and some of its features as we continue to add elements throughout the tutorial.

Find files example so far Source View

Next, we will look at how to create stacks.

« Previous
Next »
Document Tags and Contributors
Tags: 

    Tutorials XUL XUL_Tutorial 

Contributors to this page: Sheppy, james_oh, teoli, ethertank, trevorh, Chbok, Mgjbot, Morishoji, Ptak82, Pmash, Dria
Last updated by: Sheppy, Apr 14, 2014, 10:34:53 AM
Learn the best of web development

Get the latest and greatest from MDN delivered straight to your inbox.
E-mail
MDN Web Docs

    Web Technologies
    Learn Web Development
    About MDN
    Feedback

Mozilla

    About
    Contact Us
    Firefox

Other languages:

    Terms Privacy Cookies 

© 2005-2018 Mozilla and individual contributors.

Content is available under these licenses.


    Skip to main content
    Select language
    Skip to search

MDN Web Docs

    Technologies References & Guides Feedback 

    Sign in

Search

Edit

Stacks and Decks

« Previous
Next »

There may be need to display elements as a set of overlapping cards. The stack and deck elements can be used for this purpose.
Containers

Each XUL box is a container that can contain any other element. There are a number of elements that are specialized types of boxes, such as toolbars and tabbed panels. The box tag creates the simplest box with no special properties. However, the specialized types of boxes work just like regular boxes in the way they orient the elements inside them, but they have additional features.

In fact, many components can contain other elements. We've already seen that buttons may contain other things besides the default. A scroll bar is just a special type of box that creates its own elements if you don't provide them. It also handles the moving of the scroll bar thumb.

In the next few sections, we'll introduce some elements that are designed for holding other elements. They are all special types of boxes and allow all of the attributes of boxes on them.
Stacks

The stack element is a simple box. It works like any other box but has the special property that its children are laid out all on top of each other. The first child of the stack is drawn underneath, the second child is drawn next, followed by the third and so on. Any number of elements may be stacked up in a stack.

The orient property has little meaning on a stack as children are laid out above each other rather than from side to side. The size of the stack is determined by its largest child, but you can use the CSS properties width, height, min-width and other related properties on both the stack and its children.

The stack element might be used for cases where a status indicator needs to be added over an existing element. For example, a progress bar might be created using a bar and a label overlaid on top of it.
Shadowing with stacks

One convenient use of the stack element however is that you could emulate a number of CSS properties with it. For example, you could create an effect similar to the text-shadow property with the following:

Example 1 : Source View

<stack>
  <description value="Shadowed" style="padding-left: 1px; padding-top: 1px; font-size: 15pt"/>
  <description value="Shadowed" style="color: red; font-size: 15pt;"/>
</stack>

Image:stacks1.png

Both description elements create text with a size of 15 points. The first, however is offset one pixel to the right and down by adding a padding to its left and top sides. This has the result of drawing the same text 'Shadowed' again but slightly offset from the other. The second description element is drawn in red so the effect is more visible.

This method has advantages over using text-shadow because you could completely style the shadow apart from the main text. It could have its own font, underline or size. (You could even make the shadow blink). It is also useful as Mozilla doesn't currently support CSS text shadowing. A disadvantage is that the area taken up by the shadow makes the size of the stack larger. Shadowing is very useful for creating the disabled appearance of buttons:

Example 2 : Source View

<stack style="background-color: #C0C0C0">
  <description value="Disabled" style="color: white; padding-left: 1px; padding-top: 1px;"/>
  <description value="Disabled" style="color: grey;"/>
</stack>

This arrangement of text and shadow colors creates the disabled look under some platforms.

Note that events such as mouse clicks and keypresses are passed to the element on the top of the stack, that is, the last element in the stack. That means that buttons will only work properly as the last element of the stack.
Decks

A deck element also lays out its children on top of each other much like the stack element, however decks only display one of their children at a time. This would be useful for a wizard interface where a series of similar panels are displayed in sequence. Rather than create separate windows and add navigation buttons to each of them, you would create one window and use a deck where the content changes.

Like stacks, the direct children of the deck element form the pages of the deck. If there are three children of the deck element, the deck will have three children. The displayed page of the deck can be changed by setting an selectedIndex attribute on the deck element. The index is a number that identifies which page to display. Pages are numbered starting from zero. So, the first child of the deck is page 0, the second is page 1 and so on.

Example 3 : Source View

<deck selectedIndex="2">
  <description value="This is the first page"/>
  <button label="This is the second page"/>
  <box>
    <description value="This is the third page"/>
    <button label="This is also the third page"/>
  </box>
</deck>

Three pages exist here, the default being the third one. The third page is a box with two elements inside it. Both the box and the elements inside it make up the page. The deck will be as large as the largest child, which here should be the third page.

You can switch pages by using a script to modify the selectedIndex attribute. More on this in the section on events and the DOM.

The next section describes how stacks can be used to position child elements.

« Previous
Next »
Document Tags and Contributors
Tags: 

    Tutorials XUL XUL_Tutorial 

Contributors to this page: SphinxKnight, Sheppy, teoli, trevorh, Chbok, Vshih, Morishoji, Ptak82, Mgjbot, Pmash, Dria
Last updated by: SphinxKnight, Feb 9, 2018, 11:22:19 AM
Learn the best of web development

Get the latest and greatest from MDN delivered straight to your inbox.
E-mail
MDN Web Docs

    Web Technologies
    Learn Web Development
    About MDN
    Feedback

Mozilla

    About
    Contact Us
    Firefox

Other languages:

    Terms Privacy Cookies 

© 2005-2018 Mozilla and individual contributors.

Content is available under these licenses.


    Skip to main content
    Select language
    Skip to search

MDN Web Docs

    Technologies References & Guides Feedback 

    Sign in

Search

Edit

Stack Positioning

« Previous
Next »

This section will describe how to position items in a stack.
Placement of Stack Children

Normally, the child elements of a stack stretch to fit the size of the stack. However, you may also place the children at specific coordinates. For example, if a stack has two buttons as children, one may be placed 20 pixels from the left edge and 50 pixels from the top edge. The second button can be placed at 100 pixels from the left edge and 5 pixels from the top edge.

The position of a child element may be specified by placing two attributes on the element. For the horizontal position, use the left attribute and for the vertical position, use the top attribute. If you don't put these attributes on a child of a stack, the child will stretch to fit the size of the stack.

Example 1 : Source View
Image:bulletins1.png

<stack>
  <button label="Goblins" left="5" top="5"/>
  <button label="Trolls" left="60" top="20"/>
  <button label="Vampires" left="10" top="60"/>
</stack>

The stack here contains three elements, each positioned at the coordinates given by the left and top attributes. Here, all three children are buttons, but the elements do not have to be same type. They may be any element, including boxes and other stacks.

The size of a stack is determined by the positions of the child elements. It is always sized so that all of the child elements are visible (excluding any children with -moz-stack-sizing: ignore). So if you set a left attribute to 400, the stack will have a width around 400 pixels plus the width of the element. You can override this size with the various style properties such as width and max-width.

You can use a script to adjust the value of the left and top attributes and thus make the elements move around. Stacks have the advantage that when one absolutely positioned element changes its position, the position of the other elements is not affected. If you tried to move elements in a regular box, other elements might shuffle their positions around.

It is also possible to place the child elements so that they overlap. When drawing the child elements, the elements are shown in the order that they appear in the stack. That is, the first child of the stack appears at the back, the next child appears next and so on. The last element appears on top. You can use the DOM functions to move the order of the elements around.

When responding to mouse events, the elements on top will capture the events first. That means that if two buttons overlap, the top button will capture a mouse click where it covers the other one.
Width and Height

The  bottom and right attributes can also be used in conjunction with top and left to set the width and/or height of the children of the stack.

Note that bottom and right attributes, unlike rect, are relative to the bottom and right of the stack.

When using these attributes to set width or height, both attributes for the given axis must be explicitly set, eg, if setting width, both "left" and "right" must be set.

In this example, the resulting width of the top-most hbox will be 400px:

<stack width="600">
  <hbox flex="1">
    <!-- content -->
  </hbox>
  <hbox left="0" right="200" >
    <!-- Some content here. -->
  </hbox>
</stack>

In some case, setting the width or height this way may even be necessary because using the width/height attributes (eg, "width", "minwidth" and "maxwidth") inside a stack can sometimes produce unpredictable and undesireable results.

 

The next section describes tabboxes which are like decks but provide their own navigation.

« Previous
Next »
Document Tags and Contributors
Tags: 

    Tutorials XUL XUL_Tutorial 

Contributors to this page: SphinxKnight, MarkusStange, Sheppy, Allasso, teoli, openjck, trevorh, Chbok, Morishoji, Ptak82, Mgjbot, Pmash, Dria
Last updated by: SphinxKnight, Feb 9, 2018, 10:48:04 AM
Learn the best of web development

Get the latest and greatest from MDN delivered straight to your inbox.
E-mail
MDN Web Docs

    Web Technologies
    Learn Web Development
    About MDN
    Feedback

Mozilla

    About
    Contact Us
    Firefox

Other languages:

    Terms Privacy Cookies 

© 2005-2018 Mozilla and individual contributors.

Content is available under these licenses.


    Skip to main content
    Select language
    Skip to search

MDN Web Docs

    Technologies References & Guides Feedback 

    Sign in

Search

Edit

Tabboxes
Jump to:

    TabboxesAdding Tabs to our Find Files Dialog 

« Previous
Next »

It is common in preference dialogs for tabbed pages to appear. We'll find out how to create them here.
Tabboxes

Tabboxes are typically used in an application in the preferences window. A series of tabs appears across the top of a window. The user can click each tab to see a different set of options. It is useful in cases when you have more options than will fit in one screen.

XUL provides a method to create such dialogs. It involves five new elements, which are described briefly here and in more detail below.

tabbox
    The outer box that contains the tabs along the top and the tab pages themselves.
tabs
    The inner box that contains the individual tabs. In other words, this is the row of tabs.
tab
    A specific tab. Clicking on the tab brings the tab page to the front.
tabpanels
    The container for the pages.
tabpanel
    The body of a single page. You would place the content for a page within this element. The first tabpanel corresponds to the first tab, the second tabpanel corresponds to the second tab and so on.

The tabbox is the outer element. It consists of two children, a tabs element which contains the row of tabs and a tabpanels elements which contains the tabbed pages.

Shown below is the general syntax of a tabbox:

<tabbox id="tablist">
  <tabs>
    <!-- tab elements go here -->
  </tabs>
  <tabpanels>
    <!-- tabpanel elements go here -->
  </tabpanels>
</tabbox>

The tab elements are placed inside a tabs element, which is much like a regular box. The tabs element itself has been placed inside a tabbox. The tabbox also contains a tabpanels element which will appear below the tabs due to the vertical orientation on the whole tabbox.

There is really nothing special about the tab elements that make them different than boxes. Like boxes, tabs can contain any element. The difference is that the tabs render slightly differently and only one tab panel's contents are visible at once, much like a deck.

The contents of the individual tab pages should go inside each tabpanel element. They do not go in the tab elements, as that is where the contents of the tabs along the top go.

Each tabpanel element becomes a page on the tabbed display. The first panel corresponds to the first tab, the second panel corresponds to the second tab, and so on. There is a one-to-one relationship between the tab and tabpanel elements.

When determining the size of the tabbox, the size of the largest page is used. That means that if there are ten textboxes on one tab page and only one on another, the tab page will be sized to fit the one with the ten on it as this takes up more room. The area taken up by the tab area does not change when the user switches to a new tab page.
Tabbox example

Example 1 : Source View
Image:tabpanel1.png

<tabbox>
  <tabs>
    <tab label="Mail"/>
    <tab label="News"/>
  </tabs>
  <tabpanels>
    <tabpanel id="mailtab">
      <checkbox label="Automatically check for mail"/>
    </tabpanel>
    <tabpanel id="newstab">
      <button label="Clear News Buffer"/>
    </tabpanel>
  </tabpanels>
</tabbox>

Here, two tabs have been added, the first labeled 'Mail' and the other 'News'. When the user clicks on the Mail tab, the contents of the first tab page will be displayed below the tabs. In this case, the page with the check box labeled 'Automatically check for mail' will appear. When the second tab is clicked, the page containing the button labeled 'Clear News Buffer' will appear instead.

The currently selected tab element is given an additional selected attribute which is set to true. This alters the appearance of the currently selected tab to make it look selected. Only one tab will have a true value for this attribute at a time.
Position of the tabs

Finally, you can change the position of the tabs so that they appear on any side of the tab pages. There is no special syntax to do this. You simply set the orient and dir attributes as necessary. Remember that the tab elements are much like regular boxes in terms of layout. Moreover, the tabbox element is much like a regular container box with a default vertical orientation, whereas the tabs element is much like a container box with a default horizontal orientation.

For example, to place the tabs along the left side, change the orientation of the tabs element to vertical to make the tabs appear vertically stacked. Next, adjust the tabbox so it has horizontal orientation. This will make the tabs appear to the left of, not above, the tab pages. Note that changing the orientation of the tabpanels element will have no effect, since the tabbed pages are layered on top of each other.

You can place the tabs on the right or bottom side by moving the tabs element to after the tabpanels element in your code. Alternatively, you could set the dir attribute to reverse on the tabbox. However, you should probably leave the tabs on top, otherwise they might not look very good under particular themes.
Adding Tabs to our Find Files Dialog

Let's add a second panel to the find files dialog. We'll create an Options tab (and select it by default) that will contain some options for searching. This may not be the best interface for doing this, but we'll use it to demonstrate tabs. The label across the top and the search criteria box will need to go on the first tab. We'll add some options on the second tab. The progress bar and the buttons can stay on the main dialog, outside of the tabs.

<vbox flex="1">

<tabbox selectedIndex="1">
  <tabs>
    <tab label="Search"/>
    <tab label="Options"/>
  </tabs>

  <tabpanels>
   <tabpanel id="searchpanel" orient="vertical">

    <description>
     Enter your search criteria below and select the Find button to begin
     the search.
    </description>

    <spacer style="height: 10px"/>

    <groupbox orient="horizontal">
      <caption label="Search Criteria"/>

      <menulist id="searchtype">
        <menupopup>
          <menuitem label="Name"/>
          <menuitem label="Size"/>
          <menuitem label="Date Modified"/>
        </menupopup>
      </menulist>
      <spacer style="width: 10px;"/>
      <menulist id="searchmode">
        <menupopup>
          <menuitem label="Is"/>
          <menuitem label="Is Not"/>
        </menupopup>
      </menulist>

      <spacer style="height: 10px"/>
      <textbox id="find-text" flex="1" style="min-width: 15em;"/>

    </groupbox>
   </tabpanel>

   <tabpanel id="optionspanel" orient="vertical">
    <checkbox id="casecheck" label="Case Sensitive Search"/>
    <checkbox id="wordscheck" label="Match Entire Filename"/>
   </tabpanel>

 </tabpanels>
</tabbox>

Image:tabpanel2.png

The tab elements have been placed around the main content of the window. You can see the two tabs, Search and Options. Clicking on each one will bring up the respective tab pages. As shown by the image, the two options appear on the second tab. The first tab looks pretty much like it did before, apart from the tabs along the top.

Find files example so far : Source View

Next, we'll look at how to create grids of content.

« Previous
Next »
Document Tags and Contributors
Tags: 

    Tutorials XUL XUL_Tutorial 

Contributors to this page: fscholz, Sheppy, teoli, trevorh, Enn, Brdude, Chbok, 20after4, Morishoji, Ptak82, Mgjbot, Pmash, Napolj2, Dria
Last updated by: fscholz, Mar 17, 2015, 2:52:43 PM
Learn the best of web development

Get the latest and greatest from MDN delivered straight to your inbox.
E-mail
MDN Web Docs

    Web Technologies
    Learn Web Development
    About MDN
    Feedback

Mozilla

    About
    Contact Us
    Firefox

Other languages:

    Terms Privacy Cookies 

© 2005-2018 Mozilla and individual contributors.

Content is available under these licenses.


    Skip to main content
    Select language
    Skip to search

MDN Web Docs

    Technologies References & Guides Feedback 

    Sign in

Search

Edit

Grids

« Previous
Next »

XUL has a set of elements for creating tabular grids.
XUL Tabular Layout

XUL has a set of elements for doing layout of elements in a grid-like manner using the grid element. It has some similarities to the HTML table tag. The grid does not display anything itself; it is used only to position elements in a tabular form with rows and columns.

A grid contains elements that are aligned in rows just like tables. Inside a grid, you declare two things, the columns that are used and the rows that are used. Just like HTML tables, you put content such as labels and buttons inside the rows. However, the grid allows either row or column based organization so you may put content in either rows or in columns. It is most common to use rows, as with a table. However, you can still use columns to specify the size and appearance of the columns in a grid. Alternatively, you can put content inside the columns, and use the rows to specify the appearance. We'll look at the case of organizing elements by row first.
Declaring a grid

To declare a set of rows, use the rows tag, which should be a child element of grid. Inside that you should add row elements, which are used for each row. Inside the row element, you should place the content that you want inside that row.

Similarly, the columns are declared with the columns element, which should be placed as a child element of the grid. Inside that go individual column elements, one for each column you want in the grid.

This should be easier to understand with an example.

Example 1 : Source View
Image:grids1.png

<grid flex="1">
  
  <columns>
    <column flex="2"/>
    <column flex="1"/>
  </columns>

  <rows>
    <row>
      <button label="Rabbit"/>
      <button label="Elephant"/>
    </row>
    <row>
      <button label="Koala"/>
      <button label="Gorilla"/>
    </row>
  </rows>

</grid>

Two rows and two columns have been added to a grid. Each column is declared with the column tag. Each column has been given a flex attribute. Each row contains two elements, both buttons. The first element in each row element is placed in the first column of the grid and the second element is each row is placed in the second column. Note that you do not need an element to declare a cell -- there is no equivalent of the HTML td element. Instead, you put the contents of cells directly in the row elements.
Grid with more elements

You can use any element in place of the button element of course. If you wanted one particular cell to contain multiple elements, you can use a nested hbox or other box element. An hbox is a single element but you can put as many elements that you want inside it. For example:

Example 2 : Source View

<grid flex="1">

  <columns>
    <column/>
    <column flex="1"/>
  </columns>

  <rows>
    <row>
      <label control="doctitle" value="Document Title:"/>
      <textbox id="doctitle" flex="1"/>
    </row>
    <row>
      <label control="docpath" value="Path:"/>
      <hbox flex="1">
        <textbox id="docpath" flex="1"/>
        <button label="Browse..."/>
      </hbox>   
    </row>
  </rows>

</grid>

Image:grids2.png

Notice in the image how the first column of elements containing the labels has only a single element in each row. The second column contains a box in its second row, which in turn contains two elements, a textbox and a button. You could add additional nested boxes or even another grid inside a single cell.

If you resize the window of the last example, you will see that the textboxes resize, but no other elements do. This is because of the flex attributes added to the textboxes and the second column. The first column does not need to be flexible as the labels do not need to change size.

The initial width of a column is determined by the largest element in the column. Similarly, the height of a row is determined by the size of the elements in a row. You can use the minwidth and maxwidth and related attributes to further define the size.
Column based organization

You can also place the elements inside the column elements instead of the rows. If you do this, the rows are just declared to specify how many rows there are.

Example 3 : Source View

<grid>
  <rows>
    <row/>
    <row/>
    <row/>
  </rows>

  <columns>
    <column>
      <label control="first" value="First Name:"/>
      <label control="middle" value="Middle Name:"/>
      <label control="last" value="Last Name:"/>
    </column>
    <column>
      <textbox id="first"/>
      <textbox id="middle"/>
      <textbox id="last"/>
    </column>
  </columns>

</grid>

This grid has three rows and two columns. The row elements are just placeholders to specify how many there are. You may add the flex attribute to a row to make it flexible. The content is placed inside in each column. The first element inside each column element is placed in the first row, the second element in the second row and the third element is placed in the third row.

If you put content in both the columns and the rows, the content will overlap each other, although they will align in a grid properly. It creates an effect much like a grid of stack elements.

The order of the elements in the grid determines which is displayed on top and which is placed underneath. If the rows element is placed after the columns element, the content within the rows is displayed on top. If the columns element is placed after the rows element, the content within the columns is displayed on top. Similarly, events such as mouse buttons and keypresses are sent only to the set on top. This is why the columns were declared after the rows in the above example. If the columns has been placed first, the rows would have grabbed the events and you would not be able to type into the fields.
Flexibility of grids

One advantage that grids have over a set of nested boxes is that you can create cells that are flexible both horizontally and vertically. You can do this by putting a flex attribute on both a row and a column. The following example demonstrates this:

Example 4 : Source View

<grid flex="1">
 <columns>
  <column flex="5"/>
  <column/>
  <column/>
 </columns>
 <rows>
  <row flex="10">
    <button label="Cherry"/>
    <button label="Lemon"/>
    <button label="Grape"/>
  </row>
  <row flex="1">
    <button label="Strawberry"/>
    <button label="Raspberry"/>
    <button label="Peach"/>
  </row>
 </rows>
</grid>

The first column and both rows have been made flexible. This will result in every cell in the first column being flexible horizontally. In addition, every cell will be flexible vertically because both rows are flexible, although the first row is more so. The cell in the first column and first row (the Cherry button) will be flexible by a factor of 5 horizontally and flexible by a factor of 10 vertically. The next cell, (Lemon) will only be flexible vertically.

The flex attribute has also been added to the grid element so that the entire grid is flexible, otherwise it would only grow in one direction.
Column Spanning

There is no means of making a cell span a particular number of multiple columns or rows (See Discussion for a way of achieving the same effect). However, it is possible to make a row or column that spans the entire width or height of the grid. To do this, just add an element inside the rows element that isn't inside a row element. You can use a box type for example, and put other elements inside it if you want to use several elements. Here is a simple example:

Example 5 : Source View

<grid>
  <columns>
    <column flex="1"/>
    <column flex="1"/>
  </columns>

  <rows>
    <row>
      <label value="Northwest"/>
      <label value="Northeast"/>
    </row>
    <button label="Equator"/>
    <row>
      <label value="Southwest"/>
      <label value="Southeast"/>
    </row>
  </rows>
</grid>

The button will stretch to fit the entire width of the grid as it is not inside a grid row. You can use a similar technique to add an element in-between two columns. It would stretch to fit the height of the grid. You could also do both if that is desired.

Next, we'll look at adding content panels.

« Previous
Next »
Document Tags and Contributors
Tags: 

    Tutorials XUL XUL_Tutorial 

Contributors to this page: SphinxKnight, Sheppy, teoli, trevorh, Nalhilal, Mixticius, Chbok, Morishoji, Efredricksen, Ptak82, Mgjbot, Pmash, Napolj2, Dria, Enn
Last updated by: SphinxKnight, Feb 9, 2018, 11:05:24 AM
Learn the best of web development

Get the latest and greatest from MDN delivered straight to your inbox.
E-mail
MDN Web Docs

    Web Technologies
    Learn Web Development
    About MDN
    Feedback

Mozilla

    About
    Contact Us
    Firefox

Other languages:

    Terms Privacy Cookies 

© 2005-2018 Mozilla and individual contributors.

Content is available under these licenses.


    Skip to main content
    Select language
    Skip to search

MDN Web Docs

    Technologies References & Guides Feedback 

    Sign in

Search

Edit

Content Panels
Jump to:

    Adding Child PanelsBrowsers 

« Previous
Next »

In this section, we'll look at how to add panels that can display HTML pages or other XUL files.
Adding Child Panels

There may be times when you want to have part of a document loaded from a different page. Sometimes, you will want to change part of the window. A good example is a step-by-step wizard that guides you through a number of screens, asking a set of questions. Each time the user clicks the Next button, the next screen of the wizard is displayed.

You could create a wizard interface by opening a different window for each screen. There are three problems with this approach. First, each window could appear in a different location (although there are ways around this). Second, the elements such the Back and Next buttons are the same throughout the interface. It would be much better if just the content area of the wizard changed. Third, it would be difficult to co-ordinate scripts when running in different windows.

Note that XUL does have a wizard element which may be used to create wizard interfaces. This will be described in a later section.

Another approach is to use the iframe element, which works much like the HTML element of the same name. It creates a separate document within the window. It has the advantage that it can be placed anywhere and the contents can be loaded from a different file. Set the URL to appear in the frame with the src attribute. This URL may point to any kind of file, although it will usually point to an HTML file or another XUL file. You can use a script to change the contents of the iframe without affecting the main window.

In the Mozilla browser window, the area where the web page is displayed is created by using an iframe. When the user enters a URL or clicks on a link in a document, the source of the frame is changed.
iframe example

Example 1 : Source View

<toolbox>
  <toolbar id="nav-toolbar">
    <toolbarbutton label="Back" />
    <toolbarbutton label="Forward" />
    <textbox id="urlfield" />
  </toolbar>
</toolbox>

<iframe id="content-body" src="http://www.mozilla.org/index.html" flex="1" />

The example here has created a very simple interface for a web browser. A box has been created containing two elements: a toolbox and an iframe. A Back button, a Forward button and a field for typing is URLs has been added to the only toolbar (We'll learn about toolbar in a later section). The web pages would appear inside the iframe. In this case, the file index.html would appear by default.

This example isn't functionally complete. Next, we would want to add script which changes the src attribute at the desired time, for example when the user presses the Enter key.
Browsers

There is a second type of content panel, using the browser tag. You would use this when you want to create a frame that displays content like a browser. Actually, the iframe can do this too, but the browser has a variety of additional features. For example, the browser maintains a page history for use with Back and Forward buttons. The browser can also load pages with referers and other flags. Essentially, the browser tag should be used when you want to create a browser-like interface, but the iframe may be used when you just need a simple panel.

A similar element, tabbrowser, provides the functionality of browser but also provides a tab bar for switching between multiple pages. This is the widget used by the Mozilla browser for its tabbed browsing interface. The tabbrowser element is actually implemened as a tabbox containing a set of browser elements. Both types of browser offer similar control over pages that are displayed.
browser example
Example 2 : Source View

<browser src="http://www.mozilla.org" flex="1" />

As with the iframe, you can specify the url in a browser using the src attribute. For a tabbrowser, you cannot set the url directly like this, as it doesn't display just one url. Instead, you must use a script and call the loadURI function.

There are three classes of browser, depending on the kind of content that you want to display inside. The type may be specified using the type attribute.

The first type is the default and is used if you don't specify a type. In this case, the content loaded inside the browser is treated as if it was part of the same application and has access to the outer window. That means that when a script loaded inside the browser tries to get the topmost window, it will get the outer XUL window.

This would be suitable for a child XUL panel that is part of your application, but this isn't what you want for a browser that loads a web page. Instead, you would want to restrict the web page to only getting access to the web page content. You might note that a Mozilla browser window has XUL content for the toolbars and statusbar and so forth with a tabbrowser forming the main area. This inner area displays a web page, but the web page cannot access the XUL around it. This is because it uses the second type of browser, specified by setting the type attribute to the value content. This prevents the content from traversing up to the XUL window. An example:

<browser src="http://www.mozilla.org" type="content" flex="1" />

Important: You must set the type attribute correctly if you are going to be displaying remote web sites inside the browser element.

The tabbrowser sets the content type automatically on all tabbed browsers that it creates. So you don't have to set this explicitly for tabbed browsers.

The third type is used when your window contains multiple browser elements, for example if you have a sidebar displaying some extra content. Set the type attribute on the main browser element to content-primary to indicate that its content will be the primary content inside the window. This acts just like the content value except that the content inside is accessible using the XUL window's 'content' property. This makes it easy to access the content of the main browser using a script. The tabbrowser automatically sets the type attribute of whichever browser is currently visible to content-primary, which means that you will always be able to access the currently visible content in this way.

Next, we'll look at how to create a splitter.

« Previous
Next »
Document Tags and Contributors
Tags: 

    Tutorials XUL XUL_Tutorial 

Contributors to this page: Sheppy, ethertank, teoli, fscholz, trevorh, Chbok, Morishoji, Mgjbot, Pmash, Napolj2, Ptak82, Dria
Last updated by: Sheppy, Apr 14, 2014, 10:34:52 AM
Learn the best of web development

Get the latest and greatest from MDN delivered straight to your inbox.
E-mail
MDN Web Docs

    Web Technologies
    Learn Web Development
    About MDN
    Feedback

Mozilla

    About
    Contact Us
    Firefox

Other languages:

    Terms Privacy Cookies 

© 2005-2018 Mozilla and individual contributors.

Content is available under these licenses.


    Skip to main content
    Select language
    Skip to search

MDN Web Docs

    Technologies References & Guides Feedback 

    Sign in

Search

Edit

Splitters
Jump to:

    Splitting a BoxOur find files example 

« Previous
Next »

We'll now look at how to add splitters to a window.
Splitting a Box

There may be times when you want to have two sections of a window where the user can resize the sections. An example is the Mozilla browser window, where you can change the size of the sidebar panel by dragging the bar in-between the two frames. You can also hide the sidebar by clicking the notch.
Splitter element

This feature is accomplished using an element called a splitter. It creates a skinny bar between two sections which allows these sections to be resized. You can place a splitter anywhere you want and it will allow resizing of the elements that come before it and the elements that come after it in the same box.

When a splitter is placed inside a horizontal box, it will allow resizing horizontally. When a splitter is placed inside a vertical box, it will allow resizing vertically.

The syntax of a splitter is as follows:

<splitter
    id="identifier"
    state="open"
    collapse="before"
    resizebefore="closest"
    resizeafter="closest">

The attributes are as follows:

id
    The unique identifier of the splitter.

state
    Indicates the state of the splitter. Set this to open, the default, to have the split panel initially open, or set it to collapsed to have one of the panels shrunk down (collapsed) and the other occupying all of the space.

collapse
    This indicates which side of the panel should collapse when the splitter notch (or grippy) is clicked or set into a collapsed state. Set this to before for the element before the splitter, or after for the element after the splitter. If you set this to none, which is also the default, the splitter grippy does not collapse when clicked.

resizebefore
    When the splitter is dragged, the elements to the left (or above) resize. This attribute indicates which element should resize. Set this to closest to have the element immediately to the left of the splitter resize. Set this to farthest to have the element that is the farthest away from the splitter to the left resize. (The first element in the box). The default value is closest.

resizeafter
    When the splitter is dragged, the elements to the right (or below) resize. This attribute indicates which element should resize. Set this to closest to have the element immediately to the right of the splitter resize. Set this to farthest to have the element that is the farthest away from the splitter to the right resize. (The last element in the box). This attribute can also be set to grow, in which case the elements to the right of the splitter do not change size when the splitter is dragged, but instead the entire box changes size. The default value is closest.

If you set the collapse attribute, you should also add a grippy element inside the splitter which the user can use to collapse the element.

The width and height attributes of the elements next the splitter are adjusted when the splitter is dragged. The exact element depends on the resizebefore and resizeafter attributes.
Splitter example

An example would be helpful here:

Example 1 : Source View

<hbox flex="1">
  <iframe id="content-1" width="60" height="20" src="w1.html"/>
  <splitter collapse="before" resizeafter="farthest">
    <grippy/>
  </splitter>
  <iframe id="content-2" width="60" height="20" src="w2.html"/>
  <iframe id="content-3" width="60" height="20" src="w3.html"/>
  <iframe id="content-4" width="60" height="20" src="w4.html"/>
</hbox>

Image:splitter-ex1.jpg

Here, four iframes have been created and a splitter has been placed in-between the first and second one. The collapse has been set to a value of before, meaning that if the splitter grippy is clicked on, the first frame would disappear and the splitter and the remaining frames would shuffle to the left. The splitter grippy is drawn centered inside the splitter.

The splitter has been given a resizeafter value of farthest. This means that when the splitter is dragged, the farthest element after it will change size. In this case, frame 4 will change size.

A value has not been specified for resizebefore so it will default to a value of closest. In this case, there is only one frame before the splitter, so frame 1 will change size.

Frames 2 and 3 will only change size if you drag the splitter far enough to the right that frame 4 has reached its minimum size.
Image:splitter-ex2.jpg

This image shows the 4 panels with the splitter in a collapsed state.
Image:splitter-ex3.jpg

This image shows the 4 panels with the splitter resized to the right. Notice how the middle two panels have not changed size. Only panel 1 and panel 4 have changed size. You can just see part of the fourth panel. If you continue to drag the splitter to the right, the other two panels will shrink. You can use the style properties such as min-width, max-height on the iframes to specify minimum or maximum widths or heights in the box. If you do, the splitter will detect this and not allow the user to drag the splitter past the minimum and maximum sizes.

For example, if you specified a minimum width of 30 pixels on panel 4 above, it would not shrink below that size. The other two panels would have to shrink. If you set the minimum width of panel 1 to 50 pixels, you would only be able to drag the splitter 10 pixels to the left (as it starts at 60 pixels wide). You can still collapse the splitter however.

You can also place more than one splitter in a box if you want, in which case you could collapse different parts of it. Similarly, you do not have to collapse just iframes. Any element can be collapsed.
Our find files example

Let's see what the find file dialog looks like with a splitter in it. One possibility would be to add the results of the search in the dialog. We'll add an area in-between the search criteria and the buttons along the bottom. A splitter will allow you to collapse, or hide, the search results.

</tabbox>

  <iframe src="results.html"/>  <splitter collapse="before" resizeafter="grow">   <grippy/>  </splitter> 
 
 <hbox>

Here, a splitter and an iframe have been added to the dialog. We don't need the spacer after the tabbox any more so we can remove it. The content of the frame is contained in a file called 'results.html'. Create this file and put whatever you want in it for now. The iframe will be replaced later with a results list when we know how to create it. For now, it serves to demonstrate the splitter.

The splitter has been set to a collapse value of before meaning that the element just before the splitter will collapse. Here, it is the iframe. As the images below show, when the grippy is clicked, the iframe is collapsed and the buttons shuffle up.

The resizeafter attribute has been set to grow so that the elements after the splitter push themselves down when the splitter is dragged down. This results in the content of the frame growing to any size. It should be noted that the window does not resize itself automatically. You'll also notice that this is a horizontal splitter because it has been placed in a vertical box.

Normal State:

Image:splitter1.png

Collapsed State:

Image:splitter2.png

Find files example so far : Source View

Next, we'll find out how to create toolbars.

« Previous
Next »
Document Tags and Contributors
Tags: 

    Tutorials XUL XUL_Tutorial 

Contributors to this page: fscholz, Sheppy, teoli, trevorh, Brdude, Enn, Brettz9, Chbok, Mgjbot, Morishoji, Ptak82, Pmash, Napolj2, Dria
Last updated by: fscholz, Mar 17, 2015, 2:30:27 PM
Learn the best of web development

Get the latest and greatest from MDN delivered straight to your inbox.
E-mail
MDN Web Docs

    Web Technologies
    Learn Web Development
    About MDN
    Feedback

Mozilla

    About
    Contact Us
    Firefox

Other languages:

    Terms Privacy Cookies 

© 2005-2018 Mozilla and individual contributors.

Content is available under these licenses.


    Skip to main content
    Select language
    Skip to search

MDN Web Docs

    Technologies References & Guides Feedback 

    Sign in

Search

Edit

Toolbars

« Previous
Next »

A toolbar is usually placed along the top of a window and contains a number of buttons that perform common functions. XUL has a method to create toolbars.
Adding a Toolbar

Like a number of elements, XUL toolbars are a type of box. Usually, a row of buttons would appear in the toolbar, but any element can be placed in a toolbar. For example, the Mozilla browser window contains a textbox that displays the page URL.

Toolbars may be placed on any side of the window, either horizontally or vertically. Of course you wouldn't normally put a textbox in a vertical toolbar. Actually, because toolbars are just boxes they can actually go anywhere you want, even in the middle of a window. Typically however, a set of toolbars would appear along the top of a window. When more than one toolbar is placed next to each other, they are typically grouped together in something called a 'toolbox'.
A simple toolbar inside a toolbox

Source View
Image:toolbar1.jpg

<toolbox>
  <toolbar id="nav-toolbar">
    <toolbarbutton label="Back"/>
    <toolbarbutton label="Forward"/>
  </toolbar>
</toolbox>

This has created a toolbar containing two buttons, a Back button and a Forward button. The one toolbar has been placed inside the toolbox. This has involved four new tags, which are described here.

toolbox
    A box that contains toolbars.

toolbar
    A single toolbar that contains toolbar items such as buttons.

toolbarbutton
    A button on a toolbar, which has all the same features of a regular button but is usually drawn differently.

The toolbar is the main element that creates the actual toolbar. Inside it are placed the individual toolbar items, usually buttons, but they can be other elements.

In the example above, only one toolbar was created. Multiple toolbars can be created just as easily by adding more toolbar elements after the first one.

The toolbox is a container for toolbars. In some applications, you will have several toolbars along the top of the window. You can put them all inside a toolbox.

You do not have to put toolbar elements inside a toolbox.
Customizable toolbars

Firefox or other Toolkit applications have customizable toolbars; therefore, many extensions add their toolbar buttons to the toolbar palette, rather than adding them directly to the toolbar. For more information about customizable toolbars, see Creating toolbar buttons.
Our find files example

Let's add a toolbar to the find files dialog. We don't really need one but we'll add one anyway to demonstrate its use. Two buttons will be added, an Open button and a Save button. Presumably, they would allow the user to save search results and re-open them later.

<vbox flex="1">
  <toolbox>
    <toolbar id="findfiles-toolbar">
      <toolbarbutton id="opensearch" label="Open"/>
      <toolbarbutton id="savesearch" label="Save"/>
    </toolbar>
  </toolbox>
  <tabbox>

Image:toolbar5.png

A toolbar with two buttons has been added here. In the image, you can see them appear horizontally along the top. Notice that the toolbar has been placed inside the vertical box just above the tabbox. This is because we need the vertical orientation so that the toolbar will appear above everything else.

The find files example so far: Source View

Next, we'll find out how to add a menu bar to a window.

« Previous
Next »
Document Tags and Contributors
Tags: 

    Tutorials XUL XUL_Tutorial 

Contributors to this page: Sheppy, teoli, fscholz, trevorh, Morishoji, Chbok, Enn, James0, Ptak82, Mgjbot, Pmash, Napolj2, Dria
Last updated by: Sheppy, Apr 14, 2014, 10:34:52 AM
Learn the best of web development

Get the latest and greatest from MDN delivered straight to your inbox.
E-mail
MDN Web Docs

    Web Technologies
    Learn Web Development
    About MDN
    Feedback

Mozilla

    About
    Contact Us
    Firefox

Other languages:

    Terms Privacy Cookies 

© 2005-2018 Mozilla and individual contributors.

Content is available under these licenses.


    Skip to main content
    Select language
    Skip to search

MDN Web Docs

    Technologies References & Guides Feedback 

    Sign in

Search

Edit

Simple Menu Bars

« Previous
Next »

In this section, we'll see how to create a menu bar with menus on it.
Creating a Menu

XUL has a number of different ways of creating menus. The most basic way is to add a menu bar with a row of menus on it like many applications have. You can also create popup menus. The menu features of XUL consist of a number of different elements which allow you to create menu bars or popup menus. The items on the menus can be customized quite easily. We've already seen part of how to make menus using the menulist. This section will build on that.

Menu bars are usually created much like a toolbar. The menu bar can optionally be placed inside a toolbox and the menu would work just like any other toolbar. XUL does have some special elements which provide special functionality typical of menus.

There are five elements associated with creating a menu bar and its menus, which are explained briefly here and in detail afterwards:

menubar
    The container for the row of menus.

menu
    Despite the name, this is actually only the title of the menu on the menubar. This element can be placed on a menubar or can be placed separately.

menupopup
    The popup box that appears when you click on the menu title. This box contains the list of menu commands.

menuitem
    An individual command on a menu. This would be placed in a menupopup.

menuseparator
    A separator bar on a menu. This would be placed in a menupopup.

You can customize the menus on the menubar to have whatever you want on them on all platforms except the Macintosh. This is because the Macintosh has its own special menu along the top of the screen controlled by the system. Although you can create custom menus, any special style rules or non-menu elements that you place on a menu may not be applied. You should keep this is mind when creating menus.
Example of a simple menu bar

Example 1 : Source View
Image:menubar-ex1.png

<toolbox flex="1">
  <menubar id="sample-menubar">
    <menu id="file-menu" label="File">
      <menupopup id="file-popup">
        <menuitem label="New"/>
        <menuitem label="Open"/>
        <menuitem label="Save"/>
        <menuseparator/>
        <menuitem label="Exit"/>
      </menupopup>
    </menu>
    <menu id="edit-menu" label="Edit">
      <menupopup id="edit-popup">
        <menuitem label="Undo"/>
        <menuitem label="Redo"/>
      </menupopup>
    </menu>
  </menubar>
</toolbox>

Here, a simple menu bar is created using the menubar element. It will create a row for menus to be placed on. Two menus, File and Edit have been created here. The menu element creates the title at the top of the menu, which appears on the menu bar. The popups are created using the menupopup element. It will pop up when the user clicks on the parent menu title. The size of the popup will be large enough to fit the commands inside it. The commands themselves are created using the menuitem element. Each one represents a single command on the menu popup.

You can also create separators on the menus using the menuseparator element. This is used to separate groups of menuitems.
menubar element

The menubar is a box containing menus. Note that it has been placed inside a flexible toolbox. The menubar has no special attributes but it is a type of box. This means that you could create a vertical menubar by setting the orient attribute to vertical.
menu element

The menu element works much like the button element. It accepts some of the same attributes plus some additional ones:

id
    The unique identifier of the menu title button.

label
    The text to appear on the menu, such as File or Edit.

disabled
    This boolean attribute determines whether the menu is disabled. Although you can, there's rarely a need to disable an entire menu. This attribute can be set to either true or false. Of course, the latter is the default.

accesskey
    This is the key that the user can press to activate the menu item. This letter is typically shown underlined on the menu title. Mozilla will look at the label attribute and add an underline character to the character specified here. For that reason, you should specify a character that exists in the text (although the key will still work if it doesn't).

Image:menubar-ex2.jpg

The menu element is normally placed on a menubar, although it does not have to be. However, it will be given a different look. The image here shows what the earlier example would look like without the menu bar.
menupopup element

The menupopup element creates the popup window containing the menu commands. It is a type of box which defaults to a vertical orientation. You could change it to horizontal if you wanted to and the menuitems would be placed in a row. Normally only menuitems and menuseparators would be placed on a menupopup. You can place any element on a menupopup, however they will be ignored on a Macintosh.
menuitem element

The menuitem element is much like the menu element and has some of the same attributes.

id
    The unique identifier of the menu item.

label
    The text to appear on the menu item, such as Open or Save.

disabled
    This boolean attribute determines whether the menu item is disabled. This attribute can be set to either true or false where the latter is the default.

accesskey
    This is the key that the user can press to activate the menu item. This letter is typically shown underlined on the menu title. Mozilla will look at the label attribute and add an underline character to the character specified here. For that reason, you should specify a character that exists in the text.

acceltext
    This specifies the shortcut key text to appear next to the menu command text. It does not associate a key action with the menuitem however. We'll look at how to do this later.

menuseparator element

The menuseparator has no special attributes. It just creates a horizontal bar between the menuitems next to it.

Next, we'll learn some more features of menus.

« Previous
Next »
Document Tags and Contributors
Tags: 

    Tutorials XUL XUL_Tutorial 

Contributors to this page: SphinxKnight, stephaniehobson, Sheppy, teoli, MattBrubeck, trevorh, inma_610, whongfei, Chbok, Enn, Morishoji, Ptak82, Mgjbot, Pmash, Btm, Napolj2, Dria
Last updated by: SphinxKnight, Feb 9, 2018, 11:05:43 AM
Learn the best of web development

Get the latest and greatest from MDN delivered straight to your inbox.
E-mail
MDN Web Docs

    Web Technologies
    Learn Web Development
    About MDN
    Feedback

Mozilla

    About
    Contact Us
    Firefox

Other languages:

    Terms Privacy Cookies 

© 2005-2018 Mozilla and individual contributors.

Content is available under these licenses.


    Skip to main content
    Select language
    Skip to search

MDN Web Docs

    Technologies References & Guides Feedback 

    Sign in

Search

Edit

More Menu Features

« Previous
Next »

In this section, we'll look at creating submenus and checked menus
Creating Submenus

You can create submenus inside other menus (nested menus) using the existing elements. Remember that you can put any element inside a menupopup. We've looked at placing menuitems and menuseparators in menupopups. However, you can create submenus by simply placing the menu element inside the menupopup element. This works because the menu element is valid even when it isn't directly placed inside a menu bar. The example below creates a simple submenu inside the File menu

Example 1 : Source View
Image:menubar-ex3.png

<toolbox flex="1">
  <menubar id="sample-menubar">
    <menu id="file-menu" label="File">
      <menupopup id="file-popup">
        <menu id="new-menu" label="New">
          <menupopup id="new-popup">
            <menuitem label="Window"/>
            <menuitem label="Message"/>
          </menupopup>
        </menu>
        <menuitem label="Open"/>
        <menuitem label="Save"/>
        <menuseparator/>
        <menuitem label="Exit"/>
      </menupopup>
    </menu>
  </menubar>
</toolbox>

Adding a menu to our Find Files example

Let's add a menu to the find files dialog. We'll just add a few simple commands to a File menu and an Edit menu. This is similar to the example above.

<toolbox>

 <menubar id="findfiles-menubar">
  <menu id="file-menu" label="File" accesskey="f">
    <menupopup id="file-popup">
      <menuitem label="Open Search..." accesskey="o"/>
      <menuitem label="Save Search..." accesskey="s"/>  
      <menuseparator/>
      <menuitem label="Close" accesskey="c"/>
    </menupopup>
  </menu>
  <menu id="edit-menu" label="Edit" accesskey="e">
    <menupopup id="edit-popup">
      <menuitem label="Cut" accesskey="t"/>
      <menuitem label="Copy" accesskey="c"/>
      <menuitem label="Paste" accesskey="p" disabled="true"/>
    </menupopup>
  </menu>
 </menubar>

<toolbar id="findfiles-toolbar>

Image:menubar1.png

Here we have added two menus with various commands on them. Notice how the menu bar was added inside the toolbox. The three dots after Open Search and Save Search are the usual way that you indicate to the user that a dialog will open when selecting the command. Access keys have been added for each menu and menu item. You will see in the image that this letter has been underlined in the menu label. Also, the Paste command has been disabled. We'll assume that there's nothing to paste.

Our Find files example so far : Source View
Adding Checkmarks to Menus

Many applications have menu items that have checks on them. For example, a feature that is enabled has a check placed beside the command and a feature that is disabled has no check. When the user selects the menu, the check state is switched. You may also want to create radio buttons on menu items.

The checks are created in a similar way to the checkbox and radio elements. This involves the use of two attributes: type to indicate the type of check, and name to group commands together. The example below creates a menu with a checked item.

Example 2 : Source View

<toolbox>
  <menubar id="options-menubar">
    <menu id="options_menu" label="Options">
      <menupopup>
        <menuitem id="backups" label="Make Backups" type="checkbox"/>
        <menuitem id="email" label="Email Administrator" type="checkbox" checked="true"/>
      </menupopup>
    </menu>
  </menubar>
</toolbox>

The type attribute has been added which is used to make the menu item checkable. By setting its value to checkbox, the menu item can be checked on and off by selecting the menu item.
Menu with radios

In addition to standard checks, you can create the radio style of checks by setting the type to a value of radio. A radio check is used when you want a group of menu items where only one item can be checked at once. An example might be a font menu where only one font can be selected at a time. When another item is selected, the previously selected item is unchecked.

In order to group a set of menu items together, you need to put a name attribute on each one to group. Set the value to the same string. The example below demonstrates this:

Example 3 : Source View

<toolbox>
  <menubar id="planets-menubar">
    <menu id="planet-menu" label="Planet">
      <menupopup>
        <menuitem id="jupiter" label="Jupiter" type="radio" name="ringed"/>
        <menuitem id="saturn" label="Saturn" type="radio" name="ringed" checked="true"/>
        <menuitem id="uranus" label="Uranus" type="radio" name="ringed"/>
        <menuseparator/>
        <menuitem id="earth" label="Earth" type="radio" name="inhabited" checked="true"/>
        <menuitem id="moon" label="Moon" type="radio" name="inhabited"/>
      </menupopup>
    </menu>
  </menubar>
</toolbox>

If you try this example, you'll find that of the first three menu items, only one can be checked. They are grouped together because they all have the same name. The last menu item, Earth, while a radio button, is not part of this group because it has a different name.

Of course, the grouped items all have to be within the same menu. They don't have to be placed next to each other in the menu, although it doesn't make as much sense if they aren't.

Next, we'll find out how to create popup menus.

« Previous
Next »
Document Tags and Contributors
Tags: 

    Tutorials XUL XUL_Tutorial 

Contributors to this page: Sheppy, teoli, fscholz, Peterko, trevorh, Chbok, Nathymig, Morishoji, Ptak82, Mgjbot, Pmash, Enn, Napolj2, Dria
Last updated by: Sheppy, Apr 14, 2014, 10:34:46 AM
Learn the best of web development

Get the latest and greatest from MDN delivered straight to your inbox.
E-mail
MDN Web Docs

    Web Technologies
    Learn Web Development
    About MDN
    Feedback

Mozilla

    About
    Contact Us
    Firefox

Other languages:

    Terms Privacy Cookies 

© 2005-2018 Mozilla and individual contributors.

Content is available under these licenses.


    Skip to main content
    Select language
    Skip to search

MDN Web Docs

    Technologies References & Guides Feedback 

    Sign in

Search

Edit

Popup Menus

« Previous
Next »

In the last section, we looked at creating a menu on a menu bar. XUL also has the capability of creating popup menus. Popup menus are typically displayed when the user presses the right mouse button.
Creating a Popup Menu

XUL has three different types of popups, described below. The main difference is the way in which they appear.

Plain Popups
    The plain popup is a popup window which appears when the user presses the left mouse button on an element. They are much like the menus on the menu bar, except that they can be placed anywhere and can contain any content. A good example is the drop down menu that appears when you click on the little down arrows next to the back and forward buttons in a browser window.
Context Popups
    The context popup is a popup window which appears when the user presses the context menu button, which is usually the right mouse button. The exact way that a context menu is opened varies on each platform. On the Macintosh for example, the user can either press the Control key and click the mouse button, or hold the mouse button down for a moment. Also note that it is possible to open context menus without using the mouse, for example by pressing the menu key on a keyboard.
Tooltips
    A tooltip popup window will appear when the user hovers over an element with the mouse. This type of popup is usually used to provide a description of a button in more detail than can be provided on the button itself.

All three types of popups differ in the way that the user invokes them. The type of popup is determined by the element that invokes the popup.
Declaring popup content

A popup is described using the menupopup element. It has no special attributes and is a type of box. When invoked, it will display a window containing whatever you put inside the menupopup. However, you should always put an id attribute on the menupopup as it is used to associate the popup with an element. We'll see what this means soon. First, an example:

<popupset>
  <menupopup id="clipmenu">
    <menuitem label="Cut"/>
    <menuitem label="Copy"/>
    <menuitem label="Paste"/>
  </menupopup>
</popupset>

As can be seen here, a simple popup menu with three commands on it has been created. The menupopup element surrounds the three menuitem elements. You will also notice that the id has been set on the menupopup element itself.

The popupset element surrounds the entire popup menu declaration. This is a generic container for popups, and is optional. It does not draw on screen but instead is used as a placeholder where you would declare all of your popups. As the name popupset implies, you can put multiple popup declarations inside it. Just add additional ones after the first menupopup element. You can have more than one popupset in a file, but usually you will have only one.
Associating a popup with an element

Now that we've created the popup, it's time to make the popup appear. To do this we need to associate the popup with an element where it should appear. We do this because we only want the popup to appear when the user clicks in a certain area of a window. Typically, this will be a specific button or a box.

To associate the popup with an element, you add one of three attributes to the element. The attribute you add depends on which type of popup you want to create. For plain popups, add a popup attribute to the element. For context popups, add a context attribute. Finally, for tooltip popups, add a tooltip attribute.

The value of the attribute must be set to the id of the menupopup that you want to have appear. This is why you must put the id on the popup. That way it's easy to have multiple menupopups in one file.

In the example above, we want to make the popup a context menu. That means that we need to use the context attribute and add it to the element which we want to have the popup associated with. The sample below shows how we might do this:

Example 1 : Source View
Image:popups-ex1.png

<popupset>
  <menupopup id="clipmenu">
    <menuitem label="Cut"/>
    <menuitem label="Copy"/>
    <menuitem label="Paste"/>
  </menupopup>
</popupset>

<box context="clipmenu">
  <label value="Context click for menu"/>
</box>

Here, the menupopup has been associated with a box. Whenever you context-click (right-click) anywhere inside the box, the popup menu will appear. The popup will also appear even if you click on the children of the box, so it will work if you click on the label element also. The context attribute has been used to associate the box with a menupopup with the same id. In this case, the menupopup clipmenu will appear. This way, you can have a number of popups and associate them with different elements.

You could associate multiple popups with the same element by putting more attributes of different types on an element. You could also associate the same popup with multiple elements, which is one advantage of using this popup syntax. Popups can only be associated with XUL elements; they cannot be associated with HTML elements.
Tooltips

We'll look at a simple way to create tooltips here. There are two ways to create a tooltip. The simplest way, which is much more common, is to add a tooltiptext attribute to an element for which you want to assign a tooltip.

The second method is to use a tooltip element containing the content of a tooltip. This requires you to have a separate block of content for each tooltip or to have a script which sets the content. However, it does allow you to use any content besides text in a tooltip.

Example 2 : Source View

<button label="Save" tooltiptext="Click here to save your stuff"/>

<popupset>
  <tooltip id="moretip" orient="vertical" style="background-color: #33DD00;">
    <description value="Click here to see more information"/>
    <description value="Really!" style="color: red;"/>
  </tooltip>
</popupset>

<button label="More" tooltip="moretip"/>

These two buttons each have a tooltip. The first uses the default tooltip style. The second uses a custom tooltip that has a different background color and styled text. The tooltip is associated with the More button using the tooltip attribute, which is set to the corresponding id of the tooltip element. Note that the tooltip element is still placed inside a popupset element like other popup types.
Popup Alignment

By default, the popup and context windows will appear where the mouse pointer is. Tooltips will be placed slightly below the element so that the mouse pointer does not obscure it. There are cases however, where you will want to indicate in more detail where the popup appears. For example, the popup menu that appears when you click the Back button in a browser should appear underneath the back button, not where the mouse pointer is.

To change the popup position, you can use an additional attribute, position, on the menupopup. You can also add it to the menupopup element. This attribute is used to indicate the placement of the popup relative to the element invoking the popup. It can be set to a number of values, which are described briefly below:

after_start
    The popup appears below the element with the left edges of the element and the popup window aligned. If the popup window is larger than the element, is extends to the right. This is the value used for the drop-down menus associated with the browser's Back and Forward buttons.
after_end
    The popup appears below the element with the right edges of the element and the popup window aligned.
before_start
    The popup appears above the element with the left edges of the element and the popup window aligned.
before_end
    The popup appears above the element with the right edges of the element and the popup window aligned.
end_after
    The popup appears to the right of the element with the bottom edges of the element and the popup window aligned.
end_before
    The popup appears to the right of the element with the top edges of the element and the popup window aligned.
start_after
    The popup appears to the left of the element with the bottom edges of the element and the popup window aligned.
start_before
    The popup appears to the left of the element with the top edges of the element and the popup window aligned.
overlap
    The popup appears on top of the element.
at_pointer
    The popup appears at the mouse pointer position (at_pointer is not currently implemented with respect to the mouse pointer. It currently appears at the top of the anchor).
after_pointer
    The popup appears at the same horizontal position as the mouse pointer but appears below the element. This is how tooltips appear (after_pointer is not currently implemented with respect to the mouse pointer. It currently offsets 21 pixels vertically from the anchor, see Bug 619887).

By adding the position attribute to a popup element, you can specify precisely where the popup appears. You cannot specify an exact pixel position. The position attribute can be used with all three popup types, although you probably would not change the value for tooltips. The example below demonstrates creating a back button with a popup menu:

Example 3 : Source View

<popupset>
  <menupopup id="backpopup" position="after_start">
    <menuitem label="Page 1"/>
    <menuitem label="Page 2"/>
  </menupopup>
</popupset>

<button label="Pop Me Up" popup="backpopup"/>

Our find files example

Let's add a simple popup menu to the find files dialog. For simplicity, we'll just replicate the contents of the Edit menu. Let's have the popup appear when clicking over the first tab panel:

<popupset>
  <menupopup id="editpopup">
    <menuitem label="Cut" accesskey="t"/>
    <menuitem label="Copy" accesskey="c"/>
    <menuitem label="Paste" accesskey="p" disabled="true"/>
  </menupopup>
</popupset>

<vbox flex="1">
.
.
.

<tabpanel id="searchpanel" orient="vertical" context="editpopup">

Here a simple popup that is similar to the edit menu has been added to the first tabpanel. If you right-click (Control-click on the Macintosh) anywhere on the first panel, the popup will appear. However, the popup will not appear if you click anywhere else. Note that the textbox has its own built-in popup menu which will override the one we specified.

Our Find files example so far : Source View

Next, we'll look at how to create scrolling menus.

« Previous
Next »
Document Tags and Contributors
Tags: 

    Tutorials XUL XUL_Tutorial 

Contributors to this page: gportioli, Sheppy, teoli, fscholz, CheekySp, trevorh, Chbok, Enn, Kray2, Nathymig, Morishoji, Ptak82, Mgjbot, Pmash, Napolj2, Dria
Last updated by: gportioli, Aug 8, 2015, 8:59:17 AM
Learn the best of web development

Get the latest and greatest from MDN delivered straight to your inbox.
E-mail
MDN Web Docs

    Web Technologies
    Learn Web Development
    About MDN
    Feedback

Mozilla

    About
    Contact Us
    Firefox

Other languages:

    Terms Privacy Cookies 

© 2005-2018 Mozilla and individual contributors.

Content is available under these licenses.


    Skip to main content
    Select language
    Skip to search

MDN Web Docs

    Technologies References & Guides Feedback 

    Sign in

Search

Edit

Scrolling Menus

« Previous
Next »

This section will describe scrolling menus and how to use the mechanism with other elements.
Creating a Large Menu

You might wonder what happens if you create a menu with a lot of commands on it, such that all the items won't fit on the screen at once. Mozilla will provide a scrolling mechanism that will allow you to scroll through the items.
Image:menuscroll1.png

If the available space is too small, arrows will appear on each end of the menu. If you move the mouse over the arrows, the menu will scroll up and down. If the available space is large enough, the arrows will not appear. Note that the exact behavior of the scrolling will depend on the current theme.

This behavior is automatic. You do not have to do anything in order to get scrolling menus. It will apply to menus on menubars, in popups or menulists. It is implemented using an arrowscrollbox element. This element can be used to create a scrolling box with arrows.

The arrowscrollbox can be used anywhere a regular box can be used. You don't have to use it in menus. It is always a vertical box and may contain any elements inside it. You could use it to implement a list when you don't want it to be a drop-down.
Example - scrolling list of buttons

The following example shows how to create a scrolling list of buttons (you will need to resize the window to see the arrow buttons):

Example 1 : Source View

<arrowscrollbox orient="vertical" flex="1">
  <button label="Red"/>
  <button label="Blue"/>
  <button label="Green"/>
  <button label="Yellow"/>
  <button label="Orange"/>
  <button label="Silver"/>
  <button label="Lavender"/>
  <button label="Gold"/>
  <button label="Turquoise"/>
  <button label="Peach"/>
  <button label="Maroon"/>
  <button label="Black"/>
</arrowscrollbox>

If you try this example, it will first open at full size. However, if you shrink the height of the window, the scroll arrows will appear. Making the window larger again will cause the arrows to disappear.

You can set a CSS max-height property on the arrowscrollbox to limit the size of the scrolling box and thus make the arrows appear all the time.

The arrowscrollbox is mainly useful in menus and popups however.

Next, we'll see how to add some event handlers to XUL elements.

« Previous
Next »
Document Tags and Contributors
Tags: 

    Tutorials XUL XUL_Tutorial 

Contributors to this page: Sheppy, teoli, fscholz, trevorh, Chbok, Nathymig, Morishoji, Ptak82, Mgjbot, Pmash, Dria
Last updated by: Sheppy, Apr 14, 2014, 10:34:51 AM
Learn the best of web development

Get the latest and greatest from MDN delivered straight to your inbox.
E-mail
MDN Web Docs

    Web Technologies
    Learn Web Development
    About MDN
    Feedback

Mozilla

    About
    Contact Us
    Firefox

Other languages:

    Terms Privacy Cookies 

© 2005-2018 Mozilla and individual contributors.

Content is available under these licenses.


    Skip to main content
    Select language
    Skip to search

MDN Web Docs

    Technologies References & Guides Feedback 

    Sign in

Search

Edit

Adding Event Handlers

« Previous
Next »

The find files dialog so far looks quite good. We haven't cleaned it up much but we have created a simple user interface easily. Next, we will show how to add scripts to it.
Using Scripts

To make the find files dialog functional, we need to add some scripts which will execute when the user interacts with the dialog. We would want to add a script to handle the Find button, the Cancel button and to handle each menu command. We write this using JavaScript functions much in the same way as HTML.

You can use the script element to include scripts in XUL files. You can embed the script code directly in the XUL file in between the opening and closing script tags but it is much better to include code in a separate file as the XUL window will load slightly faster. The src attribute is used to link in an external script file.
Our find files example

Let's add a script to the find file dialog. Although it does not matter what the script file is called, usually it would be the same as the XUL file with a .js extension. In this case, findfile.js will be used. Add the line below just after the opening window tag and BEFORE any elements.

<script src="findfile.js"/>

We'll create the script file later when we know what we want to put in it. We'll define some functions in the file and we can call them in event handlers.

You can include multiple scripts in a XUL file by using multiple script tags, each pointing to a different script. You may use relative or absolute URLs. For example, you may use URLs of the following form:

<script src="findfile.js"/>
<script src="chrome://findfiles/content/help.js"/>
<script src="http://www.example.com/js/items.js"/>

This tutorial does not attempt to describe how to use JavaScript (except as related to event handling) as this is a fairly large topic and there are plenty of other resources that are available for this.
By default the JavaScript console only shows errors from web content. To show errors in chrome JavaScript, it is necessary to change the preference javascript.options.showInConsole to true. You can also change the preference javascript.options.strict for debugging. By setting this value to true, non-standard, poorly written, or syntax prone to cause logic errors are logged to the JavaScript console.
Responding to Events

The script will contain code which responds to various events triggered by the user or other situations. There are about thirty or so different events that may be handled in several different ways. A typical event is the user pressing a mouse button or pressing a key. Each XUL element has the ability to trigger certain events in different situations. Some events are triggered only by certain elements.

Each event has a name, for example, 'mousemove' is the name of the event that is triggered when the user moves the mouse over a UI element. XUL uses the same event mechanism as defined by DOM Events. When an action occurs that would trigger an event, such as the user moving the mouse, an event object is created corresponding to that event type. Various properties are set on the event object such as the mouse position, the key that was pressed, and so forth.

The event is then sent to the XUL in phases.

    In the capturing phase, the event is first sent to the window, then to the document, followed by each ancestor of the XUL element where the event occured downwards until it reaches that element.
    In the target phase, the event is sent to the target XUL element.
    In the bubbling phase, the event is sent to each element back upwards until it reaches the window again.

You can respond to an event during either the capturing or bubbling phase. Once the event has finished propagating, any default action will occur, which is the built in behaviour of the element.

For example, when the mouse is moved over a button that is inside a box, a 'mousemove' event is generated, and sent first to the window, followed by the document, and then the box. That completes the capturing phase. Next, the 'mousemove' event is sent to the button. Finally, the bubbling phase causes the event to be sent to the box, document and window. The bubbling phase is essentially the reverse of the capturing phase. Note that some events don't do the bubbling phase.

You can attach listeners to each element to listen to the events during each step of event propagation. Due to the way a single event is passed to all the ancestors, you may attach a listener to a specific element or to an element higher in the hierarchy. Naturally, an event attached to an element higher up will receive notification of all elements inside it, whereas an event attached to a button will only receive events pertaining to that button. This is useful if there are several elements you would like to handle using the same or similar code.

The most common event used is the 'command' event. The command event is fired when a user activates an element, for example by pressing a button, changing a checkbox or selecting an item from a menu. The command event is a useful event since it automatically handles different ways of activating the element. For example, the command event will occur regardless of whether the user uses the mouse to click a button, or presses the Enter key.

There are two ways to attach an event listener to an element. First, by using an attribute with a script as its value. Second, by calling an element's addEventListener method. The former may only handle bubbling events but tends to be simpler to write. The latter can handle events at any phase and may also be used to attach multiple listeners for an event to an element. Using the attribute form is more common for most events.
Attribute Event Listeners

To use the attribute form, place an attribute on the element where you want the event listener to be, the name of which should be the event name preceded by the word 'on'. For example, the corresponding attribute for the 'command' event is 'oncommand'. The value of the attribute should be some script that should be executed when the event occurs. Typically, this code will be short or just call a function defined in a separate script. An example of responding to a button being pressed:

Example 1 : Source View

<button label="OK" oncommand="alert('Button was pressed!');"/>

Since the command event will bubble, it is also possible to place the event listener on an enclosing element. In the example below, the listener has been placed on a box and will receive events for both elements.

Example 2 : Source View

<vbox oncommand="alert(event.target.tagName);">
  <button label="OK"/>
  <checkbox label="Show images"/>
</vbox>

In this example, the command event will bubble up from the button or checkbox to the vbox, where it is handled. If a second listener (the oncommand attribute) were placed on the button, its code will be called first, followed by the handler on the vbox. Event handlers are passed the event object as an implied argument called 'event'. This is used to get specific information about the event. One commonly used property is the 'target' property of the event, which holds the element where the event actually occured. In the example we display an alert containing the target's tag name. The target is useful when using a bubbling event so that you could have a set of buttons which are all handled by a single script.

You might notice that the attribute syntax is similar to that used for events in HTML documents. In fact, both HTML and XUL share the same event mechanism. One important difference is that while the 'click' event (or the onclick attribute) is used in HTML to respond to buttons, in XUL the command event should be used instead. XUL does have a click event, but it only responds to mouse clicks, not to keyboard usage. Thus, the click event should be avoided in XUL, unless you have a reason to have an element that can only be handled with a mouse. In addition, whereas the command event will not be sent if an element is disabled, the click event will be sent regardless of whether the element is disabled or not.
Our find files example

A command handler can be placed on the Find and Cancel buttons in the find files dialog. Pressing the Find button should start the search. Because we aren't going to implement this part yet, we'll leave it out for now. However, pressing the Cancel button should close the window. The code below shows how to do this. While we're at it, let's add the same code to the Close menu item.

<menuitem label="Close" accesskey="c" oncommand="window.close();"/>
...
  
<button id="cancel-button" label="Cancel"
     oncommand="window.close();"/>

Two handlers have been added here. The oncommand attribute was added to the Close menu item. By using this handler, the user will be able to close the window by clicking the menu item with the mouse or by selecting it with the keyboard. The oncommand handler was also added to the Cancel button.
DOM Event Listeners

The second way to add an event handler is to call an element's addEventListener method. This allows you to attach an event listener dynamically and listen for events during the capturing phase. The syntax is as follows:

Example 3 : Source View

<button id="okbutton" label="OK"/>

<script>
function buttonPressed(event){
  alert('Button was pressed!');
}

var button = document.getElementById("okbutton");
button.addEventListener('command', buttonPressed, true);
</script>

The getElementById() function returns the element with a given id, in this case the button. The addEventListener() function is called to add a new capturing event listener. The first argument is the name of the event to listen to. The second argument is the event listener function which will be called when the event occurs. Finally, the last argument should be true for capturing listeners. You can also listen during the bubbling phase by setting the last argument to false. The event listener function passed as the second argument should take one argument, the event object, as shown in the declaration for the buttonPressed function above.
Find files example so far : Source View

 

Next, we'll look at some more details about the event object.

« Previous
Next »
Document Tags and Contributors
Tags: 

    Tutorials XUL XUL_Tutorial 

Contributors to this page: stephaniehobson, Sheppy, teoli, fscholz, trevorh, LJR, Brettz9, Chbok, Mgjbot, James0, Enn, Morishoji, Perlmonkee, Jjinux, Pmash, Ptak82, Dria
Last updated by: stephaniehobson, Oct 21, 2015, 10:31:06 AM
Learn the best of web development

Get the latest and greatest from MDN delivered straight to your inbox.
E-mail
MDN Web Docs

    Web Technologies
    Learn Web Development
    About MDN
    Feedback

Mozilla

    About
    Contact Us
    Firefox

Other languages:

    Terms Privacy Cookies 

© 2005-2018 Mozilla and individual contributors.

Content is available under these licenses.


    Skip to main content
    Select language
    Skip to search

MDN Web Docs

    Technologies References & Guides Feedback 

    Sign in

Search

Edit

More Event Handlers

« Previous
Next »

In this section, the event object is examined and additional events are described.
The Event Object

Every event handler is passed an event object. In the attribute form of the event handler, the event object is an implied argument to the script code which can be referred to using the name 'event'. In the addEventListener form, the first argument passed to the listener function will be the event object.  Other arguments can be passed to a listener function, if required.

 

The event object has a number of properties which can be examined during an event. For a full list, see the reference.

We already saw the event's target property in the last section. It holds a reference to the element where the event occurred. A similar property currentTarget holds the element that is currently having its event listeners handled. In the example below, currentTarget is always the vbox, whereas target would be the specific element, either the button or checkbox, that was activated.

Example 1 : Source View

<vbox oncommand="alert(event.currentTarget.tagName);">
  <button label="OK"/>
  <checkbox label="Show images"/>
</vbox>

Stop Event Propagation

Once you handle an event, regardless of where in the propagation the event is, you will likely want to stop the event from being sent to further elements, essentially stopping the capturing or bubbling phases from continuing. Depending on how you attach the event listener to an element, there are different ways of doing this.

Recall that the capturing phase occurs before the bubbling phase, so any capturing listeners will trigger before any bubbling listeners. If a capturing event stops the event propagation, none of the later capturing listeners, nor any of the bubbling listeners will ever receive notification about the events. To manually stop event propagation, call the event object's stopPropagation method, as in the following example.

Example 2 : Source View

<hbox id="outerbox">
  <button id="okbutton" label="OK"/>
</hbox>

<script>
function buttonPressed(event){
  alert('Button was pressed!');
}

function boxPressed(event){
  alert('Box was pressed!');
  event.stopPropagation();
}

var button = document.getElementById("okbutton");
button.addEventListener('command',buttonPressed,true);

var outerbox = document.getElementById("outerbox");
outerbox.addEventListener('command',boxPressed,true);
</script>

Here, an event listener has been added to the button and another event listener has been added to the box. The stopPropagation method has been called in the box's listener, so the button's listener never gets called. If this call was removed, both listeners would be called and both alerts would appear.
Prevent Default Action

If no event handlers have been registered for an event, then after completing the capturing and bubbling phases, the element will handle the event in a default way. What will happen depends on the event and the type of element. For example, the 'popupshowing' event is sent to a popup just before it is displayed. The default action is to display the popup. If the default action is prevented, the popup will not be displayed. The default action can be prevented with the event object's preventDefault method, as in the example below.

Example 3 : Source View

<button label="Types" type="menu">
  <menupopup onpopupshowing="event.preventDefault();">
    <menuitem label="Glass"/>
    <menuitem label="Plastic"/>
  </menupopup>
</button>

Alternatively, for attribute event listeners, you can just return false from the code. Note that preventing the default action is not the same as stopping event propagation with the stopPropagation method. Even if the default action has been prevented, the event will still continue to propagate. Similarly, calling the stopPropagation method won't prevent the default action. You must call both methods to stop both from occuring.

Note that once propagation or the default action has been prevented, neither may be re-enabled again for that event.

The following sections list some of the events that may be used. A full list is provided in the XULPlanet event reference.
Mouse Events

There are several events which can be used to handle mouse specific actions, which are described briefly below:

click
    Called when the mouse is pressed and released on an element.
dblclick
    Called when the a mouse button is double clicked.
mousedown
    Called when a mouse button is pressed down on an element. The event handler will be called as soon as a mouse button is pressed, even if it hasn't been released yet.
mouseup
    Called when a mouse button is released on an element.
mouseover
    Called when the mouse pointer is moved onto an element. You could use this to highlight the element, however CSS provides a way to do this automatically so you shouldn't do it with an event. You might, however, want to display some help text on a status bar.
mousemove
    Called when the mouse pointer is moved while over an element. The event may be called many times as the user moves the mouse so you should avoid performing lengthy tasks from this handler.
mouseout
    Called when the mouse pointer is moved off of an element. You might then unhighlight the element or remove status text.

There are also a set of drag related events, which occur when the user holds down a mouse button and drags the mouse around. Those events are described in Drag and Drop.
Mouse Button Event Properties

When a mouse button event occurs, a number of additional properties are available to determine which mouse buttons were pressed and the location of the mouse pointer. The event's button property can be used to determine which button was pressed, where possible values are 0 for the left button, 1 for the middle button and 2 for the right button. If you've configured your mouse differently, these values may be different.

The detail property holds the number of times the button has been clicked quickly in sequence. This allows you to check for single, double or triple clicks. Of course, if you just want to check for double clicks, you can also use the dblclick event instead. The click event will be fired once for the first click, again for the second click, and again for the third click, but the dblclick event will only be fired once for a double click.

The button and detail properties only apply to the mouse button related events, not mouse movement events. For the mousemove event, for example, both properties will be set to 0.
Mouse Position Event Properties

However, all mouse events will be supplied with properties that hold the coordinates of the mouse position where the event occurred. There are two sets of coordinates. The first is the screenX and screenY properties and are relative to the top left corner of the screen. The second set, clientX and clientY, are relative to the top left corner of the document. Here is an example which displays the current mouse coordinates:

Example 4 : Source View

<script>

function updateMouseCoordinates(e){
  var text = "X:" + e.clientX + " Y:" + e.clientY;
  document.getElementById("xy").value = text;
}
</script>

<label id="xy"/>
<hbox width="400" height="400" onmousemove="updateMouseCoordinates(event);"/>

In this example, the size of the box has been set explicitly so the effect is easier to see. The event handler gets the clientX and clientY properties and creates a string from them. This string is then assigned to the value property of the label. Note that the event argument must be passed to the updateMouseCoordinates function. If you move the mouse quickly across the border of the box, you might notice that the coordinates don't generally stop right at 400. This is because the mousemove events occur at intervals depending on the speed at which the mouse moves and the mouse is usually moved some distance past the border by the time the next event fires. Obviously, it would be much too inefficient to send a mousemove event for every pixel the mouse is moved.
Element Relative Coordinates

You will often want to get the coordinates of an event relative to the element where the event occurred rather than the entire window. You can do this by subtracting the element's position from the event position, as in the following code.

var element = event.target;
var elementX = event.clientX - element.boxObject.x;
var elementY = event.clientY - element.boxObject.y;

XUL elements have a box object that can be retrieved using the boxObject property. We'll learn more about the box object in a later section, but it holds information pertaining to how the element is displayed, including the x and y position of the element. In this example code, these coordinates are subtracted from the event coordinates to get the event position relative to the element.
Load Events

The load event is sent to the document (the window tag) once the XUL file has finished loading and just before the content is displayed. This event is commonly used to initialize fields and perform other tasks that need to be done before the user can use the window. You should use a load event to do these kinds of things as opposed to adding a script at the top level outside of a function. This is because the XUL elements may not have loaded or fully initialized yet, so some things may not work as expected. To use a load event, place an onload attribute on the window tag. Call code within the load handler which will initialize the interface as necessary.

There is also an unload event which is called once the window has closed, or in a browser context, when the page is switched to another URL. You can use this event to save any changed information, for example.


Next, we'll find out how to add keyboard shortcuts.

« Previous
Next »
Document Tags and Contributors
Tags: 

    Tutorials XUL XUL_Tutorial 

Contributors to this page: bjori, Sheppy, jukbot, teoli, fscholz, Allasso, trevorh, kiteroa, Enn, Brettz9, Lego, Fredchat, Jomel, Mixticius, Chbok, James0, Nathymig, Morishoji, Holbech, Mgjbot, Pmash, Laolu, Ptak82, Dria
Last updated by: bjori, Dec 23, 2014, 3:53:00 PM
Learn the best of web development

Get the latest and greatest from MDN delivered straight to your inbox.
E-mail
MDN Web Docs

    Web Technologies
    Learn Web Development
    About MDN
    Feedback

Mozilla

    About
    Contact Us
    Firefox

Other languages:

    Terms Privacy Cookies 

© 2005-2018 Mozilla and individual contributors.

Content is available under these licenses.


    Skip to main content
    Select language
    Skip to search

MDN Web Docs

    Technologies References & Guides Feedback 

    Sign in

Search

Edit

Keyboard Shortcuts

« Previous
Next »

You could use keyboard event handlers to respond to the keyboard. However, it would be tedious to do that for every button and menu item (though it could be necessary when one's key commands are only triggered when the user is focused on a particular element).
Creating a Keyboard Shortcut

XUL provides methods in which you can define keyboard shortcuts. We've already seen in the section on menus that we can define an attribute called accesskey which specifies the key which a user can press to activate the menu or menu item. In the example below, the File menu can be selected by pressing Alt + F together (or some other key combination for a specific platform). Once the File menu is open, the Close menu item can be selected by pressing C.

Example 1 : Source View

<menubar id="sample-menubar">
  <menu id="file-menu" label="File" accesskey="f">
    <menupopup id="file-popup">
      <menuitem id="close-command" label="Close" accesskey="c"/>
    </menupopup>
  </menu>
</menubar>

You can also use the accesskey attribute on buttons. When the key is pressed in this case, the button is selected.

You might want to set up more general keyboard shortcuts however. For example, pressing Ctrl + C to copy text to the clipboard. Although shortcuts such as this might not always be valid, they will usually work any time the window is open. Usually, a keyboard shortcut will be allowed at any time and you can check to see whether it should do something using a script. For example, copying text to the clipboard should only work when some text is selected.
Key element

XUL provides an element, key, which lets you define a keyboard shortcut for a window. It has attributes to specify the key that should be pressed and what modifier keys (such as Shift or Ctrl) need to be pressed. An example is shown below:

<keyset>
  <key id="sample-key" modifiers="shift" key="R"/>
</keyset>

This sample defines a keyboard shortcut that is activated when the user presses the Shift key and R. The key attribute (note that it has the same name as the element itself) can be used to indicate which key should be pressed, in this case R. You could add any character for this attribute to require that key to be pressed. The modifiers that must be pressed are indicated with the modifiers attribute. It is a space-separated list of modifier keys, which are listed below.

alt
    The user must press the Alt key. On the Macintosh, this is the Option key.
control
    The user must press the Ctrl key.
meta
    The user must press the Meta key. This is the Command key on the Macintosh.
shift
    The user must press the Shift key.
os
    The user must press the Win key. This is the Super key or the Hyper key on Linux. If this value is used, typically the key combination conflicts with system wide shortcut keys. So, you shouldn't use this value as far as possible. 
accel
    The user must press the special accelerator key. The key used for keyboard shortcuts on the user's platform. Usually, this would be the value you would use.
access
    The user must press the special access key. The key used for access keys on the user's platform.

Your keyboard won't necessarily have all of the keys, in which case they will be mapped to modifier keys that you do have.

The key element must be placed inside a keyset element. This element is designed for holding a set of key elements, which serves to group all of the key definitions in one place in a file. Any key elements outside of a keyset element will not work.

Each platform generally uses a different key for keyboard shortcuts. For example, Windows uses the Control key and the Macintosh uses the Command key. It would be inconvenient to define separate key elements for each platform. Luckily, there is a solution. The modifier accel refers to the special platform-specific key used for shortcuts. It works just like the other modifiers, but won't be the same on every platform.

Here are some additional examples:

<keyset>
  <key id="copy-key" modifiers="control" key="C"/>
  <key id="explore-key" modifiers="control alt" key="E"/>
  <key id="paste-key" modifiers="accel" key="V"/>
</keyset>

Keycode attribute

The key attribute is used to specify the key that must be pressed. However, there will also be cases where you want to refer to keys that cannot be specified with a character (such as the Enter key or the function keys). The key attribute can only be used for printable characters. Another attribute, keycode can be used for non-printable characters.

The keycode attribute should be set to a special code which represents the key you want. A table of the keys is listed below. Not all of the keys are available on all keyboards.
VK_CANCEL 	VK_BACK 	VK_TAB 	VK_CLEAR
VK_RETURN 	VK_ENTER 	VK_SHIFT 	VK_CONTROL
VK_ALT 	VK_PAUSE 	VK_CAPS_LOCK 	VK_ESCAPE
VK_SPACE 	VK_PAGE_UP 	VK_PAGE_DOWN 	VK_END
VK_HOME 	VK_LEFT 	VK_UP 	VK_RIGHT
VK_DOWN 	VK_PRINTSCREEN 	VK_INSERT 	VK_DELETE
VK_0 	VK_1 	VK_2 	VK_3
VK_4 	VK_5 	VK_6 	VK_7
VK_8 	VK_9 	VK_SEMICOLON 	VK_EQUALS
VK_A 	VK_B 	VK_C 	VK_D
VK_E 	VK_F 	VK_G 	VK_H
VK_I 	VK_J 	VK_K 	VK_L
VK_M 	VK_N 	VK_O 	VK_P
VK_Q 	VK_R 	VK_S 	VK_T
VK_U 	VK_V 	VK_W 	VK_X
VK_Y 	VK_Z 	VK_NUMPAD0 	VK_NUMPAD1
VK_NUMPAD2 	VK_NUMPAD3 	VK_NUMPAD4 	VK_NUMPAD5
VK_NUMPAD6 	VK_NUMPAD7 	VK_NUMPAD8 	VK_NUMPAD9
VK_MULTIPLY 	VK_ADD 	VK_SEPARATOR 	VK_SUBTRACT
VK_DECIMAL 	VK_DIVIDE 	VK_F1 	VK_F2
VK_F3 	VK_F4 	VK_F5 	VK_F6
VK_F7 	VK_F8 	VK_F9 	VK_F10
VK_F11 	VK_F12 	VK_F13 	VK_F14
VK_F15 	VK_F16 	VK_F17 	VK_F18
VK_F19 	VK_F20 	VK_F21 	VK_F22
VK_F23 	VK_F24 	VK_NUM_LOCK 	VK_SCROLL_LOCK
VK_COMMA 	VK_PERIOD 	VK_SLASH 	VK_BACK_QUOTE
VK_OPEN_BRACKET 	VK_BACK_SLASH 	VK_CLOSE_BRACKET 	VK_QUOTE
VK_HELP 	  	  	 

For example, to create a shortcut that is activated when the user presses Alt and F5, do the following:

<keyset>
  <key id="test-key" modifiers="alt" keycode="VK_F5"/>
</keyset>

The example below demonstrates some more keyboard shortcuts:

<keyset>
  <key id="copy-key" modifiers="accel" key="C"/>
  <key id="find-key" keycode="VK_F3"/>
  <key id="switch-key" modifiers="control alt" key="1"/>
</keyset>

The first key is invoked when the user presses their platform-specific shortcut key and C. The second is invoked when the user presses F3. The third is invoked on a press of the Ctrl key, the Alt key and 1. If you wanted to distinguish between keys on the main part of the keyboard and the numeric keypad, use the VK_NUMPAD keys (such as VK_NUMPAD1).

Refer to the Mozilla Keyboard Planning FAQ and Cross Reference for more information about selecting keyboard shortcuts to use in applications.
Using the Keyboard Shortcuts

Now that we know how to define keyboard shortcuts, we'll find out how we can use them. There are two ways.

The first way is the simplest and just requires that you use the command event handler on the key element. When the user presses the key, the script will be invoked. An example is shown below:

<keyset>
  <key id="copy-key" modifiers="accel" key="C" oncommand="DoCopy();"/>
</keyset>

The function DoCopy will be called when the user presses the keys specified by the key element, which in this example, are the keys for copying to the clipboard (such as Control + C). This will work as long as the window is open. The DoCopy function should check to see if text is selected and then copy the text to the clipboard. Note that textboxes have the clipboard shortcuts built-in so you don't have to implement them yourself.
Assigning a keyboard shortcut on a menu

The second way is: If you are assigning a keyboard shortcut that performs a command that also exists on a menu, you can associate the key element directly with the menu command. To do this, also add a key attribute on the menuitem. Set its value to the id of the key that you want to use. The example below demonstrates this.

Example 2 : Source View
Image:keyshort1.jpg

<keyset>
  <key id="paste-key" modifiers="accel" key="V"
          oncommand="alert('Paste invoked')"/>
</keyset>

<menubar id="sample-menubar">
  <menu id="edit-menu" label="Edit" accesskey="e">
    <menupopup id="edit-popup">
      <menuitem id="paste-command" 
         accesskey="p" key="paste-key" 
         label="Paste" oncommand="alert('Paste invoked')"/>
    </menupopup>
  </menu>
</menubar>

The menuitem's key attribute, which here is paste-key is equal to the id of the defined key. You can use this for additional keys as well to define keyboard shortcuts for any number of menu items.

You'll also notice in the image that text has been placed next to the Paste menu command to indicate that Ctrl and the V key can be pressed to invoke the menu command. This is added for you based on the modifiers on the key element. Keyboard shortcuts attached to menus will work even if the menu is not open.

One additional feature of key definitions is that you can disable them easily. To do this add a disabled attribute to the key element and set it to the value true. This disables the keyboard shortcut so that it cannot be invoked. It is useful to change the disabled attribute using a script.

 
Our find files example

Let's add keyboard shortcuts to the find files dialog. We'll add four of them, one for each of the Cut, Copy, and Paste commands and also one for the Close command when the user presses Esc.

<?xml version="1.0" encoding="utf-8"?>
<window xmlns="http://www.mozilla.org/keymaster/gat...re.is.only.xul">
 <keyset>
   <key id="cut_cmd" modifiers="accel" key="X"/>
   <key id="copy_cmd" modifiers="accel" key="C"/>
   <key id="paste_cmd" modifiers="accel" key="V"/>
   <key id="close_cmd" keycode="VK_ESCAPE" oncommand="window.close();"/>
 </keyset>

 <vbox flex="1">
  <toolbox>
   <menubar id="findfiles-menubar">
    <menu id="file-menu" label="File" accesskey="f">
     <menupopup id="file-popup">
      <menuitem label="Open Search..." accesskey="o"/>
      <menuitem label="Save Search..." accesskey="s"/>
      <menuseparator/>
      <menuitem label="Close" accesskey="c" key="close_cmd" 
       oncommand="window.close();"/>
     </menupopup>
    </menu>
    <menu id="edit-menu" label="Edit" accesskey="e">
     <menupopup id="edit-popup">
      <menuitem label="Cut" accesskey="t" key="cut_cmd"/>
      <menuitem label="Copy" accesskey="c" key="copy_cmd"/>
      <menuitem label="Paste" accesskey="p" key="paste_cmd" disabled="true"/>
     </menupopup>
    </menu>
   </menubar>
  </toolbox>
 </vbox>
</window>

Now we can use those shortcuts to activate the commands. Of course, the clipboard commands don't do anything anyway, as we haven't written those scripts.
Key Events

There are three keyboard events that may be used if the key related features described above aren't suitable. These events are:

keypress
    Called when a key is pressed and released when an element has the focus. You might use this to check for allowed characters in a field.
keydown
    Called when a key is pressed down while an element has the focus. Note that the event will be called as soon as the key is pressed, even if it hasn't been released yet.
keyup
    Called when a key is released while an element has the focus.

Key events are only sent to the element that has the focus. Typically, this will include textboxes, buttons, checkboxes and so forth. If no element is focused, the key event will instead be targeted at the XUL document itself. In this case, you can add an event listener to the window tag. Normally though, if you want to respond to keys globally, you will use a keyboard shortcut as described earlier.

The key event object has two properties which hold the key that was pressed. The keyCode property holds the key code and may be compared to one of the constants from the key table earlier in this section. The charCode is used for printable characters and will hold the numeric Unicode character code value for the key that was pressed.
Our Find files example so far : Source View

Next, we'll find out how to handle focus and the selection.

« Previous
Next »
Document Tags and Contributors
Tags: 

    Tutorials XUL XUL_Tutorial 

Contributors to this page: SphinxKnight, arshdkhn1, stephaniehobson, bassam, maybe, Sheppy, teoli, Masayuki, trevorh, Yadra, LJR, Sspitzer, Kennykaiyinyu, Brettz9, Chbok, Nathymig, Marcellino, Morishoji, Bent, Mgjbot, Pmash, Enn, Ptak82, Dria
Last updated by: SphinxKnight, Feb 9, 2018, 11:07:16 AM
Learn the best of web development

Get the latest and greatest from MDN delivered straight to your inbox.
E-mail
MDN Web Docs

    Web Technologies
    Learn Web Development
    About MDN
    Feedback

Mozilla

    About
    Contact Us
    Firefox

Other languages:

    Terms Privacy Cookies 

© 2005-2018 Mozilla and individual contributors.

Content is available under these licenses.


    Skip to main content
    Select language
    Skip to search

MDN Web Docs

    Technologies References & Guides Feedback 

    Sign in

Search

Edit

Focus and Selection

« Previous
Next »

The section will describe how to handle the focus and selection of elements.
Focused Elements

The focused element refers to the element which currently receives input events. If there are three textboxes on a window, the one that has the focus is the one that the user can currently enter text into. Only one element per window has the focus at a time.

The user can change the focus by clicking an element with the mouse or by pressing the TAB key. When the TAB key is pressed, the next element is given the focus. To step backwards, the Shift key and Tab key can be pressed.
Rearranging the tab order

You can change the order in which elements are focused when the user presses the TAB key by adding a tabindex attribute to an element. This attribute should be set to a number. When the user presses TAB, the focus will shift to the element with the next highest tab index. That means that you can order the elements by setting indices on elements in sequence. Usually, however, you would not set the tabindex attribute. If you do not, pressing TAB will set the focus to the next displayed element. You only need to set tab indices if you wish to use a different order. Here is an example:

Example 1 : Source View

<button label="Button 1" tabindex="2"/>
<button label="Button 2" tabindex="1"/>
<button label="Button 3" tabindex="3"/>

The focus event

The focus event is used to respond when the focus is given to an element. The blur event is used to respond when the focus is removed from an element. You can respond to focus changes by adding an onfocus or onblur attribute on an element. They work just like their HTML counterparts. You might use these event handlers to highlight the element or display text on a status bar. The following example can be used to apply a function to handle a focus event.

Example 2 : Source View

<script>

function displayFocus(){
  var elem=document.getElementById('sbar');
  elem.setAttribute('value','Enter your phone number.');
}

</script>

<textbox id="tbox1"/>
<textbox id="tbox2" onfocus="displayFocus();"/>
<description id="sbar" value=""/>

The focus event, when it occurs, will call the displayFocus function. This function will change the value of the text label. We could extend this example to remove the text when the blur event occurs. Typically, you will use focus and blur events to update parts of the interface as the user selects elements. For instance, you might update a total as the user enters values in other fields, or use focus events to validate certain values. Don't display an alert during a focus or blur event as this will be distracting for the user and is poor user interface design.

You can also add event handlers dynamically using the DOM function addEventListener. You can use it for any element and event type. It takes three parameters, the event type, a function to execute when the event occurs and a boolean indicating whether to capture or not.
Getting the currently focused element

The currently focused element is held by an object called a command dispatcher, of which there is only one for the window. The command dispatcher is responsible for keeping track of the focused element as the user uses the interface. The command dispatcher has other roles, which will be discussed in a later section on commands. For now, we'll look at some of the focus related features of the command dispatcher.

You can retrieve the command dispatcher from a window using the document's commandDispatcher property. From there, you can get the focused element with the dispatcher's focusedElement property. The example below shows this.

Example 3 : Source View

<window id="focus-example" title="Focus Example"
        onload="init();"
        xmlns="http://www.mozilla.org/keymaster/gatekeeper/there.is.only.xul">

<script>
function init(){
  addEventListener("focus",setFocusedElement,true);
}

function setFocusedElement(){
  var focused = document.commandDispatcher.focusedElement;
  document.getElementById("focused").value = focused.tagName;
}
</script>

<hbox>
  <label control="username" value="User Name:"/>
  <textbox id="username"/>
</hbox>

<button label="Hello"/>
<checkbox label="Remember This Decision"/>

<label id="focused" value="-No focus-"/>

</window>

In this example, a focus event handler is attached the window. We want to use a capturing event handler, so the addEventListener method needs to be used. It registers a capturing event handler with the window which will call the setFocusedElement method. This method gets the focused element from the command dispatcher and sets a label to its tag name. As the focused element is changed, the label will show the tagname of the element.

A few things to note.

    First, when the textbox is focused, the tag name is 'html:input', not 'textbox' as we might expect. This is because XUL text boxes are implemented using the HTML input widget, so the focus event is received for that element instead.
    Second, clicking the textbox's label changes the focus to the textbox. This is because the label has a control attribute pointing to the id of the textbox.
    Finally, the other label which displays the tag name has no control attribute, so clicking it has no effect on the focused element. Only focusable elements can be focused.

Making a label focusable

If you were creating custom elements, you might have a need to change whether an element can have the focus or not. For this, you can use a special style property -moz-user-focus. This property controls whether an element can be focused. For instance, you could make a label focusable, as in the example below.

Example 4 : Source View

<label id="focused" style="-moz-user-focus: normal;"
          onkeypress="alert('Label Focused');" value="Focus Me"/>

The style property is set to normal. You can also set it to ignore to turn off the focus for an element. This shouldn't be used for disabling an element, however; the disabled attribute or property should be used instead, since that is what it is designed for. Once the label in the example is focused, it can respond to key presses. Naturally, the label gives no indication that it is focused, since it isn't normally expected to ever be focused.
Changing the focus

There are several ways to change the currently focused element. The simplest is to call the focus method of a the XUL element that you wish to set the focus to. The blur method can be used to remove the focus from an element. The following example demonstrates this:

Example 5 : Source View

<textbox id="addr"/>

<button label="Focus" oncommand="document.getElementById('addr').focus()"/>

Or, you can use the methods advanceFocus and rewindFocus on the command dispatcher. These methods move the focus to the next element in sequence or the previous element respectively. This is what happens when the user presses TAB or Shift+Tab.

For textboxes, a special attribute, focused is added whenever the element has the focus. You can check for the presence of this attribute to determine if the element has the focus, either from a script or within a style sheet. It will have the value true if the textbox has the focus and, if the textbox does not have the focus, the attribute will not be present.

Suppose you wanted to move the focus from where it currently is, to the next place the browser thinks it should be. A user typically does this by hitting the "Tab" key. You can do this anywhere you have a XUL browser document by simply:

document.commandDispatcher.advanceFocus();

In fact, the commandDispatcher simply implements the nsIDOMXULCommandDispatcher interface. This interface also provides a number of other methods which may be useful.
Platform Specific Behaviors

Mac OS X
    There is a preference called "Full Keyboard Access" (FKA). Note that XUL adheres to this. This means that when the FKA preference is off, only textboxes and lists/trees are focusable with the keyboard, as well as from your code using focus().

Handling Text Changes

There are two events that can be used when the user changes the value of a textbox. Naturally, these events will only be sent to the textbox that has the focus.

    The input event is fired whenever the text is modified in the field. The new value will be different than the old value. You may want to use this event instead of using key events, as some keys such as the shift key don't change the value. Also, the input event would not fire if a letter key was pressed and there were already more characters than will fit in the textbox.
    The change event is similar in that it fires only when the field is changed. However it only fires once the textbox loses the focus, thus, only once per set of changes.

Text Selection

When working with a textbox, you may wish to retrieve not the entire contents of a field but only what the user has selected. Or, you may wish to change the current selection.

XUL textboxes support a way to retrieve and modify the selection. The simplest one is to select all of the text in a textbox. This involves using the select method of the textbox.

tbox.select();

However, you may wish to select only part of the text. To do this you can use the setSelectionRange function. It takes two parameters, the first is the starting character and the second is the character after the last one that you want to have selected. Values are zero-based, so the first character is 0, the second is 1 and so on.

tbox.setSelectionRange(4,8);

This example will select the fifth character displayed, as well as the sixth, seventh and eighth. If there were only six characters entered into the field, only the fifth and sixth characters would be selected. No error would occur.

If you use the same value for both parameters, the start and end of the selection changes to the same position. This results in changing the cursor position within the textbox. For example, the line below can be used to move the cursor to the beginning of the text.

tbox.setSelectionRange(0,0);

You can retrieve the current selection by using the selectionStart and selectionEnd properties. These properties are set to the starting and ending positions of the current selection respectively. If both are set to the same value no text is selected, and the cursor is moved to that position. Once you have the start and end positions, you can pull out the substring from the whole text.

You can retrieve and modify the contents of the textbox by using the value property.

One additional useful property of textboxes is the textLength property, which holds the total number of characters in the field.

Next, we'll find out how to use commands.

« Previous
Next »
Document Tags and Contributors
Tags: 

    Tutorials XUL XUL_Tutorial 

Contributors to this page: Sheppy, teoli, fscholz, trevorh, Mixticius, Enn, The moose, Chbok, Another sam, Itfische, Morishoji, Dria, Jjinux, Mgjbot, Pmash, Hakanw, Dougt, Ptak82
Last updated by: Sheppy, Apr 14, 2014, 10:34:51 AM
Learn the best of web development

Get the latest and greatest from MDN delivered straight to your inbox.
E-mail
MDN Web Docs

    Web Technologies
    Learn Web Development
    About MDN
    Feedback

Mozilla

    About
    Contact Us
    Firefox

Other languages:

    Terms Privacy Cookies 

© 2005-2018 Mozilla and individual contributors.

Content is available under these licenses.


    Skip to main content
    Select language
    Skip to search

MDN Web Docs

    Technologies References & Guides Feedback 

    Sign in

Search

Edit

Commands

« Previous
Next »

A command is an operation which may be invoked.
Command Elements

The command element is used to create commands which can be used to carry out operations. You don't need to use commands, since you can just call a script to handle things. However, a command has the advantage that it can be disabled when needed and can be invoked without needing to know about the details of its implementation. Commands provide a suitable way to abstract operations from the code. Commands are especially useful for larger applications.

For instance, in order to implement the clipboard menu commands, cut, copy and paste, you can use commands. If you did not use commands, you would need to figure out which field has the focus, then check to ensure that the operation is suitable for that element. In addition, the menu commands would need to be enabled and disabled depending on whether the focused element had selected text or not, and for paste operations, whether there is something suitable on the clipboard to paste. As you can see, this becomes complicated. By using commands, much of the work is handled for you.

You can use a command for any operation. Mozilla uses them for almost every menu command. In addition, text fields and other widgets have a number of commands which they already support that you can invoke. You should use them when the operation depends on which element is focused.

A command is identified by its id attribute. Mozilla uses the convention that command id's start with 'cmd_'. You will probably want to use the same id if a command is already being used, however, for your own commands, you can use any command id you wish. To avoid conflicts, you may wish to include the application name in the command id. A simple way of using commands is as follows:
Example: Simple command

Example 1 : Source View

<command id="cmd_openhelp" oncommand="alert('Help!');"/>
<button label="Help" command="cmd_openhelp"/>

In this example, instead of placing the oncommand attribute on the button, we instead place it on a command element. The two are then linked using the command attribute, which has the value of the command's id. The result is that when the button is pressed, the command 'cmd_openhelp' is invoked.

There are two advantages to using this approach.

    First, it moves all your operations onto commands which can all be grouped together in one section of the XUL file. This means that code is all together and not scattered throughout the UI code.
    The other advantage is that several buttons or other UI elements can be hooked up to the same command. For instance, you might have a menu item, a toolbar button and a keyboard shortcut all for the same operation. Rather than repeat the code three times, you can hook all three up to the same command. Normally, you would only hook up elements that would send a command event.

Additionally,

    If you set the disabled attribute on the command, the command will be disabled and it will not be invoked.
    Any buttons and menu items hooked up to it will be disabled automatically.
    If you re-enable the command, the buttons will become enabled again.

Example: Toggling command disabled

Example 2 : Source View

<command id="cmd_openhelp" oncommand="alert('Help');"/>
<button label="Help" command="cmd_openhelp"/>
<button label="More Help" command="cmd_openhelp"/>

<button label="Disable"
        oncommand="document.getElementById('cmd_openhelp').setAttribute('disabled','true');"/>
<button label="Enable"
        oncommand="document.getElementById('cmd_openhelp').removeAttribute('disabled');"/>

In this example, both buttons use the same command. When the Disable button is pressed, the command is disabled by setting its disabled attribute, and both buttons will be disabled as well.

It is normal to put a group of commands inside a commandset element, together near the top of the XUL file, as in the following:

<commandset>
  <command id="cmd_open" oncommand="alert('Open!');"/>
  <command id="cmd_help" oncommand="alert('Help!');"/>
</commandset>

A command is invoked when the user activates the button or other element attached to the command. You can also invoke a command by calling the doCommand method either of the command element or an element attached to the command such as a button.
Command Dispatching

You can also use commands without using command elements, or at least, without adding a oncommand attribute to the command. In this case, the command will not invoke a script directly, but instead, find an element or function which will handle the command. This function may be separate from the XUL itself, and might be handled internally by a widget. In order to find something to handle the command, XUL uses an object called a command dispatcher. This object locates a handler for a command. A handler for a command is called a controller. So, essentially, when a command is invoked, the command dispatcher locates a controller which can handle the command. You can think of the command element as a type of controller for the command.

The command dispatcher locates a controller by looking at the currently focused element to see if it has a controller which can handle the command. XUL elements have a controllers property which is used to check. You can use the controllers property to add your own controllers. You might use this to have a listbox respond to cut, copy and paste operations, for instance. An example of this will be provided later. By default, only textboxes have a controller that does anything. The textbox controller handles clipboard operations, selection, undo and redo as well as some editing operations. Note that an element may have multiple controllers, which will all be checked.

If the currently focused element does not have a suitable controller, the window is checked next. The window also has a controllers property which you can modify if desired. If the focus is inside a frame, each frame leading to the top-level window is checked as as well. This means that commands will work even if the focus is inside a frame. This works well for a browser, since editing commands invoked from the main menu will work inside the content area. Note that HTML also has a commands and controller system although you can't use it on unprivileged web pages, but you may use it from, for example, a browser extension. If the window doesn't provide a controller capable of handling the command, nothing will happen.

You can get the command dispatcher using the document's commandDispatcher property, or you can retrieve it from the controllers list on an element or a window. The command dispatcher contains methods for retrieving controllers for commands and for retrieving and modifying the focus.
Adding Controllers

You can implement your own controllers to respond to commands. You could even override the default handling of a command with careful placement of the controller. A controller is expected to implement four methods, which are listed below:

supportsCommand (command)
    this method should return true if the controller supports a command. If you return false, the command is not handled and command dispatcher will look for another controller. A single controller may support multiple commands.
isCommandEnabled (command)
    this method should return true if the command is enabled, or false if it is disabled. Corresponding buttons will be disabled automatically.
doCommand (command)
    execute the command. This is where you would put the code to handle the command.
onEvent (event)
    this method handles an event.

Example: Controller implementation

Let's assume that we want to implement a listbox that handles the delete command. When the user selects Delete from the menu, the listbox deletes the selected row. In this case, you just need to attach a controller to a listbox which will perform the action in its doCommand method.

Try opening the example below (Source View) in a browser window and selecting items from the list. You'll notice that the Delete command on the browser's Edit menu is enabled and that selecting it will delete a row. (See Discussion about opening this example). The example below isn't completely polished. Really, we should ensure that the selection and focus is adjusted appropriately after a deletion.

<window id="controller-example" title="Controller Example" onload="init();"
        xmlns="http://www.mozilla.org/keymaster/gatekeeper/there.is.only.xul">

<script>
function init()
{
  var list = document.getElementById("theList");

  var listController = {
    supportsCommand : function(cmd){ return (cmd == "cmd_delete"); },
    isCommandEnabled : function(cmd){
      if (cmd == "cmd_delete") return (list.selectedItem != null);
      return false;
    },
    doCommand : function(cmd){
      list.removeItemAt(list.selectedIndex);
    },
    onEvent : function(evt){ }
  };

  list.controllers.appendController(listController);
}
</script>

<listbox id="theList">
  <listitem label="Ocean"/>
  <listitem label="Desert"/>
  <listitem label="Jungle"/>
  <listitem label="Swamp"/>
</listbox>

</window>

The controller (listController) implements the four methods described above. The supportsCommand method returns true for the 'cmd_delete' command, which is the name of the command used when the Delete menu item is selected. For other commands, false should be returned since the controller does not handle any other commands. If you wanted to handle more commands, check for them here, since you will often use a single controller for multiple related commands.

The isCommandEnabled method returns true if the command should be enabled. In this case, we check if there is a selected item in the listbox and return true if there is. If there is no selection, false is returned. If you delete all the rows in the example, the Delete command will become disabled. You may have to click the listbox to update the menu in this simple example. The doCommand method will be called when the Delete menu item is selected, and this will cause the selected row in the listbox to be deleted. Nothing needs to happen for the onEvent method, so no code is added for this method.
Override Default Controller

We attach this controller to the listbox by calling the appendController method of the listbox's controllers. nsIControllers has a number of methods that may be used to manipulate the controllers. For instance, there is also an insertControllerAt method which inserts a controller into an element before other ones. This might be useful to override commands. For example, the following example will disable pasting into a textbox.

var tboxController = {
  supportsCommand : function(cmd){ return (cmd == "cmd_paste"); },
  isCommandEnabled : function(cmd){ return false; },
  doCommand : function(cmd){ },
  onEvent : function(evt){ }
};

document.getElementById("tbox").controllers.insertControllerAt(0,tboxController);

In this example, we insert the controller at index 0, which means before any others. The new controller supports the 'cmd_paste' command and always indicates that the command is disabled. The default textbox controller never gets called because the command dispatcher found the controller above to handle the command first.

Next, we'll find out how to update commands.

« Previous
Next »
Document Tags and Contributors
Tags: 

    Tutorials XUL XUL_Tutorial 

Contributors to this page: SphinxKnight, Sheppy, teoli, trevorh, Neil, madarche, kiteroa, The moose, Chbok, Nathymig, Morishoji, Jjinux, Mgjbot, Pmash, Laolu, Ptak82, Dria
Last updated by: SphinxKnight, Feb 9, 2018, 12:46:34 PM
Learn the best of web development

Get the latest and greatest from MDN delivered straight to your inbox.
E-mail
MDN Web Docs

    Web Technologies
    Learn Web Development
    About MDN
    Feedback

Mozilla

    About
    Contact Us
    Firefox

Other languages:

    Terms Privacy Cookies 

© 2005-2018 Mozilla and individual contributors.

Content is available under these licenses.


    Skip to main content
    Select language
    Skip to search

MDN Web Docs

    Technologies References & Guides Feedback 

    Sign in

Search

Edit

Updating Commands

« Previous
Next »

In this section, we will look at how to update commands.
Invoking Commands

If a command has an oncommand attribute, you can invoke it just by using the doCommand method of the command or an element which links to it. For other commands, you will need to use a couple of additional lines of code. You will need to use these extra steps when invoking commands implemented by a controller. In addition, you will need to do this when creating your own menu commands, for instance to implement the edit menu commands in your own application.

Fortunately, the extra code is fairly simple. All we need to do is get the needed controller and call the command. A simple way of doing this is the following:

var controller = document.commandDispatcher.getControllerForCommand("cmd_paste");
if (controller && controller.isCommandEnabled("cmd_paste")){
  controller.doCommand(command);
}

The code above first retrieves the controller for the 'cmd_paste' command from the command dispatcher. Then, it checks to see whether the command is enabled, and then executes the command using the doCommand method of the controller. Note that we don't need to figure out which element to use or which controller to use. The command dispatcher handles that part. Also, we could just call doCommand without checking if the command was enabled or not, although we probably shouldn't do that.

The code above is generic enough that it can be a function that takes a command as an argument and executes that command. This function could then be reused for all commands. In fact, this is common enough that Mozilla includes a library which does just that. If you include the script 'chrome://global/content/globalOverlay.js' in a XUL file, you can call the goDoCommand method which executes the command passed as the argument. The code for this function is only a few lines long so you could include it directly in your code if for some reason you didn't want to include the library.

<script src="chrome://global/content/globalOverlay.js"/>

<command id="cmd_paste" oncommand="goDoCommand('cmd_paste');"/>
<button label="Paste" command="cmd_paste"/>

The example above will implement a Paste button. It is attached to the command which will invoke the command on the necessary controller when called. The code above is all you need to implement the functionality of the paste command in your application. The only other thing you need to do is ensure that the enabled status of the paste command, and therefore the button, is updated at the right time, which is described below.
Command Updaters

A command updater is a feature of the commandset element which allows it to update commands when certain events happen. You will need to think about when a command is valid and when it is not. In addition, you will need to consider when the state could change and when the commands should be updated.

For example, the paste command is valid when a textbox has the focus and there is something on the clipboard to paste. The command will become enabled whenever a textbox is focused and when the clipboard contents change. A command updater will listen for these situations and code can be executed which enables and disables commands as necessary.

A simple command updater looks like this:

<commandset id="updatePasteItem"
            commandupdater="true"
            events="focus"
            oncommandupdate="goUpdateCommand('cmd_paste');"/>

A command updater is indicated by using the commandupdater attribute, which should be set to true. The events attribute is used to list the events that the command updater listens for. You can specify multiple events by separating them with commas. In the example above, the command updater listens for the focus event. This causes commands to be updated when an element receives the focus.

When a focus event occurs, the code in the oncommandupdate attribute is called. In the example, the goUpdateCommand method is called which is a function provided by the globalOverlay.js script described earlier. It will update the command and enable or disable necessary buttons and menu items. The code behind it is fairly simple. It just gets the necessary controller, calls its isCommandEnabled method, and then enables or disables the command. If you have several commands to update, call the goUpdateCommand method once for each command.

Note that the command updater will receive notifications about all focus events on all elements, even if other event handlers respond to the event. Essentially, a command updater is like a global event handler.

Command updaters have a number of events which they can respond to which are listed below. It is also possible to create your own.

    focus: occurs when the focused element changes.
    select: occurs when the selected text changes.
    undo: occurs when the undo buffer changes.
    clipboard: occurs when the contents of the clipboard changes.

The following example shows the command updaters used in the Mozilla browser to update the edit menu commands. The functions used can be found in the 'chrome://communicator/content/utilityOverlay.js' script.

<commandset id="globalEditMenuItems"
            commandupdater="true"
            events="focus"
            oncommandupdate="goUpdateGlobalEditMenuItems()"/>
<commandset id="selectEditMenuItems"
            commandupdater="true"
            events="select"
            oncommandupdate="goUpdateSelectEditMenuItems()"/>
<commandset id="undoEditMenuItems"
            commandupdater="true"
            events="undo"
            oncommandupdate="goUpdateUndoEditMenuItems()"/>
<commandset id="clipboardEditMenuItems"
            commandupdater="true"
            events="clipboard"
            oncommandupdate="goUpdatePasteMenuItems()"/>

Next, we'll find out how to use observers.

« Previous
Next »
Document Tags and Contributors
Tags: 

    Tutorials XUL XUL_Tutorial 

Contributors to this page: Sheppy, teoli, fscholz, masi, trevorh, kiteroa, Ujoux, Chbok, Ptak82, Morishoji, Mgjbot, SylvainPasche, Dria
Last updated by: Sheppy, Apr 14, 2014, 10:34:50 AM
Learn the best of web development

Get the latest and greatest from MDN delivered straight to your inbox.
E-mail
MDN Web Docs

    Web Technologies
    Learn Web Development
    About MDN
    Feedback

Mozilla

    About
    Contact Us
    Firefox

Other languages:

    Terms Privacy Cookies 

© 2005-2018 Mozilla and individual contributors.

Content is available under these licenses.


    Skip to main content
    Select language
    Skip to search

MDN Web Docs

    Technologies References & Guides Feedback 

    Sign in

Search

Edit

Broadcasters and Observers

« Previous
Next »

There may be times when you want several elements to respond to events or changes of state easily. To do this, we can use broadcasters.
Command Attribute Forwarding

We've already seen that elements such as buttons can be hooked up to commands. In addition, if you place the disabled attribute on the command element, any elements hooked up to it will also become disabled automatically. This is a useful way to simplify the amount of code you need to write. The technique also works for other attributes as well. For instance, if you place a label attribute on a command element, any buttons attached to the command will share the same label.

Example 1 : Source View

<command id="my_command" label="Open"/>

<button command="my_command"/>
<checkbox label="Open in a new window" command="my_command"/>

In this example, the button does not have a label attribute, however it is attached to a command that does. The button will share the label with the command. The checkbox already has a label, however, it will be overridden by the command's label. The result is that both the button and the checkbox will have the same label 'Open'.

If you were to modify the command's label attribute, the label on the button and checkbox will adjust accordingly. We saw something like this in a previous section where the disabled attribute was adjusted once and propagated to other elements.

This attribute forwarding is quite useful for a number of purposes. For instance, let's say that we want the Back action in a browser to be disabled. We would need to disable the Back command on the menu, the Back button on the toolbar, the keyboard shortcut (Alt+Left for example) and any Back commands on popup menus. Although we could write a script to do this, it is quite tedious. It also has the disadvantage that we would need to know all of the places where a Back action could be. If someone added a new Back button using an extension, it wouldn't be handled. It is convenient to simply disable the Back action and have all the elements that issue the Back action disable themselves. We can use the attribute forwarding of commands to accomplish this.
Broadcasters

There is a similar element called a broadcaster. Broadcasters support attribute forwarding in the same way that commands do. They work the same as commands except that a command is used for actions, while a broadcaster is instead used for holding state information. For example, a command would be used for an action such as Back, Cut or Delete. A broadcaster would be used to hold, for instance, a flag to indicate whether the user was online or not. In the former case, menu items and toolbar buttons would need to be disabled when there was no page to go back to, or no text to cut or delete. In the latter case, various user interface elements might need to update when the user switches from offline mode to online mode.

The simplest broadcaster is shown below. You should always use an id attribute so that it can be referred to by other elements.

<broadcasterset>
  <broadcaster id="isOffline" label="Offline"/>
</broadcasterset>

Any elements that are watching the broadcaster will be modified automatically whenever the broadcaster has its label attribute changed. This results in these elements having a new label. Like other non-displayed elements, the broadcasterset element serves as a placeholder for broadcasters. You should declare all your broadcasters inside a broadcasterset element so that they are all kept together.
Making elements observers

Elements that are watching the broadcaster are called observers because they observe the state of the broadcaster. To make an element an observer, add an observes attribute to it. This is analogous to using the command attribute when attaching an element to a command element. For example, to make a button an observer of the broadcaster above:

<button id="offline_button" observes="isOffline"/>

The observes attribute has been placed on the button and its value has been set to the value of the id on the broadcaster to observe. Here the button will observe the broadcaster which has the id isOffline, which is the one defined earlier. If the value of the label attribute on the broadcaster changes, the observers will update the values of their label attributes also.

We could continue with additional elements. As many elements as you want can observe a single broadcaster. You can also have only one if you wanted to but that would accomplish very little since the main reason for using broadcasters is to have attributes forwarded to multiple places. You should only use broadcasters when you need multiple elements that observe an attribute. Below, some additional observers are defined:

<broadcaster id="offline_command" label="Offline" accesskey="f"/>

<keyset>
  <key id="goonline_key" observes="offline_command" modifiers="accel" key="O"/>
</keyset>
<menuitem id="offline_menuitem" observes="offline_command"/>
<toolbarbutton id="offline_toolbarbutton" observes="offline_command"/>

In this example, both the label and the accesskey will be forwarded from the broadcaster to the key, menu item and the toolbar button. The key won't use any of the received attributes for anything, but it will be disabled when the broadcaster is disabled.

You can use a broadcaster to observe any attribute that you wish. The observers will grab all the values of any attributes from the broadcasters whenever they change. Whenever the value of any of the attributes on the broadcaster changes, the observers are all notified and they update their own attributes to match. Attributes of the observers that the broadcaster doesn't have itself are not modified. The only attributes that are not updated are the id and persist attributes; these attributes are never shared. You can also use your own custom attributes if you wish.

Broadcasters aren't used frequently as commands can generally handle most uses. One thing to point out is that there really is no difference between the command element and the broadcaster element. They both do the same thing. The difference is more semantic. Use commands for actions and use broadcasters for state. In fact, any element can act as broadcaster, as long as you observe it using the observes attribute.
The Observes Element

There is also a way to be more specific about which attribute of the broadcaster to observe. This involves an observes element. Like its attribute counterpart, it allows you to define an element to be an observer. The observes element should be placed as a child of the element that is to be the observer. An example is shown below:

Example 2 : Source View

<broadcasterset>
  <broadcaster id="isOffline" label="Offline" accesskey="f"/>
</broadcasterset>

<button id="offline_button">
  <observes element="isOffline" attribute="label"/>
</button>

Two attributes have been added to the observes element. The first, element, specifies the id of the broadcaster to observe. The second, attribute, specifies the attribute to observe. The result here is that the button will receive its label from the broadcaster, and when the label is changed, the label on the button is changed. The observes element itself does not change but the element it is inside changes, which in this case is a button. Notice that the accesskey is not forwarded to the button, since it is not being obseved. If you wanted it to be, another observes element would need to be added. If you don't use any observes elements, and instead use the observes attribute directly on the button, all attributes will be observed.
Broadcast event

There is an additional event handler that we can place on the observes element which is onbroadcast. The event is called whenever the observer notices a change to the attributes of the broadcaster that it is watching. An example is shown below.

Example 3 : Source View

<broadcasterset>
  <broadcaster id="colorChanger" style="color: black"/>
</broadcasterset>

<button label="Test">
  <observes element="colorChanger" attribute="style" onbroadcast="alert('Color changed');"/>
</button>

<button label="Observer"
  oncommand="document.getElementById('colorChanger').setAttribute('style','color: red');"
/>

Two buttons have been created, one labeled Test and the other labeled Observer. If you click on the Test button, nothing special happens. However, if you click on the Observer button, two things happen. First, the button changes to have red text and, second, an alert box appears with the message 'Color changed'.

What happens is the oncommand handler on the second button is called when the user presses on it. The script here gets a reference to the broadcaster and changes the style of it to have a color that is red. The broadcaster is not affected by the style change because it doesn't display on the screen. However, the first button has an observer which notices the change in style. The element and the attribute on the observes tag detect the style change. The style is applied to the first button automatically.

Next, because the broadcast occured, the event handler onbroadcast is called. This results in an alert message appearing. Note that the broadcast only occurs if the attributes on the broadcaster element are changed. Changing the style of the buttons directly will not cause the broadcast to occur so the alert box will not appear.

If you tried duplicating the code for the first button several times, you would end up with a series of alert boxes appearing, one for each button. This is because each button is an observer and will be notified when the style changes.

Next, we'll look at using the Document Object Model with XUL elements.

« Previous
Next »
Document Tags and Contributors
Tags: 

    Tutorials XUL XUL_Tutorial 

Contributors to this page: SphinxKnight, Sheppy, teoli, trevorh, LJR, Brettz9, Chbok, Nathymig, Morishoji, Mgjbot, SylvainPasche, Ptak82, Dria
Last updated by: SphinxKnight, Feb 9, 2018, 11:44:02 AM
Learn the best of web development

Get the latest and greatest from MDN delivered straight to your inbox.
E-mail
MDN Web Docs

    Web Technologies
    Learn Web Development
    About MDN
    Feedback

Mozilla

    About
    Contact Us
    Firefox

Other languages:

    Terms Privacy Cookies 

© 2005-2018 Mozilla and individual contributors.

Content is available under these licenses.


    Skip to main content
    Select language
    Skip to search

MDN Web Docs

    Technologies References & Guides Feedback 

    Sign in

Search

Edit

Document Object Model

« Previous
Next »

The Document Object Model (DOM) can be used with XUL elements to get information about them or modify them.
DOM Introduction

The Document Object Model (DOM) is used to store the tree of XUL nodes. When an XUL file is loaded, the tags are parsed and converted into a hierarchical document structure of nodes, one for each tag and block of text. The DOM structure may be examined and modified using various methods provided for this purpose. Specific XUL elements also provide additional functions which may be used.

Each XUL file that is loaded will have its own document displayed in a window or frame. Although there is only ever one document associated with a window at a time, you may load additional documents using various methods.

In Mozilla, the DOM may be accessed and manipulated using JavaScript. The various DOM objects have functions which may be accessed in script, however, it is important to note that the DOM is an API that is accessible by JavaScript. JavaScript itself is just a scripting language that can access these objects because Mozilla provides these objects for use.

In JavaScript, there is always a single global object that is always available. You can refer to the properties and methods of the global object without having to qualify them with an object. For example, if the global object has a 'name' property, you can change the name with the code 'name = 7', without having to specify any object to use. In a browser context, the window is the global object, and the same is also true for XUL. Naturally, this global object will be different for each window. Each frame will also have a separate window object.

The window is often referred to using the window property, although this is optional. Sometimes, this is done just to clarify the scope of the method you are referring to. For example, the following two lines which open a new window are functionally equivalent:

window.open("test.xul","_new");
open("test.xul","_new");

When you declare a function or a variable at the top level of a script, that is outside another function, you are actually declaring a property of the global object. In XUL, each function you declare will be set as a property of the window object. For example, the following code will display the text 'Message' in an alert twice.

function getText(){
  return "Message";
}

alert(getText());
alert(window.getText());

Thus, if you want to access variables or call a function declared in a script used by another window, you just need to access it using the other window's window object. For example, if we combined the last two examples into a single file, we might want to call the getText() function from within the other window (for example, the test.xul window). To do this, we can do the following:

alert(window.opener.getText());

Each window has an opener property which holds the window object that opened this one. In this example, we retrieve the opener window and call the getText() function declared in a script used there. Note that we qualified the property with the 'window' identifier just to be clearer.

The window's open() method also returns a reference to the new window so you can call functions of the new window from the opener. It is important to note, however, that the open() method returns before the window has fully loaded, so functions will not typically be available yet.

The window object isn't defined by any DOM specification, but in Mozilla is sometimes considered part of DOM Level 0, a name used by some developers to refer to the DOM-like functions before they were added to specifications. The actual document displayed in a window can be retrieved using the window's document property. Since it is one of the most commonly referenced properties of the window, the document property is usually referenced without the 'window.' qualifier.

Mozilla provides several different document objects depending on the kind of document. The three main documents are HTMLDocument, XMLDocument, and XULDocument, for HTML, XML and XUL documents respectively. Obviously, it is this latter type of document used for XUL. The three document types are very similar, in fact they all share the same base implementation. However, there are a few functions that are specific to one document type or the other.
Retrieving Elements

The most common method to retrieve an element in a document is to give the element an id attribute and the use the document's getElementById() method. We've added the id attribute to a number of elements in the find file dialog. For example, we could get the state of a check box by using the code below:

var state = document.getElementById('casecheck').checked;

The value casecheck corresponds to the id of the case sensitive checkbox. Once we have an indication of whether it is checked or not, we can use the state to perform the search. We could do something similar for the other check box, or any other element that has an id. We'll need to get the text in the input field for example.
Our find files example

It doesn't make sense to have the progress bar and the dummy tree data displayed when the find files dialog is first displayed. These were added so that we could see them. Let's take them out now and have them displayed when the Find button is pressed. First, we need to make them initially invisible. The hidden attribute is used to control whether an element is visible or not.

We'll change the progress meter so that its initially hidden. Also, we'll add an id attribute so that we can refer to it in a script to show and hide it. While we're at it, let's also hide the splitter and results tree as we only need to show them after a search is performed.

<tree id="results" hidden="true" flex="1">
  .
  .
  .
<splitter id="splitbar" resizeafter="grow" hidden="true"/>

<hbox>

  <progressmeter id="progmeter" value="50%"
    style="margin: 4px;" hidden="true"/>

We've added the hidden attribute and set the value to true. This causes the element to be hidden when it first appears.

Next, let's add a function that is called when the Find button is pressed. We'll put scripts in a separate file findfile.js. In the last section, we added the script element in the XUL file. If you haven't done this, do that now, as shown below. An oncommand handler will also be added to the Find button.

<script src="findfile.js"/>
  .
  .
  .
<button id="find-button" label="Find"
   oncommand="doFind();"/>

Now, create another file called findfile.js in the same directory as findfile.xul. We'll add the doFind() function is this file. The script tag does allow code to be contained directly inside of it. However, for various reasons, including better performance, you should always put scripts in separate files, except for short fragments which can be put directly in the event handler.

function doFind(){
  var meter = document.getElementById('progmeter');
  meter.hidden = false;
}

This function first gets a reference to the progress meter using its id, progmeter. The second line of the function body changes the hidden state so that the element is visible again.

Finally, let's have an alert box pop up that displays what will be searched for. Of course, we wouldn't want this in the final version but we'll add it to reassure us that something would happen.

function doFind(){
  var meter=document.getElementById('progmeter');
  meter.hidden = false;
  var searchtext=document.getElementById('find-text').value;
  alert("Searching for \"" + searchtext + "\"");
}

Now, with that alert box in there, we know what should happen when we click the Find button. We could add additional code to get the selections from the drop-down boxes too.
XUL Element DOM

Every XUL element has a set of attributes, a set of properties and a set of children.

    The attributes are declared in the source, for example, flex="1", is a flex attribute set to the value 1.
    Properties are available in JavaScript using the dot syntax. For example, element.hidden refers to the hidden property of an element.
    The children are the child tags of the element and will be nested inside the element in the source.

It is possible to manipulate the attributes, properties and children of an element dynamically using DOM methods.

It is important to note that attributes and properties are separate things. Just because there is an attribute with a given name does not mean that there is a corresponding property with the same name. However, it will often be the case that such a property will exist. For example, to get the flex of an element, you can use the flex property. In this case, the underlying code just returns the value of the attribute. For other properties, XUL will perform more complex calculations.

You can manipulate the attributes of an element using any of the following methods:

getAttribute ( name )
    Return the value of the attribute with the given name.
hasAttribute ( name )
    Return true if the attribute with the given name has a value.
setAttribute ( name , value )
    Set the value of the attribute with the given name to the given value.
removeAttribute ( name )
    Remove the attribute with the given name.

These functions allow you to get and change the value of an attribute at any time. For example, to use the value of the flex attribute, you might use code like the following:

var box = document.getElementById('somebox');
 var flex = box.getAttribute("flex");
 
 var box2 = document.getElementById('anotherbox');
 box2.setAttribute("flex", "2");

However, the flex attribute has a corresponding script property which can be used instead. It isn't any more efficient, but it does mean slightly less typing. The following example accomplishes the same as above using the flex property instead.

var box = document.getElementById('somebox');
 var flex = box.flex;
 
 var box2 = document.getElementById('anotherbox');
 box2.flex = 2;

Once you have a reference to an element, you can call the properties of that element. For example, to get the hidden property for an element, you can use the syntax element.hidden where 'element' is a reference to the element. You might note that most of the properties listed in the reference correlate to common attributes on elements. There are differences, of course, for example, while getAttribute("hidden") will return the string 'true' for a hidden element, whereas the hidden property returns a boolean true value. In this case, the type conversion is done for you so the property is more convenient.

As with each document, there is a different element object for XUL elements as for HTML and XML elements. Every XUL element implements the XULElement (reference broken) interface. A XUL element is any element declared with the XUL namespace. So, XUL elements will have that interface even if added to other XML documents, and non-XUL elements will not have that interface. The XULElement interface has a number of properties and methods specific to XUL elements, many inherited from the generic DOM Element interface.

A namespace is a URI which specifies the kind of element. Here are some examples:

<button xmlns="http://www.mozilla.org/keymaster/gatekeeper/there.is.only.xul"/>
<button xmlns="http://www.w3.org/1999/xhtml"/>
<html:button xmlns:html="http://www.w3.org/1999/xhtml"/>
<html:button xmlns:html="http://www.mozilla.org/keymaster/gatekeeper/there.is.only.xul"/>

Namespaces are specified using the xmlns attribute.

    The first button is a XUL element as it has been placed in the XUL namespace.
    The second element is an XHTML element as it has been given the XHTML namespace.
    In the third example, the prefix 'html' is mapped to the namespace 'http://www.w3.org/1999/xhtml'. You can also use the prefix syntax with a colon to use a specific namespace . This is used when you are using several namespaces in a document and you need to be more precise with which namespace is meant. This causes an XHTML button to be created for this case.
    The fourth button is a little confusing, but might clarify that it is the URI that is important and not the prefix. The fourth example is a XUL button, not an HTML button, despite what the prefix might say.

This is an important distinction. In fact, the actual text used for the prefix is irrelevant when determining what kind of element is used.

The DOM provides a number of namespace related functions similar to the non-namespace ones. For example, the getAttributeNS() function is similar to the the getAttribute() function except an additional argument may be supplied to specify an attribute in a specific namespace.

Many XUL elements have their own properties that are unique to that element. Refer to the element reference for a full guide to the attributes and properties available for an element.
Navigating the DOM

The DOM is a tree structure with a single root node with children. You can get a reference to the root node using the document's documentElement property. The root node is always an element, but this is not the case for other nodes in the tree. An element corresponds to a tag is the XUL source, but you may also find text nodes, comment nodes and a few other types in a document tree. In the case of XUL, the root element will be the window tag in the XUL document. Each node in the tree may have children and those children may have child nodes of their own. Since the DOM is a tree structure, you can navigate through the tree using a variety of properties. Some common properties are listed below:

firstChild 
    reference to the first child node of an element
lastChild 
    reference to the last child node of an element
childNodes 
    holds a list of the children of an element
parentNode 
    reference to the parent of an node
nextSibling 
    reference to the next sibling in sequence
previousSibling 
    reference to the previous sibling in sequence

These properties allow you to navigate through a document is various ways. For example, you might get the first child of an element using the firstChild property and then navigate through the children using the nextSibling property. Or, you might accomplish the same thing by iterating through the items in the childNodes list. In Mozilla, the latter method is more efficient.

The following example shows how to iterate over the children of the root node:

var childNodes = document.documentElement.childNodes;
for (var i = 0; i < childNodes.length; i++) {
  var child = childNodes[i];
  // do something with child
}

The childNodes variable will hold the children of the document root element. We then use a for loop to iterate over the children, accessing each item using an array-like notation.
Find files example so far: Source View
See also

    A re-introduction to JavaScript
    JavaScript reference

Next we'll learn how to modify the DOM.

« Previous
Next »
Document Tags and Contributors
Tags: 

    DOM Tutorials XUL XUL_Tutorial 

Contributors to this page: SphinxKnight, o.marce, Sheppy, teoli, trevorh, Chbok, Mgjbot, Morishoji, Pmash, Nickolay, Ptak82, Dria
Last updated by: SphinxKnight, Feb 9, 2018, 11:21:34 AM
Learn the best of web development

Get the latest and greatest from MDN delivered straight to your inbox.
E-mail
MDN Web Docs

    Web Technologies
    Learn Web Development
    About MDN
    Feedback

Mozilla

    About
    Contact Us
    Firefox

Other languages:

    Terms Privacy Cookies 

© 2005-2018 Mozilla and individual contributors.

Content is available under these licenses.


    Skip to main content
    Select language
    Skip to search

MDN Web Docs

    Technologies References & Guides Feedback 

    Sign in

Search

Edit

Modifying a XUL Interface

« Previous
Next »

The DOM provides various functions to modify the document.
Creating New Elements

You can create new elements using the createElement() function of the document. It takes one argument, the tag name of the element to create. You can then set attributes of the element using the setAttribute() function and append it to the XUL document using the appendChild() function. For example, the following will add a button to a XUL window:

Example 1 : Source View

<script>
function addButton(){
  var aBox = document.getElementById("aBox");
  var button = document.createElement("button");
  button.setAttribute("label","A new Button");
  aBox.appendChild(button);
}
</script>

<box id="aBox" width="200">
  <button label="Add" oncommand="addButton();"/>
</box>

    This example has two parts
        a box container element in XUL. Notice that this is NOT the same as a vbox or an hbox. (This is discussed more in the Box Model pages.)
            the box element has two attributes "id" and "width".
            the box element contains a button element.
                the button element has two attributes "label" and "oncommand"
        a Javascript function named "addButton()"
            This script first gets a reference to the box with getElementById(), which is the container to add a new button to. The function getElementbyID() does not know that the box it is looking for happens to be containing the tag that has the oncommand attribute that referenced it. getElementById() only knows the box it is looking to find has an id with the value "aBox". This is a subtle dependency between the function and the XUL element to which you should pay attention.
            addButton() the calls the createElement() function to create a new button. Note this button is not visible, nor is it attached to anything yet.
            addButton() then assigns the label 'A new Button' to the button using the setAttribute() function.
            Finally the appendChild() function of the particular box found by getElementbyID() is called to add the button to it. At this point, the button is attached to a visible box, so it becomes visible as well.
        The button with the label "Add" can be pressed multiple times and it will continue to add new buttons, each of which will have the label "A new Button", and will only be distinguishable by their place as children in the box element with the id "abox".

The createElement() function will create the default type of element for the document. For XUL documents, this generally means that a XUL element will be created. For an HTML document, an HTML element will be created, so it will have the features and functions of an HTML element instead. The createElementNS() function may be used to create elements in a different namespace.

The appendChild() function is used to add an element as a child of another element. Three related functions are the insertBefore(), replaceChild() and removeChild functions. The syntax of these functions is as follows:

parent.appendChild(child);
parent.insertBefore(child, referenceChild);
parent.replaceChild(newChild, oldChild);
parent.removeChild(child);

It should be fairly straightforward from the function names what these functions do.

    The insertBefore() function inserts a new child node before an existing one. This is used to insert into the middle of a list of children of the parent element instead of at the end like appendChild() does.
    The replaceChild() function removes an existing child and adds a new one in its place at the same position in the list of its parent element.
    Finally the removeChild() function removes a child from the list of its parent element.

Note that for all these functions, the object referred to by the variable referenceChild or the variables newChild and oldChild must already exist or an error occurs. Likewise the object referred to by the variable child which is to be removed must already exist or an error occurs.
Moving Nodes to a different Place

It is often the case that you want to remove an existing element and add it somewhere else. If so, you can just add the element without removing it first. Since a node may only be in one place at a time, the insertion call will always remove the node from its existing location first. This is a convenient way to move nodes around in the document.
Copying Nodes

To copy nodes however, you may call the cloneNode() function. This function makes a copy of an existing node so that you can add it somewhere else. The original node will stay where it is. It takes one boolean argument which indicates whether to copy all of the node's children or not. If false, only the node is copied, such that the copy won't have any children. If true, all of the children are copied as well. This is done recursively, so for large tree structures make sure that this is desired before passing true to the cloneNode() function. Here is an example:

Example 2 : Source View

<hbox height="400">
  <button label="Copy"
          oncommand="this.parentNode.appendChild(this.nextSibling.cloneNode(true));"/>

  <vbox>
    <button label="First"/>
    <button label="Second"/>
  </vbox>
</hbox>

When the Copy button is pressed..

    we retrieve the nextSibling of the button, which will be the vbox element.
    a copy of this element is made using the cloneNode() function
    and the copy is appended using appendChild().

Note that some elements, such as listbox and menulist provide some additional specialized modification functions which you should use instead when you can. These are described in the next section.
Manipulating Basic Elements

The main XUL elements such as buttons, checkboxes and radio buttons may be manipulated using a number of script properties. The properties available are listed in the element reference as those available are different for each element. Common properties that you will manipulate include the label, value, checked and disabled properties. They set or clear the corresponding attribute as necessary.
Label and value properties examples

Here is a simple example which changes the label on a button:

Example 3 : Source View

<button label="Hello" oncommand="this.label = 'Goodbye';"/>

When the button is pressed, the label is changed. This technique will work for a variety of different elements that have labels. For a textbox, you can do something similar for the value property.

Example 4 : Source View

<button label="Add" oncommand="this.nextSibling.value += '1';"/>
<textbox/>

This example adds a '1' to the textbox each time the button is pressed. The nextSibling property navigates from the button (this) to the next element, the textbox. The += operator is used to add to the current value so a 1 will be added onto the end of the existing text. Note that you can still enter text into the textbox. You can also retrieve the current label or value using these properties, as in the following example:

Example 5 : Source View

<button label="Hello" oncommand="alert(this.label);"/>

Toggling a checkbox

Checkboxes have a checked property which may be used to check or uncheck the checkbox. It should be easy to determine how this is used. In this next example, we reverse the state of the checked property whenever the button is pressed. Note that while the label and value properties are strings, the checked property is a boolean property which will be set either true or false.
Note: If you're creating the checkbox dynamically and it's not yet added to the DOM, you must use setAttribute("checked", "false") instead, because the XBL isn't initiated yet.)

Example 6 : Source View

<button label="Change" oncommand="this.nextSibling.checked = !this.nextSibling.checked;"/>
<checkbox label="Check for messages"/>

Radio buttons may be selected as well using properties, however since only one in a group may be selected at a time, the others must all be unchecked when one is checked. You don't have to do this manually of course. The radiogroup's selectedIndex property may be used to do this. The selectedIndex property may be used to retrieve the index of the selected radio button in the group and well as change it.
Changing a element disabled or enabled

It is common to disable particular fields that don't apply in a given situation. For example, in a preferences dialog, one might have the choice of several possibilities, but one choice allows additional customization. Here is an example of how to create this type of interface.

Example 7 : Source View

<script>
function updateState(){
  var name = document.getElementById("name");
  var sindex = document.getElementById("group").selectedIndex;
  name.disabled = sindex == 0;
}
</script>

<radiogroup id="group" onselect="updateState();">
  <radio label="Random name" selected="true"/>
  <hbox>
    <radio label="Specify a name:"/>
    <textbox id="name" value="Jim" disabled="true"/>
  </hbox>
</radiogroup>

In this example a function updateState() is called whenever a select event is fired on the radio group. This will happen whenever a radio button is selected. This function will retrieve the currently selected radio element using the selectedIndex property. Note that even though one of the radio buttons is inside an hbox, it is still part of the radio group. If the first radio button is selected (index of 0), the textbox is enabled by setting its disabled property to true. If the second radio button is selected, the textbox is enabled.

The next section will provide more details about manipulating radio groups as well as manipulating lists.

« Previous
Next »
Document Tags and Contributors
Tags: 

    DOM Tutorials XUL XUL_Tutorial 

Contributors to this page: Sheppy, teoli, fscholz, realraven, trevorh, jswisher, waldir, DavidWhitten, Chbok, Nathymig, Morishoji, Mgjbot, Pmash, Nickolay, Ptak82, Dria
Last updated by: Sheppy, Apr 14, 2014, 10:34:49 AM
Learn the best of web development

Get the latest and greatest from MDN delivered straight to your inbox.
E-mail
MDN Web Docs

    Web Technologies
    Learn Web Development
    About MDN
    Feedback

Mozilla

    About
    Contact Us
    Firefox

Other languages:

    Terms Privacy Cookies 

© 2005-2018 Mozilla and individual contributors.

Content is available under these licenses.


    Skip to main content
    Select language
    Skip to search

MDN Web Docs

    Technologies References & Guides Feedback 

    Sign in

Search

Edit

Manipulating Lists

« Previous
Next »

The XUL listbox provides a number of specialized methods.
List Manipulation

The listbox element provides numerous methods to retrieve and manipulate its items. Although listboxes may be manipulated using the standard DOM functions as well, it is recommended that the specialized listbox functions be used instead when possible. These functions are bit simpler and will do the right thing.

The appendItem() function is used to append a new item to the end a list. Similar to the DOM appendChild() function except that it takes a string label, and you do not have to worry about where to add it in the list structure. Here is an example:

Example 1 : Source View

<script>
function addItem(){
  document.getElementById('thelist').appendItem("Thursday", "thu");
}
</script>

<listbox id="thelist"/>

<button label="Add" oncommand="addItem();"/>

The appendItem() takes two arguments, the label, in this case 'Thursday', and a value 'thu'. The two arguments correspond to the label attribute and the value attribute on the listitem element. The value is used only as an extra optional value associated with an item which you might wish to use in a script.

Similarly, there is also an insertItemAt() and a removeItemAt() function which inserts a new item and removes an existing item respectively. The syntax is as follows:

list.insertItemAt(3, "Thursday", "thu");
list.removeItemAt(3);

The insertItemAt() function takes an additional argument, the position to insert the new item. The new item is inserted at this index, so, in the example, the new item will be added at position 3 while the item previously at that position will now be at position 4. Remember that the first item is 0. The removeItemAt() function will remove the item at a specific index.

These three methods are also available for several other XUL elements and work in the same manner. In fact, these methods are part of the nsIDOMXULSelectControlElement interface so any XUL elements which implement this interface have these methods. This includes the menulist, radiogroup and tabs elements. For example, to add a new item to a menulist, you can use the same syntax as for a listbox. The right kind of element will be appended in each situation.
List Selection

The nsIDOMXULSelectControlElement interface provides two additonal properties, selectedIndex and selectedItem. The former returns the index of the selected item while the latter returns the selected element. For instance the selectedItem of a menulist will be the menuitem that is selected. If no item is selected, selectedIndex will return -1, while selectedItem will return null.
Getting the selected item

These two properties are commonly inspected during a select event, as in the following example:

Example 2 : Source View

<listbox id="thelist" onselect="alert(this.selectedItem.label);">
  <listitem label="Short"/>
  <listitem label="Medium"/>
  <listitem label="Tall"/>
</listbox>

The select event is fired for a listbox when an item in the list is selected. The select handler here displays an alert containing the label of the selected item in the list. Since the select event fired we can assume that an item is selected. In other cases, you may wish to check to ensure that selectedItem is not null before continuing.

The select event is also fired when a radio button in a radiogroup is selected and when a tab is selected in a tabs element. However, menulists do not fire the select event; instead you can listen to the command event to handle when an item is selected.

For the tabs element, it is often more convenient to use functions of the tabbox element instead. It also has a selectedIndex function which will return the index of the selected tab. However, to get the selected item, use the tabbox's selectedTab property instead. Or, use the selectedPanel property to get the selected panel, that is, return the content associated with the tab.
Changing the selection

All of the selection related properties described above may also be assigned a new value to change the selection. In the next example, the selectedIndex property of a radiogroup element is changed based on the value entered in a textbox. This code isn't foolproof though; for example it doesn't check if the value entered is out of range. You will want to make sure that you add this kind of error checking.

Example 3 : Source View

<script>
function doSelect(){
  var val = document.getElementById('number').value;
  val = Number(val);
  if (val != null)
    document.getElementById('level').selectedIndex = val - 1;
}
</script>

<hbox align="center">
  <label value="Enter a number from 1 to 3:"/>
  <textbox id="number"/>
  <button label="Select" oncommand="doSelect();"/>
</hbox>

<radiogroup id="level">
  <radio label="Excellent"/>
  <radio label="Good"/>
  <radio label="Poor"/>
</radiogroup>

Listboxes also support multiple selection and the functions of the nsIDOMXULMultiSelectControlElement interface. This interface provides a number of functions specifically for handling multiple selection. For example the selectedItems property holds a list of the items that are selected, while the selectedCount property holds the number of items selected. Typically, you will use these properties to iterate over the list and perform some operation for each item. Be careful when iterating over the selected items; if you modify the items in the list while iterating, the list will change and the selection properties may return different values. This is one reason why it is useful to manipulate the list by the item rather than by index.
Deleting selected items

The following example shows a method of deleting the selected items properly:

Example 4 : Source View

<script>
function deleteSelection(){
  var list = document.getElementById('thelist');
  var count = list.selectedCount;
  while (count--){
    var item = list.selectedItems[0];
    list.removeItemAt(list.getIndexOfItem(item));
  }
}
</script>

<button label="Delete" oncommand="deleteSelection();"/>

<listbox id="thelist" seltype="multiple">
  <listitem label="Cheddar"/>
  <listitem label="Cheshire"/>
  <listitem label="Edam"/>
  <listitem label="Gouda"/>
  <listitem label="Havartie"/>
</listbox>

Inside the while loop we

    first get the selectedItem at index 0. The first selected item is always retrieved as the size of the array will decrease as the items are removed.
    Next, we remove the item using the removeItemAt() function. Since this function requires an index
    we can convert between an item and an index using the getIndexOfItem() function. There is also a corresponding getItemAtIndex() function to do the reverse.

The nsIDOMXULMultiSelectControlElement interface also provides methods for modifying the selected items. For instance, the addItemToSelection() function adds a new item to the set of selected items, without clearing the existing selection. The removeItemFromSelection() function removes a single item from the selection.
List Scrolling

If there are more rows in the listbox than can be displayed, a scroll bar will appear to allow the user to scroll the list. The scroll position may be adjusted using a couple of listbox methods.

The scrollToIndex() method scrolls to a given row. This listbox will scroll such that the row will be the top row visible, unless the row is near the end of the list of items. The ensureIndexIsVisible() method is similar in that it also scrolls to show a row, but this method does not scroll if the item is already visible. Thus, the former function is used to scroll to a specific row while the latter is used just to make sure that a row is visible. There is also an ensureIndexIsVisible() that takes an item as an argument instead of an index. Compare the effect of both functions at different scroll positions in this example:

Example 5 : Source View

<button label="scrollToIndex"
           oncommand="document.getElementById('thelist').scrollToIndex(4);"/>
<button label="ensureIndexIsVisible"
           oncommand="document.getElementById('thelist').ensureIndexIsVisible(4);"/>

<listbox id="thelist" rows="5">
  <listitem label="1"/>
  <listitem label="2"/>
  <listitem label="3"/>
  <listitem label="4"/>
  <listitem label="5"/>
  <listitem label="6"/>
  <listitem label="7"/>
  <listitem label="8"/>
  <listitem label="9"/>
  <listitem label="10"/>
  <listitem label="11"/>
  <listitem label="12"/>
</listbox>

Next, we'll look at XUL box objects.

« Previous
Next »
Document Tags and Contributors
Tags: 

    Tutorials XUL XUL_Tutorial 

Contributors to this page: SphinxKnight, Sheppy, teoli, trevorh, Mixticius, Chbok, Nathymig, Morishoji, Jjinux, Mgjbot, Pmash, Ptak82, Dria
Last updated by: SphinxKnight, Feb 9, 2018, 12:38:47 PM
Learn the best of web development

Get the latest and greatest from MDN delivered straight to your inbox.
E-mail
MDN Web Docs

    Web Technologies
    Learn Web Development
    About MDN
    Feedback

Mozilla

    About
    Contact Us
    Firefox

Other languages:

    Terms Privacy Cookies 

© 2005-2018 Mozilla and individual contributors.

Content is available under these licenses.


    Skip to main content
    Select language
    Skip to search

MDN Web Docs

    Technologies References & Guides Feedback 

    Sign in

Search

Edit

Box Objects

« Previous
Next »

This section describes the box object, which holds display and layout related information about a XUL box as well as some details about XUL layout.
About Mozilla Layout

Mozilla divides things into two sets of trees, the content tree and the layout tree. The content tree stores the nodes as they are found in the source code. The layout tree holds a different tree of nodes for each individual component that can be displayed. The layout tree holds the structure as the nodes are expected to be displayed There is not necessarily a one to one relationship between content and layout nodes. Some content nodes may have several layout objects, for example, each line of a paragraph has a separate layout object. Conversely, some content nodes have no layout objects at all. For instance, the key element doesn't have any layout objects since it isn't displayed in any way. Similarly, any element that has been hidden will not have a layout object either.

The DOM is generally used only to get and modify information pertaining to the content or structure of the document. It's relatively simple to determine what kind of content tree node will be created for a given element. A XUL element, for example, will have a XULElement type of content node.

The layout objects that will be created are determined in a more complex manner. Various pieces of information are used such as the tag name, the attributes on an element, various CSS properties, the elements and layout objects around the element, and the XBL associated with an element (XBL is described in a later section). Unless you change the style for an element, most XUL elements will usually use the box layout object or one of its subtypes. Recall that all XUL elements are types of boxes, that is the box is the base of all displayed XUL elements. However, there are a number of subtypes, about 25 or so, for specific XUL elements. Some of these subtypes, such as the stack or listbox are needed for more complex layouts than the basic box, while others such as the button are used only to add extra mouse and key event handling.

The layout object associated with an element can be removed and a completely different type of object created just by changing the CSS display property, among others. For example changing the display property of a button element to block will cause the button layout object to be deleted and a block object to be created instead. Naturally, this will change the appearance and function of the element.

It isn't necessary to know the details of how the layout objects are constructed but it is quite useful to at least have at least the knowledge of what is described above of XUL layout for more advanced XUL development.
Box Objects

The layout objects are not accessible to the developer for manipulating. They are internal to the XUL layout components. However, XUL provides some helper objects, called box objects, which can provide some layout related information. As the name implies, they are available for all box-based elements.

There are several subtypes of box object, although only a couple of them are generally used. The others have functions which are more easily accessible by methods mapped directly onto the element, since those types are generally only used with one particular element. The base box object, or the interface nsIBoxObject, however, has a number of properties which are quite useful for XUL development.

The base box object has two useful features. First, you can retrieve the position and size of the XUL element as displayed, and second, you can determine the order of the elements in the box as displayed, instead of their order as they are stored in the DOM.
Retrieving Position and Size

The box object provides six read only properties, x, y, screenX, screenY, width and height, for determining the currently displayed position and size of an element.  All values returned are in pixels.

x, y

The x and y coordinates are referenced from the top left corner of the document in the window (that portion which excludes the window border and title bar) and refer to the top left corner of the element, including CSS padding.  CSS margins and borders* are excluded.  The top left corner of the document is that which is obtained from screen.mozInnerScreenX, screen.mozInnerScreenY

screenX, screenY

The screenX and screenY coordinates are referenced from the absolute top left corner of the screen (screen.top,screen.left) and refer to the top left corner of the element, including CSS padding and borders.  CSS margins are excluded.

width, height

The width and height properties are also measured in pixels and return the width and height of the element, including CSS padding and borders.  CSS margins are excluded.
*Note that x, y refers to the portion of the element that is just inside any borders, which is inconsistent with the other four boxObject position and size references, which include borders as part of the element.

The values are always the position and sizes as currently displayed, so the values will be different if the element is moved or resized. For example, a flexible element will change in size and the box object dimensions will update accordingly. The following example shows this.

Example 1 : Source View

<button label="Click Me"
        oncommand="alert('The width is ' + this.boxObject.width);"/>

You can use the width and height attributes of the element to specify the element's width and height, respectively.  Note that retrieving these values will return the size only if it was explicitly specified. These properties will return an empty string if the width or height attributes or properties were not set already. That is, you cannot get the current size with these properties; instead you must use the box object properties.

This may be a bit confusing, but, the key is to remember that the width and height properties on an element return the size that was set in the XUL while the width and height properties of the box object return the current size.
Hidden or Collapsed Element

The hidden attribute will hide an element such that it will not be displayed. Since it is not displayed, all four position and size properties of the box object will have the value 0. When an element is hidden, it is removed from the display and the layout objects are removed for it. Thus, since it isn't displayed anywhere, it will have no position or size.

The collapsed attribute will have the same effect to the user element visually, except that it leaves the element on screen and keeps the layout objects intact, but changes the size of the element to 0. This means that while both hidden and collapsed elements have 0 width and height, hidden elements have a x and y position of 0, while collapsed elements will retain their position in the window.

To retrive or modify the hidden or collapsed state use the corresponding properties as in the following example.

Example 2 : Source View

<script>
function showPositionAndSize()
{
  var labelbox = document.getElementById('thelabel').boxObject;

  alert("Position is (" + labelbox.x + "," + labelbox.y +
        ") and size is (" + labelbox.width + "," +
        labelbox.height + ")");
}
</script>

<button label="Hide"
        oncommand="document.getElementById('thelabel').hidden = true;"/>
<button label="Show"
        oncommand="document.getElementById('thelabel').hidden = false;"/>
<button label="Collapse"
        oncommand="document.getElementById('thelabel').collapsed = true;"/>
<button label="Uncollapse"
        oncommand="document.getElementById('thelabel').collapsed = false;"/>
<button label="Show Position/Size"
        oncommand="showPositionAndSize();"/>
<label id="thelabel" value="I am a label"/>

Note that if you hide and collapse the label, it will be treated as hidden. You will then have to unhide and uncollapse the label for it to appear again.
XUL Element Ordering

The box object may also be used to determine the display order of elements, which may not be the same as the order in the source. Recall that DOM properties such as childNodes, firstChild, and nextSibling may be used to navigate the document tree. The box object also allows navigation in a similar means but retrieves the elements in display order.

The box object provides several properties, firstChild, lastChild, nextSibling, previousSibling, and parentBox. Their function should be self explanatory from their names. These properties return elements, for example, the firstChild property returns the first displayed child element. There is no corresponding childNodes property for box navigation; instead you must navigate by sibling using the nextSibling or previousSibling properties.

Unlike with navigating the DOM tree, hidden elements are not included when navigating by box objects. That means that for a box with six children where the first two are hidden, the firstChild property will return the third element. However, collapsed elements are included since they are still displayed but have no size. For example, the next box sibling of button 1 is this next example will be button 3, because button 2 is hidden. But the next box sibling of button 3 will be button 4 because it is only collapsed.

Example 3 : Source View

<hbox>
  <button label="Button 1"
          oncommand="alert('Next is: ' + this.boxObject.nextSibling.label);"/>
  <button label="Button 2" hidden="true"/>
  <button label="Button 3"
          oncommand="alert('Next is: ' + this.boxObject.nextSibling.label);"/>
  <button label="Button 4" collapsed="true"/>
</hbox>

Box Ordering Attributes

When a XUL box is laid out on a window, the elements are ordered according to a number of properties, for instance the orientation, their ordinal group and their direction.
orient attribute

The orientation is commonly modified using the orient attribute. There is also a corresponding CSS property -moz-box-orient which may be used instead, depending on the situation. This attribute was explained earlier in the section on boxes.
ordinal attribute

The ordinal attribute on an element may be used to specify the ordinal group of the element, or you can use the CSS property -moz-box-ordinal-group.

Elements with a lower ordinal value are placed in the box before elements with a higher ordinal value. For example, if one element has an ordinal of 2, it would be placed before an element with ordinal 3 or higher but after one with ordinal 1. The default value if the ordinal is not specified is 1. This means that if you want to change the displayed order of elements, you will often need to adjust the ordinals of several elements.

Adjusting the ordinal of an element is not commonly done as you would usually just place the elements in a different order in the source. It might be used to rearrange items later without adjusting the DOM. The following example demonstrates this.

Example 4 : Source View

<hbox>
  <button label="One" oncommand="this.ordinal++;"/>
  <button label="Two" oncommand="this.ordinal++;"/>
  <button label="Three" oncommand="this.ordinal++;"/>
</hbox>

If you press the first button, its ordinal will increase by one, from 1 to 2. It will appear at the end of the row since the other buttons have an ordinal of 1. If you press the second button, its ordinal will increase by one and will be moved to the end of the row. Items with the same ordinal value appear in the same order as in the source. If you then press the button labeled One again, its ordinal will increase to 3 and will move to the end. Finally, pressing the button labeled Three will increase its ordinal to 2 and it will appear in between the other two buttons.
dir attribute

The final box ordering attribute is the dir attribute, or the -moz-box-direction CSS property. If this is set to reverse, all of the children in the box are displayed in reverse order. In a horizontal box, the elements will be displayed from right to left; in a vertical box, the elements will be displayed from bottom to top. Here is an example:

Example 5 : Source View

<hbox dir="reverse">
  <button label="Left"/>
  <button label="Center"/>
  <button label="Right"/>
</hbox>

Navigation through the nodes using the box object ordering will return the elements in the order they are displayed accounting for the ordinal adjustments made. Thus, if you change the ordinal of an element, it will have a different position in the box order. Reversing the direction, however, does not change the box order.

Next, we'll find out how to use XPCOM objects from XUL and scripts.

« Previous
Next »
Document Tags and Contributors
Tags: 

    Tutorials XUL XUL_Tutorial 

Contributors to this page: Sheppy, Allasso, teoli, fscholz, trevorh, Mixticius, Chbok, Morishoji, Mgjbot, Pmash, Andreas Wuest, Ptak82, Dria
Last updated by: Sheppy, Apr 14, 2014, 10:34:50 AM
Learn the best of web development

Get the latest and greatest from MDN delivered straight to your inbox.
E-mail
MDN Web Docs

    Web Technologies
    Learn Web Development
    About MDN
    Feedback

Mozilla

    About
    Contact Us
    Firefox

Other languages:

    Terms Privacy Cookies 

© 2005-2018 Mozilla and individual contributors.

Content is available under these licenses.


    Skip to main content
    Select language
    Skip to search

MDN Web Docs

    Technologies References & Guides Feedback 

    Sign in

Search

Edit

XPCOM Interfaces

 

« Previous
Next »

In this section, we'll take a brief look at XPCOM (Cross-platform Component Object Model), which is the Object system that Mozilla uses.
Calling Native Objects

By using XUL we can build a complex user interface. We can attach scripts which modify the interface and perform tasks. However, there are quite a number of things that cannot be performed directly with JavaScript. For example, if we wanted to create a mail application, we would need to write scripts which would connect to mail servers to retrieve and send mail. JavaScript does not have the capability to do such things.

The only way to handle this would be to write native code that would get mail. We also need to have a way for our scripts to call the native code easily. Mozilla provides such a method which involves using XPCOM (Cross-platform Component Object Model).
Mozilla provides many XPCOM components and interfaces. So, in most cases, you don't need to write native code for yourself. After learning this section, you can search suitable interfaces using XULPlanet XPCOM Reference
About XPCOM

Mozilla is constructed from a collection of components, each of which performs a certain task. For example, there is a component for each menu, button and element. The components are constructed from a number of definitions called interfaces.

An interface in Mozilla is a definition of a set of functionality that could be implemented by components. Components are what implement the code in Mozilla that does things. Each component implements the functionality as described by interfaces. A single component might implement multiple interfaces. And multiple components might implement the same interface.

Let's take an example of a file component. An interface would need to be created which describes properties and functions that can be performed on files. A file would need properties for its name, modification date and its size. Functions of a file would include moving, copying and deleting it.

The File interface only describes the characteristics of a file, it does not implement it. The implementation of the File interface is left to a component. The component will have code which can retrieve the file's name, date and size. In addition, it will have code which copies and renames it.

We don't care how the component implements it, as long as it implements the interface correctly. Of course, we'll have different implementations anyway, one for each platform. The Windows and Macintosh versions of a file component would be significantly different. However, they would both implement the same interface. Thus, we can use a component by accessing it using the functions we know from the interface.

In Mozilla, interfaces are usually preceded by 'nsI' or 'mozI' so that they are easily recognized as interfaces. For example, the nsIAddressBook is the interface for interacting with an address book, nsISound is used for playing files and nsILocalFile is used for files. For more interfaces in Mozilla, see Interfaces.

XPCOM components are typically implemented natively, which means that they generally do things that JavaScript cannot do itself. However, there is a way in which you can call them, which we will see shortly. We can call any of the functions provided by the component as described by the interfaces it implements. For example, once we have a component, we can check if it implements nsISound, and, if so, we can play sound through it.

The process of calling XPCOM from a script is called XPConnect, which is a layer which translates script objects into native objects.
Creating XPCOM Objects

There are three steps to calling an XPCOM component.

    Get a component
    Get the part of the component that implements the interface that we want to use.
    Call the function we need

Once you've done the first two steps, you can repeat the last step as often as necessary. Let's say we want to rename a file. For this we can use the nsILocalFile interface. The first step is getting a file component. Second, we query the file component and get the portion of it that implements the nsILocalFile interface. Finally, we call functions provided by the interface. This interface is used to represent a single file.

We've seen that interfaces are often named starting with 'nsI' or 'mozI'. Components, however, are referred to using a URI like string. Mozilla stores a list of all the components that are available in its own registry. A particular user can install new components as needed. It works much like plug-ins.

Mozilla provides a file component, that is, a component that implements nsILocalFile. This component can be referred to using the string '@mozilla.org/file/local;1'. This string is called a contract ID. The syntax of a contract ID is:

@<internetdomain>/module[/submodule[...]];<version>[?<name>=<value>[&<name>=<value>[...]]]

Other components can be referred to in a similar way.

The contract ID of the component can be used to get the component. You can get a component using JavaScript code like that below:

var aFile = Components.classes["@mozilla.org/file/local;1"].createInstance();

The file component is retrieved and stored in the aFile variable. Components in the example above refers to a general object that provides some component related functions. Here, we get a component class from the classes property. The classes property is an array of all of the available components. To get a different component, just replace the contract ID inside the square brackets with the contract ID of the component you want to use. Finally, an instance is created with the createInstance() function.

You should check the return value of createInstance() to ensure that it is not null, which would indicate that the component does not exist.

However, at this point, we only have a reference to the file component itself. In order to call the functions of it we need to get one of its interfaces, in this case nsILocalFile. A second line of code needs to be added as follows:

var aFile = Components.classes["@mozilla.org/file/local;1"].createInstance();
if (aFile) aFile.QueryInterface(Components.interfaces.nsILocalFile);

The function QueryInterface() is a function provided by all components which can be used to get a specific interface of that component. This function takes one parameter, the interface that you want to get. The interfaces property of the Components object contains a list of all the interfaces that are available. Here, we use the nsILocalFile interface and pass it as a parameter to QueryInterface(). The result is that aFile will be a reference to the part of the component that implements the nsILocalFile interface.

The two JavaScript lines above can be used to get any interface of any component. Just replace the component name with the name of the component you want to use and change the interface name. You can also use any variable names of course. For example, to get a sound interface, you can do the following:

var sound = Components.classes["@mozilla.org/sound;1"].createInstance();
if (sound) sound.QueryInterface(Components.interfaces.nsISound);

XPCOM interfaces can inherit from other interfaces. The interfaces that inherit from others have their own functions and the functions of all the interfaces that they inherit from. All interfaces inherit from a top-level interface called nsISupports. It has one function supplied to JavaScript, QueryInterface(), which we have already seen. Because the interface nsISupports is implemented by all components, the function QueryInterface() function is available in every component.

Several components may implement the same interface. Typically, they might be subclasses of the original but not necessarily. Any component may implement the functionality of nsILocalFile. In addition, a component may implement several interfaces. It is for these reasons that two steps are involved in getting an interface to call functions through.

However, there is a shortcut we can use because we'll often use both of these lines together:

var aLocalFile = Components.classes["@mozilla.org/file/local;1"].createInstance(Components.interfaces.nsILocalFile);

This will do the same thing as the two lines but in one line of code. It eliminates the need to create the instance and then query it for an interface in two separate steps.

If you call QueryInterface() on an object and the requested interface is not supported by an object, an exception is thrown. If you're not sure that an interface is supported by a component, you can use the instanceof operator to check:

var aFile = Components.classes["@mozilla.org/file/local;1"].createInstance();
if (aFile instanceof Components.interfaces.nsILocalFile){
  // do something
}

The instanceof operator returns true if aFile implements the nsILocalFile interface. This also has the side effect of calling QueryInterface(), so aFile will be a valid nsILocalFile afterwards.
Calling the Functions of an Interface

Now that we have an object that refers to a component with the nsILocalFile interface, we can call the functions of nsILocalFile through it. The table below shows some of the properties and methods of the nsILocalFile interface.

initWithPath 
    This method is used to initialize the path and filename for the nsILocalFile. The first parameter should be the file path, such as '/usr/local/mozilla'.
leafName 
    The filename without the directory part.
fileSize 
    The size of the file.
isDirectory() 
    Returns true if the nsILocalFile represents a directory.
remove(recursive) 
    Deletes a file. If the recursive parameter is true, a directory and all of its files and subdirectories will also be deleted.
copyTo(directory,newname) 
    Copies a file to another directory, optionally renaming the file. The directory should be a nsILocalFile holding the directory to copy the file to.
moveTo(directory,newname) 
    Moves a file to another directory, or renames a file. The directory should be a nsILocalFile holding the directory to move the file to.

In order to delete a file we first need to assign a file to the nsILocalFile. We can call the method initWithPath() to indicate which file we mean. Just assign the path of the file to this property. Next, we call the remove() function. It takes one parameter which is whether to delete recursively. The code below demonstrates these two steps:

var aFile = Components.classes["@mozilla.org/file/local;1"].createInstance();
if (aFile instanceof Components.interfaces.nsILocalFile){
  aFile.initWithPath("/mozilla/testfile.txt");
  aFile.remove(false);
}

This code will take the file at /mozilla/testfile.txt and delete it. Try this example by adding this code to an event handler. You should change the filename to an existing file that you have that you would like to delete.

In the functions table above, you will see two functions copyTo() and moveTo(). These two functions can be used to copy files and move files respectively. Note that they do not take a string parameter for the directory to copy or move to, but instead take an nsILocalFile. That means that you'll need to get two file components. The example below shows how to copy a file.

function copyFile(sourcefile,destdir)
{
  // get a component for the file to copy
  var aFile = Components.classes["@mozilla.org/file/local;1"]
    .createInstance(Components.interfaces.nsILocalFile);
  if (!aFile) return false;

  // get a component for the directory to copy to
  var aDir = Components.classes["@mozilla.org/file/local;1"]
    .createInstance(Components.interfaces.nsILocalFile);
  if (!aDir) return false;

  // next, assign URLs to the file components
  aFile.initWithPath(sourcefile);
  aDir.initWithPath(destdir);

  // finally, copy the file, without renaming it
  aFile.copyTo(aDir,null);
}

copyFile("/mozilla/testfile.txt","/etc");

XPCOM Services

Some XPCOM components are special components called services. You do not create instances of them because only one should exist. Services provide general functions which either get or set global data or perform operations on other objects. Instead of calling createInstance(), you call getService() to get a reference to the service component. Other than that, services are not very different from other components.

One such service provided with Mozilla is a bookmarks service. It allows you to add bookmarks to the user's current bookmark list. An example is shown below:

var bmarks = Components.classes["@mozilla.org/browser/bookmarks-service;1"].getService();
bmarks.QueryInterface(Components.interfaces.nsIBookmarksService);
bmarks.addBookmarkImmediately("http://www.mozilla.org","Mozilla",0,null);

First, the component "@mozilla.org/browser/bookmarks-service;1" is retrieved and its service is placed in the variable bmarks. We use QueryInterface() to get the nsIBookmarksService interface. The function addBookmarkImmediately() provided by this interface can be used to add bookmarks. The first two parameters to this function are the bookmark's URL and its title. The third parameter is the bookmark type which will normally be 0 and the last parameter is the character encoding of the document being bookmarked, which may be null.

Next, we will see some of the interfaces provided with Mozilla that we can use.

« Previous
Next »
Document Tags and Contributors
Tags: 

    Tutorials XPCOM:Language Bindings XPConnect XUL XUL_Tutorial 

Contributors to this page: SphinxKnight, stephaniehobson, Sheppy, Szopen.Xiao, teoli, alzhu, Chbok, CN, Morishoji, Mgjbot, Nickolay, Pmash, Ptak82, Dria
Last updated by: SphinxKnight, Dec 15, 2017, 2:42:55 PM
Learn the best of web development

Get the latest and greatest from MDN delivered straight to your inbox.
E-mail
MDN Web Docs

    Web Technologies
    Learn Web Development
    About MDN
    Feedback

Mozilla

    About
    Contact Us
    Firefox

Other languages:

    Terms Privacy Cookies 

© 2005-2018 Mozilla and individual contributors.

Content is available under these licenses.


    Skip to main content
    Select language
    Skip to search

MDN Web Docs

    Technologies References & Guides Feedback 

    Sign in

Search

Edit

XPCOM Examples

« Previous
Next »

This section provides some examples of using XPCOM along with some additional interfaces.
Window Management

This example contains RDF datasource that will be seen in the later section. You might skip this example for the time being, except when you have already had that knowledge.
Creating a Window menu

The list of currently open Mozilla windows can be used as an RDF datasource. This allows you to create a Window menu with a list of the currently open windows in the application. The datasource for this is rdf:window-mediator. We can use this as in the following example:

Example 1 : Source

<toolbox>
 <menubar id="windowlist-menubar">
  <menu label="Window">
   <menupopup id="window-menu" datasources="rdf:window-mediator" ref="NC:WindowMediatorRoot">
    <template>
     <rule>
      <menuitem uri="rdf:*" label="rdf:http://home.netscape.com/NC-rdf#Name"/>
     </rule>
    </template>
   </menupopup>
  </menu>          
 </menubar>
</toolbox>

A Window menu will be created with a list of all the open windows. Try this example by opening a number of browser windows and you'll see that they are all listed on the menu.
Window mediator component

This is fine for displaying a list of open windows, but we would like to enhance this so that clicking on the menu item will switch to that window. This is accomplished by using the window mediator component. It implements the interface nsIWindowDataSource. The code below shows how to get a component which implements it:

var wmdata = Components.classes["@mozilla.org/rdf/datasource;1?name=window-mediator"].getService();
wmdata.QueryInterface(Components.interfaces.nsIWindowDataSource);

This code retrieves a window mediator data source component. The component used here is the same one that handles the window-mediator RDF datasource. You can also get this component through the RDF service, which is another service that manages RDF datasources.

The nsIWindowDataSource interface has a function getWindowForResource, which can be used to get the window given a resource. In the earlier example, we generated the list of windows and added it to a menu via a template. The template generates an id attribute on each menuitem element. The value of this attribute can be used as the resource. That means that in order to switch the window focus, we need to do the following:

    Determine the element that the user clicked on.
    Get the value of the id attribute from the element.
    Pass this value to getWindowForResource() to get a window object.
    Switch the focus to this window.

The example below shows how we might do this:

<toolbox>
 <menubar id="windowlist-menubar">
  <menu label="Window" oncommand="switchFocus(event.target);">
   <menupopup id="window-menu" datasources="rdf:window-mediator" ref="NC:WindowMediatorRoot">
    <template>
     <rule>
      <menuitem uri="rdf:*" label="rdf:http://home.netscape.com/NC-rdf#Name"/>
     </rule>
    </template>
   </menupopup>
  </menu>          
 </menubar>
</toolbox>

<script>
function switchFocus(elem)
{
  var mediator = Components.classes["@mozilla.org/rdf/datasource;1?name=window-mediator"].getService();
  mediator.QueryInterface(Components.interfaces.nsIWindowDataSource);

  var resource = elem.getAttribute('id');
  switchwindow = mediator.getWindowForResource(resource);

  if (switchwindow){
    switchwindow.focus();
  }
}
</script>

A command handler was added to the menu element which calls the function switchFocus() with a parameter of the element that was selected from the menu. The function switchFocus():

    first gets a reference to a component which implements the window mediator interface.
    Next, we get the id attribute for the element. We can use the value of the id attribute as the resource.
    The function getWindowForResource() takes the resource and returns a window that matches it. This window, stored in the switchwindow variable, is the same as the JavaScript window object.
    This means that you can call any of the functions provided by it, one of which is the focus() function.

Cookies

Next, we will get a list of cookies that have been saved in the browser. The cookie service can be used for such a purpose. It implements the nsICookieManager interface which can be used to enumerate over all of the cookies. Here is an example which populates a menu list with the names of all of the cookies set from MozillaZine.

<script>

function getCookies()
{
  var menu = document.getElementById("cookieMenu");
  menu.removeAllItems();

  var cookieManager = Components.classes["@mozilla.org/cookiemanager;1"]
                        .getService(Components.interfaces.nsICookieManager);

  var iter = cookieManager.enumerator;
  while (iter.hasMoreElements()){
    var cookie = iter.getNext();
    if (cookie instanceof Components.interfaces.nsICookie){
      if (cookie.host == "www.mozillazine.org")
        menu.appendItem(cookie.name,cookie.value);
    }
  }
}
</script>

<hbox>
  <menulist id="cookieMenu" onpopupshowing="getCookies();"/>
</hbox>

The getCookies() function will be called whenever the menu is opened, as indicated by the onpopupshowing attribute on the menulist. The first two lines of getCookies() get the menulist and remove all of the existing items in the menu. This is done because getCookies() is called every time the menu is opened and we don't want to leave the old items there each time.

Next, the cookie manager is retrieved. The cookie manager has an enumerator method which returns an object which implements nsISimpleEnumerator. This can be used to iterate over all of the cookies. An enumerator has a hasMoreElements() method which will return true until we get to the last cookie. The getNext() method gets a cookie and moves the enumerator index to the next cookie. Since an enumerator just returns a generic object, we need to QueryInterface() it to an nsICookie before we can use it. In this case, we just use the instanceof operator to accomplish this.

Finally, an item is added to the menu for the cookie. The host, name and value properties of the cookie are used for this. Menus have an appendItem() function which can be used to add an item to the menu, given a label and a value.
See also

More examples are available below.

    Code snippets
    http://kb.mozillazine.org/Category:XPCOM_example_code

Next, we'll look at how to create trees.

« Previous
Next »
Document Tags and Contributors
Tags: 

    Tutorials XUL XUL_Tutorial 

Contributors to this page: Sheppy, teoli, trevorh, TomC, Chbok, CN, Morishoji, Mgjbot, Ptak82, SylvainPasche, Pmash, Dria
Last updated by: Sheppy, Apr 14, 2014, 10:35:00 AM
Learn the best of web development

Get the latest and greatest from MDN delivered straight to your inbox.
E-mail
MDN Web Docs

    Web Technologies
    Learn Web Development
    About MDN
    Feedback

Mozilla

    About
    Contact Us
    Firefox

Other languages:

    Terms Privacy Cookies 

© 2005-2018 Mozilla and individual contributors.

Content is available under these licenses.


    Skip to main content
    Select language
    Skip to search

MDN Web Docs

    Technologies References & Guides Feedback 

    Sign in

Search

Edit

Trees

« Previous
Next »

XUL provides a way to create tabular or hierarchical lists using a tree.
The Tree

One of the more complex elements in XUL is the tree. A tree may be used to display rows of text in columns. It can be used with rows either in a flat list or arranged into a hierarchy. A tree also allows the user to rearrange, resize and hide individual columns. Some examples of trees include the list of messages in a mail application, or the Bookmarks window in Mozilla.

In some ways, a tree has some similarities with the listbox. Both can be used to create tables of data with multiple rows and columns, and both may contain column headers. The tree also supports nested rows, whereas the listbox does not. However, listboxes may contain any type of content, whereas trees may only contain text and images. (Using advanced features, progress meters or checkboxes also can be added to the tree)

A tree consists of two parts, the set of columns, and the tree body.

    The set of columns is defined by a number of treecol elements, one for each column. Each column will appear as a header at the top of the tree.
    The second part, the tree body, contains the data to appear in the tree and is created with a treechildren tag.

The tree is unique in that the body of the tree consists only of a single widget which draws all of the data in the tree. This contrasts with the listbox, where individual listitem and listcell tags are used to specify the rows in the listbox. In a tree, all of the data to be displayed is supplied by a separate object, called a tree view. When it comes time to display a cell, the tree widget will call out to this tree view to determine what to display, which in turn will be drawn by the tree. The tree is smart enough to only ask for information from the view for those rows that need to be displayed. This allows the view to be optimized such that it only needs to load the data for displayed content. For instance, a tree might have thousands of rows, yet most of them will be scrolled off the border of the tree, hidden from view. This means that the tree is scalable to any number of rows without any performance problems. Of course, this is independant of the performance of the view object itself.

A tree view is an object which implements the nsITreeView interface. This interface contains thirty properties and functions which you may implement. These functions will be called by the tree as necessary to retrieve data and state about the tree. For instance, the getCellText() function will be called to get the label for a particular cell in the tree.

An advantage of using a tree view is that it allows the view to store the data in a manner which is more suitable for the data, or to load the data on demand as rows are displayed. This allows more flexibility when using trees.

Naturally, having to implement a tree view with thirty or so properties and methods for every tree can be very cumbersome, especially for simple trees. Fortunately, XUL provides a couple of built-in view implementations which do most of the hard work for you. For most trees, especially when you first start to use trees, you will use one of these built-in types. However, you can create a view entirely from scratch if necessary. If you do, you might store the data in an array or JavaScript data structure, or load the data from an XML file.

Since the entire body of the tree is a single widget, you can't change the style of individual rows or cells in the normal way. This is because there are no elements that display the individual cells, like there is with the listbox. Instead, all drawing is done by the tree body using data supplied by the view. This is an important point and many XUL developers have trouble understanding this aspect. To modify the appearance of a tree cell, the view must instead associate a set of keywords for a row and cell. A special CSS syntax is used which styles components of the tree body with those keywords. In a sense, it is somewhat like using CSS classes. Tree styling will be discussed in detail in a later section.
Tree Elements

Trees can be created with the tree element, which is described in the following sections. There are also two elements used to define the columns to be displayed in the tree.

tree
    This is the outer element of a tree.
treecols
    This element is a placeholder for a set of treecol elements.
treecol
    This is used to declare a column of the tree. By using this element, you can specify additional information about how the data in the columns are sorted and if the user can resize the columns. You should always place an id attribute on a column, as Mozilla uses the ids to identify the columns when rearranging and hiding them. This is no longer required in Mozilla 1.8 and later, but it is still a good idea to use ids on columns.
treechildren
    This contains the main body of the tree where the individual rows of data will be displayed.

Example tree with two columns

Example 1 : Source View

<tree flex="1">

  <treecols>
    <treecol id="nameColumn" label="Name" flex="1"/>
    <treecol id="addressColumn" label="Address" flex="2"/>
  </treecols>

  <treechildren/>

</tree>

Trees-Example 1

First, the entire table is surrounded with a tree element. This declares an element that is used as a table or tree. As with HTML tables, the data in a tree is always organized into rows. The columns are specified using the treecols tag.

You may place as many columns as you wish in a tree. As with listboxes, a header row will appear with column labels. A drop-down menu will appear in the upper-right corner of the tree, which the user may use to show and hide individual columns. Each column is created with a treecol element. You can set the header label using the label attribute. You may also want to make the columns flexible if your tree is flexible, so that the columns stretch as the tree does. In this example, the second column will be approximately twice as wide as the first column. All of the columns should be placed directly inside a treecols element.

In this case we haven't specified a view to supply the tree's data, so we'll only see column headers and an empty tree body. You may have to resize the window to see anything since there isn't any data to display. Since the tree has been marked as flexible, the body will stretch to fit the available space. Making a tree flexible is quite commonly done, as it is often the case that the data in the tree is the most significant information displayed, so it makes sense to make the tree grow to fit. However, you may specify a specific number of rows to appear in a tree by setting the rows attribute on the tree element. This attribute specifies how many rows are displayed in the user interface, not how many rows of data there are. The total number of rows is supplied by the tree view. If there are more rows of data in the tree, a scrollbar will appear to allow the user to see the rest of them. If you don't specify the rows attribute, the default value is 0, which means that none of the rows will appear. In this case, you would make the tree flexible. If your tree is flexible, it doesn't need a rows attribute since it will grow to fit the available space.
The Content Tree View

Having said that the data to be displayed in a tree comes from a view and not from XUL tags, there happens to be a built-in tree view which gets its data from XUL tags. This may be a bit confusing, but essentially, one of the built-in tree views uses a set of tags which can be used to specify information about the data in the tree. The following tags are used:

treeitem
    This contains a single parent row and all its descendants. This element also serves as the item which can be selected by the user. The treeitem tag would go around the entire row so that it is selectable as a whole.
treerow
    A single row in the tree, which should be placed inside a treeitem tag.
treecell
    A single cell in a tree. This element would go inside a treerow element.

Conveniently, these tags may be placed directly inside the treechildren tag, nested in the order above. The tags define the data to be displayed in the tree body. In this case, the tree uses the built-in tree view, called a content tree view, which uses the labels and values specified on these elements as the data for the tree. When the tree needs to display a row, the tree asks the content tree view for a cell's label by calling the view's getCellText function, which in turn gets the data from the label of the appropriate treecell.

However, the three elements listed above are not displayed directly. They are used only as the source for the data for the view. Thus, only a handful of attributes apply to the treeitem and related elements. For instance, you cannot change the appearance of the tree rows using the style attribute or with other CSS properties and the box related features such as flexibility and orientation cannot be used. In addition, mouse and other user interface events do not fire at these elements.

In fact, apart from some tree specific attributes, the only attributes that will have any effect will be the label attribute to set a text label for a cell and the src attribute to set an image. However, there are special ways of styling the tree and setting other features which we will see in later sections.

Also, events do not get sent to treeitem element and their children; instead they get sent to the treechildren element.

That the treeitems are unlike other XUL elements is a common source of confusion for XUL developers. Essentially, the tree content view is a view where the data for the cells is supplied from tags placed inside the tree. Naturally, if you are using a different kind of view, the data will be supplied from another source, and there won't be any treeitem elements at all.

Let's start by looking at how to create a simple tree with multiple columns using the tree content view. This could be used to create a list of mail messages. There might be multiple columns, such as the sender and the subject.
Example tree with treechildren

Example 2 : Source View

<tree flex="1">

  <treecols>
    <treecol id="sender" label="Sender" flex="1"/>
    <treecol id="subject" label="Subject" flex="2"/>
  </treecols>

  <treechildren>
    <treeitem>
      <treerow>
        <treecell label="joe@somewhere.com"/>
        <treecell label="Top secret plans"/>
      </treerow>
    </treeitem>
    <treeitem>
      <treerow>
        <treecell label="mel@whereever.com"/>
        <treecell label="Let's do lunch"/>
      </treerow>
    </treeitem>
  </treechildren>

</tree>

Trees-Example 2

As can be seen in the image, the tree has been created with two rows of data.

This tree has two columns, the second of which will take up more space than the first. You will usually make the columns flexible. You can also supply widths with the width attribute. You should include the same number of treecol elements as there are columns in the tree. Otherwise strange things might happen.

The header row is created automatically. The button in the upper right corner can be used to hide and show the columns. You can place a hidecolumnpicker attribute on the tree and set it to true if you would like to hide this button. If this button is hidden, the user will not be able to hide columns.

Make sure that you set an id attribute on each column or the hiding and showing of columns will not work in all versions of Mozilla.

The treechildren element surrounds all of the rows. Inside the body are individual rows, which may in turn contain other rows. For a simpler tree, each row is created with the treeitem and treerow elements. The treerow element surrounds all of the cells in a single row, while a treeitem element would surround a row and all of its children. Trees with nested rows are described in the next section.

Inside the rows, you will put individual tree cells. These are created using the treecell element. You can set the text for the cell using the label attribute. The first treecell in a row determines the content that will appear in the first column, the second treecell determines the content that will appear in the second column, and so on.

The user can select the treeitems by clicking on them with the mouse, or by highlighting them with the keyboard. The user can select multiple items by holding down the Shift or Control keys and clicking additional rows. To disable multiple selection, place a seltype attribute on the tree, set to the value single. With this, the user may only select a single row at a time.
Add a tree to our find files example

Let's add a tree to the find files window where the results of the search would be displayed. The tree will use a content tree view. The following code should be added in place of the iframe.

<tree flex="1">
  <treecols>
    <treecol id="name" label="Filename" flex="1"/>
    <treecol id="location" label="Location" flex="2"/>
    <treecol id="size" label="Size" flex="1"/>
  </treecols>

  <treechildren>
   <treeitem>
     <treerow>
       <treecell label="mozilla"/>
       <treecell label="/usr/local"/>
       <treecell label="2520 bytes"/>
     </treerow>
   </treeitem>
  </treechildren>
</tree>

<splitter collapse="before" resizeafter="grow"/>

We've added a tree with three columns for the filename, the location and the file size. The second column will appear twice as wide due to the larger flexibility. A single row has been added to demonstrate what the table would look like with a row. In a real implementation, the rows would be added by a script as the search was performed, or a custom view would be created to hold the data.

Find files example so far : Source View

Next, we'll learn how to create more advanced trees.

« Previous
Next »
Document Tags and Contributors
Tags: 

    Tutorials XUL XUL_Tutorial 

Contributors to this page: ygkorea, Sheppy, morkil, teoli, trevorh, mnoorenberghe, fscholz, Enn, Morishoji, Chbok, EkinoX, Nathymig, Pmash, Mgjbot, Ptak82, Ted_Mielczarek, Dria
Last updated by: ygkorea, Feb 1, 2016, 11:02:15 PM
Learn the best of web development

Get the latest and greatest from MDN delivered straight to your inbox.
E-mail
MDN Web Docs

    Web Technologies
    Learn Web Development
    About MDN
    Feedback

Mozilla

    About
    Contact Us
    Firefox

Other languages:

    Terms Privacy Cookies 

© 2005-2018 Mozilla and individual contributors.

Content is available under these licenses.


    Skip to main content
    Select language
    Skip to search

MDN Web Docs

    Technologies References & Guides Feedback 

    Sign in

Search

Edit

More Tree Features
Jump to:

    Hierarchical treesMore about Tree Columns 

« Previous
Next »

Here, we'll see more features of trees.
Hierarchical trees

The tree element is also used to create hierarchical lists, like that found in a file manager or a browser's bookmarks list. The tree view has a number of functions which specify the hierarchy of the items in a tree. Each item in the tree has a level starting at 0. The topmost items in the tree will have a level of 0, the children of those items will have a level of 1, the children below that will have a level of 2, and so on. The tree will query the view for the level of each item in order to determine how to draw the rows.

The tree will draw the open and close icons to open and close a parent item as well as lines connecting the children to their parents. The tree will also handle drawing the rows with the right level of indenting. However, the view will need to make sure it keeps track of the level of the rows as necessary. This can sometimes be quite tricky, but fortunately, the built-in content tree view does all of the hard work for us.

To create a set of nested rows, all we need to do is add a second treechildren element inside the parent treeitem. You can then add items inside that to specify the child rows of an item. Don't put the inner treechildren element inside the treerow as this won't work.

You can repeat this process to create deeply nested trees. Essentially, a treeitem element can contain either single rows which are declared with the treerow element or a set of rows which are declared with the treechildren element.

There are two other things you need to do to make sure the hierarchy works properly. First, you need to mark the treeitem element that has children as a container. You do this by adding the container attribute to it as follows:

<treeitem container="true" />

This allows the user to click on the row that corresponds to the treeitem to expand and collapse the inner rows. You can have the child rows initially displayed by adding the open attribute. When the user expands and collapses the parent, the view's toggleOpenState function will be called to toggle the item between open and closed. For a content tree view, this will set the open attribute to reflect the current state.

The second change is that you must put the primary attribute on the first column. This causes a small triangle or plus sign to appear before the cells in that column to indicate the cells that can be expanded. In addition, child rows are indented. Note also that the user cannot hide the primary column using the drop down to the right of the columns.
Example hierarchical tree

The following is a simple example:

Example 1 : Source View

<tree rows="6">
  <treecols>
      <treecol id="firstname" label="First Name" primary="true" flex="3" />
      <treecol id="lastname" label="Last Name" flex="7" />
  </treecols>

  <treechildren>
    <treeitem container="true" open="true">
      <treerow>
        <treecell label="Guys" />
      </treerow>

      <treechildren>
        <treeitem>
          <treerow>
            <treecell label="Bob" />
            <treecell label="Carpenter" />
          </treerow>
        </treeitem>
        <treeitem>
          <treerow>
            <treecell label="Jerry" />
            <treecell label="Hodge" />
          </treerow>
        </treeitem>
      </treechildren>
    </treeitem>
  </treechildren>
</tree>

This has created a hierarchical tree. As can be seen in the image, a small plus or minus sign (often called a twisty) has appeared next to the first row, indicating that it contains child rows. By clicking the row, the user can open and close the list. The child rows are indented. Notice how the Guys row only needs one column as it is a placeholder item for its children.

The outer treeitem element contains a single treerow element and a treechildren element. The former creates the data for the parent row and the latter contains the child items.

You can nest rows deeper as well. Remember that you must use the container attribute on rows which contain child rows. The simple presence of child rows isn't sufficient to indicate this, as you may have a container that has no children but should still be treated like a container. For example, a directory with no files in it should still be treated like a container whereas a file should not.
More about Tree Columns

One additional attribute you can add to the tree is enableColumnDrag. (Note the mixed case.) If set to true, the user may drag the column headers around to rearrange the order of the columns.

A user will also likely want to change the column widths. You can do this by placing a splitter element in between each treecol element. A small notch will appear in between each column header which the user may drag to change the width of a column. You can use the style class tree-splitter to hide the notch, although the column may still be resized.

You can set a minimum or maximum width of a column using the minwidth or maxwidth attributes.

You can set the hidden attribute on a column to true to have the column hidden by default. The user can choose to show the column by selecting it from the drop-down on the end of the header row.
Remembering State of Columns

As with all elements, the persist attribute can be used to save the state of the columns in-between sessions. (That feature will see in the later section). Thus, once the user has decided on a column layout they like, it will automatically be saved for next time. You will need to save a number of attributes as indicated in the example below:

Example 2 : Source View

<tree enableColumnDrag="true" flex="1">
  <treecols>
    <treecol id="runner" label="Runner" flex="2" persist="width ordinal hidden" />
    <splitter class="tree-splitter" />
    <treecol id="city" label="Home City" flex="2" persist="width ordinal hidden" />
    <splitter class="tree-splitter" />
    <treecol id="starttime" label="Start Time" flex="1" persist="width ordinal hidden" />
    <splitter class="tree-splitter" />
    <treecol id="endtime" label="End Time" flex="1" persist="width ordinal hidden" />
  </treecols>

  <treechildren>
    <treeitem>
      <treerow>
        <treecell label="Joshua Granville" />
        <treecell label="Vancouver" />
        <treecell label="7:06:00" />
        <treecell label="9:10:26" />
      </treerow>
    </treeitem>
    <treeitem>
      <treerow>
        <treecell label="Robert Valhalla" />
        <treecell label="Seattle" />
        <treecell label="7:08:00" />
        <treecell label="9:15:51" />
      </treerow>
    </treeitem>
  </treechildren>
</tree>

Three attributes of the columns must be persisted,

    the width attribute to save the column widths,
    the ordinal attribute which holds the position of the column,
    and the hidden attribute which holds whether the column is hidden or visible.

Next, we'll find out to get and set the selection for trees.

« Previous
Next »
Document Tags and Contributors
Tags: 

    Tutorials XUL XUL_Tutorial 

Contributors to this page: Sheppy, ethertank, teoli, fscholz, Brettz9, Chbok, Mgjbot, Ptak82, Morishoji, Pmash, Takenbot, Dria
Last updated by: Sheppy, Apr 14, 2014, 10:35:00 AM
Learn the best of web development

Get the latest and greatest from MDN delivered straight to your inbox.
E-mail
MDN Web Docs

    Web Technologies
    Learn Web Development
    About MDN
    Feedback

Mozilla

    About
    Contact Us
    Firefox

Other languages:

    Terms Privacy Cookies 

© 2005-2018 Mozilla and individual contributors.

Content is available under these licenses.


    Skip to main content
    Select language
    Skip to search

MDN Web Docs

    Technologies References & Guides Feedback 

    Sign in

Search

Edit

Tree Selection

 

« Previous
Next »

The section will describe how to get and set the selected items in a tree.
Getting the Selected Tree Items

Each item in a tree (that corresponds to treeitem element, if using content tree view) may be selected individually. If you add the seltype attribute to the tree, set to the value single, the user may only select one row at a time. Otherwise, the user may select multiple rows, which will not necessarily be contiguous. The tree provides a number of functions which can be used to determine whether an item is selected.
Handling the Select Event

First, let's see how we can determine when an item is selected. The onselect() event handler may be added to the tree element. When the user selects an item from the tree, the event handler is called. The user may also change the selection by using the cursor keys. If the user holds down the cursor key to rapidly scroll through the items, the event handler is not called until the user stops. This results in a performance improvement. This also means that the highlight will appear on several items even though the select event is never fired for those items.

The syntax of the onselect() event handler is shown below.

<tree id="treeset" onselect="alert('You selected something!');">

Tree Indices

The tree has a property currentIndex, which can be used to get the currently selected item, where the first row is 0.

Child items are included in the count just after their parents. This means that if there are 3 top-level items and each has two child items, there will be a total of 9 items. The first item (at index 0) will be the first top-level item. The next item at index 1 will be its first child. The second child will be at index 2 and the second parent item will be at position 3 and so on.
Image:seltree1.png

In the image, there are eight rows displayed, of which two are selected. The first selected row has an index of 4 and the second has an index of 7. The rows that are not displayed are not included in the index count.
Multiple Selection

For trees that allow multiple selection, getting the list of selected rows is a bit more complicated. The tree's view has a selection property which holds information about the selected rows. This selection will be an nsITreeSelection object. The view doesn't need to implement this object itself, the tree will assign a selection object to the view's selection property when the view is attached to a tree. From the tree, you can get the selection using the tree's view property and then retrieve the view's selection property. You can use the methods of the selection object to retrieve the set of selected items or modify the selection.

Because the selected items in a multiple selection tree are not necessarily contiguous, you can retrieve each block of contigous selections using the getRangeCount() and getRangeAt() functions. The first function returns the number of selection ranges there are. If only one value is selected, this value will be 1. You would then write a loop for the number of ranges, calling getRangeAt() to get the actual indices of the start and end of the range.

The getRangeAt() function takes three arguments.

    The first is the range index.
    The second is an object which will be filled in by the function with the index of the first selected item.
    The third argument is an object which will be filled in with the index of the last selected item.

getRangeAt Example

var start = new Object();
var end = new Object();
var numRanges = tree.view.selection.getRangeCount();

for (var t = 0; t < numRanges; t++){
  tree.view.selection.getRangeAt(t,start,end);
  for (var v = start.value; v <= end.value; v++){
    alert("Item " + v + " is selected.");
  }
}

We create two objects called 'start' and 'end'. Then, we iterate over the set of ranges, the number of which is returned by the getRangeCount() function. The getRangeAt() function is called passing the range index and the start and end objects. This function will fill in the start and end indicies by assigning them to the 'value' property. So if the first range is from the third item to the seventh item, 'start.value' will be 2 (remember that indices start with 0, so we subtract one.) and 'end.value' will be 6. An alert is displayed for each index.

If you just want to find out if a specific row is selected, use can use the isSelected() function. It takes a row index as an argument and returns true if that row is selected.

alert(tree.view.selection.isSelected(3));

Modifying the Tree Selection

The selection object has a number of functions which may be used to change the selection. The simplest function is the select() function, which deselects any rows that are currently selected and selects one specific row. For example, the following code will select the row at index 5:

tree.view.selection.select(5);

Note that you should not just change the tree's currentIndex property to change the selection. Instead, you should use the selection's select function as in the example above. You can select all rows with the selectAll() function. Note that rows nested inside containers that are not open will not be selected. Naturally, this will only have any effect for trees that use multiple selection. There is also a clearSelection() function to clear the selection, and an invertSelection function to reverse the selection, that is, deselect all selected rows and select all unselected rows.

To select specific rows, use the rangedSelect() function which selects all rows in between two indices. Here is an example which selects rows between index 2 and 7. Note that rows 2 and 7 will also be selected.

tree.view.selection.rangedSelect(2,7,true);

The last argument indicates whether to add to the current selection or not. If true, the range will be added to the existing selection. If false, all existing selected rows will be deselected first. Finally, the clearRange() function may be used to deselect a range of rows, leaving rows outside the range unaffected.

tree.view.selection.clearRange(2,7);

Next, we'll find out how to create a custom view for a tree.

« Previous
Next »
Document Tags and Contributors
Tags: 

    Tutorials XUL XUL_Tutorial 

Contributors to this page: Sheppy, teoli, fscholz, Rcampbell, Wjjohnst, Julen, Chbok, Mgjbot, Ptak82, Pmash, Enn, Morishoji, Dria
Last updated by: Sheppy, Apr 14, 2014, 10:34:57 AM
Learn the best of web development

Get the latest and greatest from MDN delivered straight to your inbox.
E-mail
MDN Web Docs

    Web Technologies
    Learn Web Development
    About MDN
    Feedback

Mozilla

    About
    Contact Us
    Firefox

Other languages:

    Terms Privacy Cookies 

© 2005-2018 Mozilla and individual contributors.

Content is available under these licenses.


    Skip to main content
    Select language
    Skip to search

MDN Web Docs

    Technologies References & Guides Feedback 

    Sign in

Search

Edit

Custom Tree Views

« Previous
Next »

The tree view holds the data to be displayed in the tree.
Creating a Custom View

So far, we have only been using the built-in content tree view. In this section, we will look at creating a custom view. This is necessary when the page has a lot of data or that data is highly nested. For instance, 5000 rows of treeitems would load too slowly. You might also implement a custom view when you want to perform computations on the data to be displayed. Since the view can store and retrieve the data in the most suitable manner for the kind of data used, the tree can be used even when there are hundreds of thousands of rows to be displayed.
Note: The tree-related interfaces changed in Gecko 1.8. See Tree Widget Changes for details.

To implement a custom view, you will need to create an object which implements the nsITreeView interface. You can create these objects in JavaScript, but you will need a separate object for each tree. Naturally, since a custom tree view is being used, the content tree view will not be used, so the treeitem, treerow, and treecell elements will have no purpose since the custom view will get its data from elsewhere. Thus, you can just leave the treechildren element empty. The following example shows this:

<tree id="my-tree" flex="1">
  <treecols>
    <treecol id="namecol" label="Name" flex="1"/>
    <treecol id="datecol" label="Date" flex="1"/>
  </treecols>
  <treechildren/>
</tree>

To assign data to be displayed in the tree, the view object needs to be created which is used to indicate the value of each cell, the total number of rows plus other optional information. The tree will call methods of the view to get the information that it needs to display.

In general, although the tree view has thirty or so functions that may be implemented, you only need to implement the ones that the tree will call. Three methods that you should implement are listed below.

rowCount
    This property should be set to the total number of rows in the tree.

getCellText( row , column )
    This method should return the text contents at the specified row and column. This will be called to display data for each cell. The rows are supplied as numeric values starting at 0. The columns are TreeColumn objects. In the older versions of Mozilla (before Firefox 1.5 or Mozilla 1.8), the columns are supplied as the values of the id attribute on the columns. If you need a id attribute like older versions, the id property of TreeColumn can be used.
setTree( tree )
    This method is called once to set the tree element on the view.

Here is an example of defining such as object, which can be called whatever you want:

//Moz 1.8
var treeView = {
    rowCount : 10000,
    getCellText : function(row,column){
      if (column.id == "namecol") return "Row "+row;
      else return "February 18";
    },
    setTree: function(treebox){ this.treebox = treebox; },
    isContainer: function(row){ return false; },
    isSeparator: function(row){ return false; },
    isSorted: function(){ return false; },
    getLevel: function(row){ return 0; },
    getImageSrc: function(row,col){ return null; },
    getRowProperties: function(row,props){},
    getCellProperties: function(row,col,props){},
    getColumnProperties: function(colid,col,props){}
};

The functions in the example not described above do not need to perform any action, but they must be implemented as the tree calls them to gather additional information.

This example can be used for a tree with 10,000 rows. The contents of the cells in the first column will be set to the text 'Row X' where X is the row number. The contents of the cells in the second column will be set to 'February 18'. The if statement in the function getCellText() compares the id property of the column argument to the text 'namecol'. This text 'namecol' corresponds to the id of the first treecol in the example above. This example is very simple of course -- in reality you would have more complex data in each cell.

The final step is to associate the view object with the tree. The tree has a property view, which can be assigned to the view object declared above. We can assign a value to this property at any time to set or change the view.

function setView(){
    document.getElementById('my-tree').view = treeView;
}

The following presents the example together. An inline script has been used here to simplify the example. Normally, you would put the script in an external script file.
Example custom tree

Source
Image:treeview1.png

<?xml version="1.0"?>
<?xml-stylesheet href="chrome://global/skin/" type="text/css"?>

<window title="Tree Example" id="tree-window"
   xmlns="http://www.mozilla.org/keymaster/gatekeeper/there.is.only.xul"
   onload="setView();">

<script>
//Moz 1.8
var treeView = {
    rowCount : 10000,
    getCellText : function(row,column){
      if (column.id == "namecol") return "Row "+row;
      else return "February 18";
    },
    setTree: function(treebox){ this.treebox = treebox; },
    isContainer: function(row){ return false; },
    isSeparator: function(row){ return false; },
    isSorted: function(){ return false; },
    getLevel: function(row){ return 0; },
    getImageSrc: function(row,col){ return null; },
    getRowProperties: function(row,props){},
    getCellProperties: function(row,col,props){},
    getColumnProperties: function(colid,col,props){}
};

function setView(){
    document.getElementById('my-tree').view = treeView;
}
</script>

<tree id="my-tree" flex="1">
  <treecols>
    <treecol id="namecol" label="Name" flex="1"/>
    <treecol id="datecol" label="Date" flex="1"/>
  </treecols>
  <treechildren/>
</tree>

</window>

In the image, you can see two columns, each with data taken from the getCellText() function. The setView() function has been called in the onload() handler for the window, but you could also set the view later if you wish. You can change the view at any time.

One thing to note is that the getCellText() function is only called when necessary to display the contents. In the 10,000 row example above, getCellText() is only called for the cells that are currently displayed. In the image, only seven rows are displayed, the last only partially, so getCellText() will be called only 14 times, one for each row and column. It is called for other rows when the user scrolls through them. This makes the tree much more efficient.

Note that the view object is also available for trees using the built-in content view. You can use this to get the cell labels and other information.

The nsITreeView interface lists all of the properties and methods that you can implement for the tree view. We'll look at more of these in the next section.

Next, we'll look at more features of tree views.

« Previous
Next »
Document Tags and Contributors
Tags: 

    Tutorials XUL XUL_Tutorial 

Contributors to this page: isomorphismes, Sheppy, teoli, trevorh, fscholz, Sevenspade, Brettz9, Chbok, Ptak82, Mgjbot, Morishoji, Pmash, Cverdon, Nickolay, Callek, Klfoster, Dria, Enn
Last updated by: isomorphismes, Jun 7, 2017, 1:35:38 PM
Learn the best of web development

Get the latest and greatest from MDN delivered straight to your inbox.
E-mail
MDN Web Docs

    Web Technologies
    Learn Web Development
    About MDN
    Feedback

Mozilla

    About
    Contact Us
    Firefox

Other languages:

    Terms Privacy Cookies 

© 2005-2018 Mozilla and individual contributors.

Content is available under these licenses.


    Skip to main content
    Select language
    Skip to search

MDN Web Docs

    Technologies References & Guides Feedback 

    Sign in

Search

Edit

Tree View Details

« Previous
Next »

This section will describe some more features of tree views.
Creating a Hierarchical Custom View

In the last section, we created a simple tree view that implemented only a minimum amount of functionality. Next, let's look at some additional functions that views may implement. Here, we will examine how to create a hierarchical set of items using the view. This is a fairly tricky process as it involves keeping track of which items have children and also which rows are open and closed.
Nesting Level

Every row in the tree has a nesting level. The topmost rows are at level 0, the children of those rows are at level 1, their children at level 2 and so on. The tree will query the view for each row by calling its getLevel method to find out the level of that row. The view will need to return 0 for the outermost rows and higher values for inner rows. The tree will use this information to determine the hierarchical structure of the rows.

In addition to the getLevel method, there is a hasNextSibling function which, given a row, should return true if there is another row afterwards at the same level. This function is used, specifically, to draw the nesting lines along the side of the tree.

The getParentIndex method is expected to return the parent row of a given row, that is, the row before it with a lower nesting value. All of these methods must be implemented by the view for children to be handled properly.
Containers

There are also three functions, isContainer, isContainerEmpty and isContainerOpen that are used to handle a parent item in the tree.

    The isContainer method should return true if a row is a container and might contain children.
    The isContainerEmpty method should return true if a row is an empty container, for instance, a directory with no files in it.
    The isContainerOpen method is used to determine which items are opened and closed. The view is required to keep track of this. The tree will call this method to determine which containers are open and which are closed.

Note that the tree will call neither isContainerEmpty nor isContainerOpen for rows that are not containers as indicated by the return value of the isContainer method.

A container may be rendered differently than a non-container. For instance, a container may have a folder icon beside it. A style sheet may be used to style items based on various properties such as whether a row is open. This is described in a later section. A non-empty container will be displayed with a twisty next to it so that the user may open and close the row to see child items. Empty containers will not have a twisty, but will still be treated like a container.

When the user clicks the twisty to open a row, the tree will call the view's toggleOpenState method. The view should then perform any necessary operations to retrieve the child rows and then update the tree with the new rows.
Note: As of this writing (Gecko 2.0), custom nsITreeView implementations must be prepared to handle a call to toggleOpenState for any row index which returns true for a call to isContainer, regardless of whether the container is empty.
Review of the Methods

Here is a review of the methods needed to implement hierarchical views:

getLevel(row)
hasNextSibling(row, afterIndex)
getParentIndex(row)
isContainer(row)
isContainerEmpty(row)
isContainerOpen(row)
toggleOpenState(row)

The afterIndex argument to hasNextSibling function is used as optimization to only start looking for the next sibling after that point. For instance, the caller might already know where the next sibling might possibly be. Imagine a situation where a row had subrows and those subrows had child rows of their own and several are open. It could take a while in some implementations to determine what the next sibling's row index would be in such a case.
Example of Hierarchical Custom View

Let's put this together into a simple example that takes an array and constructs a tree from it. This tree will only support a single parent level with an inner child level, but it could be extended to support additional levels without too much effort. We'll examine it piece by piece.

<window onload="init();"
        xmlns="http://www.mozilla.org/keymaster/gatekeeper/there.is.only.xul">

<tree id="elementList" flex="1">
  <treecols>
    <treecol id="element" label="Element" primary="true" flex="1"/>
  </treecols>
  <treechildren/>
</tree>

</window>

We use a simple tree here with no data in the treechildren. The 'init' function is called when the window is loaded to initialize the tree. It simply sets the custom view by retrieving the tree and setting its 'view' property. We will define 'treeView' next.

function init() {
  document.getElementById("elementList").view = treeView;
}

The custom tree view will need to implement a number of methods, of which the important ones will be examined individually. First we'll define two structures to hold the data for the tree, the first will hold a map between parents and the children they contain, and the second will hold an array of the visible items. Remember that a custom view must keep track of which items are visible itself.

var treeView = {
  childData : {
    Solids: ["Silver", "Gold", "Lead"],
    Liquids: ["Mercury"],
    Gases: ["Helium", "Nitrogen"]
  },

  visibleData : [
    ["Solids", true, false],
    ["Liquids", true, false],
    ["Gases", true, false]
  ],

The childData structure holds an array of the children for each of the three parent nodes. The visibleData array begins with only three items visible, the three top level items. Items will be added and removed from this array when items are opened or closed. Essentially, when a parent row is opened, the children will be taken from the childData map and inserted into the visibleData array. For example, if the Liquids row was opened, the corresponding array from childData, which in this case contains only the single Mercury child, will be inserted into the visibleData array after Liquids but before Gases. This will increase the array size by one. The two booleans in each row in the visibleData structure indicate whether a row is a container and whether it is open respectively. Obviously, the new inserted child items will have both values set to false.
Implement the Tree View Interface

Next, we need to implement the tree view interface. First, the simple functions:

treeBox: null,
selection: null,
get rowCount()                     { return this.visibleData.length; },
setTree: function(treeBox)         { this.treeBox = treeBox; },
getCellText: function(idx, column) { return this.visibleData[idx][0]; },
isContainer: function(idx)         { return this.visibleData[idx][1]; },
isContainerOpen: function(idx)     { return this.visibleData[idx][2]; },
isContainerEmpty: function(idx)    { return false; },
isSeparator: function(idx)         { return false; },
isSorted: function()               { return false; },
isEditable: function(idx, column)  { return false; },

The rowCount function will return the length of the visibleData array. Note that it should return the current number of visible rows, not the total. So, at first, only three items are visible and the rowCount should be three, even though six rows are hidden. Also note the use of the JavaScript get operator to bind a property to a function, so to have the value of the rowCount property that can change dynamically over time, as rowCount has to be a read-only attribute as defined in nsITreeView.

The setTree function will be called to set the tree's box object. The tree box object is a specialized type of box object specific to trees and will be examined in detail in the next section. It is used to aid in drawing the tree. In this example, we will only need one function of the box object, to be able to redraw the tree when items are added or removed.

The getCellText, isContainer and isContainerOpen functions just return the corresponding element from the visibleData array. Finally, the remaining functions can just return false since we don't need those features. If we had a row that had no children we would want to implement the isContainerEmpty function so that it returned true for those elements.

getParentIndex: function(idx) {
    if (this.isContainer(idx)) return -1;
    for (var t = idx - 1; t >= 0 ; t--) {
      if (this.isContainer(t)) return t;
    }
  },

The getParentIndex will need to find the parent of a given index. In our simple example, there are only two levels, so we know that containers don't have parents, so -1 is returned for these items. Otherwise, we just iterate backwards through the rows looking for one that is a container. Next, the getLevel function:

getLevel: function(idx) {
    if (this.isContainer(idx)) return 0;
    return 1;
  },

The getLevel function is simple. It just returns 0 for container rows and 1 for non-containers. If we wanted to add an additional level of children, those rows would have a level of 2.

hasNextSibling: function(idx, after) {
    var thisLevel = this.getLevel(idx);
    for (var t = after + 1; t < this.visibleData.length; t++) {
      var nextLevel = this.getLevel(t);
      if (nextLevel == thisLevel) return true;
      if (nextLevel < thisLevel) break;
    }
    return false;
  },

The hasNextSibling function needs to return true if there is a row at the same level after a given row. The code above uses a brute force method which simply iterates over the rows looking for one, returning true if a row exists with the same level and false once it finds a row that has a lower level. In this simple example, this method is fine, but a tree with a larger set of data will want to use a more optimal method of determining whether a later sibling exists.
Opening or Closing a Row

The final function of note is toggleOpenState, which is the most complex. It needs to modify the visibleData array when a row is opened or closed.

toggleOpenState: function(idx) {
    var item = this.visibleData[idx];
    if (!item[1]) return;

    if (item[2]) {
      item[2] = false;

      var thisLevel = this.getLevel(idx);
      var deletecount = 0;
      for (var t = idx + 1; t < this.visibleData.length; t++) {
        if (this.getLevel(t) > thisLevel) deletecount++;
        else break;
      }
      if (deletecount) {
        this.visibleData.splice(idx + 1, deletecount);
        this.treeBox.rowCountChanged(idx + 1, -deletecount);
      }
    }
    else {
      item[2] = true;

      var label = this.visibleData[idx][0];
      var toinsert = this.childData[label];
      for (var i = 0; i < toinsert.length; i++) {
        this.visibleData.splice(idx + i + 1, 0, [toinsert[i], false]);
      }
      this.treeBox.rowCountChanged(idx + 1, toinsert.length);
    }
    this.treeBox.invalidateRow(idx);
  },

First we will need to check if the row is a container. If not, the function just returns since non-containers cannot be opened or closed. Since the third element in the item array (with an index of 2) holds whether the row is open or not, we use two code paths, the first to close a row and the second to open a row. Let's examine each block of code, but let's look at the second block for opening a row first.

item[2] = true;

      var label = this.visibleData[idx][0];
      var toinsert = this.childData[label];
      for (var i = 0; i < toinsert.length; i++) {
        this.visibleData.splice(idx + i + 1, 0, [toinsert[i], false]);
      }
      this.treeBox.rowCountChanged(idx + 1, toinsert.length);

The first line makes the row open in the array so that we will know the next time the toggleOpenState function is called that the row will need to be closed instead. Next, we look up the data in the childData map for the row. The result is that 'toinsert' will be set to one of the child arrays, for example ["Silver", "Gold", "Lead"] if the Solids row is the one being opened. Next, we use the array's splice function to insert a new row for each item. For Solids, three items will be inserted.

Finally, the tree box's rowCountChanged function needs to be called. Recall that treeBox is a tree box object and was set earlier by a call to the setTree function. The tree box object will be created by the tree for you and you can call its functions. In this case, we

use the rowCountChanged function to inform the tree that some rows were added to the underlying data. The tree will then redraw the tree as needed and the result is that the child rows will appear inside the container. The various other functions implemented above such as getLevel and isContainer are used by the tree to determine how to draw the tree.

The rowCountChanged function takes two arguments, the index where the first row was inserted and the number of rows to insert. In the code above we indicate that the starting row is the value of 'idx' plus one, which will be the first child under the parent. The tree will use this information and add space for the appropriate number of rows and push the rows afterwards down. Make sure to pass the right number or the tree might redraw incorrectly or try to draw more rows than necessary.

The following code is used to delete rows when a row is closed.

item[2] = false;

      var thisLevel = this.getLevel(idx);
      var deletecount = 0;
      for (var t = idx + 1; t < this.visibleData.length; t++) {
        if (this.getLevel(t) > thisLevel) deletecount++;
        else break;
      }
      if (deletecount) {
        this.visibleData.splice(idx + 1, deletecount);
        this.treeBox.rowCountChanged(idx + 1, -deletecount);
      }

First, the item is set closed in the array. Then, we scan along the rows until we come to one that is at the same level. All those that have a higher level will need to be removed, but a row at the same level will be the next container which should not be removed.

Finally, we use the splice function to remove the rows from the visibleData array and call the rowCountChanged function to redraw the tree. When deleting rows, you will need to supply a negative count of the number of rows to delete.

Whether opening or closing a row, we need to tell the tree to repaint the twisty in the new state. The easiest way to do this is to invalidate the row.
Complete Example

There are several other view functions we can implement but they don't need to do anything in this example, so we can create functions that do nothing for those. They are added near the end of the complete example, shown here:

<?xml version="1.0"?>
<?xml-stylesheet href="chrome://global/skin/" type="text/css"?>

<window onload="init();"
        xmlns="http://www.mozilla.org/keymaster/gatekeeper/there.is.only.xul">

<tree id="elementList" flex="1">
  <treecols>
    <treecol id="element" label="Element" primary="true" flex="1"/>
  </treecols>
  <treechildren/>
</tree>

<script>
<![CDATA[

var treeView = {
  childData : {
    Solids: ["Silver", "Gold", "Lead"],
    Liquids: ["Mercury"],
    Gases: ["Helium", "Nitrogen"]
  },

  visibleData : [
    ["Solids", true, false],
    ["Liquids", true, false],
    ["Gases", true, false]
  ],

  treeBox: null,
  selection: null,

  get rowCount()                     { return this.visibleData.length; },
  setTree: function(treeBox)         { this.treeBox = treeBox; },
  getCellText: function(idx, column) { return this.visibleData[idx][0]; },
  isContainer: function(idx)         { return this.visibleData[idx][1]; },
  isContainerOpen: function(idx)     { return this.visibleData[idx][2]; },
  isContainerEmpty: function(idx)    { return false; },
  isSeparator: function(idx)         { return false; },
  isSorted: function()               { return false; },
  isEditable: function(idx, column)  { return false; },

  getParentIndex: function(idx) {
    if (this.isContainer(idx)) return -1;
    for (var t = idx - 1; t >= 0 ; t--) {
      if (this.isContainer(t)) return t;
    }
  },
  getLevel: function(idx) {
    if (this.isContainer(idx)) return 0;
    return 1;
  },
  hasNextSibling: function(idx, after) {
    var thisLevel = this.getLevel(idx);
    for (var t = after + 1; t < this.visibleData.length; t++) {
      var nextLevel = this.getLevel(t);
      if (nextLevel == thisLevel) return true;
      if (nextLevel < thisLevel) break;
    }
    return false;
  },
  toggleOpenState: function(idx) {
    var item = this.visibleData[idx];
    if (!item[1]) return;

    if (item[2]) {
      item[2] = false;

      var thisLevel = this.getLevel(idx);
      var deletecount = 0;
      for (var t = idx + 1; t < this.visibleData.length; t++) {
        if (this.getLevel(t) > thisLevel) deletecount++;
        else break;
      }
      if (deletecount) {
        this.visibleData.splice(idx + 1, deletecount);
        this.treeBox.rowCountChanged(idx + 1, -deletecount);
      }
    }
    else {
      item[2] = true;

      var label = this.visibleData[idx][0];
      var toinsert = this.childData[label];
      for (var i = 0; i < toinsert.length; i++) {
        this.visibleData.splice(idx + i + 1, 0, [toinsert[i], false]);
      }
      this.treeBox.rowCountChanged(idx + 1, toinsert.length);
    }
    this.treeBox.invalidateRow(idx);
  },

  getImageSrc: function(idx, column) {},
  getProgressMode : function(idx,column) {},
  getCellValue: function(idx, column) {},
  cycleHeader: function(col, elem) {},
  selectionChanged: function() {},
  cycleCell: function(idx, column) {},
  performAction: function(action) {},
  performActionOnCell: function(action, index, column) {},
  getRowProperties: function(idx, prop) {},
  getCellProperties: function(idx, column, prop) {},
  getColumnProperties: function(column, element, prop) {},
};

function init() {
  document.getElementById("elementList").view = treeView;
}

]]></script>

</window>

Next, we'll look in more detail at the tree box object.

« Previous
Next »
Document Tags and Contributors
Tags: 

    Tutorials XUL XUL_Tutorial 

Contributors to this page: Sheppy, teoli, fscholz, Sevenspade, julienfabre, madarche, Neil, Brettz9, MykMelez, Chbok, Nickolay, Jjinux, Mgjbot, Morishoji, Pmash, Ptak82, Dria
Last updated by: Sheppy, Apr 14, 2014, 10:34:49 AM
Learn the best of web development

Get the latest and greatest from MDN delivered straight to your inbox.
E-mail
MDN Web Docs

    Web Technologies
    Learn Web Development
    About MDN
    Feedback

Mozilla

    About
    Contact Us
    Firefox

Other languages:

    Terms Privacy Cookies 

© 2005-2018 Mozilla and individual contributors.

Content is available under these licenses.


    Skip to main content
    Select language
    Skip to search

MDN Web Docs

    Technologies References & Guides Feedback 

    Sign in

Search

Edit

Tree Box Objects
Jump to:

    About the Tree Box ObjectCell Coordinates 

« Previous
Next »

This section will describe the tree box object used to handle how a tree is displayed.
About the Tree Box Object

Box objects were described in an earlier section. The tree box object is a specialized box object used specifically for trees. The tree box implements the TreeBoxObject interface.
Redrawing the Tree

We already saw the rowCountChanged() function of the tree box object in the previous section. It is used to indicate that one or more rows have been added to the tree or removed from the tree. The tree will redraw the affected area. You don't need to call the rowCountChanged() function when a row has simply changed in some way, for example if a cell's label changes. In this case, there are other drawing functions that can be used. The simplest is to call invalidateRow() which will redraw a specific row in the tree. The tree will call the view to get the updated data and update the contents of the tree on screen.

Other redrawing functions are

    invalidateCell() to redraw only a single cell
    invalidateColumn() to redraw a column
    invalidateRange() to redraw a range of rows
    invalidate() to redraw the entire tree.

Note that redrawing does not occur until the calling script ends since Mozilla does not redraw in the background.
Note: it is not necessary to run tree.boxObject.QueryInterface(Components.interfaces.nsITreeBoxObject) as shown in the code examples on this page because:

let boxobject = tree.treeBoxObject;

Note: is equivalent to:

let boxobject = tree.boxObject;
boxobject.queryInterface("Components.interfaces.nsITreeBoxObject");

Scrolling the Tree

You can also scroll the tree using four different methods, similar to those available for listboxes. The scrollToRow() function may be used to scroll to a particular row. Here is a simple example.
Example 1 : Source View

<script>
function doScroll(){
  var value = document.getElementById("tbox").value;
  var tree = document.getElementById("thetree");

  var boxobject = tree.boxObject;
  boxobject.QueryInterface(Components.interfaces.nsITreeBoxObject);
  boxobject.scrollToRow(value);
}
</script>

<tree id="thetree" rows="4">
  <treecols>
    <treecol id="row" label="Row" primary="true" flex="1"/>
  </treecols>
  <treechildren>
    <treeitem label="Row 0"/>
    <treeitem label="Row 1"/>
    <treeitem label="Row 2"/>
    <treeitem label="Row 3"/>
    <treeitem label="Row 4"/>
    <treeitem label="Row 5"/>
    <treeitem label="Row 6"/>
    <treeitem label="Row 7"/>
    <treeitem label="Row 8"/>
    <treeitem label="Row 9"/>
  </treechildren>
</tree>

<hbox align="center">
  <label value="Scroll to row:"/>
  <textbox id="tbox"/>
  <button label="Scroll" oncommand="doScroll();"/>
</hbox>

Note that we use the rows attribute on the tree to specify that only four rows are displayed at a time. This makes it easier to see how the example works. Also, notice that the first row is 0.

The doScroll() function gets the box object and calls the scrollToRow() function with an argument set to the value of the textbox. You might notice that the tree box object can be retrieved in the same way as other box objects using the boxObject (FIXME: dealLink)property, however we need to call QueryInterface() to cast the box object to the more specific tree box object. The functions of the more general box object are also available to trees.

Additional scroll methods include the scrollByLines(), scrollByPages() and ensureRowIsVisible() functions.

The scrollByLines() scrolls up or down by a certain number of rows. Use a positive number to go down and a negative number to go up. The scrollByPages() function scrolls by a number of pages and is called automatically when the user presses the page up or page down keys while the tree is focused. A page is equal to the number of visible rows. For example if the tree shows 10 rows at a time, a page will be equivalent to 10 rows. This is a convenient method since when the user resizes a flexible tree, the page size will grow and shrink, so you don't need to calculate the page size manually. This isn't too hard to calculate manually anyway since the tree box object also provides a getPageLength() function which returns the number of rows in a page. In the scrolling example above, getPageLength() would return four.

Note that in Firefox 1.0 and Mozilla 1.7 and earlier, the getPageLength() function is called getPageCount() instead. The name was changed to getPageLength() since it was confusing before since it doesn't return the number of pages, but the size of each page. You could determine the number of pages by dividing the total number of rows by the page length.

The ensureRowIsVisible() function will scroll to a row just as scrollToRow() does, but does not scroll if the row is already visible.
Cell Coordinates

Some of the most interesting functions of the tree box object are several functions which may be used to get the parts of the tree at specific coordinates and vice versa.

    The getCellAt() function may be used to get the cell at specific pixel location
    The getRowAt() function may be used to get a row at a specific location. The getRowAt() function takes two arguments, the x and y coordinates to use.

tree.boxObject.getRowAt( 50, 100 );

This example will return the index of the row with a horizontal position of 50 and a vertical position of 100. Naturally, it doesn't really matter what the value of the x coordinate is since rows always take up the entire horizontal space of the tree.
One important thing to note is that the coordinates are measured from the upper left corner of the containing document, not the edge of the tree itself.

This makes it easy to pass event coordinates directly to these functions, as in the following example of the getCellAt() function.
Example 2 : Source View

<script>
function updateFields(event){
  var row = {}, column = {}, part = {};
  var tree = document.getElementById("thetree");

  var boxobject = tree.boxObject;
  boxobject.QueryInterface(Components.interfaces.nsITreeBoxObject);
  boxobject.getCellAt(event.clientX, event.clientY, row, column, part);

  if (column.value && typeof column.value != "string")
    column.value = column.value.id;

  document.getElementById("row").value = row.value;
  document.getElementById("column").value = column.value;
  document.getElementById("part").value = part.value;
}
</script>

<tree id="thetree" flex="1" onmousemove="updateFields(event);">
  <treecols>
    <treecol id="utensil" label="Utensil" primary="true" flex="1"/>
    <treecol id="count" label="Count" flex="1"/>
  </treecols>
  <treechildren>
    <treeitem>
      <treerow>
        <treecell label="Fork"/>
        <treecell label="5"/>
      </treerow>
    </treeitem>
    <treeitem>
      <treerow>
        <treecell label="Knife"/>
        <treecell label="2"/>
      </treerow>
    </treeitem>
    <treeitem>
      <treerow>
        <treecell label="Spoon"/>

        <treecell label="8"/>
      </treerow>
    </treeitem>
  </treechildren>
</tree>

<label value="Row:"/>
<label id="row"/>
<label value="Column:"/>
<label id="column"/>
<label value="Child Type:"/>
<label id="part"/>

The getCellAt() function takes five arguments, the coordinates to look up and three out parameters. An out parameter is used since the function needs to return more that one value. You will see a number of interfaces that use out parameters in the XULPlanet object reference (FIXME: deadLink). These are indicated by the word 'out' before the argument. For these, you will need to supply an empty object and the function will fill in the 'value' property with the necessary value.

The three out parameters will be filled in with the row, the column and a child type. The row is the index of the row the mouse is over, since we call it with the event coordinates of a mousemove event. If the coordinate is not over a row, the row value will be set to -1. The column is a column object in Mozilla 1.8 and later. In earlier versions, columns are identified with a string, the id of the column. In later versions, a separate column object exists, which can be queried for column data.

The following line is used so that the example above will work in all versions.

if (column.value && typeof column.value != "string") {
  column.value = column.value.id;
}

If the column is a string, we are running on Mozilla 1.7 or earlier, but for later versions we get the column id from the column object. If you are writing code for multiple versions, you should check for this as above.

The last argument to getCellAt() is the child type which is filled in with a string depending on what part of the cell the coordinate is at. If you move the mouse around in the previous example, you might notice the label change between 'text' and 'cell'. The value 'text' indicates the area where the text would be drawn and the value 'cell' indicates the area around the text, for example, the margin on the left side where the open and close twisties are normally drawn. If there was a twisty, however, the value would be 'twisty' instead. This is convenient since you could determine whether the user clicked on a twisty instead of another part of the row. In fact, this is what the underlying tree code does when the user double clicks the twisty. The final value that may be returned is 'image' if there is an image in the tree cell and the coordinate corresponds to a location where the image is. Of course, in many cases you may not care what part of the cell the coordinate is on and just want the row and column.

To go in reverse and get the coordinates of a specific cell, use the getCoordsForCellItem() function. It takes seven arguments, as described below.

var x = {}, y = {}, width = {}, height = {};
if (typeof tree.columns != "undefined") column = tree.columns[column];
tree.boxObject.getCoordsForCellItem( row, column, part, x, y, width, height );

The row, column, and part arguments are similar to those returned from the getCellAt() function. Again, the column should be either a string or a column object depending on which version you are using. The cell part type may be used to get the coordinates of either the text, the entire cell, the twisty or the image in the cell. The same values as the getCellAt() function are used. The getCoordsForCellItem() function returns the x and y coordinates in addition to the width and height, all as out parameters.

Next, we'll look at RDF which can be used to automatically populate trees and other elements.

« Previous
Next »
Document Tags and Contributors
Tags: 

    Tutorials XUL XUL_Tutorial 

Contributors to this page: stephaniehobson, Allasso, Sheppy, kscarfone, ethertank, teoli, fscholz, julienfabre, Sevenspade, Yaroukh, Svaarala, Chbok, Nathymig, Chris Chittleborough, Mgjbot, Morishoji, Pmash, Ptak82, Dria
Last updated by: stephaniehobson, Oct 21, 2015, 10:31:37 AM
Learn the best of web development

Get the latest and greatest from MDN delivered straight to your inbox.
E-mail
MDN Web Docs

    Web Technologies
    Learn Web Development
    About MDN
    Feedback

Mozilla

    About
    Contact Us
    Firefox

Other languages:

    Terms Privacy Cookies 

© 2005-2018 Mozilla and individual contributors.

Content is available under these licenses.


    Skip to main content
    Select language
    Skip to search

MDN Web Docs

    Technologies References & Guides Feedback 

    Sign in

Search

Edit

Introduction to RDF

 

« Previous
Next »

In this section, we'll look at RDF (Resource Description Framework).
Resource Description Framework

We can use the  tree  elements to display a set of data, such as bookmarks or mail messages. However, it would be inconvenient to do so by entering the data directly into the XUL file. It would make it very difficult to modify the bookmarks if they were directly in the XUL file. The way to solve this is by using RDF datasources.

RDF (Resource Description Framework) is a format that can be used to store resources such as bookmarks or mail. Alternatively, data in other formats can be used and code written that will read the file and create RDF data from it. This is how Mozilla works when it reads data such as bookmarks, the history or mail messages. Mozilla provides datasources for this common data so that you can easily use them.

You can use any of the provided RDF datasources to populate trees with data or you can point to an RDF file stored in XML which contains the data. This makes it very convenient to display trees with lots of rows in them. RDF can also populate other XUL elements as well such as listboxes and menus. We'll see this in the next section.

A very brief overview of RDF will be provided here.  For more information about RDF, see the RDF specification.
RDF/XML

RDF consists of a model, which is a graph representation of data. RDF/XML is an XML language which can be used to represent RDF data. It contains a fairly simple set of elements. The sample below shows a simple RDF template.

<?xml version="1.0"?>
<RDF:RDF
  xmlns:RDF="http://www.w3.org/1999/02/22-rdf-syntax-ns#">
  ...
</RDF:RDF>

This has some similarities to the XUL header. Instead of the window element, the RDF element is used. You can see the namespace for RDF was declared so that the RDF elements are recognized properly. Inside, the RDF element, you will place the data. To see some example RDF/XML files, look at those provided with Mozilla. They have the extension rdf.
RDF database

Let's take the example of a bookmarks list generated from RDF. A bookmarks list contains a set of records, each with a set of data associated with it, such as a URL, a bookmark title and a visited date.

Think of the bookmarks as a database, which is stored as a large table with numerous fields. In the case of RDF however, the lists may be hierarchical as well. This is necessary so that we can have folders or categories of bookmarks. Each of the fields in an RDF database is a resource, each with a name associated with it. The name is described by a URI.

For example, a selection of the fields in the Mozilla bookmark list is described by the URIs below:
Name 	http://home.netscape.com/NC-rdf#Name 	Bookmark name
URL 	http://home.netscape.com/NC-rdf#URL 	URL to link to
Description 	http://home.netscape.com/NC-rdf#Description 	Bookmark description
Last Visited 	http://home.netscape.com/WEB-rdf#LastVisitDate 	Date of last visit

These are generated by taking a namespace name and appending the field name. In the next section, we'll look at how we can use these to fill in the field values automatically. Note that the last visited date has a slightly different namespace than the other three.
RDF/XML file example

Below, a sample RDF/XML file is shown, listing a table with three records and three fields.

<RDF:RDF xmlns:RDF="http://www.w3.org/1999/02/22-rdf-syntax-ns#"
         xmlns:ANIMALS="http://www.some-fictitious-zoo.com/rdf#">

  <RDF:Seq about="http://www.some-fictitious-zoo.com/all-animals">
    <RDF:li>
       <RDF:Description about="http://www.some-fictitious-zoo.com/mammals/lion">
         <ANIMALS:name>Lion</ANIMALS:name>
         <ANIMALS:species>Panthera leo</ANIMALS:species>
         <ANIMALS:class>Mammal</ANIMALS:class>
       </RDF:Description>
    </RDF:li>
    <RDF:li>
       <RDF:Description about="http://www.some-fictitious-zoo.com/arachnids/tarantula">
         <ANIMALS:name>Tarantula</ANIMALS:name>
         <ANIMALS:species>Avicularia avicularia</ANIMALS:species>
         <ANIMALS:class>Arachnid</ANIMALS:class>
       </RDF:Description>
    </RDF:li>
    <RDF:li>
       <RDF:Description about="http://www.some-fictitious-zoo.com/mammals/hippopotamus">
         <ANIMALS:name>Hippopotamus</ANIMALS:name>
         <ANIMALS:species>Hippopotamus amphibius</ANIMALS:species>
         <ANIMALS:class>Mammal</ANIMALS:class>
       </RDF:Description>
    </RDF:li>
  </RDF:Seq>
</RDF:RDF>

Here, three records have been described, one for each animal. Each RDF:Description tag describes a single record. Within each record, three fields are described, name, species and class. It isn't necessary for all records to have the same fields but it makes more sense to have them all the same.

Each of three fields have been given a namespace of ANIMALS, the URL of which has been declared on the RDF tag. The name was selected because it has meaning in this case, but we could have selected something else. The namespace feature is useful because the class field might conflict with that used for styles.

The Seq and li elements are used to specify that the records are in a list. This is much like how HTML lists are declared. The Seq element is used to indicate that the elements are ordered, or in sequence. Instead of the Seq element, you can also use Bag to indicate unordered data, and Alt to indicate data where each record specifies alternative values (such as mirror URLs).

The resources can be referred to in a XUL file by combining the namespace URL followed by the field name. In the example above, the following URIs are generated which can be used to refer to the specific fields:
Name 	http://www.some-fictitious-zoo.com/rdf#name
Species 	http://www.some-fictitious-zoo.com/rdf#species
Class 	http://www.some-fictitious-zoo.com/rdf#class

Next, we'll see how we can use RDF to populate XUL elements.

« Previous
Next »
Document Tags and Contributors
Tags: 

    RDF Tutorials XUL XUL_Tutorial 

Contributors to this page: Sheppy, teoli, fscholz, bovinespirit, alzhu, LJR, Chbok, Nathymig, Morishoji, Mgjbot, Pmash, Ptak82, Dria
Last updated by: Sheppy, Apr 14, 2014, 10:34:47 AM
Learn the best of web development

Get the latest and greatest from MDN delivered straight to your inbox.
E-mail
MDN Web Docs

    Web Technologies
    Learn Web Development
    About MDN
    Feedback

Mozilla

    About
    Contact Us
    Firefox

Other languages:

    Terms Privacy Cookies 

© 2005-2018 Mozilla and individual contributors.

Content is available under these licenses.


    Skip to main content
    Select language
    Skip to search

MDN Web Docs

    Technologies References & Guides Feedback 

    Sign in

Search

Edit

Templates

« Previous
Next »

In this section, we'll find out how to populate elements with data.
Populating Elements

XUL provides a method in which we create elements from data supplied by RDF, either from an RDF file or from an internal datasource. Numerous datasources are provided with Mozilla such as bookmarks, history and mail messages. More details on these will be provided in the later section.

Usually, elements such as treeitems and menuitems will be populated with data. However, you can use other elements if you want although they are more useful for specialized cases. Nevertheless, we'll start with these other elements because trees and menus require more code.

To allow the creation of elements based on RDF data, you need to provide a simple template which will be duplicated for each element that is created. Essentially, you provide only the first element and the remaining elements are constructed based on the first one.

The template is created using the template element. Inside it, you can place the elements that you want to use for each constructed element. The template element should be placed inside the container that will contain the constructed elements. For example, if you are using a tree, you should place the template element inside a tree element.
Simple Template Example

This is better explained with an example. Let's take a simple example where we want to create a button for each top-level bookmark. Mozilla provides a bookmarks datasource so it can be used to get the data. This example will only get the top-level bookmarks (or bookmark folders) as we're going to create buttons. For child bookmarks, we would need to use an element that displays a hierarchy such as a tree or menu.

This example and any others that reference internal RDF datasources will only work if you load them from a chrome URL. For security reasons, Mozilla doesn't allow access to them from other sources.

To view this example, you will need to create a chrome package and load the file from there (you can do this easily, see Manifest Files). You can then enter the chrome URL into the browser URL field.

Example 1 : Source

<vbox datasources="rdf:bookmarks" ref="NC:BookmarksRoot" flex="1">
  <template>
    <button uri="rdf:*" label="rdf:http://home.netscape.com/NC-rdf#Name"/>
  </template>
</vbox>

Image:templates1.jpg

Here, a vertical box has been created that will contain a column of buttons, one for each top-level bookmark. You can see that the template contains a single button. This single button is used as a basis for all the buttons that need to be created. You can see in the image that the set of buttons has been created, one for each bookmark.

Try adding a bookmark in the browser while you have the example window open. You'll notice that the buttons in the example get updated instantly. (You may have to focus the window for it to change.)
Container and Datasources

The template itself is placed inside a vertical box. The box has two special attributes that allow it to be used for templates, which are used to specify where the data comes from. The first attribute on the box is the datasources attribute. This is used to declare what RDF datasource will be providing the data to create the elements. In this case, rdf:bookmarks is used. You can probably guess that this means to use the bookmarks datasource. This datasource is provided by Mozilla. To use your own datasource, specify the URL of an RDF file for the datasources attribute, as indicated in the example below:

<box datasources="chrome://zoo/content/animals.rdf"
     ref="http://www.some-fictitious-zoo.com/all-animals">

You can even specify multiple datasources at a time by separating them with a space in the attribute value. This can be used to display data from multiple sources.

The ref attribute indicates where in the datasource you would like to retrieve data from. In the case of the bookmarks, the value NC:BookmarksRoot is used to indicate the root of the bookmarks hierarchy. Other values that you can use will depend on the datasource you are using. If you are using your own RDF file as a datasource, the value will usually correspond to the value of an about attribute on an RDF Bag, Seq or Alt element.
Inside the Template

By adding these two attributes to the box above, it allows the generation of elements using the template. However, the elements inside the template need to be declared differently. You may notice in the example above that the button has a uri attribute and an unusual value for the label attribute.

An attribute value inside the template that begins with 'rdf:' indicates that the value should be taken from the datasource. In the example earlier, this is the case for the label attribute. The remainder of the value refers to the name property in the datasource. It is constructed by taking the namespace URL used by the datasource and appending the property name. If you don't understand this, try re-reading the last part of the previous section. It explains how resources in RDF can be referred to. Here, we only use the name of the bookmark but numerous other fields are available.

The label of the buttons is set to this special URI because we want the labels on the buttons to be set to the names of the bookmarks. We could have put a URI in any of the attributes of the button, or any other element. The values of these attributes are replaced with data supplied by the datasource which, in this case, is the bookmarks. So we end up with the labels on the buttons set to the names of the bookmarks.

The example below shows how we might set other attributes of a button using a datasource. Of course, this assumes that the datasource supplies the appropriate resources. If a particular one is not found, the value of the attribute will be set to an empty string.

<button class="rdf:http://www.example.com/rdf#class"
        uri="rdf:*"
        label="rdf:http://www.example.com/rdf#name"
        crop="rdf:http://www.example.com/rdf#crop"/>

As you can see, you can dynamically generate lists of elements with the attributes provided by a separate datasource.

The uri attribute is used to specify the element where content generation will begin. Content earlier will only be generated once whereas content inside will be generated for each resource. We'll see more about this when we get to creating templates for trees.
More Examples

By adding these features to the container the template is in, which in this case is a box, and to the elements inside the template, we can generate various interesting lists of content from external data. We can of course put more than one element inside a template and add the special RDF references to the attributes on any of the elements. The example below demonstrates this.

Example 2 : Source

<vbox datasources="rdf:bookmarks" ref="NC:BookmarksRoot" flex="1">
  <template>
    <vbox uri="rdf:*">
      <button label="rdf:http://home.netscape.com/NC-rdf#Name"/>
      <label value="rdf:http://home.netscape.com/NC-rdf#URL"/>
    </vbox>
  </template>
</vbox>

This creates a vertical box with a button and a label for each bookmark. The button will have the name of the bookmark and the label will have the URL.

The new elements that are created are functionally no different from ones that you put directly in the XUL file. The id attribute is added to every element created through a template which is set to a value which identifies the resource. You can use this to identify the resource.

You can also specify mutliple resource values in the same attribute by separating them with a space, as in the example below. More about resource syntax (XULPlanet).

Example 3 : Source

<vbox datasources="rdf:bookmarks" ref="NC:BookmarksRoot"
     flex="1">
  <template>
    <label uri="rdf:*" value="rdf:http://home.netscape.com/NC-rdf#Name rdf:http://home.netscape.com/NC-rdf#URL"/>
  </template>
</vbox>

How Templates are Built

When an element has a datasources attribute, it indicates that the element is expected to be built from a template. Note that it isn't the template tag that determines whether content is built, it is the datasources attribute. When this attribute is present, an object called a Builder is added to the element which is responsible for building the content from the template. In JavaScript you can access the builder object with the builder property, although usually you would only need to do this to have the builder regenerate the content in situations where it is not done automatically.

There are two different types of builders. The first is a content builder and is used in most situations, and the other is a tree builder which is used only for trees.
Content Builder

The content builder takes the content inside the template element and duplicates it for each row. For instance, if the user had ten bookmarks in the example above, ten label elements would be created and added as children of the vbox element. If you were to use DOM functions to traverse the tree, you will find these elements there and can query their properties. These elements get displayed, but the template itself is not displayed, although it still exists in the document tree. In addition, the id of each of the labels will be set to the RDF resource for that row.

The content builder always starts at the place where uri="rdf:*" is specfied. If the uri attribute is placed on an element lower in the element tree, the elements outside are only created once. In the example below, one hbox will be created and it will be filled with a label for each item.

<template>
  <hbox>
    <label uri="rdf:*" value="rdf:http://home.netscape.com/NC-rdf#Name"/>
  </hbox>
</template>

If there is other content inside the element with the datasources attribute but outside the template, that content will also appear. This way, you can mix static content and dynamic content from a template.
Tree Builder

The tree builder, on the other hand, doesn't generate the DOM elements for the rows. Instead, it gets the data directly from the RDF datasource whenever it needs it. Since trees are often expected to display thousands of rows of data, this is much more efficient. Creating an element for every cell would be too costly. However, the tradeoff is that trees may only display text, and, since no elements are created, you can't use CSS properties to style tree cells in the same way.

The tree builder is only used for trees. Other elements use only the content builder. This isn't a problem though, since other elements such as menus wouldn't be expected to display too many items. It's also possible to use the content builder for trees as well, and a treeitem element and related elements will be created for each row.
Rules

In the image of the earlier example, you may have noticed that the third button is simply a button with hyphens on it. This is a separator in the bookmark list. In the way that we have been using it, the RDF bookmarks datasource supplies the separators as if they were just regular bookmarks. What we would really like to do is add a small amount of spacing instead of a button for separator resources. That means that we want to have two different types of content be created, one type for regular bookmarks and a second type for separators.

We can do this by using the rule element. We define a rule for each variation of elements that we want to have created. In our case, we would need a rule for bookmarks and a rule for separators. Attributes placed on the rule element determine which rules apply to which RDF resource.

When scanning for which rule applies to the data, each rule element is checked in sequence for a match. That means that the order in which you define rules is important. Earlier rules will override later rules.
Rule Example

The following example demonstrates the earlier example with two rules:

Example 4 : Source

<window
  id="example-window"
  title="Bookmarks List"
  xmlns:html="http://www.w3.org/1999/xhtml"
  xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#"  
  xmlns="http://www.mozilla.org/keymaster/gatekeeper/there.is.only.xul">

 <vbox datasources="rdf:bookmarks" ref="NC:BookmarksRoot" flex="1">
   <template>

    <rule rdf:type="http://home.netscape.com/NC-rdf#BookmarkSeparator">
     <spacer uri="rdf:*" height="16"/>
    </rule>

    <rule>
      <button uri="rdf:*" label="rdf:http://home.netscape.com/NC-rdf#Name"/>
    </rule>
  
  </template>
 </vbox>

</window>

Image:templates2.jpg

By using two rules, we have allowed the contents of the template to be selectively generated. In the first rule, bookmark separators are selected, as can be seen by the rdf:type attribute. The second rule does not have any attributes so all data matches it.

All of the attributes placed on the rule tag are used as match criteria. In this case, the bookmarks datasource supplies a rdf:type property to distinguish separators. This attribute is set to a special value for separators in the RDF bookmarks datasource. This is how we can distinguish them from non-separators. You can use a similar technique for any attribute that might be on an RDF Description element.

The special URL value given in the example above for the first rule is used for separators. That means that separators will follow rule one and generate a spacer element, which will display a 16 pixel gap. Elements that are not separators will not match rule one and will fall through to rule two. Rule two does not have any attributes on it. This means that it will match all data. This is, of course, what we want to have happen to the rest of the data.

You should also have noticed that because we wanted to get an attribute from the RDF namespace (rdf:type), we needed to add the namespace declaration to the window tag. If we didn't do this, the attribute would be looked for in the XUL namespace. Because it does not exist there, the rule will not match. If you use attributes in your own custom namespace, you need to add the namespace declaration in order to match them.

You should be able to guess what would happen if the second rule was removed. The result would be a single spacer displayed but no bookmarks because they don't match any of the rules.

Put simply, a rule matches if all of the attributes placed on the rule element match the corresponding attributes on the RDF resource. In the case of an RDF file, the resources would be the Description elements.

There are some small exceptions however. You cannot match based on the attributes id, rdf:property or rdf:instanceOf. Because you can just use your own attributes with your own namespace, it probably doesn't really matter anyway.

Note that a template with no rules in it, as in the first example, is really equivalent functionally to a template with a single rule with no attributes.

Next, we'll look at using templates with trees.

« Previous
Next »
Document Tags and Contributors
Tags: 

    Tutorials XUL XUL_Tutorial 

Contributors to this page: SphinxKnight, Sheppy, teoli, trevorh, LJR, Chbok, Mgjbot, Morishoji, Pmash, Ptak82, Dria
Last updated by: SphinxKnight, Feb 9, 2018, 10:41:43 AM
Learn the best of web development

Get the latest and greatest from MDN delivered straight to your inbox.
E-mail
MDN Web Docs

    Web Technologies
    Learn Web Development
    About MDN
    Feedback

Mozilla

    About
    Contact Us
    Firefox

Other languages:

    Terms Privacy Cookies 

© 2005-2018 Mozilla and individual contributors.

Content is available under these licenses.


    Skip to main content
    Select language
    Skip to search

MDN Web Docs

    Technologies References & Guides Feedback 

    Sign in

Search

Edit

Trees and Templates

« Previous
Next »

The following describes how to use a template with a tree.
Adding Datasources to Trees

When using a tree, you will often use a template to build its content, to handle a large amount of hierarchial data. Using a template with a tree uses very much the same syntax as with other elements. You need to add a datasources and a ref attribute to the tree element, which specify the datasource and root node to display. Multiple rules can be used to indicate different content for different types of data.

The following example uses the history datasource:

<tree datasources="rdf:history" ref="NC:HistoryByDate"
          flags="dont-build-content">

As described in the previous section, the tree may use a tree builder for template generation instead of the normal content builder. This means that elements will not be created for every row in the tree, making it more efficient. The flags attribute set to the value dont-build-content, as used in the example above, indicates that the tree builder should be used. If you leave the attribute out, the content builder will be used. You can see the difference by using Mozilla's DOM Inspector on a tree with and without the flag.

If you do use a content builder instead, note that the content won't generally get built until it is needed. With hierarchical trees, the children don't get generated until the parent nodes have been opened by the user.

In the template, there will be one treecell for each column in the tree. The cells should have a label attribute to set the label for the cell. This would normally be set to an RDF property so that the label is pulled from the datasource.
Template-built Tree Example

The following example demonstrates a template-built tree, in this case for the file system.

Example 1 : Source

<tree id="my-tree" flex="1"
       datasources="rdf:files" ref="NC:FilesRoot" flags="dont-build-content">
  <treecols>
    <treecol id="Name" label="Name" primary="true" flex="1"/>
    <splitter/>
    <treecol id="Date" label="Date" flex="1"/>
  </treecols>

    <template>
      <rule>
        <treechildren>
          <treeitem uri="rdf:*">
            <treerow>
              <treecell label="rdf:http://home.netscape.com/NC-rdf#Name"/>
              <treecell label="rdf:http://home.netscape.com/WEB-rdf#LastModifiedDate"/>
            </treerow>
          </treeitem>
        </treechildren>
      </rule>
    </template>
</tree>

Here, a tree is created with two columns, for the name and date of a file. The tree should display a list of the files in the root directory. Only one rule is used, but you may add others if needed. Like with other templates, the uri attribute on an element indicates where to start generating content. The two cells grab the name and date from the datasource and place the values in the cell labels.

This example shows why the uri attribute becomes useful. Notice how it has been placed on the treeitem in the example, even though it is not a direct descendant of the rule element. We need to put this attribute on only those elements that we want repeated for each resource. Because we don't want multiple treechildren elements, we don't put it there. Instead we put the uri attributes on the treeitem elements. Effectively, the elements outside (or above) the element with the uri attribute are not duplicated whereas the element with the uri attribute and the elements inside it are duplicated for each resource.

As the tree builder is used and not the content builder, the structure of the elements in the above example must be as shown, with the treechildren element inside the rule. Although the tree builder doesn't build these elements, it does require this structure in order to determine what to generate correctly.
Image:rdfoutl1.jpg

Note in the image that additional child elements below the top-level elements have been added automatically. XUL knows how to add child elements when the templates or rules contain tree elements or menu elements. It will generate tree elements as nested as necessary based on the available RDF data.

An interesting part of RDF datasources is that the resource values are only determined when the data is needed. This means that values that are deeper in the resource hierarchy are not determined until the user navigates to that node in the tree. This becomes useful for certain datasources where the data is determined dynamically.
Sorting Columns

If you try the previous example, you might note that the list of files is not sorted. Trees which generate their data from a datasource have the optional ability to sort their data. You can sort either ascending or descending on any column. The user may change the sort column and direction by clicking the column headers. This sorting feature is not available for trees with static content, although you can write a script to sort the data.

Sorting involves three attributes, which should be placed on the columns. The first attribute, sort, should be set to an RDF property that is used as the sort key. Usually, this would be the same as that used in the label of the cell in that column. If you set this on a column, the data will be sorted in that column. The user can change the sort direction by clicking the column header. If you do not set the sort attribute on a column, the data cannot be sorted by that column.

The sortDirection attribute (note the mixed case) is used to set the direction in which the column will be sorted by default. Three values are possible:

ascending 
    the data is displayed is ascending order.
descending 
    the data is displayed is descending order.
natural 
    the data is displayed in natural order, which means the order the data is stored in the RDF datasource.

The final attribute, sortActive should be set to true for one column, the one that you would like to be sorted by default.

The following example changes the columns in the earlier example to incorporate the extra features:

<treecols>
  <treecol id="Name" label="Name" flex="1" primary="true"
            sortActive="true" sortDirection="ascending"
            sort="rdf:http://home.netscape.com/NC-rdf#Name"/>
  <splitter/>
  <treecol id="Date" label="Date" flex="1"
           sort="rdf:http://home.netscape.com/WEB-rdf#LastModifiedDate"/>
</treecols>

Persisting Column State

One additional thing you might want to do is persist which column is currently sorted, so that it is remembered between sessions. To do this, we use the persist attribute on each treecol element. There are five attributes of columns that need to be persisted, to save the column width, the column order, whether the column is visible, which column is currently sorted and the sort direction. The following example shows a sample column:

<treecol id="Date" label="Date" flex="1"
             persist="width ordinal hidden sortActive sortDirection"
             sort="rdf:http://home.netscape.com/WEB-rdf#LastModifiedDate"/>

More details about the persist attribute will be described in the later section.
Additional Rule Attributes

There are two additional attributes that can be added to the rule element that allow it to match in certain special circumstances. Both are boolean attributes.

iscontainer
    If this attribute is set to true, then the rule will match all resources that have children. For example, we could use this rule to match bookmark folders. This is convenient as the RDF datasource does not need to include any special attributes to indicate this.

isempty
    If this attribute is set to true, then the rule will match all resources that have no children.

A resource might be a container and be an empty one as well. However, this is different from a resource that is not a container. For example, a bookmark folder is a container but it might or might not have children. However a single bookmark or separator is not a container.

You can combine these two elements with other attribute matches for more specific rules.

Next, we'll look at some of the datasources provided by Mozilla.

« Previous
Next »
Document Tags and Contributors
Tags: 

    Tutorials XUL XUL_Tutorial 

Contributors to this page: Sheppy, teoli, fscholz, Dongdong, Chbok, Mgjbot, Jjinux, Morishoji, Enn, Pmash, Ptak82, Dria
Last updated by: Sheppy, Apr 14, 2014, 10:34:57 AM
Learn the best of web development

Get the latest and greatest from MDN delivered straight to your inbox.
E-mail
MDN Web Docs

    Web Technologies
    Learn Web Development
    About MDN
    Feedback

Mozilla

    About
    Contact Us
    Firefox

Other languages:

    Terms Privacy Cookies 

© 2005-2018 Mozilla and individual contributors.

Content is available under these licenses.


    Skip to main content
    Select language
    Skip to search

MDN Web Docs

    Technologies References & Guides Feedback 

    Sign in

Search

Edit

RDF Datasources

 

« Previous
Next »

Here, we'll look at additional datasources and how to use your own RDF files as datasources.
Other Mozilla Datasources

Mozilla provides a number of other built-in datasources. Some of them are listed here with a few examples. They work very similarly to the bookmarks, although the fields will be different in each case.
The History List

The history datasource provides access to the user's history list which is the list of URLs the user has visited recently. The resource can be referred to using rdf:history as the datasource. The table below shows the resources (or fields) that you can retrieve from the history datasource. Put the URL values below where you want the value of the resource to be used.
Date 	http://home.netscape.com/NC-rdf#Date 	Date of last visit
Name 	http://home.netscape.com/NC-rdf#Name 	Title of the page
Page 	http://home.netscape.com/NC-rdf#Page 	Page name
Referrer 	http://home.netscape.com/NC-rdf#Referrer 	Referrer of the page
URL 	http://home.netscape.com/NC-rdf#URL 	URL of the page
Visit Count 	http://home.netscape.com/NC-rdf#VisitCount 	Number of page visits

A typical history list will display a tree with a selection of these fields. To use them, just put the URL values above in the label attributes of the buttons or treecells. You can use NC:HistoryRoot as the value of the ref attribute. You can also use the value NC:HistoryByDate to get the history sorted into days.
Using The History List Example

Let's see an example of displaying the history list. We'll display the history in a tree with three columns, the Name, the URL and the Date.

Example 1 : Source

<tree flex="1" datasources="rdf:history" ref="NC:HistoryRoot">

  <treecols>
    <treecol id="name" label="Name" flex="1"/>
    <treecol id="url" label="URL" flex="1"/>
    <treecol id="date" label="Date" flex="1"/>
  </treecols>

  <template>

    <rule>
      <treechildren>
       <treeitem uri="rdf:*">
         <treerow>
           <treecell label="rdf:http://home.netscape.com/NC-rdf#Name"/>
           <treecell label="rdf:http://home.netscape.com/NC-rdf#URL"/>
           <treecell label="rdf:http://home.netscape.com/NC-rdf#Date"/>
         </treerow>
       </treeitem>
      </treechildren>
    </rule>
      
  </template>
</tree>

Other Datasources

The tables below list some of the other datasources available with Mozilla. You can use any of the resources that you want.

Bookmarks (rdf:bookmarks)
    The bookmarks are generated from the user's bookmark list.

Resources
Added Date 	http://home.netscape.com/NC-rdf#BookmarkAddDate 	Date the bookmark was added
Description 	http://home.netscape.com/NC-rdf#Description 	Bookmark description
Last Modified 	http://home.netscape.com/WEB-rdf#LastModifiedDate 	Date of last modification
Last Visited 	http://home.netscape.com/WEB-rdf#LastVisitDate 	Date of last visit
Name 	http://home.netscape.com/NC-rdf#Name 	Bookmark name
Shortcut URL 	http://home.netscape.com/NC-rdf#ShortcutURL 	Custom keywords field
URL 	http://home.netscape.com/NC-rdf#URL 	The URL to link to
Possible Bookmarks Roots
NC:BookmarksRoot 	The top level of the bookmarks hierarchy
NC:IEFavoritesRoot 	The bookmark folder that corresponds to the user's IE favorites.
NC:PersonalToolbarFolder 	The bookmark folder that corresponds to the personal toolbar folder.

Files (rdf:files)
    A view of the user's files.

Resources
Name 	http://home.netscape.com/NC-rdf#Name 	Name of the file
URL 	http://home.netscape.com/NC-rdf#URL 	URL of the file
Content-Length 	http://home.netscape.com/NC-rdf#Content-Length 	The length of the file.
LastModifiedDate 	http://home.netscape.com/NC-rdf#LastModifiedDate 	The date that URL was last modified.
extension 	http://home.netscape.com/NC-rdf#extension 	The extension of the file, including the period. This property is only available on platforms that use file extensions.
Possible Files Roots
NC:FilesRoot 	Top level of the filesystem (usually the list of drives)
A file URL 	By using a file URL for the ref attribute, you can select a specific directory to be returned. For example, you might use file:///windows or files:///usr/local.

The files datasource is an example of a datasource that determines its resources only when necessary. We don't want every file in the filesystem to be determined before the data is displayed. Instead, only the files and directories that the tree element (or other elements) will need to display at a given time will be determined.
Composite Datasources

You can specify multiple datasources in the datasources attribute by separating them with whitespace as in the example below. This has the effect of reading the data from all the datasources mentioned.

<tree datasources="rdf:bookmarks rdf:history animals.rdf" ref="NC:BookmarksRoot">

This example reads the resources from the bookmarks, history and the animals.rdf file. They are combined into a single composite datasource and can be used as if they were one.

The special datasource rdf:null corresponds to nothing. You can use this datasource if you want to dynamically set the datasource using a script, but don't want one initially or don't know its exact URL.
Custom RDF Datasources

You can use any of the above internal datasources if you wish. There are several others for mail, address books and searching and so on. However, you might want to use your own RDF datasource stored in an RDF file. The file can be either a local file or a remote file. Just put the URL of the RDF file in the datasources attribute.

Using RDF files provides just as much functionality as any of the internal datasources. You can use rules to match specific types of content. The attributes on the rule element will match if they match the attributes on an RDF Description element. You can also create RDF files that are hierarchical.
Using RDF file Example

The following is an example of how an RDF file can be used as a datasource. The RDF file is fairly large and can be viewed separately: Source RDF

Example 2 : Source View

<tree flex="1" width="200" height="200"
      datasources="animals.rdf" ref="http://www.some-fictitious-zoo.com/all-animals">

  <treecols>
    <treecol id="name" label="Name" primary="true" flex="1"/>
    <treecol id="species" label="Species" flex="1"/>
  </treecols>

  <template>
    <rule>
      <treechildren>
       <treeitem uri="rdf:*">
         <treerow>
           <treecell label="rdf:http://www.some-fictitious-zoo.com/rdf#name"/>
           <treecell label="rdf:http://www.some-fictitious-zoo.com/rdf#species"/>
         </treerow>
       </treeitem>
      </treechildren>
    </rule>

  </template>
</tree>

Image:datasrc1.jpg

Here, the data has been generated from the file. The ref attribute has been set to the root element in the RDF file, which is the top-level Seq. This will give us a complete list of animals. If we wanted to, we could set the ref attribute to any of the other about attribute values to limit the set of data that is returned. For example, to display only the reptiles, use a value of http://www.some-fictitious-zoo.com/reptiles.
Setting the ref Attribute Example

The example below shows how to display a particular piece of an RDF datasource by setting the ref attribute.

Example 3 : Source View

<window
  id="example-window"
  title="History List"
  xmlns:ANIMALS="http://www.some-fictitious-zoo.com/rdf#"
  xmlns="http://www.mozilla.org/keymaster/gatekeeper/there.is.only.xul">

<button label="Click here to see the mammals the zoo has" type="menu"
        datasources="animals.rdf" ref="http://www.some-fictitious-zoo.com/mammals">
  <template>
    <rule ANIMALS:specimens="0"></rule>
    <rule>
      <menupopup>
        <menuitem uri="rdf:*" label="rdf:http://www.some-fictitious-zoo.com/rdf#name"/>
      </menupopup>
    </rule>
  </template>
</button>

</window>

In this case only the mammals are desired, so we select the URI of the mammals list. You will notice that the value of the ref attribute in the example is http://www.some-fictitious-zoo.com/mammals which corresponds to one of the Seq elements in the RDF file. This causes only the descendants of this list to be returned.

Two rules have been used here. The first rule catches all the resources that have their ANIMALS:specimens attribute set to 0. You can see this attribute in the RDF file on each of the Description elements. Some of them have a value of 0. So in these cases, rule one will match. Because rule one has no content, nothing will be displayed for these ones. This is an effective way to hide data that we don't want to display.

The second rule applies to all other resources and creates a row in a popup menu. The end effect is that we get a popup menu containing all the mammals which have a specimen that is not 0.

Next, we'll look at the full rule syntax.

« Previous
Next »
Document Tags and Contributors
Tags: 

    Tutorials XUL XUL_Tutorial 

Contributors to this page: Sheppy, teoli, fscholz, Epskampie, Chbok, Jjinux, Mgjbot, Morishoji, Pmash, Ptak82, Dria, Enn
Last updated by: Sheppy, Apr 14, 2014, 10:35:00 AM
Learn the best of web development

Get the latest and greatest from MDN delivered straight to your inbox.
E-mail
MDN Web Docs

    Web Technologies
    Learn Web Development
    About MDN
    Feedback

Mozilla

    About
    Contact Us
    Firefox

Other languages:

    Terms Privacy Cookies 

© 2005-2018 Mozilla and individual contributors.

Content is available under these licenses.


    Skip to main content
    Select language
    Skip to search

MDN Web Docs

    Technologies References & Guides Feedback 

    Sign in

Search

Edit

Advanced Rules
Jump to:

    The Full Rule SyntaxRule ConditionsGenerating ContentAdding Additional Bindings 

« Previous
Next »

This section describes the more advanced rule syntax.
The Full Rule Syntax

The rule syntax described so far is useful for some datasources but sometimes you will need to display data in more complicated ways. The simple rule syntax is really just a shortcut for the full rule syntax which is described below. Like the simple rules, full rules are placed within the rule tag.

Full rules contain three child tags, a conditions tag, a bindings tag and an action tag, although the bindings tag is not always needed.

The conditions element is used to specify the criteria for matching a given resource. You can specify a number of conditions, all of which must match. In the simple rule syntax, the conditions are placed directly on the rule element itself.

If the conditions match for a resource, the content placed within the actions tag is generated. In the simple syntax, the content is placed directly inside the rule.
Rule Conditions

When a tree, menu or other element with a datasource generates content, the template builder first finds the resource referred to by the ref attribute. It then iterates over all that resource's child resources. It applies each resource to the conditions. If the conditions match for that resource, the content in the actions element is generated for that resource. If the conditions do not match, no content is generated.
content element

The conditions element can contain three elements. The first is the content element, which should always exist once and only once. It serves as a placeholder as the template builder iterates through the resources. It specifies the name of a variable in which is placed a reference to the root resource while the conditions are analyzed for a match. The root resource is the one specified by the ref attribute on the element containing the template.

The syntax of the content element is as follows:

<content uri="?var"/>

The question mark indicates that the text following it is a variable. You can then use the variable 'var' within the remainder of the conditions. Of course, you can name the variable whatever you want.
member element

The next element is the member element, which is used to iterate through a set of child resources. In RDF terms, that means a container such a Seq, Bag or Alt. Let's say you have a list of cities described in the following RDF/XML fragment:

<RDF:Seq about="http://www.xulplanet.com/rdf/weather/cities">
  <RDF:li resource="http://www.xulplanet.com/rdf/weather/city/Paris"/>
  <RDF:li resource="http://www.xulplanet.com/rdf/weather/city/Manchester"/>
  <RDF:li resource="http://www.xulplanet.com/rdf/weather/city/Melbourne"/>
  <RDF:li resource="http://www.xulplanet.com/rdf/weather/city/Kiev"/>
</RDF:Seq>

<RDF:Description about="http://www.xulplanet.com/rdf/weather/city/Paris">
  <cityset:name>Paris</cityset:name>
</RDF:Description>
.
.
.

You want to display a row in a tree for each city. To do this, use the member element as in the following:

<tree id="citiesTree" datasources="weather.rdf"
      ref="http://www.xulplanet.com/rdf/weather/cities"> 
  <template>
    <rule>
      <conditions>
        <content uri="?list"/>
        <member container="?list" child="?city"/>
      </conditions>
    <rule>
  <template>
</tree>

The template builder starts by grabbing the value of the ref attribute, which in this case is http://www.xulplanet.com/rdf/weather/cities. This resource will be placed in the 'list' variable as specified by the content tag. We can then get related resources to the root resource by using the 'list' variable.

The template builder then sees the member element. It causes the builder to iterate over the children of an element. The parent is specified by the container attribute and the children are specified by the child attribute. In the example above, the value of the container attribute is the variable 'list'. Thus the parent will be the value of the list variable, which has been set to the root resource 'http://www.xulplanet.com/rdf/weather/cities'. The effect will be to iterate through the list of children of 'http://www.xulplanet.com/rdf/weather/cities'.

If you look at the RDF above, the 'http://www.xulplanet.com/rdf/weather/cities' resource has four children, one for each different city. The template builder iterates through each one, matching the child against the value of the child attribute. In this case, it is just set to the variable 'city'. So the builder will set the 'city' variable to the each child resource in turn.

Because there are no more conditions, the condition matches for each of those four resources and the builder will generate content for each of the four. Of course, the example above doesn't have any content. We'll add that later.
triple element

The next element is the triple element. It is used to check for the existence of a given triple (or assertion) in the RDF datasource. A triple is like a property of a resource. For example, a triple exists between a bookmark and its URL. This might be expressed as follows:

A Bookmark to mozilla.org  ->  URL  ->  www.mozilla.org

This means that there is a triple between the bookmark 'A Bookmark to mozilla.org' and 'www.mozilla.org' by the URL property. The first part of this expression is called the subject, the second part is called the predicate and the last part is called the object. As a triple element, it would be expressed as follows:

<triple subject="A Bookmark to mozilla.org"
           predicate="URL"
           object="www.mozilla.org"/>

This has been simplified a bit from the real thing. The predicate would normally include the namespace, and the subject would be the bookmark's resource id, not the bookmark's title as used here. In fact, the bookmark's title would be another triple in the datasource using the Name predicate.

You can replace the subject and object on the triple element with variable references, in which case values will be substituted for the variables. If no value is defined for a variable yet, the template builder will look up the value in the datasource and assign it to the variable.

Let's say we wanted to add a weather prediction to the city datasource. The following conditions might be used:

<conditions>
  <content uri="?list"/>
  <member container="?list" child="?city"/>
  <triple subject="?city"
             predicate="http://www.xulplanet.com/rdf/weather#prediction"
             object="?pred"/>
</conditions>

The template builder will iterate over each city as before. When it gets to the triple, it will look for an assertion in the RDF datasource for a city's weather prediction. The variable 'pred' will be assigned the prediction. The builder will repeat this for each of the four cities. A match occurs and the builder will generate content for each city that has a prediction. If the city has no prediction resource, the condition does not match and no content will be generated for that city. Note that you do not need to put 'rdf:' at the beginning of the predicate, as that part is assumed.

We could also replace the object with an in-line value. For example:

<conditions>
  <content uri="?city"/>
  <triple subject="?city"
             predicate="http://www.xulplanet.com/rdf/weather#prediction"
             object="Cloudy"/>
</conditions>

This example is similar but we specify that we want to match on 'Cloudy'. The result is that the conditions will only match for cities where the prediction is 'Cloudy'.

We can add more triples to require more matches. For example, in the example above, we might want to check for the temperature and the wind speed. To do this just add another triple which checks for the additional resource. The condition will match if all of the triples provide values.

The example below will check for an extra triple for the name of the city. It will be assigned to the 'name' variable. The condition will only match if the city has both a name and a prediction.

<conditions>
  <content uri="?list"/>
  <member container="?list" child="?city"/>
  <triple subject="?city"
             predicate="http://www.xulplanet.com/rdf/weather#name"
             object="?name"/>
  <triple subject="?city"
             predicate="http://www.xulplanet.com/rdf/weather#prediction"
             object="?pred"/>
</conditions>

Generating Content

The content to generate for a rule is specified inside the action element. This should be the content for the rows of the tree, menu items, or whatever content you want to generate. Within the content, you can refer to variables that were defined in the conditions. Thus, in the weather example above, you could use the variables 'name' or 'pred' to display the city or prediction. You can use the 'list' or 'city' variables also, but they hold resources, not text, so they won't likely have meaningful values to users.

In the simple rule syntax, you use the syntax uri='rdf:*' to indicate where content should be generated. In the full syntax, you set the value of the uri attribute to a variable which you used in the conditions. Usually, this will be the variable assigned in the child attribute of the member element.
Complete Tree Example

The following example shows a complete tree with conditions and an action. You can view the RDF file separately:
wheater.rdf

Example 1 : Source

<tree id="weatherTree" flex="1" datasources="weather.rdf"
      ref="http://www.xulplanet.com/rdf/weather/cities">
  <treecols>
    <treecol id="city" label="City" primary="true" flex="1"/>
    <treecol id="pred" label="Prediction" flex="1"/>
  </treecols>

  <template>
    <rule>
      <conditions>
        <content uri="?list"/>
        <member container="?list" child="?city"/>
        <triple subject="?city"
                predicate="http://www.xulplanet.com/rdf/weather#name"
                object="?name"/>
        <triple subject="?city"
                predicate="http://www.xulplanet.com/rdf/weather#prediction"
                object="?pred"/>
      </conditions>
      <action>
        <treechildren>
          <treeitem uri="?city">
            <treerow>
              <treecell label="?name"/>
              <treecell label="?pred"/>
            </treerow>
          </treeitem>
        </treechildren>
      </action>
    </rule>
  </template>
</tree>

Two columns appear in this tree, one which displays the value of the name for each row and the other which displays the value of the prediction.

If using the dont-build-content flag on a tree, replace the content element with a treeitem element.
Adding Additional Bindings

The final element you can add inside a rule is the bindings element. Inside it, you place one or more binding elements. A binding in a rule has the same syntax as a triple and performs almost the same function. For example, in the weather example above we could add the following binding:

<bindings>
  <binding subject="?city"
             predicate="http://www.xulplanet.com/rdf/weather#temperature"
             object="?temp"/>
</bindings>

This binding will grab the temperature resource of each city and assign it to the 'temp' variable. This is similar to what a triple does. The difference is that a binding is not examined when attempting to check the conditions. This means that the city must have a name and prediction to be displayed, yet it does not matter if it has a temperature. However, if it does, it will be placed in the 'temp' variable so it can be used in the action. If a city does not have a temperature, the 'temp' variable will be set to an empty string.

Next, we'll find out how to save the state of XUL elements.

« Previous
Next »
Document Tags and Contributors
Tags: 

    Tutorials XUL XUL_Tutorial 

Contributors to this page: Sheppy, ethertank, teoli, trevorh, fscholz, Goolic, Chbok, Mgjbot, Morishoji, Pmash, Ptak82, Dria
Last updated by: Sheppy, Apr 14, 2014, 10:34:48 AM
Learn the best of web development

Get the latest and greatest from MDN delivered straight to your inbox.
E-mail
MDN Web Docs

    Web Technologies
    Learn Web Development
    About MDN
    Feedback

Mozilla

    About
    Contact Us
    Firefox

Other languages:

    Terms Privacy Cookies 

© 2005-2018 Mozilla and individual contributors.

Content is available under these licenses.


    Skip to main content
    Select language
    Skip to search

MDN Web Docs

    Technologies References & Guides Feedback 

    Sign in

Search

Edit

Persistent Data

 

« Previous
Next »

This section describes how to save the state of a XUL window.
Remembering State

When building a large application, you will typically want to be able to save some of the state of a window across sessions. For example, the window should remember which toolbars are collapsed even after the user exits.

One possibility would be to write a script to collect information about what you would like to save and then save it to a file. However, that would be a pain to do for every application. Conveniently, XUL provides such a mechanism to save the state of a window.

The information is collected and stored in a RDF file (localstore.rdf) in the same directory as other user preferences. It holds state information about each window. This method has the advantage that it works with Mozilla user profiles, so that each user can have different settings.

XUL allows you to save the state of any element. You will typically want to save toolbar states, window positions and whether certain panels are displayed or not, but you can save almost anything.
persist attribute

To allow the saving of state, you simply add a persist attribute to the element which holds a value you want to save. The persist attribute should be set to a space-separated list of attributes of the element that you want to save. The element must also have an id attribute in order to identify it.

For example, to save the size of a window, you would do the following:

<window
  id="someWindow"
  width="200"
  height="300"
  persist="width height"
  .
  .
  .

The two attributes of the window element, the width and the height will be saved. You could add additional attributes by adding a space and another attribute name to the persist attribute. You can add the persist attribute to any element and store any attribute. You might use unusual values if you adjust attributes using a script.
Our Find Files Example

Let's add the persist attribute to some of the elements in the find files dialog. To save the position of the window. To do this, we need to modify the window.

<window
  id="findfile-window"
  title="Find Files"
  persist="screenX screenY width height"
  orient="horizontal"
  xmlns="http://www.mozilla.org/keymaster/gatekeeper/there.is.only.xul">

This will cause the x and y position of the window and the width and height of the window to be saved. We could extend it further to save the collapsed state of the splitter. It doesn't really make sense to save the current tab state.

Find files example so far : Source View

Next, we'll look at using style sheets with XUL files.

« Previous
Next »
Document Tags and Contributors
Tags: 

    Tutorials XUL XUL_Tutorial 

Contributors to this page: Sheppy, teoli, fscholz, Chbok, Mgjbot, Morishoji, Jjinux, Pmash, Ptak82, Dria
Last updated by: Sheppy, Apr 14, 2014, 10:35:02 AM
Learn the best of web development

Get the latest and greatest from MDN delivered straight to your inbox.
E-mail
MDN Web Docs

    Web Technologies
    Learn Web Development
    About MDN
    Feedback

Mozilla

    About
    Contact Us
    Firefox

Other languages:

    Terms Privacy Cookies 

© 2005-2018 Mozilla and individual contributors.

Content is available under these licenses.


    Skip to main content
    Select language
    Skip to search

MDN Web Docs

    Technologies References & Guides Feedback 

    Sign in

Search

Edit

Adding Style Sheets

« Previous
Next »

We have hardly modified the look of the elements we have created so far. XUL uses CSS (Cascading Style Sheets) to customize elements.
Style Sheets

A style sheet is a file which contains style information for elements. It was originally designed for HTML elements but can be applied to XUL elements also, or to any XML for that matter. The style sheet contains information such as the fonts, colors, borders, and size of elements.

Mozilla applies a default style sheet to each XUL window. In many cases, it will suffice to leave the defaults as is. Other times, however, you will want to provide a custom style sheet. In general, you will associate a single style sheet with each XUL file.

You can place a style sheet anywhere you wish. If your XUL file is stored remotely and accessed via an HTTP URL, you can store the style sheet remotely as well. If you are creating a XUL package to be installed as part of the chrome system, you have two choices. First, you could store the style sheet in the same directory as the XUL file. This method has the disadvantage because it means your application will not be themeable. The second method involves placing your files as part of a theme.

Let's assume that we are building the find files dialog for themeability, because the find files dialog can be referred to with the URL chrome://findfile/content/findfile.xul so the style sheet file will be stored in chrome://findfile/skin/findfile.css.

All the XUL examples so far have actually been using a style sheet already. The second line has always been:

<?xml-stylesheet href="chrome://global/skin/" type="text/css"?>

This line indicates that we want to use the style provided by chrome://global/skin/. In Mozilla, this will be translated as the file global.css, which contains default style information for XUL elements. You could leave the line out and the elements will still work, however they will look fairly plain. The style sheet applies various fonts, colors and borders to make the elements look more suitable.
Changing the Styles

However, there will be times when the default look of elements will not give the look that is desired. For this, we will need to add a style sheet of our own. So far, we have been applying styles using the style attribute on elements. Although this works, it is not really the best thing to do. It is much better to create a separate style sheet. The reason is so that different looks, or skins, can be applied easily.

There may be certain cases where the style attribute is acceptable. An example would be when a script changes the style, or where a difference in layout might change the meaning of the element. However you should avoid this as much as possible.

For installed files, you'll have to create or modify a manifest file and install the skin.
Our Find Files Dialog Example

Let's modify the find files dialog so that its style comes from a separate style file. First, the modifed lines of findfile.xul:

<?xml-stylesheet href="chrome://global/skin/" type="text/css"?>
<?xml-stylesheet href="findfile.css" type="text/css"?>
  ...
<spacer class="titlespace"/>
  <groupbox orient="horizontal">
    <caption label="Search Criteria"/>

      <menulist id="searchtype">
        <menupopup>
          <menuitem label="Name"/>
          <menuitem label="Size"/>
          <menuitem label="Date Modified"/>
        </menupopup>
      </menulist>
      <spacer class="springspace"/>
      <menulist id="searchmode">
        <menupopup>
          <menuitem label="Is"/>
          <menuitem label="Is Not"/>
        </menupopup>
      </menulist>

      <spacer class="springspace"/>
      <menulist id="find-text" flex="1"
          editable="true"
          datasources="file:///mozilla/recents.rdf"
          ref="http://www.xulplanet.com/rdf/recent/all"/>
  ...
<spacer class="titlespace"/>
<hbox>

  <progressmeter id="progmeter" value="50%" style="display:none;"/>

The new xml-stylesheet line is used to import the style sheet. It will contain the styles instead of having them directly in the XUL file. You can include any number of style sheets in a similar way. Here the style sheet is placed in the same directory as findfile.xul.

Some of the styles in the code above have been removed. One that wasn't was the display property on the progressmeter. This will be changed by a script so it was left in, as it doesn't really make sense to have the progress bar visible initially. You could still put this in a separate style sheet if you really wanted to. A class was added to the spacers so that they can be referred to.

A style sheet also needs to be created. Create a file findfile.css in the same directory as the XUL file. (It would normally be put into a separate skin). In this file, we'll add the style declarations, as shown below:

#find-text {
  min-width: 15em;
}

#progmeter {
  margin: 4px;
}

.springspace {
  width: 10px;
}

.titlespace {
  height: 10px;
}

Notice how these styles are equivalent to the styles we had before. However, it is much easier for someone to change the look of the find files dialog now because they could add or modify the style declarations by either modifying the file or by changing the skin. If the user changes the interface skin, the files in a directory other than default will be applied.
Importing Style Sheets

We've already seen how to import style sheets for use. An example is shown below:

<?xml-stylesheet href="chrome://bookmarks/skin/" type="text/css"?>

This might be the first lines of a bookmarks window. It imports the bookmarks style sheet, which is bookmarks.css. Mozilla's skin system is smart enough to figure out which style sheet to use, because the specific filename was not indicated here. We have done a similar thing with the global style sheet file (chrome://global/skin).

A style sheet may import styles from another stylesheet using the import directive. Normally, you will only import one style sheet from each XUL file. The global style sheet can be imported from within the style sheet associated with the XUL file. This can be done with the code below, allowing you to remove the import from the XUL file:

Style import from XUL:
<?xml-stylesheet href="chrome://global/skin/"  type="text/css"?>

Style import from CSS:
@import url(chrome://global/skin/);

The second syntax is preferred because it reduces the number of dependencies within the XUL file itself.

Remove the global style sheet import from findfile.xul and add the import to findfile.css.

All elements can be styled using CSS. You can use selectors to select the element that you wish to have styled. (The selector is the part before the curly brace in a style rule). The following list summarizes some of the selectors available:

button 
    Matches all button tags.
#special-button 
    Matches the element with an id of special-button.
.bigbuttons 
    Matches all elements with a class of bigbuttons.
button.bigbuttons 
    Matches all button elements with a class of bigbuttons.
toolbar > button 
    Matches all buttons that are directly inside toolbar elements.
toolbar > button.bigbuttons 
    Matches all button elements with a class of bigbuttons that are directly inside toolbar elements.
button.bigbuttons:hover 
    Matches all button elements with a class of bigbuttons but only while the mouse is over them.
button#special-button:active 
    Matches all button elements with an id of special-button but only while they are active (being clicked on).
box[orient="horizontal"]
    Matches all box elements that have an orient attribute that is set to horizontal.

You can combine these rules in any way that you wish. It is always a good idea to be as precise as possible when specifying what gets styled. It is more efficient and it also reduces the likelihood that you'll style the wrong thing.

Find files example so far : Source View

In the next section, we will look at how to apply styles to trees.

« Previous
Next »
Document Tags and Contributors
Tags: 

    Tutorials XUL XUL_Tutorial 

Contributors to this page: Sheppy, teoli, trevorh, fscholz, Chbok, Mgjbot, Morishoji, Ptak82, Pmash, SylvainPasche, Dria
Last updated by: Sheppy, Apr 14, 2014, 10:35:02 AM
Learn the best of web development

Get the latest and greatest from MDN delivered straight to your inbox.
E-mail
MDN Web Docs

    Web Technologies
    Learn Web Development
    About MDN
    Feedback

Mozilla

    About
    Contact Us
    Firefox

Other languages:

    Terms Privacy Cookies 

© 2005-2018 Mozilla and individual contributors.

Content is available under these licenses.


    Skip to main content
    Select language
    Skip to search

MDN Web Docs

    Technologies References & Guides Feedback 

    Sign in

Search

Edit

Styling a Tree

« Previous
Next »

The following describes how to style a tree.
Styling the Tree

You can style the tree border and the column headers in the same way as other elements. Style added to the tree element will apply to the entire tree. Adding a style to the treecol element does not cause the style to be applied to the column but only to the header.

The body of the tree must be styled in a somewhat different way than other elements. This is because the tree body is stored in a different way to other elements. The outer treechildren is the only real element in the tree body. The inner elements are just placeholders.
Setting Properties

Instead, you must use the properties attribute on the rows or cells to set one or more named properties. This can be used with trees with static content, RDF built content or with those with a custom view. Let's say we want a particular row to have a blue background color. This would be used to implement Mozilla Mail's labels feature. We'll use a property called 'makeItBlue'. You can use whatever name you want. You can set multiple properties by separating them with spaces.

Set the property on a row or cell, as in the following example:

<treerow properties="makeItBlue">

CSS Selectors for the Tree

The style sheet can take this property and use it to change the appearance of the row for unread messages or labels. You can think of the properties as functioning much like style classes, although they require a somewhat more complex syntax to use in a style sheet. This is because you can specify the style for a number of parts of the cell individually. You can style not only the cell and its text, but the twisty and indentation. The following is the syntax that needs to be used:

treechildren::-moz-tree-row(makeItBlue)
{
  background-color: blue;
}

This style which has a complex selector is used to style the background color of rows that have the 'makeItBlue' property. This special syntax is needed because the cells themselves are not separate elements. All of the content inside the tree's body is rendered by the treechildren element. However, CSS has a concept to access parts of elements considering them to be pseudo-elements. This selector matches some tree rows inside the treechildren element as pseudo-elements. The style sets style rules for particular parts of what it displays. This style rule means, inside a treechildren element, set the background color to blue for all tree rows that have the 'makeItBlue' property.

The text '::-moz-tree-row' specifies what content area is desired, which in this case is a row. You can also use the following values:

    ::-moz-tree-cell: a cell. Use this to set borders and background colors.
    ::-moz-tree-cell-text: the text in a cell. Use this to set the font and text color.
    ::-moz-tree-twisty: the appearance of the twisty used to expand and collapse child rows.
    ::-moz-tree-image: the image for a cell. You can set the image with the list-style-image property.
    ::-moz-tree-row: a row. Use this to set the background color of a row.
    ::-moz-tree-indentation: the indentation to the left of rows that are children of other rows.
    ::-moz-tree-column: a column.
    ::-moz-tree-line: the lines that are drawn to connect child rows to parent rows.
    ::-moz-tree-separator: a separator in a tree.
    ::-moz-tree-progressmeter: content for progressmeter cells. You can create a progressmeter column by setting the type attribute on the column to progressmeter.
    ::-moz-tree-drop-feedback: the drag and drop feedback.
    ::-moz-tree-checkbox: the image to use for checkbox columns.

You can check for multiple properties by separating them with commas. The example below sets the background color to grey for rows that have the 'readonly' and 'unread' properties. For properties that are 'readonly', it adds a red border around the row. Note that the first rule will apply to any row that is 'readonly' regardless of whether other properties such as 'unread' are set.

treechildren::-moz-tree-row(readonly)
{
  border: 1px solid red;
}
treechildren::-moz-tree-row(readonly, unread)
{
  background-color: rgb(80%, 80%, 80%);
}

Default Properties

The properties list for tree elements contain a small number of default properties, which you can also use in a style sheet. You can use these extra properties to set the appearance of containers or selected rows. The following properties are automatically set as needed:

checked
    this property is set cells whose column is type="checkbox"
focus
    this property is set if the tree currently has the focus.
selected
    this property is set for rows or cells that are currently selected.
current
    this property is set if the keyboard cursor is at the row. Only one row will have this property set at a time.
hover
    this property is set if the mouse cursor is on the row. Only one row will have this property set at a time.
container
    this property is set for rows or cells that have child rows.
leaf
    this property is set for rows or cells that do not have child rows.
open
    this property is set for rows or cells which are expanded.
closed
    this property is set for rows or cells which are collapsed.
primary
    this property is set for cells in the primary column.
sorted
    this property is set for cells in the current sorted column.
even
    this property is set for even numbered rows.
odd
    this property is set for odd numbered rows. This property, along with the even property allow you to set, for example, alternating colors for each row.
dragSession
    this property is set if something is currently being dragged.
dropOn
    if a drag is occuring over the tree, this property is set for the row currently being dragged over, as long as the mouse pointer is hovering over the row.
dropBefore
    this property is set if the mouse pointer is hovering before the row currently being dragged over.
dropAfter
    this property is set if the mouse pointer is hovering after the row currently being dragged over.
progressNormal
    this property is set for progress meter cells.
progressUndetermined
    this property is set for undeterminate progress meter cells.
progressNone
    this property is set for non-progress meter cells.

The properties are set for rows or cells in rows with the necessary state. For columns and cells, one additional property, the id of the column or column the cell is in will be set.
Setting Properties for the RDF-built Trees

For RDF-built trees, you can use the same syntax. However, you will often set the properties based on values in the datasource.
Setting Properties for the Custom View

For trees with a custom view script, you can set properties by supplying the functions getRowProperties(), getColumnProperties() and getCellProperties() in the view. These return information about an individual row, column and cell. Arguments to these functions indicate which row and/or column.

Prior to Gecko 22 the last argument to each of these functions is a properties list which the view is expected to fill with a list of properties. The function getColumnProperties() also supplies the corresponding treecol element for the column.

getRowProperties : function(row,prop){}
getColumnProperties : function(column,columnElement,prop){}
getCellProperties : function(row,column,prop){}

From Gecko 22 you can return a string of space-separated property names from these functions.

getRowProperties : function(row){ return ''}
getColumnProperties : function(column,columnElement){ return ''}
getCellProperties : function(row,column){ return ''}

Let's look at an example of changing a specific cell. Let's make every fourth row have blue text, using the example from a previous section. We'll need to add code to the getCellProperties() function, to add a property 'makeItBlue' for cells in every fourth row. (We don't use getRowProperties() as the text color will not be inherited into each cell.)

Prior to Gecko 22 the properties object that is passed as the last argument to the getCellProperties() is an XPCOM object that implements nsISupportsArray. It is really just an XPCOM version of an array. It contains a function AppendElement() which can be used to add an element to the array. We can use the interface nsIAtomService to construct string atoms for the properties.

getCellProperties: function(row,col,props){
  if ((row %4) == 0){
    var aserv=Components.classes["@mozilla.org/atom-service;1"].
              getService(Components.interfaces.nsIAtomService);
    props.AppendElement(aserv.getAtom("makeItBlue"));
  }
}

The properties list requires an array of atom objects, which can be thought of as constant strings. We create them using the XPCOM interface nsIAtomService and add them to the array using the AppendElement() function. Here, we create an atom 'makeItBlue'. You can call AppendElement() again to add additional properties.

From Gecko 22 your function should return a string containing the property.

getCellProperties: function(row,col){
  if ((row %4) == 0){
    return "makeItBlue";
  }
}

To support Gecko versions before and after this change use.

getCellProperties: function(row,col,props){
  if ((row %4) == 0){
    if (props) {
      var aserv=Components.classes["@mozilla.org/atom-service;1"].
                getService(Components.interfaces.nsIAtomService);
      props.AppendElement(aserv.getAtom("makeItBlue"));
    } else {
      return "makeItBlue";
    }
  }
}

This function would be defined as part of a view object. It first checks to see which row is being requested and sets a property for cells in every fourth row.
Example style sheet

treechildren::-moz-tree-row(selected)            { background-color: #FFFFAA; }
treechildren::-moz-tree-row(odd)                 { background-color: #EEEEEE; }
treechildren::-moz-tree-row(odd, selected)       { background-color: #FFFFAA; }
treechildren::-moz-tree-cell-text(selected)      { color: #000000; }
treechildren::-moz-tree-cell-text(odd, selected) { color: #000000; }

Note that you can also style e.g. all rows, without the need of using properties; see Building Trees for an example.

Next, we'll look at how to modify the default skin.

« Previous
Next »
Document Tags and Contributors
Tags: 

    Tutorials XUL XUL_Tutorial 

Contributors to this page: mfluehr, Sheppy, trevorh, teoli, custom.firefox.lady, ericjung, Np, Chbok, T.BugReporter, Mgjbot, Morishoji, Jjinux, Pmash, Cverdon, Ptak82, Dria
Last updated by: mfluehr, Sep 19, 2017, 8:28:16 AM
Learn the best of web development

Get the latest and greatest from MDN delivered straight to your inbox.
E-mail
MDN Web Docs

    Web Technologies
    Learn Web Development
    About MDN
    Feedback

Mozilla

    About
    Contact Us
    Firefox

Other languages:

    Terms Privacy Cookies 

© 2005-2018 Mozilla and individual contributors.

Content is available under these licenses.


    Skip to main content
    Select language
    Skip to search

MDN Web Docs

    Technologies References & Guides Feedback 

    Sign in

Search

Edit

Modifying the Default Skin

« Previous
Next »

This documentation has not been updated for Firefox Quantum. Support for the userChrome.css file and any of its elements described below are not guaranteed in future versions of Firefox. Using it may lead to hard-to-diagnose bugs or crashes. Use at your own risk!

This section describes how to modify the skin of a window.
Skin Basics

A skin is a set of style sheets, images and behaviors that are applied to a XUL file. By applying a different skin, you can change the look of a window without changing its functionality. Firefox provides a skin by default, and you may download others. The XUL for any skins is the same, however the style sheets and images used are different.

For a simple personalized look to a Firefox window, you can easily change the style sheets associated with it. Larger changes can be done by creating an entirely new skin. Firefox has a Theme Manager for changing the default skin. (Although the underlying code for Mozilla calls them skins and the user interface calls them themes, they're both referring to the same thing).

A skin is described using CSS, allowing you to define the colors, borders and images used to draw elements. The file classic.jar contains the skin definitions. The global directory within this archive contains the main style definitions for how to display the various XUL elements. By changing these files, you can change the look of the XUL applications.
Customize with userChrome.css

If you place a file called 'userChrome.css' in a directory called 'chrome' inside your user profile directory, you can override settings without changing the archives themselves. This directory should be created when you create a profile and some examples placed there. The file 'userContent.css' customizes Web pages, whereas 'userChrome.css' customizes chrome files.

For example, by adding the following to the end of the file, you can change all menubar elements to have a red background.

menubar {
  background-color: red;
}

If you open any Firefox window after making this change, the menu bars will be red. Because this change was made to the user style sheet, it affects all windows. This means that the browser menu bar, the bookmarks menu bar and even the find files menu bar will be red.
Skin Packages

To have the change affect only one window, change the style sheet associated with that XUL file. For example, to add a red border around the menu commands in the bookmarks manager window, add the following to bookmarksManager.css in the classic.jar or your favorite skin archive.

menuitem {
  border: 1px solid red;
}

If you look in one of the skin archives, you will notice that each contain a number of style sheets and a number of images. The style sheets refer to the images. You should avoid putting references to images directly in XUL files if you want your content to be skinnable. This is because a particular skin's design might not use images and it may need some more complex design. By referring to the images with CSS, they are easy to remove. It also removes the reliance on specific image filenames.

You can assign images to a button, checkbox and other elements by using the list-style-image property as in the following:

checkbox {
  list-style-image: url("chrome://findfile/skin/images/check-off.jpg");
}

checkbox[checked="true"] {
  list-style-image: url("chrome://findfile/skin/images/check-on.jpg");
}

This code changes the image associated with a checkbox. The first style sets the image for a normal checkbox and the second style sets the image for a checked checkbox. The modifier 'checked=true' makes the style only apply to elements which have their checked attributes set to true.

See also : creating a skin for Firefox and CSS getting started

In the next section, we will look at creating a new skin.

« Previous
Next »
Document Tags and Contributors
Tags: 

    Customization Firefox Intermediate Tutorials XUL XUL_Tutorial 

Contributors to this page: mconca, Potch, mfluehr, Sheppy, davelupt, seanclayton, KangHangsu, teoli, fscholz, Chbok, Mgjbot, Morishoji, Pmash, Ptak82, Dria
Last updated by: mconca, Nov 17, 2017, 12:13:04 PM
Learn the best of web development

Get the latest and greatest from MDN delivered straight to your inbox.
E-mail
MDN Web Docs

    Web Technologies
    Learn Web Development
    About MDN
    Feedback

Mozilla

    About
    Contact Us
    Firefox

Other languages:

    Terms Privacy Cookies 

© 2005-2018 Mozilla and individual contributors.

Content is available under these licenses.


    Skip to main content
    Select language
    Skip to search

MDN Web Docs

    Technologies References & Guides Feedback 

    Sign in

Search

Edit

Creating a Skin

« Previous
Next »

This documentation has not updated for Firefox Quantum. Modifying userChrome.css is not guaranteed to work between versions of Firefox and may lead to hard-to-diagnose bugs. Use at your own risk!

This section describes how to create a simple skin. For simplicity, we'll only apply it to the find files dialog.
A Simple Skin

The image below shows the current find files dialog. Let's create a skin that we can apply to it. Normally, a skin would apply to the entire application, but we'll focus on just the find files dialog to make it easier. For this reason, we'll modify only the file findfile.css rather than the global.css file. This section assumes that you are starting with the Classic skin. You may wish to make a copy of the files used by the find files dialog before editing.

Image:cskin1.jpg

You need to create a file 'findfile.css' in a custom skin. Or, you can temporarily place it in the content directory and refer to it using a stylesheet directive. You can modify the existing findfile.css directly to see what it looks like, or you can create a custom skin and link to that.
Creating a Custom Skin Package

To create a skin, do the following: (If you are using Firefox 1.5 or later, see Manifest Files instead of below)

    Create a directory somewhere where you want to place the skin files.
    Copy a manifest file (contents.rdf) from the Classic or Modern skin into this new directory.
    Modify the references in the manifest file to a custom name for your skin. For example, change references of 'classic/1.0' to 'blueswayedshoes/1.0'.
    Add a line to the file 'chrome/installed-chrome.txt of the following form: skin,install,url,file:///stuff/blueswayedshoes/ where the last part points to the directory you created. Make sure to add a slash at the end.

Copy the original findfile.css into the new directory. We'll use this as a basis for the new skin. We can then refer to it using the URL 'chrome://findfile/skin/findfile.css'.
Adding Style Rules

First, let's decide what kind of changes we want to make. We'll make some simple color changes, modify the button styles, and modify the spacing a bit. Let's start with the menus, toolbars and the overall tab panel.

The following style rules added to findfile.css will cause the changes shown in the accompanying image.

window > box {
  background-color: #0088CC;  
}

menubar,menupopup,toolbar,tabpanels {
  background-color: lightblue;
  border-top: 1px solid white;
  border-bottom: 1px solid #666666;
  border-left: 1px solid white;
  border-right: 1px solid #666666;
}

caption {
  background-color: lightblue;
}

Image:cskin2.jpg

    The inner box of the window (which actually surrounds all of the window content) has been changed to have a medium blue color.
    You can see this blue behind the tab strip and along the bottom of the window.
    Four elements, the menubar, the menupopup, the toolbar and the tabpanels appear in light blue.
    The border around these four elements has been changed to give a heavier 3D appearance. You can see this if you look closely.
    The background color of the caption has also been changed to match the background.

The first rule above (for 'window > box') specifies that the child box of the window has a different color. This probably isn't the best way to do this. We should really change this to use a style class. Let's do this. That way, we can modify the XUL without needing to keep the box as the first child of the window.

CSS:
.findfilesbox {
  background-color: #0088CC;
}

XUL:
<vbox class="findfilesbox" orient="vertical" flex="100%">
<toolbox>

Rounding on the Tabs

Next, let's modify the tabs. We will make the selected tab bold and change the rounding on the tabs.

tab:first-child {
  -moz-border-radius: 4px 0px 0px 0px;
}

tab:last-child {
  -moz-border-radius: 0px 4px 0px 0px;
}

tab[selected="true"] {
  color: #000066;
  font-weight: bold;
  text-decoration: underline;
}

Image:cskin3.jpg

Two rules change the normal tab appearance, the first sets the rounding on the first tab and the second sets the rounding on the last tab. Used here is a special Mozilla style rule, -moz-border-radius, that creates rounded border corners. The upper left border of the first tab and the upper right border of the second tab are rounded by four pixels and the other corners have a round corner of zero pixels, which is equivalent to no rounding. Increase the values here for more rounding and decrease them for a more rectangular look.

The last rule only applies to tabs that have their selected attribute set to true. It makes the text in the selected tab appear bold, underlined and dark blue. Note in the image that this style has applied only to the first tab, because it's the selected one.
Adding Toolbar Icons

It is somewhat difficult to distinguish the buttons on the toolbar from the commands on the menu. We could add some icons to the buttons to make them clearer. Mozilla Composer provides some icons for open and save buttons, which we'll just use here to save time. We can set the image for a button using the list-style-image CSS property.

#opensearch {
  list-style-image: url("chrome://editor/skin/icons/btn1.gif");
  -moz-image-region: rect(48px 16px 64px 0);  
  -moz-box-orient: vertical;
}

#savesearch {
  list-style-image: url("chrome://editor/skin/icons/btn1.gif");
  -moz-image-region: rect(80px 16px 96px 0);  
  -moz-box-orient: vertical;
}

Image:cskin4.jpg

Mozilla provides a custom style property -moz-image-region which can be used to make an element use part of an image. You can think of it as a clip region for the image. You set the property to a position and size within an image and the button will display only that section of the image. This allows you to use the same image for multiple buttons and set a different region for each one. When you have lots of buttons, with states for hover, active and disabled, this saves space that would normally be occupied by mutliple images. In the code above, we use the same image for each button, but set a different image region each one. If you look at this image (btn1.gif), you will notice that it contains a grid of smaller images, each one 16 by 16 pixels.

The -moz-box-orient property is used to orient the button vertically, so that the image appears above the label. This property has the same meaning as the orient attribute. This is convenient because the skin cannot change the XUL. Most of the box attributes have corresponding CSS properties.
Other Changes

Next, we'll make a couple of changes to the buttons along the bottom, again reusing some icons from Mozilla to save time. If creating your own skin, you will need to create new icons or copy the icons to new files. If following the example in this section, just copy the files to your new skin and change the URLs accordingly.

#find-button {
  list-style-image: url("chrome://global/skin/checkbox/images/cbox-check.jpg");
  font-weight: bold;
}
  
#cancel-button {
  list-style-image: url("chrome://global/skin/icons/images/close-button.jpg");
}

button:hover {
  color: #000066;
}

Image:cskin5.jpg

We add some images to the buttons and make the Find button have bold text to indicate that it is the default button. The last rule applies to buttons when the mouse is hovering over them. We set the text color to dark blue in this case. Finally, some minor changes to the spacing around the items, by setting margins:

tabbox {
  margin: 4px;
}

toolbarbutton {
  margin-left: 3px;
  margin-right: 3px;
}

After those changes, the find files dialog now looks like the image.

Image:cskin6.jpg

As you can see, some simple changes to the style rules has resulted in quite a different appearance to the find files dialog. We could continue by changing the menus, the grippies on the toolbar and the input and checkbox elements.
Creating a Global Skin

The skin created above is simple and only applies to the find files dialog. Some of the changes made to the skin could be placed in the global style sheets (those in the global directory of the skin) to be applied to all applications. For example, having different images for the check boxes in the find files dialog as other windows looks a little odd. This change should really be moved into the global style sheet.

Try moving the CSS styles from findfile.css into global.css and then look at some of the dialogs in Mozilla. (The cookie viewer is a good example.) You will notice that it has adopted the rules that we have added. Some of the rules conflict with those already in the global stylesheets. For example, rules are already defined for buttons and tabs and so on and we defined additional rules for them. When changing the global skin, you would need to merge the changes into the existing rules.

For the best skinnability, it is best to declare appearance related style rules in the global directory rather than in individual style files. This includes colors, fonts and general widget appearances. If you change the color of something in a local skin file (such as findfile.css), the dialog may look odd if the user changes their global skin. Don't expect the user to be using the default one.

Our Find files example with this skin : Source View Stylesheet
See also

    Mozilla CSS extensions
    CSS reference

The next section discusses how to make a XUL application localizable.

« Previous
Next »
Document Tags and Contributors
Tags: 

    Tutorials XUL XUL_Tutorial 

Contributors to this page: Potch, mfluehr, Sheppy, teoli, trevorh, fscholz, Chbok, Mgjbot, Morishoji, Jjinux, Pmash, Ptak82, Dria
Last updated by: Potch, Nov 17, 2017, 11:14:12 AM
Learn the best of web development

Get the latest and greatest from MDN delivered straight to your inbox.
E-mail
MDN Web Docs

    Web Technologies
    Learn Web Development
    About MDN
    Feedback

Mozilla

    About
    Contact Us
    Firefox

Other languages:

    Terms Privacy Cookies 

© 2005-2018 Mozilla and individual contributors.

Content is available under these licenses.


    Skip to main content
    Select language
    Skip to search

MDN Web Docs

    Technologies References & Guides Feedback 

    Sign in

Search

Edit

Localization

XUL and XML provide entities which are a convenient way of allowing localization.
Entities

Many applications are built such that translating the interface into a different language is as simple as possible. Usually, a table of strings is created for each language. Instead of hard-coding text directly into an application, each piece of text is only a reference into the string table. XML provides entities which can be used for a similar purpose.

You should already be familiar with entities if you have written HTML. The codes &lt; and &gt; are examples of entities which can be used to place less than and greater than signs into the text. XML has a syntax which allows you to declare custom entities. You can use these so that the entity is replaced with its value, which can be a string of text. Entities may be used whenever text occurs, including the values of attributes. The example below demonstrates the use of an entity in a button.

<button label="&findLabel;"/>

The text that will appear on the label will be the value that the entity &findLabel; has. A file is created which contains the entity declarations for each supported language. In English, the &findLabel; entity will probably be declared to have the text "Find".
DTD Files

Entities are declared in Document Type Definition (DTD) files. Files of this type are normally used to declare the syntax and semantics of a particular XML file, but they also let you declare entities. In the Mozilla chrome system, you will find DTD files located in the locales subdirectory. You would normally have one DTD file (with an extension .dtd) per XUL file.

If you look in the chrome directory, you should see an archive for your language. (en-US.jar is the default for English.) You might have locale files in multiple languages, for example, US English (en-US) and French (fr). Inside these archives, you will find the files that hold the localized text for each window. The structure of the archives is very similar to the directory structure used for skins.

Inside the archives, you would place the DTD files in which you declare entities. Typically, you will have one DTD file for each XUL file, usually with the same filename except with a .dtd extension. So, for the find files dialog, we will need a file called findfile.dtd.

For non-installed chrome files, you can just put the DTD file in the same directory as the XUL file.
Note: You should encode DTD files as UTF-8 for non-ASCII characters. That is, you should save them in UTF-8 format (without BOM). For more information, see Mozilla Language Packs.

Once you have created a DTD file for your XUL, you will need to add a line to the XUL file which indicates that you want to use the DTD file. Otherwise, errors will occur as it won't be able to find the entities. To do this, add a line of the following form somewhere near the top of the XUL file:

<!DOCTYPE window SYSTEM "chrome://findfile/locale/findfile.dtd">

This line specifies that the URL indicated is to be used as a DTD for the file. In this case, we have declared that we want to use the findfile.dtd DTD file. This line is typically placed just before the window element.

You also need to add the locale to the chrome.manifest file, for example:

locale findfile en-US locale/

Declaring Entities

The entities are declared using a simple syntax as shown below:

<!ENTITY findLabel "Find">

This example creates an entity with the name findLabel and the value "Find". This means that wherever the text "&findLabel;" appears in the XUL file, it will be replaced with the text "Find". Note that entity declarations do not have a trailing slash at the end of them. In the DTD file for a different language, the text for that language will be used instead.

for Japanese:
<!ENTITY findLabel "検索">

For example, the following text:

<description value="&findLabel;"/>

is translated as:

English version:
<description value="Find"/>

Japanese version:
<description value="検索"/>

You would declare an entity for each label or string of text that you use in your interface. You should not have any directly displayed text in the XUL file at all.

In addition to using entities for text labels, you should use them for any value that could be different in a different language. Access keys and keyboard shortcuts for example.

XUL
 <menuitem label="&undo.label;" accesskey="&undo.key;"/>
 DTD
 <!ENTITY undo.label "Undo">
 <!ENTITY undo.key "u">

The example above uses two entities, one for the label on the Undo menu item and the second for the access key.

 
Changing the Find Files example

Let's take a look at how we would put all of this together by modifying the find files dialog so that it uses a DTD file for all of its text strings. The entire XUL file is shown below with the changes shown in red.

<?xml version="1.0"?>

<?xml-stylesheet href="chrome://global/skin/" type="text/css"?>
<?xml-stylesheet href="findfile.css" type="text/css"?>

<!DOCTYPE window SYSTEM "chrome://findfile/locale/findfile.dtd">

<window
  id="findfile-window"
  title="&findWindow.title;"
  persist="screenX screenY width height"
  orient="horizontal"
  onload="initSearchList()"
  xmlns="http://www.mozilla.org/keymaster/gatekeeper/there.is.only.xul">

<script src="findfile.js"/>

<popupset>
   <menupopup id="editpopup">
     <menuitem label="&cutCmd.label;" accesskey="&cutCmd.accesskey;"/>
     <menuitem label="&copyCmd.label;" accesskey="&copyCmd.accesskey;"/>
     <menuitem label="&pasteCmd.label;" accesskey="&pasteCmd.accesskey;" disabled="true"/>
   </menupopup>
</popupset>

<keyset>
   <key id="cut_cmd" modifiers="accel" key="&cutCmd.commandkey;"/>
   <key id="copy_cmd" modifiers="accel" key="&copyCmd.commandkey;"/>
   <key id="paste_cmd" modifiers="accel" key="&pasteCmd.commandkey;"/>
   <key id="close_cmd" keycode="VK_ESCAPE" oncommand="window.close();"/>
</keyset>

<vbox flex="1">

 <toolbox>

  <menubar id="findfiles-menubar">
    <menu id="file-menu" label="&fileMenu.label;"
        accesskey="&fileMenu.accesskey;">
      <menupopup id="file-popup">
        <menuitem label="&openCmd.label;"
                  accesskey="&openCmd.accesskey;"/>
        <menuitem label="&saveCmd.label;"
                  accesskey="&saveCmd.accesskey;"/>
        <menuseparator/>
        <menuitem label="&closeCmd.label;"
                  accesskey="&closeCmd.accesskey;" key="close_cmd" oncommand="window.close();"/>
      </menupopup>
    </menu>
    <menu id="edit-menu" label="&editMenu.label;"
          accesskey="&editMenu.accesskey;">
      <menupopup id="edit-popup">
        <menuitem label="&cutCmd.label;"
                  accesskey="&cutCmd.accesskey;" key="cut_cmd"/>
        <menuitem label="&copyCmd.label;"
                  accesskey="&copyCmd.accesskey;" key="copy_cmd"/>
        <menuitem label="&pasteCmd.label;"
                  accesskey="&pasteCmd.accesskey;" key="paste_cmd" disabled="true"/>
      </menupopup>
    </menu>
  </menubar>

  <toolbar id="findfiles-toolbar">
    <toolbarbutton id="opensearch" label="&openCmdToolbar.label;"/>
    <toolbarbutton id="savesearch" label="&saveCmdToolbar.label;"/>
  </toolbar>
 </toolbox>

 <tabbox>
  <tabs>
    <tab label="&searchTab;" selected="true"/>
    <tab label="&optionsTab;"/>
  </tabs>

  <tabpanels>

   <tabpanel id="searchpanel" orient="vertical" context="editpopup">

   <description>
     &findDescription;
   </description>

   <spacer class="titlespace"/>

   <groupbox orient="horizontal">
     <caption label="&findCriteria;"/>

     <menulist id="searchtype">
       <menupopup>
         <menuitem label="&type.name;"/>
         <menuitem label="&type.size;"/>
         <menuitem label="&type.date;"/>
       </menupopup>
     </menulist>
   <spacer class="springspace"/>
     <menulist id="searchmode">
       <menupopup>
         <menuitem label="&mode.is;"/>
         <menuitem label="&mode.isnot;"/>
       </menupopup>
     </menulist>
   <spacer class="springspace"/>

   <menulist id="find-text" flex="1"
             editable="true"
             datasources="file:///mozilla/recents.rdf"
             ref="http://www.xulplanet.com/rdf/recent/all">
     <template>
       <menupopup>
         <menuitem label="rdf:http://www.xulplanet.com/rdf/recent#Label" uri="rdf:*"/>
       </menupopup>
     </template>
   </menulist>

   </groupbox>

  </tabpanel>

  <tabpanel id="optionspanel" orient="vertical">
     <checkbox id="casecheck" label="&casesensitive;"/>
     <checkbox id="wordscheck" label="&matchfilename;"/>
    </tabpanel>

  </tabpanels>
 </tabbox>

 <tree id="results" style="display: none;" flex="1">
   <treecols>
     <treecol id="name" label="&results.filename;" flex="1"/>
     <treecol id="location" label="&results.location;" flex="2"/>
     <treecol id="size" label="&results.size;" flex="1"/>
   </treecols>

   <treechildren>
     <treeitem>
       <treerow>
         <treecell label="mozilla"/>
         <treecell label="/usr/local"/>
         <treecell label="&bytes.before;2520&bytes.after;"/>
       </treerow>
     </treeitem>
   </treechildren>
 </tree>

 <splitter id="splitbar" resizeafter="grow" style="display: none;"/>

 <spacer class="titlespace"/>

 <hbox>
   <progressmeter id="progmeter" value="50%" style="display: none;"/>
   <spacer flex="1"/>
   <button id="find-button" label="&button.find;"
           oncommand="doFind()"/>
   <button id="cancel-button" label="&button.cancel;"
           oncommand="window.close();"/>
 </hbox>
</vbox>

</window>

Each text string has been replaced by an entity reference. A DTD file has been included near the beginning of the XUL file. Each entity that was added should be declared in the DTD file. The window will not be displayed if an entity is found in the XUL file that hasn't been declared.

Note that the name of the entity is not important. In the example above, words in entities have been separated with periods. You don't have to do this. The entity names here follow similar conventions as the rest of the Mozilla code.

You might notice that the text '2520 bytes' has been replaced by two entities. This is because the phrase structure may be different in another locale. For example, the number might need to appear before the equivalent of 'bytes' instead of after. Of course, this might need to be more complicated in order to display KB or MB as needed.

The access keys and keyboard shortcuts have also been translated into entities because they will likely be different in a different locale.

Next, the DTD file - findfile.dtd:

<!ENTITY findWindow.title "Find Files">
<!ENTITY fileMenu.label "File">
<!ENTITY editMenu.label "Edit">
<!ENTITY fileMenu.accesskey "f">
<!ENTITY editMenu.accesskey "e">
<!ENTITY openCmd.label "Open Search...">
<!ENTITY saveCmd.label "Save Search...">
<!ENTITY closeCmd.label "Close">
<!ENTITY openCmd.accesskey "o">
<!ENTITY saveCmd.accesskey "s">
<!ENTITY closeCmd.accesskey "c">
<!ENTITY cutCmd.label "Cut">
<!ENTITY copyCmd.label "Copy">
<!ENTITY pasteCmd.label "Paste">
<!ENTITY cutCmd.accesskey "t">
<!ENTITY copyCmd.accesskey "c">
<!ENTITY pasteCmd.accesskey "p">
<!ENTITY cutCmd.commandkey "X">
<!ENTITY copyCmd.commandkey "C">
<!ENTITY pasteCmd.commandkey "V">
<!ENTITY openCmdToolbar.label "Open">
<!ENTITY saveCmdToolbar.label "Save">
<!ENTITY searchTab "Search">
<!ENTITY optionsTab "Options">
<!ENTITY findDescription "Enter your search criteria below and select the Find button to begin the search.">
<!ENTITY findCriteria "Search Criteria">
<!ENTITY type.name "Name">
<!ENTITY type.size "Size">
<!ENTITY type.date "Date Modified">
<!ENTITY mode.is "Is">
<!ENTITY mode.isnot "Is Not">
<!ENTITY casesensitive "Case Sensitive Search">
<!ENTITY matchfilename "Match Entire Filename">
<!ENTITY results.filename "Filename">
<!ENTITY results.location "Location">
<!ENTITY results.size "Size">
<!ENTITY bytes.before "">
<!ENTITY bytes.after "bytes">
<!ENTITY button.find "Find">
<!ENTITY button.cancel "Cancel">

Now, to add text for a new language all you need to do is create another DTD file. By using the chrome system to add the DTD file to a different locale, the same XUL file can be used in any language.

Find files example so far: Source

Next, we'll look at property files.

« Previous
Next »
See Also

    How to localize html pages, xul files, and js/jsm files from bootstrapped add-ons: Bootstrapped Extensions :: Localization (L10n)

Document Tags and Contributors
Tags: 

    Internationalization Localization Tutorials XUL XUL_Tutorial 

Contributors to this page: SphinxKnight, Noitidart, Sheppy, teoli, jbeatty, fscholz, Chbok, Enn, Jomel, J05ef, MarkFinkle, Nickolay, Pmash, Morishoji, Prodizy, Mgjbot, Ricky, Ptak82, Dria, Jens.B
Last updated by: SphinxKnight, Feb 9, 2018, 10:10:14 AM
Learn the best of web development

Get the latest and greatest from MDN delivered straight to your inbox.
E-mail
MDN Web Docs

    Web Technologies
    Learn Web Development
    About MDN
    Feedback

Mozilla

    About
    Contact Us
    Firefox

Other languages:

    Terms Privacy Cookies 

© 2005-2018 Mozilla and individual contributors.

Content is available under these licenses.
