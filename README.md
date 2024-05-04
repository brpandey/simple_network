# neuron_dance

<p float="center">
  <img src='images/dance.jpg' width='600' height='425'/>
</p>

```rust

let mut tts = dataset.train_test_split(train_percentage);
let mut subsets = tts.get_ref();
let mut model;

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

<p float="center">
  <img src='images/fashion.jpg' width='950' height='325'/>
</p>

<p float="center">
  <img src='images/digits.jpg' width='950' height='325'/>
</p>

> Example Usage

```rust

$ cargo run --release -- -t mnist
    Finished release [optimized] target(s) in 0.19s
     Running `/home/brpandey/Workspace/ml/rust/neuron_dance/target/release/neuron_dance -t mnist`

Data subset shapes are x_train shape [60000, 784], y_train shape  [60000, 1],
x_test shape [10000, 784], y_test shape [10000, 1]

Epoch 0/5
	Accuracy 0.9571 9571/10000 (MiniBatch + Adam)

Epoch 1/5
	Accuracy 0.9659 9659/10000 (MiniBatch + Adam)

Epoch 2/5
	Accuracy 0.9689 9689/10000 (MiniBatch + Adam)

Epoch 3/5
	Accuracy 0.9700 9700/10000 (MiniBatch + Adam)

Epoch 4/5
	Accuracy 0.9720 9720/10000 (MiniBatch + Adam)

Successful y prediction, correct label is 9
-- See reduced, randomly chosen x input image below --
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
Successful y prediction, correct label is 3
-- See reduced, randomly chosen x input image below --
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
Successful y prediction, correct label is 7
-- See reduced, randomly chosen x input image below --
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
No match! y prediction 3 is different from correct y label 5
-- See reduced, randomly chosen x input image below --
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

```rust

$ cargo run --release -- -t iris
    Finished release [optimized] target(s) in 0.20s
     Running `/home/brpandey/Workspace/ml/rust/neuron_dance/target/release/neuron_dance -t iris`
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
Data subset shapes x_train shape [100, 4], y_train shape  [100, 1], x_test shape [50, 4], y_test shape [50, 1]

Epoch 0/50
	Accuracy 0.1600 8/50 (MiniBatch)
	Avg Loss 0.8364 41.8182/50 (MiniBatch)

Epoch 1/50
	Accuracy 0.2800 14/50 (MiniBatch)
	Avg Loss 0.8345 41.7261/50 (MiniBatch)

...

Epoch 9/50
	Accuracy 0.6400 32/50 (MiniBatch)
	Avg Loss 0.5291 26.4553/50 (MiniBatch)

Epoch 10/50
	Accuracy 0.6800 34/50 (MiniBatch)
	Avg Loss 0.5026 25.1296/50 (MiniBatch)

...

Epoch 25/50
	Accuracy 0.8400 42/50 (MiniBatch)
	Avg Loss 0.3456 17.2794/50 (MiniBatch)

...

Epoch 48/50
	Accuracy 0.9200 46/50 (MiniBatch)
	Avg Loss 0.2935 14.6738/50 (MiniBatch)

Epoch 49/50
	Accuracy 0.9200 46/50 (MiniBatch)
	Avg Loss 0.2918 14.5919/50 (MiniBatch)

Successful y prediction, correct label is 2
-- See corresponding, x input below --
╭───┬─────┬───┬─────╮
│ 6 ┆ 2.2 ┆ 5 ┆ 1.5 │
╰───┴─────┴───┴─────╯
Successful y prediction, correct label is 1
-- See corresponding, x input below --
╭───┬─────┬─────┬─────╮
│ 6 ┆ 2.9 ┆ 4.5 ┆ 1.5 │
╰───┴─────┴─────┴─────╯
Successful y prediction, correct label is 0
-- See corresponding, x input below --
╭─────┬─────┬─────┬─────╮
│ 5.2 ┆ 3.5 ┆ 1.5 ┆ 0.2 │
╰─────┴─────┴─────┴─────╯
Successful y prediction, correct label is 0
-- See corresponding, x input below --
╭─────┬─────┬─────┬─────╮
│ 5.4 ┆ 3.4 ┆ 1.5 ┆ 0.4 │
╰─────┴─────┴─────┴─────╯
Accuracy 0.9200 46/50 
Avg Loss 0.2918 14.5919/50 

```
