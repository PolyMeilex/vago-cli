# vago-cli
Quick and easy CLI way to traverse directiories with fuzzy matching

![image](https://github.com/PolyMeilex/vago-cli/assets/20758186/d180e8da-5786-48ba-bbf3-7a6cbc684fd2)


### Example usage with fish
```fish
function vago
  command vago $argv
  if test $status -eq 200
    cd (cat /tmp/vago-result)
  end
  clear
end
```
