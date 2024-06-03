# neuron_dance

<p float="center">
  <img src='images/dance.jpg' width='600' height='425'/>
</p>

```rust
// main.rs

// split dataset into train and test
let mut tts = dataset.train_test_split(train_percentage);
let mut subsets = tts.get_ref();
let mut model;

// for example, match cmd line option iris and build matching model iris
NetworkType::Iris => {
    model = Network::new();
    model.add(Input1(4));
    model.add(Dense(10, Act::Relu));
    model.add(Dense(10, Act::Relu));
    model.add(Dense(3, Act::Sigmoid));
    model.compile(Loss::CrossEntropy, 0.005, 0.3, Metr("accuracy, cost"));
    model.fit(&subsets, 100, Batch::Mini(5), Eval::Test);
    model.eval(&subsets, Eval::Test);
```

> What to wear for the neuron dance? |> head 0..14 mnist-fashion-file | heatmap

<p float="center">
  <img src='images/fashion.jpg' width='950' height='325'/>
</p>

## Model Examples

### 1) Mnist handwritten digits

```rust

$ cargo run --release -- -t mnist
    Finished release [optimized] target(s) in 0.19s
     Running `/home/brpandey/Workspace/ml/rust/neuron_dance/target/release/neuron_dance -t mnist`

Data subset shapes are x_train shape [60000, 784], y_train shape  [60000, 1],
x_test shape [10000, 784], y_test shape [10000, 1]

// head 0..7 mnist-file | heatmap

Epoch 1/5
	Accuracy 0.9571 9571/10000 (MiniBatch + Adam)

Epoch 2/5
	Accuracy 0.9659 9659/10000 (MiniBatch + Adam)

Epoch 3/5
	Accuracy 0.9689 9689/10000 (MiniBatch + Adam)

Epoch 4/5
	Accuracy 0.9700 9700/10000 (MiniBatch + Adam)

Epoch 5/5
	Accuracy 0.9720 9720/10000 (MiniBatch + Adam)

[Successful y prediction] correct label is 9
=> for reduced x input image, see grid below
╭───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───╮
│   ┆   ┆   ┆   ┆   ┆   ┆   ┆   ┆   ┆   ┆   ┆   ┆   ┆   │
├╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┤
│   ┆   ┆   ┆   ┆   ┆ @ ┆ X ┆ X ┆ # ┆ X ┆   ┆   ┆   ┆   │
├╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┤
│   ┆   ┆   ┆   ┆ % ┆ X ┆ X ┆ X ┆ X ┆ @ ┆   ┆   ┆   ┆   │
├╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┤
│   ┆   ┆   ┆   ┆ X ┆ X ┆ X ┆   ┆   ┆   ┆   ┆   ┆   ┆   │
├╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┤
│   ┆   ┆   ┆   ┆ % ┆ X ┆ X ┆ X ┆ X ┆ @ ┆   ┆   ┆   ┆   │
├╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┤
│   ┆   ┆   ┆   ┆ % ┆ X ┆ X ┆ X ┆ X ┆ X ┆   ┆   ┆   ┆   │
├╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┤
│   ┆   ┆   ┆   ┆   ┆   ┆   ┆ % ┆ X ┆ @ ┆   ┆   ┆   ┆   │
├╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┤
│   ┆   ┆   ┆   ┆   ┆   ┆   ┆ X ┆ X ┆ @ ┆   ┆   ┆   ┆   │
├╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┤
│   ┆   ┆   ┆   ┆   ┆   ┆   ┆ X ┆ X ┆ % ┆   ┆   ┆   ┆   │
├╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┤
│   ┆   ┆   ┆   ┆   ┆   ┆ % ┆ X ┆ X ┆   ┆   ┆   ┆   ┆   │
├╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┤
│   ┆   ┆   ┆   ┆   ┆   ┆ % ┆ X ┆ X ┆   ┆   ┆   ┆   ┆   │
├╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┤
│   ┆   ┆   ┆   ┆   ┆   ┆   ┆   ┆   ┆   ┆   ┆   ┆   ┆   │
╰───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───╯
[Successful y prediction] correct label is 3
=> for reduced x input image, see grid below
╭───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───╮
│   ┆   ┆   ┆   ┆   ┆   ┆   ┆   ┆   ┆   ┆   ┆   ┆   ┆   │
├╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┤
│   ┆   ┆   ┆   ┆ @ ┆ # ┆ X ┆ @ ┆   ┆   ┆   ┆   ┆   ┆   │
├╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┤
│   ┆   ┆   ┆   ┆ X ┆ X ┆ X ┆ X ┆ X ┆ % ┆   ┆   ┆   ┆   │
├╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┤
│   ┆   ┆   ┆   ┆   ┆   ┆   ┆ % ┆ X ┆ X ┆   ┆   ┆   ┆   │
├╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┤
│   ┆   ┆   ┆   ┆   ┆   ┆   ┆   ┆ % ┆ X ┆ X ┆   ┆   ┆   │
├╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┤
│   ┆   ┆   ┆   ┆   ┆   ┆   ┆ @ ┆ @ ┆ X ┆ X ┆   ┆   ┆   │
├╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┤
│   ┆   ┆   ┆   ┆   ┆   ┆ @ ┆ X ┆ X ┆ X ┆ X ┆   ┆   ┆   │
├╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┤
│   ┆   ┆   ┆ X ┆   ┆   ┆   ┆ X ┆ X ┆ X ┆   ┆   ┆   ┆   │
├╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┤
│   ┆   ┆   ┆ X ┆ % ┆   ┆   ┆ % ┆ X ┆ X ┆   ┆   ┆   ┆   │
├╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┤
│   ┆   ┆   ┆ X ┆ X ┆ X ┆ X ┆ X ┆ X ┆ X ┆   ┆   ┆   ┆   │
├╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┤
│   ┆   ┆   ┆   ┆ @ ┆ X ┆ X ┆ X ┆ @ ┆   ┆   ┆   ┆   ┆   │
├╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┤
│   ┆   ┆   ┆   ┆   ┆   ┆   ┆   ┆   ┆   ┆   ┆   ┆   ┆   │
╰───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───╯
[Successful y prediction] correct label is 7
=> for reduced x input image, see grid below
╭───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───╮
│   ┆   ┆   ┆   ┆   ┆   ┆   ┆   ┆   ┆   ┆   ┆   ┆   ┆   │
├╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┤
│   ┆   ┆   ┆   ┆   ┆   ┆ % ┆ # ┆ # ┆   ┆   ┆   ┆   ┆   │
├╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┤
│   ┆   ┆ X ┆ X ┆ X ┆ X ┆ X ┆ X ┆ X ┆ X ┆ % ┆   ┆   ┆   │
├╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┤
│   ┆   ┆ X ┆ X ┆ X ┆ X ┆ X ┆ @ ┆ X ┆ X ┆ @ ┆   ┆   ┆   │
├╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┤
│   ┆   ┆   ┆   ┆   ┆   ┆   ┆   ┆ X ┆ X ┆ X ┆   ┆   ┆   │
├╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┤
│   ┆   ┆   ┆   ┆   ┆   ┆   ┆   ┆ X ┆ X ┆ @ ┆   ┆   ┆   │
├╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┤
│   ┆   ┆   ┆   ┆   ┆   ┆   ┆ @ ┆ X ┆ X ┆ % ┆   ┆   ┆   │
├╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┤
│   ┆   ┆   ┆   ┆   ┆   ┆   ┆ X ┆ X ┆ X ┆   ┆   ┆   ┆   │
├╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┤
│   ┆   ┆   ┆   ┆   ┆   ┆ @ ┆ X ┆ X ┆ % ┆   ┆   ┆   ┆   │
├╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┤
│   ┆   ┆   ┆   ┆   ┆   ┆ X ┆ X ┆ X ┆   ┆   ┆   ┆   ┆   │
├╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┤
│   ┆   ┆   ┆   ┆   ┆   ┆ X ┆ X ┆ % ┆   ┆   ┆   ┆   ┆   │
├╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┤
│   ┆   ┆   ┆   ┆   ┆   ┆ X ┆ X ┆   ┆   ┆   ┆   ┆   ┆   │
╰───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───╯
[No match!] y prediction 3 is different from correct y label 5
=> for reduced x input image, see grid below
╭───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───╮
│   ┆   ┆   ┆   ┆   ┆   ┆   ┆   ┆   ┆   ┆   ┆   ┆   ┆   │
├╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┤
│   ┆   ┆   ┆   ┆   ┆   ┆   ┆ % ┆ X ┆ # ┆ X ┆   ┆   ┆   │
├╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┤
│   ┆   ┆   ┆   ┆   ┆   ┆ % ┆ X ┆ X ┆ X ┆ @ ┆   ┆   ┆   │
├╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┤
│   ┆   ┆   ┆   ┆   ┆ % ┆ # ┆ X ┆ X ┆ X ┆   ┆   ┆   ┆   │
├╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┤
│   ┆   ┆   ┆   ┆   ┆ X ┆ X ┆ X ┆ X ┆ X ┆   ┆   ┆   ┆   │
├╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┤
│   ┆   ┆   ┆   ┆   ┆ X ┆ X ┆ X ┆ X ┆ X ┆ % ┆   ┆   ┆   │
├╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┤
│   ┆   ┆ % ┆ @ ┆ % ┆   ┆ @ ┆ @ ┆ @ ┆ # ┆ % ┆   ┆   ┆   │
├╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┤
│   ┆ % ┆ X ┆ X ┆ X ┆   ┆   ┆   ┆ X ┆ X ┆ % ┆   ┆   ┆   │
├╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┤
│   ┆ % ┆ X ┆ X ┆ # ┆ % ┆ % ┆ X ┆ X ┆ X ┆   ┆   ┆   ┆   │
├╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┤
│   ┆   ┆ X ┆ X ┆ X ┆ X ┆ X ┆ X ┆ X ┆ @ ┆   ┆   ┆   ┆   │
├╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┤
│   ┆   ┆   ┆ @ ┆ X ┆ X ┆ X ┆ X ┆ % ┆   ┆   ┆   ┆   ┆   │
├╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┤
│   ┆   ┆   ┆   ┆   ┆   ┆   ┆   ┆   ┆   ┆   ┆   ┆   ┆   │
╰───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───╯
Accuracy 0.9720 9720/10000

```

### 2) Iris flower species

```rust

$ cargo run --release -- -t iris
    Finished release [optimized] target(s) in 0.20s
     Running `/home/brpandey/Workspace/ml/rust/neuron_dance/target/release/neuron_dance -t iris`

> head csv-file
╭──────────────┬─────────────┬──────────────┬─────────────┬─────────╮
│ sepal_length ┆ sepal_width ┆ petal_length ┆ petal_width ┆ species │
╞══════════════╪═════════════╪══════════════╪═════════════╪═════════╡
│ 6.5          ┆ 3.2         ┆ 5.1          ┆ 2           ┆ 2       │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ 7.2          ┆ 3.6         ┆ 6.1          ┆ 2.5         ┆ 2       │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ 7.4          ┆ 2.8         ┆ 6.1          ┆ 1.9         ┆ 2       │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ 6.1          ┆ 2.9         ┆ 4.7          ┆ 1.4         ┆ 1       │
╰──────────────┴─────────────┴──────────────┴─────────────┴─────────╯
Data subset shapes x_train shape [100, 4], y_train shape  [100, 1],
x_test shape [50, 4], y_test shape [50, 1]

Epoch 1/50
	Accuracy 0.1600 8/50 (MiniBatch)
	Avg Loss 0.8364 41.8182/50 (MiniBatch)
...

Epoch 10/50
	Accuracy 0.6800 34/50 (MiniBatch)
	Avg Loss 0.5026 25.1296/50 (MiniBatch)
...

Epoch 25/50
	Accuracy 0.8400 42/50 (MiniBatch)
	Avg Loss 0.3456 17.2794/50 (MiniBatch)
...

Epoch 50/50
	Accuracy 0.9200 46/50 (MiniBatch)
	Avg Loss 0.2918 14.5919/50 (MiniBatch)

[Successful y prediction] correct label is Versicolor
=> for corresponding x input features, see tabular row
╭───┬─────┬───┬─────╮
│ 6 ┆ 2.2 ┆ 5 ┆ 1.5 │
╰───┴─────┴───┴─────╯
[Successful y prediction] correct label is Virginica
=> for corresponding x input features, see tabular row
╭───┬─────┬─────┬─────╮
│ 6 ┆ 2.9 ┆ 4.5 ┆ 1.5 │
╰───┴─────┴─────┴─────╯
[Successful y prediction] correct label is Setosa
=> for corresponding x input features, see tabular row
╭─────┬─────┬─────┬─────╮
│ 5.2 ┆ 3.5 ┆ 1.5 ┆ 0.2 │
╰─────┴─────┴─────┴─────╯
[Successful y prediction] correct label is Setosa
=> for corresponding x input features, see tabular row
╭─────┬─────┬─────┬─────╮
│ 5.4 ┆ 3.4 ┆ 1.5 ┆ 0.4 │
╰─────┴─────┴─────┴─────╯
Accuracy 0.9200 46/50 
Avg Loss 0.2918 14.5919/50 

```

### 3) Pima Indians diabetes

```rust
$ cargo run --release -- -t diab
   Compiling neuron_dance v0.1.0 (/home/brpandey/Workspace/ml/rust/neuron_dance)
    Finished release [optimized] target(s) in 32.87s
     Running `target/release/neuron_dance -t diab`
> head csv-file
╭─────────────┬─────────┬───────────────┬───────────────┬─────────┬──────┬──────────────────────────┬─────┬─────────╮
│ Pregnancies ┆ Glucose ┆ BloodPressure ┆ SkinThickness ┆ Insulin ┆ BMI  ┆ DiabetesPedigreeFunction ┆ Age ┆ Outcome │
╞═════════════╪═════════╪═══════════════╪═══════════════╪═════════╪══════╪══════════════════════════╪═════╪═════════╡
│ 4           ┆ 95      ┆ 60            ┆ 32            ┆ 0       ┆ 35.4 ┆ 0.284                    ┆ 28  ┆ 0       │
├╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ 5           ┆ 103     ┆ 108           ┆ 37            ┆ 0       ┆ 39.2 ┆ 0.305                    ┆ 65  ┆ 0       │
├╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ 0           ┆ 137     ┆ 68            ┆ 14            ┆ 148     ┆ 24.8 ┆ 0.143                    ┆ 21  ┆ 0       │
├╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ 1           ┆ 173     ┆ 74            ┆ 0             ┆ 0       ┆ 36.8 ┆ 0.088                    ┆ 38  ┆ 1       │
╰─────────────┴─────────┴───────────────┴───────────────┴─────────┴──────┴──────────────────────────┴─────┴─────────╯
Data subset shapes x_train shape [512, 8], y_train shape  [512, 1], x_test shape [256, 8], y_test shape [256, 1]

Epoch 1/50
	Accuracy 0.6426 329/512 (MiniBatch)
	Avg Loss 0.6524 334.0516/512 (MiniBatch)

...

Epoch 50/50
	Accuracy 0.7578 388/512 (MiniBatch)
	Avg Loss 0.4798 245.6351/512 (MiniBatch)

Network {
    layers: Some(
        LayerStack contains 4 layers,
    ),
    weights: [
        [[0.017459398681886108, -0.3963610559385346, 0.28278924447170894, -0.434870635557442, 0.6528612364200012, -0.8612078350343334, 0.14272056108247513, 0.2900210161595015],
         [-0.05563445058127381, -0.15838418325772946, -0.08669751863741845, -0.0050832688272863455, 0.14617174882732015, 0.09086819903185947, -0.45285707401148967, 0.3589401335322466],
         [-0.5831414334931037, -1.3272823182968827, 0.10102594304481513, -1.0553820595173047, 0.6287911485365377, -0.32361688711020725, 0.5865956777759658, -0.1572324612755122],
         [0.33891623297924417, 0.9035082978917123, -0.3997205053182783, -0.02490401981930365, -0.7314016717728404, -0.05069701375549613, -0.2996337875642089, -0.07884395498855908],
         [-0.853254566560719, -1.028895114696597, 0.38184176630056005, -0.4341586266196852, 1.4393961596352733, -0.9656056585423078, -0.4085250062307234, -2.5518040239203135],
         [-0.344166206395312, 0.6589085434264966, 0.05666224633162948, -0.5261813457305841, -0.48256484597531135, -0.27656256570485066, 0.24526192939066765, -0.3695891714200727],
         [-0.9420890585290603, 0.7117645284573233, -0.6606932110731797, -0.2135454268482832, -0.07528444658964849, -0.32946646039045646, 0.563492597784996, -0.17233193497861712],
         [-0.6046117670498008, -1.3978970740645333, 0.910099724305276, -0.2760721141319019, -0.16258737858536232, -0.6797774223174735, -1.0170436124982465, -1.8819154944184728],
         [0.5716768826731953, 2.711430826301673, -0.6149105630313838, -0.2286010809622307, -0.3160471536506773, 1.762826050344381, 1.3352256420357154, 0.3030836092231372],
         [-0.0848849895071951, -0.8902874004171117, 0.37168083192465423, 0.1208420443346921, -0.9715450501327896, 0.23531300359178628, -1.0351052700034769, 0.8007176294518894],
         [0.3842990052219456, 0.6310118092258753, 0.9727224356564895, 0.4590684139239929, -1.0471795350797182, 0.13053056887264897, -0.07220204515506594, 0.9861112868846237],
         [0.04895539049401111, 1.0950003437341869, 0.47944853900868006, -0.13460400400855702, 0.21837434505718786, -0.42577240164927765, 0.6170747367720187, 0.06783574932648238]], shape=[12, 8], strides=[8, 1], layout=Cc (0x5), const ndim=2,
        [[0.921805779297582, 0.11239214930592611, 0.4805911515209199, -0.286001976648738, -1.2107707665514618, -0.6810374987227378, -0.29162247791661544, 0.3546371028159011, -1.4780605994689415, -0.2534862432909149, 0.6184009714904577, 0.32984627194447413],
         [0.22814071482434112, 0.05152288004812963, 0.8223360797054142, 0.0426201991133223, 1.7991843103539877, 0.19307640882151905, 0.5154939229732399, 0.8976872998923541, -0.45394898107802684, -0.2654605490299825, -1.025297594948223, -0.21663908670894202],
         [-0.42396344446031275, 0.3298098507981673, 0.6473525498438248, 0.30655470223572556, 0.24914889232693277, -0.26990940920792, 0.09377279329778955, 0.3340110727686183, -0.47848392759682, 0.12337691426166934, -0.27860696424912307, -0.9520759735746437],
         [-0.09201912734022963, -0.0016257845599023412, -0.2923157266522854, -0.11898770859120823, -0.18488520783620105, 0.2809229362247702, 0.08164327233691729, -0.680999705200826, -0.3202561132269621, -0.27118189315230945, 0.13278035199206223, -0.0037715261028515162],
         [0.25476174721471273, -0.13054131704202407, 0.5928780690577841, -0.7992218860508149, 0.1639962867693147, -0.09940858963238872, 0.2587229774885629, -0.19146631491051638, 0.2734343162342141, 0.12088152469544343, 0.03035232518748058, 0.6496160695547145],
         [-0.5316457868677319, -0.602710561180626, 0.05259898797834368, 0.24428673650047428, 0.39910435046556325, -0.7100445109810222, 0.4132487895627697, 0.4694463062634862, -0.3537737563928948, -0.4311325840423173, 0.20572912848012742, 0.030015704919707054],
         [-0.06366122937533722, 0.6065285691467471, -0.14035433020102078, -0.1815639882387285, 0.5569362451903892, 0.5827808195537811, -0.5400955158671323, 0.08248029661571157, -0.4916175870905958, -0.08345136331359995, -0.4462025084150292, -0.06966719150477428],
         [-0.09782411287573264, -0.30039845860832587, -0.724782633120201, 0.5864945384656849, -1.3618832615542842, 0.3735598068508272, 0.20326997029179483, -1.331434075076982, 1.8965593754250423, -0.626701387184544, 0.158805947289328, 0.26014484120356957]], shape=[8, 12], strides=[12, 1], layout=Cc (0x5), const ndim=2,
        [[1.147488262901232, -1.0312366743392625, -0.5408462226900261, 0.5074680472720474, 0.8191190674666096, 0.4117187663157184, -0.4257344197730393, 0.943346168938654]], shape=[1, 8], strides=[8, 1], layout=CFcf (0xf), const ndim=2,
    ],
    biases: [
        [[-0.7951713064339844],
         [-0.32620118043747415],
         [1.5226698222301238],
         [0.268867696551361],
         [1.8856735229564638],
         [0.6602804198166574],
         [0.5193734968603725],
         [1.6115652905742635],
         [-1.2348761822633525],
         [0.39490227412246215],
         [-0.598039840864072],
         [-1.4058106912740473]], shape=[12, 1], strides=[1, 1], layout=CFcf (0xf), const ndim=2,
        [[0.9378647357997031],
         [-0.6772472934524957],
         [-2.9352921934742593],
         [0.7033831623692821],
         [-1.0318811005429436],
         [-0.5479435467382909],
         [0.32954315638850473],
         [-0.7104435451957632]], shape=[8, 1], strides=[1, 1], layout=CFcf (0xf), const ndim=2,
        [[-2.0657455373434463]], shape=[1, 1], strides=[1, 1], layout=CFcf (0xf), const ndim=2,
    ],
    forward: [
        0x0000588962bc9430,
        0x0000588962bc9430,
        0x0000588962bbb370,
    ],
    hypers: Hypers {
        learning_rate: 0.5,
        l2_rate: 0.0,
        optimizer: Some(
            Some optimizer variant,
        ),
        class_size: 1,
        activations: [
            Relu,
            Relu,
            Sigmoid,
        ],
        loss_type: BinaryCrossEntropy,
        batch_type: Mini(
            10,
        ),
        optimizer_type: Default,
    },
    cache: None,
    metrics: Some(
        Metrics {
            metrics_map: {
                Cost: true,
                Accuracy: true,
            },
            cost_fp: 0x0000588962ba9080,
            one_hot: {
                0: [[1.0]], shape=[1, 1], strides=[1, 1], layout=CFcf (0xf), const ndim=2,
            },
            l2_rate: 0.0,
        },
    ),
}
Accuracy 0.7734 198/256 
Avg Loss 0.4883 124.9985/256 

Accuracy 0.7734 198/256 

```

### 4) Preloaded dataset from fashion mnist model dump

```rust
$ cargo run --release -- -t pre
    Finished release [optimized] target(s) in 0.19s
     Running `target/release/neuron_dance -t pre`
// > head 0..7 mnist-file | heatmap

Data subset shapes are x_train shape [60000, 784], y_train shape  [60000, 1], x_test shape [10000, 784], y_test shape [10000, 1]

[Successful y prediction] correct label is Sneaker
=> for reduced x input image, see grid below
╭───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───╮
│   ┆   ┆   ┆   ┆   ┆   ┆   ┆   ┆   ┆   ┆   ┆   ┆   ┆   │
├╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┤
│   ┆   ┆   ┆   ┆   ┆   ┆   ┆ % ┆ @ ┆ % ┆   ┆   ┆   ┆   │
├╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┤
│   ┆   ┆   ┆   ┆   ┆ % ┆ @ ┆ @ ┆ X ┆ @ ┆   ┆   ┆ @ ┆ X │
├╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┤
│   ┆   ┆ % ┆ % ┆ % ┆ @ ┆ X ┆ X ┆ @ ┆ @ ┆ X ┆ @ ┆ X ┆ X │
├╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┤
│ % ┆ % ┆ % ┆ @ ┆ @ ┆ @ ┆ @ ┆ X ┆ X ┆ % ┆ @ ┆ X ┆ X ┆ X │
├╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┤
│ % ┆ % ┆ % ┆ @ ┆ @ ┆ @ ┆ @ ┆ X ┆ X ┆ X ┆ @ ┆ X ┆ @ ┆ X │
├╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┤
│ % ┆ % ┆ % ┆ % ┆ % ┆ % ┆ % ┆ % ┆ % ┆ % ┆ % ┆ % ┆ % ┆ % │
├╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┤
│   ┆   ┆   ┆   ┆   ┆   ┆   ┆   ┆   ┆   ┆   ┆   ┆   ┆   │
╰───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───╯
[Successful y prediction] correct label is Sandal
=> for reduced x input image, see grid below
╭───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───╮
│   ┆   ┆   ┆   ┆   ┆   ┆   ┆   ┆   ┆   ┆   ┆   ┆   ┆   │
├╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┤
│   ┆   ┆   ┆   ┆   ┆   ┆   ┆ @ ┆ @ ┆ % ┆ % ┆ % ┆ % ┆   │
├╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┤
│   ┆   ┆   ┆ % ┆   ┆   ┆   ┆ % ┆ % ┆ @ ┆ X ┆ @ ┆ @ ┆   │
├╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┤
│   ┆   ┆   ┆ @ ┆ @ ┆ % ┆   ┆   ┆ @ ┆ # ┆ X ┆ X ┆ % ┆   │
├╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┤
│   ┆   ┆ % ┆ % ┆ @ ┆ @ ┆ @ ┆   ┆   ┆ X ┆ X ┆ @ ┆   ┆   │
├╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┤
│ % ┆   ┆ @ ┆ @ ┆ @ ┆ % ┆ @ ┆ @ ┆ % ┆ X ┆ X ┆ % ┆ % ┆ % │
├╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┤
│ % ┆ % ┆ @ ┆ % ┆ % ┆ % ┆ % ┆ % ┆ % ┆ % ┆ % ┆ % ┆ % ┆ % │
├╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┤
│   ┆   ┆   ┆   ┆   ┆   ┆   ┆   ┆   ┆   ┆   ┆   ┆   ┆   │
╰───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───╯
[Successful y prediction] correct label is Sneaker
=> for reduced x input image, see grid below
╭───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───╮
│   ┆   ┆   ┆   ┆   ┆   ┆   ┆   ┆   ┆   ┆   ┆   ┆   ┆   │
├╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┤
│   ┆   ┆   ┆   ┆   ┆   ┆   ┆ % ┆ % ┆   ┆ % ┆   ┆ % ┆ % │
├╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┤
│   ┆   ┆   ┆   ┆ % ┆ @ ┆ % ┆ @ ┆ X ┆ @ ┆ % ┆ % ┆ X ┆ % │
├╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┤
│   ┆   ┆   ┆ @ ┆ X ┆ X ┆ X ┆ X ┆ X ┆ X ┆ # ┆ X ┆ X ┆ @ │
├╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┤
│ % ┆ @ ┆ @ ┆ @ ┆ @ ┆ @ ┆ X ┆ X ┆ X ┆ X ┆ X ┆ X ┆ X ┆ @ │
├╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┤
│ % ┆ % ┆ % ┆ @ ┆ @ ┆ @ ┆ @ ┆ @ ┆ % ┆ % ┆ % ┆ % ┆ % ┆ % │
├╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┤
│ % ┆ % ┆ % ┆ % ┆ % ┆ % ┆ % ┆ % ┆ % ┆ % ┆ % ┆ % ┆ % ┆ % │
├╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┤
│   ┆   ┆   ┆   ┆   ┆   ┆   ┆   ┆   ┆   ┆   ┆   ┆   ┆   │
╰───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───╯
[Successful y prediction] correct label is Bag
=> for reduced x input image, see grid below
╭───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───╮
│   ┆   ┆   ┆   ┆ % ┆ % ┆ @ ┆ @ ┆ % ┆   ┆   ┆   ┆   ┆   │
├╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┤
│   ┆   ┆   ┆   ┆ % ┆ @ ┆ % ┆ @ ┆ % ┆ % ┆   ┆   ┆   ┆   │
├╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┤
│   ┆   ┆   ┆ % ┆ % ┆ % ┆ % ┆ % ┆ % ┆ % ┆   ┆   ┆   ┆   │
├╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┤
│   ┆   ┆   ┆ % ┆ % ┆ % ┆   ┆ % ┆ % ┆ % ┆ % ┆   ┆   ┆   │
├╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┤
│   ┆   ┆ % ┆ @ ┆ % ┆ % ┆   ┆   ┆ % ┆ @ ┆ % ┆   ┆   ┆   │
├╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┤
│   ┆   ┆   ┆ @ ┆ % ┆ % ┆ % ┆ % ┆ % ┆ @ ┆ % ┆ % ┆   ┆   │
├╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┤
│   ┆   ┆   ┆ @ ┆ @ ┆ % ┆ % ┆ % ┆ % ┆ % ┆ % ┆ % ┆   ┆   │
├╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┤
│   ┆   ┆ @ ┆ X ┆ X ┆ X ┆ X ┆ X ┆ X ┆ X ┆ @ ┆   ┆   ┆   │
├╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┤
│   ┆   ┆ X ┆ X ┆ X ┆ @ ┆ @ ┆ @ ┆ @ ┆ X ┆ @ ┆ % ┆   ┆   │
├╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┤
│   ┆   ┆ # ┆ X ┆ X ┆ X ┆ X ┆ X ┆ X ┆ X ┆ X ┆   ┆   ┆   │
├╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┤
│   ┆   ┆ # ┆ X ┆ X ┆ X ┆ X ┆ X ┆ X ┆ X ┆ X ┆ % ┆   ┆   │
├╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┤
│   ┆   ┆ X ┆ X ┆ X ┆ X ┆ # ┆ X ┆ X ┆ X ┆ X ┆ @ ┆   ┆   │
├╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┤
│   ┆   ┆ X ┆ X ┆ X ┆ X ┆ @ ┆ X ┆ X ┆ X ┆ X ┆ X ┆   ┆   │
├╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┼╌╌╌┤
│   ┆   ┆ # ┆ X ┆ X ┆ X ┆ # ┆ X ┆ X ┆ X ┆ X ┆ % ┆   ┆   │
╰───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───╯
Accuracy 0.8409 8409/10000 

```


## Additional installation (Linux)

```bash

// for blas - optimized matrix compute
sudo apt install cmake
sudo apt install gfortran

// for flamegraph / perf
sudo apt install linux-tools-common linux-tools-generic
export CARGO_PROFILE_RELEASE_DEBUG=true

sudo apt-get install libatlas-base-dev
cargo flamegraph

sudo sysctl kernel.perf_event_paranoid=3

perf record --call-graph=dwarf -- target/release/neuron_dance -t mnist

perf report -n

// for plotters (heatmaps)
sudo apt-get install libfontconfig libfontconfig1-dev

```

[TODO](TODO.md)

