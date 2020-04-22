# selenium

```bash
sudo pip install -i https://pypi.tuna.tsinghua.edu.cn/simple selenium
```

```py
from selenium import webdriver
from selenium.common.exceptions import TimeoutException
from selenium.webdriver.support.ui import WebDriverWait
from selenium.webdriver.support import expected_conditions as EC

driver = webdriver.Firefox()
driver.get("http://www.google.com")

print driver.title

inputElement = driver.find_element_by_name("q")
inputElement.send_keys("cheese!")
inputElement.submit()

WebDriverWait(driver, 10).until(EC.title_contains("cheese!"))

driver.quit()
```

## 查找元素

* e = driver.find_element_by_id("coolestWidgetEvah")                   By ID
* e = driver.find_element_by_class_name("cheese")                      By Class Name
* e = driver.find_element_by_tag_name("iframe")                        By Tag Name
* e = driver.find_element_by_name("cheese")                            By Name
* e = driver.find_element_by_link_text("cheese")                       By Link Text
* e = driver.find_element_by_partial_link_text("cheese")               By Partial Link Text
* e = driver.find_element_by_css_selector("#food span.dairy.aged")     By CSS
* e = driver.find_element_by_xpath("//input")                          By XPath

返回一个列表, 包含所有匹配的元素

* find_elements_by_id()
* find_elements_by_name()
* find_elements_by_tag_name()
* find_elements_by_link_text()
* find_elements_by_partial_link_text()
* find_elements_by_xpath()
* find_elements_by_css_selector()
    
## wait

```py
driver.implicitly_wait(3)
```
    
## JavaScript

```py
element = driver.execute_script("return $('.cheese')[0]")
```

## Getting text values

```py
element = driver.find_element_by_id("element_id")
element.text
```
    
## User Input

```py
select = driver.find_element_by_tag_name("select")
allOptions = select.find_elements_by_tag_name("option")
for option in allOptions:
    print "Value is: " + option.get_attribute("value")
    option.click()
    
driver.find_element_by_id("submit").click()
```
    
## Moving Between Windows and Frames

```py
driver.switch_to.window("windowName")
    
for handle in driver.window_handles:
    driver.switch_to.window(handle)
```
    
## Popup Dialogs

```py
alert = driver.switch_to.alert
alert.dismiss()
```
    
## Navigation: History and Location

```py
driver.get("http://www.example.com")  # python doesn't have driver.navigate
driver.forward()
driver.back()
```
    
## Cookies

```py
driver.get("http://www.example.com")

driver.add_cookie({'name':'key', 'value':'value', 'path':'/'})
# additional keys that can be passed in are:
# 'domain' -> String,
# 'secure' -> Boolean,
# 'expiry' -> Milliseconds since the Epoch it should expire.

# And now output all the available cookies for the current URL
for cookie in driver.get_cookies():
    print "%s -> %s" % (cookie['name'], cookie['value'])

driver.get_cookie("key")

# You can delete cookies in 2 ways
# By name
driver.delete_cookie("CookieName")
# Or all of them
driver.delete_all_cookies()
```
    
## Changing the User Agent

```py
profile = webdriver.FirefoxProfile()

profile.set_preference("general.useragent.override", "some UA string")
profile.native_events_enabled = True

driver = webdriver.Firefox(profile)
```

## Drag And Drop

```py
from selenium.webdriver.common.action_chains import ActionChains

element = driver.find_element_by_name("source")
target =  driver.find_element_by_name("target")

ActionChains(driver).drag_and_drop(element, target).perform()
```

## 常用操作

* webEle.clear()            清楚元素的内容, 假如这个元素是一个文本元素
* webEle,is_displayed()     当前元素是否可见
* webEle.is_enabled()       当前元素是否禁止, 比如经常会禁用一些元素的点击
* webEle.is_selected()      当前元素是否选中 文本输入框的内容
* webEle.tag_name           当前元素的标签名
* browser.curren_url        当前加载页面的 URL
* browser.close()           关闭当前窗口
* browser.name              当前浏览器的名字
* browser.current_window_handle                 当前窗口的， 相当于指针, 指向当前窗口
* browser.switch_to_window(window_handle)       切换 window_handle 指向的窗口
* browser.execute_async_script(script, *args)   异步执行 js 脚本

# ldtp