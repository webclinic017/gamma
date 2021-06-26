#ifndef ALGORITHM_HPP_
#define ALGORITHM_HPP_

#include <map>
#include <vector>

class Algorithm
{
    protected:
        std::map<std::string, double> target;

    public:
        Algorithm() {};
        virtual ~Algorithm() {};
        virtual std::map<std::string, double>*
        run(std::vector<double>::iterator, std::vector<double>::iterator) {
            return &(this->target);
        }
};

#endif