# usage

```shell
cd py_playground


python3 -m unittest knapsack_test

python3 main.py

```



mac 删除python环境

```
sudo rm -rf /Library/Frameworks/Python.framework/Versions/2.7

sudo rm -rf "/Applications/Python 2.7"

ls -l /usr/local/bin | grep '../Library/Frameworks/Python.framework/Versions/2.7' 

cd /usr/local/bin/

ls -l /usr/local/bin | grep '../Library/Frameworks/Python.framework/Versions/2.7' | awk '{print $9}' | tr -d @ | xargs rm

```
