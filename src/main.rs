use simple_network::{
    network::Network,
    dataset::{DataSet, csv::{CSVType, CSVData},
              idx::{MnistType, MnistData}},
    layers::{Act, Batch, Eval, Loss, Mett, Input1, Input2, Dense},
};

fn main() {
    let csv = false; //false;
    let train_percentage = 2.0/3.0;     // train / total ratio, test = total - train
    let mut dataset: Box<dyn DataSet>;

    if csv {
        dataset = Box::new(CSVData::new(CSVType::RGB));
    } else {
        dataset = Box::new(MnistData::new(MnistType::Regular));
    }

    let tts = dataset.train_test_split(train_percentage);
    let mut model;
    let subsets = tts.get_ref();

    if csv {
        model = Network::new();
        model.add(Input1(3));
        model.add(Dense(3, Act::Relu));
        model.add(Dense(1, Act::Sigmoid));
        model.compile(Loss::Quadratic, 0.2, vec![Mett::Accuracy, Mett::Cost]);
        model.fit(&subsets, 10000, Batch::SGD, Eval::Train); // using SGD approach (doesn't have momentum supported)
    } else {
        model = Network::new();
        model.add(Input2(28, 28));
        model.add(Dense(50, Act::Sigmoid));
        model.add(Dense(10, Act::Sigmoid));
        model.compile(Loss::Quadratic, 0.3, vec![Mett::Accuracy, Mett::Cost]);
        model.fit(&subsets, 20, Batch::Mini(32), Eval::Train);
    }

    model.eval(&subsets, Eval::Test);
}
