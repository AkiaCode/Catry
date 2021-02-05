# Catry Story Book (CSB)

## Keywords
```
BRANCH
CHOICE
GOTO
etc...
```

### !! WARNING !!
Every program needs to start with `BRANCH: 1`. (Otherwise it'll not work.)

#### Keyword description

##### `BRANCH: [number]` is something like a middle of the story.
```
BRANCH: 1
ㄴ script
ㄴ script
ㄴ script
ㄴ ...
```

##### `CHOICE: [message 1] | [message 2] | ....` is to let user select a message.
If there's no `GOTO: [number]` for `CHOICE` result then it'll go to branch 1.
```
CHOICE: Like it | Don't like it
GOTO: 2
GOTO: 1
```

##### `GOTO: [number]` is for moving to branch `number`.
```
GOTO: 2
ㄴ BRANCH: 2
GOTO: 1
ㄴ BRANCH: 1
```

##### Every other things will be printed as `[keyword]: [message]`.
