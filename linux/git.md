# git

dadmars
github: burning2001

## 创建版本库

```bash
git clone --bare a a.git
git daemon --verbose --export-all --base-path=./
```

## 配置

* /etc/gitconfig    所有用户的配置文件
* ~/.gitconfig      当前用户的配置文件
* .git/config       当前工作区的配置文件

### 列出配置

```bash
git config --system
git config --global
git config --list
git config user.name
```

### 设置

```bash
git config --global user.name "John Doe" 
git config --global user.email johndoe@example.com

git config --global alias.co checkout
git config --global alias.br branch
git config --global alias.ci commit
git config --global alias.st status
git config --global alias.unstage 'reset HEAD --'
git config --global alias.last 'log -1 HEAD'
```

## 命令

```bash
git status

git diff                staged状态与modified之间的不同
git diff --staged       staged状态与commited之间的不同
git diff --cached       上同

git rm --cached README      删除文件，但文件仍然在工作区
git rm readme               删除文件，同时删除工作区文件
git rm -f readme            当文件被修改时

git log -p -2          最近的2条，显示patch
git log --stat         修改的文件
git log --oneline --decorate   分支的HEAD指向哪里
git log --pretty=oneline  输出格式化
git log --pretty=format:"%h - %an, %ar : %s"
git log --pretty=format:"%h %s" --graph    显示分支
--name-only
--name-status

git tag -a v1.4 -m 'my version 1.4'     创建tag
git tag -a v1.2 9fceb02                 在一个点上打tag
git tag v1.4-lw                         创建轻量tag

git tag                列出所有tag
git tag -l 'v1.8.5*'   查找tag
git show v1.4          查看tag信息

git push origin v1.5          将tag上传到源，默认push不上传tag
git push origin --tags        上传所有tag

git branch -v     列出分支
git branch -a     列出所有分支
```

## 配置服务器

```bash
sudo adduser git
su git
cd
mkdir .ssh && chmod 700 .ssh
touch .ssh/authorized_keys && chmod 600 .ssh/authorized_keys
cat /tmp/id_rsa.john.pub >> ~/.ssh/authorized_keys
cat /tmp/id_rsa.josie.pub >> ~/.ssh/authorized_keys
cat /tmp/id_rsa.jessica.pub >> ~/.ssh/authorized_keys

cd /opt/git
mkdir project.git
cd project.git
git init --bare
# 或增加组写权限
git init --bare --shared

# on Johns computer
$ cd myproject
$ git init
$ git add .
$ git commit -m 'initial commit'
$ git remote add origin git@gitserver:/opt/git/project.git
$ git push origin master

$ git clone git@gitserver:/opt/git/project.git
$ cd project
$ vim README
$ git commit -am 'fix for the README file'
$ git push origin master

$ cat /etc/shells   # see if `git-shell` is already in there.  If not...
$ which git-shell   # make sure git-shell is installed on your system.
$ sudo vim /etc/shells  # and add the path to git-shell from last command

sudo chsh git  # and enter the path to git-shell, usually: /usr/bin/git-shell

ssh git@gitserver
```
