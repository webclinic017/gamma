#include <iostream>
#include <vector>

#include "engine.hpp"

/* algorithms */
#include "algorithms/test_algo.hpp"



int main()
{
    TestAlgorithm testalgo;

    std::vector<double> data {1.0, 2.0};

    Engine engine(data, testalgo);
    engine.run();
}