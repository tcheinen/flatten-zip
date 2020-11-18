# flatten-zip

Sometimes you just have a zip file which looks like this

```text
.
└── root.zip/
    ├── folder1.zip/
    │   └── folder/
    │       └── data.txt
    ├── folder2.zip/
    │   └── data.txt
    └── folder3.zip/
        └── folder/
            └── folder/
                ├── folder/
                │   └── data1.txt
                └── folder2/
                    └── data2.txt
```
and want it to look like this (or at least I do, presumably you do also if you're reading this)
```text
.
└── root/
    ├── folder1/
    │   └── data.txt
    ├── folder2/
    │   └── data.txt
    └── folder3/
        ├── data1.txt
        └── data2.txt
```

Well, you're in luck because this'll do it for you.  

`flatten-zip root.zip`