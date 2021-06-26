#ifndef TEST_ALGO_HPP_
#define TEST_ALGO_HPP_

#include <map>
#include <vector>
#include <string>

#include "algorithm.hpp"


class TestAlgorithm : public Algorithm {
    public:
        TestAlgorithm()
        {
            this->target = std::map<std::string, double>();
        }
        
        ~TestAlgorithm() {}

        std::map<std::string, double>*
        run(std::vector<double>::iterator b, std::vector<double>::iterator e)
        {
            this->target.insert(
                std::make_pair(std::string("GOOG"), 1.0)
            );

            return &(this->target);
        }

};

#endif