# Minra

## Model design

### Source

a pdf, webpage that will be continuously read over time

| id  | checksum         | title             | author            | description                                                        | path                    |
| --- | ---------------- | ----------------- | ----------------- | ------------------------------------------------------------------ | ----------------------- |
| 1   | aGVsbG8gd29ybGQ= | Dark Lords Answer | Eliezer Yudkowsky | Transported to another world story with a focus on macro economics | ./dark-lords-answer.pdf |


### tags
| id  | name    |
| --- | ------- |
| 1   | fiction |
  
### source_tagmaps

mapping between sources and tags, allowing for many to many relation

| id  | source | tag |
| --- | ------ | --- |
| 1   | 1      | 1   |

### chunk
| id  | source | start | end | comment |
| --- | ------ | ----- | --- | ------- |
| 1   | 1      | 0.0   | 5.0 |         |

### chunk_tagmaps

mapping between chunks and tags

| id  | chunk | tag |
| --- | ----- | --- |


review_item:


## Related work

- https://github.com/Networks-Learning/memorize
- https://github.com/open-spaced-repetition/fsrs4anki/wiki/Research-resources
- https://github.com/open-spaced-repetition